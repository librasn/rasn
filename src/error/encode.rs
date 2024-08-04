use crate::prelude::Integer;
use crate::types::constraints::{Bounded, Size};
use num_bigint::BigInt;
use snafu::Snafu;
#[cfg(feature = "backtraces")]
use snafu::{Backtrace, GenerateImplicitData};

use alloc::{boxed::Box, string::ToString};

/// Variants for every codec-specific `EncodeError` kind.
#[derive(Debug)]
#[non_exhaustive]
pub enum CodecEncodeError {
    Ber(BerEncodeErrorKind),
    Cer(CerEncodeErrorKind),
    Der(DerEncodeErrorKind),
    Uper(UperEncodeErrorKind),
    Aper(AperEncodeErrorKind),
    #[cfg(feature = "jer")]
    Jer(JerEncodeErrorKind),
    Coer(CoerEncodeErrorKind),
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
#[cfg(feature = "jer")]
impl_from!(Jer, JerEncodeErrorKind);
impl_from!(Coer, CoerEncodeErrorKind);

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
///
/// use rasn::prelude::*;
/// use rasn::error::{EncodeErrorKind, strings::PermittedAlphabetError};
/// use rasn::codec::Codec;
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
    pub kind: Box<EncodeErrorKind>,
    pub codec: crate::Codec,
    #[cfg(feature = "backtraces")]
    pub backtrace: Backtrace,
}
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
    #[must_use]
    pub fn value_constraint_not_satisfied(
        value: Integer<BigInt>,
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
    #[must_use]
    pub fn length_exceeds_platform_size(codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::LengthExceedsPlatformSize, codec)
    }
    /// An error for failed conversion from `BitInt` or `BigUint` to primitive integer types
    #[must_use]
    pub fn integer_type_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::IntegerTypeConversionFailed { msg }, codec)
    }
    #[must_use]
    pub fn invalid_length(length: usize, expected: Bounded<usize>, codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::InvalidLength { length, expected }, codec)
    }
    #[must_use]
    pub fn opaque_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::OpaqueConversionFailed { msg }, codec)
    }
    #[must_use]
    pub fn variant_not_in_choice(codec: crate::Codec) -> Self {
        Self::from_kind(EncodeErrorKind::VariantNotInChoice, codec)
    }
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
            CodecEncodeError::Cer(_) => crate::Codec::Cer,
            CodecEncodeError::Der(_) => crate::Codec::Der,
            CodecEncodeError::Uper(_) => crate::Codec::Uper,
            CodecEncodeError::Aper(_) => crate::Codec::Aper,
            #[cfg(feature = "jer")]
            CodecEncodeError::Jer(_) => crate::Codec::Jer,
            CodecEncodeError::Coer(_) => crate::Codec::Coer,
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
    #[snafu(display("Failed to convert BIT STRING unused bits to u8: {err}"))]
    FailedBitStringUnusedBitsToU8 { err: core::num::TryFromIntError },
    #[snafu(display("invalid length, expected: {expected}; actual: {length}"))]
    InvalidLength {
        /// Actual length of the data
        length: usize,
        /// Expected length of the data
        expected: Bounded<usize>,
    },
    #[snafu(display("invalid length, exceeds platform maximum size usize::MAX"))]
    LengthExceedsPlatformSize,
    #[snafu(display("Integer does not fit to the reserved octets {expected}; actual: {value}"))]
    MoreBytesThanExpected { value: usize, expected: usize },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: alloc::string::String },
    #[snafu(display("Wrapped codec-specific encode error"))]
    CodecSpecific { inner: CodecEncodeError },
    #[snafu(display("Alphabet constraint not satisfied: {reason}"))]
    AlphabetConstraintNotSatisfied {
        /// Inner error from mapping realized characters to allowed characters
        reason: super::strings::PermittedAlphabetError,
    },
    #[snafu(display("Size constraint not satisfied: expected: {expected}; actual: {size}"))]
    SizeConstraintNotSatisfied {
        /// Actual sie of the data
        size: usize,
        /// Expected size by the constraint
        expected: Bounded<usize>,
    },
    #[snafu(display("Value constraint not satisfied: expected: {expected}; actual: {value}"))]
    ValueConstraintNotSatisfied {
        /// Actual value of the data
        value: Integer<BigInt>,
        /// Expected value by the constraint
        expected: Bounded<i128>,
    },
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed { msg: alloc::string::String },
    #[snafu(display("Conversion to Opaque type failed: {msg}"))]
    OpaqueConversionFailed { msg: alloc::string::String },
    #[snafu(display("Selected Variant not found from Choice"))]
    VariantNotInChoice,
}
/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for BER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum BerEncodeErrorKind {
    #[snafu(display("Cannot encode `ANY` types in `SET` fields"))]
    AnyInSet,
    /// `OBJECT IDENTIFIER` must have at least two components.
    #[snafu(display(
    "Invalid Object Identifier: must have at least two components and first octet must be 0, 1 or 2. Provided: {:?}", oid
    ))]
    InvalidObjectIdentifier { oid: alloc::vec::Vec<u32> },
}
impl BerEncodeErrorKind {
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
#[cfg(feature = "jer")]
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum JerEncodeErrorKind {
    /// Upstream `serde` error
    JsonEncodingError { upstream: alloc::string::String },
    /// Error to be thrown when the JER encoder contains no encoded root value
    #[snafu(display("No encoded JSON root value found!"))]
    NoRootValueFound,
    /// Internal JSON encoder error
    #[snafu(display("Error in JSON encoder: {}", msg))]
    JsonEncoder {
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display("Exceeds supported integer range -2^63..2^63 ({:?}).", value))]
    ExceedsSupportedIntSize {
        /// value failed to encode
        value: Integer<BigInt>,
    },
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

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum CoerEncodeErrorKind {
    #[snafu(display("Provided data is too long to be encoded with COER."))]
    TooLongValue { length: u128 },
    #[snafu(display(
    "Provided length in not correct format. Should be bits as multiple of 8. {remainder}; actual: {length}"
    ))]
    LengthNotAsBitLength { length: usize, remainder: usize },
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
        let result = enc.encode_object_identifier(Tag::OBJECT_IDENTIFIER, &oid);
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
