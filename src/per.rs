pub mod de;
pub mod enc;

use crate::types::Constraints;
use alloc::vec::Vec;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<T, crate::per::de::Error> {
    T::decode(&mut crate::per::de::Decoder::new(
        crate::types::BitStr::from_slice(input),
        options,
    ))
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    options: de::DecoderOptions,
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::per::de::Error> {
    T::decode_with_constraints(
        &mut crate::per::de::Decoder::new(crate::types::BitStr::from_slice(input), options),
        constraints,
    )
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    options: enc::EncoderOptions,
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output())
}

pub(crate) const fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}

pub(crate) fn range_from_bits(bits: u32) -> i128 {
    2i128.pow(bits) - 1
}

// Workaround for https://github.com/ferrilab/bitvec/issues/228
pub(crate) fn to_vec(
    slice: &bitvec::slice::BitSlice<u8, bitvec::order::Msb0>,
    pad_start: bool,
) -> Vec<u8> {
    use bitvec::prelude::*;
    let mut vec = Vec::new();

    if pad_start {
        for slice in pad(slice).chunks(8) {
            vec.push(slice.load_be());
        }
    } else {
        for slice in slice.chunks(8) {
            vec.push(slice.load_be());
        }
    }

    vec
}

fn pad(
    output: &bitvec::slice::BitSlice<u8, bitvec::order::Msb0>,
) -> bitvec::prelude::BitVec<u8, bitvec::order::Msb0> {
    let missing_bits = 8 - output.len() % 8;
    if missing_bits == 8 {
        output.to_bitvec()
    } else {
        let mut padding = bitvec::bitvec![u8, bitvec::prelude::Msb0; 0; missing_bits];
        padding.append(&mut output.to_bitvec());
        padding
    }
}
