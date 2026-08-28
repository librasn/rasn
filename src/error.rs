//! Error module includes all encode and decode errors among all codecs.
//!
//! Encoding can result to `EncodeError` and decoding can result to `DecodeError`.
//! Backtraces are enabled by default with `backtraces` feature.
//! See submodules for other error types.
#![allow(clippy::module_name_repetitions)]
mod components;
mod decode;
mod encode;
mod string;

pub mod strings {
    //! Errors specific to string conversions, permitted alphabets, and other type problems.
    pub use super::string::{
        InvalidBmpString, InvalidGeneralString, InvalidGraphicString, InvalidIA5String,
        InvalidNumericString, InvalidPrintableString, InvalidRestrictedString,
        InvalidTeletexString, InvalidVisibleString, PermittedAlphabetError,
    };
}

pub use decode::CodecDecodeError;
pub use decode::DecodeError;
pub use decode::DecodeErrorKind;

#[cfg(feature = "codec_avn")]
pub use decode::AvnDecodeErrorKind;
#[cfg(feature = "codec_ber")]
pub use decode::BerDecodeErrorKind;
#[cfg(feature = "codec_oer")]
pub use decode::CoerDecodeErrorKind;
#[cfg(feature = "codec_ber")]
pub use decode::DerDecodeErrorKind;
#[cfg(feature = "codec_jer")]
pub use decode::JerDecodeErrorKind;
#[cfg(feature = "codec_oer")]
pub use decode::OerDecodeErrorKind;
#[cfg(feature = "codec_xer")]
pub use decode::XerDecodeErrorKind;

pub use encode::CodecEncodeError;
pub use encode::EncodeError;
pub use encode::EncodeErrorKind;

#[cfg(feature = "codec_avn")]
pub use encode::AvnEncodeErrorKind;
#[cfg(feature = "codec_ber")]
pub use encode::BerEncodeErrorKind;
#[cfg(feature = "codec_oer")]
pub use encode::CoerEncodeErrorKind;
#[cfg(feature = "codec_ber")]
pub use encode::DerEncodeErrorKind;
#[cfg(feature = "codec_jer")]
pub use encode::JerEncodeErrorKind;
#[cfg(feature = "codec_xer")]
pub use encode::XerEncodeErrorKind;

pub use components::InnerSubtypeConstraintError;
