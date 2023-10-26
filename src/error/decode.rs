use super::strings::PermittedAlphabetError;
use alloc::string::ToString;

use snafu::{Backtrace, GenerateImplicitData, Snafu};

use crate::de::Error;
use crate::types::variants::Variants;
use crate::types::Tag;
use crate::Codec;

/// Variants for every codec-specific `DecodeError` kind.
#[derive(Debug)]
#[non_exhaustive]
pub enum CodecDecodeError {
    Ber(BerDecodeErrorKind),
    Cer(CerDecodeErrorKind),
    Der(DerDecodeErrorKind),
    Uper(UperDecodeErrorKind),
    Aper(AperDecodeErrorKind),
}

macro_rules! impl_from {
    ($variant:ident, $error_kind:ty) => {
        impl From<$error_kind> for DecodeError {
            fn from(error: $error_kind) -> Self {
                Self::from_codec_kind(CodecDecodeError::$variant(error))
            }
        }
    };
}

// implement From for each variant of CodecDecodeError into DecodeError
impl_from!(Ber, BerDecodeErrorKind);
impl_from!(Cer, CerDecodeErrorKind);
impl_from!(Der, DerDecodeErrorKind);
impl_from!(Uper, UperDecodeErrorKind);
impl_from!(Aper, AperDecodeErrorKind);

impl From<CodecDecodeError> for DecodeError {
    fn from(error: CodecDecodeError) -> Self {
        Self::from_codec_kind(error)
    }
}

/// An error type for failed decoding for every decoder.
/// Abstracts over the different generic and codec-specific errors.
///
/// `kind` field is used to determine the kind of error that occurred.
/// `codec` field is used to determine the codec that failed.
/// `backtrace` field is used to determine the backtrace of the error.
///
/// There is `Kind::CodecSpecific` variant which wraps the codec-specific
/// errors as `CodecEncodeError` type.
///
/// # Example
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
#[allow(clippy::module_name_repetitions)]
pub struct DecodeError {
    pub kind: Kind,
    pub codec: Codec,
    pub backtrace: Backtrace,
}

impl DecodeError {
    #[must_use]
    pub fn alphabet_constraint_not_satisfied(reason: PermittedAlphabetError, codec: Codec) -> Self {
        Self::from_kind(Kind::AlphabetConstraintNotSatisfied { reason }, codec)
    }
    #[must_use]
    pub fn range_exceeds_platform_width(needed: u32, present: u32, codec: Codec) -> Self {
        Self::from_kind(Kind::RangeExceedsPlatformWidth { needed, present }, codec)
    }
    #[must_use]
    pub fn fixed_string_conversion_failed(
        tag: Tag,
        actual: usize,
        expected: usize,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            Kind::FixedStringConversionFailed {
                tag,
                actual,
                expected,
            },
            codec,
        )
    }
    #[must_use]
    pub fn incorrect_item_number_in_sequence(expected: usize, actual: usize, codec: Codec) -> Self {
        Self::from_kind(
            Kind::IncorrectItemNumberInSequence { expected, actual },
            codec,
        )
    }
    #[must_use]
    pub fn integer_overflow(max_width: u32, codec: Codec) -> Self {
        Self::from_kind(Kind::IntegerOverflow { max_width }, codec)
    }
    #[must_use]
    pub fn integer_type_conversion_failed(msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(Kind::IntegerTypeConversionFailed { msg }, codec)
    }
    #[must_use]
    pub fn invalid_bit_string(bits: u8, codec: Codec) -> Self {
        Self::from_kind(Kind::InvalidBitString { bits }, codec)
    }
    #[must_use]
    pub fn missing_tag_class_or_value_in_sequence_or_set(
        class: crate::types::Class,
        value: u32,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            Kind::MissingTagClassOrValueInSequenceOrSet { class, value },
            codec,
        )
    }

    #[must_use]
    pub fn type_not_extensible(codec: Codec) -> Self {
        Self::from_kind(Kind::TypeNotExtensible, codec)
    }
    #[must_use]
    pub fn parser_fail(msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(Kind::Parser { msg }, codec)
    }

    #[must_use]
    pub fn required_extension_not_present(tag: crate::types::Tag, codec: Codec) -> Self {
        Self::from_kind(Kind::RequiredExtensionNotPresent { tag }, codec)
    }
    #[must_use]
    pub fn enumeration_index_not_found(index: usize, extended_list: bool, codec: Codec) -> Self {
        Self::from_kind(
            Kind::EnumerationIndexNotFound {
                index,
                extended_list,
            },
            codec,
        )
    }
    #[must_use]
    pub fn choice_index_exceeds_platform_width(needed: u32, present: u64, codec: Codec) -> Self {
        Self::from_kind(
            Kind::ChoiceIndexExceedsPlatformWidth { needed, present },
            codec,
        )
    }

    #[must_use]
    pub fn choice_index_not_found(index: usize, variants: Variants, codec: Codec) -> Self {
        Self::from_kind(Kind::ChoiceIndexNotFound { index, variants }, codec)
    }
    #[must_use]
    pub fn string_conversion_failed(tag: Tag, msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(Kind::StringConversionFailed { tag, msg }, codec)
    }
    #[must_use]
    pub fn unexpected_extra_data(length: usize, codec: Codec) -> Self {
        Self::from_kind(Kind::UnexpectedExtraData { length }, codec)
    }

    pub fn assert_length(
        expected: usize,
        actual: usize,
        codec: Codec,
    ) -> core::result::Result<(), DecodeError> {
        if expected == actual {
            Ok(())
        } else {
            Err(DecodeError::from_kind(
                Kind::MismatchedLength { expected, actual },
                codec,
            ))
        }
    }

    pub fn map_nom_err<T: core::fmt::Debug>(
        error: nom::Err<nom::error::Error<T>>,
        codec: Codec,
    ) -> DecodeError {
        let msg = match error {
            nom::Err::Incomplete(needed) => return DecodeError::incomplete(needed, codec),
            err => alloc::format!("Parsing Failure: {err}"),
        };
        DecodeError::parser_fail(msg, codec)
    }
    #[must_use]
    pub fn from_kind(kind: Kind, codec: Codec) -> Self {
        Self {
            kind,
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    fn from_codec_kind(inner: CodecDecodeError) -> Self {
        let codec = match inner {
            CodecDecodeError::Ber(_) => crate::Codec::Ber,
            CodecDecodeError::Cer(_) => crate::Codec::Cer,
            CodecDecodeError::Der(_) => crate::Codec::Der,
            CodecDecodeError::Uper(_) => crate::Codec::Uper,
            CodecDecodeError::Aper(_) => crate::Codec::Aper,
        };
        Self {
            kind: Kind::CodecSpecific { inner },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
}

/// `DecodeError` kinds which are common for all codecs.
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[derive(Debug)]
#[non_exhaustive]
pub enum Kind {
    #[snafu(display("Alphabet constraint not satisfied {}", reason))]
    AlphabetConstraintNotSatisfied { reason: PermittedAlphabetError },
    #[snafu(display("Wrapped codec-specific decode error"))]
    CodecSpecific { inner: CodecDecodeError },

    #[snafu(display(
        "Enumeration index '{}' did not match any variant. Extended list: {}",
        index,
        extended_list
    ))]
    EnumerationIndexNotFound {
        /// The found index of the enumerated variant.
        index: usize,
        /// Whether the index was checked from the extended variants.
        extended_list: bool,
    },
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
    #[snafu(display(
        "Invalid item number in Sequence: expected {}, actual {}",
        expected,
        actual
    ))]
    IncorrectItemNumberInSequence {
        /// The expected item number.
        expected: usize,
        /// The actual item number.
        actual: usize,
    },
    #[snafu(display("Actual integer larger than expected {} bits", max_width))]
    IntegerOverflow {
        /// The maximum integer width.
        max_width: u32,
    },
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed { msg: alloc::string::String },
    #[snafu(display("BitString contains an invalid amount of unused bits: {}", bits))]
    InvalidBitString {
        /// The amount of invalid bits.
        bits: u8,
    },
    /// BOOL value is not `0` or `0xFF`. Applies: BER/OER/PER?
    #[snafu(display(
        "Bool value is not `0` or `0xFF` as canonical requires. Actual: {}",
        value
    ))]
    InvalidBool { value: u8 },
    /// The length does not match what was expected.
    #[snafu(display("Expected {:?} bytes, actual length: {:?}", expected, actual))]
    MismatchedLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    #[snafu(display("Missing field `{}`", name))]
    MissingField {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("Expected class: {}, value: {} in sequence or set Missing tag class or value in sequence or set", class, value))]
    MissingTagClassOrValueInSequenceOrSet {
        /// The field's name.
        class: crate::types::Class,
        value: u32,
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
    #[snafu(display("Error in Parser: {}", msg))]
    Parser {
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display(
        "Failed to convert byte array into valid ASN.1 string. String type as tag: {} Error: {}",
        tag,
        msg
    ))]
    StringConversionFailed {
        /// Universal tag of the string type.
        tag: Tag,
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display(
    "Failed to convert byte array into valid fixed-sized ASN.1 string. String type as tag: {}, actual: {}, expected: {}",
    tag,
    actual,
    expected
    ))]
    FixedStringConversionFailed {
        /// Tag of the string type.
        tag: Tag,
        /// Expected length
        expected: usize,
        /// Actual length
        actual: usize,
    },
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
    #[snafu(display("Unknown field with index {} and tag {}", index, tag))]
    UnknownField { index: usize, tag: Tag },
}

/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for BER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum BerDecodeErrorKind {
    #[snafu(display("Discriminant value '{}' did not match any variant", discriminant))]
    DiscriminantValueNotFound {
        /// The found value of the discriminant
        discriminant: isize,
    },
    #[snafu(display("Indefinite length encountered but not allowed."))]
    IndefiniteLengthNotAllowed,
    #[snafu(display("Invalid constructed identifier for ASN.1 value: not primitive."))]
    InvalidConstructedIdentifier,
    /// Invalid date.
    #[snafu(display("Invalid date string: {}", msg))]
    InvalidDate { msg: alloc::string::String },
    #[snafu(display("Invalid object identifier with missing or corrupt root nodes."))]
    InvalidObjectIdentifier,
    /// The tag does not match what was expected.
    #[snafu(display("Expected {:?} tag, actual tag: {:?}", expected, actual))]
    MismatchedTag {
        /// The expected tag.
        expected: Tag,
        /// The actual tag.
        actual: Tag,
    },
}

impl BerDecodeErrorKind {
    #[must_use]
    pub fn invalid_date(msg: alloc::string::String) -> CodecDecodeError {
        CodecDecodeError::Ber(Self::InvalidDate { msg })
    }
    pub fn assert_tag(expected: Tag, actual: Tag) -> core::result::Result<(), DecodeError> {
        if expected == actual {
            Ok(())
        } else {
            Err(Self::MismatchedTag { expected, actual }.into())
        }
    }
}
// TODO check if there are more codec-specific errors here
/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for CER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum CerDecodeErrorKind {}

/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for DER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum DerDecodeErrorKind {
    #[snafu(display("Constructed encoding encountered but not allowed."))]
    ConstructedEncodingNotAllowed,
}

// TODO check if there codec-specific errors here
/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for UPER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum UperDecodeErrorKind {}

// TODO check if there codec-specific errors here
/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for APER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum AperDecodeErrorKind {}

impl crate::de::Error for DecodeError {
    fn custom<D: core::fmt::Display>(msg: D, codec: Codec) -> Self {
        Self::from_kind(
            Kind::Custom {
                msg: msg.to_string(),
            },
            codec,
        )
    }

    fn incomplete(needed: nom::Needed, codec: Codec) -> Self {
        Self::from_kind(Kind::Incomplete { needed }, codec)
    }

    fn exceeds_max_length(length: num_bigint::BigUint, codec: Codec) -> Self {
        Self::from_kind(Kind::ExceedsMaxLength { length }, codec)
    }

    fn missing_field(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(Kind::MissingField { name }, codec)
    }

    fn no_valid_choice(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(Kind::NoValidChoice { name }, codec)
    }

    fn field_error<D: core::fmt::Display>(name: &'static str, error: D, codec: Codec) -> Self {
        Self::from_kind(
            Kind::FieldError {
                name,
                msg: error.to_string(),
            },
            codec,
        )
    }

    fn duplicate_field(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(Kind::DuplicateField { name }, codec)
    }
    fn unknown_field(index: usize, tag: Tag, codec: Codec) -> Self {
        Self::from_kind(Kind::UnknownField { index, tag }, codec)
    }
}
