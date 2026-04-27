#[cfg(feature = "codec_per")]
use rasn::{error::EncodeErrorKind, prelude::*, uper};

#[cfg(feature = "codec_per")]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-500..=504"))]
pub struct Test(pub i16);

#[cfg(feature = "codec_per")]
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

#[cfg(feature = "codec_per")]
#[test]
fn value_too_negative() {
    let x = Test(-505);
    let res = uper::encode(&x).unwrap_err().kind;
    assert!(matches!(
        *res,
        EncodeErrorKind::ValueConstraintNotSatisfied { .. }
    ));
}
