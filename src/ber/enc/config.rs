use crate::ber::EncodingRules;

/// Options for configuring the [`Encoder`][super::Encoder].
#[derive(Clone, Copy, Debug)]
pub struct EncoderOptions {
    pub(crate) encoding_rules: EncodingRules,
}

impl EncoderOptions {
    /// Return the default configuration for BER.
    pub const fn ber() -> Self {
        Self {
            encoding_rules: EncodingRules::Ber,
        }
    }

    /// Return the default configuration for CER.
    pub const fn cer() -> Self {
        Self {
            encoding_rules: EncodingRules::Cer,
        }
    }

    /// Return the default configuration for DER.
    pub const fn der() -> Self {
        Self {
            encoding_rules: EncodingRules::Der,
        }
    }
    pub fn current_codec(&self) -> crate::Codec {
        match self.encoding_rules {
            EncodingRules::Ber => crate::Codec::Ber,
            EncodingRules::Cer => crate::Codec::Cer,
            EncodingRules::Der => crate::Codec::Der,
        }
    }
}
