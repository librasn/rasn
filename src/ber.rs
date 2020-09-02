mod de;
mod enc;
mod identifier;

pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, de::Error> {
    T::decode(&mut de::Parser::new(input))
}

pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    let mut enc = enc::Encoder::default();

    value.encode(&mut enc)?;

    Ok(enc.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tag::{Class, Tag},
        types::*,
        AsnType, Decode, Encode,
    };
    use alloc::{vec, vec::Vec};

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

    #[test]
    fn sequence() {
        #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Bools {
            a: bool,
            b: bool,
            c: bool,
        }

        let raw = &[
            0x30, // Sequence tag
            9,    // Length
            1, 1, 0xff, // A
            1, 1, 0, // B
            1, 1, 0xff, // C
        ][..];

        let default = Bools {
            a: true,
            b: false,
            c: true,
        };

        assert_eq!(default, decode(&raw).unwrap());
        assert_eq!(raw, &*encode(&default).unwrap());

        // The representation of SEQUENCE and SEQUENCE OF are the same in this case.
        let bools_vec = vec![true, false, true];

        assert_eq!(bools_vec, decode::<Vec<bool>>(&raw).unwrap());
        assert_eq!(raw, &*encode(&bools_vec).unwrap());
    }

    #[test]
    fn enumerated() {
        #[derive(AsnType, Clone, Copy, Debug, Encode, Decode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[rasn(enumerated)]
        enum Foo {
            Ein,
            Zwei,
            Drei,
        }

        let ein = Foo::Ein;
        let zwei = Foo::Zwei;
        let drei = Foo::Drei;

        assert_eq!(ein, decode(&encode(&ein).unwrap()).unwrap());
        assert_eq!(zwei, decode(&encode(&zwei).unwrap()).unwrap());
        assert_eq!(drei, decode(&encode(&drei).unwrap()).unwrap());
    }

    #[test]
    fn choice() {
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[rasn(choice)]
        enum Choice {
            Bar(bool),
            Baz(OctetString),
        }

        let bar = Choice::Bar(true);
        let baz = Choice::Baz(OctetString::from(vec![1, 2, 3, 4, 5]));

        assert_eq!(bar, decode(&encode(&bar).unwrap()).unwrap());
        assert_eq!(baz, decode(&encode(&baz).unwrap()).unwrap());
    }

    /*
    #[test]
    fn optional() {
        env_logger::init();
        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        struct Struct {
            a: Optional<u8>,
        }
        let none = Struct { a: None.into() };
        let raw = encode(&none).unwrap();
        assert_eq!(&[0x30, 0][..], &*raw);
        assert_eq!(none, decode(&raw).unwrap());
        let some = Struct { a: Some(100).into() };
        assert_eq!(some, decode(&encode(&some).unwrap()).unwrap());
    }
    #[test]
    fn sequence_with_option() {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        struct Foo {
            a: u8,
            b: Optional<u8>,
        }
        let some = Foo { a: 1, b: Some(2).into() };
        let none = Foo { a: 1, b: None.into() };
        assert_eq!(some, decode(&encode(&some).unwrap()).unwrap());
        assert_eq!(none, decode(&encode(&none).unwrap()).unwrap());
    }

    #[test]
    fn nested_enum() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        enum Alpha {
            A(Implicit<Context, U0, Charlie>),
            B(Implicit<Context, U1, Charlie>),
        }


        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        enum Bravo {
            A,
            B,
        }

        impl Enumerable for Bravo {}

        type Charlie = Enumerated<Bravo>;

        let input = Alpha::A(Implicit::new(Enumerated::new(Bravo::B)));

        assert_eq!(input, decode(&encode(&input).unwrap()).unwrap())
    }
    */
}
