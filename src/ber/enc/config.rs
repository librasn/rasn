use crate::ber::EncodingRules;

/// Options for configuring the [`Encoder`][super::Encoder].
#[derive(Clone, Copy, Debug)]
pub struct EncoderOptions {
    pub(crate) encoding_rules: EncodingRules,
}

impl EncoderOptions {
    /// Return the default configuration for BER.
    #[must_use]
    pub const fn ber() -> Self {
        Self {
            encoding_rules: EncodingRules::Ber,
        }
    }

    /// Return the default configuration for CER.
    #[must_use]
    pub const fn cer() -> Self {
        Self {
            encoding_rules: EncodingRules::Cer,
        }
    }

    /// Return the default configuration for DER.
    #[must_use]
    pub const fn der() -> Self {
        Self {
            encoding_rules: EncodingRules::Der,
        }
    }

    /// Returns the currently selected codec.
    #[must_use]
    pub fn current_codec(&self) -> crate::Codec {
        self.encoding_rules.codec()
    }
}
