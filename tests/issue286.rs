use rasn::{error::EncodeErrorKind, prelude::*, uper};

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-500..=504"))]
pub struct Test(pub i16);

#[test]
fn value_too_positive() {
    let x = Test(505);
    let res = uper::encode(&x).unwrap_err().kind;
    assert!(matches!(
        *res,
        EncodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));

    let x = Test(815);
    let res = uper::encode(&x).unwrap_err().kind;
    assert!(matches!(
        *res,
        EncodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));
}

#[test]
fn value_too_negative() {
    let x = Test(-505);
    let res = uper::encode(&x).unwrap_err().kind;
    assert!(matches!(
        *res,
        EncodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));
}
