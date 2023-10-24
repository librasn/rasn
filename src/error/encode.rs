use crate::types::constraints::{Bounded, Size};
use snafu::{Backtrace, GenerateImplicitData, Snafu};

use alloc::string::ToString;

/// Variants for every codec-specific error kind.
#[derive(Debug)]
#[non_exhaustive]
pub enum CodecEncodeError {
    Ber(BerEncodeErrorKind),
    Cer(CerEncodeErrorKind),
    Der(DerEncodeErrorKind),
    Uper(UperEncodeErrorKind),
    Aper(AperEncodeErrorKind),
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

impl From<CodecEncodeError> for EncodeError {
    fn from(error: CodecEncodeError) -> Self {
        Self::from_codec_kind(error)
    }
}

/// An error type for failed encoding for every encoder.
/// Abstracts over the different generic and codec-specific errors.
/// `kind` field is used to determine the kind of error that occurred.
/// `codec` field is used to determine the codec that failed.
/// `backtrace` field is used to determine the backtrace of the error.
///
/// There is `Kind::CodecSpecific` variant which wraps the codec-specific
/// errors as `CodecEncodeError` type.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
#[allow(clippy::module_name_repetitions)]
pub struct EncodeError {
    kind: Kind,
    codec: crate::Codec,
    backtrace: Backtrace,
}
impl EncodeError {
    #[must_use]
    pub fn constraint_not_satisfied(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self {
            kind: Kind::ConstraintNotSatisfied { msg },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    pub fn check_length(length: usize, expected: &Size, codec: crate::Codec) -> Result<(), Self> {
        expected.contains_or_else(&length, || Self {
            kind: Kind::InvalidLength {
                length,
                expected: (**expected),
            },
            codec,
            backtrace: Backtrace::generate(),
        })
    }
    /// An error for failed conversion from `BitInt` or `BigUint` to primitive integer types
    #[must_use]
    pub fn integer_type_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self {
            kind: Kind::IntegerTypeConversionFailed { msg },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn opaque_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self {
            kind: Kind::OpaqueConversionFailed { msg },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn variant_not_in_choice(codec: crate::Codec) -> Self {
        Self {
            kind: Kind::VariantNotInChoice,
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn from_kind(kind: Kind, codec: crate::Codec) -> Self {
        Self {
            kind,
            codec,
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
        };
        Self {
            kind: Kind::CodecSpecific { inner },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
}

/// EncodeError kinds which are common for all codecs.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Kind {
    #[snafu(display("Failed to convert BIT STRING unused bytes to u8: {err}"))]
    BitStringUnusedBytesToU8 { err: core::num::TryFromIntError },
    #[snafu(display("invalid length, expected: {expected}; actual: {length}"))]
    InvalidLength {
        length: usize,
        expected: Bounded<usize>,
    },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: alloc::string::String },
    #[snafu(display("Wrapped codec-specific encode error"))]
    CodecSpecific { inner: CodecEncodeError },
    #[snafu(display("Constraint not satisfied: {msg}"))]
    ConstraintNotSatisfied { msg: alloc::string::String },
    #[snafu(display("Failed to cast integer to another integer type: {msg} "))]
    IntegerTypeConversionFailed { msg: alloc::string::String },
    #[snafu(display("Conversion to Opaque type failed: {msg}"))]
    OpaqueConversionFailed { msg: alloc::string::String },
    #[snafu(display("Selected Variant not found from Choice"))]
    VariantNotInChoice,
}
/// EncodeError kinds which are specific for BER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum BerEncodeErrorKind {
    #[snafu(display("Cannot encode `ANY` types in `SET` fields"))]
    AnyInSet,
    /// `OBJECT IDENTIFIER` must have at least two components.
    #[snafu(display(
    "Invalid Object Identifier: must have at least two components and first octet must be 0, 1 or 2"
    ))]
    InvalidObjectIdentifier,
}
impl BerEncodeErrorKind {
    #[must_use]
    pub fn invalid_object_identifier() -> Self {
        Self::InvalidObjectIdentifier
    }
}

// TODO are there CER/DER specific errors?
/// EncodeError kinds which are specific for CER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum CerEncodeErrorKind {}

/// EncodeError kinds which are specific for DER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum DerEncodeErrorKind {}

/// EncodeError kinds which are specific for UPER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum UperEncodeErrorKind {}

/// EncodeError kinds which are specific for APER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum AperEncodeErrorKind {}
impl crate::enc::Error for EncodeError {
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self {
        Self {
            kind: Kind::Custom {
                msg: msg.to_string(),
            },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
}
