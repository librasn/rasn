use crate::prelude::*;

/// A set of supported ASN.1 codecs. Can be used to dynamically encode types
/// into different codecs at runtime.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Codec {
    #[cfg(feature = "codec_per")]
    /// X.691 — Packed Encoding Rules (Aligned)
    Aper,
    #[cfg(feature = "codec_ber")]
    /// X.690 — Basic Encoding Rules
    Ber,
    #[cfg(feature = "codec_ber")]
    /// X.690 — Canonical Encoding Rules
    Cer,
    #[cfg(feature = "codec_ber")]
    /// X.690 — Distinguished Encoding Rules
    Der,
    #[cfg(feature = "codec_per")]
    /// X.691 — Packed Encoding Rules (Unaligned)
    Uper,
    #[cfg(feature = "codec_jer")]
    /// [JSON Encoding Rules](https://obj-sys.com/docs/JSONEncodingRules.pdf)
    Jer,
    #[cfg(feature = "codec_oer")]
    /// X.696 — Octet Encoding Rules
    Oer,
    #[cfg(feature = "codec_oer")]
    /// X.696 — Canonical Octet Encoding Rules
    Coer,
    #[cfg(feature = "codec_xer")]
    /// X.693 — XML Encoding Rules
    Xer,
    #[cfg(feature = "codec_avn")]
    /// ASN.1 Value Notation (X.680 text format)
    Avn,
}

impl core::fmt::Display for Codec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "codec_per")]
            Self::Aper => write!(f, "APER"),
            #[cfg(feature = "codec_ber")]
            Self::Ber => write!(f, "BER"),
            #[cfg(feature = "codec_ber")]
            Self::Cer => write!(f, "CER"),
            #[cfg(feature = "codec_ber")]
            Self::Der => write!(f, "DER"),
            #[cfg(feature = "codec_per")]
            Self::Uper => write!(f, "UPER"),
            #[cfg(feature = "codec_jer")]
            Self::Jer => write!(f, "JER"),
            #[cfg(feature = "codec_oer")]
            Self::Oer => write!(f, "OER"),
            #[cfg(feature = "codec_oer")]
            Self::Coer => write!(f, "COER"),
            #[cfg(feature = "codec_xer")]
            Self::Xer => write!(f, "XER"),
            #[cfg(feature = "codec_avn")]
            Self::Avn => write!(f, "AVN"),
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
            #[cfg(feature = "codec_per")]
            Self::Aper => crate::aper::encode(value),
            #[cfg(feature = "codec_ber")]
            Self::Ber => crate::ber::encode(value),
            #[cfg(feature = "codec_ber")]
            Self::Cer => crate::cer::encode(value),
            #[cfg(feature = "codec_ber")]
            Self::Der => crate::der::encode(value),
            #[cfg(feature = "codec_per")]
            Self::Uper => crate::uper::encode(value),
            #[cfg(feature = "codec_jer")]
            Self::Jer => crate::jer::encode(value).map(alloc::string::String::into_bytes),
            #[cfg(feature = "codec_oer")]
            Self::Oer => crate::oer::encode(value),
            #[cfg(feature = "codec_oer")]
            Self::Coer => crate::coer::encode(value),
            #[cfg(feature = "codec_xer")]
            Self::Xer => crate::xer::encode(value),
            #[cfg(feature = "codec_avn")]
            Self::Avn => crate::avn::encode(value).map(alloc::string::String::into_bytes),
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
            #[cfg(feature = "codec_per")]
            Self::Aper => crate::aper::decode(input),
            #[cfg(feature = "codec_ber")]
            Self::Ber => crate::ber::decode(input),
            #[cfg(feature = "codec_ber")]
            Self::Cer => crate::cer::decode(input),
            #[cfg(feature = "codec_ber")]
            Self::Der => crate::der::decode(input),
            #[cfg(feature = "codec_per")]
            Self::Uper => crate::uper::decode(input),
            #[cfg(feature = "codec_oer")]
            Self::Oer => crate::oer::decode(input),
            #[cfg(feature = "codec_oer")]
            Self::Coer => crate::coer::decode(input),
            #[cfg(feature = "codec_xer")]
            Self::Xer => crate::xer::decode(input),
            #[cfg(feature = "codec_jer")]
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
            #[cfg(feature = "codec_avn")]
            Self::Avn => alloc::string::String::from_utf8(input.to_vec()).map_or_else(
                |e| {
                    Err(crate::error::DecodeError::from_kind(
                        crate::error::DecodeErrorKind::Custom {
                            msg: alloc::format!("Failed to decode AVN from UTF8 bytes: {e:?}"),
                        },
                        *self,
                    ))
                },
                |s| crate::avn::decode(&s),
            ),
        }
    }
    /// Decodes `input` to `D` based on the encoded defined by `Codec`, returning the decoded value and the remaining input.
    pub fn decode_from_binary_with_remainder<'input, D: Decode>(
        &self,
        input: &'input [u8],
    ) -> Result<(D, &'input [u8]), crate::error::DecodeError> {
        match self {
            #[cfg(feature = "codec_per")]
            Self::Aper => crate::aper::decode_with_remainder(input),
            #[cfg(feature = "codec_ber")]
            Self::Ber => crate::ber::decode_with_remainder(input),
            #[cfg(feature = "codec_ber")]
            Self::Cer => crate::cer::decode_with_remainder(input),
            #[cfg(feature = "codec_ber")]
            Self::Der => crate::der::decode_with_remainder(input),
            #[cfg(feature = "codec_per")]
            Self::Uper => crate::uper::decode_with_remainder(input),
            #[cfg(feature = "codec_oer")]
            Self::Oer => crate::oer::decode_with_remainder(input),
            #[cfg(feature = "codec_oer")]
            Self::Coer => crate::coer::decode_with_remainder(input),
            #[cfg(feature = "codec_xer")]
            Self::Xer => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "XER does not support decoding with remainder. ".into(),
                },
                *self,
            )),
            #[cfg(feature = "codec_jer")]
            Self::Jer => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "JER does not support decoding with remainder. ".into(),
                },
                *self,
            )),
            #[cfg(feature = "codec_avn")]
            Self::Avn => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: "AVN does not support decoding with remainder. ".into(),
                },
                *self,
            )),
        }
    }

    #[cfg(any(feature = "codec_jer", feature = "codec_avn"))]
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
            #[cfg(feature = "codec_jer")]
            Self::Jer => crate::jer::encode(value),
            #[cfg(feature = "codec_avn")]
            Self::Avn => crate::avn::encode(value),
            codec => Err(crate::error::EncodeError::from_kind(
                crate::error::EncodeErrorKind::Custom {
                    msg: alloc::format!(
                        "{codec} is a binary-based encoding. Call `Codec::encode_to_binary` instead."
                    ),
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
    #[cfg_attr(
        not(any(feature = "codec_jer", feature = "codec_avn")),
        allow(unused_variables)
    )]
    pub fn decode_from_str<D: Decode>(&self, input: &str) -> Result<D, crate::error::DecodeError> {
        match self {
            #[cfg(feature = "codec_jer")]
            Self::Jer => crate::jer::decode(input),
            #[cfg(feature = "codec_avn")]
            Self::Avn => crate::avn::decode(input),
            codec => Err(crate::error::DecodeError::from_kind(
                crate::error::DecodeErrorKind::Custom {
                    msg: alloc::format!(
                        "{codec} is a binary-based encoding. Call `Codec::decode_from_binary` instead."
                    ),
                },
                *codec,
            )),
        }
    }
}
