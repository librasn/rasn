use snafu::*;

#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    #[snafu(display("OBJECT IDENTIFIER must have at least two components."))]
    InvalidObjectIdentifier,
    #[snafu(display("Custom Error:\n{}", msg))]
    Custom { msg: alloc::string::String },
}

impl crate::error::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}
