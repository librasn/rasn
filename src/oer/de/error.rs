use crate::types::constraints::{Bounded, Size, Value};
use crate::types::Integer;
use core::fmt::Display;
use nom::Needed;
use num_bigint::BigUint;
use snafu::*;
// use x509_parser::der_parser::asn1_rs::Integer;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Propagated Error:\n{}", msg))]
    Propagated {
        /// The custom error's message.
        msg: String,
    },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: String },
}

impl crate::de::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: msg.to_string(),
        }
    }

    fn incomplete(needed: Needed) -> Self {
        todo!()
    }

    fn exceeds_max_length(length: BigUint) -> Self {
        todo!()
    }

    fn missing_field(name: &'static str) -> Self {
        todo!()
    }

    fn no_valid_choice(name: &'static str) -> Self {
        todo!()
    }

    fn field_error<D: Display>(name: &'static str, error: D) -> Self {
        todo!()
    }

    fn duplicate_field(name: &'static str) -> Self {
        todo!()
    }
}
