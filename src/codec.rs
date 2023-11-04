use crate::prelude::*;

/// A set of supported ASN.1 codecs. Can be used to dynamically encode types
/// into different codecs at runtime.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Codec {
    /// X.691 — Packed Encoding Rules (Aligned)
    Aper,
    /// X.690 — Basic Encoding Rules
    Ber,
    /// X.690 — Canonical Encoding Rules
    Cer,
    /// X.690 — Distinguished Encoding Rules
    Der,
    /// X.691 — Packed Encoding Rules (Unaligned)
    Uper,
    /// [JSON Encoding Rules](https://obj-sys.com/docs/JSONEncodingRules.pdf)
    Jer,
}

impl core::fmt::Display for Codec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Aper => write!(f, "APER"),
            Self::Ber => write!(f, "BER"),
            Self::Cer => write!(f, "CER"),
            Self::Der => write!(f, "DER"),
            Self::Uper => write!(f, "UPER"),
            Self::Jer => write!(f, "JER"),
        }
    }
}

impl Codec {
    /// Encodes a given value based on the value of `Codec`.
    ///
    /// # Errors
    /// - If the value fails to be encoded, returns `EncodeError` struct.
    pub fn encode<T: Encode>(
        self,
        value: &T,
    ) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
        match self {
            Self::Aper => crate::aper::encode(value),
            Self::Ber => crate::ber::encode(value),
            Self::Cer => crate::cer::encode(value),
            Self::Der => crate::der::encode(value),
            Self::Uper => crate::uper::encode(value),
            Self::Jer => crate::jer::encode(value).map(|s| s.as_bytes().to_vec()),
        }
    }

    /// Decodes `input` to `D` based on the value of `Codec`.
    ///
    /// # Errors
    /// - If `D` cannot be decoded from `input`, returns `DecodeError` struct.
    pub fn decode<D: Decode>(&self, input: &[u8]) -> Result<D, crate::error::DecodeError> {
        match self {
            Self::Aper => crate::aper::decode(input),
            Self::Ber => crate::ber::decode(input),
            Self::Cer => crate::cer::decode(input),
            Self::Der => crate::der::decode(input),
            Self::Uper => crate::uper::decode(input),
            Self::Jer => crate::jer::decode(
                &alloc::string::String::from_utf8(input.to_vec()).map_err(|_| {
                    crate::error::DecodeError::from_kind(
                        crate::error::DecodeErrorKind::Custom {
                            msg: "Failed to parse binary UTF8 JSON String.".into(),
                        },
                        Codec::Jer,
                    )
                })?,
            ),
        }
    }
}
