#![doc = include_str!("../README.md")]
// #![cfg_attr(not(test), no_std)]
extern crate alloc;

#[cfg(test)]
macro_rules! round_trip {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        let value: $typ = $value;
        let expected: &[u8] = $expected;
        let actual_encoding = crate::$codec::encode(&value).unwrap();

        pretty_assertions::assert_eq!(expected, &*actual_encoding);

        let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();

        pretty_assertions::assert_eq!(value, decoded_value);
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

pub mod codec;
pub mod de;
pub mod enc;
pub mod types;

// Data Formats

mod per;

pub mod aper;
pub mod ber;
mod bits;
pub mod cer;
pub mod der;
pub mod error;
pub mod jer;
mod num;
pub mod uper;

#[doc(inline)]
pub use self::{
    codec::Codec,
    de::{Decode, Decoder},
    enc::{Encode, Encoder},
    types::{AsnType, Tag, TagTree},
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
                                Err(error) => panic!("{}", error),
                            }
                        ) {
                            Ok(value) => value,
                            Err(error) => panic!("{}", error),
                        }
                    );
                )+
            }
        }

        codecs!(uper, aper);
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
                    let min = <$integer>::min_value();
                    let max = <$integer>::max_value();
                    let half_max = <$integer>::max_value() / 2;
                    let half_min = <$integer>::min_value() / 2;

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
        isize,
        u8,
        u16,
        u32,
        u64,
        usize
    }

    #[test]
    fn integer() {
        round_trip(&Integer::from(89));
        round_trip(&Integer::from(256));
        round_trip(&Integer::from(u64::MAX));
        round_trip(&Integer::from(i64::MIN));
    }

    #[test]
    fn semi_constrained_integer() {
        #[derive(PartialEq, Debug)]
        struct CustomInt(i32);

        impl crate::AsnType for CustomInt {
            const TAG: Tag = Tag::INTEGER;
            const CONSTRAINTS: Constraints<'static> =
                Constraints::new(&[Constraint::Value(constraints::Extensible::new(
                    constraints::Value::new(constraints::Bounded::start_from(127)),
                ))]);
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag_and_constraints<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer(tag, constraints, &self.0.into())
                    .map(drop)
            }
        }

        impl crate::Decode for CustomInt {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<Self, D::Error> {
                use crate::de::Error;
                use core::convert::TryFrom;

                let integer = decoder.decode_integer(tag, constraints)?;

                Ok(Self(
                    <_>::try_from(integer).map_err(|e| D::Error::custom(e, decoder.codec()))?,
                ))
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
    #[ignore]
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
