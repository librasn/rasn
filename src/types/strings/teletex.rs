use super::{
    AsnType, Constraints, Decode, Decoder, Encode, Encoder, Identifier, PermittedAlphabetError,
    StaticPermittedAlphabet, Tag, constrained,
};

use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string containing the encoded octets defined by T.61.
///
/// Octets, including escape sequences and non-spacing accents, are preserved.
/// This type does not translate T.61 into Unicode or validate escape sequences.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(pub(super) Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl TeletexString {
    /// Returns the encoded T.61 octets.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    /// Attempts to convert the provided bytes into [Self].
    ///
    /// # Errors
    /// If any of the provided bytes does not match the allowed character set.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }

    /// Convert the teletex string into a `Vec<u8>`, consuming the original string.
    #[must_use]
    pub fn into_vec(self) -> alloc::vec::Vec<u8> {
        self.0
    }
}
const fn octet_alphabet() -> [u32; 256] {
    let mut octets = [0; 256];
    let mut index = 0;
    while index < octets.len() {
        octets[index] = index as u32;
        index += 1;
    }
    octets
}

impl StaticPermittedAlphabet for TeletexString {
    type T = u8;
    const CHARACTER_SET: &'static [u32] = &octet_alphabet();
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Teletex;

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch as u8);
    }

    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    fn chars(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().map(|&octet| u32::from(octet))
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
