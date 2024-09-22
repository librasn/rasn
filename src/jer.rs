//! JSON Encoding Rules.

pub mod de;
pub mod enc;

/// Attempts to decode `T` from `input` using JER.
/// # Errors
/// Returns error specific to JER decoder if decoding is not possible.
pub fn decode<T: crate::Decode>(input: &str) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut de::Decoder::new(input)?)
}

/// Attempts to encode `value` to JER.
/// # Errors
/// Returns error specific to JER encoder if encoding is not possible.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::string::String, crate::error::EncodeError> {
    let mut encoder = enc::Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.to_string())
}

#[cfg(test)]
mod tests {
    macro_rules! round_trip_jer {
        ($typ:ty, $value:expr, $expected:expr) => {{
            let value: $typ = $value;
            let expected: &'static str = $expected;
            let actual_encoding = crate::jer::encode(&value).unwrap();

            pretty_assertions::assert_eq!(expected, &*actual_encoding);

            let decoded_value: $typ = crate::jer::decode(&actual_encoding).unwrap();

            pretty_assertions::assert_eq!(value, decoded_value);
        }};
    }

    macro_rules! round_trip_string_type {
        ($typ:ty) => {{
            let string = String::from(" 1234567890");
            let expected: &'static str = "\" 1234567890\"";
            let value: $typ = string.try_into().unwrap();
            let actual_encoding = crate::jer::encode(&value).unwrap();

            pretty_assertions::assert_eq!(expected, &actual_encoding);

            let decoded_value: $typ = crate::jer::decode(&actual_encoding).unwrap();

            pretty_assertions::assert_eq!(value, decoded_value);
        }};
    }

    use crate::prelude::*;

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    struct TestTypeA {
        #[rasn(value("0..3", extensible))]
        juice: Integer,
        wine: Inner,
        #[rasn(extension_addition)]
        grappa: BitString,
    }

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum Inner {
        #[rasn(value("0..3"))]
        Wine(u8),
    }

    #[derive(AsnType, Decode, Encode, Debug, Clone, Copy, PartialEq)]
    #[rasn(automatic_tags, enumerated)]
    #[rasn(crate_root = "crate")]
    enum SimpleEnum {
        Test1 = 5,
        Test2 = 2,
    }

    #[derive(AsnType, Decode, Encode, Debug, Clone, Copy, PartialEq)]
    #[rasn(automatic_tags, enumerated)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    enum ExtEnum {
        Test1 = 5,
        Test2 = 2,
        #[rasn(extension_addition)]
        Test3 = -1,
    }

    #[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq, Ord, Eq, PartialOrd, Hash)]
    #[rasn(automatic_tags, choice)]
    #[rasn(crate_root = "crate")]
    enum SimpleChoice {
        Test1(u8),
        #[rasn(size("0..3"))]
        Test2(Utf8String),
    }

    #[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
    #[rasn(automatic_tags, choice)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    enum ExtChoice {
        Test1(u8),
        #[rasn(size("0..3"))]
        Test2(Utf8String),
        #[rasn(extension_addition)]
        Test3(bool),
    }

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    struct Very {
        #[rasn(extension_addition)]
        a: Option<Nested>,
    }

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct Nested {
        very: Option<Struct>,
        nested: Option<bool>,
    }

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct Struct {
        strct: Option<u8>,
    }

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(crate_root = "crate", delegate, size("3", extensible))]
    struct ConstrainedOctetString(pub OctetString);

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(crate_root = "crate", delegate, value("-5..=5", extensible))]
    struct ConstrainedInt(pub Integer);

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(crate_root = "crate", delegate, size("3"))]
    struct ConstrainedBitString(pub BitString);

    #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct Renamed {
        #[rasn(identifier = "so-very")]
        very: Integer,
        #[rasn(identifier = "re_named")]
        renamed: Option<bool>,
    }

    #[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
    #[rasn(automatic_tags, choice)]
    #[rasn(crate_root = "crate")]
    enum Renumed {
        #[rasn(identifier = "test-1")]
        #[rasn(size("0..3"))]
        Test1(Utf8String),
    }

    #[test]
    fn bool() {
        round_trip_jer!(bool, true, "true");
        round_trip_jer!(bool, false, "false");
    }

    #[test]
    fn integer() {
        round_trip_jer!(u8, 1, "1");
        round_trip_jer!(i8, -1, "-1");
        round_trip_jer!(u16, 0, "0");
        round_trip_jer!(i16, -14321, "-14321");
        round_trip_jer!(i64, -1213428598524996264, "-1213428598524996264");
        round_trip_jer!(Integer, 1.into(), "1");
        round_trip_jer!(Integer, (-1235352).into(), "-1235352");
        round_trip_jer!(ConstrainedInt, ConstrainedInt(1.into()), "1");
    }

    #[test]
    fn bit_string() {
        round_trip_jer!(
            BitString,
            BitString::from_iter([true, false].into_iter()),
            r#"{"length":2,"value":"80"}"#
        );
        round_trip_jer!(
            ConstrainedBitString,
            ConstrainedBitString(BitString::from_iter([true, false, true].into_iter())),
            "\"A0\""
        );
    }

    #[test]
    fn octet_string() {
        round_trip_jer!(OctetString, OctetString::from_static(&[1, 255]), "\"01FF\"");
        round_trip_jer!(
            ConstrainedOctetString,
            ConstrainedOctetString(OctetString::from_static(&[1, 255, 0, 254])),
            "\"01FF00FE\""
        );
    }

    #[test]
    fn object_identifier() {
        round_trip_jer!(
            ObjectIdentifier,
            ObjectIdentifier::from(Oid::JOINT_ISO_ITU_T_DS_NAME_FORM),
            "\"2.5.15\""
        );
    }

    #[test]
    fn string_types() {
        round_trip_string_type!(NumericString);
        round_trip_string_type!(GeneralString);
        round_trip_string_type!(VisibleString);
        round_trip_string_type!(UniversalString);
        round_trip_string_type!(PrintableString);
        round_trip_string_type!(Ia5String);
        round_trip_string_type!(Utf8String);
    }

    #[test]
    fn enumerated() {
        round_trip_jer!(SimpleEnum, SimpleEnum::Test1, "\"Test1\"");
        round_trip_jer!(SimpleEnum, SimpleEnum::Test2, "\"Test2\"");
        round_trip_jer!(ExtEnum, ExtEnum::Test1, "\"Test1\"");
        round_trip_jer!(ExtEnum, ExtEnum::Test2, "\"Test2\"");
        round_trip_jer!(ExtEnum, ExtEnum::Test3, "\"Test3\"");
    }

    #[test]
    fn choice() {
        round_trip_jer!(SimpleChoice, SimpleChoice::Test1(3), "{\"Test1\":3}");
        round_trip_jer!(
            SimpleChoice,
            SimpleChoice::Test2("foo".into()),
            "{\"Test2\":\"foo\"}"
        );
        round_trip_jer!(ExtChoice, ExtChoice::Test1(255), "{\"Test1\":255}");
        round_trip_jer!(
            ExtChoice,
            ExtChoice::Test2("bar".into()),
            "{\"Test2\":\"bar\"}"
        );
        round_trip_jer!(ExtChoice, ExtChoice::Test3(true), "{\"Test3\":true}");
    }

    #[test]
    fn sequence_of() {
        round_trip_jer!(
            SequenceOf<SimpleChoice>,
            alloc::vec![SimpleChoice::Test1(3)],
            "[{\"Test1\":3}]"
        );
        round_trip_jer!(
            SequenceOf<u8>,
            alloc::vec![1, 2, 3, 4, 5, 5, 3],
            "[1,2,3,4,5,5,3]"
        );
        round_trip_jer!(SequenceOf<bool>, alloc::vec![], "[]");
    }

    #[test]
    fn set_of() {
        round_trip_jer!(
            SetOf<SimpleChoice>,
            SetOf::from_vec(alloc::vec![SimpleChoice::Test1(3)]),
            "[{\"Test1\":3}]"
        );
        // SetOf is not ordered, and does not maintain order, so we need to adapt a bit
        let set = SetOf::from_vec(alloc::vec![1, 2, 3, 4, 5]);
        let actual_encoding = crate::jer::encode(&set).unwrap();
        let trimmed = actual_encoding
            .trim_start_matches('[')
            .trim_end_matches(']');

        // Split the string by commas and sum the values
        let sum = trimmed
            .split(',')
            .map(|num_str| num_str.trim().parse::<i32>())
            .sum::<Result<i32, _>>()
            .unwrap();
        assert_eq!(sum, 15);
        let decoded_value: SetOf<_> = crate::jer::decode(&actual_encoding).unwrap();

        assert_eq!(set, decoded_value);
        round_trip_jer!(SetOf<bool>, SetOf::from_vec(alloc::vec![]), "[]");
    }

    #[test]
    fn seqence() {
        round_trip_jer!(
            TestTypeA,
            TestTypeA {
                juice: 0.into(),
                wine: Inner::Wine(4),
                grappa: BitString::from_iter([true, false].iter())
            },
            r#"{"grappa":{"length":2,"value":"80"},"juice":0,"wine":{"Wine":4}}"#
        );
        round_trip_jer!(
            Very,
            Very {
                a: Some(Nested {
                    very: Some(Struct { strct: None }),
                    nested: Some(false)
                })
            },
            r#"{"a":{"nested":false,"very":{}}}"#
        );
    }

    #[test]
    fn with_identifier_annotation() {
        round_trip_jer!(
            Renamed,
            Renamed {
                very: 1.into(),
                renamed: Some(true),
            },
            r#"{"re_named":true,"so-very":1}"#
        );

        round_trip_jer!(Renumed, Renumed::Test1("hel".into()), r#"{"test-1":"hel"}"#);
    }
}
