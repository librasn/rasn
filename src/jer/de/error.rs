use snafu::Snafu;

/// An error that occurred when decoding BER or any of its variants.
#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[derive(Debug)]
pub enum Error {
    /// Upstream `serde` error
    SerdeDecodingError {
        upstream: alloc::string::String,
    },
    /// A custom error.
    #[snafu(display("{}", msg))]
    Custom {
        /// the error's message.
        msg: alloc::string::String,
    },
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeDecodingError { upstream: alloc::format!("Encountered an error during JER-encoding with serde: {value:#?}") }
    }
}
