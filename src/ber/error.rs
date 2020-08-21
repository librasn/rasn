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

#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    #[snafu(display("Invalid UTF-8 in UTF8String"))]
    InvalidUtf8,
    #[snafu(display("Error in BER Parser:\n{}", backtrace))]
    Parser { backtrace: snafu::Backtrace },
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

impl crate::error::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        use alloc::string::ToString;
        Self::Custom {
            msg: msg.to_string(),
        }
    }
}
