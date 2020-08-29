use snafu::*;

#[derive(Snafu)]
#[snafu(visibility = "pub(crate)")]
#[derive(Debug)]
pub enum Error {
    #[snafu(display("Size of data is too large to encode with definite length."))]
    DataTooLarge,
    #[snafu(display("Custom Error:\n{}", msg))]
    Custom {
        msg: alloc::string::String
    },
}

impl crate::error::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}

