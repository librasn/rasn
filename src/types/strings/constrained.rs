use alloc::collections::BTreeMap;

use crate::error::strings::PermittedAlphabetError;
use alloc::{boxed::Box, vec::Vec};
use bitvec::prelude::*;
use once_cell::race::OnceBox;

use crate::types;

pub(crate) trait StaticPermittedAlphabet: Sized + Default {
    const CHARACTER_SET: &'static [u32];
    const CHARACTER_WIDTH: u32 = crate::num::log2(Self::CHARACTER_SET.len() as i128);

    fn push_char(&mut self, ch: u32);
    fn chars(&self) -> Box<dyn Iterator<Item = u32> + '_>;
    fn char_range_to_bit_range(mut range: core::ops::Range<usize>) -> core::ops::Range<usize> {
        let width = Self::CHARACTER_WIDTH as usize;
        range.start *= width;
        range.end *= width;
        range
    }

    fn to_index_or_value_bitstring(&self) -> types::BitString {
        if should_be_indexed(Self::CHARACTER_WIDTH, Self::CHARACTER_SET) {
            self.to_index_string()
        } else {
            self.to_bit_string()
        }
    }

    fn to_index_string(&self) -> types::BitString {
        let index_map = Self::index_map();
        let mut index_string = types::BitString::new();
        let width = Self::CHARACTER_WIDTH;
        for ch in self.chars() {
            let index = index_map[&ch];
            index_string
                .extend_from_bitslice(&index.view_bits::<Msb0>()[(u32::BITS - width) as usize..]);
        }
        index_string
    }

    fn to_octet_aligned_index_string(&self) -> Vec<u8> {
        let index_map = Self::index_map();
        let mut index_string = types::BitString::new();
        let width = Self::CHARACTER_WIDTH;
        let new_width = self.octet_aligned_char_width() as usize;

        for ch in self.chars() {
            let ch = &index_map[&ch].view_bits::<Msb0>()[(u32::BITS - width) as usize..];
            let mut padding = types::BitString::new();
            for _ in 0..(new_width - width as usize) {
                padding.push(false);
            }
            padding.extend_from_bitslice(ch);
            index_string.extend(padding);
        }

        crate::bits::to_vec(&index_string)
    }

    fn octet_aligned_char_width(&self) -> u32 {
        Self::CHARACTER_WIDTH
            .is_power_of_two()
            .then_some(Self::CHARACTER_WIDTH)
            .unwrap_or_else(|| Self::CHARACTER_WIDTH.next_power_of_two())
    }

    fn to_bit_string(&self) -> types::BitString {
        let mut octet_string = types::BitString::new();
        let width = Self::CHARACTER_WIDTH;

        for ch in self.chars() {
            octet_string
                .extend_from_bitslice(&ch.view_bits::<Msb0>()[(u32::BITS - width) as usize..]);
        }
        octet_string
    }

    fn to_octet_aligned_string(&self) -> Vec<u8> {
        let mut octet_string = types::BitString::new();
        let width = self.octet_aligned_char_width();

        for ch in self.chars() {
            octet_string
                .extend_from_bitslice(&ch.view_bits::<Msb0>()[(u32::BITS - width) as usize..]);
        }
        crate::bits::to_vec(&octet_string)
    }

    fn character_width() -> u32 {
        crate::num::log2(Self::CHARACTER_SET.len() as i128)
    }

    fn len(&self) -> usize {
        self.chars().count()
    }

    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        static MAP: OnceBox<BTreeMap<u32, u32>> = OnceBox::new();

        MAP.get_or_init(|| {
            Box::new(
                Self::CHARACTER_SET
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, e)| (e, i as u32))
                    .collect(),
            )
        })
    }

    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32> {
        static MAP: OnceBox<BTreeMap<u32, u32>> = OnceBox::new();

        MAP.get_or_init(|| {
            Box::new(
                Self::CHARACTER_SET
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, e)| (i as u32, e))
                    .collect(),
            )
        })
    }

    fn try_from_permitted_alphabet(
        input: &types::BitStr,
        alphabet: Option<&BTreeMap<u32, u32>>,
    ) -> Result<Self, PermittedAlphabetError> {
        let alphabet = alphabet.unwrap_or_else(|| Self::character_map());
        try_from_permitted_alphabet(input, alphabet)
    }

    #[track_caller]
    fn try_from_bits(
        bits: crate::types::BitString,
        character_width: usize,
    ) -> Result<Self, PermittedAlphabetError> {
        let mut string = Self::default();
        if bits.len() % character_width != 0 {
            return Err(PermittedAlphabetError::InvalidData {
                length: bits.len(),
                width: character_width,
            });
        }

        for ch in bits.chunks_exact(character_width) {
            string.push_char(ch.load_be());
        }

        Ok(string)
    }
}

pub(crate) fn try_from_permitted_alphabet<S: StaticPermittedAlphabet>(
    input: &types::BitStr,
    alphabet: &BTreeMap<u32, u32>,
) -> Result<S, PermittedAlphabetError> {
    let mut string = S::default();
    let permitted_alphabet_char_width = crate::num::log2(alphabet.len() as i128);
    // Alphabet should be always indexed key-alphabetvalue pairs at this point
    let values_only = alphabet.values().copied().collect::<Vec<u32>>();
    if should_be_indexed(permitted_alphabet_char_width, &values_only) {
        for ch in input.chunks_exact(permitted_alphabet_char_width as usize) {
            let index = ch.load_be();
            string.push_char(
                *alphabet
                    .get(&index)
                    .ok_or_else(|| PermittedAlphabetError::IndexNotFound { index })?,
            );
        }
    } else {
        for ch in input.chunks_exact(permitted_alphabet_char_width as usize) {
            let value = ch.load_be();
            string.push_char(value);
        }
    }

    Ok(string)
}
pub(crate) fn should_be_indexed(width: u32, character_set: &[u32]) -> bool {
    let largest_value = character_set.iter().copied().max().unwrap_or(0);
    if 2u32.pow(width) > largest_value {
        false
    } else {
        true
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynConstrainedCharacterString {
    character_set: BTreeMap<u32, u32>,
    buffer: types::BitString,
}

impl DynConstrainedCharacterString {
    pub fn from_bits(
        data: impl Iterator<Item = u32>,
        character_set: &[u32],
    ) -> Result<Self, PermittedAlphabetError> {
        let mut buffer = types::BitString::new();
        let char_width = crate::num::log2(character_set.len() as i128);
        let indexed = should_be_indexed(char_width, character_set);
        let alphabet: BTreeMap<u32, u32>;
        if indexed {
            alphabet = character_set
                .iter()
                .enumerate()
                .map(|(i, a)| (*a, i as u32))
                .collect::<BTreeMap<_, _>>();
            for ch in data {
                let Some(index) = alphabet.get(&ch).copied() else {
                    return Err(PermittedAlphabetError::CharacterNotFound { character: ch });
                };
                let range = ((u32::BITS - char_width) as usize)..(u32::BITS as usize);
                let bit_ch = &index.view_bits::<Msb0>()[range];
                buffer.extend_from_bitslice(bit_ch);
            }
        } else {
            alphabet = character_set
                .iter()
                .enumerate()
                .map(|(i, a)| (i as u32, *a))
                .collect::<BTreeMap<_, _>>();
            for ch in data {
                let range = ((u32::BITS - char_width) as usize)..(u32::BITS as usize);
                let bit_ch = &ch.view_bits::<Msb0>()[range];
                buffer.extend_from_bitslice(bit_ch);
            }
        }

        Ok(Self {
            character_set: alphabet,
            buffer,
        })
    }

    pub fn character_width(&self) -> usize {
        crate::num::log2(self.character_set.len() as i128) as usize
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.buffer.len() / self.character_width()
    }

    #[allow(unused)]
    fn as_bitstr(&self) -> &types::BitStr {
        &self.buffer
    }

    #[allow(unused)]
    fn iter(&self) -> impl Iterator<Item = &types::BitStr> + '_ {
        self.buffer.chunks_exact(self.character_width())
    }
}

impl core::ops::Index<usize> for DynConstrainedCharacterString {
    type Output = types::BitStr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index..index * self.character_width()]
    }
}

impl core::ops::Index<core::ops::Range<usize>> for DynConstrainedCharacterString {
    type Output = types::BitStr;

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        let width = self.character_width();
        &self.buffer[index.start * width..index.end * width]
    }
}
