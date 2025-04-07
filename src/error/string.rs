use alloc::string::String;

/// A character which does not match the allowed character set for BMP.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid BMP string, character decimal value: {}", character))]
pub struct InvalidBmpString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for General.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid general string, character decimal value: {}", character))]
pub struct InvalidGeneralString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for General.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid graphic string, character decimal value: {}", character))]
pub struct InvalidGraphicString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for IA5.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid ISO 646/ASCII, character decimal value: {}", character))]
pub struct InvalidIA5String {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for Teletex.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid teletex string, character decimal value: {}", character))]
pub struct InvalidTeletexString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for Visible.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid visible string: only space (0x20) and all graphically visible characters (0x21-0x7E) allowed, character decimal value: {}", character))]
pub struct InvalidVisibleString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for Numeric.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid numeric string, character decimal value: {}", character))]
pub struct InvalidNumericString {
    /// The invalid character.
    pub character: u32,
}

/// A character which does not match the allowed character set for Printable.
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid printable string, character decimal value: {}", character))]
pub struct InvalidPrintableString {
    /// The invalid character.
    pub character: u32,
}

macro_rules! from_u32 {
    ($($type:ident),*) => {
        $(
            impl From<u32> for $type {
                fn from(character: u32) -> Self {
                    $type { character }
                }
            }
        )*
    };
}
from_u32!(
    InvalidBmpString,
    InvalidGeneralString,
    InvalidGraphicString,
    InvalidIA5String,
    InvalidNumericString,
    InvalidPrintableString,
    InvalidTeletexString,
    InvalidVisibleString
);

/// The set of all possible error types for invalid string representation.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum InvalidRestrictedString {
    InvalidBmpString(InvalidBmpString),
    InvalidGeneralString(InvalidGeneralString),
    InvalidGraphicString(InvalidGraphicString),
    InvalidIA5String(InvalidIA5String),
    InvalidNumericString(InvalidNumericString),
    InvalidPrintableString(InvalidPrintableString),
    InvalidTeletexString(InvalidTeletexString),
    InvalidVisibleString(InvalidVisibleString),
}

impl core::fmt::Display for InvalidRestrictedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidRestrictedString::InvalidBmpString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidGeneralString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidGraphicString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidIA5String(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidNumericString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidPrintableString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidVisibleString(e) => write!(f, "{e}"),
            InvalidRestrictedString::InvalidTeletexString(e) => write!(f, "{e}"),
        }
    }
}
impl snafu::Error for InvalidRestrictedString {}

/// The set of errors that can occur when parsing a restricted permitted alphabet.
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum PermittedAlphabetError {
    /// Uncategorised error eith converting to string.
    #[snafu(display("error converting to string: {}", message))]
    Other {
        /// The error message.
        message: String,
    },
    /// The length of bits given isn't divisible by the width.
    #[snafu(display(
        "length of bits ({length}) provided not divisible by character width ({width})"
    ))]
    InvalidData {
        /// The length of data given.
        length: usize,
        /// The character width.
        width: usize,
    },
    /// The index found in the given data was not found in the character set.
    #[snafu(display("index not found {}", 0))]
    IndexNotFound {
        /// The index found in the data.
        index: usize,
    },
    /// The character found in the value was not included in the permitted list.
    #[snafu(display(
        "Character with decimal value {} not found from the permitted list.",
        character
    ))]
    CharacterNotFound {
        /// The character provided.
        character: u32,
    },
    /// An error specific to a known restricted string type.
    #[snafu(display("Invalid alphabet constrained string: {}", source))]
    InvalidRestrictedString {
        /// The inner error type.
        #[snafu(source)]
        source: InvalidRestrictedString,
    },
}
