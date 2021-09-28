#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingRules {
    Ber,
    Cer,
    Der,
}

impl EncodingRules {
    pub fn is_ber(self) -> bool {
        matches!(self, Self::Ber)
    }

    pub fn is_cer(self) -> bool {
        matches!(self, Self::Cer)
    }

    pub fn is_der(self) -> bool {
        matches!(self, Self::Der)
    }

    pub fn allows_constructed_strings(self) -> bool {
        matches!(self, Self::Ber | Self::Cer)
    }

    pub fn allows_indefinite(self) -> bool {
        matches!(self, Self::Ber | Self::Cer)
    }

    pub fn max_string_length(self) -> usize {
        match self {
            Self::Cer => 1000,
            _ => usize::MAX,
        }
    }
}
