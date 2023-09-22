use snafu::*;

use crate::de::Error;
use alloc::string::ToString;

use crate::types::variants::Variants;
use crate::types::Tag;

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
pub struct DecodeError {
    kind: Kind,
    backtrace: Backtrace,
}

impl DecodeError {
    #[must_use]
    pub fn range_exceeds_platform_width(needed: u32, present: u32) -> Self {
        Self::from(Kind::RangeExceedsPlatformWidth { needed, present })
    }
    #[must_use]
    pub fn integer_overflow(max_width: u32) -> Self {
        Self::from(Kind::IntegerOverflow { max_width })
    }

    #[must_use]
    pub fn type_not_extensible() -> Self {
        Self::from(Kind::TypeNotExtensible)
    }
    #[must_use]
    pub fn parser_fail(msg: alloc::string::String) -> Self {
        Self::from(Kind::Parser { msg })
    }

    #[must_use]
    pub fn required_extension_not_present(tag: crate::types::Tag) -> Self {
        Self::from(Kind::RequiredExtensionNotPresent { tag })
    }

    #[must_use]
    pub fn choice_index_exceeds_platform_width(needed: u32, present: u64) -> Self {
        Self::from(Kind::ChoiceIndexExceedsPlatformWidth { needed, present })
    }

    #[must_use]
    pub fn choice_index_not_found(index: usize, variants: Variants) -> Self {
        Self::from(Kind::ChoiceIndexNotFound { index, variants })
    }

    pub(crate) fn assert_tag(expected: Tag, actual: Tag) -> core::result::Result<(), DecodeError> {
        if expected != actual {
            Err(DecodeError::from(Kind::MismatchedTag { expected, actual }))
        } else {
            Ok(())
        }
    }

    pub(crate) fn assert_length(
        expected: usize,
        actual: usize,
    ) -> core::result::Result<(), DecodeError> {
        if expected != actual {
            Err(DecodeError::from(Kind::MismatchedLength {
                expected,
                actual,
            }))
        } else {
            Ok(())
        }
    }

    pub(crate) fn map_nom_err(error: nom::Err<nom::error::Error<&[u8]>>) -> DecodeError {
        let msg = match error {
            nom::Err::Incomplete(needed) => return DecodeError::incomplete(needed),
            err => alloc::format!("Parsing Failure: {}", err),
        };

        DecodeError::parser_fail(msg)
    }
}

impl From<Kind> for DecodeError {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            backtrace: Backtrace::generate(),
        }
    }
}

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Kind {
    /// Constructed encoding encountered but not allowed.
    ConstructedEncodingNotAllowed,
    #[snafu(display("choice index '{index}' did not match any variant"))]
    ChoiceIndexNotFound {
        /// The found index of the choice variant.
        index: usize,
        /// The variants checked for presence.
        variants: Variants,
    },
    #[snafu(display("integer range larger than possible to address on this platform. needed: {needed} present: {present}"))]
    ChoiceIndexExceedsPlatformWidth {
        /// Amount of bytes needed.
        needed: u32,
        /// Amount of bytes needed.
        present: u64,
    },
    #[snafu(display("Custom: {}", msg))]
    Custom {
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display("Duplicate field for `{}`", name))]
    DuplicateField {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("Expected maximum of {} items", length))]
    ExceedsMaxLength {
        /// The maximum length.
        length: num_bigint::BigUint,
    },
    #[snafu(display("Error when decoding field `{}`: {}", name, msg))]
    FieldError {
        /// The field's name.
        name: &'static str,
        msg: alloc::string::String,
    },
    #[snafu(display("Need more bytes to continue ({:?}).", needed))]
    Incomplete {
        /// Amount of bytes needed.
        needed: nom::Needed,
    },
    /// Indefinite length encountered but not allowed.
    IndefiniteLengthNotAllowed,
    /// The actual integer exceeded the expected width.
    #[snafu(display("Actual integer larger than expected {} bits", max_width))]
    IntegerOverflow {
        /// The maximum integer width.
        max_width: u32,
    },
    /// The bit string contains invalid bits.
    #[snafu(display("BitString contains an invalid amount of unused bits: {}", bits))]
    InvalidBitString {
        /// The amount of invalid bits.
        bits: u8,
    },
    /// BOOL value is not `0` or `0xFF`. Applies: BER/OER/PER?
    InvalidBool,
    /// OBJECT IDENTIFIER with missing or corrupt root nodes.
    InvalidObjectIdentifier,
    /// Invalid UTF-8 data.
    InvalidUtf8,
    /// Invalid date.
    InvalidDate,
    /// The length does not match what was expected.
    #[snafu(display("Expected {:?} bytes, actual length: {:?}", expected, actual))]
    MismatchedLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },
    /// The tag does not match what was expected.
    #[snafu(display("Expected {:?} tag, actual tag: {:?}", expected, actual))]
    MismatchedTag {
        /// The expected tag.
        expected: Tag,
        /// The actual tag.
        actual: Tag,
    },
    #[snafu(display("Missing field `{}`", name))]
    MissingField {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("integer range larger than possible to address on this platform. needed: {needed} present: {present}"))]
    RangeExceedsPlatformWidth {
        /// Amount of bytes needed.
        needed: u32,
        /// Amount of bytes needed.
        present: u32,
    },
    #[snafu(display("Extension with class `{}` and tag `{}` required, but not present", tag.class, tag.value))]
    RequiredExtensionNotPresent { tag: crate::types::Tag },
    // #[snafu(display("Need more bytes to continue ({:?}).", needed))]
    // ExceedsMaxLength {
    //     /// Amount of bytes needed.
    //     needed: num_bigint::BigUint,
    // },
    #[snafu(display("Error in Parser: {}", msg))]
    Parser {
        /// The error's message.
        msg: alloc::string::String,
    },
    // #[snafu(display("Error in wrapped BER: {}", source))]
    // Ber {
    //     /// The error's message.
    //     source: crate::ber::de::Error,
    // },
    #[snafu(display("No valid choice for `{}`", name))]
    NoValidChoice {
        /// The field's name.
        name: &'static str,
    },

    #[snafu(display("Attempted to decode extension on non-extensible type"))]
    TypeNotExtensible,
    /// Unexpected extra data found.
    #[snafu(display("Unexpected extra data found: length `{}` bytes", length))]
    UnexpectedExtraData {
        /// The amount of garbage data.
        length: usize,
    },
}

// impl Kind {
//     fn to_error() {}
// }

impl crate::de::Error for DecodeError {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::from(Kind::Custom {
            msg: msg.to_string(),
        })
    }

    fn incomplete(needed: nom::Needed) -> Self {
        Self::from(Kind::Incomplete { needed })
    }

    fn exceeds_max_length(length: num_bigint::BigUint) -> Self {
        Self::from(Kind::ExceedsMaxLength { length })
    }

    fn missing_field(name: &'static str) -> Self {
        Self::from(Kind::MissingField { name })
    }

    fn no_valid_choice(name: &'static str) -> Self {
        Self::from(Kind::NoValidChoice { name })
    }

    fn field_error<D: core::fmt::Display>(name: &'static str, error: D) -> Self {
        Self::from(Kind::FieldError {
            name,
            msg: error.to_string(),
        })
    }

    fn duplicate_field(name: &'static str) -> Self {
        Self::from(Kind::DuplicateField { name })
    }
}

impl From<nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>>
    for DecodeError
{
    fn from(
        error: nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>,
    ) -> Self {
        let msg = match error {
            nom::Err::Incomplete(needed) => return Self::from(Kind::Incomplete { needed }),
            err => alloc::format!("Parsing Failure: {}", err),
        };

        Self::from(Kind::Parser { msg })
    }
}
