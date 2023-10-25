//! Error module includes all encode and decode errors among all codecs.
//! Encoding can result to `EncodeError` and decoding can result to `DecodeError`.

#![allow(clippy::module_name_repetitions)]
mod decode;
mod encode;
pub use decode::Kind as DecodeErrorKind;
pub use decode::{BerDecodeErrorKind, CodecDecodeError, DecodeError, DerDecodeErrorKind};
pub use encode::Kind as EncodeErrorKind;
pub use encode::{BerEncodeErrorKind, CodecEncodeError, EncodeError};
