use super::*;
use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A Basic Multilingual Plane (BMP) string, which is a subtype of [`UniversalString`]
/// containing only the BMP set of characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BmpString(pub(super) Vec<u16>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl BmpString {
    /// Converts the string into a set of big endian bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|ch| ch.to_be_bytes()).collect()
    }
}

impl StaticPermittedAlphabet for BmpString {
    type T = u16;
    const CHARACTER_SET: &'static [u32] = &{
        let mut array = [0u32; 0xFFFE];
        let mut i = 0;
        while i < 0xFFFE {
            array[i as usize] = i;
            i += 1;
        }
        array
    };
    const CHARACTER_SET_NAME: constrained::CharacterSetName = constrained::CharacterSetName::Bmp;

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch as u16);
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

impl AsnType for BmpString {
    const TAG: Tag = Tag::BMP_STRING;
}

impl Encode for BmpString {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_bmp_string(tag, constraints, self, identifier).map(drop)
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
