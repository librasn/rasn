//! Error types associated with decoding from ASN.1 codecs.

use core::num::ParseIntError;

use super::strings::PermittedAlphabetError;
use alloc::{boxed::Box, string::ToString};

use snafu::Snafu;
#[cfg(feature = "backtraces")]
use snafu::{Backtrace, GenerateImplicitData};

use crate::de::Error;
use crate::types::{constraints::Bounded, variants::Variants, Tag};
use crate::Codec;
use num_bigint::BigInt;

/// Variants for every codec-specific `DecodeError` kind.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CodecDecodeError {
    Ber(BerDecodeErrorKind),
    Cer(CerDecodeErrorKind),
    Der(DerDecodeErrorKind),
    Uper(UperDecodeErrorKind),
    Aper(AperDecodeErrorKind),
    Jer(JerDecodeErrorKind),
    Oer(OerDecodeErrorKind),
    Coer(CoerDecodeErrorKind),
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
impl_from!(Jer, JerDecodeErrorKind);
impl_from!(Oer, OerDecodeErrorKind);
impl_from!(Coer, CoerDecodeErrorKind);

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
/// ```rust
/// use nom::Needed;
/// use rasn::{Codec, error::DecodeErrorKind, prelude::*};
///
/// #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
/// #[rasn(delegate)]
/// struct MyString(pub VisibleString);
///
/// fn main() {
///     // Hello, World! in decimal bytes with trailing zeros
///     // Below sample requires that `backtraces` feature is enabled
///     let hello_data = vec![
///         13, 145, 151, 102, 205, 235, 16, 119, 223, 203, 102, 68, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///         0,
///     ];
///     // Initially parse the first 2 bytes for Error demonstration purposes
///     let mut total = 2;
///
///     loop {
///         let decoded = Codec::Uper.decode_from_binary::<MyString>(&hello_data[0..hello_data.len().min(total)]);
///         match decoded {
///             Ok(succ) => {
///                 println!("Successful decoding!");
///                 println!("Decoded string: {}", succ.0);
///                 break;
///             }
///             Err(e) => {
///                 // e is DecodeError, kind is boxed
///                 match *e.kind {
///                     DecodeErrorKind::Incomplete { needed } => {
///                         println!("Codec error source: {}", e.codec);
///                         println!("Error kind: {}", e.kind);
///                         // Here you need to know, that VisibleString has width of 7 bits and UPER parses input
///                         // as bits, if you want to build logic around it, and feed exactly the correct amount of data.
///                         // Usually you might need to just provide one byte at time instead when something is missing, since
///                         // inner logic might not be known to you, and data structures can get complex.
///                         total += match needed {
///                             Needed::Size(n) => {
///                                 let missing_bytes = n.get() / 7;
///                                 missing_bytes
///
///                             }
///                             _ => {
///                                 #[cfg(feature = "backtraces")]
///                                 println!("Backtrace:\n{:?}", e.backtrace);
///                                 panic!("Unexpected error! {e:?}");
///                             }
///                         }
///                     }
///                     k => {
///                         #[cfg(feature = "backtraces")]
///                         println!("Backtrace:\n{:?}", e.backtrace);
///                         panic!("Unexpected error! {k:?}");
///                     }
///                 }
///             }
///         }
///     }
/// }
///```
/// The previous will produce something like following:
/// ```text
/// Codec error: UPER
/// Error kind: Need more BITS to continue: (Size(83)).
/// Successful decoding!
/// Decoded string: Hello, world!
/// ```
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DecodeError {
    /// The kind of decoding error received.
    pub kind: Box<DecodeErrorKind>,
    /// The codec that returned the error.
    pub codec: Codec,
    /// The backtrace associated with the error.
    #[cfg(feature = "backtraces")]
    pub backtrace: Backtrace,
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Error Kind: {}", self.kind)?;
        writeln!(f, "Codec: {}", self.codec)?;
        #[cfg(feature = "backtraces")]
        write!(f, "\nBacktrace:\n{}", self.backtrace)?;
        Ok(())
    }
}

impl DecodeError {
    /// Creates a wrapper around a permitted alphabet error from a given codec.
    #[must_use]
    pub fn permitted_alphabet_error(reason: PermittedAlphabetError, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::PermittedAlphabetError { reason }, codec)
    }

    /// Creates a wrapper around a size error from a given codec.
    #[must_use]
    pub fn size_constraint_not_satisfied(
        size: Option<usize>,
        expected: alloc::string::String,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            DecodeErrorKind::SizeConstraintNotSatisfied { size, expected },
            codec,
        )
    }

    /// Creates a wrapper around a value error from a given codec.
    #[must_use]
    pub fn value_constraint_not_satisfied(
        value: BigInt,
        expected: Bounded<i128>,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            DecodeErrorKind::ValueConstraintNotSatisfied { value, expected },
            codec,
        )
    }

    /// Creates a wrapper around a discriminant value error from a given codec.
    #[must_use]
    pub fn discriminant_value_not_found(discriminant: isize, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::DiscriminantValueNotFound { discriminant },
            codec,
        )
    }

    /// Creates a wrapper around a range value error from a given codec.
    #[must_use]
    pub fn range_exceeds_platform_width(needed: u32, present: u32, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::RangeExceedsPlatformWidth { needed, present },
            codec,
        )
    }

    /// Creates a wrapper around a fixed string conversion error from a given codec.
    #[must_use]
    pub fn fixed_string_conversion_failed(
        tag: Tag,
        actual: usize,
        expected: usize,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            DecodeErrorKind::FixedStringConversionFailed {
                tag,
                actual,
                expected,
            },
            codec,
        )
    }

    /// Creates a wrapper around a sequence item error from a given codec.
    #[must_use]
    pub fn incorrect_item_number_in_sequence(expected: usize, actual: usize, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::IncorrectItemNumberInSequence { expected, actual },
            codec,
        )
    }

    /// Creates a wrapper around a integer overflow error from a given codec.
    #[must_use]
    pub fn integer_overflow(max_width: u32, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::IntegerOverflow { max_width }, codec)
    }

    /// Creates a wrapper around a integer conversion error from a given codec.
    #[must_use]
    pub fn integer_type_conversion_failed(msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::IntegerTypeConversionFailed { msg }, codec)
    }

    /// Creates a wrapper around a invalid bit string error from a given codec.
    #[must_use]
    pub fn invalid_bit_string(bits: u8, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::InvalidBitString { bits }, codec)
    }

    /// Creates a wrapper around a missing tag error from a given codec.
    #[must_use]
    pub fn missing_tag_class_or_value_in_sequence_or_set(
        class: crate::types::Class,
        value: u32,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            DecodeErrorKind::MissingTagClassOrValueInSequenceOrSet { class, value },
            codec,
        )
    }

    /// Creates a wrapper around a unexpected non-extensible type error from a given codec.
    #[must_use]
    pub fn type_not_extensible(codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::TypeNotExtensible, codec)
    }

    /// Creates a wrapper around a parser error from a given codec.
    #[must_use]
    pub fn parser_fail(msg: alloc::string::String, codec: Codec) -> Self {
        DecodeError::from_kind(DecodeErrorKind::Parser { msg }, codec)
    }

    /// Creates a wrapper around a missing required extension error from a given codec.
    #[must_use]
    pub fn required_extension_not_present(tag: Tag, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::RequiredExtensionNotPresent { tag }, codec)
    }

    /// Creates a wrapper around a missing enum index error from a given codec.
    #[must_use]
    pub fn enumeration_index_not_found(index: usize, extended_list: bool, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::EnumerationIndexNotFound {
                index,
                extended_list,
            },
            codec,
        )
    }

    /// Creates a wrapper around a choice index exceeding platform width error from a given codec.
    #[must_use]
    pub fn choice_index_exceeds_platform_width(
        needed: u32,
        present: DecodeError,
        codec: Codec,
    ) -> Self {
        Self::from_kind(
            DecodeErrorKind::ChoiceIndexExceedsPlatformWidth { needed, present },
            codec,
        )
    }

    /// Creates a wrapper around a length exceeding platform width error from a given codec.
    #[must_use]
    pub fn length_exceeds_platform_width(msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::LengthExceedsPlatformWidth { msg }, codec)
    }

    /// Creates a wrapper around a missing choice index error from a given codec.
    #[must_use]
    pub fn choice_index_not_found(index: usize, variants: Variants, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::ChoiceIndexNotFound { index, variants },
            codec,
        )
    }

    /// Creates a wrapper around a string conversion error from a given codec.
    #[must_use]
    pub fn string_conversion_failed(tag: Tag, msg: alloc::string::String, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::StringConversionFailed { tag, msg }, codec)
    }

    /// Creates a wrapper around unexpected extra data error from a given codec.
    #[must_use]
    pub fn unexpected_extra_data(length: usize, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::UnexpectedExtraData { length }, codec)
    }

    /// Creates a wrapper around unexpected empty input error from a given codec.
    #[must_use]
    pub fn unexpected_empty_input(codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::UnexpectedEmptyInput, codec)
    }

    /// Checks whether the length matches, and returns an error if not.
    pub fn assert_length(
        expected: usize,
        actual: usize,
        codec: Codec,
    ) -> core::result::Result<(), DecodeError> {
        if expected == actual {
            Ok(())
        } else {
            Err(DecodeError::from_kind(
                DecodeErrorKind::MismatchedLength { expected, actual },
                codec,
            ))
        }
    }

    pub(crate) fn map_nom_err<T: core::fmt::Debug>(
        error: nom::Err<nom::error::Error<T>>,
        codec: Codec,
    ) -> DecodeError {
        let msg = match error {
            nom::Err::Incomplete(needed) => return DecodeError::incomplete(needed, codec),
            err => alloc::format!("Parsing Failure: {err}"),
        };
        DecodeError::parser_fail(msg, codec)
    }

    /// Creates a new error from a given decode error kind and codec.
    #[must_use]
    pub fn from_kind(kind: DecodeErrorKind, codec: Codec) -> Self {
        Self {
            kind: Box::new(kind),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        }
    }

    #[must_use]
    fn from_codec_kind(inner: CodecDecodeError) -> Self {
        let codec = match inner {
            CodecDecodeError::Ber(_) => crate::Codec::Ber,
            #[allow(unreachable_patterns)]
            CodecDecodeError::Cer(_) => crate::Codec::Cer,
            CodecDecodeError::Der(_) => crate::Codec::Der,
            #[allow(unreachable_patterns)]
            CodecDecodeError::Uper(_) => crate::Codec::Uper,
            #[allow(unreachable_patterns)]
            CodecDecodeError::Aper(_) => crate::Codec::Aper,
            CodecDecodeError::Jer(_) => crate::Codec::Jer,
            CodecDecodeError::Oer(_) => crate::Codec::Oer,
            CodecDecodeError::Coer(_) => crate::Codec::Coer,
        };
        Self {
            kind: Box::new(DecodeErrorKind::CodecSpecific { inner }),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        }
    }
}

impl core::error::Error for DecodeError {}

/// `DecodeError` kinds which are common for all codecs.
#[derive(Snafu)]
#[snafu(visibility(pub))]
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeErrorKind {
    /// Permitted alphabet constraint wasn't satisfied.
    #[snafu(display("Alphabet constraint not satisfied {}", reason))]
    PermittedAlphabetError {
        /// The reason the constraint wasn't satisfied.
        reason: PermittedAlphabetError,
    },

    /// Size constraint wasn't satisfied.
    #[snafu(display("Size constraint not satisfied: expected: {expected}; actual: {size:?}"))]
    SizeConstraintNotSatisfied {
        /// Actual sie of the data
        size: Option<usize>,
        /// Expected size by the constraint
        expected: alloc::string::String,
    },

    /// Value constraint wasn't satisfied.
    #[snafu(display("Value constraint not satisfied: expected: {expected}; actual: {value}"))]
    ValueConstraintNotSatisfied {
        /// Actual value of the data
        value: BigInt,
        /// Expected value by the constraint
        expected: Bounded<i128>,
    },

    /// Codec specific error.
    #[snafu(display("Wrapped codec-specific decode error"))]
    CodecSpecific {
        /// The inner error type.
        inner: CodecDecodeError,
    },

    /// Enumeration index didn't match any variant.
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

    /// Choice index didn't match any variant.
    #[snafu(display("choice index '{index}' did not match any variant"))]
    ChoiceIndexNotFound {
        /// The found index of the choice variant.
        index: usize,
        /// The variants checked for presence.
        variants: Variants,
    },

    /// Choice index exceeds maximum possible address width.
    #[snafu(display("integer range larger than possible to address on this platform. needed: {needed} present: {present}"))]
    ChoiceIndexExceedsPlatformWidth {
        /// Amount of bytes needed.
        needed: u32,
        /// Inner error
        present: DecodeError,
    },

    /// Uncategorised error.
    #[snafu(display("Custom: {}", msg))]
    Custom {
        /// The error's message.
        msg: alloc::string::String,
    },

    /// Discriminant index didn't match any variant.
    #[snafu(display("Discriminant value '{}' did not match any variant", discriminant))]
    DiscriminantValueNotFound {
        /// The found value of the discriminant
        discriminant: isize,
    },

    /// Duplicate fields found.
    #[snafu(display("Duplicate field for `{}`", name))]
    DuplicateField {
        /// The field's name.
        name: &'static str,
    },

    /// Exceeds maxmium allowed length.
    #[snafu(display("Expected maximum of {} items", length))]
    ExceedsMaxLength {
        /// The maximum length.
        length: num_bigint::BigUint,
    },

    ///  More than `usize::MAX` number of data requested.
    #[snafu(display("Length of the incoming data is either exceeds platform address width."))]
    LengthExceedsPlatformWidth {
        /// The specific message of the length error.
        msg: alloc::string::String,
    },

    /// An error when decoding a field in a constructed type.
    #[snafu(display("Error when decoding field `{}`: {}", name, nested))]
    FieldError {
        /// The field's name.
        name: &'static str,
        /// The underlying error type.
        nested: Box<DecodeError>,
    },

    /// Input is provided as BIT slice for nom in UPER/APER.
    /// On BER/CER/DER/OER/COER it is a BYTE slice.
    /// Hence, `needed` field can describe either bits or bytes depending on the codec.
    #[snafu(display("Need more data to continue: ({:?}).", needed))]
    Incomplete {
        /// Amount of bits/bytes needed.
        needed: nom::Needed,
    },

    /// Invalid item number in sequence.
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

    /// Integer conversion overflow.
    #[snafu(display("Actual integer larger than expected {} bits", max_width))]
    IntegerOverflow {
        /// The maximum integer width.
        max_width: u32,
    },

    /// Integer conversion failure.
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed {
        /// The reason the conversion failed.
        msg: alloc::string::String,
    },

    /// BitString contains an invalid amount of unused bits.
    #[snafu(display("BitString contains an invalid amount of unused bits: {}", bits))]
    InvalidBitString {
        /// The amount of invalid bits.
        bits: u8,
    },

    /// BOOL value is not `0` or `0xFF`.
    #[snafu(display(
        "Bool value is not `0` or `0xFF` as canonical requires. Actual: {}",
        value
    ))]
    InvalidBool {
        /// The invalid bool value.
        value: u8,
    },

    /// Length of Length cannot be zero.
    #[snafu(display("Length of Length cannot be zero"))]
    ZeroLengthOfLength,
    /// The length does not match what was expected.
    #[snafu(display("Expected {:?} bytes, actual length: {:?}", expected, actual))]
    MismatchedLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    /// Missing field in a sequence.
    #[snafu(display("Missing field `{}`", name))]
    MissingField {
        /// The field's name.
        name: &'static str,
    },
    /// When there is a mismatch between the expected and actual tag class or `value`.
    #[snafu(display("Expected class: {}, value: {} in sequence or set Missing tag class or value in sequence or set", class, value))]
    MissingTagClassOrValueInSequenceOrSet {
        /// The tag's class.
        class: crate::types::Class,
        /// The tag's value.
        value: u32,
    },

    /// The range of the integer exceeds the platform width.
    #[snafu(display("integer range larger than possible to address on this platform. needed: {needed} present: {present}"))]
    RangeExceedsPlatformWidth {
        /// Amount of bytes needed.
        needed: u32,
        /// Amount of bytes needed.
        present: u32,
    },
    /// A specific required extension not present.
    #[snafu(display("Extension with class `{}` and tag `{}` required, but not present", tag.class, tag.value))]
    RequiredExtensionNotPresent {
        /// The tag of the required extension.
        tag: crate::types::Tag,
    },
    /// General error when parsing data.
    #[snafu(display("Error in Parser: {}", msg))]
    Parser {
        /// The error's message.
        msg: alloc::string::String,
    },
    /// General error for failed ASN.1 string conversion from bytes.
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
    /// General error for failed ASN.1 fixed-sized string conversion from bytes.
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
    /// An error when the choice cannot be created from the given variant.
    #[snafu(display("No valid choice for `{}`", name))]
    NoValidChoice {
        /// The field's name.
        name: &'static str,
    },

    /// An error when the type is not extensible when it should.
    #[snafu(display("Attempted to decode extension on non-extensible type"))]
    TypeNotExtensible,
    /// Unexpected extra data found.
    #[snafu(display("Unexpected extra data found: length `{}` bytes", length))]
    UnexpectedExtraData {
        /// The amount of garbage data.
        length: usize,
    },
    /// An error when a unknown field is found when decoding sequence.
    #[snafu(display("Unknown field with index {} and tag {}", index, tag))]
    UnknownField {
        /// Index of the field.
        index: usize,
        /// Tag of the field.
        tag: Tag,
    },
    /// An error when there should be more data but it is not present.
    #[snafu(display("SEQUENCE has at least one required field, but no input provided"))]
    UnexpectedEmptyInput,
}

/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for BER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum BerDecodeErrorKind {
    /// An error when the length is not definite.
    #[snafu(display("Indefinite length encountered but not allowed."))]
    IndefiniteLengthNotAllowed,
    /// An error if the value is not primitive when required.
    #[snafu(display("Invalid constructed identifier for ASN.1 value: not primitive."))]
    InvalidConstructedIdentifier,
    /// Invalid date.
    #[snafu(display("Invalid date string: {}", msg))]
    InvalidDate {
        /// The reason as string
        msg: alloc::string::String,
    },
    /// An error when the object identifier is invalid.
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
    /// A helper function to create an error [BerDecodeErrorKind::InvalidDate].
    #[must_use]
    pub fn invalid_date(msg: alloc::string::String) -> CodecDecodeError {
        CodecDecodeError::Ber(Self::InvalidDate { msg })
    }
    /// A helper function to create an error [BerDecodeErrorKind::MismatchedTag].
    pub fn assert_tag(expected: Tag, actual: Tag) -> core::result::Result<(), DecodeError> {
        if expected == actual {
            Ok(())
        } else {
            Err(BerDecodeErrorKind::MismatchedTag { expected, actual }.into())
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
    /// An error when constructed encoding encountered but not allowed.
    #[snafu(display("Constructed encoding encountered but not allowed."))]
    ConstructedEncodingNotAllowed,
}

/// An error that occurred when decoding JER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum JerDecodeErrorKind {
    /// An error when the end of input is reached, but more data is needed.
    #[snafu(display("Unexpected end of input while decoding JER JSON."))]
    EndOfInput {},
    /// An error when the JSON type is not the expected type.
    #[snafu(display(
        "Found mismatching JSON value. Expected type {}. Found value {}.",
        needed,
        found
    ))]
    TypeMismatch {
        /// Expected JSON type.
        needed: &'static str,
        /// Found JSON value.
        found: alloc::string::String,
    },
    /// An error when the JSON value is not a valid bit string.
    #[snafu(display("Found invalid byte in bit string. {parse_int_err}"))]
    InvalidJerBitstring {
        /// The error that occurred when parsing the BitString byte.
        parse_int_err: ParseIntError,
    },
    /// An error when the JSON value is not a valid octet string.
    #[snafu(display("Found invalid character in octet string."))]
    InvalidJerOctetString {},
    /// An error when the JSON value is not a valid OID string.
    #[snafu(display("Failed to construct OID from value {value}",))]
    InvalidOIDString {
        /// The JSON value that could not be converted to an OID.
        value: serde_json::Value,
    },
    /// An error when the JSON value is not a valid enumerated discriminant.
    #[snafu(display("Found invalid enumerated discriminant {discriminant}",))]
    InvalidEnumDiscriminant {
        /// The invalid enumerated discriminant.
        discriminant: alloc::string::String,
    },
}

impl JerDecodeErrorKind {
    /// Helper function to create an error [JerDecodeErrorKind::EndOfInput].
    pub fn eoi() -> CodecDecodeError {
        CodecDecodeError::Jer(JerDecodeErrorKind::EndOfInput {})
    }
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

/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for OER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum OerDecodeErrorKind {
    /// Tag class must be one of Universal (0b00), Application (0b01), Context (0b10) or Private (0b11).
    #[snafu(display("Invalid tag class when decoding choice: actual {:?}", class))]
    InvalidTagClassOnChoice {
        /// The actual class.
        class: u8,
    },
    /// The tag number was incorrect when decoding a Choice type.
    #[snafu(display("Invalid tag number when decoding Choice. Value: {value}"))]
    InvalidTagNumberOnChoice {
        /// Invalid tag bytes at the time of decoding.
        value: u32,
    },
    /// An error scenario where the tag number is not present in the expected Choice type.
    #[snafu(display(
        "Tag not found from the variants of the platform when decoding Choice. Tag: {value}, extensible status: {is_extensible}"
    ))]
    InvalidTagVariantOnChoice {
        /// The tag number that was not found.
        value: Tag,
        /// The extensible status of the Choice type.
        is_extensible: bool,
    },

    /// An error scenario where the extension header in preamble is invalid.
    InvalidExtensionHeader {
        /// The amount of invalid bits.
        msg: alloc::string::String,
    },
    /// An error scenario where the BitString is invalid for some reason.
    #[snafu(display("Invalid BitString: {msg}"))]
    InvalidOerBitString {
        /// The amount of invalid bits.
        msg: alloc::string::String,
    },
    /// An error scenario where the preamble is invalid.
    #[snafu(display("Invalid preamble: {msg}"))]
    InvalidPreamble {
        /// Message related to reason
        msg: alloc::string::String,
    },
}

impl OerDecodeErrorKind {
    #[must_use]
    /// Helper function to create an error [OerDecodeErrorKind::InvalidTagNumberOnChoice].
    pub fn invalid_tag_number_on_choice(value: u32) -> DecodeError {
        CodecDecodeError::Oer(Self::InvalidTagNumberOnChoice { value }).into()
    }
    #[must_use]
    /// Helper function to create an error [OerDecodeErrorKind::InvalidTagVariantOnChoice].
    pub fn invalid_tag_variant_on_choice(value: Tag, is_extensible: bool) -> DecodeError {
        CodecDecodeError::Oer(Self::InvalidTagVariantOnChoice {
            value,
            is_extensible,
        })
        .into()
    }

    /// Helper function to create an error [OerDecodeErrorKind::InvalidExtensionHeader].
    #[must_use]
    pub fn invalid_extension_header(msg: alloc::string::String) -> DecodeError {
        CodecDecodeError::Oer(Self::InvalidExtensionHeader { msg }).into()
    }
    /// Helper function to create an error [OerDecodeErrorKind::InvalidOerBitString].
    #[must_use]
    pub fn invalid_bit_string(msg: alloc::string::String) -> DecodeError {
        CodecDecodeError::Oer(Self::InvalidOerBitString { msg }).into()
    }
    /// Helper function to create an error [OerDecodeErrorKind::InvalidPreamble].
    #[must_use]
    pub fn invalid_preamble(msg: alloc::string::String) -> DecodeError {
        CodecDecodeError::Oer(Self::InvalidPreamble { msg }).into()
    }
}

/// `DecodeError` kinds of `Kind::CodecSpecific` which are specific for COER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum CoerDecodeErrorKind {
    /// An error of a result where the stricter Canonical Octet Encoding is not reached.
    #[snafu(display("Invalid Canonical Octet Encoding, not encoded as the smallest possible number of octets: {msg}"))]
    NotValidCanonicalEncoding {
        /// Reason for the error.
        msg: alloc::string::String,
    },
}

impl crate::de::Error for DecodeError {
    fn custom<D: core::fmt::Display>(msg: D, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::Custom {
                msg: msg.to_string(),
            },
            codec,
        )
    }
    fn incomplete(needed: nom::Needed, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::Incomplete { needed }, codec)
    }

    fn exceeds_max_length(length: num_bigint::BigUint, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::ExceedsMaxLength { length }, codec)
    }

    fn missing_field(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::MissingField { name }, codec)
    }

    fn no_valid_choice(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::NoValidChoice { name }, codec)
    }

    fn field_error(name: &'static str, nested: DecodeError, codec: Codec) -> Self {
        Self::from_kind(
            DecodeErrorKind::FieldError {
                name,
                nested: Box::new(nested),
            },
            codec,
        )
    }

    fn duplicate_field(name: &'static str, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::DuplicateField { name }, codec)
    }
    fn unknown_field(index: usize, tag: Tag, codec: Codec) -> Self {
        Self::from_kind(DecodeErrorKind::UnknownField { index, tag }, codec)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn test_ber_decode_date() {
        use crate::error::{DecodeError, DecodeErrorKind};
        // "230122130000-050Z" as bytes
        let data = [
            23, 17, 50, 51, 48, 49, 50, 50, 49, 51, 48, 48, 48, 48, 45, 48, 53, 48, 90,
        ];
        let result = crate::ber::decode::<UtcTime>(&data);
        match result {
            Err(DecodeError { kind, .. }) => {
                if let DecodeErrorKind::CodecSpecific {
                    inner:
                        crate::error::CodecDecodeError::Ber(
                            crate::error::BerDecodeErrorKind::InvalidDate { msg },
                        ),
                    ..
                } = *kind
                {
                    assert_eq!(msg, "230122130000-050Z");
                } else {
                    // Handle other kinds of errors
                    panic!("Unexpected error kind: {kind}");
                }
            }
            Ok(_) => panic!("Expected error"),
        }
    }
    #[test]
    fn test_uper_missing_choice_index() {
        use crate as rasn;
        use crate::error::{DecodeError, DecodeErrorKind};
        use crate::Codec;
        #[derive(AsnType, Decode, Debug, PartialEq)]
        #[rasn(choice, automatic_tags)]
        enum MyChoice {
            Normal(Integer),
            High(Integer),
            Medium(Integer),
        }
        // Value 333 encoded for missing choice index 3
        let data = [192, 128, 83, 64];
        let result = Codec::Uper.decode_from_binary::<MyChoice>(&data);
        match result {
            Ok(_) => {
                panic!("Unexpected OK!");
            }
            Err(DecodeError { kind, .. }) => {
                if let DecodeErrorKind::ChoiceIndexNotFound { index, .. } = *kind {
                    assert_eq!(index, 3);
                } else {
                    // Handle other kinds of errors
                    panic!("Unexpected error kind: {kind}");
                }
            }
        }
    }
}
