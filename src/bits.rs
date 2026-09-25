//! Module for different bit modification functions which are used in the library.

use alloc::{borrow::Cow, vec::Vec};

use bitvec::domain::Domain;

use crate::types::{BitStr, BitString};

pub(crate) fn range_from_len(bit_length: u32) -> i128 {
    2i128.pow(bit_length) - 1
}

/// Appends `src` to `dst` by shifting whole words, whatever the alignment of
/// either side.
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

/// The octets of `bits` when it starts and ends on an octet boundary, so
/// they can be borrowed rather than copied.
pub(crate) fn aligned_octets(bits: &BitStr) -> Option<&[u8]> {
    match bits.domain() {
        Domain::Region {
            head: None,
            body,
            tail: None,
        } => Some(body),
        _ => None,
    }
}

/// The octets of `bits`, whose length must be a multiple of eight: borrowed
/// when octet-aligned, otherwise shifted into a single allocation.
pub(crate) fn octets(bits: &BitStr) -> Cow<'_, [u8]> {
    debug_assert!(bits.len().is_multiple_of(8));
    match aligned_octets(bits) {
        Some(octets) => Cow::Borrowed(octets),
        None => {
            let mut owned = Vec::with_capacity(bits.len() / 8);
            extend_vec_from_bitslice(&mut owned, bits);
            Cow::Owned(owned)
        }
    }
}

/// Appends `src`, whose length must be a multiple of eight bits, to `dst` as
/// octets. Octet-aligned input is copied directly.
pub(crate) fn extend_vec_from_bitslice(dst: &mut Vec<u8>, src: &BitStr) {
    debug_assert!(src.len().is_multiple_of(8));
    if let Some(octets) = aligned_octets(src) {
        dst.extend_from_slice(octets);
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

/// A growable bit sequence backed by octets, most significant bit first.
///
/// The octets always cover exactly `len` bits, and the bits past `len` in
/// the last octet are zero, so appending only ever ORs new bits in and
/// growing is a single `Vec::resize`.
#[derive(Debug, Default, Clone)]
pub(crate) struct BitBuffer {
    bytes: Vec<u8>,
    len: usize,
}

impl BitBuffer {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: Vec::new(),
            len: 0,
        }
    }

    /// Reuses the allocation of `bytes`, starting empty.
    pub(crate) fn from_vec(mut bytes: Vec<u8>) -> Self {
        bytes.clear();
        Self { bytes, len: 0 }
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn clear(&mut self) {
        self.bytes.clear();
        self.len = 0;
    }

    /// Reserves room for at least `additional` more octets.
    pub(crate) fn reserve_bytes(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }

    /// The octets holding the bits; the last one is zero past `len`.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the buffer, padding it to a whole number of octets.
    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.bytes
    }

    /// Appends `bits` zero bits and returns the position of the first.
    fn grow(&mut self, bits: usize) -> usize {
        let position = self.len;
        self.len += bits;
        let needed = self.len.div_ceil(8);
        if needed > self.bytes.len() {
            self.bytes.resize(needed, 0);
        }
        position
    }

    /// Grows to `new_len` bits, which must not be less than `len`, filling
    /// the new bits with `bit`.
    pub(crate) fn resize(&mut self, new_len: usize, bit: bool) {
        debug_assert!(new_len >= self.len);
        let position = self.grow(new_len - self.len);
        if bit {
            for index in position..new_len {
                self.set(index, true);
            }
        }
    }

    pub(crate) fn push(&mut self, bit: bool) {
        let position = self.grow(1);
        if bit {
            self.set(position, true);
        }
    }

    pub(crate) fn set(&mut self, index: usize, bit: bool) {
        debug_assert!(index < self.len);
        let mask = 0x80u8 >> (index % 8);
        if bit {
            self.bytes[index / 8] |= mask;
        } else {
            self.bytes[index / 8] &= !mask;
        }
    }

    /// Appends the low `width` bits of `value`, most significant bit first.
    /// `width` is at most 128.
    pub(crate) fn push_bits(&mut self, value: u128, width: usize) {
        debug_assert!(width <= 128);
        if width == 0 {
            return;
        }
        let position = self.grow(width);
        let bytes = (value << (128 - width)).to_be_bytes();
        or_bits(&mut self.bytes, position, &bytes, width);
    }

    pub(crate) fn extend_from_bytes(&mut self, src: &[u8]) {
        let position = self.grow(src.len() * 8);
        or_bits(&mut self.bytes, position, src, src.len() * 8);
    }

    pub(crate) fn extend_from_bitslice(&mut self, src: &BitStr) {
        let position = self.grow(src.len());
        write_bitslice(&mut self.bytes, position, src);
    }

    pub(crate) fn extend_from_buffer(&mut self, src: &BitBuffer) {
        let position = self.grow(src.len);
        or_bits(&mut self.bytes, position, &src.bytes, src.len);
    }

    /// Reserves `additional` zero bits for the returned appender to fill.
    pub(crate) fn appender(&mut self, additional: usize) -> BitAppender<'_> {
        let position = self.grow(additional);
        BitAppender {
            bytes: &mut self.bytes,
            position,
        }
    }
}

/// Reads `src`, which holds at most 128 bits, as an unsigned integer with the
/// most significant bit first.
pub(crate) fn read_u128(src: &BitStr) -> u128 {
    debug_assert!(src.len() <= 128);
    match src.domain() {
        Domain::Enclave(element) => {
            let tail = usize::from(element.tail().into_inner());
            u128::from(element.load_value() >> (8 - tail))
        }
        Domain::Region { head, body, tail } => {
            let mut value = head.map_or(0, |element| u128::from(element.load_value()));
            for &byte in body {
                value = (value << 8) | u128::from(byte);
            }
            if let Some(element) = tail {
                let tail = usize::from(element.tail().into_inner());
                value = (value << tail) | u128::from(element.load_value() >> (8 - tail));
            }
            value
        }
    }
}

/// Appends fixed-width codes to a bit string without an intermediate buffer.
pub(crate) struct BitAppender<'a> {
    bytes: &'a mut [u8],
    position: usize,
}

impl BitAppender<'_> {
    /// Writes the low `width` bits of `value`, most significant bit first.
    /// `width` is at most 32.
    pub(crate) fn push(&mut self, value: u32, width: usize) {
        debug_assert!(width <= 32);
        if width == 0 {
            return;
        }
        let bytes = (u64::from(value) << (64 - width)).to_be_bytes();
        or_bits(self.bytes, self.position, &bytes, width);
        self.position += width;
    }
}

/// Reads `width` bits (at most 32) from `bytes` starting at bit `position`,
/// most significant bit first.
pub(crate) fn read_bits(bytes: &[u8], position: usize, width: usize) -> u32 {
    debug_assert!(width <= 32);
    if width == 0 {
        return 0;
    }
    let first = position / 8;
    let shift = position % 8;
    let needed = (shift + width).div_ceil(8);
    let mut accumulator = 0u64;
    for &byte in &bytes[first..first + needed] {
        accumulator = (accumulator << 8) | u64::from(byte);
    }
    let excess = needed * 8 - shift - width;
    ((accumulator >> excess) & ((1u64 << width) - 1)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::*;

    fn filled(len: usize) -> BitBuffer {
        let mut buffer = BitBuffer::new();
        buffer.resize(len, true);
        buffer
    }

    fn as_bitstring(buffer: &BitBuffer) -> BitString {
        let mut bits = BitString::from_vec(buffer.as_bytes().to_vec());
        bits.truncate(buffer.len());
        bits
    }

    #[test]
    fn push_and_read_wide_values() {
        for width in 0..=128usize {
            let mask = if width == 128 {
                u128::MAX
            } else {
                (1u128 << width) - 1
            };
            let value = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128 & mask;
            for dst_len in [0usize, 1, 5, 7, 8, 13] {
                let mut actual = filled(dst_len);
                actual.push_bits(value, width);
                let actual = as_bitstring(&actual);
                let mut expected = BitString::repeat(true, dst_len);
                let (high, low) = ((value >> 64) as u64, value as u64);
                if width > 64 {
                    expected.extend_from_bitslice(&high.view_bits::<Msb0>()[128 - width..]);
                    expected.extend_from_raw_slice(&low.to_be_bytes());
                } else {
                    expected.extend_from_bitslice(&low.view_bits::<Msb0>()[64 - width..]);
                }
                assert_eq!(actual, expected, "width {width} at {dst_len}");
                assert_eq!(
                    read_u128(&actual[dst_len..]),
                    value,
                    "width {width} at {dst_len}"
                );
            }
        }
    }

    #[test]
    fn appender_and_reader_round_trip() {
        for width in 0..=32usize {
            let mask = ((1u64 << width) - 1) as u32;
            let values: Vec<u32> = (0..20u32)
                .map(|i| i.wrapping_mul(0x9E37_79B9) & mask)
                .collect();
            let mut actual = filled(3);
            let mut expected = BitString::repeat(true, 3);
            {
                let mut appender = actual.appender(values.len() * width);
                for &value in &values {
                    appender.push(value, width);
                }
            }
            for &value in &values {
                expected.extend_from_bitslice(&value.view_bits::<Msb0>()[32 - width..]);
            }
            assert_eq!(as_bitstring(&actual), expected, "width {width}");
            let bytes = actual.as_bytes();
            for (i, &value) in values.iter().enumerate() {
                assert_eq!(
                    read_bits(bytes, 3 + i * width, width),
                    value,
                    "width {width} at {i}"
                );
            }
        }
    }

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
            let mut actual = filled(dst_len);
            actual.extend_from_bytes(&bytes);
            assert_eq!(as_bitstring(&actual), expected, "dst {dst_len}");
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
