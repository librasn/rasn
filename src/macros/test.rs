//! Macros used for automating common testing functionality.

/// Compares $value with $expected after encoding and then decoding it into
/// $codec to ensure no information was lost and there are no codec errors with
/// the expected valid value.
macro_rules! round_trip {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        let value: $typ = $value;
        let expected: &[u8] = $expected;
        let result = crate::$codec::encode(&value);
        let actual_encoding = match result {
            Ok(actual_encoding) => {
                pretty_assertions::assert_eq!(expected, &*actual_encoding);
                actual_encoding
            }
            Err(error) => {
                panic!("Unexpected encoding error: {:?}", error);
            }
        };
        let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();
        pretty_assertions::assert_eq!(value, decoded_value);
    }};
}

/// unwrap_err but includes the encoding in the error message.
macro_rules! encode_error {
    ($codec:ident, $typ:ty, $value:expr) => {{
        let value: $typ = $value;
        let result = crate::$codec::encode(&value);
        match result {
            Ok(actual_encoding) => {
                panic!(
                    "Expected an encoding error but got a valid encoding: {:?}",
                    &*actual_encoding
                );
            }
            Err(_) => {
                // Expected an encoding error, so we're good!
            }
        }
    }};
}

/// unwrap_err for decoding.
macro_rules! decode_error {
    ($codec:ident, $typ:ty, $value:expr) => {{
        match crate::$codec::decode::<$typ>($value) {
            Ok(_) => {
                panic!("Unexpected decoding success!");
            }
            Err(_) => {
                // Expected a decoding error, so we're good!
            }
        }
    }};
}

/// unwrap for decoding.
macro_rules! decode_ok {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        match crate::$codec::decode::<$typ>($value) {
            Ok(result) => {
                pretty_assertions::assert_eq!(result, $expected);
            }
            Err(e) => {
                panic!("Unexpected decoding failure!: {e}");
            }
        }
    }};
}

/// Same functionality [round_trip] but with a constraints object for testing
/// constrained types.
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

/// Same functionality [encode_error] but with a constraints object for testing
/// constrained types.
macro_rules! encode_error_with_constraints {
    ($codec:ident, $typ:ty, $constraints:expr, $value:expr) => {{
        let value: $typ = $value;
        let result = crate::$codec::encode_with_constraints($constraints, &value);
        match result {
            Ok(actual_encoding) => {
                panic!(
                    "Expected an encoding error but got a valid encoding: {:?}",
                    &*actual_encoding
                );
            }
            Err(_) => {
                // Expected an encoding error, so we're good!
            }
        }
    }};
}
