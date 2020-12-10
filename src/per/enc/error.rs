#[derive(Clone, Debug)]
pub enum Error {
    Custom{ msg: alloc::string::String },
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: alloc::string::ToString::to_string(&msg),
        }
    }
}
