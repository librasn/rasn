use super::*;

use crate::error::strings::InvalidPrintableString;
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use once_cell::race::OnceBox;

/// A string, which contains the characters defined in X.680 41.4 Section, Table 10.
///
/// You must use `try_from` or `from_*` to construct a `PrintableString`.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct PrintableString(Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl StaticPermittedAlphabet for PrintableString {
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
        debug_assert!(
            Self::CHARACTER_SET.contains(&ch),
            "{ch} not in character set"
        );
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

impl PrintableString {
    /// Construct a new `PrintableString` from a byte array.
    ///
    /// # Errors
    /// Raises `InvalidPrintableString` if the byte array contains invalid characters,
    /// other than in `CHARACTER_SET`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidPrintableString> {
        bytes.iter().copied().map(u32::from).try_for_each(|byte| {
            if Self::CHARACTER_SET.contains(&byte) {
                Ok(())
            } else {
                Err(InvalidPrintableString { character: byte })
            }
        })?;

        Ok(Self(bytes.to_owned()))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<String> for PrintableString {
    type Error = InvalidPrintableString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_bytes(value.as_bytes())
    }
}

impl TryFrom<alloc::vec::Vec<u8>> for PrintableString {
    type Error = InvalidPrintableString;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl TryFrom<&'_ [u8]> for PrintableString {
    type Error = InvalidPrintableString;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}
impl TryFrom<&'_ str> for PrintableString {
    type Error = InvalidPrintableString;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_bytes(value.as_bytes())
    }
}

impl AsnType for PrintableString {
    const TAG: Tag = Tag::PRINTABLE_STRING;
}

impl Encode for PrintableString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_printable_string(tag, constraints, self)
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
