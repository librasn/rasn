//! Explicit extension additions distinguish absence from malformed known values.
use rasn::prelude::*;

macro_rules! addition_types {
    ($optional:ident, $defaulted:ident, $required:ident, $tag:meta, [$($decode:ident)?] $(, $constraint:meta)?) => {
        #[derive(AsnType, Debug, $($decode,)? Encode, PartialEq)]
        #[non_exhaustive]
        struct $optional {
            #[rasn(tag(context, 0))]
            root: u32,
            #[rasn(extension_addition, $tag)]
            $(#[$constraint])?
            addition: Option<u32>,
        }

        #[derive(AsnType, Debug, $($decode,)? Encode, PartialEq)]
        #[non_exhaustive]
        struct $defaulted {
            #[rasn(tag(context, 0))]
            root: u32,
            #[rasn(extension_addition, $tag, default = "default_addition")]
            $(#[$constraint])?
            addition: u32,
        }

        #[derive(AsnType, Debug, $($decode,)? Encode, PartialEq)]
        #[non_exhaustive]
        struct $required {
            #[rasn(tag(context, 0))]
            root: u32,
            #[rasn(extension_addition, $tag)]
            $(#[$constraint])?
            addition: u32,
        }
    };
}

addition_types!(
    OptionalAddition,
    DefaultedAddition,
    RequiredAddition,
    tag(explicit(context, 1)),
    [Decode]
);
addition_types!(
    ConstrainedOptionalAddition,
    ConstrainedDefaultedAddition,
    ConstrainedRequiredAddition,
    tag(explicit(context, 1)),
    [Decode],
    rasn(value("0..=255"))
);

fn default_addition() -> u32 {
    7
}

#[derive(AsnType, Debug, Decode, Encode, PartialEq)]
#[non_exhaustive]
struct LaterAddition {
    #[rasn(tag(context, 0))]
    root: u32,
    #[rasn(extension_addition, tag(explicit(context, 1)))]
    first: Option<u32>,
    #[rasn(extension_addition, tag(context, 2))]
    second: Option<u32>,
}

#[derive(AsnType, Debug, Decode, Encode, PartialEq)]
#[rasn(set)]
#[non_exhaustive]
struct ExplicitSetAddition {
    #[rasn(tag(context, 0))]
    root: u32,
    #[rasn(extension_addition, tag(explicit(context, 1)))]
    addition: Option<u32>,
}

macro_rules! check_round_trip {
    ($value:expr, $ty:ty, $($codec:ident),+ $(,)?) => {
        $(
            let encoded = rasn::$codec::encode(&$value).unwrap();
            assert_eq!(
                rasn::$codec::decode::<$ty>(&encoded).unwrap(),
                $value,
                stringify!($codec)
            );
        )+
    };
}

fn round_trip_sequence<T: AsnType + Encode + Decode + core::fmt::Debug + PartialEq>(value: T) {
    check_round_trip!(value, T, ber, cer, der, jer, xer, avn);
}

#[test]
fn optional_explicit_additions_round_trip_present_and_absent() {
    for addition in [None, Some(9)] {
        round_trip_sequence(OptionalAddition { root: 1, addition });
        round_trip_sequence(ConstrainedOptionalAddition { root: 1, addition });
    }
}

#[test]
fn defaulted_explicit_additions_round_trip_default_and_non_default() {
    for addition in [default_addition(), 9] {
        round_trip_sequence(DefaultedAddition { root: 1, addition });
        round_trip_sequence(ConstrainedDefaultedAddition { root: 1, addition });
    }
}

#[test]
fn required_explicit_additions_round_trip() {
    round_trip_sequence(RequiredAddition {
        root: 1,
        addition: 9,
    });
    round_trip_sequence(ConstrainedRequiredAddition {
        root: 1,
        addition: 9,
    });
}

#[test]
fn an_absent_explicit_addition_does_not_consume_the_next_field() {
    round_trip_sequence(LaterAddition {
        root: 1,
        first: None,
        second: Some(9),
    });
    assert_eq!(
        rasn::ber::decode::<LaterAddition>(&[0x30, 6, 0x80, 1, 1, 0x82, 1, 9]).unwrap(),
        LaterAddition {
            root: 1,
            first: None,
            second: Some(9),
        }
    );
}

#[test]
fn set_explicit_additions_round_trip_present_and_absent() {
    for addition in [None, Some(9)] {
        let value = ExplicitSetAddition { root: 1, addition };
        check_round_trip!(value, ExplicitSetAddition, ber, cer, der, jer, avn);
    }
}

addition_types!(
    OptionalPeer,
    DefaultedPeer,
    RequiredPeer,
    tag(context, 1),
    []
);
addition_types!(
    ConstrainedOptionalPeer,
    ConstrainedDefaultedPeer,
    ConstrainedRequiredPeer,
    tag(context, 1),
    [],
    rasn(value("0..=255"))
);

#[derive(AsnType, Encode)]
#[non_exhaustive]
struct LaterImplicitPeer {
    #[rasn(tag(context, 0))]
    root: u32,
    #[rasn(extension_addition, tag(context, 1))]
    first: Option<u32>,
    #[rasn(extension_addition, tag(context, 2))]
    second: Option<u32>,
}

#[test]
fn explicit_additions_decode_valid_per_and_oer_extension_encodings() {
    // PER and OER do not encode explicit tags. Equivalent implicit schemas
    // supply valid extension bitmaps and open types, testing the shared derive
    // decoder independently of the explicit-extension encoder.
    macro_rules! check {
        ($optional_peer:ident, $defaulted_peer:ident, $required_peer:ident,
         $optional:ident, $defaulted:ident, $required:ident, $codec:ident) => {
            for addition in [None, Some(9)] {
                let encoded = rasn::$codec::encode(&$optional_peer { root: 1, addition }).unwrap();
                assert_eq!(
                    rasn::$codec::decode::<$optional>(&encoded).unwrap(),
                    $optional { root: 1, addition },
                    stringify!($codec)
                );
                let default_value = addition.unwrap_or_else(default_addition);
                let default_encoded = rasn::$codec::encode(&$defaulted_peer {
                    root: 1,
                    addition: default_value,
                })
                .unwrap();
                assert_eq!(
                    rasn::$codec::decode::<$defaulted>(&default_encoded).unwrap(),
                    $defaulted {
                        root: 1,
                        addition: default_value,
                    },
                    stringify!($codec)
                );
                if let Some(addition) = addition {
                    let encoded =
                        rasn::$codec::encode(&$required_peer { root: 1, addition }).unwrap();
                    assert_eq!(
                        rasn::$codec::decode::<$required>(&encoded).unwrap(),
                        $required { root: 1, addition },
                        stringify!($codec)
                    );
                } else {
                    let error = rasn::$codec::decode::<$required>(&encoded).unwrap_err();
                    assert_error_field(error, concat!(stringify!($required), ".addition"));
                }
            }
        };
    }
    macro_rules! check_codec {
        ($codec:ident) => {
            check!(
                OptionalPeer,
                DefaultedPeer,
                RequiredPeer,
                OptionalAddition,
                DefaultedAddition,
                RequiredAddition,
                $codec
            );
            check!(
                ConstrainedOptionalPeer,
                ConstrainedDefaultedPeer,
                ConstrainedRequiredPeer,
                ConstrainedOptionalAddition,
                ConstrainedDefaultedAddition,
                ConstrainedRequiredAddition,
                $codec
            );
            let encoded = rasn::$codec::encode(&LaterImplicitPeer {
                root: 1,
                first: None,
                second: Some(9),
            })
            .unwrap();
            assert_eq!(
                rasn::$codec::decode::<LaterAddition>(&encoded).unwrap(),
                LaterAddition {
                    root: 1,
                    first: None,
                    second: Some(9)
                },
                stringify!($codec)
            );
        };
    }
    check_codec!(aper);
    check_codec!(uper);
    check_codec!(oer);
    check_codec!(coer);
}

fn assert_field_error<T: Decode + core::fmt::Debug>(input: &[u8], name: &str) {
    let error = rasn::ber::decode::<T>(input).unwrap_err();
    assert_error_field(error, name);
}

fn assert_error_field(error: rasn::error::DecodeError, name: &str) {
    match *error.kind {
        rasn::error::DecodeErrorKind::FieldError {
            name: actual_name, ..
        } => assert_eq!(actual_name, name),
        kind => panic!("expected error for {name}, got {kind:?}"),
    }
}

#[test]
fn absent_required_explicit_additions_report_the_field_name() {
    let input = &[0x30, 3, 0x80, 1, 1];
    assert_field_error::<RequiredAddition>(input, "RequiredAddition.addition");
    assert_field_error::<ConstrainedRequiredAddition>(
        input,
        "ConstrainedRequiredAddition.addition",
    );
}

#[test]
fn malformed_present_text_additions_report_the_field_name() {
    assert_error_field(
        rasn::jer::decode::<OptionalAddition>(r#"{"root":1,"addition":true}"#).unwrap_err(),
        "OptionalAddition.addition",
    );
    assert_error_field(
        rasn::jer::decode::<DefaultedAddition>(r#"{"root":1,"addition":true}"#).unwrap_err(),
        "DefaultedAddition.addition",
    );
    assert_error_field(
        rasn::xer::decode::<OptionalAddition>(
            b"<OptionalAddition><root>1</root><addition>invalid</addition></OptionalAddition>",
        )
        .unwrap_err(),
        "OptionalAddition.addition",
    );
    assert_error_field(
        rasn::xer::decode::<DefaultedAddition>(
            b"<DefaultedAddition><root>1</root><addition>invalid</addition></DefaultedAddition>",
        )
        .unwrap_err(),
        "DefaultedAddition.addition",
    );
}

#[test]
fn malformed_known_explicit_additions_propagate_their_errors() {
    // Both wrappers carry BOOLEAN rather than the expected INTEGER. An unknown
    // trailing addition must not turn the known field's failure into absence.
    for wrapper in [
        &[0xa1, 3, 1, 1, 0xff][..],
        &[0xa1, 0x80, 1, 1, 0xff, 0, 0][..],
    ] {
        for with_unknown in [false, true] {
            let mut contents = vec![0x80, 1, 1];
            contents.extend_from_slice(wrapper);
            if with_unknown {
                contents.extend_from_slice(&[0x82, 1, 3]);
            }
            let mut input = vec![0x30, contents.len() as u8];
            input.extend_from_slice(&contents);
            assert_field_error::<OptionalAddition>(&input, "OptionalAddition.addition");
            assert_field_error::<ConstrainedOptionalAddition>(
                &input,
                "ConstrainedOptionalAddition.addition",
            );
            assert_field_error::<DefaultedAddition>(&input, "DefaultedAddition.addition");
            assert_field_error::<ConstrainedDefaultedAddition>(
                &input,
                "ConstrainedDefaultedAddition.addition",
            );
            assert_field_error::<RequiredAddition>(&input, "RequiredAddition.addition");
            assert_field_error::<ConstrainedRequiredAddition>(
                &input,
                "ConstrainedRequiredAddition.addition",
            );
        }
    }
}

#[test]
fn malformed_known_explicit_wrappers_are_not_absent() {
    for wrapper in [
        // Truncated contents and truncated tag/length headers.
        &[0xa1, 3, 2, 1][..],
        &[0xa1][..],
        &[0xa1, 0x82, 1][..],
        // An indefinite wrapper without its end-of-contents marker.
        &[0xa1, 0x80, 2, 1, 9][..],
    ] {
        let mut input = vec![0x30, (3 + wrapper.len()) as u8, 0x80, 1, 1];
        input.extend_from_slice(wrapper);
        assert_field_error::<OptionalAddition>(&input, "OptionalAddition.addition");
        assert_field_error::<DefaultedAddition>(&input, "DefaultedAddition.addition");
    }
}
