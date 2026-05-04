//! ASN.1 Value Notation (AVN) encoding.

pub mod de;
pub mod enc;
pub mod value;

/// Attempts to encode `value` to AVN text format.
///
/// # Errors
/// Returns an error specific to the AVN encoder if encoding is not possible.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::string::String, crate::error::EncodeError> {
    let mut encoder = enc::Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.to_string())
}

/// Attempts to decode `input` (AVN text) to type `T`.
///
/// # Errors
/// Returns an error specific to the AVN decoder if decoding is not possible.
pub fn decode<T: crate::Decode>(input: &str) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut de::Decoder::new(input)?)
}

#[cfg(test)]
mod tests {
    /// Encode `$value` as `$typ`, assert the output equals `$expected`, then decode
    /// back and assert round-trip equality.
    macro_rules! round_trip_avn {
        ($typ:ty, $value:expr, $expected:expr) => {{
            let value: $typ = $value;
            let actual = crate::avn::encode(&value).unwrap();
            pretty_assertions::assert_eq!($expected, actual.as_str());
            let decoded: $typ = crate::avn::decode(&actual).unwrap();
            pretty_assertions::assert_eq!(value, decoded);
        }};
    }

    use crate::prelude::*;

    // -----------------------------------------------------------------------
    // Test types (shared with JER tests, adapted for AVN)
    // -----------------------------------------------------------------------

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

    #[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq, Ord, Eq, PartialOrd, Hash)]
    #[rasn(automatic_tags, choice)]
    #[rasn(crate_root = "crate")]
    enum SimpleChoice {
        Test1(u8),
        #[rasn(size("0..3"))]
        Test2(Utf8String),
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

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn bool_values() {
        round_trip_avn!(bool, true, "TRUE");
        round_trip_avn!(bool, false, "FALSE");
    }

    #[test]
    fn integer_values() {
        round_trip_avn!(u8, 42_u8, "42");
        round_trip_avn!(u8, 1_u8, "1");
        round_trip_avn!(i8, -1_i8, "-1");
        round_trip_avn!(u16, 0_u16, "0");
        round_trip_avn!(Integer, 1.into(), "1");
        round_trip_avn!(Integer, (-1_235_352).into(), "-1235352");
    }

    #[test]
    fn null_value() {
        round_trip_avn!((), (), "NULL");
    }

    #[test]
    fn octet_string_value() {
        round_trip_avn!(
            OctetString,
            OctetString::from_static(&[0x01, 0xFF]),
            "'01FF'H"
        );
        round_trip_avn!(OctetString, OctetString::from_static(&[]), "''H");
    }

    #[test]
    fn bit_string_value() {
        // Non-byte-aligned: 2 bits [1, 0] -> '10'B
        round_trip_avn!(
            BitString,
            [true, false].into_iter().collect::<BitString>(),
            "'10'B"
        );
        // Byte-aligned: 8 bits -> hex
        round_trip_avn!(
            BitString,
            [true, false, true, false, false, false, false, false]
                .into_iter()
                .collect::<BitString>(),
            "'A0'H"
        );
        // Empty
        round_trip_avn!(BitString, BitString::default(), "''H");
    }

    #[test]
    fn object_identifier_value() {
        round_trip_avn!(
            ObjectIdentifier,
            ObjectIdentifier::from(Oid::JOINT_ISO_ITU_T_DS_NAME_FORM),
            "{ 2 5 15 }"
        );
    }

    #[test]
    fn enumerated_value() {
        round_trip_avn!(SimpleEnum, SimpleEnum::Test1, "Test1");
        round_trip_avn!(SimpleEnum, SimpleEnum::Test2, "Test2");
    }

    #[test]
    fn choice_value() {
        round_trip_avn!(SimpleChoice, SimpleChoice::Test1(3), "Test1 : 3");
        round_trip_avn!(
            SimpleChoice,
            SimpleChoice::Test2("foo".into()),
            "Test2 : \"foo\""
        );
    }

    #[test]
    fn sequence_of_values() {
        round_trip_avn!(
            SequenceOf<u8>,
            alloc::vec![1, 2, 3],
            "{\n  1,\n  2,\n  3\n}"
        );
        round_trip_avn!(SequenceOf<bool>, alloc::vec![], "{}");
    }

    #[test]
    fn sequence_with_optional() {
        // Absent optional field is omitted
        round_trip_avn!(
            Very,
            Very {
                a: Some(Nested {
                    very: Some(Struct { strct: None }),
                    nested: Some(false)
                })
            },
            "{\n  a {\n    very {},\n    nested FALSE\n  }\n}"
        );
    }

    #[test]
    fn sequence_value() {
        round_trip_avn!(
            TestTypeA,
            TestTypeA {
                juice: 0.into(),
                wine: Inner::Wine(4),
                grappa: [true, false].into_iter().collect::<BitString>()
            },
            "{\n  juice 0,\n  wine Wine : 4,\n  grappa '10'B\n}"
        );
    }

    // Named-number INTEGER round-trip test
    #[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
    #[rasn(delegate, crate_root = "crate")]
    #[rasn(value("0..=255"), named_values("pukAppl1" = 1, "pukAppl2" = 2, "secondPUKAppl1" = 129, "secondPUKAppl2" = 130))]
    struct PUKKeyRef(pub u8);

    #[test]
    fn named_integer_round_trip() {
        // Named value should encode as identifier and decode back
        round_trip_avn!(PUKKeyRef, PUKKeyRef(1), "pukAppl1");
        round_trip_avn!(PUKKeyRef, PUKKeyRef(2), "pukAppl2");
        round_trip_avn!(PUKKeyRef, PUKKeyRef(129), "secondPUKAppl1");
    }

    #[test]
    fn named_integer_unlisted_value_uses_number() {
        // Values not in the NamedNumberList fall back to bare numbers
        round_trip_avn!(PUKKeyRef, PUKKeyRef(99), "99");
    }

    #[test]
    fn with_identifier_annotation() {
        round_trip_avn!(
            Renamed,
            Renamed {
                very: 1.into(),
                renamed: Some(true),
            },
            "{\n  so-very 1,\n  re_named TRUE\n}"
        );
        round_trip_avn!(Renumed, Renumed::Test1("hel".into()), "test-1 : \"hel\"");
    }
}
