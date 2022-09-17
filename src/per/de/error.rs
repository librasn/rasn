use snafu::*;

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
pub struct Error {
    kind: Kind,
    backtrace: Backtrace,
}

impl Error {
    pub fn range_exceeds_platform_width(needed: u32, present: u32) -> Self {
        Self {
            kind: Kind::RangeExceedsPlatformWidth { needed, present },
            backtrace: Backtrace::generate(),
        }
    }
}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            backtrace: Backtrace::generate(),
        }
    }
}

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Kind {
    #[snafu(display("integer range larger than possible to address on this platform. needed: {needed} present: {present}"))]
    RangeExceedsPlatformWidth {
        /// Amount of bytes needed.
        needed: u32,
        /// Amount of bytes needed.
        present: u32,
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
    #[snafu(display("Error in wrapped BER: {}", source))]
    Ber {
        /// The error's message.
        source: crate::ber::de::Error,
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
    #[snafu(display("Custom: {}", msg))]
    Custom {
        /// The error's message.
        msg: alloc::string::String,
    },
}

impl crate::de::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::from(Kind::Custom {
            msg: msg.to_string(),
        })
    }

    fn incomplete(needed: nom::Needed) -> Self {
        Self::from(Kind::Incomplete { needed })
    }

    fn exceeds_max_length(needed: num_bigint::BigUint) -> Self {
        Self::from(Kind::ExceedsMaxLength { needed })
    }

    fn missing_field(name: &'static str) -> Self {
        Self::from(Kind::MissingField { name })
    }

    fn field_error<D: core::fmt::Display>(name: &'static str, error: D) -> Self {
        Self::from(Kind::FieldError { name, msg: error.to_string() })
    }

    fn duplicate_field(name: &'static str) -> Self {
        todo!()
    }

    fn no_valid_choice(name: &'static str) -> Self {
        todo!()
    }
}

impl From<nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>> for Error {
    fn from(
        error: nom::Err<nom::error::Error<nom_bitvec::BSlice<'_, u8, bitvec::order::Msb0>>>,
    ) -> Self {
        let msg = match error {
            nom::Err::Incomplete(needed) => return Self::from(Kind::Incomplete { needed }),
            err => alloc::format!("Parsing Failure: {}", err),
        };

        Self::from(Kind::Parser { msg })
    }
}
