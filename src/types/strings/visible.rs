use super::*;

use crate::error::strings::PermittedAlphabetError;
use alloc::{borrow::ToOwned, vec::Vec};
use once_cell::race::OnceBox;

/// A string which contains a subset of the ISO 646 character set.
/// Type **should be** constructed by using `try_from` or `from` methods.
///
/// This type is restricted to allow only the graphically visible characters and space character (0x20),
/// which is defined in X.680 (02/2021), Section 41, Table 8.
///
/// Should contain registration 6. set from ISO International Register of Coded Character Sets and SPACE character.
/// Graphical restrictions (registration 6.) are defined freely and publicly in sister standard ITU-T T.50, section 6.4.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct VisibleString(pub(super) Vec<u8>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl VisibleString {
    /// Create a new `VisibleString` from ISO 646 bytes (also known as US-ASCII/IA5/IRA5).
    ///
    /// # Errors
    ///
    /// Error of type `InvalidIso646Bytes` is raised if the restriction is not met.
    pub fn from_iso646_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }
    /// Converts the `VisibleString` into ISO 646 bytes (also known as US-ASCII/IA5/IRA5).
    #[must_use]
    pub fn as_iso646_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl StaticPermittedAlphabet for VisibleString {
    type T = u8;
    /// Includes Space (0x20) and all graphically visible characters (0x21-0x7E).
    const CHARACTER_SET: &'static [u32] = &[
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E,
        0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D,
        0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C,
        0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B,
        0x5C, 0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A,
        0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79,
        0x7A, 0x7B, 0x7C, 0x7D, 0x7E,
    ];
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Visible;

    fn chars(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().map(|&byte| byte as u32)
    }

    #[track_caller]
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

impl core::fmt::Display for VisibleString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(self.as_iso646_bytes()).unwrap())
    }
}

impl AsnType for VisibleString {
    const TAG: Tag = Tag::VISIBLE_STRING;
}

impl Encode for VisibleString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_visible_string(tag, constraints, self)
            .map(drop)
    }
}

impl Decode for VisibleString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_visible_string(tag, constraints)
    }
}

impl From<VisibleString> for bytes::Bytes {
    fn from(value: VisibleString) -> Self {
        value.0.into()
    }
}

impl From<VisibleString> for alloc::string::String {
    fn from(value: VisibleString) -> Self {
        Self::from_utf8(value.as_iso646_bytes().to_owned()).unwrap()
    }
}
