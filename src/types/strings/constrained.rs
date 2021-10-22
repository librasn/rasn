use alloc::collections::BTreeMap;

use bitvec::prelude::*;
use once_cell::race::OnceBox;

use crate::types;

#[derive(Debug, snafu::Snafu)]
pub enum FromPermittedAlphabetError {
    #[snafu(display("error converting to string: {}", message))]
    Other { message: String },
    #[snafu(display(
        "length of bits ({length}) provided not divisible by character width ({width})"
    ))]
    InvalidData { length: usize, width: usize },
    #[snafu(display("index not found {}", 0))]
    IndexNotFound { index: u32 },
}

pub(crate) trait StaticPermittedAlphabet: Sized + Default {
    const CHARACTER_SET: &'static [u32];
    const CHARACTER_WIDTH: u32 = crate::per::log2(Self::CHARACTER_SET.len() as i128);

    fn push_char(&mut self, ch: u32);
    fn chars(&self) -> Box<dyn Iterator<Item = u32> + '_>;

    fn char_range_to_bit_range(mut range: core::ops::Range<usize>) -> core::ops::Range<usize> {
        let width = Self::CHARACTER_WIDTH as usize;
        range.start *= width;
        range.end *= width;
        range
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

        crate::per::to_vec(&index_string)
    }

    fn octet_aligned_char_width(&self) -> u32 {
        Self::CHARACTER_WIDTH
            .is_power_of_two()
            .then_some(Self::CHARACTER_WIDTH)
            .unwrap_or_else(|| Self::CHARACTER_WIDTH.next_power_of_two())
    }

    fn to_octet_aligned_string(&self) -> Vec<u8> {
        let mut octet_string = types::BitString::new();
        let width = Self::CHARACTER_WIDTH as usize;
        let new_width = self.octet_aligned_char_width() as usize;

        for ch in self.chars() {
            let mut padding = types::BitString::repeat(false, new_width - width);
            padding.extend_from_bitslice(&ch.view_bits::<Msb0>()[width..]);
            octet_string.extend(padding);
        }
        crate::per::to_vec(&octet_string)
    }

    fn character_width() -> u32 {
        crate::per::log2(Self::CHARACTER_SET.len() as i128)
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
    ) -> Result<Self, FromPermittedAlphabetError> {
        let alphabet = alphabet.unwrap_or_else(|| Self::character_map());
        try_from_permitted_alphabet(input, alphabet)
    }
}

pub(crate) fn try_from_permitted_alphabet<S: StaticPermittedAlphabet>(
    input: &types::BitStr,
    alphabet: &BTreeMap<u32, u32>,
) -> Result<S, FromPermittedAlphabetError> {
    let mut string = S::default();
    let permitted_alphabet_char_width = crate::per::log2(alphabet.len() as i128);

    for ch in input.chunks_exact(permitted_alphabet_char_width as usize) {
        let index = ch.load_be();

        string.push_char(
            *alphabet
                .get(&index)
                .ok_or(FromPermittedAlphabetError::IndexNotFound { index })?,
        );
    }

    Ok(string)
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynConstrainedCharacterString {
    character_set: BTreeMap<u32, u32>,
    buffer: types::BitString,
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
#[snafu(display("character not found in character set"))]
pub struct ConstrainedConversionError;

impl DynConstrainedCharacterString {
    pub fn from_bits(
        data: impl Iterator<Item = u32>,
        character_set: &[u32],
    ) -> Result<Self, ConstrainedConversionError> {
        let mut buffer = types::BitString::new();
        let char_width = crate::per::log2(character_set.len() as i128);
        let alphabet = BTreeMap::from_iter(
            character_set
                .iter()
                .enumerate()
                .map(|(i, a)| (*a, i as u32)),
        );

        for ch in data {
            let Some(index) = alphabet.get(&ch).copied() else {
                return Err(ConstrainedConversionError)
            };
            let range = ((u32::BITS - char_width) as usize)..(u32::BITS as usize);
            let bit_ch = &index.view_bits::<Msb0>()[range];
            buffer.extend_from_bitslice(bit_ch);
        }

        Ok(Self {
            character_set: alphabet,
            buffer,
        })
    }

    pub fn character_width(&self) -> usize {
        crate::per::log2(self.character_set.len() as i128) as usize
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
