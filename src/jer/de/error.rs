use jzon::JsonValue;
use snafu::Snafu;

/// An error that occurred when decoding JER.
#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Error {
    #[snafu(display("Unexpected end of input while decoding JER JSON."))]
    EndOfInput {},
    #[snafu(display(
        "Found mismatching JSON value. Expected type {}. Found value {}.",
        needed,
        found
    ))]
    TypeMismatch {
        needed: &'static str,
        found: alloc::string::String,
    },
    #[snafu(display("Need more bytes to continue ({:?}).", needed))]
    ExceedsMaxLength {
        /// Amount of bytes needed.
        needed: num_bigint::BigUint,
    },
    #[snafu(display("Need more bytes to continue ({:?}).", needed))]
    Incomplete {
        /// Amount of bytes needed.
        needed: nom::Needed,
    },
    #[snafu(display("Error in Parser: {}", msg))]
    Parser {
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display("Missing field `{}`", name))]
    MissingField {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("Error when decoding field `{}`: {}", name, msg))]
    FieldError {
        /// The field's name.
        name: &'static str,
        msg: alloc::string::String,
    },
    #[snafu(display("Duplicate field for `{}`", name))]
    DuplicateField {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("No valid choice for `{}`", name))]
    NoValidChoice {
        /// The field's name.
        name: &'static str,
    },
    #[snafu(display("No valid enumerated variant for `{}`", discriminator))]
    NoValidVariant {
        /// The variant's discriminator.
        discriminator: JsonValue,
    },
    #[snafu(display("Custom: {}", msg))]
    Custom {
        /// The error's message.
        msg: alloc::string::String,
    },
}

impl Error {
    pub fn eoi() -> Self {
        Self::EndOfInput {}
    }

    pub fn no_valid_variant(discriminator: JsonValue) -> Self {
        Self::NoValidVariant { discriminator }
    }
}

impl crate::de::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::format!("{msg}"),
        }
    }

    fn incomplete(needed: nom::Needed) -> Self {
        Self::Incomplete { needed }
    }

    fn exceeds_max_length(needed: num_bigint::BigUint) -> Self {
        Self::ExceedsMaxLength { needed }
    }

    fn missing_field(name: &'static str) -> Self {
        Self::MissingField { name }
    }

    fn field_error<D: core::fmt::Display>(name: &'static str, error: D) -> Self {
        Self::FieldError {
            name,
            msg: alloc::format!("{error}"),
        }
    }

    fn duplicate_field(name: &'static str) -> Self {
        Self::DuplicateField { name }
    }

    fn no_valid_choice(name: &'static str) -> Self {
        Self::NoValidChoice { name }
    }
}

impl From<crate::ber::de::Error> for Error {
    fn from(value: crate::ber::de::Error) -> Self {
        match value {
            crate::der::de::Error::InvalidDate => Self::Custom {
                msg: "Error parsing time string!".into(),
            },
            e => Self::Custom {
                msg: alloc::format!("Error invoking BER parser: {e:?}"),
            },
        }
    }
}

impl From<jzon::Error> for Error {
    fn from(value: jzon::Error) -> Self {
        Self::Custom { msg: alloc::format!("Error decoding JSON: {value:?}") }
    }
}