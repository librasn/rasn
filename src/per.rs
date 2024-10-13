pub mod de;
pub mod enc;

use crate::macros::{constraints, value_constraint};
use crate::types::Constraints;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;
const SMALL_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(0, 63));
const LARGE_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(start: 0));

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut crate::per::de::Decoder::<0, 0>::new(
        crate::types::BitStr::from_slice(input),
        options,
    ))
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    options: de::DecoderOptions,
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    T::decode_with_constraints(
        &mut crate::per::de::Decoder::<0, 0>::new(crate::types::BitStr::from_slice(input), options),
        constraints,
    )
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    options: enc::EncoderOptions,
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output())
}
