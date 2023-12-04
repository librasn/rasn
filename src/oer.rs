//! Codec for Octet Encoding Rules (OER).
//! Encodes in canonical format (COER), and decodes in more versatile format (OER).
pub mod de;
pub mod enc;
mod ranges;

pub use self::{de::Decoder, enc::Encoder};
use crate::error::{DecodeError, EncodeError};
use crate::types::Constraints;
/// Attempts to decode `T` from `input` using OER.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid OER encoding specific to the expected type.
pub(crate) fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, DecodeError> {
    T::decode(&mut Decoder::new(
        crate::types::BitStr::from_slice(input),
        de::DecoderOptions::oer(),
    ))
}
/// Attempts to encode `value` of type `T` to OER.
///
/// # Errors
/// Returns `EncodeError` if `value` cannot be encoded as COER, usually meaning that constraints
/// are not met.
pub(crate) fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, EncodeError> {
    let mut enc = Encoder::new(enc::EncoderOptions::coer());
    value.encode(&mut enc)?;
    Ok(enc.output())
}
/// Attempts to decode `T` from `input` using OER with constraints.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid OER encoding, while setting specific constraints.
#[allow(dead_code)]
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    constraints: Constraints,
    input: &[u8],
) -> Result<T, DecodeError> {
    T::decode_with_constraints(
        &mut Decoder::new(
            crate::types::BitStr::from_slice(input),
            de::DecoderOptions::oer(),
        ),
        constraints,
    )
}
/// Attempts to encode `value` to COER with constraints.
///
/// # Errors
/// Returns `EncodeError` if `value` cannot be encoded as COER, while setting specific constraints.
#[allow(dead_code)]
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, EncodeError> {
    let mut enc = Encoder::new(enc::EncoderOptions::coer());
    value.encode_with_constraints(&mut enc, constraints)?;
    Ok(enc.output())
}

#[cfg(test)]
mod tests {
    // Test differences of OER and COER.
    // On some cases, COER is more stricter than OER.
    use crate as rasn;
    use crate::prelude::*;

    #[test]
    fn test_bool() {
        for value in 0x01..=0xFEu8 {
            let bytes = [value];
            // Coer fails for other values than 0x00 and 0xFF.
            decode_error!(coer, bool, &bytes);
            decode_ok!(oer, bool, &bytes, true);
        }
    }
    #[test]
    fn test_length_determinant() {
        // short with leading zeros
        decode_error!(coer, Integer, &[0x00, 0x00, 0x00, 0x01, 0x01]);
        decode_ok!(oer, Integer, &[0x00, 0x00, 0x00, 0x01, 0x01], 1.into());
        decode_error!(coer, Integer, &[0x00, 0x00, 0x00, 0x01, 0x01]);
        // Long form when not needed
        decode_error!(coer, Integer, &[0b1000_0001, 0x01, 0x01]);
        decode_ok!(oer, Integer, &[0b1000_0001, 0x01, 0x01], 1.into());
    }
    #[test]
    fn test_enumerated() {
        // short with leading zeros
        #[derive(AsnType, Decode, Encode, Debug, Clone, Copy, PartialEq)]
        #[rasn(enumerated)]
        enum Test {
            A = 1,
            B = 2,
        }
        // Leading zeroes
        decode_error!(coer, Test, &[0x00, 0x00, 0x00, 0x01, 0x01]);
        decode_ok!(oer, Test, &[0x00, 0x00, 0x00, 0x01, 0x01], Test::A);
        // Long form when not needed
        decode_error!(coer, Test, &[0b1000_0001, 0x01, 0x01]);
        decode_ok!(oer, Test, &[0b1000_0001, 0x01, 0x01], Test::A);
    }
}
