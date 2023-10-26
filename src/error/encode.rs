use crate::types::constraints::{Bounded, Size};
use snafu::{Backtrace, GenerateImplicitData, Snafu};

use alloc::string::ToString;

/// Variants for every codec-specific `EncodeError` kind.
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
/// fn main() {
///
///     let constrained_str = MyConstrainedString(VisibleString::try_from("abcD").unwrap());
///     let encoded = Codec::Uper.encode(&constrained_str);
///     match encoded {
///         Ok(succ) => {
///             println!("Successful encoding!");
///             dbg!(succ);
///         }
///         Err(e) => {
///             // e is EncodeError
///             match e.kind {
///                 EncodeErrorKind::AlphabetConstraintNotSatisfied {
///                     reason: PermittedAlphabetError::CharacterNotFound {character },
///                 } => {
///                     println!("Codec: {}", e.codec);
///                     println!("Character {} not found from the permitted list.",
///                              char::from_u32(character).unwrap());
///                     println!("Backtrace:\n{}", e.backtrace);
///                 }
///                 _ => {
///                     panic!("Unexpected error!");
///                 }
///             }
///         }
///     }
///     // Should print ->
///     //
///     // Codec: UPER
///     // Character D not found from the permitted list.
///     // Backtrace: ...
///
/// }
/// ```
///
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
#[allow(clippy::module_name_repetitions)]
pub struct EncodeError {
    pub kind: EncodeErrorKind,
    pub codec: crate::Codec,
    pub backtrace: Backtrace,
}
impl EncodeError {
    #[must_use]
    pub fn alphabet_constraint_not_satisfied(
        reason: super::strings::PermittedAlphabetError,
        codec: crate::Codec,
    ) -> Self {
        Self {
            kind: EncodeErrorKind::AlphabetConstraintNotSatisfied { reason },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    pub fn check_length(length: usize, expected: &Size, codec: crate::Codec) -> Result<(), Self> {
        expected.contains_or_else(&length, || Self {
            kind: EncodeErrorKind::InvalidLength {
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
            kind: EncodeErrorKind::IntegerTypeConversionFailed { msg },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn opaque_conversion_failed(msg: alloc::string::String, codec: crate::Codec) -> Self {
        Self {
            kind: EncodeErrorKind::OpaqueConversionFailed { msg },
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn variant_not_in_choice(codec: crate::Codec) -> Self {
        Self {
            kind: EncodeErrorKind::VariantNotInChoice,
            codec,
            backtrace: Backtrace::generate(),
        }
    }
    #[must_use]
    pub fn from_kind(kind: EncodeErrorKind, codec: crate::Codec) -> Self {
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
            kind: EncodeErrorKind::CodecSpecific { inner },
            codec,
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
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: alloc::string::String },
    #[snafu(display("Wrapped codec-specific encode error"))]
    CodecSpecific { inner: CodecEncodeError },
    #[snafu(display("Constraint not satisfied: {reason}"))]
    AlphabetConstraintNotSatisfied {
        /// Inner error from mapping realized characters to allowed characters
        reason: super::strings::PermittedAlphabetError,
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
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum UperEncodeErrorKind {}

/// `EncodeError` kinds of `Kind::CodecSpecific` which are specific for APER.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum AperEncodeErrorKind {}

impl crate::enc::Error for EncodeError {
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self {
        Self {
            kind: EncodeErrorKind::Custom {
                msg: msg.to_string(),
            },
            codec,
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
        let oid_encoded = crate::Codec::Ber.encode(&oid);
        assert!(oid_encoded.is_ok());

        let oid = vec![3, 5, 4, 3];

        let mut enc = enc::Encoder::new(enc::EncoderOptions::ber());
        let result = enc.encode_object_identifier(Tag::OBJECT_IDENTIFIER, &oid);
        assert!(result.is_err());
        match result {
            Err(EncodeError {
                kind:
                    EncodeErrorKind::CodecSpecific {
                        inner:
                            CodecEncodeError::Ber(BerEncodeErrorKind::InvalidObjectIdentifier {
                                ..
                            }),
                    },
                ..
            }) => {}
            _ => panic!("Expected invalid object identifier error of specific type!"),
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
        let encoded = Codec::Uper.encode(&constrained_str);
        match encoded {
            Ok(succ) => {
                println!("Successful encoding!");
                dbg!(succ);
            }
            Err(e) => {
                // EncodeError
                match e.kind {
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