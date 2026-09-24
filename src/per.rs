pub mod de;
pub mod enc;

use crate::types::Constraints;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;
const SMALL_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(0, 63));
const LARGE_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(start: 0));

/// The identifier octets that start a DER encoding.
///
/// PER carries `DATE` as an octet string holding the value's complete DER
/// encoding, so both the encoder and the decoder need the identifier of the
/// value's universal tag.
#[derive(Clone, Copy, Debug)]
struct DerIdentifier {
    octets: [u8; 2],
    len: usize,
}

impl DerIdentifier {
    /// The identifier of a primitive universal `tag` with a number below 128:
    /// one octet for numbers below 31, otherwise the two-octet form.
    const fn primitive(tag: crate::types::Tag) -> Self {
        const HIGH_TAG_NUMBER: u32 = 0x1F;
        debug_assert!(matches!(tag.class, crate::types::Class::Universal));
        debug_assert!(tag.value < 0x80);
        if tag.value < HIGH_TAG_NUMBER {
            Self {
                octets: [tag.value as u8, 0],
                len: 1,
            }
        } else {
            Self {
                octets: [HIGH_TAG_NUMBER as u8, tag.value as u8],
                len: 2,
            }
        }
    }

    fn as_slice(&self) -> &[u8] {
        &self.octets[..self.len]
    }
}

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
/// Attempts to decode `T` from `input` using PER. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid PER encoding specific to the expected type.
pub(crate) fn decode_with_remainder<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    let decoder = &mut Decoder::<0, 0>::new(crate::types::BitStr::from_slice(input), options);
    let decoded_instance = T::decode(decoder)?;
    let remaining_bits = decoder.input().len();
    // Consider only whole bytes, ignore padding bits
    let remaining_size = remaining_bits / 8;
    debug_assert!(input.len() >= remaining_size);
    Ok((decoded_instance, &input[input.len() - remaining_size..]))
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::per::enc::Encoder::<0, 0>::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Encodes `value` to PER into an existing `buffer`, reusing its allocation.
/// The buffer is cleared before encoding.
pub(crate) fn encode_buf<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
    buffer: &mut alloc::vec::Vec<u8>,
) -> Result<(), crate::error::EncodeError> {
    let raw = core::mem::take(buffer);
    let output = crate::types::BitString::from_vec(raw);
    let mut enc = crate::per::enc::Encoder::<0, 0>::new_with_output(options, output);
    value.encode(&mut enc)?;
    *buffer = enc.output_into_vec();
    Ok(())
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
    let mut enc = crate::per::enc::Encoder::<0, 0>::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output())
}
