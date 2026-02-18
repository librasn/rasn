use super::{
    AsnType, Constraints, Decode, Decoder, Encode, Encoder, Identifier, StaticPermittedAlphabet,
    Tag, bytes_to_chars, constrained,
};

use crate::error::strings::PermittedAlphabetError;
use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string, which contains the characters defined in X.680 41.4 Section, Table 10.
///
/// You must use `try_from` or `from_*` to construct a `PrintableString`.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct PrintableString(pub(super) Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl PrintableString {
    /// Construct a new `PrintableString` from a byte array.
    ///
    /// # Errors
    /// Raises `PermittedAlphabetError` if the byte array contains invalid characters,
    /// other than in `CHARACTER_SET`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }

    /// Returns a slice of bytes representing the current string value.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl StaticPermittedAlphabet for PrintableString {
    type T = u8;
    /// `PrintableString` contains only "printable" characters.
    /// Latin letters, digits, (space) '()+,-./:=?
    const CHARACTER_SET: &'static [u32] = &bytes_to_chars([
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b' ', b'\'', b'(', b')', b'+', b',', b'-', b'.', b'/', b':', b'=', b'?',
    ]);
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Printable;

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch as u8);
    }

    fn chars(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().map(|&byte| byte as u32)
    }

    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        INDEX_MAP.get_or_init(Self::build_index_map)
    }

    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        CHARACTER_MAP.get_or_init(Self::build_character_map)
    }
}

impl AsnType for PrintableString {
    const TAG: Tag = Tag::PRINTABLE_STRING;
    const IDENTIFIER: Identifier = Identifier::PRINTABLE_STRING;
}

impl Encode for PrintableString {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder
            .encode_printable_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl Decode for PrintableString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_printable_string(tag, constraints)
    }
}
