use super::{
    AsnType, Constraints, Decode, Decoder, Encode, Encoder, Identifier, PermittedAlphabetError,
    StaticPermittedAlphabet, Tag, constrained,
};

use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string, which contains the characters defined in T.61 standard.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(pub(super) Vec<u32>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl TeletexString {
    /// Converts the string into a set of big endian bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|ch| ch.to_be_bytes()).collect()
    }

    /// Attempts to convert the provided bytes into [Self].
    ///
    /// # Errors
    /// If any of the provided bytes does not match the allowed character set.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }
}
impl StaticPermittedAlphabet for TeletexString {
    type T = u32;
    // TODO add correct character set, see https://github.com/mouse07410/asn1c/blob/84d3a59c1bb89c59be6ca0625bb14ebea9084ba5/skeletons/TeletexString.c
    const CHARACTER_SET: &'static [u32] = &[0];
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Teletex;
    // TODO remove once correct character set is added
    fn contains_char(_: u32) -> bool {
        true
    }

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch);
    }
    fn chars(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().copied()
    }

    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        INDEX_MAP.get_or_init(Self::build_index_map)
    }

    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        CHARACTER_MAP.get_or_init(Self::build_character_map)
    }
}

impl AsnType for TeletexString {
    const TAG: Tag = Tag::TELETEX_STRING;
    const IDENTIFIER: Identifier = Identifier::TELETEX_STRING;
}

impl Encode for TeletexString {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder
            .encode_teletex_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Decode for TeletexString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_teletex_string(tag, constraints)
    }
}
