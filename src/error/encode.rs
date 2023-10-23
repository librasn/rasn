use crate::types::constraints::{Bounded, Size};
use snafu::{Backtrace, GenerateImplicitData, Snafu};

use alloc::string::ToString;

#[derive(Debug)]
pub enum CodecEncodeError {
    Ber(BerEncodeError),
    Cer(CerEncodeError),
    Der(DerEncodeError),
    // Uper(UperEncodeError),
    // Aper(AperEncodeError),
}

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
    #[must_use]
    pub fn integer_type_conversion(msg: alloc::string::String, codec: crate::Codec) -> Self {
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
    fn codec_specific(inner: CodecEncodeError, codec: crate::Codec) -> Self {
        Self {
            kind: Kind::CodecSpecific { inner },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
}
impl From<BerEncodeError> for EncodeError {
    fn from(error: BerEncodeError) -> Self {
        Self::codec_specific(CodecEncodeError::Ber(error), crate::Codec::Ber)
    }
}
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
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
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum BerEncodeError {
    #[snafu(display("Cannot encode `ANY` types in `SET` fields"))]
    AnyInSet,
    /// `OBJECT IDENTIFIER` must have at least two components.
    #[snafu(display(
    "Invalid Object Identifier: must have at least two components and first octet must be 0, 1 or 2"
    ))]
    InvalidObjectIdentifier,
}
impl BerEncodeError {
    #[must_use]
    pub fn invalid_object_identifier() -> Self {
        Self::InvalidObjectIdentifier
    }
}

// TODO are there CER/DER specific errors?
type CerEncodeError = BerEncodeError;
type DerEncodeError = BerEncodeError;

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
