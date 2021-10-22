#![doc = include_str!("../README.md")]
// #![no_std]
extern crate alloc;

mod per;

pub mod de;
pub mod enc;
pub mod types;

// Data Formats

pub mod aper;
pub mod ber;
pub mod cer;
pub mod der;
pub mod uper;

#[doc(inline)]
pub use self::{
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

    fn round_trip<T: Decode + Encode + PartialEq + core::fmt::Debug>(value: &T) {
        macro_rules! codecs {
            ($($codec:ident),+ $(,)?) => {
                $(
                    assert_eq!(
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

        codecs!(ber, der, cer, uper, aper);
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

                    round_trip(&min);
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
        i128,
        isize,
        u8,
        u16,
        u32,
        u64,
        u128,
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
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer(
                        tag,
                        Constraints::from(&[
                            constraints::Range::start_from(Integer::from(127)).into()
                        ]),
                        &self.0.into(),
                    )
                    .map(drop)
            }
        }

        impl crate::Decode for CustomInt {
            fn decode_with_tag<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
            ) -> Result<Self, D::Error> {
                use crate::de::Error;
                use core::convert::TryFrom;

                let integer = decoder.decode_integer(
                    tag,
                    Constraints::from(&[constraints::Range::start_from(Integer::from(127)).into()]),
                )?;

                Ok(Self(<_>::try_from(integer).map_err(D::Error::custom)?))
            }
        }

        round_trip(&CustomInt(256));
        round_trip(&CustomInt(i32::MAX));
    }

    #[test]
    fn bit_string() {
        round_trip(&BitString::from_slice(&vec![1u8, 2, 3, 4, 5]));
        round_trip(&BitString::from_slice(&vec![5u8, 4, 3, 2, 1]));
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

    // #[test]
    // fn object_identifier() {
    //     round_trip(&ObjectIdentifier::new(vec![1, 2]));
    //     round_trip(&ObjectIdentifier::new(vec![1, 2, 840]));
    //     round_trip(&ObjectIdentifier::new(vec![1, 2, 840, 113549]));
    //     round_trip(&ObjectIdentifier::new(vec![1, 2, 840, 113549, 1]));
    //     round_trip(&ObjectIdentifier::new(vec![0, 3, 0, 3]));
    // }

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
