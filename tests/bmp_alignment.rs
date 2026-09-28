use rasn::prelude::*;

macro_rules! bmp_type {
    ($name:ident, $size:literal) => {
        #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
        struct $name {
            before: bool,
            #[rasn(size($size))]
            text: BmpString,
            after: bool,
        }
    };
}
bmp_type!(Empty, "0");
bmp_type!(One, "1");

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
struct Extensible {
    before: bool,
    #[rasn(size("2", extensible))]
    text: BmpString,
    after: bool,
}

#[test]
fn fixed_bmp_alignment_and_extension_bit_order() {
    let empty = Empty {
        before: true,
        text: BmpString::try_from(&[][..]).unwrap(),
        after: true,
    };
    assert_eq!(rasn::aper::encode(&empty).unwrap(), [0xc0]);
    assert_eq!(rasn::aper::decode::<Empty>(&[0xc0]).unwrap(), empty);
    let one = One {
        before: true,
        text: BmpString::try_from(&[0, 0x61][..]).unwrap(),
        after: true,
    };
    assert_eq!(rasn::aper::encode(&one).unwrap(), [0x80, 0x30, 0xc0]);
    assert_eq!(rasn::aper::decode::<One>(&[0x80, 0x30, 0xc0]).unwrap(), one);
    let extensible = Extensible {
        before: true,
        text: BmpString::try_from(&[0, 0x61, 0, 0x62][..]).unwrap(),
        after: true,
    };
    assert_eq!(
        rasn::aper::encode(&extensible).unwrap(),
        [0x80, 0, 0x61, 0, 0x62, 0x80]
    );
    assert_eq!(
        rasn::aper::decode::<Extensible>(&[0x80, 0, 0x61, 0, 0x62, 0x80]).unwrap(),
        extensible
    );
}
