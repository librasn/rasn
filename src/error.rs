pub trait Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self;
}
