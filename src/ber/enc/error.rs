use snafu::*;

/// An error that ocurred when encoding BER or any of its variants.
#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    /// `OBJECT IDENTIFIER` must have at least two components.
    InvalidObjectIdentifier,
    /// A custom error.
    #[snafu(display("Custom Error:\n{}", msg))]
    Custom {
        /// The custom error's message.
        msg: alloc::string::String,
    },
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}
