//! # Unaligned Packed Encoding Rules
//!
//! Codec functions for UPER, rasn provides a "basic" decoder, and canonical encoder.
//! This means that users are able decode any valid UPER value, and that rasn's
//! encoding will always produce the same output for the same value.

use crate::types::Constraints;

pub use super::per::*;

/// Attempts to decode `T` from `input` using UPER-BASIC.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    crate::per::decode(de::DecoderOptions::unaligned(), input)
}

/// Attempts to decode `T` from `input` using UPER-BASIC. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid UPER-BASIC encoding specific to the expected type.
pub fn decode_with_remainder<T: crate::Decode>(
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    crate::per::decode_with_remainder(de::DecoderOptions::unaligned(), input)
}

/// Attempts to encode `value` to UPER-CANONICAL.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    crate::per::encode(enc::EncoderOptions::unaligned(), value)
}

/// Attempts to decode `T` from `input` using UPER-BASIC.
pub fn decode_with_constraints<T: crate::Decode>(
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    crate::per::decode_with_constraints(de::DecoderOptions::unaligned(), constraints, input)
}

/// Attempts to encode `value` to UPER-CANONICAL.
pub fn encode_with_constraints<T: crate::Encode>(
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    crate::per::encode_with_constraints(enc::EncoderOptions::unaligned(), constraints, value)
}

#[cfg(test)]
mod tests {
    use crate::{
        macros::{constraints, permitted_alphabet_constraint, size_constraint},
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

        round_trip!(uper, B, B::new(5), &[0x00]);
        round_trip!(uper, B, B::new(6), &[0x02]);
        round_trip!(uper, B, B::new(99), &[0xbc]);
        round_trip!(uper, C, C::new(-10), &[0x00]);
        round_trip!(uper, C, C::new(-1), &[0x48]);
        round_trip!(uper, C, C::new(0), &[0x50]);
        round_trip!(uper, C, C::new(1), &[0x58]);
        round_trip!(uper, C, C::new(10), &[0xa0]);
        // round_trip!(uper, D, 99, &[0x5e]);
        round_trip!(uper, E, E::new(1000), &[]);
    }

    #[test]
    fn sequence_of() {
        round_trip!(uper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
        round_trip!(aper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn numeric_string() {
        round_trip!(
            uper,
            NumericString,
            " 0123456789".try_into().unwrap(),
            &[0x0b, 0x01, 0x23, 0x45, 0x67, 0x89, 0xa0]
        );
        const CONSTRAINT_1: Constraints = constraints!(size_constraint!(5));
        round_trip_with_constraints!(
            uper,
            NumericString,
            CONSTRAINT_1,
            "1 9 5".try_into().unwrap(),
            &[0x20, 0xa0, 0x60]
        );

        const CONSTRAINT_2: Constraints = constraints!(size_constraint!(19, 134));
        round_trip_with_constraints!(
            uper,
            NumericString,
            CONSTRAINT_2,
            "0123456789 9876543210".try_into().unwrap(),
            &[0x04, 0x24, 0x68, 0xac, 0xf1, 0x34, 0x15, 0x30, 0xec, 0xa8, 0x64, 0x20]
        );
        const CONSTRAINT_3: Constraints = constraints!(permitted_alphabet_constraint!(&[
            b'0' as u32,
            b'1' as u32,
            b'2' as u32,
            b'3' as u32,
            b'4' as u32,
            b'5' as u32
        ]));

        round_trip_with_constraints!(
            uper,
            NumericString,
            CONSTRAINT_3,
            "5".try_into().unwrap(),
            &[0x01, 0xa0]
        );
    }

    #[test]
    fn visible_string() {
        const CONSTRAINT_1: Constraints = constraints!(size_constraint!(19, 133));
        round_trip_with_constraints!(
            uper,
            VisibleString,
            CONSTRAINT_1,
            "HejHoppHappHippAbcde".try_into().unwrap(),
            &[
                0x03, 0x23, 0x2e, 0xa9, 0x1b, 0xf8, 0x70, 0x91, 0x87, 0x87, 0x09, 0x1a, 0x78, 0x70,
                0x83, 0x8b, 0x1e, 0x4c, 0xa0
            ]
        );
        const CONSTRAINT_2: Constraints = constraints!(size_constraint!(5));
        round_trip_with_constraints!(
            uper,
            VisibleString,
            CONSTRAINT_2,
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
        const CONSTRAINT_3: Constraints = constraints!(
            size_constraint!(1, 255),
            permitted_alphabet_constraint!(ALPHABET)
        );
        round_trip_with_constraints!(
            uper,
            VisibleString,
            CONSTRAINT_3,
            "hej".try_into().unwrap(),
            &[0x02, 0x39, 0x12]
        );
    }
    #[test]
    fn printable_string() {
        const CONSTRAINT_1: Constraints = constraints!(size_constraint!(16));
        round_trip_with_constraints!(
            uper,
            PrintableString,
            CONSTRAINT_1,
            PrintableString::from_bytes("0123456789abcdef".as_bytes()).unwrap(),
            &[0x60, 0xc5, 0x93, 0x36, 0x8d, 0x5b, 0x37, 0x70, 0xe7, 0x0e, 0x2c, 0x79, 0x32, 0xe6]
        );
        const CONSTRAINT_2: Constraints = constraints!(size_constraint!(0, 31));
        round_trip_with_constraints!(
            uper,
            PrintableString,
            CONSTRAINT_2,
            "".try_into().unwrap(),
            &[0x00]
        );
        const CONSTRAINT_3: Constraints = constraints!(size_constraint!(0, 31));
        round_trip_with_constraints!(
            uper,
            PrintableString,
            CONSTRAINT_3,
            "2".try_into().unwrap(),
            &[0x0b, 0x20]
        );

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct PrintStruct {
            a: bool,
            #[rasn(size("36"))]
            b: PrintableString,
            c: bool,
        }
        round_trip!(
            uper,
            PrintStruct,
            PrintStruct {
                a: true,
                b: "123123123123123123123123123123123123".try_into().unwrap(),
                c: true
            },
            &[
                0xb1, 0x64, 0xcd, 0x8b, 0x26, 0x6c, 0x59, 0x33, 0x62, 0xc9, 0x9b, 0x16, 0x4c, 0xd8,
                0xb2, 0x66, 0xc5, 0x93, 0x36, 0x2c, 0x99, 0xb1, 0x64, 0xcd, 0x8b, 0x26, 0x6c, 0x59,
                0x33, 0x62, 0xc9, 0x9c
            ]
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

    #[test]
    fn constrained_extension_addition() {
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct TestSequence {
            #[rasn(size("0..=8"))]
            hello: OctetString,
            #[rasn(
                extension_addition,
                value("0..=9"),
                default = "test_sequence_world_default"
            )]
            world: u8,
        }

        fn test_sequence_world_default() -> u8 {
            8
        }

        let ext_value = TestSequence {
            hello: vec![1, 2, 3, 4].into(),
            world: 4,
        };

        round_trip!(
            uper,
            TestSequence,
            ext_value,
            &[0xA0, 0x08, 0x10, 0x18, 0x20, 0x08, 0x0A, 0x00]
        );
    }

    #[test]
    fn recursive_types() {
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum TestChoice {
            Number1(()),
            Number2(bool),
            Number3(Box<TopLevel>),
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[rasn(automatic_tags)]
        struct TopLevel {
            #[rasn(value("1..=8"))]
            pub test: u8,
            pub choice: TestChoice,
        }

        impl TopLevel {
            pub fn new(test: u8, choice: TestChoice) -> Self {
                Self { test, choice }
            }
        }

        let test_value = TopLevel::new(
            1,
            TestChoice::Number3(Box::new(TopLevel {
                test: 2,
                choice: TestChoice::Number1(()),
            })),
        );
        round_trip!(uper, TopLevel, test_value, &[8, 128]);
    }
    #[test]
    fn deeply_nested_choice() {
        use crate as rasn;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        enum Choice {
            Normal(Integer),
            High(Integer),
            Medium(Integer),
        }
        round_trip!(
            uper,
            Choice,
            Choice::Medium(333.into()),
            &[128, 128, 83, 64]
        );
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        enum BoolChoice {
            A(bool),
            B(bool),
            C(Choice),
        }
        round_trip!(
            uper,
            BoolChoice,
            BoolChoice::C(Choice::Normal(333.into())),
            &[128, 32, 20, 208]
        );

        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        enum TripleChoice {
            A(bool),
            B(BoolChoice),
        }
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        enum FourthChoice {
            A(TripleChoice),
            B(bool),
        }
        round_trip!(
            uper,
            TripleChoice,
            TripleChoice::B(BoolChoice::C(Choice::Normal(333.into()))),
            &[192, 16, 10, 104]
        );
        round_trip!(
            uper,
            FourthChoice,
            FourthChoice::A(TripleChoice::B(BoolChoice::C(Choice::Normal(333.into())))),
            &[96, 8, 5, 52]
        );
    }
    #[test]
    fn test_object_identifier() {
        round_trip!(
            uper,
            ObjectIdentifier,
            ObjectIdentifier::new(vec![1, 2]).unwrap(),
            &[0x01u8, 0x2a]
        );
        round_trip!(
            uper,
            ObjectIdentifier,
            ObjectIdentifier::new(vec![1, 2, 3321]).unwrap(),
            &[0x03u8, 0x2a, 0x99, 0x79]
        );
        #[derive(AsnType, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct B {
            a: bool,
            b: ObjectIdentifier,
        }
        round_trip!(
            uper,
            B,
            B {
                a: true,
                b: ObjectIdentifier::new(vec![1, 2]).unwrap()
            },
            &[0x80, 0x95, 0x00]
        );
    }
    #[test]
    fn test_null_in_extended_option() {
        use crate as rasn;
        #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Omitted {
            pub a: Option<OctetString>,
            #[rasn(extension_addition)]
            pub omitted: Option<()>,
        }
        round_trip!(
            uper,
            Omitted,
            Omitted {
                a: Some(OctetString::from_static(&[0x00, 0x01, 0x02])),
                omitted: Some(())
            },
            &[192, 192, 0, 64, 128, 64, 0]
        );
    }
    #[test]
    // https://github.com/librasn/rasn/issues/271
    fn test_untagged_duplicate_type_option_on_sequence() {
        use crate as rasn;
        #[derive(AsnType, Decode, Encode, Clone, Debug, PartialEq, Eq)]
        pub struct SequenceOptionals {
            pub it: Integer,
            pub is: Option<OctetString>,
            pub late: Option<Integer>,
        }
        round_trip!(
            uper,
            SequenceOptionals,
            SequenceOptionals {
                it: 1.into(),
                is: Some(OctetString::from_static(&[0x01, 0x02, 0x03])),
                late: None
            },
            &[0b10000000, 0x40, 0x40, 0xc0, 0x40, 0x80, 0xc0]
        );
        #[derive(AsnType, Decode, Encode, Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub struct SequenceDuplicatesExtended {
            pub it: Integer,
            pub is: Option<OctetString>,
            pub late: Option<Integer>,
            #[rasn(extension_addition)]
            pub today: OctetString,
        }
        round_trip!(
            uper,
            SequenceDuplicatesExtended,
            SequenceDuplicatesExtended {
                it: 1.into(),
                is: Some(OctetString::from_static(&[0x01, 0x02, 0x03])),
                late: None,
                today: OctetString::from_static(&[0x01, 0x02, 0x03])
            },
            &[0b11000000, 0x20, 0x20, 0x60, 0x20, 0x40, 0x60, 0x20, 0x80, 0x60, 0x20, 0x40, 0x60]
        );
    }

    /// Tests that unaligned OctetStrings are encoded and decoded correctly (UPER).
    #[test]
    fn test_unaligned_sequence_with_octet_string() {
        use crate as rasn;
        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Unaligned {
            #[rasn(value("0..=7"))]
            pub offset_bits: u8,
            #[rasn(size("0..=255"))]
            pub the_string: OctetString,
        }
        /// Describes the encodings of a given string (first array in a tuple) into its
        /// UPER representation (second array in a tuple).
        const UPER_UNALIGNED_CASES: &[(&[u8], &[u8])] = &[
            (&[], &[0xe0, 0x00]), // The minimum encoding contains 3 + 8 bits.
            (&[0x00; 1], &[0xe0, 0x20, 0x00]),
            (&[0xF0; 1], &[0xe0, 0x3e, 0x00]),
            (&[0xFF; 1], &[0xe0, 0x3f, 0xe0]),
            (&[0x00; 4], &[0xe0, 0x80, 0x00, 0x00, 0x00, 0x00]),
            (&[0xFF; 4], &[0xe0, 0x9f, 0xff, 0xff, 0xff, 0xe0]),
            (
                &[0x00; 10],
                &[
                    0xe1, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                &[0xFF; 10],
                &[
                    0xe1, 0x5f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe0,
                ],
            ),
            (
                &[0x00; 100],
                &[
                    0xec, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                &[0x00; 127],
                &[
                    0xef, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                &[0x00; 128],
                &[
                    0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                &[0x00; 200],
                &[
                    0xf9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
            (
                &[0x00; 255],
                &[
                    0xff, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
        ];
        for (case, expected) in UPER_UNALIGNED_CASES {
            round_trip!(
                uper,
                Unaligned,
                Unaligned {
                    offset_bits: 7,
                    the_string: OctetString::from_static(case)
                },
                expected
            );
        }
    }

    #[test]
    fn test_encoding_of_zero_size_octet_string() {
        use crate as rasn;

        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Unaligned {
            #[rasn(value("0..=7"))]
            pub offset_bits: u8,
            #[rasn(size("0..=255"))]
            pub the_string: OctetString,
        }

        round_trip!(
            uper,
            Unaligned,
            Unaligned {
                offset_bits: 7,
                the_string: OctetString::from_static(&[])
            },
            &[0b11100000, 0b00000000]
        );

        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct UnalignedZeroLength {
            #[rasn(value("0..=7"))]
            pub offset_bits: u8,
            #[rasn(size("0"))]
            pub the_string: OctetString,
        }

        round_trip!(
            uper,
            UnalignedZeroLength,
            UnalignedZeroLength {
                offset_bits: 7,
                the_string: OctetString::from_static(&[])
            },
            &[0b11100000]
        );

        #[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct AlignedZeroLength {
            #[rasn(size("0"))]
            pub the_string: OctetString,
        }

        round_trip!(
            uper,
            AlignedZeroLength,
            AlignedZeroLength {
                the_string: OctetString::from_static(&[])
            },
            &[]
        );
    }
}
