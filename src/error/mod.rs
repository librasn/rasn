pub(crate) mod decode;
pub(crate) mod encode;

pub use decode::DecodeError;
pub use decode::Kind as DecodeErrorKind;
pub use encode::BerEncodeError;
pub use encode::CodecEncodeError;
pub use encode::EncodeError;
pub use encode::Kind as EncodeErrorKind;

// pub trait CodecError {
//     fn codec(&self) -> crate::Codec;
// }
