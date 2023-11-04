use snafu::*;

/// An error that ocurred when encoding BER or any of its variants.
#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Error {
    /// Upstream `serde` error
    JsonEncodingError { upstream: alloc::string::String },
    /// Error to be thrown when the JER encoder contains no encoded root value
    #[snafu(display("No encoded root value found!"))]
    NoRootValueFound,
    #[snafu(display("Error in Parser: {}", msg))]
    Parser {
        /// The error's message.
        msg: alloc::string::String,
    },
    #[snafu(display("Exceeds supported integer range -2^63..2^63 ({:?}).", value))]
    ExceedsSupportedIntSize {
        /// value failed to encode
        value: num_bigint::BigInt,
    },
    #[snafu(display("Invalid character: {:?}", error))]
    InvalidCharacter {
        /// value failed to encode
        error: alloc::string::FromUtf8Error,
    },
    /// A custom error.
    #[snafu(display("Custom Error:\n{}", msg))]
    Custom {
        /// The custom error's message.
        msg: alloc::string::String,
    },
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}

impl From<jzon::Error> for Error {
    fn from(value: jzon::Error) -> Self {
        Self::JsonEncodingError {
            upstream: alloc::format!("Encountered an error during JER-encoding: {value:#?}"),
        }
    }
}
