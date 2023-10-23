use snafu::{Backtrace, GenerateImplicitData, Snafu};

use crate::de::Error;
use crate::Codec;
use alloc::string::ToString;

use crate::types::variants::Variants;
use crate::types::Tag;

#[derive(Debug)]
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

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
#[allow(clippy::module_name_repetitions)]
pub struct DecodeError {
    kind: Kind,
    codec: Codec,
    backtrace: Backtrace,
}

impl DecodeError {
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

    pub(crate) fn assert_tag(
        expected: Tag,
        actual: Tag,
        codec: Codec,
    ) -> core::result::Result<(), DecodeError> {
        if expected == actual {
            Ok(())
        } else {
            Err(DecodeError::from_kind(
                Kind::MismatchedTag { expected, actual },
                codec,
            ))
        }
    }

    pub(crate) fn assert_length(
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

    pub(crate) fn map_nom_err(
        error: nom::Err<nom::error::Error<&[u8]>>,
        codec: Codec,
    ) -> DecodeError {
        let msg = match error {
            nom::Err::Incomplete(needed) => return DecodeError::incomplete(needed, codec),
            err => alloc::format!("Parsing Failure: {}", err),
        };

        DecodeError::parser_fail(msg, codec)
    }
    pub(crate) fn from_kind(kind: Kind, codec: Codec) -> Self {
        Self {
            kind,
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub(crate) fn from_codec_kind(inner: CodecDecodeError) -> Self {
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

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Kind {
    #[snafu(display("Wrapped codec-specific decode error"))]
    CodecSpecific { inner: CodecDecodeError },

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
    #[snafu(display("Invalid discriminant for enumerated type."))]
    InvalidDiscriminant,
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
    IncorrectItemNumberInSequence {
        /// The expected item number.
        expected: usize,
        /// The actual item number.
        actual: usize,
    },
    /// Indefinite length encountered but not allowed.
    IndefiniteLengthNotAllowed,
    /// The actual integer exceeded the expected width.
    #[snafu(display("Actual integer larger than expected {} bits", max_width))]
    IntegerOverflow {
        /// The maximum integer width.
        max_width: u32,
    },
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed { msg: alloc::string::String },
    /// The bit string contains invalid bits.
    #[snafu(display("BitString contains an invalid amount of unused bits: {}", bits))]
    InvalidBitString {
        /// The amount of invalid bits.
        bits: u8,
    },
    /// BOOL value is not `0` or `0xFF`. Applies: BER/OER/PER?
    #[snafu(display("Bool value is not `0` or `0xFF` as canonical requires."))]
    InvalidBool,
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
        /// Universal tag of the string type.
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
}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum BerDecodeErrorKind {
    #[snafu(display("Invalid constructed identifier for ASN.1 value: not primitive."))]
    InvalidConstructedIdentifier,
    /// Invalid date.
    #[snafu(display("Invalid date string: {}", msg))]
    InvalidDate { msg: alloc::string::String },
    #[snafu(display("Invalid object identifier with missing or corrupt root nodes."))]
    InvalidObjectIdentifier,
}

impl BerDecodeErrorKind {
    pub fn invalid_constructed_identifier() -> CodecDecodeError {
        CodecDecodeError::Ber(Self::InvalidConstructedIdentifier)
    }
    pub fn invalid_date(msg: alloc::string::String) -> CodecDecodeError {
        CodecDecodeError::Ber(Self::InvalidDate { msg })
    }
}

#[derive(Snafu, Debug)]
// TODO check if there codec-specific errors here
#[snafu(visibility(pub(crate)))]
pub enum CerDecodeErrorKind {}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum DerDecodeErrorKind {
    #[snafu(display("Constructed encoding encountered but not allowed."))]
    ConstructedEncodingNotAllowed,
}

// TODO check if there codec-specific errors here
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum UperDecodeErrorKind {}

// TODO check if there codec-specific errors here
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
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
}

// impl From<nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>>
//     for DecodeError
// {
//     fn from(
//         error: nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>,
//     ) -> Self {
//         let msg = match error {
//             nom::Err::Incomplete(needed) => return Self::from_kind(Kind::Incomplete { needed }, codec),
//             err => alloc::format!("Parsing Failure: {}", err),
//         };
//
//         Self::from(Kind::Parser { msg })
//     }
// }
