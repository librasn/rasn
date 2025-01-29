use core::fmt;

use alloc::collections::BTreeMap;
use num_traits::{AsPrimitive, FromPrimitive, PrimInt, ToPrimitive, Unsigned};

use crate::error::strings::{InvalidRestrictedString, PermittedAlphabetError};
use alloc::{boxed::Box, vec::Vec};
use bitvec::prelude::*;

use crate::types;
pub(crate) enum CharacterSetName {
    Bmp,
    General,
    Graphic,
    IA5,
    Numeric,
    Printable,
    Teletex,
    Visible,
}
impl fmt::Display for CharacterSetName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bmp => write!(f, "BMPString"),
            Self::General => write!(f, "GeneralString"),
            Self::Graphic => write!(f, "GraphicString"),
            Self::IA5 => write!(f, "IA5String"),
            Self::Numeric => write!(f, "NumericString"),
            Self::Printable => write!(f, "PrintableString"),
            Self::Teletex => write!(f, "TeletexString"),
            Self::Visible => write!(f, "VisibleString"),
        }
    }
}

pub(crate) trait StaticPermittedAlphabet: Sized + Default {
    type T: PrimInt
        + Unsigned
        + ToPrimitive
        + FromPrimitive
        + AsPrimitive<u8>
        + AsPrimitive<u16>
        + AsPrimitive<u32>;
    const CHARACTER_SET: &'static [u32];
    /// Bits needed to represent a character in the character set so that every character can be represented
    /// Encoding specific requirement
    const CHARACTER_SET_WIDTH: usize = crate::num::log2(Self::CHARACTER_SET.len() as i128) as usize;
    const CHARACTER_SET_NAME: CharacterSetName;

    fn push_char(&mut self, ch: u32);
    fn chars(&self) -> impl Iterator<Item = u32> + '_;
    fn contains_char(ch: u32) -> bool {
        Self::CHARACTER_SET.contains(&ch)
    }
    fn invalid_restricted_string(ch: u32) -> InvalidRestrictedString {
        match Self::CHARACTER_SET_NAME {
            CharacterSetName::Bmp => InvalidRestrictedString::InvalidBmpString(ch.into()),
            CharacterSetName::General => InvalidRestrictedString::InvalidGeneralString(ch.into()),
            CharacterSetName::Graphic => InvalidRestrictedString::InvalidGraphicString(ch.into()),
            CharacterSetName::IA5 => InvalidRestrictedString::InvalidIA5String(ch.into()),
            CharacterSetName::Numeric => InvalidRestrictedString::InvalidNumericString(ch.into()),
            CharacterSetName::Printable => {
                InvalidRestrictedString::InvalidPrintableString(ch.into())
            }
            CharacterSetName::Teletex => InvalidRestrictedString::InvalidTeletexString(ch.into()),
            CharacterSetName::Visible => InvalidRestrictedString::InvalidVisibleString(ch.into()),
        }
    }
    fn try_from_slice(input: impl AsRef<[u8]>) -> Result<Vec<Self::T>, PermittedAlphabetError> {
        Self::try_from_slice_with_width(input, core::mem::size_of::<Self::T>())
    }
    fn try_from_slice_with_width(
        input: impl AsRef<[u8]>,
        width: usize,
    ) -> Result<Vec<Self::T>, PermittedAlphabetError> {
        let input = input.as_ref();
        // We currently only support character widths up to 4 bytes on error logic
        // Width can be larger than 4 only if we create new types with larger character widths
        debug_assert!(width <= 4);
        if width == 0 {
            return Err(PermittedAlphabetError::Other {
                message: alloc::format!(
                    "Character set width set to zero when parsing string {}",
                    Self::CHARACTER_SET_NAME
                ),
            });
        }
        // Input must be aligned with character encoding width to be valid input
        if input.len() % width != 0 {
            return Err(PermittedAlphabetError::InvalidData {
                length: input.len(),
                width,
            });
        }
        let num_elements = input.len() / width;
        let mut vec = Vec::with_capacity(num_elements);
        // Character width can be more than 1 byte, and combined bytes define the character encoding width
        let process_chunk: fn(&[u8]) -> Option<Self::T> = match width {
            1 => |chunk: &[u8]| Self::T::from_u8(chunk[0]),
            2 => |chunk: &[u8]| {
                Self::T::from_u16(u16::from_be_bytes(chunk.try_into().unwrap_or_default()))
            },
            3 | 4 => |chunk: &[u8]| {
                Self::T::from_u32(u32::from_be_bytes(chunk.try_into().unwrap_or_default()))
            },
            _ => unreachable!(),
        };

        for chunk in input.chunks_exact(width) {
            if let Some(character) = process_chunk(chunk) {
                if Self::contains_char(character.as_()) {
                    vec.push(character);
                } else {
                    return Err(PermittedAlphabetError::InvalidRestrictedString {
                        source: Self::invalid_restricted_string(
                            character.to_u32().unwrap_or_default(),
                        ),
                    });
                }
            }
        }
        Ok(vec)
    }
    fn index_map() -> &'static alloc::collections::BTreeMap<u32, u32>;
    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32>;
    fn char_range_to_bit_range(mut range: core::ops::Range<usize>) -> core::ops::Range<usize> {
        let width = Self::CHARACTER_SET_WIDTH;
        range.start *= width;
        range.end *= width;
        range
    }

    fn to_index_or_value_bitstring(&self) -> types::BitString {
        if should_be_indexed(Self::CHARACTER_SET_WIDTH as u32, Self::CHARACTER_SET) {
            self.to_index_string()
        } else {
            self.to_bit_string()
        }
    }

    fn to_index_string(&self) -> types::BitString {
        let index_map = Self::index_map();
        let mut index_string = types::BitString::new();
        let width = Self::CHARACTER_SET_WIDTH;
        for ch in self.chars() {
            let index = index_map.get(&ch).unwrap();
            index_string
                .extend_from_bitslice(&index.view_bits::<Msb0>()[(u32::BITS as usize - width)..]);
        }
        index_string
    }

    fn to_octet_aligned_index_string(&self) -> Vec<u8> {
        let index_map = Self::index_map();
        let mut index_string = types::BitString::new();
        let width = Self::CHARACTER_SET_WIDTH;
        let new_width = self.octet_aligned_char_width();

        for ch in self.chars() {
            let ch = &index_map[&ch].view_bits::<Msb0>()[(u32::BITS as usize - width)..];
            let mut padding = types::BitString::new();
            for _ in 0..(new_width - width) {
                padding.push(false);
            }
            padding.extend_from_bitslice(ch);
            index_string.extend(padding);
        }
        index_string.as_raw_slice().to_vec()
    }

    fn octet_aligned_char_width(&self) -> usize {
        Self::CHARACTER_SET_WIDTH
            .is_power_of_two()
            .then_some(Self::CHARACTER_SET_WIDTH)
            .unwrap_or_else(|| Self::CHARACTER_SET_WIDTH.next_power_of_two())
    }

    fn to_bit_string(&self) -> types::BitString {
        let mut octet_string = types::BitString::new();
        let width = Self::CHARACTER_SET_WIDTH;

        for ch in self.chars() {
            octet_string
                .extend_from_bitslice(&ch.view_bits::<Msb0>()[(u32::BITS as usize - width)..]);
        }
        octet_string
    }

    fn to_octet_aligned_string(&self) -> Vec<u8> {
        let mut octet_string = types::BitString::new();
        let width = self.octet_aligned_char_width();

        for ch in self.chars() {
            octet_string
                .extend_from_bitslice(&ch.view_bits::<Msb0>()[(u32::BITS as usize - width)..]);
        }
        octet_string.as_raw_slice().to_vec()
    }

    fn character_width() -> u32 {
        crate::num::log2(Self::CHARACTER_SET.len() as i128)
    }

    fn len(&self) -> usize {
        self.chars().count()
    }

    #[allow(clippy::box_collection)]
    fn build_index_map() -> Box<alloc::collections::BTreeMap<u32, u32>> {
        Box::new(
            Self::CHARACTER_SET
                .iter()
                .copied()
                .enumerate()
                .map(|(i, e)| (e, u32::from_usize(i).unwrap_or_default()))
                .collect(),
        )
    }

    #[allow(clippy::box_collection)]
    fn build_character_map() -> Box<alloc::collections::BTreeMap<u32, u32>> {
        Box::new(
            Self::CHARACTER_SET
                .iter()
                .copied()
                .enumerate()
                .map(|(i, e)| (u32::from_usize(i).unwrap_or_default(), e))
                .collect(),
        )
    }

    fn try_from_permitted_alphabet(
        input: crate::types::BitString,
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
        if character_width == 0 || bits.len() % character_width != 0 {
            return Err(PermittedAlphabetError::InvalidData {
                length: bits.len(),
                width: character_width,
            });
        }
        for ch in bits.chunks_exact(character_width) {
            let ch = ch.load_be::<u32>();
            if Self::contains_char(ch) {
                string.push_char(ch);
            } else {
                return Err(PermittedAlphabetError::InvalidRestrictedString {
                    source: Self::invalid_restricted_string(ch),
                });
            }
        }
        Ok(string)
    }
}

pub(crate) fn try_from_permitted_alphabet<S: StaticPermittedAlphabet>(
    input: crate::types::BitString,
    alphabet: &BTreeMap<u32, u32>,
) -> Result<S, PermittedAlphabetError> {
    let mut string = S::default();
    let permitted_alphabet_char_width = crate::num::log2(alphabet.len() as i128) as usize;
    // Alphabet should be always indexed key-alphabetvalue pairs at this point
    let values_only = alphabet.values().copied().collect::<Vec<u32>>();
    if should_be_indexed(permitted_alphabet_char_width as u32, &values_only) {
        for ch in input.chunks_exact(permitted_alphabet_char_width as usize) {
            let index = ch.load_be::<u32>();
            string.push_char(*alphabet.get(&index).ok_or(
                PermittedAlphabetError::IndexNotFound {
                    index: index.to_usize().unwrap_or_default(),
                },
            )?);
        }
    } else {
        string = S::try_from_bits(input, permitted_alphabet_char_width)?
    }
    Ok(string)
}
pub(crate) fn should_be_indexed(width: u32, character_set: &[u32]) -> bool {
    let largest_value = character_set.iter().copied().max().unwrap_or_default();
    2u32.pow(width) <= largest_value
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
