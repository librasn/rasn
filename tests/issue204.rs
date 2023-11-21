use rasn::prelude::*;

#[derive(Debug, AsnType, rasn::Encode, rasn::Decode, PartialEq)]
#[rasn(delegate, size("1..=255"))]
struct SimpleNumericString(pub NumericString);

#[derive(Debug, AsnType, rasn::Encode, rasn::Decode, PartialEq)]
#[rasn(delegate, from("0..=9"), size("1..=255"))]
struct SimpleConstrainedNumericString(pub NumericString);

#[derive(Debug, AsnType, rasn::Encode, rasn::Decode, PartialEq)]
#[rasn(delegate, from("0..=2"), size("1..=255"))]
struct Simple123NumericString(pub NumericString);

#[test]
fn round_trip_numeric_string_aper() {
    let string = SimpleNumericString(
        NumericString::from_bytes(b"12345 12345 12345 0")
            .expect("Failed to construct NumericString"),
    );
    let encoded = rasn::aper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[18, 35, 69, 96, 35, 69, 96, 35, 69, 96, 16]);
    let decoded: SimpleNumericString = rasn::aper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}

#[test]
fn round_trip_numeric_string_uper() {
    let string = SimpleNumericString(
        NumericString::from_bytes(b"12345 12345 12345 0")
            .expect("Failed to construct NumericString"),
    );
    let encoded = rasn::uper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[18, 35, 69, 96, 35, 69, 96, 35, 69, 96, 16]);
    let decoded: SimpleNumericString = rasn::uper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}

#[test]
fn round_trip_constrained_numeric_string_aper() {
    let string = SimpleConstrainedNumericString(
        NumericString::from_bytes(b"123451234512345").expect("Failed to construct NumericString"),
    );
    let encoded = rasn::aper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[14, 18, 52, 81, 35, 69, 18, 52, 80]);
    let decoded: SimpleConstrainedNumericString =
        rasn::aper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}

#[test]
fn round_trip_constrained_numeric_string_uper() {
    let string = SimpleConstrainedNumericString(
        NumericString::from_bytes(b"123451234512345").expect("Failed to construct NumericString"),
    );
    let encoded = rasn::uper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[14, 18, 52, 81, 35, 69, 18, 52, 80]);
    let decoded: SimpleConstrainedNumericString =
        rasn::uper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}

#[test]
fn round_trip_limited_charset_numeric_string_aper() {
    let string = Simple123NumericString(
        NumericString::from_bytes(b"120120120").expect("Failed to construct NumericString"),
    );
    let encoded = rasn::aper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[8, 97, 134, 0]);
    let decoded: Simple123NumericString = rasn::aper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}

#[test]
fn round_trip_limited_charset_numeric_string_uper() {
    let string = Simple123NumericString(
        NumericString::from_bytes(b"120120120").expect("Failed to construct NumericString"),
    );
    let encoded = rasn::uper::encode(&string).expect("Failed to encode");
    assert_eq!(&encoded, &[8, 97, 134, 0]);
    let decoded: Simple123NumericString = rasn::uper::decode(&encoded).expect("Failed to decode");
    assert_eq!(decoded, string);
}
