mod error;

use alloc::vec::Vec;
use bitvec::field::BitField;
use snafu::*;

use super::{FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    de::Error as _,
    types::{
        self,
        constraints::{self, Extensible},
        fields::{Field, Fields},
        strings::StaticPermittedAlphabet,
        Constraints, Tag,
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
    fields: alloc::collections::VecDeque<(Field, bool)>,
}

impl<'input> Decoder<'input> {
    pub fn new(input: &'input crate::types::BitStr, options: DecoderOptions) -> Self {
        Self {
            input: input.into(),
            options,
            fields: <_>::default(),
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
        input: InputSlice<'input>,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

        if mask[0] == false {
            let (input, length) =
                nom::bytes::streaming::take(7u8)(input).map(|(i, bs)| (i, bs.to_bitvec()))?;
            (decode_fn)(input, length.load_be::<usize>())
        } else {
            let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

            if mask[0] == false {
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
                }
                .into();

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

    pub fn decode_length(
        &mut self,
        input: InputSlice<'input>,
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
                (decode_fn)(input, size_constraint.start())
            } else {
                let (input, length) =
                    nom::bytes::streaming::take(super::log2(range as i128))(input)?;
                (decode_fn)(input, length.load_be::<usize>() + size_constraint.start())
            }
        } else {
            self.decode_unknown_length(input, decode_fn)
        }
    }

    fn parse_one_bit(&mut self) -> Result<bool> {
        let (input, boolean) = nom::bytes::streaming::take(1u8)(self.input)?;
        self.input = input;
        Ok(boolean[0] as bool)
    }

    fn parse_normally_small_integer(&mut self) -> Result<types::Integer> {
        let is_large = self.parse_one_bit()?;
        let constraints = if is_large {
            constraints::Value::new(constraints::Range::start_from(0)).into()
        } else {
            constraints::Value::new(constraints::Range::new(0, 63)).into()
        };

        self.parse_integer(Constraints::new(&[constraints]))
    }

    fn parse_integer(&mut self, constraints: Constraints) -> Result<types::Integer> {
        let extensible = self.parse_extensible_bit(&constraints)?;
        let value_constraint = constraints.value();

        let Some(value_constraint) = value_constraint else {
            let bytes = self.decode_octets()?.into_vec();
            return Ok(num_bigint::BigInt::from_signed_bytes_be(&bytes))
        };

        let number = if let Some(range) = value_constraint
            .constraint
            .range()
            .filter(|range| !extensible && *range < SIXTY_FOUR_K.into())
        {
            let bits = super::log2(range);
            let (input, data) = nom::bytes::streaming::take(bits)(self.input)?;
            self.input = input;
            num_bigint::BigUint::from_bytes_be(&data.to_bitvec().into_vec()).into()
        } else {
            let bytes = self.decode_octets()?.into_vec();
            value_constraint
                .constraint
                .as_start()
                .map(|_| num_bigint::BigUint::from_bytes_be(&bytes).into())
                .unwrap_or_else(|| num_bigint::BigInt::from_signed_bytes_be(&bytes))
        };

        Ok(value_constraint.constraint.start() + number)
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

        Ok(types::Any::new(octet_string.into_vec()))
    }

    fn decode_bool(&mut self, _: Tag) -> Result<bool> {
        self.parse_one_bit()
    }

    fn decode_enumerated(&mut self, tag: Tag, constraints: Constraints) -> Result<types::Integer> {
        self.decode_integer(tag, constraints)
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
        let mut bit_string = types::BitString::default();
        let char_width = constraints
            .permitted_alphabet()
            .map(|alphabet| crate::per::log2(alphabet.constraint.len() as i128) as usize)
            .unwrap_or(7);

        self.decode_extensible_container(constraints.clone(), |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * char_width)(input)?;
            bit_string.extend(&*part);
            Ok(input)
        })?;

        match constraints.permitted_alphabet() {
            Some(alphabet) => {
                let map = alphabet
                    .constraint
                    .into_iter()
                    .copied()
                    .enumerate()
                    .map(|(i, e)| (i as u32, e))
                    .collect();
                types::strings::try_from_permitted_alphabet::<types::VisibleString, _>(
                    &bit_string,
                    &map,
                    types::VisibleString::character_width(),
                )
                .map_err(Error::custom)
            }
            None => Ok(types::VisibleString::from_raw_bits(bit_string)),
        }
    }

    fn decode_ia5_string(&mut self, _: Tag, _constraints: Constraints) -> Result<types::Ia5String> {
        todo!()
    }

    fn decode_printable_string(
        &mut self,
        _: Tag,
        _constraints: Constraints,
    ) -> Result<types::PrintableString> {
        todo!()
    }

    fn decode_numeric_string(
        &mut self,
        _: Tag,
        _constraints: Constraints,
    ) -> Result<types::NumericString> {
        todo!()
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

    fn decode_sequence<D, F>(
        &mut self,
        _: Tag,
        constraints: Constraints,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        let is_extensible = self.parse_extensible_bit(&constraints)?;
        let bitmap = self.parse_optional_and_default_field_bitmap(&D::FIELDS)?;

        let value = {
            let mut sequence_decoder = Self::new(self.input(), self.options);
            sequence_decoder.fields = D::FIELDS
                .optional_and_default_fields()
                .zip(bitmap.into_iter().map(|b| *b))
                .collect();
            let value = (decode_fn)(&mut sequence_decoder)?;

            self.input = sequence_decoder.input;
            value
        };

        if is_extensible {
            todo!()
        }

        Ok(value)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _: Tag) -> Result<D> {
        D::decode(self)
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        _: Tag,
        constraints: Constraints,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let is_extensible = self.parse_extensible_bit(&constraints)?;
        let bitmap = self.parse_optional_and_default_field_bitmap(&SET::FIELDS)?;
        let field_map = SET::FIELDS
            .optional_and_default_fields()
            .zip(bitmap.into_iter().map(|b| *b))
            .collect::<alloc::collections::BTreeMap<_, _>>();

        let fields = {
            let mut fields = Vec::new();
            let mut set_decoder = Self::new(self.input(), self.options);
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

            self.input = set_decoder.input;
            fields
        };

        if is_extensible {
            todo!()
        }

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

    fn decode_choice<D, F>(
        &mut self,
        constraints: Constraints,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: Decode + crate::types::Choice,
        F: FnOnce(&mut Self, Tag) -> Result<D, Self::Error>,
    {
        let is_extensible = self.parse_extensible_bit(&constraints)?;
        let variants = crate::types::variants::Variants::from_static(
            is_extensible
                .then(|| D::EXTENDED_VARIANTS)
                .unwrap_or(D::VARIANTS),
        );

        let index = if variants.len() != 1 || is_extensible {
            usize::try_from(if is_extensible {
                self.parse_normally_small_integer()?
            } else {
                let variance = variants.len();
                let constraints =
                    constraints::Value::new(constraints::Range::new(0, variance as i128)).into();
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

        Ok((decode_fn)(self, *tag)?)
    }

    fn decode_extension_addition_group<D: Decode + crate::types::Constructed>(
        &mut self,
    ) -> Result<D, Self::Error> {
        todo!()
    }

    fn decode_extension_addition<D, F>(&mut self, _extension: F) -> Result<D, Self::Error>
    where
        D: Decode,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        todo!()
    }
}
