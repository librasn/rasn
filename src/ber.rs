pub mod de;
pub mod enc;
mod identifier;
mod rules;

pub use identifier::Identifier;
pub(crate) use rules::EncodingRules;

pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, de::Error> {
    T::decode(&mut de::Decoder::new(input, de::DecoderOptions::ber()))
}

pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    let mut enc = enc::Encoder::new(enc::EncoderOptions::ber());

    value.encode(&mut enc)?;

    Ok(enc.output)
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{
        tag::{Class, Tag},
        types::*,
        AsnType,
    };

    use super::*;

    #[test]
    fn null() {
        assert_eq!((), decode::<()>(&*encode(&()).unwrap()).unwrap());
    }

    #[test]
    fn bool() {
        assert_eq!(true, decode(&encode(&true).unwrap()).unwrap());
        assert_eq!(false, decode(&encode(&false).unwrap()).unwrap());
    }

    macro_rules! integer_tests {
        ($($integer:ident),*) => {
            $(
                #[test]
                fn $integer() {
                    let min = <$integer>::min_value();
                    let max = <$integer>::max_value();

                    assert_eq!(min, decode(&encode(&min).unwrap()).unwrap());
                    assert_eq!(max, decode(&encode(&max).unwrap()).unwrap());
                }
            )*
        }
    }

    integer_tests! {
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        u8,
        u16,
        u32,
        u64,
        u128,
        usize
    }

    #[test]
    fn octet_string() {
        let a = OctetString::from(vec![1u8, 2, 3, 4, 5]);
        let b = OctetString::from(vec![5u8, 4, 3, 2, 1]);

        assert_eq!(
            a,
            decode::<OctetString>(&encode(&a).expect("encoding")).expect("decoding")
        );
        assert_eq!(b, decode::<OctetString>(&encode(&b).unwrap()).unwrap());
    }

    #[test]
    fn utf8_string() {
        let name = "Jones";
        assert_eq!(
            name,
            decode::<Utf8String>(&*encode(&name).unwrap()).unwrap()
        );
    }

    #[test]
    fn long_sequence_of() {
        let vec = vec![5u8; 0xffff];
        assert_eq!(
            vec,
            decode::<alloc::vec::Vec<u8>>(&encode(&vec).unwrap()).unwrap()
        );
    }

    #[test]
    fn object_identifier() {
        let iso = ObjectIdentifier::new(vec![1, 2]);
        let us_ansi = ObjectIdentifier::new(vec![1, 2, 840]);
        let rsa = ObjectIdentifier::new(vec![1, 2, 840, 113549]);
        let pkcs = ObjectIdentifier::new(vec![1, 2, 840, 113549, 1]);

        assert_eq!(iso.clone(), decode(&encode(&iso).unwrap()).unwrap());
        assert_eq!(us_ansi.clone(), decode(&encode(&us_ansi).unwrap()).unwrap());
        assert_eq!(rsa.clone(), decode(&encode(&rsa).unwrap()).unwrap());
        assert_eq!(pkcs.clone(), decode(&encode(&pkcs).unwrap()).unwrap());
    }

    #[test]
    fn bit_string() {
        let bits = BitString::from_slice(&[0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD]);

        assert_eq!(bits, decode::<BitString>(&encode(&bits).unwrap()).unwrap());
    }

    #[test]
    fn implicit_prefix() {
        #[derive(Debug, PartialEq)]
        struct C0;
        impl AsnType for C0 {
            const TAG: Tag = Tag::new(Class::Context, 0);
        }

        type MyInteger = Implicit<C0, u64>;

        let new_int = MyInteger::new(5);

        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }

    #[test]
    fn explicit_prefix() {
        #[derive(Debug, PartialEq)]
        struct C0;
        impl AsnType for C0 {
            const TAG: Tag = Tag::new(Class::Context, 0);
        }

        type MyInteger = Explicit<C0, u64>;

        let new_int = MyInteger::new(5);

        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }
}
