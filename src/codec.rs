use crate::prelude::*;

use snafu::*;

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

impl Codec {
    pub fn encode<T: Encode>(self, value: &T) -> Result<Vec<u8>, EncodeError> {
        match self {
            Self::Aper => crate::aper::encode(value).context(AperSnafu),
            Self::Ber => crate::ber::encode(value).context(BerSnafu),
            Self::Cer => crate::cer::encode(value).context(CerSnafu),
            Self::Der => crate::der::encode(value).context(DerSnafu),
            Self::Uper => crate::uper::encode(value).context(UperSnafu),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum EncodeError {
    Aper{ source: crate::aper::enc::Error },
    Ber{ source: crate::ber::enc::Error },
    Cer{ source: crate::der::enc::Error },
    Der{ source: crate::der::enc::Error },
    Uper{ source: crate::uper::enc::Error },
}
