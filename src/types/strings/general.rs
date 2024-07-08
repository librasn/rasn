use super::*;

use crate::error::strings::{InvalidGeneralString, PermittedAlphabetError};
use alloc::{string::String, vec::Vec};
use once_cell::race::OnceBox;

/// A "general" string containing the `C0` Controls plane, `SPACE`,
/// Basic Latin, `DELETE`, and Latin-1 Supplement characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralString(Vec<u8>);

static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl GeneralString {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PermittedAlphabetError> {
        Ok(Self(Self::try_from_slice(bytes)?))
    }
}

impl StaticPermittedAlphabet for GeneralString {
    type T = u8;
    const CHARACTER_SET: &'static [u32] = &[
        // 0x00..=0x1F, // C0 Controls (C set)
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, //  0x20, SPACE
        0x20, // 0x21..=0x7E, Basic Latin (G set)
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E,
        0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D,
        0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C,
        0x5D, 0x5E, 0x5F, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B,
        0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A,
        0x7B, 0x7C, 0x7D, 0x7E, //  0x7F, DELETE
        0x7F, //  0xA1..=0xFF, Latin-1 Supplement (G set)
        0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
        0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE,
        0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD,
        0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDC,
        0xDD, 0xDE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB,
        0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA,
        0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
    ];
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::General;
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

impl TryFrom<Vec<u8>> for GeneralString {
    type Error = PermittedAlphabetError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(value.as_slice())?))
    }
}

impl TryFrom<&'_ str> for GeneralString {
    type Error = PermittedAlphabetError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(value)?))
    }
}

impl TryFrom<&'_ [u8]> for GeneralString {
    type Error = PermittedAlphabetError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(value)?))
    }
}

impl TryFrom<String> for GeneralString {
    type Error = PermittedAlphabetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Self::try_from_slice(value.as_bytes())?))
    }
}

impl core::ops::Deref for GeneralString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for GeneralString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsnType for GeneralString {
    const TAG: Tag = Tag::GENERAL_STRING;
}

impl Decode for GeneralString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_general_string(tag, constraints)
    }
}

impl Encode for GeneralString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_general_string(tag, constraints, self)
            .map(drop)
    }
}
