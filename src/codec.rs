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
    /// X.696 — Octet Encoding Rules
    Oer,
    /// X.696 — Canonical Octet Encoding Rules
    Coer,
    /// X.693 — XML Encoding Rules
    Xer,
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
            Self::Oer => write!(f, "OER"),
            Self::Coer => write!(f, "COER"),
            Self::Xer => write!(f, "XER"),
        }
    }
}

impl Codec {
    /// Encodes a given value based on the value of `Codec`.
    /// This method shall be used when using binary-based encoding rules.
    ///
    /// # Errors
    /// - If the value fails to be encoded returns `EncodeError` struct.
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
            Self::Jer => crate::jer::encode(value).map(alloc::string::String::into_bytes),
            Self::Oer => crate::oer::encode(value),
            Self::Coer => crate::coer::encode(value),
            Self::Xer => crate::xer::encode(value),
        }
    }

    /// Decodes `input` to `D` based on the value of `Codec`.
    /// This method shall be used when using binary-based encoding rules.
    ///
    /// # Errors
    /// - If `D` cannot be decoded from `input` returns `DecodeError` struct.
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
            Self::Oer => crate::oer::decode(input),
            Self::Coer => crate::coer::decode(input),
            Self::Xer => crate::xer::decode(input),
            Self::Jer => alloc::string::String::from_utf8(input.to_vec()).map_or_else(
                |e| {
                    Err(crate::error::DecodeError::from_kind(
                        crate::error::DecodeErrorKind::Custom {
                            msg: alloc::format!("Failed to decode JER from UTF8 bytes: {e:?}"),
                        },
                        *self,
                    ))
                },
                |s| crate::jer::decode(&s),
            ),
        }
    }
    /// Decodes `input` to `D` based on the encoded defined by `Codec`, returning the decoded value and the remaining input.
    pub fn decode_from_binary_with_remainder<'input, D: Decode>(
        &self,
        input: &'input [u8],
    ) -> Result<(D, &'input [u8]), crate::error::DecodeError> {
        match self {
            Self::Aper => crate::aper::decode_with_remainder(input),
            Self::Ber => crate::ber::decode_with_remainder(input),
            Self::Cer => crate::cer::decode_with_remainder(input),
            Self::Der => crate::der::decode_with_remainder(input),
            Self::Uper => crate::uper::decode_with_remainder(input),
            Self::Oer => crate::oer::decode_with_remainder(input),
            Self::Coer => crate::coer::decode_with_remainder(input),
            Self::Xer => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "XER does not support decoding with remainder. ".into(),
                },
                *self,
            )),
            Self::Jer => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "JER does not support decoding with remainder. ".into(),
                },
                *self,
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
                *codec,
            )),
        }
    }
}
