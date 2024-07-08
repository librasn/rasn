//! Error module includes all encode and decode errors among all codecs.
//! Encoding can result to `EncodeError` and decoding can result to `DecodeError`.
//! Backtraces are enabled by default with `backtraces` feature.
//! See submodules for other error types.
#![allow(clippy::module_name_repetitions)]
mod decode;
mod encode;
mod string;

pub mod strings {
    //! Errors specific to string conversions, permitted alphabets, and other type problems.
    pub use super::string::{
        InvalidBmpString, InvalidGeneralString, InvalidIA5String, InvalidNumericString,
        InvalidPrintableString, InvalidRestrictedString, InvalidVisibleString,
        PermittedAlphabetError,
    };
}

pub use decode::DecodeErrorKind;
pub use decode::{
    BerDecodeErrorKind, CodecDecodeError, CoerDecodeErrorKind, DecodeError, DerDecodeErrorKind,
    JerDecodeErrorKind, OerDecodeErrorKind,
};
pub use encode::EncodeErrorKind;
#[cfg(feature = "jer")]
pub use encode::JerEncodeErrorKind;
pub use encode::{BerEncodeErrorKind, CodecEncodeError, CoerEncodeErrorKind, EncodeError};
