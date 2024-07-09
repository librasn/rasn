use alloc::string::String;

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid BMP string, character decimal value: {}", character))]
pub struct InvalidBmpString {
    pub character: u32,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid general string, character decimal value: {}", character))]
pub struct InvalidGeneralString {
    pub character: u32,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid ISO 646/ASCII, character decimal value: {}", character))]
pub struct InvalidIA5String {
    pub character: u32,
}
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid teletex string, character decimal value: {}", character))]
pub struct InvalidTeletexString {
    pub character: u32,
}
#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid visible string: only space (0x20) and all graphically visible characters (0x21-0x7E) allowed, character decimal value: {}", character))]
pub struct InvalidVisibleString {
    pub character: u32,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid numeric string, character decimal value: {}", character))]
pub struct InvalidNumericString {
    pub character: u32,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid printable string, character decimal value: {}", character))]
pub struct InvalidPrintableString {
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
    InvalidIA5String,
    InvalidNumericString,
    InvalidPrintableString,
    InvalidTeletexString,
    InvalidVisibleString
);

#[derive(Debug)]
pub enum InvalidRestrictedString {
    InvalidBmpString(InvalidBmpString),
    InvalidGeneralString(InvalidGeneralString),
    InvalidIA5String(InvalidIA5String),
    InvalidNumericString(InvalidNumericString),
    InvalidPrintableString(InvalidPrintableString),
    InvalidTeletexString(InvalidTeletexString),
    InvalidVisibleString(InvalidVisibleString),
}

impl core::fmt::Display for InvalidRestrictedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidRestrictedString::InvalidBmpString(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidGeneralString(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidIA5String(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidNumericString(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidPrintableString(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidVisibleString(e) => write!(f, "{}", e),
            InvalidRestrictedString::InvalidTeletexString(e) => write!(f, "{}", e),
        }
    }
}
impl snafu::Error for InvalidRestrictedString {}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum PermittedAlphabetError {
    #[snafu(display("error converting to string: {}", message))]
    Other { message: String },
    #[snafu(display(
        "length of bits ({length}) provided not divisible by character width ({width})"
    ))]
    InvalidData { length: usize, width: usize },
    #[snafu(display("index not found {}", 0))]
    IndexNotFound { index: usize },
    #[snafu(display(
        "Character with decimal value {} not found from the permitted list.",
        character
    ))]
    CharacterNotFound { character: u32 },
    #[snafu(display("Invalid alphabet constrained string: {}", source))]
    InvalidRestrictedString {
        #[snafu(source)]
        source: InvalidRestrictedString,
    },
}
