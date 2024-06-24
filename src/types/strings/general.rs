use super::*;

use crate::error::strings::InvalidGeneralString;
use alloc::{borrow::ToOwned, string::String, vec::Vec};

/// A "general" string containing the `C0` Controls plane, `SPACE`,
/// Basic Latin, `DELETE`, and Latin-1 Supplement characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralString(Vec<u8>);

impl GeneralString {
    fn is_valid(bytes: &[u8]) -> Result<(), InvalidGeneralString> {
        for byte in bytes {
            let is_in_set = matches!(
                byte,
                | 0x00..=0x1F // C0 Controls (C set)
                | 0x20        // SPACE
                | 0x21..=0x7E // Basic Latin (G set)
                | 0x7F        // DELETE
                | 0xA1..=0xFF // Latin-1 Supplement (G set)
            );

            if !is_in_set {
                return Err(InvalidGeneralString { character: *byte });
            }
        }
        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidGeneralString> {
        Self::is_valid(bytes)?;
        Ok(Self(bytes.to_owned()))
    }
}

// impl StaticPermittedAlphabet for GeneralString {
//     const CHARACTER_SET: &'static [u32] = &[
//         0x00..=0x1F, // C0 Controls (C set)
//         0x20,        // SPACE
//         0x21..=0x7E, // Basic Latin (G set)
//         0x7F,        // DELETE
//         0xA1..=0xFF, // Latin-1 Supplement (G set)
//     ];
//     const CHARACTER_SET_NAME: constrained::CharacterSetName =
//         constrained::CharacterSetName::General;
// }

impl TryFrom<Vec<u8>> for GeneralString {
    type Error = InvalidGeneralString;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::is_valid(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<String> for GeneralString {
    type Error = InvalidGeneralString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::is_valid(value.as_bytes())?;
        Ok(Self(value.into_bytes()))
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
