mod error;

use alloc::vec::Vec;
use bitvec::field::BitField;

use super::{FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    de::Error as _,
    types::{self, constraints, Constraints, Tag},
    Decode,
};

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub use error::Error;

type InputSlice<'input> = nom_bitvec::BSlice<'input, u8, bitvec::order::Msb0>;

#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
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
}

impl<'input> Decoder<'input> {
    pub fn new(input: &'input crate::types::BitStr, options: DecoderOptions) -> Self {
        Self {
            input: input.into(),
            options,
        }
    }

    /// Returns the remaining input, if any.
    pub fn input(&self) -> &'input crate::types::BitStr {
        self.input.0
    }

    fn decode_extensible_container(
        &mut self,
        constraints: Constraints,
        mut decode_fn: impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<()> {
        let extensible_is_present = constraints
            .extensible()
            .map(|_| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();

        let constraints = if constraints.size().range() == Some(0) {
            // NO-OP
            <_>::default()
        } else {
            let constraints = extensible_is_present
                .then(Default::default)
                .unwrap_or_else(|| constraints.size());

            let options = self.options;
            let input = self.decode_length(self.input, constraints, &mut decode_fn)?;
            self.input = input;
            constraints
        };

        if let Some(min) = constraints.as_min().filter(|min| **min != 0) {
            (decode_fn)(self.input, *min)?;
        }

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

    pub fn decode_length(
        &mut self,
        input: InputSlice<'input>,
        constraints: constraints::Range<usize>,
        decode_fn: &mut impl FnMut(InputSlice<'input>, usize) -> Result<InputSlice<'input>>,
    ) -> Result<InputSlice<'input>> {
        Ok(
            if let Some(range) = constraints.range().filter(|range| *range < SIXTY_FOUR_K) {
                let (input, length) = nom::bytes::streaming::take(range)(input)?;
                (decode_fn)(input, length.load_be::<usize>())?
            } else if let Some(min) = constraints.as_min() {
                let input = self.decode_length(input, <_>::default(), decode_fn)?;
                (decode_fn)(input, *min)?
            } else {
                let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

                if mask[0] == false {
                    let (input, length) = nom::bytes::streaming::take(7u8)(input)
                        .map(|(i, bs)| (i, bs.to_bitvec()))?;
                    (decode_fn)(input, length.load_be::<usize>())?
                } else {
                    let (input, mask) = nom::bytes::streaming::take(1u8)(input)?;

                    if mask[0] == false {
                        let (input, length) = nom::bytes::streaming::take(14u8)(input)
                            .map(|(i, bs)| (i, bs.to_bitvec()))?;
                        (decode_fn)(input, length.load_be::<usize>())?
                    } else {
                        let (input, mask) = nom::bytes::streaming::take(6u8)(input)?;
                        let length = match mask.load_be::<u8>() {
                            1 => SIXTEEN_K,
                            2 => THIRTY_TWO_K,
                            3 => FOURTY_EIGHT_K,
                            4 => SIXTY_FOUR_K,
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

                        input
                    }
                }
            },
        )
    }

    fn parse_one_bit(&mut self) -> Result<bool> {
        let (input, boolean) = nom::bytes::streaming::take(1u8)(self.input)?;
        self.input = input;
        Ok(boolean[0] as bool)
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = Error;

    fn decode_any(&mut self) -> Result<types::Any> {
        todo!()
    }

    fn decode_bool(&mut self, _: Tag) -> Result<bool> {
        self.parse_one_bit()
    }

    fn decode_enumerated(&mut self, tag: Tag, constraints: Constraints) -> Result<types::Integer> {
        self.decode_integer(tag, constraints)
    }

    fn decode_integer(&mut self, _: Tag, constraints: Constraints) -> Result<types::Integer> {
        let extensible = constraints
            .extensible()
            .map(|_| self.parse_one_bit())
            .transpose()?
            .unwrap_or_default();
        let value_range = constraints.value();

        let number = if let Some(range) = value_range
            .range()
            .filter(|range| !extensible && *range < SIXTY_FOUR_K.into())
        {
            let bits: usize = range.bits().try_into().map_err(|_| Error::range_exceeds_platform_width(<u64>::BITS, <usize>::BITS))?;
            let (input, data) = nom::bytes::streaming::take(bits)(self.input)?;
            self.input = input;
            num_bigint::BigUint::from_bytes_be(&data.to_bitvec().into_vec()).into()
        } else {
            let bytes = self.decode_octets()?.into_vec();
            value_range
                .as_min()
                .map(|_| num_bigint::BigUint::from_bytes_be(&bytes).into())
                .unwrap_or_else(|| types::Integer::from_signed_bytes_be(&bytes))
        };

        Ok(value_range.min() + number)
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

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        todo!()
    }

    fn decode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::BitString> {
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
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::VisibleString> {
        let mut bit_string = types::BitString::default();

        self.decode_extensible_container(constraints, |input, length| {
            let (input, part) = nom::bytes::streaming::take(length * 7)(input)?;
            bit_string.extend(&*part);
            Ok(input)
        })?;

        Ok(types::VisibleString::from_raw_bits(bit_string))
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
        todo!()
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        todo!()
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

    fn decode_sequence<D, F: FnOnce(&mut Self) -> Result<D>>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        decode_fn: F,
    ) -> Result<D> {
        todo!()
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _: Tag) -> Result<D> {
        D::decode(self)
    }

    fn decode_set<FIELDS, SET, F>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        decode_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode,
        FIELDS: Decode,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        todo!()
    }
}
