#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(test)]
macro_rules! round_trip {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        let value: $typ = $value;
        let expected: &[u8] = $expected;
        let result = crate::$codec::encode(&value);
        let actual_encoding = match result {
            Ok(actual_encoding) => {
                pretty_assertions::assert_eq!(expected, &*actual_encoding);
                actual_encoding
            }
            Err(error) => {
                panic!("Unexpected encoding error: {:?}", error);
            }
        };
        let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();
        pretty_assertions::assert_eq!(value, decoded_value);
    }};
}
#[cfg(test)]
macro_rules! encode_error {
    ($codec:ident, $typ:ty, $value:expr) => {{
        let value: $typ = $value;
        let result = crate::$codec::encode(&value);
        match result {
            Ok(actual_encoding) => {
                panic!(
                    "Expected an encoding error but got a valid encoding: {:?}",
                    &*actual_encoding
                );
            }
            Err(_) => {
                // Expected an encoding error, so we're good!
            }
        }
    }};
}
#[cfg(test)]
macro_rules! decode_error {
    ($codec:ident, $typ:ty, $value:expr) => {{
        match crate::$codec::decode::<$typ>($value) {
            Ok(_) => {
                panic!("Unexpected decoding success!");
            }
            Err(_) => {
                // Expected a decoding error, so we're good!
            }
        }
    }};
}
#[cfg(test)]
macro_rules! decode_ok {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        match crate::$codec::decode::<$typ>($value) {
            Ok(result) => {
                pretty_assertions::assert_eq!(result, $expected);
            }
            Err(e) => {
                panic!("Unexpected decoding failure!: {e}");
            }
        }
    }};
}

#[cfg(test)]
macro_rules! round_trip_with_constraints {
    ($codec:ident, $typ:ty, $constraints:expr, $value:expr, $expected:expr) => {{
        let value: $typ = $value;
        let expected: &[u8] = $expected;
        let actual_encoding = crate::$codec::encode_with_constraints($constraints, &value).unwrap();

        pretty_assertions::assert_eq!(expected, &*actual_encoding);

        let decoded_value: $typ =
            crate::$codec::decode_with_constraints($constraints, &actual_encoding).unwrap();

        pretty_assertions::assert_eq!(value, decoded_value);
    }};
}
#[cfg(test)]
macro_rules! encode_error_with_constraints {
    ($codec:ident, $typ:ty, $constraints:expr, $value:expr) => {{
        let value: $typ = $value;
        let result = crate::$codec::encode_with_constraints($constraints, &value);
        match result {
            Ok(actual_encoding) => {
                panic!(
                    "Expected an encoding error but got a valid encoding: {:?}",
                    &*actual_encoding
                );
            }
            Err(_) => {
                // Expected an encoding error, so we're good!
            }
        }
    }};
}

mod bits;
mod codec;
pub mod de;
pub mod enc;
pub mod error;
mod num;
mod per;
pub mod types;

// Helper macros
// Macros are in root anyway, but make them available in a separate module
pub mod macros {
    pub use crate::{
        constraints, permitted_alphabet_constraint, size_constraint, value_constraint,
    };
}

// Data Formats

pub mod aper;
pub mod ber;
pub mod cer;
pub mod coer;
pub mod der;
pub mod jer;
pub mod oer;
pub mod uper;

#[doc(inline)]
pub use self::{
    codec::Codec,
    de::{Decode, Decoder},
    enc::{Encode, Encoder},
    types::AsnType,
};

/// A prelude containing the codec traits and all types defined in the [`types`]
/// module.
pub mod prelude {
    pub use crate::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        types::*,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[track_caller]
    fn round_trip<T: Decode + Encode + PartialEq + core::fmt::Debug>(value: &T) {
        macro_rules! codecs {
            ($($codec:ident),+ $(,)?) => {
                $(
                    pretty_assertions::assert_eq!(
                        value,
                        &match crate::$codec::decode::<T>(
                            &match crate::$codec::encode(value).map_err(|error| error.to_string()) {
                                Ok(value) => value,
                                Err(error) => panic!("error encoding: {}", error),
                            }
                        ) {
                            Ok(value) => value,
                            Err(error) => panic!("error decoding: {}", error),
                        }
                    );
                )+
            }
        }

        codecs!(uper, aper, oer, coer, ber);
    }

    #[test]
    fn null() {
        round_trip(&());
    }

    #[test]
    fn bool() {
        round_trip(&true);
        round_trip(&false);
    }

    macro_rules! integer_tests {
        ($($integer:ident),*) => {
            $(
                #[test]
                fn $integer() {
                    let min = <$integer>::MIN;
                    let max = <$integer>::MAX;
                    let half_max = <$integer>::MAX / 2;
                    let half_min = <$integer>::MIN / 2;

                    round_trip(&min);
                    round_trip(&half_min);
                    round_trip(&half_max);
                    round_trip(&max);
                }
            )*
        }
    }

    integer_tests! {
        i8,
        i16,
        i32,
        i64,
        // i128, TODO i128 does not work for UPER/APER
        isize,
        u8,
        u16,
        u32,
        u64,
        // TODO cannot support u128 as it is constrained type by default and current constraints uses i128 for bounds
        // u128,
        usize
    }

    #[test]
    fn integer() {
        round_trip(&89);
        round_trip(&256);
        round_trip(&u64::MAX);
        round_trip(&i64::MIN);
    }

    #[test]
    fn semi_constrained_integer() {
        #[derive(PartialEq, Debug)]
        struct CustomInt(i32);

        impl crate::AsnType for CustomInt {
            const TAG: Tag = Tag::INTEGER;
            const CONSTRAINTS: Constraints<'static> =
                crate::macros::constraints!(crate::macros::value_constraint!(start: 127));
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag_and_constraints<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer::<i128>(tag, constraints, &self.0.into())
                    .map(drop)
            }
        }

        impl crate::Decode for CustomInt {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<Self, D::Error> {
                Ok(Self(decoder.decode_integer::<i32>(tag, constraints)?))
            }
        }

        round_trip(&CustomInt(256));
        round_trip(&CustomInt(i32::MAX));
    }

    #[test]
    fn bit_string() {
        round_trip(&BitString::from_slice(&[1u8, 2, 3, 4, 5]));
        round_trip(&BitString::from_slice(&[5u8, 4, 3, 2, 1]));
    }

    #[test]
    fn octet_string() {
        round_trip(&OctetString::from(vec![1u8, 2, 3, 4, 5]));
        round_trip(&OctetString::from(vec![5u8, 4, 3, 2, 1]));
    }

    #[test]
    fn utf8_string() {
        round_trip(&crate::types::Utf8String::from("Jones"));
    }

    #[test]
    fn visible_string() {
        round_trip(&crate::types::Utf8String::from("Jones"));
    }

    #[test]
    fn long_sequence_of() {
        round_trip(&vec![5u8; 0xffff]);
    }

    #[test]
    fn object_identifier() {
        round_trip(&ObjectIdentifier::new(vec![1, 2]).unwrap());
        round_trip(&ObjectIdentifier::new(vec![1, 2, 840]).unwrap());
        round_trip(&ObjectIdentifier::new(vec![1, 2, 840, 113549]).unwrap());
        round_trip(&ObjectIdentifier::new(vec![1, 2, 840, 113549, 1]).unwrap());
        round_trip(&ObjectIdentifier::new(vec![0, 3, 0, 3]).unwrap());
    }

    #[test]
    fn enumerated() {
        #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq)]
        #[rasn(enumerated, crate_root = "crate")]
        enum Day {
            Mon,
            Tues,
            Weds,
            Thurs,
            Fri,
            Sat,
            Sun,
        }

        round_trip(&Day::Mon);
        round_trip(&Day::Tues);
        round_trip(&Day::Sat);
    }
}
