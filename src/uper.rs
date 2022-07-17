//! # Aligned Packed Encoding Rules

pub use super::per::*;

/// Attempts to decode `T` from `input` using DER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::per::de::Error> {
    T::decode(&mut crate::per::de::Decoder::new(
        crate::types::BitStr::from_slice(input),
        crate::per::de::DecoderOptions::unaligned(),
    ))
}

/// Attempts to encode `value` to DER.
pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(crate::per::enc::EncoderOptions::unaligned());

    value.encode(&mut enc)?;

    Ok(enc.output())
}
