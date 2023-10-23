pub(crate) mod decode;
pub(crate) mod encode;

pub use decode::Kind as DecodeErrorKind;
pub use decode::{BerDecodeErrorKind, CodecDecodeError, DecodeError, DerDecodeErrorKind};
pub use encode::Kind as EncodeErrorKind;
pub use encode::{BerEncodeError, CodecEncodeError, EncodeError};

// pub trait CodecError {
//     fn codec(&self) -> crate::Codec;
// }
