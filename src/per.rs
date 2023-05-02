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
pub(crate) fn decode<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<T, crate::per::de::Error> {
    T::decode(&mut crate::per::de::Decoder::new(
        crate::types::BitStr::from_slice(input),
        options,
    ))
}

/// Attempts to encode `value` to DER.
pub(crate) fn encode<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Attempts to decode `T` from `input` using DER.
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

/// Attempts to encode `value` to DER.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    options: enc::EncoderOptions,
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    let mut enc = crate::per::enc::Encoder::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output())
}

pub(crate) fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, types::*};

    macro_rules! round_trip {
        ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
            let value = $value;
            let actual_encoding = crate::$codec::encode(&value).unwrap();

            assert_eq!($expected, &*actual_encoding);

            let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();

            assert_eq!(value, decoded_value);
        }};
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
            Medium,
        }

        round_trip!(uper, Choice, Choice::Normal, &[0]);
        round_trip!(uper, Choice, Choice::Medium, &[0x80, 1, 0]);
        round_trip!(aper, Choice, Choice::Medium, &[0x80, 1, 0]);
    }

    #[test]
    fn enumerated() {

        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Enum1 { Green, Red, Blue, }

        round_trip!(uper, Enum1, Enum1::Green, &[0]);
        round_trip!(uper, Enum1, Enum1::Red, &[0x40]);
        round_trip!(uper, Enum1, Enum1::Blue, &[0x80]);

        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        #[non_exhaustive]
        enum Enum2 {
            Red,
            Blue,
            Green,
            #[rasn(extension_addition)]
            Yellow,
            #[rasn(extension_addition)]
            Purple,
        }

        round_trip!(uper, Enum2, Enum2::Red, &[0]);
        round_trip!(uper, Enum2, Enum2::Yellow, &[0x80]);
        round_trip!(uper, Enum2, Enum2::Purple, &[0x81]);
    }

    #[test]
    fn extension_additions() {
        #[derive(AsnType, Clone, Copy, Debug, Decode, Default, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Urgency {
            #[default]
            Normal,
            High,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct MySequenceValExtension {
            #[rasn(value("0..254"))]
            alternate_item_code: u8,
            #[rasn(size("3..10"))]
            alternate_item_name: Option<Ia5String>,
        }

        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        #[non_exhaustive]
        struct MySequenceVal {
            #[rasn(value("0..254"))]
            item_code: u8,
            #[rasn(size("3..10"))]
            item_name: Option<Ia5String>,
            #[rasn(extension_addition, default)]
            urgency: Urgency,
            #[rasn(extension_addition_group)]
            v2: MySequenceValExtension,
        }

        let value = MySequenceVal {
            item_code: 0,
            item_name: None,
            urgency: Urgency::Normal,
            v2: MySequenceValExtension {
                alternate_item_code: 0,
                alternate_item_name: None,
            },
        };

        round_trip!(
            uper,
            MySequenceVal,
            value,
            &[0x80, 0x00, 0xa0, 0x40, 0x00, 0x00, 0x0a]
        );

        let value = MySequenceVal {
            item_code: 29,
            item_name: Some(Ia5String::try_from("SHERRY").unwrap()),
            urgency: Urgency::High,
            v2: MySequenceValExtension {
                alternate_item_code: 45,
                alternate_item_name: Some(Ia5String::try_from("PORT").unwrap()),
            },
        };

        round_trip!(
            uper,
            MySequenceVal,
            value,
            &[
            0xc7, 0x5d, 0x39, 0x11, 0x69, 0x52, 0xb2, 0x07, 0x01, 0x80,
            0x05, 0x96, 0x9a, 0x13, 0xe9, 0x54

            ]
        );
    }
}
