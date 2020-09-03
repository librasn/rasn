use crate::tag::Tag;
use snafu::Snafu;

pub(crate) fn assert_tag(expected: Tag, actual: Tag) -> super::Result<()> {
    if expected != actual {
        Err(Error::MismatchedTag { expected, actual })
    } else {
        Ok(())
    }
}

pub(crate) fn assert_length(expected: usize, actual: usize) -> super::Result<()> {
    if expected != actual {
        Err(Error::MismatchedLength { expected, actual })
    } else {
        Ok(())
    }
}

pub(crate) fn map_nom_err(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Error {
    use nom::{Err, Needed};
    let msg = match error {
        Err::Incomplete(Needed::Size(u)) => alloc::format!("Parsing requires {} bytes/chars", u),
        Err::Incomplete(Needed::Unknown) => alloc::format!("Parsing requires more data"),
        Err::Failure(c) => alloc::format!("Parsing Failure: {:?}", c),
        Err::Error(c) => alloc::format!("Parsing Error: {:?}", c),
    };

    Error::Parser { msg }
}

#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    #[snafu(display("Invalid UTF-8"))]
    InvalidUtf8,
    #[snafu(display("Invalid Date"))]
    InvalidDate,
    #[snafu(display("Error in Parser\n{}", msg))]
    Parser { msg: alloc::string::String },
    #[snafu(display("Expected {:?} tag, actual tag: {:?}", expected, actual))]
    MismatchedTag { expected: Tag, actual: Tag },
    #[snafu(display("Expected {:?} bytes, actual length: {:?}", expected, actual))]
    MismatchedLength { expected: usize, actual: usize },
    #[snafu(display("Actual larger than expected {} bits", max_width))]
    IntegerOverflow { max_width: u32 },
    #[snafu(display("BitString contains an invalid amount of unused bits: {}", bits))]
    InvalidBitString { bits: u8 },
    #[snafu(display("{}", msg))]
    Custom { msg: alloc::string::String },
}

impl crate::de::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}
