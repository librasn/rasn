mod error;

use alloc::{collections::VecDeque, vec::Vec};
use bitvec::field::BitField;
use snafu::*;

use super::{to_vec, FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    de::Error as _,
    types::{
        self,
        constraints::{self, Extensible},
        fields::{Field, Fields},
        strings::StaticPermittedAlphabet,
        Constraints, Enumerated, Tag,
    },
    Decode,
};

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub use error::Error;

type InputSlice<'input> = nom_bitvec::BSlice<'input, u8, bitvec::order::Msb0>;

#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    #[allow(unused)]
    aligned: bool,
}

impl DecoderOptions {
    pub fn aligned() -> Self {
        Self { aligned: true }
    }

    pub fn unaligned() -> Self {
        Self { aligned: false }
    }
}

pub struct Decoder<'input> {
    input: InputSlice<'input>,
    options: DecoderOptions,
    /// When the decoder contains fields, we check against optional or default
    /// fields to know the presence of those fields.
    fields: VecDeque<(Field, bool)>,
    extension_fields: Option<Fields>,
    extensions_present: Option<Option<VecDeque<(Field, bool)>>>,
}

impl<'input> Decoder<'input> {
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
    pub fn input(&self) -> &'input crate::types::BitStr {
        self.input.0
    }

    #[track_caller]
    fn require_field(&mut self, tag: Tag) -> Result<bool> {
        if self
            .fields
            .front()
            .map(|field| field.0.tag_tree.smallest_tag() == tag)
            .unwrap_or_default()
        {
            Ok(self.fields.pop_front().unwrap().1)
        } else {
            Err(Error::custom(alloc::format!(
                "expected class: {}, value: {} in sequence or set",
                tag.class,
                tag.value
            )))
        }
    }

    fn parse_extensible_bit(&mut self, constraints: &Constraints) -> Result<bool> {
        constraints
            .extensible()
            .then(|| self.parse_one_bit())
            .transpose()
            .map(|opt| opt.unwrap_or_default())
    }

    fn extension_is_present(&mut self) -> Result<Option<(Field, bool)>> {
        Ok(self
            .extensions_present
            .as_mut()
            .ok_or_else(Error::type_not_extensible)?
            .as_mut()
            .ok_or_else(Error::type_not_extensible)?
            .pop_front())
    }

    fn parse_padding(&self, input: InputSlice<'input>) -> Result<InputSlice<'input>> {
        if !self.options.aligned {
            Ok(input)
        } else {
            Self::force_parse_padding(input)
        }
    }

    fn force_parse_padding(input: InputSlice<'input>) -> Result<InputSlice<'input>> {
        if input.len() % 8 == 0 {
            Ok(input)
        } else {
            let (input, _) = nom::bytes::streaming::take(input.len() % 8)(input)?;
            Ok(input)
        }
    }

    fn parse_optional_and_default_field_bitmap(
        &mut self,
        fields: &Fields,
    ) -> Result<InputSlice<'input>> {
        let (input, bitset) = nom::bytes::streaming::take(
            fields.number_of_optional_and_default_fields(),
        )(self.input)?;
        self.input = input;
        Ok(bitset)
    }

    fn decode_extensible_string(
        &mut self,
        constraints: Constraints,
        is_large_string: bool,
        mut decode_fn: impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<()> {
        let extensible_is_present = self.parse_extensible_bit(&constraints)?;
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

        let input = self.decode_length(self.input, <_>::default(), &mut |input, length| {
            let (input, data) = nom::bytes::streaming::take(length * 8)(input)?;
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
        let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

        if !mask[0] {
            let (input, length) =
                nom::bytes::streaming::take(7u8)(input).map(|(i, bs)| (i, bs.to_bitvec()))?;
            (decode_fn)(input, length.load_be::<usize>())
        } else {
            let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

            if !mask[0] {
                let (input, length) =
                    nom::bytes::streaming::take(14u8)(input).map(|(i, bs)| (i, bs.to_bitvec()))?;
                (decode_fn)(input, length.load_be::<usize>())
            } else {
                let (input, mask) = nom::bytes::streaming::take(6u8)(input)?;
                let length: usize = match mask.load_be::<u8>() {
                    1 => SIXTEEN_K.into(),
                    2 => THIRTY_TWO_K.into(),
                    3 => FOURTY_EIGHT_K.into(),
                    4 => SIXTY_FOUR_K as usize,
                    _ => {
                        return Err(error::Kind::Parser {
                            msg: "Invalid length fragment".into(),
                        }
                        .into())
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

    pub fn decode_string_length(
        &mut self,
        mut input: InputSlice<'input>,
        constraints: Option<&Extensible<constraints::Size>>,
        is_large_string: bool,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        let Some(constraints) = constraints else {
            return self.decode_unknown_length(input, decode_fn)
        };

        let size_constraint = constraints.constraint;
        if let Some(range) = size_constraint
            .range()
            .filter(|range| *range <= u16::MAX.into())
        {
            if range == 0 {
                Ok(input)
            } else if range == 1 {
                (decode_fn)(input, size_constraint.minimum())
            } else {
                let range = if self.options.aligned && range > 256 {
                    input = self.parse_padding(input)?;
                    let range = super::log2(range as i128);
                    super::range_from_bits(
                        range
                            .is_power_of_two()
                            .then_some(range)
                            .unwrap_or_else(|| range.next_power_of_two()),
                    )
                } else {
                    range as i128
                };

                let (mut input, length) =
                    nom::bytes::streaming::take(super::log2(range as i128))(input)?;
                if is_large_string {
                    input = self.parse_padding(input)?;
                }
                (decode_fn)(input, length.load_be::<usize>() + size_constraint.minimum())
            }
        } else {
            self.decode_unknown_length(input, decode_fn)
        }
    }

    pub fn decode_length(
        &mut self,
        mut input: InputSlice<'input>,
        constraints: Option<&Extensible<constraints::Size>>,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        let Some(constraints) = constraints else {
            return self.decode_unknown_length(input, decode_fn)
        };

        let size_constraint = constraints.constraint;
        if let Some(range) = size_constraint
            .range()
            .filter(|range| *range <= u16::MAX.into())
        {
            if range == 0 {
                Ok(input)
            } else if range == 1 {
                (decode_fn)(input, size_constraint.minimum())
            } else {
                let range = if self.options.aligned && range > 256 {
                    input = self.parse_padding(input)?;
                    let range = super::log2(range as i128);
                    super::range_from_bits(
                        range
                            .is_power_of_two()
                            .then_some(range)
                            .unwrap_or_else(|| range.next_power_of_two()),
                    )
                } else {
                    range as i128
                };

                let (mut input, length) =
                    nom::bytes::streaming::take(super::log2(range as i128))(input)?;
                input = self.parse_padding(input)?;
                (decode_fn)(input, length.load_be::<usize>() + size_constraint.minimum())
            }
        } else {
            self.decode_unknown_length(input, decode_fn)
        }
    }

    fn parse_one_bit(&mut self) -> Result<bool> {
        let (input, boolean) = nom::bytes::streaming::take(1u8)(self.input)?;
        self.input = input;
        Ok(boolean[0])
    }

    fn parse_normally_small_integer(&mut self) -> Result<types::Integer> {
        let is_large = self.parse_one_bit()?;
        let constraints = if is_large {
            constraints::Value::new(constraints::Bounded::start_from(0)).into()
        } else {
            constraints::Value::new(constraints::Bounded::new(0, 63)).into()
        };

        self.parse_integer(Constraints::new(&[constraints]))
    }

    fn parse_non_negative_binary_integer(&mut self, range: i128) -> Result<types::Integer> {
        let bits = super::log2(range);
        let (input, data) = nom::bytes::streaming::take(bits)(self.input)?;
        self.input = input;
        let data = if data.len() < 8 {
            let mut buffer = types::BitString::repeat(false, 8 - data.len());
            buffer.extend_from_bitslice(&data);
            buffer
        } else {
            data.to_bitvec()
        };

        Ok(num_bigint::BigUint::from_bytes_be(&to_vec(&data)).into())
    }

    fn parse_integer(&mut self, constraints: Constraints) -> Result<types::Integer> {
        let extensible = self.parse_extensible_bit(&constraints)?;
        let value_constraint = constraints.value();

        let Some(value_constraint) = value_constraint.filter(|_| !extensible) else {
            let bytes = to_vec(&self.decode_octets()?);
            return Ok(num_bigint::BigInt::from_signed_bytes_be(&bytes))
        };

        const K64: i128 = SIXTY_FOUR_K as i128;
        const OVER_K64: i128 = K64 + 1;

        let number = if let Some(range) = value_constraint.constraint.range() {
            match (self.options.aligned, range) {
                (_, 0) => return Ok(value_constraint.constraint.minimum().into()),
                (true, 256) => {
                    self.input = self.parse_padding(self.input)?;
                    self.parse_non_negative_binary_integer(range)?
                }
                (true, 257..=K64) => {
                    self.input = self.parse_padding(self.input)?;
                    self.parse_non_negative_binary_integer(K64)?
                }
                (true, OVER_K64..) => {
                    let range_len_in_bytes = num_integer::div_ceil(
                        super::log2(i128::try_from(range).map_err(Error::custom)?),
                        8,
                    ) as i128;
                    let length: u32 = self
                        .parse_non_negative_binary_integer(range_len_in_bytes)?
                        .try_into()
                        .map_err(Error::custom)?;
                    self.input = self.parse_padding(self.input)?;
                    self.parse_non_negative_binary_integer(super::range_from_bits(
                        (length + 1) * 8,
                    ))?
                }
                (false, OVER_K64..) => {
                    let bytes = to_vec(&self.decode_octets()?);
                    value_constraint
                        .constraint
                        .as_start()
                        .map(|_| num_bigint::BigUint::from_bytes_be(&bytes).into())
                        .unwrap_or_else(|| num_bigint::BigInt::from_signed_bytes_be(&bytes))
                }
                (_, _) => self.parse_non_negative_binary_integer(range)?,
            }
        } else {
            let bytes = to_vec(&self.decode_octets()?);
            value_constraint
                .constraint
                .as_start()
                .map(|_| num_bigint::BigUint::from_bytes_be(&bytes).into())
                .unwrap_or_else(|| num_bigint::BigInt::from_signed_bytes_be(&bytes))
        };

        Ok(value_constraint.constraint.minimum() + number)
    }

    fn parse_extension_header(&mut self) -> Result<bool> {
        match self.extensions_present {
            Some(Some(_)) => return Ok(true),
            Some(None) => (),
            None => return Ok(false),
        }

        // The length bitfield has a lower bound of `1..`
        let extensions_length = self.parse_normally_small_integer()? + 1;
        let (input, bitfield) = nom::bytes::streaming::take(
            usize::try_from(extensions_length).map_err(Error::custom)?,
        )(self.input)?;
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
                return Err(Error::required_extension_not_present(field.tag));
            }
        }

        self.extensions_present = Some(Some(extensions_present));

        Ok(true)
    }

    fn parse_fixed_width_string<ALPHABET: StaticPermittedAlphabet>(
        &mut self,
        constraints: Constraints,
    ) -> Result<ALPHABET> {
        use crate::types::constraints::Bounded;

        let mut bit_string = types::BitString::default();
        let char_width = constraints
            .permitted_alphabet()
            .map(|alphabet| crate::per::log2(alphabet.constraint.len() as i128) as usize)
            .unwrap_or(ALPHABET::character_width() as usize);

        let char_width = (self.options.aligned && !char_width.is_power_of_two())
            .then(|| char_width.next_power_of_two())
            .unwrap_or(char_width);

        let is_large_string = if let Some(size) = constraints.size() {
            match *size.constraint {
                Bounded::Range {
                    start: Some(_),
                    end: Some(_),
                } if size.constraint.range().unwrap() * char_width as usize > 16 as usize => true,
                Bounded::Single(max) if max * char_width as usize > 16 as usize => {
                    self.input = self.parse_padding(self.input)?;
                    true
                }
                Bounded::Range {
                    start: None,
                    end: Some(max),
                } if max * char_width as usize > 16 as usize => {
                    self.input = self.parse_padding(self.input)?;
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        let mut total_length = 0;
        self.decode_extensible_string(constraints.clone(), is_large_string, |input, length| {
            total_length += length;
            if constraints
                .permitted_alphabet()
                .map_or(false, |alphabet| alphabet.constraint.len() == 1)
            {
                return Ok(input);
            }

            let (input, part) = nom::bytes::streaming::take(length * char_width)(input)?;
            bit_string.extend(&*part);
            Ok(input)
        })?;

        match constraints.permitted_alphabet() {
            Some(alphabet)
                if ALPHABET::CHARACTER_WIDTH
                    > self
                        .options
                        .aligned
                        .then(|| {
                            let alphabet_width =
                                crate::per::log2(alphabet.constraint.len() as i128);
                            alphabet_width
                                .is_power_of_two()
                                .then_some(alphabet_width)
                                .unwrap_or_else(|| alphabet_width.next_power_of_two())
                        })
                        .unwrap_or(crate::per::log2(alphabet.constraint.len() as i128)) =>
            {
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
                    ALPHABET::try_from_permitted_alphabet(&bit_string, Some(&map))
                        .map_err(Error::custom)
                }
            }
            None if !self.options.aligned => {
                ALPHABET::try_from_permitted_alphabet(&bit_string, None).map_err(Error::custom)
            }
            _ => ALPHABET::try_from_bits(
                bit_string,
                self.options
                    .aligned
                    .then(|| {
                        ALPHABET::CHARACTER_WIDTH
                            .is_power_of_two()
                            .then_some(ALPHABET::CHARACTER_WIDTH)
                            .unwrap_or_else(|| ALPHABET::CHARACTER_WIDTH.next_power_of_two())
                    })
                    .unwrap_or(ALPHABET::CHARACTER_WIDTH) as usize,
            )
            .map_err(Error::custom),
        }
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = Error;

    fn decode_any(&mut self) -> Result<types::Any> {
        let mut octet_string = types::BitString::default();

        self.decode_extensible_container(<_>::default(), |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * 8)(input)?;
            octet_string.extend(&*part);
            Ok(input)
        })?;

        Ok(types::Any::new(to_vec(&octet_string)))
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
            let index: usize = self
                .parse_normally_small_integer()?
                .try_into()
                .map_err(Error::custom)?;
            E::from_extended_enumeration_index(index)
                .ok_or_else(|| Error::custom(format!("Extended index {} not found", index)))
        } else {
            let index = self
                .parse_non_negative_binary_integer(E::variance() as i128)?
                .try_into()
                .map_err(Error::custom)?;
            E::from_enumeration_index(index)
                .ok_or_else(|| Error::custom(format!("Index {} not found", index)))
        }
    }

    fn decode_integer(&mut self, _: Tag, constraints: Constraints) -> Result<types::Integer> {
        self.parse_integer(constraints)
    }

    fn decode_octet_string(&mut self, _: Tag, constraints: Constraints) -> Result<Vec<u8>> {
        let mut octet_string = types::BitString::default();

        self.decode_extensible_container(constraints, |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * 8)(input)?;
            octet_string.extend(&*part);
            Ok(input)
        })?;

        Ok(octet_string.into_vec())
    }

    fn decode_null(&mut self, _: Tag) -> Result<()> {
        Ok(())
    }

    fn decode_object_identifier(&mut self, _: Tag) -> Result<crate::types::ObjectIdentifier> {
        let octets = self.decode_octets()?.into_vec();

        crate::ber::decode(&octets)
            .context(error::BerSnafu)
            .map_err(From::from)
    }

    fn decode_bit_string(&mut self, _: Tag, constraints: Constraints) -> Result<types::BitString> {
        let mut bit_string = types::BitString::default();

        self.decode_extensible_container(constraints, |input, length| {
            let (input, part) = nom::bytes::streaming::take(length)(input)?;
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
            .and_then(|bytes| String::from_utf8(bytes).map_err(Error::custom))
    }

    fn decode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GeneralString> {
        <_>::try_from(self.decode_octet_string(tag, constraints)?).map_err(Error::custom)
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        let bytes = self.decode_octet_string(tag, <_>::default())?;

        crate::ber::decode(&bytes)
            .context(error::BerSnafu)
            .map_err(From::from)
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        let bytes = self.decode_octet_string(tag, <_>::default())?;

        crate::ber::decode(&bytes)
            .context(error::BerSnafu)
            .map_err(From::from)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        let mut sequence_of = Vec::new();
        let options = self.options;
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

    fn decode_set_of<D: Decode + Ord>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::SetOf<D>, Self::Error> {
        self.decode_sequence_of(tag, constraints)
            .map(|seq| seq.into_iter().collect())
    }

    fn decode_sequence<D, F>(&mut self, _: Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        D: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        let is_extensible = D::EXTENDED_FIELDS
            .is_some()
            .then(|| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();
        let bitmap = self.parse_optional_and_default_field_bitmap(&D::FIELDS)?;

        let value = {
            let mut sequence_decoder = Self::new(self.input(), self.options);
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

    fn decode_explicit_prefix<D: Decode>(&mut self, _: Tag) -> Result<D> {
        D::decode(self)
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        _: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let is_extensible = SET::EXTENDED_FIELDS
            .is_some()
            .then(|| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();

        let bitmap = self.parse_optional_and_default_field_bitmap(&SET::FIELDS)?;
        let field_map = SET::FIELDS
            .optional_and_default_fields()
            .zip(bitmap.into_iter().map(|b| *b))
            .collect::<alloc::collections::BTreeMap<_, _>>();

        let fields = {
            let mut fields = Vec::new();
            let mut set_decoder = Self::new(self.input(), self.options);
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
        let is_extensible = self.parse_extensible_bit(&constraints)?;
        let variants = crate::types::variants::Variants::from_static(if is_extensible {
            D::EXTENDED_VARIANTS
        } else {
            D::VARIANTS
        });

        let index = if variants.len() != 1 || is_extensible {
            usize::try_from(if is_extensible {
                self.parse_normally_small_integer()?
            } else {
                let variance = variants.len();
                let constraints =
                    constraints::Value::new(constraints::Bounded::new(0, variance as i128)).into();
                self.parse_integer(Constraints::new(&[constraints]))?
            })
            .map_err(|error| {
                Error::choice_index_exceeds_platform_width(
                    usize::BITS,
                    error.into_original().bits(),
                )
            })?
        } else {
            0
        };

        let tag = variants
            .get(index)
            .ok_or_else(|| Error::choice_index_not_found(index, variants.clone()))?;

        if is_extensible {
            let bytes = self.decode_octets()?;
            let mut decoder = Decoder::new(&bytes, self.options);
            D::from_tag(&mut decoder, *tag)
        } else {
            D::from_tag(self, *tag)
        }
    }

    fn decode_extension_addition_group<D: Decode + crate::types::Constructed>(
        &mut self,
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
        let mut decoder = Decoder::new(&bytes, self.options);

        D::decode(&mut decoder).map(Some)
    }

    fn decode_extension_addition<D>(&mut self) -> Result<Option<D>, Self::Error>
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
        let mut decoder = Decoder::new(&bytes, self.options);

        D::decode(&mut decoder).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec() {
        use bitvec::prelude::*;
        assert_eq!(
            to_vec(&bitvec::bits![u8, Msb0;       0, 0, 0, 1, 1, 1, 0, 1]),
            vec![29]
        );
        assert_eq!(
            to_vec(&bitvec::bits![u8, Msb0; 1, 1, 0, 0, 0, 1, 1, 1, 0, 1][2..]),
            vec![29]
        );
    }
}
