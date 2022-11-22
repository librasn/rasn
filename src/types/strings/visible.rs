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
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints<'constraints>) -> Result<(), E::Error> {
        encoder.encode_visible_string(tag, constraints, &self).map(drop)
    }
}

impl Decode for VisibleString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
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
