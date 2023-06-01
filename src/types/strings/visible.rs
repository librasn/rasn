use bitvec::prelude::*;

use crate::{prelude::*, types};

use super::ConstrainedCharacterString;

const BIT_WIDTH: usize = 7;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleString(ConstrainedCharacterString<BIT_WIDTH>);

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Invalid ISO 646 bytes"))]
pub struct InvalidIso646Bytes;

impl VisibleString {
    pub fn from_iso646_bytes(bytes: &[u8]) -> Result<Self, InvalidIso646Bytes> {
        let mut buffer = types::BitString::new();

        for byte in bytes {
            if byte & 0x80 != 0 {
                return Err(InvalidIso646Bytes);
            }

            let bv = &byte.view_bits::<Msb0>()[1..8];
            debug_assert_eq!(*byte, bv.load_be::<u8>());
            buffer.extend(bv);
        }

        debug_assert!(buffer.is_empty() || buffer.len() >= BIT_WIDTH);
        debug_assert!(buffer.len() % BIT_WIDTH == 0);

        Ok(Self(ConstrainedCharacterString::from_raw_bits(buffer)))
    }

    pub fn from_raw_bits(bits: types::BitString) -> Self {
        Self(ConstrainedCharacterString::from_raw_bits(bits))
    }

    pub fn to_iso646_bytes(&self) -> Vec<u8> {
        self.iter().map(|bv| bv.load_be::<u8>()).collect()
    }

    pub fn chars(&self) -> impl Iterator<Item = u8> + '_ {
        self.to_iso646_bytes().into_iter()
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.iter().nth(index).map(|bv| bv.load_be::<u8>())
    }
}

impl From<types::BitString> for VisibleString {
    fn from(value: types::BitString) -> Self {
        Self::from_raw_bits(value)
    }
}

impl super::StaticPermittedAlphabet for VisibleString {
    const CHARACTER_SET: &'static [u32] = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
        0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    ];

    fn from_raw_bits(value: BitString) -> Self {
        Self(ConstrainedCharacterString::from_raw_bits(value))
    }
}

impl core::ops::Deref for VisibleString {
    type Target = ConstrainedCharacterString<BIT_WIDTH>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Display for VisibleString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8(self.to_iso646_bytes()).unwrap())
    }
}

impl AsnType for VisibleString {
    const TAG: Tag = Tag::VISIBLE_STRING;
}

impl Encode for VisibleString {
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints<'constraints>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_visible_string(tag, constraints, &self)
            .map(drop)
    }
}

impl Decode for VisibleString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints<'constraints>,
    ) -> Result<Self, D::Error> {
        decoder.decode_visible_string(tag, constraints)
    }
}

impl TryFrom<alloc::string::String> for VisibleString {
    type Error = InvalidIso646Bytes;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        Self::from_iso646_bytes(value.as_bytes())
    }
}

impl TryFrom<&'_ str> for VisibleString {
    type Error = InvalidIso646Bytes;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_iso646_bytes(value.as_bytes())
    }
}

impl TryFrom<alloc::vec::Vec<u8>> for VisibleString {
    type Error = InvalidIso646Bytes;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_iso646_bytes(&value)
    }
}

impl TryFrom<&'_ [u8]> for VisibleString {
    type Error = InvalidIso646Bytes;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_iso646_bytes(value)
    }
}

impl TryFrom<bytes::Bytes> for VisibleString {
    type Error = InvalidIso646Bytes;

    fn try_from(value: bytes::Bytes) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
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
        let john = VisibleString::try_from("John").unwrap();
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
