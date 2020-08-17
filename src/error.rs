use crate::identifier::Tag;
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
}
