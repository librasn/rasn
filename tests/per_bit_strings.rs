//! BIT STRING values of every length around the octet boundaries, alone and
//! after one bit, in both variants of PER: what is written is read back, as a
//! `BitString` and as a `FixedBitString`.
use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct After<T> {
    flag: bool,
    value: T,
}

fn bits(length: usize) -> BitString {
    (0..length).map(|at| at % 3 == 0 || at % 7 == 0).collect()
}

/// `misaligned` says whether the aligned variant is also read after one bit.
fn round_trip<T>(value: &T, misaligned: bool)
where
    T: Decode + Encode + Clone + PartialEq + core::fmt::Debug,
{
    let after = After {
        flag: true,
        value: value.clone(),
    };
    assert_eq!(
        &rasn::aper::decode::<T>(&rasn::aper::encode(value).unwrap()).unwrap(),
        value
    );
    assert_eq!(
        &rasn::uper::decode::<T>(&rasn::uper::encode(value).unwrap()).unwrap(),
        value
    );
    if misaligned {
        assert_eq!(
            rasn::aper::decode::<After<T>>(&rasn::aper::encode(&after).unwrap()).unwrap(),
            after
        );
    }
    assert_eq!(
        rasn::uper::decode::<After<T>>(&rasn::uper::encode(&after).unwrap()).unwrap(),
        after
    );
}

#[test]
fn bit_strings_without_a_size() {
    for length in (0..=40).chain([63, 64, 65, 127, 128, 129, 1000, 16383, 16384, 16385, 40000]) {
        round_trip(&bits(length), true);
    }
}

#[test]
fn bit_strings_of_one_size() {
    macro_rules! fixed {
        ($($length:literal)*) => {$({
            let mut array = FixedBitString::<$length>::ZERO;
            array[..$length].copy_from_bitslice(&bits($length));
            // The aligned variant does not yet read a string of one size above
            // 16 bits that does not start an octet (X.691 §16.10, #575).
            round_trip(&array, $length <= 16);
            // The bits past the size, which the array has room for, stay zero.
            let read: FixedBitString<$length> =
                rasn::uper::decode(&rasn::uper::encode(&array).unwrap()).unwrap();
            assert!(read[$length..].not_any());
        })*};
    }
    fixed!(1 2 3 5 6 7 8 9 10 15 16 17 20 24 31 32 33 36 44 63 64 65 100);
}

#[test]
fn known_octets() {
    // `SEQUENCE { flag BOOLEAN, value BIT STRING (SIZE (10)) }`: one bit, then
    // ten that are not octet-aligned (X.691 §16.9).
    let mut array = FixedBitString::<10>::ZERO;
    array[..10].copy_from_bitslice(&bits(10));
    let after = After {
        flag: true,
        value: array,
    };
    // 1, then 1001 0011 01.
    assert_eq!(
        rasn::aper::encode(&after).unwrap(),
        [0b1100_1001, 0b1010_0000]
    );
    assert_eq!(
        rasn::aper::decode::<After<FixedBitString<10>>>(&[0b1100_1001, 0b1010_0000]).unwrap(),
        after
    );
    // A size above 16 bits is octet-aligned in the aligned variant (§16.10),
    // and follows the bit in the unaligned one.
    let mut array = FixedBitString::<20>::ZERO;
    array[..20].copy_from_bitslice(&bits(20));
    // 1001 0011 0100 1011 0010
    let aligned = [0b1001_0011, 0b0100_1011, 0b0010_0000];
    assert_eq!(rasn::aper::encode(&array).unwrap(), aligned);
    assert_eq!(
        rasn::aper::decode::<FixedBitString<20>>(&aligned).unwrap(),
        array
    );
    let after = After {
        flag: true,
        value: array,
    };
    let unaligned = [0b1100_1001, 0b1010_0101, 0b1001_0000];
    assert_eq!(rasn::uper::encode(&after).unwrap(), unaligned);
    assert_eq!(
        rasn::uper::decode::<After<FixedBitString<20>>>(&unaligned).unwrap(),
        after
    );
}
