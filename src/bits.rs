//! Module for different bit modification functions which are used in the library.

use alloc::vec::Vec;
use core::cmp::Ordering;

pub(crate) fn range_from_len(bit_length: u32) -> i128 {
    2i128.pow(bit_length) - 1
}

/// The canonical encoding of SET OF values in DER requires
/// the encoded elements to be sorted in ascending order.
/// When DER-encoding a SET OF, its elements are encoded one by one.
/// The encoded elements are then compared as octet strings
/// (shorter strings are zero-padded at their backs).
/// The function is to be used as a compare function for `alloc::slice::sort_by`.
///
/// ***From ISO/IEC 8825-1:2021***
///
/// *11.6 Set of components*
///
/// *The encodings of the component values of a set-of value shall appear in ascending order,*
/// *the encodings being compared as octet strings with the shorter components being padded*
/// *at their trailing end with 0-octets.*
#[allow(clippy::ptr_arg)] // used in comparison function
pub(crate) fn octet_string_ascending(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
    let min_length = b.len().min(a.len());
    for i in 0..min_length {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            o => return o,
        }
    }
    a.len().cmp(&b.len())
}

pub fn integer_to_bytes(value: &crate::prelude::Integer, signed: bool) -> Option<Vec<u8>> {
    if signed {
        Some(value.to_signed_bytes_be())
    } else if !signed && (value.is_positive() || value.is_zero()) {
        Some(value.to_biguint()?.to_bytes_be())
    } else {
        None
    }
}
