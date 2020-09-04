#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingRules {
    Ber,
    Cer,
    Der,
}

impl EncodingRules {
    pub fn is_ber(self) -> bool {
        match self {
            Self::Ber => true,
            _ => false,
        }
    }

    pub fn is_cer(self) -> bool {
        match self {
            Self::Cer => true,
            _ => false,
        }
    }

    pub fn is_der(self) -> bool {
        match self {
            Self::Der => true,
            _ => false,
        }
    }

    pub fn allows_constructed_strings(self) -> bool {
        match self {
            Self::Ber => true,
            _ => false,
        }
    }

    pub fn allows_indefinite(self) -> bool {
        match self {
            Self::Der => false,
            _ => true,
        }
    }

    pub fn max_string_length(self) -> usize {
        match self {
            Self::Cer => 1000,
            _ => usize::MAX,
        }
    }
}
