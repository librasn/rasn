use super::*;

use crate::error::strings::PermittedAlphabetError;
use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string which can only contain numbers or `SPACE` characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericString(pub(super) Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl NumericString {
    /// Attempts to convert the provided bytes into [Self].
    ///
    /// # Errors
    /// If any of the provided bytes does not match the allowed character set.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }

    /// Provides a slice of bytes representing the current value.
    #[must_use]
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

impl AsnType for NumericString {
    const TAG: Tag = Tag::NUMERIC_STRING;
}

impl Encode for NumericString {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
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
