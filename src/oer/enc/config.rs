/// Options for configuring the [`Encoder`][super::Encoder].
#[derive(Clone, Copy, Debug)]
pub struct EncoderOptions {
    pub(crate) encoding_rules: EncodingRules,
}

impl EncoderOptions {
    // Return the default configuration for COER.
    // We reserve the possibility to use OER in the future by using the rules.
    #[must_use]
    pub const fn coer() -> Self {
        Self {
            encoding_rules: EncodingRules::Coer,
        }
    }
}
impl Default for EncoderOptions {
    fn default() -> Self {
        Self::coer()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingRules {
    Coer,
}

impl EncodingRules {
    pub fn is_coer(self) -> bool {
        matches!(self, Self::Coer)
    }
}
