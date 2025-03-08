//! # Distinguished Encoding Rules

pub use crate::ber::*;

/// Attempts to decode `T` from `input` using DER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut crate::ber::de::Decoder::new(
        input,
        crate::ber::de::DecoderOptions::der(),
    ))
}
/// Attempts to decode `T` from `input` using DER. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid DER encoding specific to the expected type.
pub fn decode_with_remainder<T: crate::Decode>(
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    let decoder = &mut de::Decoder::new(input, de::DecoderOptions::der());
    let decoded = T::decode(decoder)?;
    Ok((decoded, decoder.remaining()))
}

/// Attempts to encode `value` to DER.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::der());

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Creates a new DER encoder that can be used to encode any value.
pub fn encode_scope(
    encode_fn: impl FnOnce(&mut crate::ber::enc::Encoder) -> Result<(), crate::error::EncodeError>,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::der());

    (encode_fn)(&mut enc)?;

    Ok(enc.output())
}
