pub use super::oer::*;
use crate::types::Constraints;

/// Attempts to decode `T` from `input` using OER.
pub(crate) fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, de::Error> {
    crate::oer::decode(crate::oer::de::DecoderOptions::default(), input)
}
/// Attempts to encode `value` of type `T` to COER.
pub(crate) fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    crate::oer::encode(crate::oer::enc::EncoderOptions::coer(), value)
}
/// Attempts to decode `T` from `input` using OER with constraints.
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    constraints: Constraints,
    input: &[u8],
) -> Result<T, de::Error> {
    crate::oer::decode_with_constraints(
        crate::oer::de::DecoderOptions::default(),
        constraints,
        input,
    )
}
/// Attempts to encode `value` to COER with constraints.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    crate::oer::encode_with_constraints(crate::oer::enc::EncoderOptions::coer(), constraints, value)
}

#[cfg(test)]
mod tests {
    // use super::*;
    #![allow(clippy::items_after_statements)]
    use crate as rasn;
    use crate::prelude::*;
    use crate::types::constraints::{Bounded, Constraint, Size, Value};
    use crate::types::Integer;
    use bitvec::prelude::*;
    #[test]
    fn bool() {
        round_trip!(coer, bool, true, &[0xff]);
        round_trip!(coer, bool, false, &[0]);
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn integer_no_constraints() {
        // Without constraints, all integers should be encoded as signed, with length determinant,
        // and without padding.
        round_trip!(coer, Integer, 0.into(), &[0x01u8, 0x00]);
        round_trip!(coer, Integer, 1.into(), &[0x01u8, 0x01]);
        round_trip!(coer, Integer, (-1).into(), &[0x01u8, 0xff]);
        round_trip!(coer, Integer, 255.into(), &[0x02u8, 0x00, 0xff]);
        round_trip!(coer, Integer, (-255).into(), &[0x02u8, 0xff, 0x01]);
        round_trip!(coer, Integer, i16::MAX.into(), &[0x02u8, 0x7f, 0xff]);
        round_trip!(coer, Integer, i16::MIN.into(), &[0x02u8, 0x80, 0x00]);
        round_trip!(
            coer,
            Integer,
            (i32::from(i16::MAX) + 1).into(),
            &[0x03u8, 0x00u8, 0x80, 0x00]
        );
        round_trip!(
            coer,
            Integer,
            (i32::from(i16::MIN) - 1).into(),
            &[0x03u8, 0xff, 0x7f, 0xff]
        );
        round_trip!(
            coer,
            Integer,
            i32::MAX.into(),
            &[0x04u8, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            Integer,
            i32::MIN.into(),
            &[0x04u8, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            Integer,
            (i64::from(i32::MAX) + 1).into(),
            &[0x05u8, 0x00, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            Integer,
            (i64::from(i32::MIN) - 1).into(),
            &[0x05u8, 0xff, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            Integer,
            i64::MAX.into(),
            &[0x08u8, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            Integer,
            i64::MIN.into(),
            &[0x08u8, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            Integer,
            (i128::from(i64::MAX) + 1).into(),
            &[0x09u8, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            Integer,
            (i128::from(i64::MIN) - 1).into(),
            &[0x09u8, 0xff, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            Integer,
            i128::MAX.into(),
            &[
                0x10u8, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff
            ]
        );
        round_trip!(
            coer,
            Integer,
            i128::MIN.into(),
            &[
                0x10u8, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]
        );
        round_trip!(
            coer,
            Integer,
            Integer::from(i128::MAX) + 1,
            &[
                0x11u8, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
        round_trip!(
            coer,
            Integer,
            Integer::from(i128::MIN) - 1,
            &[
                0x11u8, 0xff, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff
            ]
        );
    }
    #[test]
    fn test_integer_with_unsigned_constraints() {
        type A = ConstrainedInteger<0, { u8::MAX as i128 }>;
        type B = ConstrainedInteger<0, { u16::MAX as i128 }>;
        type C = ConstrainedInteger<0, { u32::MAX as i128 }>;
        type D = ConstrainedInteger<0, { u64::MAX as i128 }>;
        type E = ConstrainedInteger<0, { i128::MAX }>;
        type F = ConstrainedInteger<2, { u16::MAX as i128 }>;

        round_trip!(coer, A, 0.into(), &[0x00]);
        round_trip!(coer, A, 5.into(), &[0x05]);
        round_trip!(coer, A, 255.into(), &[0xff]);
        // Paddings are expected
        round_trip!(coer, B, 0.into(), &[0x00, 0x00]);
        round_trip!(coer, B, 255.into(), &[0x00, 0xff]);
        round_trip!(coer, C, 0.into(), &[0x00, 0x00, 0x00, 0x00]);
        round_trip!(coer, C, u16::MAX.into(), &[0x00, 0x00, 0xff, 0xff]);
        round_trip!(
            coer,
            D,
            0.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            D,
            u32::MAX.into(),
            &[0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff]
        );
        // Use length determinant when upper range above u64 max
        round_trip!(
            coer,
            E,
            (i128::from(u64::MAX) + 1).into(),
            &[0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(coer, F, 2.into(), &[0x00, 0x02]);
        // Error expected, outside of range constraints
        encode_error!(coer, A, (-1).into());
        encode_error!(coer, B, (-1).into());
        encode_error!(coer, C, (-1).into());
        encode_error!(coer, D, (-1).into());
        encode_error!(coer, E, (-1).into());
        encode_error!(coer, F, (1).into());
        encode_error!(coer, A, (u16::from(u8::MAX) + 1).into());
        encode_error!(coer, B, (u32::from(u16::MAX) + 1).into());
        encode_error!(coer, C, (u64::from(u32::MAX) + 1).into());
        encode_error!(coer, D, (u128::from(u64::MAX) + 1).into());
    }
    #[test]
    fn test_integer_with_signed_constraints() {
        type A = ConstrainedInteger<{ i8::MIN as i128 }, { i8::MAX as i128 }>;
        type B = ConstrainedInteger<{ i16::MIN as i128 }, { i16::MAX as i128 }>;
        type C = ConstrainedInteger<{ i32::MIN as i128 }, { i32::MAX as i128 }>;
        type D = ConstrainedInteger<{ i64::MIN as i128 }, { i64::MAX as i128 }>;
        type E = ConstrainedInteger<-5, 5>;

        round_trip!(coer, A, 0.into(), &[0x00]);
        round_trip!(coer, A, (-1).into(), &[0xff]);
        round_trip!(coer, A, i8::MIN.into(), &[0x80]);
        round_trip!(coer, A, i8::MAX.into(), &[0x7f]);
        // Paddings (0xff as 2's complement) are sometimes expected
        round_trip!(coer, B, 0.into(), &[0x00, 0x00]);
        round_trip!(coer, B, (-1).into(), &[0xff, 0xff]);
        round_trip!(coer, B, i8::MIN.into(), &[0xff, 0x80]);
        round_trip!(coer, B, i8::MAX.into(), &[0x00, 0x7f]);
        round_trip!(coer, B, i16::MIN.into(), &[0x80, 0x00]);
        round_trip!(coer, B, i16::MAX.into(), &[0x7f, 0xff]);

        round_trip!(coer, C, 0.into(), &[0x00, 0x00, 0x00, 0x00]);
        round_trip!(coer, C, (-1).into(), &[0xff, 0xff, 0xff, 0xff]);
        round_trip!(coer, C, i16::MIN.into(), &[0xff, 0xff, 0x80, 0x00]);
        round_trip!(coer, C, i16::MAX.into(), &[0x00, 0x00, 0x7f, 0xff]);
        round_trip!(coer, C, i32::MIN.into(), &[0x80, 0x00, 0x00, 0x00]);
        round_trip!(coer, C, i32::MAX.into(), &[0x7f, 0xff, 0xff, 0xff]);

        round_trip!(
            coer,
            D,
            0.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            D,
            (-1).into(),
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            D,
            i32::MIN.into(),
            &[0xff, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            D,
            i32::MAX.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            coer,
            D,
            i64::MIN.into(),
            &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            coer,
            D,
            i64::MAX.into(),
            &[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(coer, E, 4.into(), &[0x04]);
        round_trip!(coer, E, (-4).into(), &[0xfc]);

        // Error expected, outside of range constraints
        encode_error!(coer, A, (i16::from(i8::MIN) - 1).into());
        encode_error!(coer, B, (i32::from(i16::MIN) - 1).into());
        encode_error!(coer, C, (i64::from(i32::MIN) - 1).into());
        encode_error!(coer, D, (i128::from(i64::MIN) - 1).into());

        encode_error!(coer, A, (i16::from(i8::MAX) + 1).into());
        encode_error!(coer, B, (i32::from(i16::MAX) + 1).into());
        encode_error!(coer, C, (i64::from(i32::MAX) + 1).into());
        encode_error!(coer, D, (i128::from(i64::MAX) + 1).into());
    }
    #[test]
    fn test_integer_single_constraint() {
        round_trip_with_constraints!(
            coer,
            Integer,
            Constraints::new(&[Constraint::Value(Value::new(Bounded::Single(5)).into())]),
            5.into(),
            &[0x05]
        );
    }
    #[test]
    fn test_enumerated() {
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Enum1 {
            Green,
            Red,
            Blue,
        }
        round_trip!(coer, Enum1, Enum1::Green, &[0x00]);
        round_trip!(coer, Enum1, Enum1::Red, &[0x01]);
        round_trip!(coer, Enum1, Enum1::Blue, &[0x02]);
        // TODO, check correctness https://github.com/XAMPPRocky/rasn/discussions/124#discussioncomment-6724973
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[allow(clippy::items_after_statements)]
        enum Enum2 {
            Red,
            Blue,
            Green,
            #[rasn(extension_addition_group)]
            Yellow,
            Purple,
        }
        round_trip!(coer, Enum2, Enum2::Red, &[0x00]);
        round_trip!(coer, Enum2, Enum2::Yellow, &[0x03]);
        round_trip!(coer, Enum2, Enum2::Purple, &[0x04]);
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[allow(clippy::items_after_statements)]
        enum Enum3 {
            Red = 5,
            Blue = 6,
            Green = 7,
        }
        round_trip!(coer, Enum3, Enum3::Red, &[0x05]);
        round_trip!(coer, Enum3, Enum3::Blue, &[0x06]);
        round_trip!(coer, Enum3, Enum3::Green, &[0x07]);

        // TODO negative values are not supported at the moment...
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[allow(clippy::items_after_statements)]
        enum Enum4 {
            Yes = 1000,
            No = (-1000),
        }
        round_trip!(coer, Enum4, Enum4::Yes, &[0x82, 0x03, 0xe8]);
        // round_trip!(coer, Enum4, Enum4::No, &[0x82, 0xfc, 0x18]);
    }
    #[test]
    fn test_bit_string() {
        round_trip!(
            coer,
            BitString,
            BitString::from_slice(&[0x01]),
            &[0x02, 0x00, 0x01]
        );
        let mut bv = bitvec![u8, Msb0;];
        bv.extend_from_raw_slice(&[0xff]);
        bv.push(false);
        bv.push(true);
        bv.extend([false; 4].iter());
        // bv should be 14 bits now
        round_trip_with_constraints!(
            coer,
            BitString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(14)).into())]),
            BitString::from_bitslice(&bv),
            &[0b1111_1111, 0b0100_0000]
        );
        round_trip!(
            coer,
            BitString,
            BitString::from_bitslice(&bv),
            &[0x03u8, 0x02, 0b1111_1111, 0b0100_0000]
        );
        encode_error_with_constraints!(
            coer,
            BitString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(15)).into())]),
            BitString::from_bitslice(&bv)
        );
    }
    #[test]
    fn test_octet_string() {
        round_trip!(
            coer,
            OctetString,
            OctetString::from_static(&[0x01]),
            &[0x01, 0x01]
        );
        round_trip_with_constraints!(
            coer,
            OctetString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(5)).into())]),
            OctetString::from_static(&[0x01u8, 0x02, 0x03, 0x04, 0x05]),
            &[0x01u8, 0x02, 0x03, 0x04, 0x05]
        );
        round_trip_with_constraints!(
            coer,
            OctetString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(6)
                })
                .into()
            )]),
            OctetString::from_static(&[0x01u8, 0x02, 0x03, 0x04, 0x05]),
            &[0x05u8, 0x01, 0x02, 0x03, 0x04, 0x05]
        );
        encode_error_with_constraints!(
            coer,
            OctetString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(5)).into())]),
            OctetString::from_static(&[0x01u8, 0x02, 0x03, 0x04])
        );
        encode_error_with_constraints!(
            coer,
            OctetString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(6)
                })
                .into()
            )]),
            OctetString::from_static(&[0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
        );
        encode_error_with_constraints!(
            coer,
            OctetString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(6)
                })
                .into()
            )]),
            OctetString::from_static(&[0x01u8, 0x02])
        );
    }
    #[test]
    fn test_object_identifier() {
        // ('A',                   '1.2', b'\x01\x2a'),
        // ('A',              '1.2.3321', b'\x03\x2a\x99\x79')
        round_trip!(
            coer,
            ObjectIdentifier,
            ObjectIdentifier::new(vec![1u32, 2]).unwrap(),
            &[0x01u8, 0x2a]
        );
        round_trip!(
            coer,
            ObjectIdentifier,
            ObjectIdentifier::new(vec![1, 2, 3321]).unwrap(),
            &[0x03u8, 0x2a, 0x99, 0x79]
        );
    }
    #[test]
    fn test_choice() {
        use crate as rasn;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum Choice {
            Normal(Integer),
            High(Integer),
            #[rasn(extension_addition)]
            Medium(Integer),
        }
        round_trip!(
            coer,
            Choice,
            Choice::Normal(333.into()),
            &[0x80, 0x02, 0x01, 0x4d]
        );
        round_trip!(
            coer,
            Choice,
            Choice::High(333.into()),
            &[0x81, 0x02, 0x01, 0x4d]
        );
        round_trip!(
            coer,
            Choice,
            Choice::Medium(333.into()),
            &[0x82, 0x03, 0x02, 0x01, 0x4d]
        );

        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum BoolChoice {
            A(bool),
            #[rasn(extension_addition)]
            B(bool),
            C(Choice),
        }
        round_trip!(coer, BoolChoice, BoolChoice::A(true), &[0x80, 0xff]);
        round_trip!(coer, BoolChoice, BoolChoice::B(true), &[0x81, 0x01, 0xff]);
        round_trip!(
            coer,
            BoolChoice,
            BoolChoice::C(Choice::Normal(333.into())),
            &[0x82, 0x80, 0x02, 0x01, 0x4d]
        );
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum TripleChoice {
            A(bool),
            B(BoolChoice),
        }
        round_trip!(coer, TripleChoice, TripleChoice::A(true), &[0x80, 0xff]);
        round_trip!(
            coer,
            TripleChoice,
            TripleChoice::B(BoolChoice::C(Choice::Normal(333.into()))),
            &[0x81, 0x82, 0x80, 0x02, 0x01, 0x4d]
        );
    }
    #[test]
    fn test_numeric_string() {
        round_trip!(
            coer,
            NumericString,
            "123".try_into().unwrap(),
            &[0x03, 0x31, 0x32, 0x33]
        );
        round_trip_with_constraints!(
            coer,
            NumericString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(3)).into())]),
            "123".try_into().unwrap(),
            &[0x31, 0x32, 0x33]
        );
        round_trip_with_constraints!(
            coer,
            NumericString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(7)
                })
                .into()
            )]),
            "123".try_into().unwrap(),
            &[0x03, 0x31, 0x32, 0x33]
        );
    }
    #[test]
    fn test_printable_string() {
        round_trip!(
            coer,
            PrintableString,
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
        round_trip!(
            coer,
            PrintableString,
            " '()+,-./:=?".try_into().unwrap(),
            &[0x0c, 0x20, 0x27, 0x28, 0x29, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x3a, 0x3d, 0x3f]
        );
        round_trip_with_constraints!(
            coer,
            PrintableString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(3)).into())]),
            "foo".try_into().unwrap(),
            &[0x66, 0x6f, 0x6f]
        );
        round_trip_with_constraints!(
            coer,
            PrintableString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(7)
                })
                .into()
            )]),
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
    }
    #[test]
    fn test_visible_string() {
        round_trip!(
            coer,
            VisibleString,
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
        round_trip_with_constraints!(
            coer,
            VisibleString,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(3)).into())]),
            "foo".try_into().unwrap(),
            &[0x66, 0x6f, 0x6f]
        );
        round_trip_with_constraints!(
            coer,
            VisibleString,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(7)
                })
                .into()
            )]),
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
    }
    #[test]
    fn test_ia5_string() {
        round_trip!(
            coer,
            Ia5String,
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
        round_trip_with_constraints!(
            coer,
            Ia5String,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(3)).into())]),
            "foo".try_into().unwrap(),
            &[0x66, 0x6f, 0x6f]
        );
        round_trip_with_constraints!(
            coer,
            Ia5String,
            Constraints::new(&[Constraint::Size(
                Size::new(Bounded::Range {
                    start: Some(3),
                    end: Some(7)
                })
                .into()
            )]),
            "foo".try_into().unwrap(),
            &[0x03, 0x66, 0x6f, 0x6f]
        );
    }
    #[test]
    fn test_general_string() {
        round_trip!(
            coer,
            GeneralString,
            GeneralString::from_bytes("".as_bytes()).unwrap(),
            &[0x00]
        );
        round_trip!(
            coer,
            GeneralString,
            GeneralString::from_bytes("2".as_bytes()).unwrap(),
            &[0x01, 0x32]
        );
    }
    #[test]
    fn test_utf8_string() {
        round_trip!(coer, Utf8String, "".try_into().unwrap(), &[0x00]);
        round_trip!(coer, Utf8String, "2".try_into().unwrap(), &[0x01, 0x32]);
        round_trip!(
            coer,
            Utf8String,
            "2".repeat(128).try_into().unwrap(),
            &[0x81, 0x80]
                .iter()
                .chain("2".repeat(128).as_bytes().iter())
                .copied()
                .collect::<Vec<_>>()
        );
        round_trip!(
            coer,
            Utf8String,
            "ÄÖÄÖÄÖÄÖ12e4Ä".try_into().unwrap(),
            &[
                0x16, 0xc3, 0x84, 0xc3, 0x96, 0xc3, 0x84, 0xc3, 0x96, 0xc3, 0x84, 0xc3, 0x96, 0xc3,
                0x84, 0xc3, 0x96, 0x31, 0x32, 0x65, 0x34, 0xc3, 0x84
            ]
        );
        round_trip_with_constraints!(
            coer,
            Utf8String,
            Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(3)).into())]),
            "foo".try_into().unwrap(),
            &[0x66, 0x6f, 0x6f]
        );
    }
    #[test]
    fn test_teletext_string() {
        round_trip!(
            coer,
            TeletexString,
            TeletexString::from("123".as_bytes().to_vec()),
            &[0x03, 0x31, 0x32, 0x33]
        );
    }
    #[test]
    fn test_generalized_time() {
        use chrono::NaiveDate;
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let dt = NaiveDate::from_ymd_opt(2080, 10, 9)
            .unwrap()
            .and_hms_micro_opt(13, 0, 5, 342_000)
            .unwrap()
            .and_local_timezone(offset);
        round_trip!(
            coer,
            GeneralizedTime,
            GeneralizedTime::from(dt.unwrap(),),
            &[
                0x13, 0x32, 0x30, 0x38, 0x30, 0x31, 0x30, 0x30, 0x39, 0x31, 0x33, 0x30, 0x30, 0x30,
                0x35, 0x2e, 0x33, 0x34, 0x32, 0x5a
            ]
        );

        let data = [
            24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90,
        ];

        assert!(crate::der::decode::<crate::types::Open>(&data).is_err());
    }
    #[test]
    fn test_utc_time() {
        // 2019-10-09 13:00:05 UTC
        // 191009130005Z
        round_trip!(
            coer,
            UtcTime,
            UtcTime::from(
                chrono::NaiveDate::from_ymd_opt(2019, 10, 9)
                    .unwrap()
                    .and_hms_opt(13, 0, 5)
                    .unwrap()
                    .and_utc()
            ),
            &[0x0d, 0x31, 0x39, 0x31, 0x30, 0x30, 0x39, 0x31, 0x33, 0x30, 0x30, 0x30, 0x35, 0x5a]
        );
    }
    #[test]
    /// No extension addition presence bitmap in any test case or preamble
    /// Or option or defaults
    fn test_sequence_no_extensions() {
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        // #[rasn(automatic_tags)]
        #[rasn(crate_root = "crate")]
        struct Sequence1 {
            a: Integer,
            b: Integer,
        }
        round_trip!(
            coer,
            Sequence1,
            Sequence1 {
                a: 1.into(),
                b: 2.into()
            },
            &[0x01, 0x01, 0x01, 0x02]
        );

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Sequence2 {
            a: bool,
        }
        round_trip!(coer, Sequence2, Sequence2 { a: true }, &[0xff]);

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Sequence3 {
            a: bool,
            b: Sequence1,
        }
        round_trip!(
            coer,
            Sequence3,
            Sequence3 {
                a: true,
                b: Sequence1 {
                    a: 1.into(),
                    b: 2.into()
                }
            },
            &[0xff, 0x01, 0x01, 0x01, 0x02]
        );
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", choice, automatic_tags)]
        enum Choice1 {
            A(bool),
            B(Sequence1),
        }
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Sequence4 {
            a: Integer,
            b: Choice1,
        }
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Sequence5 {
            a: bool,
            b: Sequence4,
        }
        round_trip!(
            coer,
            Sequence5,
            Sequence5 {
                a: true,
                b: Sequence4 {
                    a: 1.into(),
                    b: Choice1::B(Sequence1 {
                        a: 1.into(),
                        b: 2.into()
                    })
                }
            },
            &[0xff, 0x01, 0x01, 0x81, 0x01, 0x01, 0x01, 0x02]
        );
    }
    #[test]
    fn test_sequence_default_option() {
        fn default_a() -> Integer {
            0.into()
        }
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Sequence1 {
            #[rasn(default = "default_a")]
            a: Integer,
        }
        round_trip!(coer, Sequence1, Sequence1 { a: 0.into() }, &[0x00]);
        round_trip!(
            coer,
            Sequence1,
            Sequence1 { a: 1.into() },
            &[0b1000_0000, 0x01, 0x01]
        );
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Sequence2 {
            a: Integer,
            b: Option<Integer>,
        }
        round_trip!(
            coer,
            Sequence2,
            Sequence2 {
                a: 1.into(),
                b: Some(2.into())
            },
            &[0b1000_0000, 0x01, 0x01, 0x01, 0x02]
        );
        round_trip!(
            coer,
            Sequence2,
            Sequence2 {
                a: 1.into(),
                b: None
            },
            &[0x00, 0x01, 0x01]
        );
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        struct Sequence4 {
            #[rasn(default = "default_a")]
            a: Integer, // default is 0
            b: Option<Integer>,
        }
        round_trip!(
            coer,
            Sequence4,
            Sequence4 {
                a: 0.into(),
                b: None
            },
            &[0x00]
        );
        round_trip!(
            coer,
            Sequence4,
            Sequence4 {
                a: 1.into(),
                b: Some(3.into())
            },
            &[0b1100_0000, 0x01, 0x01, 0x01, 0x03]
        );
    }
}
