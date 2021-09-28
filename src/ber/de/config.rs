use crate::ber::EncodingRules;

/// The options for the [`Decoder`][super::Decoder].
#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    pub(crate) encoding_rules: EncodingRules,
}

impl DecoderOptions {
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
}
