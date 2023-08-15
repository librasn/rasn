use crate::types::constraints::Bounded;
use crate::types::Integer;
use alloc::string::{String, ToString};
use snafu::*;
// use x509_parser::der_parser::asn1_rs::Integer;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Integer not in the constraint range {expected}; actual: {value}"))]
    IntegerOutOfRange {
        value: Integer,
        expected: Bounded<i128>,
    },
    #[snafu(display(
        "Provided length in not correct format. Should be bits as multiple of 8. {remainder}; actual: {value}"
    ))]
    LengthNotAsBitLength { value: usize, remainder: usize },
    #[snafu(display("Provided data is too long to be encoded with COER."))]
    TooLongValue { length: u128 },
    #[snafu(display("Provided data size is not in the constraint range."))]
    NotInSizeConstraintRange { length: usize },
    #[snafu(display("Integer does not fit to the reserved octets {expected}; actual: {value}"))]
    MoreBytesThanExpected { value: usize, expected: usize },
    #[snafu(display("Propagated Error:\n{}", msg))]
    Propagated {
        /// The custom error's message.
        msg: String,
    },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: String },
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: msg.to_string(),
        }
    }
}
