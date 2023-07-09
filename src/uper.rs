//! # Unaligned Packed Encoding Rules

use crate::types::Constraints;

pub use super::per::*;

/// Attempts to decode `T` from `input` using UPER-BASIC.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::per::de::Error> {
    crate::per::decode(de::DecoderOptions::unaligned(), input)
}

/// Attempts to encode `value` to UPER-CANONICAL.
pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    crate::per::encode(enc::EncoderOptions::unaligned(), value)
}

/// Attempts to decode `T` from `input` using UPER-BASIC.
pub fn decode_with_constraints<T: crate::Decode>(
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::per::de::Error> {
    crate::per::decode_with_constraints(de::DecoderOptions::unaligned(), constraints, input)
}

/// Attempts to encode `value` to UPER-CANONICAL.
pub fn encode_with_constraints<T: crate::Encode>(
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    crate::per::encode_with_constraints(enc::EncoderOptions::unaligned(), constraints, value)
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        types::{constraints::*, *},
    };

    #[test]
    fn bool() {
        round_trip!(uper, bool, true, &[0x80]);
        round_trip!(uper, bool, false, &[0]);
    }

    #[test]
    fn integer() {
        round_trip!(uper, Integer, 32768.into(), &[0x03, 0x00, 0x80, 0x00]);
        round_trip!(uper, Integer, 32767.into(), &[0x02, 0x7f, 0xff]);
        round_trip!(uper, Integer, 256.into(), &[0x02, 0x01, 0x00]);
        round_trip!(uper, Integer, 255.into(), &[0x02, 0x00, 0xff]);
        round_trip!(uper, Integer, 128.into(), &[0x02, 0x00, 0x80]);
        round_trip!(uper, Integer, 127.into(), &[0x01, 0x7f]);
        round_trip!(uper, Integer, 1.into(), &[0x01, 0x01]);
        round_trip!(uper, Integer, 0.into(), &[0x01, 0x00]);
        round_trip!(uper, Integer, (-1).into(), &[0x01, 0xff]);
        round_trip!(uper, Integer, (-128).into(), &[0x01, 0x80]);
        round_trip!(uper, Integer, (-129).into(), &[0x02, 0xff, 0x7f]);
        round_trip!(uper, Integer, (-256).into(), &[0x02, 0xff, 0x00]);
        round_trip!(uper, Integer, (-32768).into(), &[0x02, 0x80, 0x00]);
        round_trip!(uper, Integer, (-32769).into(), &[0x03, 0xff, 0x7f, 0xff]);

        type B = ConstrainedInteger<5, 99>;
        type C = ConstrainedInteger<-10, 10>;
        //type D = ExtensibleConstrainedInteger<5, 99>;
        type E = ConstrainedInteger<1000, 1000>;

        round_trip!(uper, B, 5.into(), &[0x00]);
        round_trip!(uper, B, 6.into(), &[0x02]);
        round_trip!(uper, B, 99.into(), &[0xbc]);
        round_trip!(uper, C, (-10).into(), &[0x00]);
        round_trip!(uper, C, (-1).into(), &[0x48]);
        round_trip!(uper, C, 0.into(), &[0x50]);
        round_trip!(uper, C, 1.into(), &[0x58]);
        round_trip!(uper, C, 10.into(), &[0xa0]);
        // round_trip!(uper, D, 99, &[0x5e]);
        round_trip!(uper, E, Integer::from(1000).into(), &[]);
    }

    #[test]
    fn sequence_of() {
        round_trip!(uper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
        round_trip!(aper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
    }

    #[test]
    #[ignore]
    fn numeric_string() {
        round_trip!(
            uper,
            NumericString,
            " 0123456789".try_into().unwrap(),
            &[0x0b, 0x01, 0x23, 0x45, 0x67, 0x89, 0xa0]
        );
        round_trip_with_constraints!(
            uper,
            NumericString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(5)).into())]),
            "1 9 5".try_into().unwrap(),
            &[0x20, 0xa0, 0x60]
        );

        round_trip_with_constraints!(
            uper,
            NumericString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: 19.into(),
                    end: 134.into()
                })
                .into()
            )]),
            "0123456789 9876543210".try_into().unwrap(),
            &[0x04, 0x24, 0x68, 0xac, 0xf1, 0x34, 0x15, 0x30, 0xec, 0xa8, 0x64, 0x20]
        );

        round_trip_with_constraints!(
            uper,
            NumericString,
            Constraints::new(&[Constraint::PermittedAlphabet(
                PermittedAlphabet::new(&[
                    b'0' as u32,
                    b'1' as u32,
                    b'2' as u32,
                    b'3' as u32,
                    b'4' as u32,
                    b'5' as u32
                ])
                .into()
            )]),
            "5".try_into().unwrap(),
            &[0x01, 0xa0]
        );
    }

    #[test]
    fn visible_string() {
        round_trip_with_constraints!(
            uper,
            VisibleString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: 19.into(),
                    end: 133.into()
                })
                .into()
            )]),
            "HejHoppHappHippAbcde".try_into().unwrap(),
            &[
                0x03, 0x23, 0x2e, 0xa9, 0x1b, 0xf8, 0x70, 0x91, 0x87, 0x87, 0x09, 0x1a, 0x78, 0x70,
                0x83, 0x8b, 0x1e, 0x4c, 0xa0
            ]
        );

        round_trip_with_constraints!(
            uper,
            VisibleString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(5)).into())]),
            "Hejaa".try_into().unwrap(),
            &[0x91, 0x97, 0x56, 0x1c, 0x20]
        );

        const ALPHABET: &[u32] = &{
            let mut array = [0; 26];
            let mut i = 0;
            let mut start = 'a' as u32;
            let end = 'z' as u32;
            loop {
                array[i] = start;
                start += 1;
                i += 1;

                if start > end {
                    break;
                }
            }

            array
        };

        round_trip_with_constraints!(
            uper,
            VisibleString,
            Constraints::new(&[
                Constraint::Size(
                    Size::new(Bounded::Range {
                        start: 1.into(),
                        end: 255.into()
                    })
                    .into()
                ),
                Constraint::PermittedAlphabet(PermittedAlphabet::new(ALPHABET).into()),
            ]),
            "hej".try_into().unwrap(),
            &[0x02, 0x39, 0x12]
        );
    }

    #[test]
    fn choice() {
        use crate as rasn;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum Choice {
            Normal,
            High,
            #[rasn(extension_addition)]
            Medium,
        }

        round_trip!(uper, Choice, Choice::Normal, &[0]);
        round_trip!(uper, Choice, Choice::Medium, &[0x80, 1, 0]);
        round_trip!(aper, Choice, Choice::Medium, &[0x80, 1, 0]);
    }

    #[test]
    fn enumerated() {
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Enum1 {
            Green,
            Red,
            Blue,
        }

        round_trip!(uper, Enum1, Enum1::Green, &[0]);
        round_trip!(uper, Enum1, Enum1::Red, &[0x40]);
        round_trip!(uper, Enum1, Enum1::Blue, &[0x80]);

        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[non_exhaustive]
        enum Enum2 {
            Red,
            Blue,
            Green,
            #[rasn(extension_addition)]
            Yellow,
            #[rasn(extension_addition)]
            Purple,
        }

        round_trip!(uper, Enum2, Enum2::Red, &[0]);
        round_trip!(uper, Enum2, Enum2::Yellow, &[0x80]);
        round_trip!(uper, Enum2, Enum2::Purple, &[0x81]);
    }

    #[test]
    fn sequence() {
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct B {
            #[rasn(default)]
            a: Integer,
        }

        fn true_identity() -> bool {
            true
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct C {
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct D {
            a: bool,
            #[rasn(extension_addition_group)]
            b: Option<DE>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct DE {
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct F {
            a: bool,
            #[rasn(extension_addition_group)]
            b: Option<FE>,
            #[rasn(extension_addition)]
            c: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct FE {
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct G {
            a: bool,
            d: bool,
            #[rasn(extension_addition_group)]
            b: Option<GE>,
            #[rasn(extension_addition_group)]
            c: Option<GE>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct GE {
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct I {
            a: bool,
            #[rasn(extension_addition)]
            b: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct J {
            a: bool,
            #[rasn(extension_addition)]
            b: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct K {
            a: bool,
            #[rasn(extension_addition)]
            b: Option<bool>,
            #[rasn(extension_addition)]
            c: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct L {
            a: bool,
            #[rasn(extension_addition_group)]
            b: Option<LE>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct LE {
            a: bool,
            b: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct M {
            a: bool,
            #[rasn(extension_addition_group)]
            b: Option<ME>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct ME {
            a: Option<MESeq>,
            b: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct MESeq {
            a: Integer,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct N {
            #[rasn(default = "true_identity")]
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct O {
            #[rasn(extension_addition, default = "true_identity")]
            a: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct P {
            #[rasn(extension_addition_group)]
            a: Option<PE>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct PE {
            a: bool,
            #[rasn(default = "true_identity")]
            b: bool,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Q {
            a: C,
            b: Integer,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct R {
            a: D,
            b: Integer,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct S {
            a: bool,
            #[rasn(extension_addition)]
            b: Option<SSeq>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct SSeq {
            a: bool,
            b: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Default, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct T {
            a: Option<SequenceOf<T>>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct U {
            #[rasn(extension_addition)]
            a: USeq,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct USeq {
            a: Integer,
        }

        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct V {
            #[rasn(extension_addition)]
            a: Option<bool>,
            #[rasn(extension_addition)]
            b: Option<bool>,
            #[rasn(extension_addition)]
            c: Option<bool>,
        }

        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", automatic_tags)]
        #[non_exhaustive]
        struct W {
            #[rasn(extension_addition)]
            a1: Option<bool>,
            #[rasn(extension_addition)]
            a2: Option<bool>,
            #[rasn(extension_addition)]
            a3: Option<bool>,
            #[rasn(extension_addition)]
            a4: Option<bool>,
            #[rasn(extension_addition)]
            a5: Option<bool>,
            #[rasn(extension_addition)]
            a6: Option<bool>,
            #[rasn(extension_addition)]
            a7: Option<bool>,
            #[rasn(extension_addition)]
            a8: Option<bool>,
            #[rasn(extension_addition)]
            a9: Option<bool>,
            #[rasn(extension_addition)]
            a10: Option<bool>,
            #[rasn(extension_addition)]
            a11: Option<bool>,
            #[rasn(extension_addition)]
            a12: Option<bool>,
            #[rasn(extension_addition)]
            a13: Option<bool>,
            #[rasn(extension_addition)]
            a14: Option<bool>,
            #[rasn(extension_addition)]
            a15: Option<bool>,
            #[rasn(extension_addition)]
            a16: Option<bool>,
            #[rasn(extension_addition)]
            a17: Option<bool>,
            #[rasn(extension_addition)]
            a18: Option<bool>,
            #[rasn(extension_addition)]
            a19: Option<bool>,
            #[rasn(extension_addition)]
            a20: Option<bool>,
            #[rasn(extension_addition)]
            a21: Option<bool>,
            #[rasn(extension_addition)]
            a22: Option<bool>,
            #[rasn(extension_addition)]
            a23: Option<bool>,
            #[rasn(extension_addition)]
            a24: Option<bool>,
            #[rasn(extension_addition)]
            a25: Option<bool>,
            #[rasn(extension_addition)]
            a26: Option<bool>,
            #[rasn(extension_addition)]
            a27: Option<bool>,
            #[rasn(extension_addition)]
            a28: Option<bool>,
            #[rasn(extension_addition)]
            a29: Option<bool>,
            #[rasn(extension_addition)]
            a30: Option<bool>,
            #[rasn(extension_addition)]
            a31: Option<bool>,
            #[rasn(extension_addition)]
            a32: Option<bool>,
            #[rasn(extension_addition)]
            a33: Option<bool>,
            #[rasn(extension_addition)]
            a34: Option<bool>,
            #[rasn(extension_addition)]
            a35: Option<bool>,
            #[rasn(extension_addition)]
            a36: Option<bool>,
            #[rasn(extension_addition)]
            a37: Option<bool>,
            #[rasn(extension_addition)]
            a38: Option<bool>,
            #[rasn(extension_addition)]
            a39: Option<bool>,
            #[rasn(extension_addition)]
            a40: Option<bool>,
            #[rasn(extension_addition)]
            a41: Option<bool>,
            #[rasn(extension_addition)]
            a42: Option<bool>,
            #[rasn(extension_addition)]
            a43: Option<bool>,
            #[rasn(extension_addition)]
            a44: Option<bool>,
            #[rasn(extension_addition)]
            a45: Option<bool>,
            #[rasn(extension_addition)]
            a46: Option<bool>,
            #[rasn(extension_addition)]
            a47: Option<bool>,
            #[rasn(extension_addition)]
            a48: Option<bool>,
            #[rasn(extension_addition)]
            a49: Option<bool>,
            #[rasn(extension_addition)]
            a50: Option<bool>,
            #[rasn(extension_addition)]
            a51: Option<bool>,
            #[rasn(extension_addition)]
            a52: Option<bool>,
            #[rasn(extension_addition)]
            a53: Option<bool>,
            #[rasn(extension_addition)]
            a54: Option<bool>,
            #[rasn(extension_addition)]
            a55: Option<bool>,
            #[rasn(extension_addition)]
            a56: Option<bool>,
            #[rasn(extension_addition)]
            a57: Option<bool>,
            #[rasn(extension_addition)]
            a58: Option<bool>,
            #[rasn(extension_addition)]
            a59: Option<bool>,
            #[rasn(extension_addition)]
            a60: Option<bool>,
            #[rasn(extension_addition)]
            a61: Option<bool>,
            #[rasn(extension_addition)]
            a62: Option<bool>,
            #[rasn(extension_addition)]
            a63: Option<bool>,
            #[rasn(extension_addition)]
            a64: Option<bool>,
            #[rasn(extension_addition)]
            a65: Option<bool>,
        }

        // round_trip!(uper, B, B { a: 0.into() }, &[0]);
        // round_trip!(uper, B, B { a: 1.into() }, &[0x80, 0x80, 0x80]);
        // round_trip!(uper, C, C {a: true}, &[0x40]);
        round_trip!(uper, D, D { a: true, b: None }, &[0x40]);
        round_trip!(uper, I, I { a: true, b: None }, &[0x40]);
        round_trip!(uper, J, J { a: true, b: None }, &[0x40]);
        round_trip!(
            uper,
            K,
            K {
                a: true,
                b: None,
                c: None
            },
            &[0x40]
        );
        round_trip!(uper, L, L { a: true, b: None }, &[0x40]);
        round_trip!(uper, M, M { a: true, b: None }, &[0x40]);
        round_trip!(uper, N, N { a: true }, &[0x00]);
        round_trip!(uper, N, N { a: false }, &[0x80]);
        round_trip!(uper, P, P { a: None }, &[0x00]);
        round_trip!(
            uper,
            G,
            G {
                a: true,
                b: Some(GE { a: true }),
                c: Some(GE { a: true }),
                d: true
            },
            &[0xe0, 0x70, 0x18, 0x00, 0x18, 0x00]
        );
        round_trip!(
            uper,
            M,
            M {
                a: true,
                b: Some(ME {
                    a: Some(MESeq { a: 5.into() }),
                    b: true
                })
            },
            &[0xc0, 0x40, 0xe0, 0x20, 0xb0, 0x00]
        );
        round_trip!(
            uper,
            Q,
            Q {
                a: C { a: true },
                b: 100.into()
            },
            &[0x40, 0x59, 0x00]
        );
        round_trip!(
            uper,
            R,
            R {
                a: D {
                    a: true,
                    b: Some(DE { a: true })
                },
                b: 100.into()
            },
            &[0xc0, 0x40, 0x60, 0x00, 0x59, 0x00]
        );
        round_trip!(
            uper,
            S,
            S {
                a: true,
                b: Some(SSeq {
                    a: true,
                    b: Some(true)
                })
            },
            &[0xc0, 0x40, 0x5c, 0x00]
        );
        round_trip!(
            uper,
            T,
            T {
                a: Some(vec![<_>::default()])
            },
            &[0x80, 0x80]
        );
        round_trip!(
            uper,
            T,
            T {
                a: Some(vec![T { a: Some(vec![]) }])
            },
            &[0x80, 0xc0, 0x00]
        );
        round_trip!(
            uper,
            V,
            V {
                a: Some(false),
                ..<_>::default()
            },
            &[0x82, 0x80, 0x20, 0x00]
        );
        round_trip!(
            uper,
            V,
            V {
                b: Some(false),
                ..<_>::default()
            },
            &[0x82, 0x40, 0x20, 0x00]
        );
        round_trip!(
            uper,
            V,
            V {
                c: Some(false),
                ..<_>::default()
            },
            &[0x82, 0x20, 0x20, 0x00]
        );
        // round_trip!(uper, W, W { a1: Some(true), ..<_>::default() }, &[0xd0, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00]);
    }

    #[test]
    fn extension_additions() {
        #[derive(AsnType, Clone, Copy, Debug, Decode, Default, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Urgency {
            #[default]
            Normal,
            High,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct MySequenceValExtension {
            #[rasn(value("0..254"))]
            alternate_item_code: u8,
            #[rasn(size("3..10"))]
            alternate_item_name: Option<Ia5String>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct MySequenceVal {
            #[rasn(value("0..254"))]
            item_code: u8,
            #[rasn(size("3..10"))]
            item_name: Option<Ia5String>,
            #[rasn(extension_addition, default)]
            urgency: Urgency,
            #[rasn(extension_addition_group)]
            v2: Option<MySequenceValExtension>,
        }

        let value = MySequenceVal {
            item_code: 29,
            item_name: Some(Ia5String::try_from("SHERRY").unwrap()),
            urgency: Urgency::High,
            v2: Some(MySequenceValExtension {
                alternate_item_code: 45,
                alternate_item_name: Some(Ia5String::try_from("PORT").unwrap()),
            }),
        };

        round_trip!(
            uper,
            MySequenceVal,
            value,
            &[
                0xc7, 0x5d, 0x39, 0x11, 0x69, 0x52, 0xb2, 0x07, 0x01, 0x80, 0x05, 0x96, 0x9a, 0x13,
                0xe9, 0x54
            ]
        );
    }
}
