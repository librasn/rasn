/// Test that all errors are public and structure is stable
#[test]
#[allow(unused_imports, clippy::items_after_statements)]
fn test_error_imports() {
    use rasn::error::strings::{
        InvalidBmpString, InvalidGeneralString, InvalidGraphicString, InvalidIA5String,
        InvalidNumericString, InvalidPrintableString, InvalidRestrictedString,
        InvalidTeletexString, InvalidVisibleString, PermittedAlphabetError,
    };
    _ = PermittedAlphabetError::Other {
        message: String::from("Test"),
    };

    use rasn::error::BerDecodeErrorKind;
    use rasn::error::BerEncodeErrorKind;
    use rasn::error::DecodeError;
    use rasn::error::DecodeErrorKind;
    use rasn::error::DerDecodeErrorKind;
    use rasn::error::EncodeError;
    use rasn::error::EncodeErrorKind;
}
