use snafu::*;

use crate::prelude::*;

// pub use self::{de::DecodeError, enc::EncodeError};
pub use self::de::DecodeError;

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
}
impl core::fmt::Display for Codec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Aper => write!(f, "APER"),
            Self::Ber => write!(f, "BER"),
            Self::Cer => write!(f, "CER"),
            Self::Der => write!(f, "DER"),
            Self::Uper => write!(f, "UPER"),
        }
    }
}

impl Codec {
    /// Encodes a given value based on the value of `Codec`.
    ///
    /// # Errors
    /// - If the value fails to be encoded.
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
        }
    }

    /// Decodes `input` to `D` based on the value of `Codec`.
    ///
    /// # Errors
    /// - If `D` cannot be decoded from `input`.
    pub fn decode<D: Decode>(&self, input: &[u8]) -> Result<D, DecodeError> {
        match self {
            Self::Aper => crate::aper::decode(input).context(de::AperSnafu),
            Self::Ber => crate::ber::decode(input).context(de::BerSnafu),
            Self::Cer => crate::cer::decode(input).context(de::CerSnafu),
            Self::Der => crate::der::decode(input).context(de::DerSnafu),
            Self::Uper => crate::uper::decode(input).context(de::UperSnafu),
        }
    }
}

mod enc {
    // use super::*;

    // #[derive(Debug, Snafu)]
    // #[snafu(visibility(pub(crate)))]
    // pub enum EncodeError {
    //     #[snafu(display("APER Error: {}", source))]
    //     Aper { source: crate::aper::enc::Error },
    //     #[snafu(display("BER Error: {}", source))]
    //     Ber { source: crate::ber::enc::Error },
    //     #[snafu(display("CER Error: {}", source))]
    //     Cer { source: crate::der::enc::Error },
    //     #[snafu(display("DER Error: {}", source))]
    //     Der { source: crate::der::enc::Error },
    //     #[snafu(display("UPER Error: {}", source))]
    //     Uper { source: crate::uper::enc::Error },
    // }
}

mod de {
    use super::*;

    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum DecodeError {
        #[snafu(display("APER Error: {}", source))]
        Aper { source: crate::aper::de::Error },
        #[snafu(display("BER Error: {}", source))]
        Ber { source: crate::ber::de::Error },
        #[snafu(display("CER Error: {}", source))]
        Cer { source: crate::der::de::Error },
        #[snafu(display("DER Error: {}", source))]
        Der { source: crate::der::de::Error },
        #[snafu(display("UPER Error: {}", source))]
        Uper { source: crate::uper::de::Error },
    }
}
