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
    /// This method shall be used when using binary-based encoding rules.
    ///
    /// # Errors
    /// - If the value fails to be encoded, or if trying to encode using
    ///   text-based encoding rules, returns `EncodeError` struct.
    pub fn encode_to_binary<T: Encode>(
        self,
        value: &T,
    ) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
        match self {
            Self::Aper => crate::aper::encode(value),
            Self::Ber => crate::ber::encode(value),
            Self::Cer => crate::cer::encode(value),
            Self::Der => crate::der::encode(value),
            Self::Uper => crate::uper::encode(value),
            Self::Jer => Err(crate::error::EncodeError::from_kind(
                crate::error::EncodeErrorKind::Custom {
                    msg: "JER is a text-based encoding. Call `Codec::encode_to_string` instead."
                        .into(),
                },
                Codec::Jer,
            )),
        }
    }

    /// Decodes `input` to `D` based on the value of `Codec`.
    /// This method shall be used when using binary-based encoding rules.
    ///
    /// # Errors
    /// - If `D` cannot be decoded from `input`, or if trying to decode using
    ///   text-based encoding rules, returns `DecodeError` struct.
    pub fn decode_from_binary<D: Decode>(
        &self,
        input: &[u8],
    ) -> Result<D, crate::error::DecodeError> {
        match self {
            Self::Aper => crate::aper::decode(input),
            Self::Ber => crate::ber::decode(input),
            Self::Cer => crate::cer::decode(input),
            Self::Der => crate::der::decode(input),
            Self::Uper => crate::uper::decode(input),
            Self::Jer => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "JER is a text-based encoding. Call `Codec::decode_from_str` instead."
                        .into(),
                },
                Codec::Jer,
            )),
        }
    }

    /// Encodes a given value based on the value of `Codec`.
    /// This method shall be used when using text-based encoding rules.
    ///
    /// # Errors
    /// - If the value fails to be encoded, or if trying to encode using
    ///   binary-based encoding rules, returns `EncodeError` struct.
    pub fn encode_to_string<T: Encode>(
        self,
        value: &T,
    ) -> Result<alloc::string::String, crate::error::EncodeError> {
        match self {
            Self::Jer => crate::jer::encode(value),
            codec => Err(crate::error::EncodeError::from_kind(
                crate::error::EncodeErrorKind::Custom {
                    msg: alloc::format!("{codec} is a binary-based encoding. Call `Codec::encode_to_binary` instead."),
                },
                codec,
            )),
        }
    }

    /// Decodes `input` to `D` based on the value of `Codec`.
    /// This method shall be used when using text-based encoding rules.
    ///
    /// # Errors
    /// - If `D` cannot be decoded from `input`, or if trying to decode using
    ///   binary-based encoding rules, returns `DecodeError` struct.
    pub fn decode_from_str<D: Decode>(&self, input: &str) -> Result<D, crate::error::DecodeError> {
        match self {
            Self::Jer => crate::jer::decode(input),
            codec => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: alloc::format!("{codec} is a text-based encoding. Call `Codec::decode_from_binary` instead."),
                },
                codec.clone(),
            )),
        }
    }
}
