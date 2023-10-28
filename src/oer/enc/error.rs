use crate::types::constraints::Bounded;
use crate::types::Integer;
use alloc::string::{String, ToString};
use snafu::*;
// use x509_parser::der_parser::asn1_rs::Integer;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Provided data size is not in the constraint range."))]
    NotInSizeConstraintRange { length: usize },
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: msg.to_string(),
        }
    }
}
