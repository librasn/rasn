use super::*;

use alloc::vec::Vec;
use once_cell::race::OnceBox;

/// A string, which contains the characters defined in T.61 standard.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(pub(super) Vec<u32>);
static CHARACTER_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();
static INDEX_MAP: OnceBox<alloc::collections::BTreeMap<u32, u32>> = OnceBox::new();

impl TeletexString {
    /// Converts the string into a set of big endian bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|ch| ch.to_be_bytes()).collect()
    }
}
impl StaticPermittedAlphabet for TeletexString {
    type T = u32;
    // TODO add correct character set, see https://github.com/mouse07410/asn1c/blob/84d3a59c1bb89c59be6ca0625bb14ebea9084ba5/skeletons/TeletexString.c
    const CHARACTER_SET: &'static [u32] = &{
        let mut array = [0u32; 0xFFFF];
        let mut i = 0;
        while i < 0xFFFF {
            array[i as usize] = i;
            i += 1;
        }
        array
    };
    const CHARACTER_SET_NAME: constrained::CharacterSetName =
        constrained::CharacterSetName::Teletex;

    fn push_char(&mut self, ch: u32) {
        self.0.push(ch);
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

impl AsnType for TeletexString {
    const TAG: Tag = Tag::TELETEX_STRING;
}

impl Encode for TeletexString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_teletex_string(tag, constraints, self)
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
