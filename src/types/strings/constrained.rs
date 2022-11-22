use alloc::collections::BTreeMap;

use bitvec::prelude::*;

use crate::types;

pub enum OctetAlignedString {
    U8(types::BitString),
    U16(types::BitString),
    U32(types::BitString),
}

impl OctetAlignedString {
    const fn width(&self) -> usize {
        match self {
            Self::U8(_) => u8::BITS as usize,
            Self::U16(_) => u16::BITS as usize,
            Self::U32(_) => u32::BITS as usize,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self {
            Self::U8(vec) => vec.clone().into_vec(),
            Self::U16(vec) => vec.chunks_exact(u16::BITS as usize).flat_map(|item| item.load::<u16>().to_be_bytes()).collect(),
            Self::U32(vec) => vec.chunks_exact(u32::BITS as usize).flat_map(|item| item.load::<u16>().to_be_bytes()).collect(),
        }
    }
}

impl core::ops::Index<usize> for OctetAlignedString {
    type Output = types::BitStr;

    fn index(&self, index: usize) -> &Self::Output {
        let width = self.width();

        match self {
            Self::U8(vec) => &vec[index..index*width],
            Self::U16(vec) => &vec[index..index*width],
            Self::U32(vec) => &vec[index..index*width],
        }
    }
}

impl core::ops::Index<core::ops::Range<usize>> for OctetAlignedString {
    type Output = types::BitStr;

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        let width = self.width();
        match self {
            Self::U8(vec) => &vec[index.start * width..index.end * width],
            Self::U16(vec) => &vec[index.start * width..index.end * width],
            Self::U32(vec) => &vec[index.start * width..index.end * width],
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstrainedCharacterString<const WIDTH: usize> {
    buffer: types::BitString,
}

impl<const WIDTH: usize> ConstrainedCharacterString<WIDTH> {
    pub fn from_raw_bits(buffer: types::BitString) -> Self {
        debug_assert!(buffer.is_empty() || buffer.len() >= WIDTH);
        debug_assert!(buffer.len() % WIDTH == 0);

        Self { buffer }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn character_width(&self) -> usize {
        WIDTH
    }

    pub fn len(&self) -> usize {
        self.buffer.len() / WIDTH
    }

    pub fn as_bitstr(&self) -> &types::BitStr {
        &self.buffer
    }

    pub fn to_octet_aligned(&self) -> OctetAlignedString {
        match WIDTH.next_power_of_two() {
            0..=8 => OctetAlignedString::U8(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u8>()))),
            9..=16 => OctetAlignedString::U16(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u16>()))),
            17..=32 => OctetAlignedString::U32(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u32>()))),
            _ => unreachable!("character widths beyond 32 bits are unsupported"),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &types::BitStr> + '_ {
        self.buffer.chunks_exact(WIDTH)
    }
}

impl<const WIDTH: usize> core::ops::Index<usize> for ConstrainedCharacterString<WIDTH> {
    type Output = types::BitStr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index..index * WIDTH]
    }
}

impl<const WIDTH: usize> core::ops::Index<core::ops::Range<usize>> for ConstrainedCharacterString<WIDTH> {
    type Output = types::BitStr;

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.buffer[index.start * WIDTH..index.end * WIDTH]
    }
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
    pub fn from_bits(data: &types::BitStr, original_character_width: usize, character_set: &'static [u32]) -> Result<Self, ConstrainedConversionError> {
        let mut buffer = types::BitString::new();
        let char_width = crate::per::log2(character_set.len() as i128) as u32;
        let alphabet = BTreeMap::from_iter(character_set.into_iter().enumerate().map(|(i, a)| (*a, i as u32)));

        for ch in data.chunks(original_character_width as usize) {
            let ch = ch.load_be::<u32>();
            let Some(index) = alphabet.get(&ch).copied() else {
                return Err(ConstrainedConversionError)
            };
            let range = ((u32::BITS-char_width) as usize)..(u32::BITS as usize);
            let bit_ch = dbg!(&index.view_bits::<Msb0>()[range]);
            dbg!(bit_ch.load_be::<u8>());
            buffer.extend_from_bitslice(bit_ch);
        }

        Ok(Self {
            character_set: alphabet,
            buffer,
        })
    }

    pub fn from_known_multiplier_string<const WIDTH: usize>(string: &ConstrainedCharacterString<WIDTH>, character_set: &'static [u32]) -> Result<Self, ConstrainedConversionError> {
        Self::from_bits(string.as_bitstr(), string.character_width(), character_set)
    }

    pub fn character_width(&self) -> usize {
        crate::per::log2(self.character_set.len() as i128) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.buffer.len() / self.character_width()
    }

    pub fn as_bitstr(&self) -> &types::BitStr {
        &self.buffer
    }

    pub fn to_octet_aligned(&self) -> OctetAlignedString {
        match self.character_width().next_power_of_two() {
            0..=8 => OctetAlignedString::U8(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u8>()))),
            9..=16 => OctetAlignedString::U16(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u16>()))),
            17..=32 => OctetAlignedString::U32(collapse_bit_storage(self.iter().map(|slice| slice.load_be::<u32>()))),
            _ => unreachable!("character widths beyond 32 bits are unsupported"),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &types::BitStr> + '_ {
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

fn collapse_bit_storage<'bits, T: bitvec::store::BitStore>(iter: impl IntoIterator<Item = T>) -> types::BitString {
    let mut string = types::BitString::new();
    for slice in iter {
        string.extend_from_bitslice(slice.view_bits::<bitvec::order::Msb0>());
    }
    string
}
