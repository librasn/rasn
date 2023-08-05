use crate::types::constraints::{Bounded, Size, Value};
use crate::types::Integer;
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
    #[snafu(display("Provided data is too long to be encoded with COER."))]
    TooLongValue { length: u128 },
    #[snafu(display("Integer does not fit to the reserved octets {expected}; actual: {value}"))]
    MoreOctetsThanExpected { value: usize, expected: usize },
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
