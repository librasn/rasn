use bitvec::prelude::*;

use crate::{prelude::*, types};

const BIT_WIDTH: usize = 7;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleString(types::BitString);

impl VisibleString {
    pub fn from_iso646_bytes(bytes: &[u8]) -> Self {
        let bits = bytes
            .iter()
            .copied()
            .map(|byte| {
                debug_assert!(byte & 0x80 == 0);
                let bv = &byte.view_bits::<Msb0>()[1..8];
                debug_assert_eq!(byte, bv.load_be::<u8>());
                bv.to_bitvec()
            })
            .fold(types::BitString::default(), |mut acc, bv| {
                acc.extend(bv);
                acc
            });

        debug_assert!(bits.is_empty() || bits.len() >= BIT_WIDTH);
        debug_assert!(bits.len() % BIT_WIDTH == 0);

        Self(bits)
    }

    pub fn from_raw_bits(bits: types::BitString) -> Self {
        debug_assert!(bits.is_empty() || bits.len() >= BIT_WIDTH);
        debug_assert!(bits.len() % BIT_WIDTH == 0);

        Self(bits)
    }

    pub fn to_iso646_bytes(&self) -> Vec<u8> {
        self.iter().map(|bv| bv.load_be::<u8>()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.0.len() / BIT_WIDTH
    }

    pub fn as_bitstr(&self) -> &types::BitStr {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &types::BitStr> + '_ {
        self.0.chunks_exact(BIT_WIDTH)
    }

    pub fn chars(&self) -> impl Iterator<Item = u8> + '_ {
        self.to_iso646_bytes().into_iter()
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.iter().nth(index).map(|bv| bv.load_be::<u8>())
    }
}

impl core::fmt::Display for VisibleString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8(self.to_iso646_bytes()).unwrap())
    }
}

impl core::ops::Index<usize> for VisibleString {
    type Output = types::BitStr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index..index * BIT_WIDTH]
    }
}

impl core::ops::Index<core::ops::Range<usize>> for VisibleString {
    type Output = types::BitStr;

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.0[index.start * BIT_WIDTH..index.end * BIT_WIDTH]
    }
}

impl AsnType for VisibleString {
    const TAG: Tag = Tag::VISIBLE_STRING;
}

impl Encode for VisibleString {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder
            .encode_visible_string(tag, <_>::default(), &self)
            .map(drop)
    }
}

impl Decode for VisibleString {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_visible_string(tag, <_>::default())
    }
}

impl From<alloc::string::String> for VisibleString {
    fn from(value: alloc::string::String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<&'_ str> for VisibleString {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<alloc::vec::Vec<u8>> for VisibleString {
    fn from(value: alloc::vec::Vec<u8>) -> Self {
        Self::from(&*value)
    }
}

impl From<&'_ [u8]> for VisibleString {
    fn from(value: &[u8]) -> Self {
        Self::from_iso646_bytes(value)
    }
}

impl From<bytes::Bytes> for VisibleString {
    fn from(value: bytes::Bytes) -> Self {
        Self::from(&*value)
    }
}

impl From<VisibleString> for bytes::Bytes {
    fn from(value: VisibleString) -> Self {
        value.to_iso646_bytes().into()
    }
}

impl From<VisibleString> for alloc::string::String {
    fn from(value: VisibleString) -> Self {
        Self::from_utf8(value.to_iso646_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_compatibility() {
        let john = VisibleString::from("John");
        let mut chars = john.chars();
        assert_eq!(b'J', chars.next().unwrap());
        assert_eq!(b'o', chars.next().unwrap());
        assert_eq!(b'h', chars.next().unwrap());
        assert_eq!(b'n', chars.next().unwrap());
        assert!(chars.next().is_none());
        assert_eq!(
            bitvec::bits![
                1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0
            ],
            john.as_bitstr()
        );
        assert_eq!(b"John", &*john.to_iso646_bytes());
    }
}
