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
pub fn decode_with_constraints<T: crate::Decode>(constraints: Constraints, input: &[u8]) -> Result<T, crate::per::de::Error> {
    crate::per::decode_with_constraints(de::DecoderOptions::unaligned(), constraints, input)
}

/// Attempts to encode `value` to UPER-CANONICAL.
pub fn encode_with_constraints<T: crate::Encode>(constraints: Constraints, value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    crate::per::encode_with_constraints(enc::EncoderOptions::unaligned(), constraints, value)
}
