//! A decoder that knows fewer extension additions of a SEQUENCE or SET than
//! the encoder skips the others, and decodes what follows them.
use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Root {
    a: bool,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct OneAddition {
    a: bool,
    #[rasn(extension_addition)]
    b: Option<u8>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct ThreeAdditions {
    a: bool,
    #[rasn(extension_addition)]
    b: Option<u8>,
    #[rasn(extension_addition)]
    c: Option<OctetString>,
    #[rasn(extension_addition)]
    d: Option<bool>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct Outer<T> {
    before: bool,
    inner: T,
    after: u8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(set, automatic_tags)]
#[non_exhaustive]
struct RootSet {
    a: bool,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(set, automatic_tags)]
#[non_exhaustive]
struct SetWithAddition {
    a: bool,
    #[rasn(extension_addition)]
    b: Option<u8>,
}

fn newer() -> Outer<ThreeAdditions> {
    Outer {
        before: true,
        inner: ThreeAdditions {
            a: true,
            b: Some(7),
            c: Some(OctetString::from_static(&[1, 2, 3])),
            d: Some(true),
        },
        after: 0x5a,
    }
}

macro_rules! both_codecs {
    ($test:ident) => {
        $test(rasn::aper::encode, rasn::aper::decode);
        $test(rasn::uper::encode, rasn::uper::decode);
    };
}

type Encode<T> = fn(&T) -> Result<Vec<u8>, rasn::error::EncodeError>;
type Decode<T> = fn(&[u8]) -> Result<T, rasn::error::DecodeError>;

#[test]
fn unknown_additions_of_a_sequence_are_skipped() {
    fn check(encode: Encode<Outer<ThreeAdditions>>, decode: Decode<Outer<Root>>) {
        let wire = encode(&newer()).unwrap();
        assert_eq!(
            decode(&wire).unwrap(),
            Outer {
                before: true,
                inner: Root { a: true },
                after: 0x5a,
            }
        );
    }
    both_codecs!(check);
}

#[test]
fn additions_after_the_known_ones_are_skipped() {
    fn check(encode: Encode<Outer<ThreeAdditions>>, decode: Decode<Outer<OneAddition>>) {
        let wire = encode(&newer()).unwrap();
        assert_eq!(
            decode(&wire).unwrap(),
            Outer {
                before: true,
                inner: OneAddition {
                    a: true,
                    b: Some(7),
                },
                after: 0x5a,
            }
        );
    }
    both_codecs!(check);
}

#[test]
fn absent_unknown_additions_are_not_read() {
    fn check(encode: Encode<Outer<ThreeAdditions>>, decode: Decode<Outer<Root>>) {
        let mut value = newer();
        value.inner.b = None;
        value.inner.c = None;
        let wire = encode(&value).unwrap();
        assert_eq!(decode(&wire).unwrap().after, 0x5a);
    }
    both_codecs!(check);
}

#[test]
fn a_truncated_unknown_addition_is_an_error() {
    fn check(encode: Encode<Outer<ThreeAdditions>>, decode: Decode<Outer<Root>>) {
        let wire = encode(&newer()).unwrap();
        assert!(decode(&wire[..wire.len() - 2]).is_err());
    }
    both_codecs!(check);
}

#[test]
fn unknown_additions_of_a_set_are_skipped() {
    fn check(encode: Encode<Outer<SetWithAddition>>, decode: Decode<Outer<RootSet>>) {
        let value = Outer {
            before: false,
            inner: SetWithAddition {
                a: true,
                b: Some(9),
            },
            after: 0xa5,
        };
        let wire = encode(&value).unwrap();
        assert_eq!(
            decode(&wire).unwrap(),
            Outer {
                before: false,
                inner: RootSet { a: true },
                after: 0xa5,
            }
        );
    }
    both_codecs!(check);
}
