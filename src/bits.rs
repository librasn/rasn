//! Module for different bit modification functions which are used in the library.

use alloc::vec::Vec;

use bitvec::domain::Domain;

use crate::types::{BitStr, BitString};

pub(crate) fn range_from_len(bit_length: u32) -> i128 {
    2i128.pow(bit_length) - 1
}

/// Appends `src` to `dst`.
///
/// `bitvec` copies between differently aligned bit-slices one bit at a time;
/// this shifts whole words instead, which is what the PER codec needs since
/// its buffers are almost never octet-aligned.
pub(crate) fn extend_bitstring(dst: &mut BitString, src: &BitStr) {
    if src.is_empty() {
        return;
    }
    // A bit-vector produced by slicing may start mid-element; the raw view
    // below assumes the first element holds the first bit.
    dst.force_align();
    let position = dst.len();
    dst.resize(position + src.len(), false);
    write_bitslice(dst.as_raw_mut_slice(), position, src);
}

/// Appends the octets of `src` to `dst`.
pub(crate) fn extend_bitstring_from_bytes(dst: &mut BitString, src: &[u8]) {
    if src.is_empty() {
        return;
    }
    dst.force_align();
    let position = dst.len();
    dst.resize(position + src.len() * 8, false);
    or_bits(dst.as_raw_mut_slice(), position, src, src.len() * 8);
}

/// Appends `src`, whose length must be a multiple of eight bits, to `dst` as
/// octets. Octet-aligned input is copied directly.
pub(crate) fn extend_vec_from_bitslice(dst: &mut Vec<u8>, src: &BitStr) {
    debug_assert!(src.len().is_multiple_of(8));
    if let Domain::Region {
        head: None,
        body,
        tail: None,
    } = src.domain()
    {
        dst.extend_from_slice(body);
        return;
    }
    let position = dst.len();
    dst.resize(position + src.len() / 8, 0);
    write_bitslice(&mut dst[position..], 0, src);
}

/// ORs the bits of `src` into `dst`, most significant bit first, starting at
/// bit `position`. The target bits must be zero and `dst` must be large
/// enough to hold them.
pub(crate) fn write_bitslice(dst: &mut [u8], mut position: usize, src: &BitStr) {
    match src.domain() {
        Domain::Enclave(element) => {
            let head = usize::from(element.head().into_inner());
            let tail = usize::from(element.tail().into_inner());
            or_bits(dst, position, &[element.load_value() << head], tail - head);
        }
        Domain::Region { head, body, tail } => {
            if let Some(element) = head {
                let head = usize::from(element.head().into_inner());
                or_bits(dst, position, &[element.load_value() << head], 8 - head);
                position += 8 - head;
            }
            or_bits(dst, position, body, body.len() * 8);
            position += body.len() * 8;
            if let Some(element) = tail {
                let tail = usize::from(element.tail().into_inner());
                or_bits(dst, position, &[element.load_value()], tail);
            }
        }
    }
}

/// ORs the first `count` bits of `src`, most significant bit first, into `dst`
/// starting at bit `position`.
fn or_bits(dst: &mut [u8], position: usize, src: &[u8], count: usize) {
    if count == 0 {
        return;
    }
    let full_bytes = count / 8;
    let tail_bits = count % 8;
    let shift = position % 8;
    let mut index = position / 8;
    let (full, rest) = src.split_at(full_bytes);

    if shift == 0 {
        for (target, &byte) in dst[index..index + full_bytes].iter_mut().zip(full) {
            *target |= byte;
        }
        index += full_bytes;
    } else {
        let (chunks, remainder) = full.as_chunks::<8>();
        for chunk in chunks {
            let word = u64::from_be_bytes(*chunk);
            // Place the 64 bits at offset `shift` within a 72-bit window, which
            // spans the nine output bytes they touch.
            let window = (u128::from(word) << (8 - shift)).to_be_bytes();
            for (target, &byte) in dst[index..index + 9].iter_mut().zip(&window[7..16]) {
                *target |= byte;
            }
            index += 8;
        }
        for &byte in remainder {
            dst[index] |= byte >> shift;
            dst[index + 1] |= byte << (8 - shift);
            index += 1;
        }
    }

    if tail_bits > 0 {
        let byte = rest[0] & (0xFFu8 << (8 - tail_bits));
        dst[index] |= byte >> shift;
        if shift + tail_bits > 8 {
            dst[index + 1] |= byte << (8 - shift);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::*;

    fn pattern(len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| (i as u8).wrapping_mul(37) ^ 0x5A)
            .collect()
    }

    #[test]
    fn extend_matches_bitvec() {
        let source = pattern(40);
        let source = source.view_bits::<Msb0>();
        for dst_len in [0usize, 1, 3, 7, 8, 9, 15, 16, 17, 63, 64, 65] {
            for src_start in 0..9 {
                for src_len in [0usize, 1, 2, 7, 8, 9, 13, 16, 63, 64, 65, 72, 100, 250] {
                    let src = &source[src_start..src_start + src_len];
                    let mut expected = BitString::repeat(true, dst_len);
                    expected.extend_from_bitslice(src);
                    let mut actual = BitString::repeat(true, dst_len);
                    extend_bitstring(&mut actual, src);
                    assert_eq!(actual, expected, "dst {dst_len} src {src_start}+{src_len}");
                }
            }
        }
    }

    #[test]
    fn extend_from_bytes_matches_bitvec() {
        let bytes = pattern(21);
        for dst_len in 0..17 {
            let mut expected = BitString::repeat(true, dst_len);
            expected.extend_from_raw_slice(&bytes);
            let mut actual = BitString::repeat(true, dst_len);
            extend_bitstring_from_bytes(&mut actual, &bytes);
            assert_eq!(actual, expected, "dst {dst_len}");
        }
    }

    #[test]
    fn extend_vec_matches_realigned_copy() {
        let source = pattern(40);
        let source = source.view_bits::<Msb0>();
        for src_start in 0..9 {
            for octets in [0usize, 1, 2, 8, 9, 17] {
                let src = &source[src_start..src_start + octets * 8];
                let mut expected = src.to_bitvec();
                expected.force_align();
                let mut actual = alloc::vec![0xFF, 0x00];
                extend_vec_from_bitslice(&mut actual, src);
                assert_eq!(&actual[..2], &[0xFF, 0x00]);
                assert_eq!(
                    &actual[2..],
                    expected.as_raw_slice(),
                    "src {src_start}+{octets}"
                );
            }
        }
    }

    #[test]
    fn write_matches_bitvec_copy() {
        let source = pattern(20);
        let source = source.view_bits::<Msb0>();
        for position in 0..17 {
            for src_start in 0..9 {
                for src_len in [1usize, 5, 8, 9, 64, 70] {
                    let src = &source[src_start..src_start + src_len];
                    let mut expected = [0u8; 16];
                    expected.view_bits_mut::<Msb0>()[position..position + src_len]
                        .copy_from_bitslice(src);
                    let mut actual = [0u8; 16];
                    write_bitslice(&mut actual, position, src);
                    assert_eq!(actual, expected, "at {position} src {src_start}+{src_len}");
                }
            }
        }
    }
}
