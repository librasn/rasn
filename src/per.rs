pub mod de;
pub mod enc;

use crate::types::Constraints;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;
const TWO_FIFTY_SIX: u32 = 256;

/// Attempts to decode `T` from `input` using DER.
pub(crate) fn decode<T: crate::Decode>(options: de::DecoderOptions, input: &[u8]) -> Result<T, crate::per::de::Error> {
    T::decode(&mut crate::per::de::Decoder::new(crate::types::BitStr::from_slice(input), options))
}

/// Attempts to encode `value` to DER.
pub(crate) fn encode<T: crate::Encode>(options: enc::EncoderOptions, value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Attempts to decode `T` from `input` using DER.
pub(crate) fn decode_with_constraints<T: crate::Decode>(options: de::DecoderOptions, constraints: Constraints, input: &[u8]) -> Result<T, crate::per::de::Error> {
    T::decode_with_constraints(&mut crate::per::de::Decoder::new(crate::types::BitStr::from_slice(input), options), constraints)
}

/// Attempts to encode `value` to DER.
pub(crate) fn encode_with_constraints<T: crate::Encode>(options: enc::EncoderOptions, constraints: Constraints, value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output())
}

fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}

#[cfg(test)]
mod tests {
    use crate::{types::*, prelude::*};

    macro_rules! round_trip {
        ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
            let value = $value;
            let actual_encoding = crate::$codec::encode(&value).unwrap();

            assert_eq!($expected, &*actual_encoding);

            let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();

            assert_eq!(value, decoded_value);
        }}
    }

    #[test]
    fn sequence_of() {
        round_trip!(uper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
        round_trip!(aper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn choice() {
        use crate as rasn;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum Choice {
            Normal,
            High,
            #[rasn(extension_addition)]
            Medium
        }

        round_trip!(uper, Choice, Choice::Normal, &[0]);
        round_trip!(uper, Choice, Choice::Medium, &[0x80, 1, 0]);
        round_trip!(aper, Choice, Choice::Medium, &[0x80, 1, 0]);
    }
}
