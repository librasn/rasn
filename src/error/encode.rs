use crate::types::constraints::{Bounded, Size};
use snafu::*;

use alloc::string::ToString;

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
#[snafu(display("Error Kind: {}\nBacktrace:\n{}", kind, backtrace))]
pub struct EncodeError {
    kind: Kind,
    backtrace: Backtrace,
}
impl EncodeError {
    pub fn check_length(length: usize, expected: &Size) -> Result<(), Self> {
        expected.contains_or_else(&length, || Self {
            kind: Kind::InvalidLength {
                length,
                expected: (**expected),
            },
            backtrace: Backtrace::generate(),
        })
    }
    #[must_use]
    pub fn invalid_object_identifier() -> Self {
        Self::from(Kind::InvalidObjectIdentifier)
    }
}
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Kind {
    /// `OBJECT IDENTIFIER` must have at least two components.
    #[snafu(display(
        "Invalid Object Identifier: must have at least two components and first octet must be 0, 1 or 2"
    ))]
    InvalidObjectIdentifier,
    #[snafu(display("invalid length, expected: {expected}; actual: {length}"))]
    InvalidLength {
        length: usize,
        expected: Bounded<usize>,
    },
    // #[snafu(display("wrapped der encoding error: {source}"))]
    // Der {
    //     source: alloc::boxed::Box<crate::der::enc::Error>,
    // },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: alloc::string::String },
}
impl From<Kind> for EncodeError {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            backtrace: Backtrace::generate(),
        }
    }
}

impl crate::enc::Error for EncodeError {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self {
            kind: Kind::Custom {
                msg: msg.to_string(),
            },
            backtrace: Backtrace::generate(),
        }
    }
}
