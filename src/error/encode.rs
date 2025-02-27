//! Error types associated with encoding to ASN.1 codecs.
use crate::types::constraints::{Bounded, Size};
use num_bigint::BigInt;
use snafu::Snafu;
#[cfg(feature = "backtraces")]
use snafu::{Backtrace, GenerateImplicitData};

use alloc::{boxed::Box, string::ToString};

/// Variants for every codec-specific `EncodeError` kind.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CodecEncodeError {
    Ber(BerEncodeErrorKind),
    Cer(CerEncodeErrorKind),
    Der(DerEncodeErrorKind),
    Uper(UperEncodeErrorKind),
    Aper(AperEncodeErrorKind),
    Jer(JerEncodeErrorKind),
    Coer(CoerEncodeErrorKind),
    Xer(XerEncodeErrorKind),
}
macro_rules! impl_from {
    ($variant:ident, $error_kind:ty) => {
        impl From<$error_kind> for EncodeError {
            fn from(error: $error_kind) -> Self {
                Self::from_codec_kind(CodecEncodeError::$variant(error))
            }
        }
    };
}

// implement From for each variant of CodecEncodeError into EncodeError
impl_from!(Ber, BerEncodeErrorKind);
impl_from!(Cer, CerEncodeErrorKind);
impl_from!(Der, DerEncodeErrorKind);
impl_from!(Uper, UperEncodeErrorKind);
impl_from!(Aper, AperEncodeErrorKind);
impl_from!(Jer, JerEncodeErrorKind);
impl_from!(Coer, CoerEncodeErrorKind);
impl_from!(Xer, XerEncodeErrorKind);

impl From<CodecEncodeError> for EncodeError {
    fn from(error: CodecEncodeError) -> Self {
        Self::from_codec_kind(error)
    }
}

/// An error type for failed encoding for every encoder.
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
/// ```
/// use rasn::{Codec, error::{EncodeErrorKind, strings::PermittedAlphabetError}, prelude::*};
///
/// #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
/// #[rasn(delegate, from("a..z"))]
/// struct MyConstrainedString (
///     VisibleString,
/// );
///
/// // Below sample requires that `backtraces` feature is enabled
///
/// let constrained_str = MyConstrainedString(VisibleString::try_from("abcD").unwrap());
/// let encoded = Codec::Uper.encode_to_binary(&constrained_str);
/// match encoded {
///     Ok(succ) => {
///         println!("Successful encoding!");
///         dbg!(succ);
///     }
///     Err(e) => {
///         // e is EncodeError, Kind is boxed
///         match *e.kind {
///             EncodeErrorKind::AlphabetConstraintNotSatisfied {
///                reason: PermittedAlphabetError::CharacterNotFound {character },
///             } => {
///                 println!("Codec: {}", e.codec);
///                 println!("Character {} not found from the permitted list.",
///                          char::from_u32(character).unwrap());
///                 #[cfg(feature = "backtraces")]
///                 println!("Backtrace:\n{}", e.backtrace);
///             }
///             _ => {
///                 panic!("Unexpected error!");
///             }
///         }
///     }
/// }
/// // Should print ->
/// //
/// // Codec: UPER
/// // Character D not found from the permitted list.
/// // Backtrace: ...
/// ```
///
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct EncodeError {
    /// The inner encoding error.
    pub kind: Box<EncodeErrorKind>,
    /// The codec associated with the error.
    pub codec: crate::Codec,
    /// The backtrace for the given error.
    #[cfg(feature = "backtraces")]
    pub backtrace: Backtrace,
}

impl core::error::Error for EncodeError {}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Error Kind: {}", self.kind)?;
        writeln!(f, "Codec: {}", self.kind)?;
        #[cfg(feature = "backtraces")]
        write!(f, "\nBacktrace:\n{}", self.backtrace)?;

        Ok(())
    }
}

impl EncodeError {
    /// Returns an encode error for `codec` when the alphabet constraint is not satisfied.
    #[must_use]
    pub fn alphabet_constraint_not_satisfied(
        reason: super::strings::PermittedAlphabetError,
        codec: crate::Codec,
    ) -> Self {
        Self::from_kind(
            EncodeErrorKind::AlphabetConstraintNotSatisfied { reason },
            codec,
        )
    }

    #[must_use]
    /// Returns an encode error for `codec` when the size constraint is not satisfied.
    ///
    /// The `size` is the actual size that failed the constraint and `expected` the expected size.
    pub fn size_constraint_not_satisfied(
        size: usize,
        expected: &Size,
        codec: crate::Codec,
    ) -> Self {
        Self::from_kind(
            EncodeErrorKind::SizeConstraintNotSatisfied {
                size,
                expected: (**expected),
            },
            codec,
        )
    }

    /// Returns an encode error for `codec` when the value constraint is not satisfied.
    ///
    /// The `value` is the actual value that failed the constraint.
    #[must_use]
    pub fn value_constraint_not_satisfied(
        value: BigInt,
        expected: &Bounded<i128>,
        codec: crate::Codec,
    ) -> Self {
        Self::from_kind(
            EncodeErrorKind::ValueConstraintNotSatisfied {
                value,
                expected: (*expected),
            },
            codec,
        )
    }

    /// Check the passed `length` against the expected size and return an error if it does not match.
    pub fn check_length(length: usize, expected: &Size, codec: crate::Codec) -> Result<(), Self> {
        expected.contains_or_else(&length, || Self {
            kind: Box::new(EncodeErrorKind::InvalidLength {
                length,
                expected: **expected,
            }),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        })
    }

    /// Create an encode error for `codec` when the value is too large to be encoded on the current platform.
    #[must_use]
    pub fn length_exceeds_platform_size(codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::LengthExceedsPlatformSize, codec)
    }

    /// Create an error for failed conversion from `BitInt` or `BigUint` to primitive integer types
    #[must_use]
    pub fn integer_type_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::IntegerTypeConversionFailed { msg }, codec)
    }

    /// Create an error if conversion to opaque type failed.
    ///
    /// This is mainly used as part of SMI standard which converts type to BER encoding and handles bytes as `Opaque`.
    #[must_use]
    pub fn opaque_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::OpaqueConversionFailed { msg }, codec)
    }

    /// Create an error when the selected variant is not found in the choice.
    #[must_use]
    pub fn variant_not_in_choice(codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::VariantNotInChoice, codec)
    }

    /// Returns an encode error when the encoder doesn't support `REAL` type.
    #[must_use]
    pub fn real_not_supported(codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::RealNotSuppored, codec)
    }

    /// A helper function to construct an `EncodeError` from the given `kind` and `codec`.
    #[must_use]
    pub fn from_kind(kind: EncodeErrorKind, codec: crate::Codec) -> Self {
        Self {
            kind: Box::new(kind),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        }
    }

    #[must_use]
    fn from_codec_kind(inner: CodecEncodeError) -> Self {
        let codec = match inner {
            CodecEncodeError::Ber(_) => crate::Codec::Ber,
            #[allow(unreachable_patterns)]
            CodecEncodeError::Cer(_) => crate::Codec::Cer,
            #[allow(unreachable_patterns)]
            CodecEncodeError::Der(_) => crate::Codec::Der,
            #[allow(unreachable_patterns)]
            CodecEncodeError::Uper(_) => crate::Codec::Uper,
            #[allow(unreachable_patterns)]
            CodecEncodeError::Aper(_) => crate::Codec::Aper,
            CodecEncodeError::Jer(_) => crate::Codec::Jer,
            CodecEncodeError::Coer(_) => crate::Codec::Coer,
            CodecEncodeError::Xer(_) => crate::Codec::Xer,
        };
        Self {
            kind: Box::new(EncodeErrorKind::CodecSpecific { inner }),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        }
    }
}

/// `EncodeError` kinds which are common for all codecs.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum EncodeErrorKind {
    /// Error when the BitString does not align with the `u8` byte boundary.
    #[snafu(display("Failed to convert BIT STRING unused bits to u8: {err}"))]
    FailedBitStringUnusedBitsToU8 {
        /// Internal integer conversion error
        err: core::num::TryFromIntError,
    },
    /// Error when the length of the data is not in the constraint size range.
    #[snafu(display("invalid length, expected: {expected}; actual: {length}"))]
    InvalidLength {
        /// Actual length of the data
        length: usize,
        /// Expected length of the data
        expected: Bounded<usize>,
    },
    /// Error when the length of the data is more than we can technically handle.
    #[snafu(display("invalid length, exceeds platform maximum size usize::MAX"))]
    LengthExceedsPlatformSize,
    /// Encode error when the
    #[snafu(display(
        "The provided value does not fit to the reserved octets {expected}; actual: {value}"
    ))]
    MoreBytesThanExpected {
        /// The count of the provided bytes
        value: usize,
        /// Expected number of bytes
        expected: usize,
    },
    /// Error when the custom error is thrown.
    #[snafu(display("custom error:\n{}", msg))]
    Custom {
        /// The custom error message
        msg: alloc::string::String,
    },
    /// Wraps codec-specific errors as inner [CodecEncodeError].
    #[snafu(display("Wrapped codec-specific encode error"))]
    CodecSpecific {
        /// Inner codec-specific error
        inner: CodecEncodeError,
    },
    /// Error when the alphabet constraint is not satisfied.
    #[snafu(display("Alphabet constraint not satisfied: {reason}"))]
    AlphabetConstraintNotSatisfied {
        /// Inner error from mapping realized characters to allowed characters
        reason: super::strings::PermittedAlphabetError,
    },
    /// Error when the size constraint is not satisfied.
    #[snafu(display("Size constraint not satisfied: expected: {expected}; actual: {size}"))]
    SizeConstraintNotSatisfied {
        /// Actual sie of the data
        size: usize,
        /// Expected size by the constraint
        expected: Bounded<usize>,
    },
    /// Error when the value constraint is not satisfied.
    #[snafu(display("Value constraint not satisfied: expected: {expected}; actual: {value}"))]
    ValueConstraintNotSatisfied {
        /// Actual value of the data
        value: BigInt,
        /// Expected value by the constraint
        expected: Bounded<i128>,
    },
    /// Error when the type conversion failed between different integer types.
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed {
        /// More precise error message
        msg: alloc::string::String,
    },
    /// Error mainly used as part of SMI standard which converts type to BER encoding and handles bytes as `Opaque`.
    #[snafu(display("Conversion to Opaque type failed: {msg}"))]
    OpaqueConversionFailed {
        /// More precise error message
        msg: alloc::string::String,
    },
    /// Error when the selected variant is not found in the choice.
    #[snafu(display("Selected Variant not found from Choice"))]
    VariantNotInChoice,

    /// Error when we try to encode a `REAL` type with an unspported codec.
    #[snafu(display("Encoder doesn't support `REAL` type"))]
    RealNotSuppored,
}
/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for BER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum BerEncodeErrorKind {
    /// Error to be thrown when the BER encoder encounters an `ANY` type in a `SET` field.
    #[snafu(display("Cannot encode `ANY` types in `SET` fields"))]
    AnyInSet,
    /// `OBJECT IDENTIFIER` must have at least two components.
    #[snafu(display(
    "Invalid Object Identifier: must have at least two components and first octet must be 0, 1 or 2. Provided: {:?}", oid
    ))]
    InvalidObjectIdentifier {
        /// Bytes of the invalid object identifier
        oid: alloc::vec::Vec<u32>,
    },
}
impl BerEncodeErrorKind {
    /// Create an error [BerEncodeErrorKind::InvalidObjectIdentifier}.
    #[must_use]
    pub fn invalid_object_identifier(oid: alloc::vec::Vec<u32>) -> Self {
        Self::InvalidObjectIdentifier { oid }
    }
}

// TODO are there CER/DER/APER/UPER specific errors?
/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for CER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum CerEncodeErrorKind {}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for DER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum DerEncodeErrorKind {}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for UPER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum JerEncodeErrorKind {
    /// Upstream `serde` error
    JsonEncodingError {
        /// Wrapped error from `serde` when encoding JSON
        upstream: alloc::string::String,
    },
    /// Error to be thrown when the JER encoder contains no encoded root value
    #[snafu(display("No encoded JSON root value found!"))]
    NoRootValueFound,
    /// Internal JSON encoder error
    #[snafu(display("Error in JSON encoder: {}", msg))]
    JsonEncoder {
        /// The error's message.
        msg: alloc::string::String,
    },
    /// Error to be thrown when encoding large integers than the supported range
    #[snafu(display("Exceeds supported integer range -2^63..2^63 ({:?}).", value))]
    ExceedsSupportedIntSize {
        /// value failed to encode
        value: BigInt,
    },
    /// Error to be thrown when encoding real values that exceed the supported range
    #[snafu(display("Exceeds supported real value range"))]
    ExceedsSupportedRealRange,
    /// Error to be thrown when some character from the input data is not valid UTF-8
    #[snafu(display("Invalid character: {:?}", error))]
    InvalidCharacter {
        /// value failed to encode
        error: alloc::string::FromUtf8Error,
    },
}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for UPER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum UperEncodeErrorKind {}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for APER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum AperEncodeErrorKind {}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for XER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum XerEncodeErrorKind {
    /// Upstream `xml` error
    XmlEncodingError { upstream: alloc::string::String },
    #[snafu(display("Failed to retrieve field name."))]
    FieldName,
    #[snafu(display("Failed to encode integer."))]
    UnsupportedIntegerValue,
    #[snafu(display("Missing identifier for ASN.1 type."))]
    MissingIdentifier,
}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for COER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum CoerEncodeErrorKind {
    /// Error type for a scenario when the provided data is too long to be encoded with COER.
    #[snafu(display("Provided data is too long to be encoded with COER."))]
    TooLongValue {
        /// The length of the provided data
        length: u128,
    },
    /// Error type for a secenario when the provided integer value exceeds the limits of the constrained word sizes.
    #[snafu(display("Provided integer exceeds limits of the constrained word sizes."))]
    InvalidConstrainedIntegerOctetSize,
}

impl crate::enc::Error for EncodeError {
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self {
        Self {
            kind: Box::new(EncodeErrorKind::Custom {
                msg: msg.to_string(),
            }),
            codec,
            #[cfg(feature = "backtraces")]
            backtrace: Backtrace::generate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_ber_error() {
        use crate::ber::enc;
        use crate::enc::Encoder;

        let oid = ObjectIdentifier::new(vec![2, 5, 4, 3]);
        assert!(oid.is_some());
        // Higher level abstraction does not allow us to provide OID errors because we provide only valid types
        let oid_encoded = crate::Codec::Ber.encode_to_binary(&oid);
        assert!(oid_encoded.is_ok());

        let oid = vec![3, 5, 4, 3];

        let mut enc = enc::Encoder::new(enc::EncoderOptions::ber());
        let result = enc.encode_object_identifier(Tag::OBJECT_IDENTIFIER, &oid, None);
        assert!(result.is_err());
        match result {
            Err(e) => match *e.kind {
                EncodeErrorKind::CodecSpecific {
                    inner: CodecEncodeError::Ber(BerEncodeErrorKind::InvalidObjectIdentifier { .. }),
                } => {}
                _ => {
                    panic!("Expected invalid object identifier error of specific type!");
                }
            },
            _ => panic!("Unexpected OK!"),
        }
        // Debug output should look something like this:
        // dbg!(result.err());
        // EncodeError {
        //     kind: CodecSpecific {
        //         inner: Ber(
        //             InvalidObjectIdentifier {
        //                 oid: [
        //                     3,
        //                     5,
        //                     4,
        //                     3,
        //                 ],
        //             },
        //         ),
        //     },
        //     codec: Ber,
        //     backtrace: Backtrace( .... ),
        // },
    }
    #[test]
    fn test_uper_constrained_string_error() {
        use crate as rasn;
        use rasn::codec::Codec;
        use rasn::error::{strings::PermittedAlphabetError, EncodeErrorKind};
        #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
        #[rasn(delegate, from("a..z"))]
        struct MyConstrainedString(VisibleString);

        let constrained_str = MyConstrainedString(VisibleString::try_from("abcD").unwrap());
        let encoded = Codec::Uper.encode_to_binary(&constrained_str);
        match encoded {
            Ok(_) => {}
            Err(e) => {
                // EncodeError
                match *e.kind {
                    EncodeErrorKind::AlphabetConstraintNotSatisfied {
                        reason: PermittedAlphabetError::CharacterNotFound { .. },
                    } => {}
                    _ => {
                        panic!("Unexpected error!");
                    }
                }
            }
        }
    }
}
