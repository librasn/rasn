pub mod de;
pub mod enc;
mod utils;

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
    // options: enc::EncoderOptions,
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
    use crate::types::Integer;
    #[test]
    fn bool() {
        round_trip!(oer, bool, true, &[0xff]);
        round_trip!(oer, bool, false, &[0]);
    }
    #[test]
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
    }
}
