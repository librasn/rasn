//! # Canonical Encoding Rules

/// Attempts to decode `T` from `input` using CER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut crate::ber::de::Decoder::new(
        input,
        crate::ber::de::DecoderOptions::cer(),
    ))
}
/// Attempts to decode `T` from `input` using CER. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid CER encoding specific to the expected type.
pub fn decode_with_remainder<T: crate::Decode>(
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    let decoder = &mut crate::ber::de::Decoder::new(input, crate::ber::de::DecoderOptions::cer());
    let decoded_instance = T::decode(decoder)?;
    Ok((decoded_instance, decoder.remaining()))
}

/// Attempts to encode `value` to CER.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::cer());

    value.encode(&mut enc)?;

    Ok(enc.output())
}
