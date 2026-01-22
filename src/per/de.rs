//! Decoding Packed Encoding Rules data into Rust structures.

use alloc::{borrow::Cow, collections::VecDeque, string::ToString, vec::Vec};
use bitvec::field::BitField;

use super::{
    FOURTY_EIGHT_K, LARGE_UNSIGNED_CONSTRAINT, SIXTEEN_K, SIXTY_FOUR_K, SMALL_UNSIGNED_CONSTRAINT,
    THIRTY_TWO_K,
};
use crate::{
    de::Error as _,
    types::{
        self,
        constraints::{self, Extensible},
        fields::{Field, Fields},
        strings::{should_be_indexed, StaticPermittedAlphabet},
        Constraints, Enumerated, IntegerType, SetOf, Tag,
    },
    Decode,
};

pub use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
type Result<T, E = DecodeError> = core::result::Result<T, E>;

type InputSlice<'input> = nom_bitvec::BSlice<'input, u8, bitvec::order::Msb0>;

/// Options for configuring the [`Decoder`].
#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    #[allow(unused)]
    aligned: bool,
    // limit decoding to prevent stack overflow from deep or circular references
    remaining_depth: usize,
}

impl DecoderOptions {
    /// Returns the default decoding rules options for Aligned Packed Encoding Rules.
    #[must_use]
    pub fn aligned() -> Self {
        Self {
            aligned: true,
            remaining_depth: 128,
        }
    }

    /// Returns the default decoding rules options for unaligned Packed Encoding Rules.
    #[must_use]
    pub fn unaligned() -> Self {
        Self {
            aligned: false,
            remaining_depth: 128,
        }
    }

    #[must_use]
    fn current_codec(self) -> crate::Codec {
        if self.aligned {
            crate::Codec::Aper
        } else {
            crate::Codec::Uper
        }
    }
}

/// Decodes Packed Encoding Rules (PER) data into Rust data structures.
pub struct Decoder<'input, const RFC: usize = 0, const EFC: usize = 0> {
    input: InputSlice<'input>,
    options: DecoderOptions,
    /// When the decoder contains fields, we check against optional or default
    /// fields to know the presence of those fields.
    fields: VecDeque<(Field, bool)>,
    extension_fields: Option<Fields<EFC>>,
    extensions_present: Option<Option<VecDeque<(Field, bool)>>>,
}

impl<'input, const RFC: usize, const EFC: usize> Decoder<'input, RFC, EFC> {
    /// Returns the currently selected codec.
    #[must_use]
    pub fn codec(&self) -> crate::Codec {
        self.options.current_codec()
    }

    /// Creates a new Decoder from the given input and options.
    #[must_use]
    pub fn new(input: &'input crate::types::BitStr, options: DecoderOptions) -> Self {
        Self {
            input: input.into(),
            options,
            fields: <_>::default(),
            extension_fields: <_>::default(),
            extensions_present: <_>::default(),
        }
    }

    /// Returns the remaining input, if any.
    #[must_use]
    pub fn input(&self) -> &'input crate::types::BitStr {
        self.input.0
    }

    #[track_caller]
    fn require_field(&mut self, tag: Tag) -> Result<bool> {
        if self
            .fields
            .front()
            .is_some_and(|field| field.0.tag_tree.smallest_tag() == tag)
        {
            Ok(self.fields.pop_front().unwrap().1)
        } else {
            Err(DecodeError::missing_tag_class_or_value_in_sequence_or_set(
                tag.class,
                tag.value,
                self.codec(),
            ))
        }
    }

    fn parse_extensible_bit(&mut self, constraints: &Constraints) -> Result<bool> {
        constraints
            .extensible()
            .then(|| self.parse_one_bit())
            .transpose()
            .map(core::option::Option::unwrap_or_default)
    }

    fn extension_is_present(&mut self) -> Result<Option<(Field, bool)>> {
        let codec = self.codec();
        Ok(self
            .extensions_present
            .as_mut()
            .ok_or_else(|| DecodeError::type_not_extensible(codec))?
            .as_mut()
            .ok_or_else(|| DecodeError::type_not_extensible(codec))?
            .pop_front())
    }

    fn parse_padding(&self, input: InputSlice<'input>) -> Result<InputSlice<'input>> {
        if self.options.aligned {
            self.force_parse_padding(input)
        } else {
            Ok(input)
        }
    }

    fn force_parse_padding(&self, input: InputSlice<'input>) -> Result<InputSlice<'input>> {
        if input.len().is_multiple_of(8) {
            Ok(input)
        } else {
            let (input, _) = nom::bytes::streaming::take(input.len() % 8)(input)
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
            Ok(input)
        }
    }

    fn parse_optional_and_default_field_bitmap<const RC: usize>(
        &mut self,
        fields: &Fields<RC>,
    ) -> Result<InputSlice<'input>> {
        let (input, bitset) =
            nom::bytes::streaming::take(fields.number_of_optional_and_default_fields())(self.input)
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

        self.input = input;
        Ok(bitset)
    }

    fn decode_extensible_string(
        &mut self,
        constraints: &Constraints,
        is_large_string: bool,
        mut decode_fn: impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<()> {
        let extensible_is_present = self.parse_extensible_bit(constraints)?;
        let constraints = constraints.size().filter(|_| !extensible_is_present);
        let input =
            self.decode_string_length(self.input, constraints, is_large_string, &mut decode_fn)?;
        self.input = input;
        Ok(())
    }

    fn decode_extensible_container(
        &mut self,
        constraints: Constraints,
        mut decode_fn: impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<()> {
        let extensible_is_present = self.parse_extensible_bit(&constraints)?;
        let constraints = constraints.size().filter(|_| !extensible_is_present);
        let input = self.decode_length(self.input, constraints, &mut decode_fn)?;
        self.input = input;
        Ok(())
    }

    fn decode_octets(&mut self) -> Result<types::BitString> {
        let mut buffer = types::BitString::default();
        let codec = self.codec();

        let input = self.decode_length(self.input, <_>::default(), &mut |input, length| {
            let (input, data) = nom::bytes::streaming::take(length * 8)(input)
                .map_err(|e| DecodeError::map_nom_err(e, codec))?;
            buffer.extend(&*data);
            Ok(input)
        })?;

        self.input = input;
        Ok(buffer)
    }

    fn decode_unknown_length(
        &mut self,
        mut input: InputSlice<'input>,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        input = self.parse_padding(input)?;
        let (input, mask) = nom::bytes::streaming::take(1u8)(input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

        if !mask[0] {
            let (input, length) = nom::bytes::streaming::take(7u8)(input)
                .map(|(i, bs)| (i, bs.to_bitvec()))
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
            (decode_fn)(input, length.load_be::<usize>())
        } else {
            let (input, mask) = nom::bytes::streaming::take(1u8)(input)
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

            if !mask[0] {
                let (input, length) = nom::bytes::streaming::take(14u8)(input)
                    .map(|(i, bs)| (i, bs.to_bitvec()))
                    .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
                (decode_fn)(input, length.load_be::<usize>())
            } else {
                let (input, mask) = nom::bytes::streaming::take(6u8)(input)
                    .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
                let length: usize = match mask.load_be::<u8>() {
                    1 => SIXTEEN_K.into(),
                    2 => THIRTY_TWO_K.into(),
                    3 => FOURTY_EIGHT_K.into(),
                    4 => SIXTY_FOUR_K as usize,
                    _ => {
                        return Err(DecodeError::parser_fail(
                            "Invalid length fragment".into(),
                            self.codec(),
                        ));
                    }
                };

                let mut input = (decode_fn)(input, length)?;

                loop {
                    let new_input = self.decode_length(input, <_>::default(), decode_fn)?;

                    if input.len() == new_input.len() || new_input.is_empty() {
                        break;
                    } else {
                        input = (decode_fn)(new_input, length)?;
                    }
                }

                Ok(input)
            }
        }
    }

    fn decode_string_length(
        &mut self,
        mut input: InputSlice<'input>,
        constraints: Option<&Extensible<constraints::Size>>,
        is_large_string: bool,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        let Some(constraints) = constraints else {
            return self.decode_unknown_length(input, decode_fn);
        };

        let size_constraint = constraints.constraint;
        if let Some(range) = size_constraint
            .range()
            .filter(|range| *range <= SIXTY_FOUR_K as usize)
        {
            if range == 0 {
                Ok(input)
            } else if range == 1 {
                if self.options.aligned {
                    input = self.parse_padding(input)?;
                }
                (decode_fn)(input, size_constraint.minimum())
            } else {
                let range = if self.options.aligned && range > 256 {
                    input = self.parse_padding(input)?;
                    let range = crate::num::log2(range as i128);
                    crate::bits::range_from_len(if range.is_power_of_two() {
                        range
                    } else {
                        range.next_power_of_two()
                    })
                } else {
                    range as i128
                };

                let (mut input, length) =
                    nom::bytes::streaming::take(crate::num::log2(range))(input)
                        .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
                if is_large_string {
                    input = self.parse_padding(input)?;
                }
                length
                    .load_be::<usize>()
                    .checked_add(size_constraint.minimum())
                    .ok_or_else(|| DecodeError::exceeds_max_length(usize::MAX.into(), self.codec()))
                    .and_then(|sum| (decode_fn)(input, sum))
            }
        } else {
            self.decode_unknown_length(input, decode_fn)
        }
    }

    fn decode_length(
        &mut self,
        mut input: InputSlice<'input>,
        constraints: Option<&Extensible<constraints::Size>>,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        let Some(constraints) = constraints else {
            return self.decode_unknown_length(input, decode_fn);
        };

        let size_constraint = constraints.constraint;
        if let Some(range) = size_constraint
            .range()
            .filter(|range| *range <= SIXTY_FOUR_K as usize)
        {
            if range == 0 {
                Ok(input)
            } else if range == 1 {
                (decode_fn)(input, size_constraint.minimum())
            } else {
                let range = if self.options.aligned && range > 256 {
                    input = self.parse_padding(input)?;
                    let range = crate::num::log2(range as i128);
                    crate::bits::range_from_len(if range.is_power_of_two() {
                        range
                    } else {
                        range.next_power_of_two()
                    })
                } else {
                    range as i128
                };

                let (mut input, length) =
                    nom::bytes::streaming::take(crate::num::log2(range))(input)
                        .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
                input = self.parse_padding(input)?;
                length
                    .load_be::<usize>()
                    .checked_add(size_constraint.minimum())
                    .ok_or_else(|| DecodeError::exceeds_max_length(usize::MAX.into(), self.codec()))
                    .and_then(|sum| (decode_fn)(input, sum))
            }
        } else {
            self.decode_unknown_length(input, decode_fn)
        }
    }

    fn parse_one_bit(&mut self) -> Result<bool> {
        let (input, boolean) = nom::bytes::streaming::take(1u8)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = input;
        Ok(boolean[0])
    }

    fn parse_normally_small_integer<I: IntegerType>(&mut self) -> Result<I> {
        let is_large = self.parse_one_bit()?;
        if is_large {
            self.parse_integer::<I>(LARGE_UNSIGNED_CONSTRAINT)
        } else {
            self.parse_integer::<I>(SMALL_UNSIGNED_CONSTRAINT)
        }
    }

    fn parse_non_negative_binary_integer<I: types::IntegerType>(
        &mut self,
        range: i128,
    ) -> Result<I> {
        let bits = crate::num::log2(range);
        let (input, data) = nom::bytes::streaming::take(bits)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = input;
        let data = if data.len() < 8 {
            let mut buffer = types::BitString::repeat(false, 8 - data.len());
            buffer.extend_from_bitslice(&data);
            buffer
        } else {
            let remainder = data.len() % 8;
            if remainder != 0 {
                let mut buffer = types::BitString::repeat(false, 8 - remainder);
                buffer.extend_from_bitslice(&data);
                buffer
            } else {
                let mut vec_bytes = data.to_bitvec();
                // See https://github.com/ferrilab/bitvec/issues/228
                // At this point the data might be a result of indexing a slice (as a result of streaming::take), which might not be aligned in memory
                // To fix this we force align the data
                vec_bytes.force_align();
                vec_bytes
            }
        };
        I::try_from_unsigned_bytes(data.as_raw_slice(), self.codec())
    }

    fn parse_integer<I: types::IntegerType>(&mut self, constraints: Constraints) -> Result<I> {
        let extensible = self.parse_extensible_bit(&constraints)?;
        let value_constraint = constraints.value();

        let Some(value_constraint) = value_constraint.filter(|_| !extensible) else {
            let bytes = &self.decode_octets()?;
            return I::try_from_bytes(bytes.as_raw_slice(), self.codec());
        };

        const K64: i128 = SIXTY_FOUR_K as i128;
        const OVER_K64: i128 = K64 + 1;

        // Doing .map_error here causes 5% performance regression for unknown reason
        // It would make code cleaner though
        let minimum: I = match value_constraint.constraint.minimum().try_into() {
            Ok(value) => value,
            Err(_) => return Err(DecodeError::integer_overflow(I::WIDTH, self.codec())),
        };

        let number = if let Some(range) = value_constraint.constraint.range() {
            match (self.options.aligned, range) {
                (_, 0) => return Ok(minimum),
                (true, 256) => {
                    self.input = self.parse_padding(self.input)?;
                    self.parse_non_negative_binary_integer::<I::UnsignedPair>(range)?
                }
                (true, 257..=K64) => {
                    self.input = self.parse_padding(self.input)?;
                    self.parse_non_negative_binary_integer::<I::UnsignedPair>(K64)?
                }
                (true, OVER_K64..) => {
                    let range_len_in_bytes =
                        num_integer::div_ceil(crate::num::log2(range), 8) as i128;
                    let length: u32 = self.parse_non_negative_binary_integer(range_len_in_bytes)?;
                    self.input = self.parse_padding(self.input)?;
                    let range = length
                        .checked_add(1)
                        .ok_or_else(|| {
                            DecodeError::exceeds_max_length(u32::MAX.into(), self.codec())
                        })?
                        .checked_mul(8)
                        .ok_or_else(|| {
                            DecodeError::exceeds_max_length(u32::MAX.into(), self.codec())
                        })?;
                    self.parse_non_negative_binary_integer::<I::UnsignedPair>(
                        crate::bits::range_from_len(range),
                    )?
                }
                (_, _) => self.parse_non_negative_binary_integer::<I::UnsignedPair>(range)?,
            }
        } else {
            let bytes = &self.decode_octets()?;
            let number = value_constraint.constraint.as_start().map_or_else(
                || I::try_from_signed_bytes(bytes.as_raw_slice(), self.codec()),
                |_| I::try_from_unsigned_bytes(bytes.as_raw_slice(), self.codec()),
            )?;

            return minimum
                .checked_add(&number)
                .ok_or_else(|| DecodeError::integer_overflow(I::WIDTH, self.codec()));
        };
        Ok(minimum.wrapping_unsigned_add(number))
    }

    fn parse_extension_header(&mut self) -> Result<bool> {
        match self.extensions_present {
            Some(Some(_)) => return Ok(true),
            Some(None) => (),
            None => return Ok(false),
        }

        // The length bitfield has a lower bound of `1..`
        let extensions_length = self.parse_normally_small_integer::<usize>()? + 1;
        let (input, bitfield) = nom::bytes::streaming::take(extensions_length)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = input;

        let extensions_present: VecDeque<_> = self
            .extension_fields
            .as_ref()
            .unwrap()
            .iter()
            .zip(bitfield.iter().map(|b| *b))
            .collect();

        for (field, is_present) in &extensions_present {
            if field.is_not_optional_or_default() && !is_present {
                return Err(DecodeError::required_extension_not_present(
                    field.tag,
                    self.codec(),
                ));
            }
        }

        self.extensions_present = Some(Some(extensions_present));

        Ok(true)
    }

    fn check_recursion_depth(&self) -> Result<()> {
        if self.options.remaining_depth == 0 {
            return Err(DecodeError::from_kind(
                DecodeErrorKind::ExceedsMaxParseDepth,
                self.codec(),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn parse_fixed_width_string<ALPHABET: StaticPermittedAlphabet>(
        &mut self,
        constraints: Constraints,
    ) -> Result<ALPHABET> {
        use crate::types::constraints::Bounded;

        let mut bit_string = types::BitString::default();
        let char_width = constraints
            .permitted_alphabet()
            .map_or(ALPHABET::character_width() as usize, |alphabet| {
                crate::num::log2(alphabet.constraint.len() as i128) as usize
            });

        let char_width = if self.options.aligned && !char_width.is_power_of_two() {
            char_width.next_power_of_two()
        } else {
            char_width
        };

        let is_large_string = if let Some(size) = constraints.size() {
            match *size.constraint {
                Bounded::Range {
                    start: Some(_),
                    end: Some(_),
                } if size
                    .constraint
                    .range()
                    .unwrap()
                    .checked_mul(char_width)
                    .ok_or_else(|| {
                        DecodeError::exceeds_max_length(usize::MAX.into(), self.codec())
                    })?
                    > 16 =>
                {
                    true
                }
                Bounded::Single(max)
                    if max.checked_mul(char_width).ok_or_else(|| {
                        DecodeError::exceeds_max_length(usize::MAX.into(), self.codec())
                    })? > 16 =>
                {
                    self.input = self.parse_padding(self.input)?;
                    true
                }
                Bounded::Range {
                    start: None,
                    end: Some(max),
                } if max.checked_mul(char_width).ok_or_else(|| {
                    DecodeError::exceeds_max_length(usize::MAX.into(), self.codec())
                })? > 16 =>
                {
                    self.input = self.parse_padding(self.input)?;
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        let mut total_length = 0;
        let codec = self.codec();
        self.decode_extensible_string(&constraints, is_large_string, |input, length| {
            total_length += length;
            if constraints
                .permitted_alphabet()
                .is_some_and(|alphabet| alphabet.constraint.len() == 1)
            {
                return Ok(input);
            }

            let (input, part) = nom::bytes::streaming::take(length * char_width)(input)
                .map_err(|e| DecodeError::map_nom_err(e, codec))?;
            bit_string.extend(&*part);
            Ok(input)
        })?;

        match (
            constraints.permitted_alphabet(),
            should_be_indexed(
                ALPHABET::CHARACTER_SET_WIDTH as u32,
                ALPHABET::CHARACTER_SET,
            ),
            constraints.permitted_alphabet().map(|alphabet| {
                ALPHABET::CHARACTER_SET_WIDTH
                    > if self.options.aligned {
                        {
                            let alphabet_width =
                                crate::num::log2(alphabet.constraint.len() as i128);
                            if alphabet_width.is_power_of_two() {
                                alphabet_width
                            } else {
                                alphabet_width.next_power_of_two()
                            }
                        }
                    } else {
                        crate::num::log2(alphabet.constraint.len() as i128)
                    } as usize
            }),
        ) {
            (Some(alphabet), true, _) | (Some(alphabet), _, Some(true)) => {
                if alphabet.constraint.len() == 1 {
                    let mut string = ALPHABET::default();
                    for _ in 0..total_length {
                        string.push_char(alphabet.constraint[0]);
                    }
                    Ok(string)
                } else {
                    let map = alphabet
                        .constraint
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(i, e)| (i as u32, e))
                        .collect();
                    ALPHABET::try_from_permitted_alphabet(bit_string, Some(&map))
                        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
                }
            }
            (None, true, _) => ALPHABET::try_from_permitted_alphabet(bit_string, None)
                .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec())),
            (None, false, _) if !self.options.aligned => {
                ALPHABET::try_from_permitted_alphabet(bit_string, None)
                    .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
            }
            _ => ALPHABET::try_from_bits(
                bit_string,
                if self.options.aligned {
                    {
                        if ALPHABET::CHARACTER_SET_WIDTH.is_power_of_two() {
                            ALPHABET::CHARACTER_SET_WIDTH
                        } else {
                            ALPHABET::CHARACTER_SET_WIDTH.next_power_of_two()
                        }
                    }
                } else {
                    ALPHABET::CHARACTER_SET_WIDTH
                },
            )
            .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec())),
        }
    }
}
impl<'input, const RFC: usize, const EFC: usize> crate::Decoder for Decoder<'input, RFC, EFC> {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Decoder<'input, R, E>;

    fn codec(&self) -> crate::Codec {
        Self::codec(self)
    }
    fn decode_any(&mut self, _tag: Tag) -> Result<types::Any> {
        let mut octet_string = types::BitString::default();
        let codec = self.codec();

        self.decode_extensible_container(Constraints::default(), |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * 8)(input)
                .map_err(|e| DecodeError::map_nom_err(e, codec))?;
            octet_string.extend(&*part);
            Ok(input)
        })?;

        Ok(types::Any::new(octet_string.as_raw_slice().to_vec()))
    }

    fn decode_bool(&mut self, _: Tag) -> Result<bool> {
        self.parse_one_bit()
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _: Tag) -> Result<E> {
        let extensible = E::EXTENDED_VARIANTS
            .is_some()
            .then(|| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();

        if extensible {
            let index: usize = self.parse_normally_small_integer()?;
            E::from_extended_enumeration_index(index)
                .ok_or_else(|| DecodeError::enumeration_index_not_found(index, true, self.codec()))
        } else {
            let index = self.parse_non_negative_binary_integer::<usize>(E::variance() as i128)?;
            E::from_enumeration_index(index)
                .ok_or_else(|| DecodeError::enumeration_index_not_found(index, false, self.codec()))
        }
    }

    fn decode_integer<I: types::IntegerType>(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<I> {
        self.parse_integer::<I>(constraints)
    }

    fn decode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: Constraints,
    ) -> Result<R, Self::Error> {
        Err(DecodeError::real_not_supported(self.codec()))
    }

    fn decode_octet_string<'b, T: From<&'b [u8]> + From<Vec<u8>>>(
        &'b mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<T> {
        let mut octet_string = Vec::new();
        let codec = self.codec();

        self.decode_extensible_container(constraints, |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * 8)(input)
                .map_err(|e| DecodeError::map_nom_err(e, codec))?;

            let mut bytes = part.to_bitvec();
            bytes.force_align();
            octet_string.extend_from_slice(bytes.as_raw_slice());
            Ok(input)
        })?;
        Ok(T::from(octet_string))
    }

    fn decode_null(&mut self, _: Tag) -> Result<()> {
        Ok(())
    }

    fn decode_object_identifier(&mut self, _: Tag) -> Result<crate::types::ObjectIdentifier> {
        let octets = self.decode_octets()?.into_vec();
        let decoder = crate::ber::de::Decoder::new(&octets, crate::ber::de::DecoderOptions::ber());
        decoder.decode_object_identifier_from_bytes(&octets)
    }

    fn decode_bit_string(&mut self, _: Tag, constraints: Constraints) -> Result<types::BitString> {
        let mut bit_string = types::BitString::default();
        let codec = self.codec();

        self.decode_extensible_container(constraints, |input, length| {
            let (input, part) = nom::bytes::streaming::take(length)(input)
                .map_err(|e| DecodeError::map_nom_err(e, codec))?;
            bit_string.extend(&*part);
            Ok(input)
        })?;

        Ok(bit_string)
    }

    fn decode_visible_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<types::VisibleString> {
        self.parse_fixed_width_string(constraints)
    }

    fn decode_ia5_string(&mut self, _: Tag, constraints: Constraints) -> Result<types::Ia5String> {
        self.parse_fixed_width_string(constraints)
    }

    fn decode_printable_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<types::PrintableString> {
        self.parse_fixed_width_string(constraints)
    }

    fn decode_numeric_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<types::NumericString> {
        self.parse_fixed_width_string(constraints)
    }

    fn decode_teletex_string(
        &mut self,
        _: Tag,
        _constraints: Constraints,
    ) -> Result<types::TeletexString> {
        todo!()
    }

    fn decode_bmp_string(&mut self, _: Tag, _constraints: Constraints) -> Result<types::BmpString> {
        todo!()
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::Utf8String> {
        self.decode_octet_string(tag, constraints)
            .and_then(|bytes| {
                alloc::string::String::from_utf8(bytes).map_err(|e| {
                    DecodeError::string_conversion_failed(
                        Tag::UTF8_STRING,
                        e.to_string(),
                        self.codec(),
                    )
                })
            })
    }

    fn decode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GeneralString> {
        <types::GeneralString>::try_from(self.decode_octet_string::<Vec<u8>>(tag, constraints)?)
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GENERAL_STRING,
                    e.to_string(),
                    self.codec(),
                )
            })
    }

    fn decode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GraphicString> {
        <types::GraphicString>::try_from(self.decode_octet_string::<Vec<u8>>(tag, constraints)?)
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GRAPHIC_STRING,
                    e.to_string(),
                    self.codec(),
                )
            })
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        let bytes = self.decode_octet_string::<Cow<[u8]>>(tag, Constraints::default())?;

        crate::ber::decode(&bytes)
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        let bytes = self.decode_octet_string::<Cow<[u8]>>(tag, Constraints::default())?;

        crate::ber::decode(&bytes)
    }

    fn decode_date(&mut self, tag: Tag) -> core::result::Result<types::Date, Self::Error> {
        let bytes = self.decode_octet_string::<Cow<[u8]>>(tag, Constraints::default())?;

        crate::ber::decode(&bytes)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        self.check_recursion_depth()?;
        let mut sequence_of = Vec::new();
        let mut options = self.options;
        options.remaining_depth = options.remaining_depth.saturating_sub(1);
        self.decode_extensible_container(constraints, |mut input, length| {
            sequence_of.append(
                &mut (0..length)
                    .map(|_| {
                        let mut decoder = Self::new(input.0, options);
                        let value = D::decode(&mut decoder)?;
                        input = decoder.input;
                        Ok(value)
                    })
                    .collect::<Result<Vec<_>>>()?,
            );

            Ok(input)
        })?;

        Ok(sequence_of)
    }

    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::SetOf<D>, Self::Error> {
        self.decode_sequence_of(tag, constraints)
            .map(|seq| SetOf::from_vec(seq))
    }

    fn decode_sequence<const RC: usize, const EC: usize, D, DF, F>(
        &mut self,
        _: Tag,
        _: Option<DF>,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: crate::types::Constructed<RC, EC>,
        DF: FnOnce() -> D,
        F: FnOnce(&mut Self::AnyDecoder<RC, EC>) -> Result<D, Self::Error>,
    {
        self.check_recursion_depth()?;

        let is_extensible = D::IS_EXTENSIBLE
            .then(|| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();
        let bitmap = self.parse_optional_and_default_field_bitmap(&D::FIELDS)?;

        let value = {
            let mut sequence_decoder = Decoder::new(self.input(), self.options);
            sequence_decoder.options.remaining_depth =
                sequence_decoder.options.remaining_depth.saturating_sub(1);
            sequence_decoder.extension_fields = D::EXTENDED_FIELDS;
            sequence_decoder.extensions_present = is_extensible.then_some(None);
            sequence_decoder.fields = D::FIELDS
                .optional_and_default_fields()
                .zip(bitmap.into_iter().map(|b| *b))
                .collect();
            let value = (decode_fn)(&mut sequence_decoder)?;

            self.input = sequence_decoder.input;
            value
        };

        Ok(value)
    }
    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        tag: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_tag(tag)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        // Whether we have a choice here
        if D::IS_CHOICE {
            D::decode(self)
        } else {
            D::decode_with_tag(self, tag)
        }
    }

    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        _: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed<RC, EC>,
        FIELDS: Decode,
        D: Fn(&mut Self::AnyDecoder<RC, EC>, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        self.check_recursion_depth()?;

        let is_extensible = SET::IS_EXTENSIBLE
            .then(|| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();

        let bitmap = self.parse_optional_and_default_field_bitmap::<RC>(&SET::FIELDS)?;
        let field_map = SET::FIELDS
            .optional_and_default_fields()
            .zip(bitmap.into_iter().map(|b| *b))
            .collect::<alloc::collections::BTreeMap<_, _>>();

        let fields = {
            let mut fields = Vec::new();
            let mut set_decoder = Decoder::new(self.input(), self.options);
            set_decoder.options.remaining_depth =
                set_decoder.options.remaining_depth.saturating_sub(1);
            set_decoder.extension_fields = SET::EXTENDED_FIELDS;
            set_decoder.extensions_present = is_extensible.then_some(None);
            set_decoder.fields = SET::FIELDS
                .optional_and_default_fields()
                .zip(bitmap.into_iter().map(|b| *b))
                .collect();

            let mut field_indices = SET::FIELDS.iter().enumerate().collect::<Vec<_>>();
            field_indices.sort_by(|(_, a), (_, b)| {
                a.tag_tree.smallest_tag().cmp(&b.tag_tree.smallest_tag())
            });
            for (indice, field) in field_indices.into_iter() {
                match field_map.get(&field).copied() {
                    Some(true) | None => {
                        fields.push((decode_fn)(&mut set_decoder, indice, field.tag)?)
                    }
                    Some(false) => {}
                }
            }

            for (indice, field) in SET::EXTENDED_FIELDS
                .iter()
                .flat_map(|fields| fields.iter())
                .enumerate()
            {
                fields.push((decode_fn)(
                    &mut set_decoder,
                    indice + SET::FIELDS.len(),
                    field.tag,
                )?)
            }

            self.input = set_decoder.input;
            fields
        };

        (field_fn)(fields)
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_tag(D::TAG)
    }

    /// Decode an the optional value in a `SEQUENCE` or `SET` with `tag`.
    /// Passing the correct tag is required even when used with codecs where
    /// the tag is not present.
    fn decode_optional_with_tag<D: Decode>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error> {
        let is_present = self.require_field(tag)?;

        if is_present {
            D::decode_with_tag(self, tag).map(Some)
        } else {
            Ok(None)
        }
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        let is_present = self.require_field(D::TAG)?;

        if is_present {
            D::decode_with_constraints(self, constraints).map(Some)
        } else {
            Ok(None)
        }
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        let is_present = self.require_field(tag)?;

        if is_present {
            D::decode_with_tag_and_constraints(self, tag, constraints).map(Some)
        } else {
            Ok(None)
        }
    }

    fn decode_choice<D>(&mut self, constraints: Constraints) -> Result<D, Self::Error>
    where
        D: crate::types::DecodeChoice,
    {
        self.check_recursion_depth()?;
        let is_extensible = self.parse_extensible_bit(&constraints)?;
        let variants = crate::types::variants::Variants::from_static(if is_extensible {
            D::EXTENDED_VARIANTS.unwrap_or(&[])
        } else {
            D::VARIANTS
        });

        let index = if variants.len() != 1 || is_extensible {
            if is_extensible {
                self.parse_normally_small_integer::<usize>()
                    .map_err(|error| {
                        DecodeError::choice_index_exceeds_platform_width(
                            usize::BITS,
                            error,
                            self.codec(),
                        )
                    })?
            } else {
                debug_assert!(!variants.is_empty());
                self.parse_integer(D::VARIANCE_CONSTRAINT)
                    .map_err(|error| {
                        DecodeError::choice_index_exceeds_platform_width(
                            usize::BITS,
                            error,
                            self.codec(),
                        )
                    })?
            }
        } else {
            0
        };

        let tag = variants.get(index).ok_or_else(|| {
            DecodeError::choice_index_not_found(index, variants.clone(), self.codec())
        })?;

        if is_extensible {
            let bytes = self.decode_octets()?;
            let mut decoder = Decoder::<0, 0>::new(&bytes, self.options);
            decoder.options.remaining_depth = decoder.options.remaining_depth.saturating_sub(1);
            D::from_tag(&mut decoder, *tag)
        } else {
            self.options.remaining_depth = self.options.remaining_depth.saturating_sub(1);
            let result = D::from_tag(self, *tag);
            self.options.remaining_depth = self.options.remaining_depth.saturating_add(1);
            result
        }
    }

    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: Decode + crate::types::Constructed<RC, EC>,
    >(
        &mut self,
        _tag: Tag,
    ) -> Result<Option<D>, Self::Error> {
        if !self.parse_extension_header()? {
            return Ok(None);
        }

        let extension_is_present = self
            .extension_is_present()?
            .map(|(_, b)| b)
            .unwrap_or_default();

        if !extension_is_present {
            return Ok(None);
        }

        let bytes = self.decode_octets()?;
        let mut decoder = Decoder::<RC, EC>::new(&bytes, self.options);

        D::decode(&mut decoder).map(Some)
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_tag_and_constraints::<D>(tag, constraints)
    }

    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        if !self.parse_extension_header()? {
            return Ok(None);
        }

        let extension_is_present = self
            .extension_is_present()?
            .map(|(_, b)| b)
            .unwrap_or_default();

        if !extension_is_present {
            return Ok(None);
        }

        let bytes = self.decode_octets()?;
        let mut decoder = Decoder::<0, 0>::new(&bytes, self.options);

        D::decode_with_constraints(&mut decoder, constraints).map(Some)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn bitvec() {
        use bitvec::prelude::*;
        assert_eq!(
            bitvec::bits![u8, Msb0;       0, 0, 0, 1, 1, 1, 0, 1]
                .to_bitvec()
                .into_vec(),
            vec![29]
        );
        let bv = bitvec::bits![u8, Msb0; 1, 1, 0, 0, 0, 1, 1, 1, 0, 1];
        let slice = &bv[2..];
        let mut aligned = slice.to_bitvec();
        // Indexing results to different internal memory layout than we would expect when converting to u8 vec
        // We need to unify the layout as continuos bits after indexing to get the correct result
        aligned.force_align();
        assert_eq!(aligned.into_vec(), vec![29]);
    }
}
