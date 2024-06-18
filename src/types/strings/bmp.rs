use super::*;

use crate::error::strings::InvalidBmpString;
use alloc::{boxed::Box, string::String, vec::Vec};
use once_cell::race::OnceBox;

/// A Basic Multilingual Plane (BMP) string, which is a subtype of [`UniversalString`]
/// containing only the BMP set of characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BmpString(Vec<u16>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl BmpString {
    /// Converts the string into a set of big endian bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|ch| ch.to_be_bytes()).collect()
    }
}

impl StaticPermittedAlphabet for BmpString {
    const CHARACTER_SET: &'static [u32] = &{
        let mut array = [0u32; 0xFFFE];
        let mut i = 0;
        while i < 0xFFFE {
            array[i as usize] = i;
            i += 1;
        }
        array
    };
    fn alphabet_name() -> &'static str {
        "BmpString"
    }

    fn push_char(&mut self, ch: u32) {
        debug_assert!(
            Self::CHARACTER_SET.contains(&ch),
            "{} not in character set",
            ch
        );
        self.0.push(ch as u16);
    }

    fn chars(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::from(self.0.iter().map(|ch| *ch as u32))
    }

    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        INDEX_MAP.get_or_init(Self::build_index_map)
    }

    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        CHARACTER_MAP.get_or_init(Self::build_character_map)
    }
}

impl TryFrom<String> for BmpString {
    type Error = InvalidBmpString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}

impl TryFrom<Vec<u8>> for BmpString {
    type Error = InvalidBmpString;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut vec = Vec::with_capacity(value.len());
        for ch in value {
            if matches!(ch as u16, 0x0..=0xFFFF) {
                vec.push(ch as u16);
            } else {
                return Err(InvalidBmpString {
                    character: ch as u16,
                });
            }
        }
        Ok(Self(vec))
    }
}

impl TryFrom<&'_ str> for BmpString {
    type Error = InvalidBmpString;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes().to_vec())
    }
}

impl AsnType for BmpString {
    const TAG: Tag = Tag::BMP_STRING;
}

impl Encode for BmpString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bmp_string(tag, constraints, self).map(drop)
    }
}

impl Decode for BmpString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_bmp_string(tag, constraints)
    }
}
