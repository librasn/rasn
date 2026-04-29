//! Module for different bit modification functions which are used in the library.

pub(crate) fn range_from_len(bit_length: u32) -> i128 {
    2i128.pow(bit_length) - 1
}
