use core::fmt;

use alloc::collections::BTreeMap;
use num_traits::{AsPrimitive, FromPrimitive, PrimInt, ToPrimitive, Unsigned};

use crate::error::strings::{InvalidRestrictedString, PermittedAlphabetError};
use alloc::{boxed::Box, vec::Vec};
use bitvec::prelude::*;

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
    /// Whether `CHARACTER_SET` is sorted in ascending order, which allows
    /// membership tests by binary search.
    const CHARACTER_SET_IS_SORTED: bool = is_sorted(Self::CHARACTER_SET);
    /// The largest character value in `CHARACTER_SET`.
    const CHARACTER_SET_MAX: u32 = largest_character(Self::CHARACTER_SET);
    /// Membership of the character values below 256 in `CHARACTER_SET`, one
    /// bit per value.
    const CHARACTER_SET_BITMAP: [u64; 4] = bitmap_of(Self::CHARACTER_SET);
    /// Whether PER writes the characters of the unconstrained type by index
    /// rather than by value, because the largest value does not fit in
    /// `CHARACTER_SET_WIDTH` bits (ITU-T X.691 §30.5.4).
    const IS_INDEXED: bool = Self::CHARACTER_SET_WIDTH < 64
        && (1u64 << Self::CHARACTER_SET_WIDTH) <= Self::CHARACTER_SET_MAX as u64;

    fn push_char(&mut self, ch: u32);
    /// Reserves room for `additional` more characters.
    fn reserve(&mut self, additional: usize);
    fn chars(&self) -> impl Iterator<Item = u32> + '_;
    fn contains_char(ch: u32) -> bool {
        if Self::CHARACTER_SET_MAX < 256 {
            bitmap_contains(&Self::CHARACTER_SET_BITMAP, ch)
        } else if Self::CHARACTER_SET_IS_SORTED {
            Self::CHARACTER_SET.binary_search(&ch).is_ok()
        } else {
            Self::CHARACTER_SET.contains(&ch)
        }
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
    fn character_map() -> &'static alloc::collections::BTreeMap<u32, u32>;
    fn character_width() -> u32 {
        crate::num::log2(Self::CHARACTER_SET.len() as i128)
    }

    fn len(&self) -> usize {
        self.chars().count()
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
        if character_width == 0 || !bits.len().is_multiple_of(character_width) {
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
        for ch in input.chunks_exact(permitted_alphabet_char_width) {
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

const fn is_sorted(set: &[u32]) -> bool {
    let mut i = 1;
    while i < set.len() {
        if set[i - 1] > set[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn largest_character(set: &[u32]) -> u32 {
    let mut largest = 0;
    let mut i = 0;
    while i < set.len() {
        if set[i] > largest {
            largest = set[i];
        }
        i += 1;
    }
    largest
}

/// One bit per character value below 256; larger values are not recorded.
const fn bitmap_of(set: &[u32]) -> [u64; 4] {
    let mut bitmap = [0u64; 4];
    let mut i = 0;
    while i < set.len() {
        if set[i] < 256 {
            bitmap[(set[i] / 64) as usize] |= 1 << (set[i] % 64);
        }
        i += 1;
    }
    bitmap
}

fn bitmap_contains(bitmap: &[u64; 4], ch: u32) -> bool {
    ch < 256 && (bitmap[(ch / 64) as usize] >> (ch % 64)) & 1 == 1
}

/// Sets up to this size are scanned linearly, which vectorises well; larger
/// sorted sets are searched by bisection.
const SMALL_SET: usize = 64;

fn invalid_base_character<S: StaticPermittedAlphabet>(ch: u32) -> PermittedAlphabetError {
    PermittedAlphabetError::InvalidRestrictedString {
        source: S::invalid_restricted_string(ch),
    }
}

/// The effective alphabet of a known-multiplier character string in PER
/// (ITU-T X.691 §30.5): the set the characters are drawn from, the number of
/// bits each character occupies, and whether characters are written by value
/// or by their index in the set.
pub(crate) struct CharacterAlphabet<'a> {
    set: &'a [u32],
    width: usize,
    indexed: bool,
    sorted: bool,
    /// Membership by value, present when every character is below 256.
    bitmap: Option<[u64; 4]>,
    invalid: fn(u32) -> PermittedAlphabetError,
}

impl<'a> CharacterAlphabet<'a> {
    /// Resolves the alphabet of `S`, narrowed to `permitted` when that
    /// constraint changes the encoding, for the aligned or unaligned variant.
    pub(crate) fn new<S: StaticPermittedAlphabet>(
        permitted: Option<&'a [u32]>,
        aligned: bool,
    ) -> Self {
        let align = |width: usize| {
            if aligned && !width.is_power_of_two() {
                width.next_power_of_two()
            } else {
                width
            }
        };
        // A single-character alphabet carries no information, so its
        // characters occupy no bits.
        let width_of = |count: usize| {
            if count <= 1 {
                0
            } else {
                align(crate::num::log2(count as i128) as usize)
            }
        };
        let is_indexed = |width: usize, largest: u32| (1u64 << width) <= u64::from(largest);

        match permitted {
            Some(set)
                if S::IS_INDEXED
                    || S::CHARACTER_SET_WIDTH
                        > align(crate::num::log2(set.len().max(1) as i128) as usize) =>
            {
                let sorted = set.is_sorted();
                let largest = if sorted {
                    set.last().copied()
                } else {
                    set.iter().copied().max()
                }
                .unwrap_or_default();
                let width = width_of(set.len());
                Self {
                    set,
                    width,
                    indexed: is_indexed(width, largest),
                    sorted,
                    bitmap: (largest < 256).then(|| bitmap_of(set)),
                    invalid: |character| PermittedAlphabetError::CharacterNotFound { character },
                }
            }
            _ => {
                let width = width_of(S::CHARACTER_SET.len());
                Self {
                    set: S::CHARACTER_SET,
                    width,
                    indexed: is_indexed(width, S::CHARACTER_SET_MAX),
                    sorted: S::CHARACTER_SET_IS_SORTED,
                    bitmap: (S::CHARACTER_SET_MAX < 256).then_some(S::CHARACTER_SET_BITMAP),
                    invalid: invalid_base_character::<S>,
                }
            }
        }
    }

    /// The number of bits each character occupies.
    pub(crate) fn width(&self) -> usize {
        self.width
    }

    fn contains(&self, ch: u32) -> bool {
        match self.bitmap {
            Some(bitmap) => bitmap_contains(&bitmap, ch),
            None if self.sorted && self.set.len() > SMALL_SET => {
                self.set.binary_search(&ch).is_ok()
            }
            None => self.set.contains(&ch),
        }
    }

    fn position(&self, ch: u32) -> Option<usize> {
        if self.sorted && self.set.len() > SMALL_SET {
            self.set.binary_search(&ch).ok()
        } else {
            self.set.iter().position(|&candidate| candidate == ch)
        }
    }

    /// The code that represents `ch` in the encoding.
    pub(crate) fn encode(&self, ch: u32) -> Result<u32, PermittedAlphabetError> {
        if self.indexed {
            self.position(ch)
                .map(|index| index as u32)
                .ok_or_else(|| (self.invalid)(ch))
        } else if self.contains(ch) {
            Ok(ch)
        } else {
            Err((self.invalid)(ch))
        }
    }

    /// The character that `code` represents in the encoding.
    pub(crate) fn decode(&self, code: u32) -> Result<u32, PermittedAlphabetError> {
        if self.indexed {
            self.set
                .get(code as usize)
                .copied()
                .ok_or(PermittedAlphabetError::IndexNotFound {
                    index: code as usize,
                })
        } else if self.contains(code) {
            Ok(code)
        } else {
            Err((self.invalid)(code))
        }
    }
}
