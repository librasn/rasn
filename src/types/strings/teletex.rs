use super::{
    AsnType, Constraints, Decode, Decoder, Encode, Encoder, Identifier, PermittedAlphabetError,
    StaticPermittedAlphabet, Tag, constrained,
};

use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string of the characters T.61 defines: a sequence of octets in the
/// code of ISO/IEC 2022, in which escape sequences select character sets
/// and diacritics precede the letters they modify (X.690 §8.23.5).
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(pub(super) Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl TeletexString {
    /// The octets of the string.
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
impl StaticPermittedAlphabet for TeletexString {
    type T = u8;
    /// Every octet value: which characters the octets denote depends on the
    /// character sets the escape sequences among them select.
    const CHARACTER_SET: &'static [u32] = &{
        let mut octets = [0u32; 256];
        let mut i = 0;
        while i < 256 {
            octets[i as usize] = i;
            i += 1;
        }
        octets
    };
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
        constraints: &Constraints,
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
        constraints: &Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_teletex_string(tag, constraints)
    }
}
