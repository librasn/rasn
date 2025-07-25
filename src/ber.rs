//! # Basic Encoding Rules

pub mod de;
pub mod enc;
mod identifier;
mod rules;

pub use identifier::Identifier;
pub(crate) use rules::EncodingRules;

/// Attempts to decode `T` from `input` using BER.
/// # Errors
/// Returns error specific to BER decoder if decoding is not possible.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut de::Decoder::new(input, de::DecoderOptions::ber()))
}

/// Attempts to decode `T` from `input` using BER. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid BER encoding specific to the expected type.
pub fn decode_with_remainder<T: crate::Decode>(
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    let decoder = &mut de::Decoder::new(input, de::DecoderOptions::ber());
    let decoded = T::decode(decoder)?;
    Ok((decoded, decoder.remaining()))
}

/// Attempts to encode `value` to BER.
/// # Errors
/// Returns error specific to BER encoder if encoding is not possible.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = enc::Encoder::new(enc::EncoderOptions::ber());

    value.encode(&mut enc)?;

    Ok(enc.output())
}

/// Creates a new BER encoder that can be used to encode any value.
/// # Errors
/// Returns error specific to BER encoder if encoding is not possible.
pub fn encode_scope(
    encode_fn: impl FnOnce(&mut crate::ber::enc::Encoder) -> Result<(), crate::error::EncodeError>,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::ber());

    (encode_fn)(&mut enc)?;

    Ok(enc.output())
}

#[cfg(test)]
mod tests {
    use crate::error::DecodeErrorKind;
    use alloc::borrow::ToOwned;
    use alloc::vec;
    use alloc::vec::Vec;
    use bitvec::order::Msb0;
    use chrono::{DateTime, FixedOffset, NaiveDate, Utc};

    use crate::{
        ber::{decode, encode},
        types::*,
    };

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct C0;
    impl AsnType for C0 {
        const TAG: Tag = Tag::new(Class::Context, 0);
    }

    #[test]
    fn oversized_integer() {
        const DATA: &[u8] = &[0x02, 0x06, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        assert!(matches!(
            &*decode::<u32>(DATA).unwrap_err().kind,
            DecodeErrorKind::IntegerOverflow { max_width: 32 }
        ));

        assert!(matches!(
            &*decode::<i32>(DATA).unwrap_err().kind,
            DecodeErrorKind::IntegerOverflow { max_width: 32 }
        ));
    }

    #[test]
    fn leading_integer_bytes() {
        const DATA: &[u8] = &[0x02, 0x06, 0x00, 0x00, 0x33, 0x44, 0x55, 0x66];
        assert_eq!(decode::<u32>(DATA).unwrap(), 0x33445566u32);

        const SIGNED_DATA: &[u8] = &[0x02, 0x06, 0xFF, 0xFF, 0x83, 0x44, 0x55, 0x66];
        assert_eq!(decode::<i32>(SIGNED_DATA).unwrap(), -2092673690);
    }

    #[test]
    fn bit_string() {
        const DATA: &[u8] = &[0, 0xD0];
        let small = BitString::from_vec(DATA.to_owned());
        let bits = BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());
        let padding_test = BitString::from_element(0x42);
        let padding_expected: &[u8] = &[0x03, 0x02, 0x00, 0x42];
        let trailing_test = bitvec::bitvec![u8, Msb0; 1, 0, 0, 0, 0, 1, 1, 0];
        let trailing_expected: &[u8] = &[0x03, 0x02, 0x00, 0x86];

        assert_eq!(
            small,
            decode::<BitString>(&encode(&small).unwrap()).unwrap()
        );
        assert_eq!(bits, decode::<BitString>(&encode(&bits).unwrap()).unwrap());
        assert_eq!(padding_expected, encode(&padding_test).unwrap());
        assert_eq!(trailing_expected, encode(&trailing_test).unwrap());
    }

    #[test]
    fn implicit_prefix() {
        type MyInteger = Implicit<C0, u64>;

        let new_int = MyInteger::new(5);

        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }

    #[test]
    fn explicit_prefix() {
        type MyInteger = Explicit<C0, u64>;

        let new_int = MyInteger::new(5);
        let data = &[0xA0, 3, 0x2, 0x1, 5][..];

        assert_eq!(data, &encode(&new_int).unwrap());
        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }

    #[test]
    fn implicit_tagged_constructed() {
        type ImpVec = Implicit<C0, Vec<i32>>;

        let value = ImpVec::new(vec![1, 2]);
        let data = &[0xA0, 6, 2, 1, 1, 2, 1, 2][..];

        assert_eq!(data, &*crate::ber::encode(&value).unwrap());
        assert_eq!(value, crate::ber::decode::<ImpVec>(data).unwrap());
    }

    #[test]
    fn explicit_empty_tag() {
        use crate as rasn;
        use rasn::prelude::*;
        #[derive(Debug, AsnType, Encode, Decode, PartialEq)]
        struct EmptyTag {
            #[rasn(tag(explicit(0)))]
            a: Option<()>,
        }

        let value = EmptyTag { a: None };
        // Absent field - only sequence tag present with length of 0
        let data = &[0x30, 0][..];

        assert_eq!(data, &*crate::ber::encode(&value).unwrap());
        assert_eq!(value, crate::ber::decode::<EmptyTag>(data).unwrap());
    }

    #[test]
    #[allow(clippy::items_after_statements)]
    fn set() {
        #[derive(Debug, PartialEq)]
        struct Set {
            age: u32,
            name: Utf8String,
        }

        impl AsnType for Set {
            const TAG: Tag = Tag::SET;
        }

        impl crate::types::Constructed<2, 0> for Set {
            const FIELDS: crate::types::fields::Fields<2> =
                crate::types::fields::Fields::from_static([
                    crate::types::fields::Field::new_required(0, u32::TAG, u32::TAG_TREE, "age"),
                    crate::types::fields::Field::new_required(
                        1,
                        Utf8String::TAG,
                        Utf8String::TAG_TREE,
                        "name",
                    ),
                ]);
        }

        let example = Set {
            age: 1,
            name: "Jane".into(),
        };
        let age_then_name = [0x31, 0x9, 0x2, 0x1, 0x1, 0xC, 0x4, 0x4a, 0x61, 0x6e, 0x65];
        let name_then_age = [0x31, 0x9, 0xC, 0x4, 0x4a, 0x61, 0x6e, 0x65, 0x2, 0x1, 0x1];

        assert_eq!(&age_then_name[..], crate::ber::encode(&example).unwrap());

        assert_eq!(
            crate::ber::decode::<Set>(&age_then_name).unwrap(),
            crate::ber::decode::<Set>(&name_then_age).unwrap()
        );

        impl crate::Decode for Set {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                _: Constraints,
            ) -> Result<Self, D::Error> {
                use crate::de::Error;

                #[derive(crate::AsnType, crate::Decode)]
                #[rasn(crate_root = "crate")]
                #[rasn(choice)]
                pub enum Fields {
                    Age(u32),
                    Name(Utf8String),
                }
                let codec = decoder.codec();
                decoder.decode_set::<2, 0, Fields, _, _, _>(
                    tag,
                    |decoder, indice, tag| match (indice, tag) {
                        (0, u32::TAG) => <_>::decode(decoder).map(Fields::Age),
                        (1, Utf8String::TAG) => <_>::decode(decoder).map(Fields::Name),
                        (_, _) => Err(D::Error::custom("unknown field", codec)),
                    },
                    |fields| {
                        let mut age = None;
                        let mut name = None;

                        for field in fields {
                            match field {
                                Fields::Age(value) => age = value.into(),
                                Fields::Name(value) => name = value.into(),
                            }
                        }

                        Ok(Self {
                            age: age.ok_or_else(|| D::Error::missing_field("age", codec))?,
                            name: name.ok_or_else(|| D::Error::missing_field("name", codec))?,
                        })
                    },
                )
            }
        }

        impl crate::Encode for Set {
            fn encode_with_tag_and_constraints<'b, EN: crate::Encoder<'b>>(
                &self,
                encoder: &mut EN,
                tag: crate::types::Tag,
                _: Constraints,
                _: crate::types::Identifier,
            ) -> Result<(), EN::Error> {
                encoder.encode_set::<2, 0, Self, _>(
                    tag,
                    |encoder| {
                        self.age.encode(encoder)?;
                        self.name.encode(encoder)?;
                        Ok(())
                    },
                    crate::types::Identifier::EMPTY,
                )?;

                Ok(())
            }
        }
    }
    #[test]
    fn test_generalized_time() {
        // "20801009130005.342Z"
        let offset = chrono::FixedOffset::east_opt(0).unwrap();
        let dt = NaiveDate::from_ymd_opt(2080, 10, 9)
            .unwrap()
            .and_hms_micro_opt(13, 0, 5, 342_000)
            .unwrap()
            .and_local_timezone(offset);
        round_trip!(
            ber,
            GeneralizedTime,
            GeneralizedTime::from(dt.unwrap(),),
            &[
                0x18, 0x13, 0x32, 0x30, 0x38, 0x30, 0x31, 0x30, 0x30, 0x39, 0x31, 0x33, 0x30, 0x30,
                0x30, 0x35, 0x2e, 0x33, 0x34, 0x32, 0x5a
            ]
        );

        // https://github.com/XAMPPRocky/rasn/issues/57
        let data = [
            24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90,
        ];
        assert!(crate::der::decode::<crate::types::Open>(&data).is_err());
        assert!(crate::ber::decode::<crate::types::Open>(&data).is_err());

        // "20180122132900Z"
        round_trip!(
            ber,
            GeneralizedTime,
            GeneralizedTime::from(
                NaiveDate::from_ymd_opt(2018, 1, 22)
                    .unwrap()
                    .and_hms_opt(13, 29, 0)
                    .unwrap()
                    .and_utc()
            ),
            &[
                0x18, 0x0f, 0x32, 0x30, 0x31, 0x38, 0x30, 0x31, 0x32, 0x32, 0x31, 0x33, 0x32, 0x39,
                0x30, 0x30, 0x5a
            ]
        );
        // "20180122130000Z"
        round_trip!(
            ber,
            GeneralizedTime,
            GeneralizedTime::from(
                NaiveDate::from_ymd_opt(2018, 1, 22)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap()
                    .and_utc()
            ),
            &[
                0x18, 0x0f, 0x32, 0x30, 0x31, 0x38, 0x30, 0x31, 0x32, 0x32, 0x31, 0x33, 0x30, 0x30,
                0x30, 0x30, 0x5a
            ]
        );

        // "20230122130000-0500" - converts to canonical form "20230122180000Z"
        let offset = FixedOffset::east_opt(-3600 * 5).unwrap();
        let dt1: DateTime<FixedOffset> = GeneralizedTime::from(DateTime::<Utc>::from(
            NaiveDate::from_ymd_opt(2023, 1, 22)
                .unwrap()
                .and_hms_opt(13, 0, 0)
                .unwrap()
                .and_local_timezone(offset)
                .unwrap(),
        ));
        round_trip!(
            ber,
            GeneralizedTime,
            dt1,
            &[
                0x18, 0x0f, 0x32, 0x30, 0x32, 0x33, 0x30, 0x31, 0x32, 0x32, 0x31, 0x38, 0x30, 0x30,
                0x30, 0x30, 0x5a
            ]
        );
        // "20230122130000-0500" as bytes
        let data = [
            24, 19, 50, 48, 50, 51, 48, 49, 50, 50, 49, 51, 48, 48, 48, 48, 45, 48, 53, 48, 48,
        ];
        let result = crate::ber::decode::<crate::types::GeneralizedTime>(&data);
        assert!(result.is_ok());
        assert_eq!(dt1, result.unwrap());
        // Above without timezone
        let data = [
            24, 14, 50, 48, 50, 51, 48, 49, 50, 50, 49, 51, 48, 48, 48, 48,
        ];
        let result = crate::ber::decode::<crate::types::GeneralizedTime>(&data);
        // if err, print error
        if result.is_err() {
            println!("{result:?}");
        }
        assert!(result.is_ok());
        let data = [
            24, 22, 50, 48, 50, 51, 48, 49, 50, 50, 49, 51, 48, 48, 48, 48, 45, 45, 0xE2, 0x82,
            0xAC, 45, 45, 45,
        ];
        let result = crate::ber::decode::<crate::types::GeneralizedTime>(&data);
        assert!(result.is_err());
    }
    #[test]
    fn test_utc_time() {
        // "180122132900Z"
        round_trip!(
            ber,
            UtcTime,
            UtcTime::from(
                NaiveDate::from_ymd_opt(2018, 1, 22)
                    .unwrap()
                    .and_hms_opt(13, 29, 0)
                    .unwrap()
                    .and_utc()
            ),
            &[
                0x17, 0x0d, 0x31, 0x38, 0x30, 0x31, 0x32, 0x32, 0x31, 0x33, 0x32, 0x39, 0x30, 0x30,
                0x5a
            ]
        );
        // "230122130000-0500" - converts to canonical form "230122180000Z"
        let offset = FixedOffset::east_opt(-3600 * 5).unwrap();
        let dt1 = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2023, 1, 22)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap(),
            offset,
        );
        round_trip!(
            ber,
            UtcTime,
            dt1.into(),
            &[
                0x17, 0x0d, 0x32, 0x33, 0x30, 0x31, 0x32, 0x32, 0x31, 0x38, 0x30, 0x30, 0x30, 0x30,
                0x5a
            ]
        );
        // "230122130000-0500" as bytes
        let data = [
            23, 17, 50, 51, 48, 49, 50, 50, 49, 51, 48, 48, 48, 48, 45, 48, 53, 48, 48,
        ];
        let result = crate::ber::decode::<crate::types::UtcTime>(&data);
        assert!(result.is_ok());
        assert_eq!(dt1, result.unwrap());
    }

    #[test]
    fn test_date() {
        round_trip!(
            ber,
            Date,
            NaiveDate::from_ymd_opt(2012, 12, 21).unwrap(),
            &[0x1f, 0x1f, 0x08, 0x32, 0x30, 0x31, 0x32, 0x31, 0x32, 0x32, 0x31]
        );
    }
    #[test]
    fn test_extended_sequence() {
        use crate as rasn;
        use rasn::prelude::*;
        #[derive(AsnType, Debug, Clone, Encode, Decode, PartialEq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct ExtendedInteger {
            #[rasn(extension_addition)]
            pub extension: Option<u64>,
        }
        round_trip!(
            ber,
            ExtendedInteger,
            ExtendedInteger {
                extension: Some(42)
            },
            &[0x30, 0x03, 0x80, 0x01, 0x2A]
        );
        #[derive(AsnType, Debug, Clone, Encode, Decode, PartialEq)]
        #[non_exhaustive]
        pub struct ExtendedExplicitInteger {
            #[rasn(extension_addition)]
            #[rasn(tag(explicit(5)))]
            pub extension: u64,
        }

        round_trip!(
            ber,
            ExtendedExplicitInteger,
            ExtendedExplicitInteger { extension: 42 },
            &[0x30, 0x05, 0xA5, 0x03, 0x02, 0x01, 0x2A]
        );
    }
}
