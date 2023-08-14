pub mod de;
pub mod enc;
mod helpers;

pub use self::{de::Decoder, enc::Encoder};
use crate::types::Constraints;
/// Attempts to decode `T` from `input` using OER.
pub(crate) fn decode<T: crate::Decode>(
    // options: de::DecoderOptions,
    input: &[u8],
) -> Result<T, de::Error> {
    T::decode(&mut Decoder::new(crate::types::BitStr::from_slice(input)))
}
/// Attempts to encode `value` of type `T` to COER.
pub(crate) fn encode<T: crate::Encode>(
    // _: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    let mut enc = Encoder::new();
    value.encode(&mut enc)?;
    Ok(enc.output())
}
/// Attempts to decode `T` from `input` using OER with constraints.
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    // options: de::DecoderOptions,
    constraints: Constraints,
    input: &[u8],
) -> Result<T, de::Error> {
    T::decode_with_constraints(
        &mut Decoder::new(crate::types::BitStr::from_slice(input)),
        constraints,
    )
}
/// Attempts to encode `value` to COER with constraints.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    // options: enc::EncoderOptions,
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    let mut enc = Encoder::new();
    value.encode_with_constraints(&mut enc, constraints)?;
    Ok(enc.output())
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::prelude::*;
    use crate::types::constraints::{Bounded, Constraint, Value};
    use crate::types::Integer;
    #[test]
    fn bool() {
        round_trip!(oer, bool, true, &[0xff]);
        round_trip!(oer, bool, false, &[0]);
    }
    #[test]
    #[allow(clippy::too_many_lines)]
    fn integer_no_constraints() {
        // Without constraints, all integers should be encoded as signed, with length determinant,
        // and without padding.
        round_trip!(oer, Integer, 0.into(), &[0x01u8, 0x00]);
        round_trip!(oer, Integer, 1.into(), &[0x01u8, 0x01]);
        round_trip!(oer, Integer, (-1).into(), &[0x01u8, 0xff]);
        round_trip!(oer, Integer, 255.into(), &[0x02u8, 0x00, 0xff]);
        round_trip!(oer, Integer, (-255).into(), &[0x02u8, 0xff, 0x01]);
        round_trip!(oer, Integer, i16::MAX.into(), &[0x02u8, 0x7f, 0xff]);
        round_trip!(oer, Integer, i16::MIN.into(), &[0x02u8, 0x80, 0x00]);
        round_trip!(
            oer,
            Integer,
            (i32::from(i16::MAX) + 1).into(),
            &[0x03u8, 0x00u8, 0x80, 0x00]
        );
        round_trip!(
            oer,
            Integer,
            (i32::from(i16::MIN) - 1).into(),
            &[0x03u8, 0xff, 0x7f, 0xff]
        );
        round_trip!(
            oer,
            Integer,
            i32::MAX.into(),
            &[0x04u8, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            Integer,
            i32::MIN.into(),
            &[0x04u8, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            Integer,
            (i64::from(i32::MAX) + 1).into(),
            &[0x05u8, 0x00, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            Integer,
            (i64::from(i32::MIN) - 1).into(),
            &[0x05u8, 0xff, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            Integer,
            i64::MAX.into(),
            &[0x08u8, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            Integer,
            i64::MIN.into(),
            &[0x08u8, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            Integer,
            (i128::from(i64::MAX) + 1).into(),
            &[0x09u8, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            Integer,
            (i128::from(i64::MIN) - 1).into(),
            &[0x09u8, 0xff, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            Integer,
            i128::MAX.into(),
            &[
                0x10u8, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff
            ]
        );
        round_trip!(
            oer,
            Integer,
            i128::MIN.into(),
            &[
                0x10u8, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]
        );
        round_trip!(
            oer,
            Integer,
            Integer::from(i128::MAX) + 1,
            &[
                0x11u8, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
        round_trip!(
            oer,
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

        round_trip!(oer, A, 0.into(), &[0x00]);
        round_trip!(oer, A, 5.into(), &[0x05]);
        round_trip!(oer, A, 255.into(), &[0xff]);
        // Paddings are expected
        round_trip!(oer, B, 0.into(), &[0x00, 0x00]);
        round_trip!(oer, B, 255.into(), &[0x00, 0xff]);
        round_trip!(oer, C, 0.into(), &[0x00, 0x00, 0x00, 0x00]);
        round_trip!(oer, C, u16::MAX.into(), &[0x00, 0x00, 0xff, 0xff]);
        round_trip!(
            oer,
            D,
            0.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            D,
            u32::MAX.into(),
            &[0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff]
        );
        // Use length determinant when upper range above u64 max
        round_trip!(
            oer,
            E,
            (i128::from(u64::MAX) + 1).into(),
            &[0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(oer, F, 2.into(), &[0x00, 0x02]);
        // Error expected, outside of range constraints
        encode_error!(oer, A, (-1).into());
        encode_error!(oer, B, (-1).into());
        encode_error!(oer, C, (-1).into());
        encode_error!(oer, D, (-1).into());
        encode_error!(oer, E, (-1).into());
        encode_error!(oer, F, (1).into());
        encode_error!(oer, A, (u16::from(u8::MAX) + 1).into());
        encode_error!(oer, B, (u32::from(u16::MAX) + 1).into());
        encode_error!(oer, C, (u64::from(u32::MAX) + 1).into());
        encode_error!(oer, D, (u128::from(u64::MAX) + 1).into());
    }
    #[test]
    fn test_integer_with_signed_constraints() {
        type A = ConstrainedInteger<{ i8::MIN as i128 }, { i8::MAX as i128 }>;
        type B = ConstrainedInteger<{ i16::MIN as i128 }, { i16::MAX as i128 }>;
        type C = ConstrainedInteger<{ i32::MIN as i128 }, { i32::MAX as i128 }>;
        type D = ConstrainedInteger<{ i64::MIN as i128 }, { i64::MAX as i128 }>;
        type E = ConstrainedInteger<-5, 5>;

        round_trip!(oer, A, 0.into(), &[0x00]);
        round_trip!(oer, A, (-1).into(), &[0xff]);
        round_trip!(oer, A, i8::MIN.into(), &[0x80]);
        round_trip!(oer, A, i8::MAX.into(), &[0x7f]);
        // Paddings (0xff as 2's complement) are sometimes expected
        round_trip!(oer, B, 0.into(), &[0x00, 0x00]);
        round_trip!(oer, B, (-1).into(), &[0xff, 0xff]);
        round_trip!(oer, B, i8::MIN.into(), &[0xff, 0x80]);
        round_trip!(oer, B, i8::MAX.into(), &[0x00, 0x7f]);
        round_trip!(oer, B, i16::MIN.into(), &[0x80, 0x00]);
        round_trip!(oer, B, i16::MAX.into(), &[0x7f, 0xff]);

        round_trip!(oer, C, 0.into(), &[0x00, 0x00, 0x00, 0x00]);
        round_trip!(oer, C, (-1).into(), &[0xff, 0xff, 0xff, 0xff]);
        round_trip!(oer, C, i16::MIN.into(), &[0xff, 0xff, 0x80, 0x00]);
        round_trip!(oer, C, i16::MAX.into(), &[0x00, 0x00, 0x7f, 0xff]);
        round_trip!(oer, C, i32::MIN.into(), &[0x80, 0x00, 0x00, 0x00]);
        round_trip!(oer, C, i32::MAX.into(), &[0x7f, 0xff, 0xff, 0xff]);

        round_trip!(
            oer,
            D,
            0.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            D,
            (-1).into(),
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            D,
            i32::MIN.into(),
            &[0xff, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            D,
            i32::MAX.into(),
            &[0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff]
        );
        round_trip!(
            oer,
            D,
            i64::MIN.into(),
            &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            oer,
            D,
            i64::MAX.into(),
            &[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(oer, E, 4.into(), &[0x04]);
        round_trip!(oer, E, (-4).into(), &[0xfc]);

        // Error expected, outside of range constraints
        encode_error!(oer, A, (i16::from(i8::MIN) - 1).into());
        encode_error!(oer, B, (i32::from(i16::MIN) - 1).into());
        encode_error!(oer, C, (i64::from(i32::MIN) - 1).into());
        encode_error!(oer, D, (i128::from(i64::MIN) - 1).into());

        encode_error!(oer, A, (i16::from(i8::MAX) + 1).into());
        encode_error!(oer, B, (i32::from(i16::MAX) + 1).into());
        encode_error!(oer, C, (i64::from(i32::MAX) + 1).into());
        encode_error!(oer, D, (i128::from(i64::MAX) + 1).into());
    }
    #[test]
    fn test_integer_single_constraint() {
        round_trip_with_constraints!(
            oer,
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
        // round_trip!(oer, Enum1, Enum1::Green, &[0x00]);
        // round_trip!(oer, Enum1, Enum1::Red, &[0x01]);
        // round_trip!(oer, Enum1, Enum1::Blue, &[0x02]);
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[non_exhaustive]
        #[allow(clippy::items_after_statements)]
        enum Enum2 {
            Red,
            Blue,
            Green,
            #[rasn(extension_addition)]
            Yellow,
            Purple,
        }

        round_trip!(oer, Enum2, Enum2::Red, &[0x00]);
        // round_trip!(oer, Enum2, Enum2::Yellow, &[0x03]);
        // round_trip!(oer, Enum2, Enum2::Purple, &[0x04]);
    }
}
