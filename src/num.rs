//! Module for different number-related functions which are used in the library.
use crate::types::PrimitiveInteger;
pub(crate) const fn log2(x: PrimitiveInteger) -> u32 {
    PrimitiveInteger::BITS - (x - 1).leading_zeros()
}
