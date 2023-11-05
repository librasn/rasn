//! Module for different number-related functions which are used in the library.
pub(crate) const fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}
