use rasn::{aper, error::DecodeErrorKind, prelude::*, uper};

/// `S1 ::= INTEGER (1..10)`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("1..=10"))]
struct S1(u8);

/// `S2 ::= SEQUENCE (SIZE (1..10)) OF BOOLEAN`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=10"))]
struct S2(SequenceOf<bool>);

#[test]
fn uper_rejects_surplus_constraint_values() {
    let error = uper::decode::<S1>(&[0xf0]).unwrap_err();
    assert!(matches!(
        *error.kind,
        DecodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));
    assert_eq!(uper::decode::<S1>(&[0x90]).unwrap(), S1(10));

    let error = uper::decode::<S2>(&[0xff; 3]).unwrap_err();
    assert!(matches!(
        *error.kind,
        DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(16), .. }
    ));
    assert_eq!(
        uper::decode::<S2>(&[0b1001_1111, 0b1111_1100]).unwrap(),
        S2(SequenceOf::from(vec![true; 10]))
    );
}

#[test]
fn aper_rejects_surplus_constraint_values() {
    let error = aper::decode::<S1>(&[0xf0]).unwrap_err();
    assert!(matches!(
        *error.kind,
        DecodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));
    assert_eq!(aper::decode::<S1>(&[0x90]).unwrap(), S1(10));

    // APER aligns the sequence contents after the four-bit length determinant.
    let error = aper::decode::<S2>(&[0xf0, 0xff, 0xff]).unwrap_err();
    assert!(matches!(
        *error.kind,
        DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(16), .. }
    ));
    assert_eq!(
        aper::decode::<S2>(&[0x90, 0xff, 0xc0]).unwrap(),
        S2(SequenceOf::from(vec![true; 10]))
    );
}
