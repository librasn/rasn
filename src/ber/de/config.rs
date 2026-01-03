use crate::ber::EncodingRules;

/// The options for the [`Decoder`][super::Decoder].
#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    pub(crate) encoding_rules: EncodingRules,
    // limit decoding to prevent stack overflow from deep or circular references
    pub(crate) remaining_depth: usize,
}

impl DecoderOptions {
    /// Return the default configuration for BER.
    #[must_use]
    pub const fn ber() -> Self {
        Self {
            encoding_rules: EncodingRules::Ber,
            remaining_depth: 128,
        }
    }

    /// Return the default configuration for CER.
    #[must_use]
    pub const fn cer() -> Self {
        Self {
            encoding_rules: EncodingRules::Cer,
            remaining_depth: 128,
        }
    }

    /// Return the default configuration for DER.
    #[must_use]
    pub const fn der() -> Self {
        Self {
            encoding_rules: EncodingRules::Der,
            remaining_depth: 128,
        }
    }

    /// Returns the currently selected codec.
    #[must_use]
    pub fn current_codec(&self) -> crate::Codec {
        match self.encoding_rules {
            EncodingRules::Ber => crate::Codec::Ber,
            EncodingRules::Cer => crate::Codec::Cer,
            EncodingRules::Der => crate::Codec::Der,
        }
    }
}
