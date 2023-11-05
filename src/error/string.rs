use alloc::string::String;

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid BMP string, character decimal value: {}", character))]
pub struct InvalidBmpString {
    pub character: u16,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid general string, character decimal value: {}", character))]
pub struct InvalidGeneralString {
    pub character: u8,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid ISO 646/ASCII, character decimal value: {}", character))]
pub struct InvalidIso646Character {
    pub character: u8,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid numeric string, character decimal value: {}", character))]
pub struct InvalidNumericString {
    pub character: u8,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("Invalid printable string, character decimal value: {}", character))]
pub struct InvalidPrintableString {
    pub character: u32,
}

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
    IndexNotFound { index: u32 },
    #[snafu(display(
        "Character with decimal value {} not found from the permitted list.",
        character
    ))]
    CharacterNotFound { character: u32 },
}
