use super::*;

use crate::error::strings::PermittedAlphabetError;
use alloc::{string::String, vec::Vec};
use once_cell::race::OnceBox;

/// A string which can only contain numbers or `SPACE` characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericString(Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl NumericString {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl StaticPermittedAlphabet for NumericString {
    type T = u8;
    const CHARACTER_SET: &'static [u32] = &bytes_to_chars([
        b' ', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    ]);
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Numeric;

    fn chars(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().map(|&byte| byte as u32)
    }

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch as u8);
    }

    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        INDEX_MAP.get_or_init(Self::build_index_map)
    }

    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        CHARACTER_MAP.get_or_init(Self::build_character_map)
    }
}

impl TryFrom<String> for NumericString {
    type Error = PermittedAlphabetError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(string.as_bytes())?))
    }
}

impl TryFrom<&'_ str> for NumericString {
    type Error = PermittedAlphabetError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(string.as_bytes())?))
    }
}

impl TryFrom<&'_ [u8]> for NumericString {
    type Error = PermittedAlphabetError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(value)?))
    }
}

impl TryFrom<Vec<u8>> for NumericString {
    type Error = PermittedAlphabetError;

    fn try_from(string: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(string.as_slice())?))
    }
}

impl TryFrom<BitString> for NumericString {
    type Error = PermittedAlphabetError;

    fn try_from(string: BitString) -> Result<Self, Self::Error> {
        Self::try_from_permitted_alphabet(string, None)
    }
}

impl AsnType for NumericString {
    const TAG: Tag = Tag::NUMERIC_STRING;
}

impl Encode for NumericString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_numeric_string(tag, constraints, self)
            .map(drop)
    }
}

impl Decode for NumericString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_numeric_string(tag, constraints)
    }
}
