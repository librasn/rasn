use crate::tag::Tag;
use snafu::Snafu;

#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    Parser,
    #[snafu(display("Expected {:?} tag, actual tag: {:?}", expected, actual))]
    MismatchedTag {
        expected: Tag,
        actual: Tag,
    },
    #[snafu(display("Expected {:?} bytes, actual length: {:?}", expected, actual))]
    MismatchedLength {
        expected: usize,
        actual: usize,
    },
    #[snafu(display("Actual larger than expected {} bits", max_width))]
    IntegerOverflow {
        max_width: u32,
    },
    #[snafu(display("{}", msg))]
    Custom {
        msg: alloc::string::String,
    }
}

impl crate::error::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        use alloc::string::ToString;
        Self::Custom {
            msg: msg.to_string(),
        }
    }
}
