mod error;

use alloc::{borrow::ToOwned, vec::Vec};

use bitvec::prelude::*;
use snafu::*;

use super::{FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    enc::Error as _,
    types::{
        self,
        constraints::{self, Extensible},
        fields::FieldPresence,
        strings::{BitStr, DynConstrainedCharacterString, StaticPermittedAlphabet},
        BitString, Constraints, Enumerated, Tag,
    },
    Encode,
};

pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Clone, Copy, Default)]
pub struct EncoderOptions {
    aligned: bool,
    set_encoding: bool,
}

impl EncoderOptions {
    pub fn aligned() -> Self {
        Self {
            aligned: true,
            ..<_>::default()
        }
    }

    pub fn unaligned() -> Self {
        Self {
            aligned: false,
            ..<_>::default()
        }
    }

    fn without_set_encoding(mut self) -> Self {
        self.set_encoding = false;
        self
    }
}

#[derive(Debug)]
pub struct Encoder {
    options: EncoderOptions,
    output: BitString,
    set_output: alloc::collections::BTreeMap<Tag, BitString>,
    field_bitfield: alloc::collections::BTreeMap<Tag, (FieldPresence, bool)>,
    extension_fields: Vec<Vec<u8>>,
    is_extension_sequence: bool,
    parent_output_length: Option<usize>,
}

impl Encoder {
    pub fn new(options: EncoderOptions) -> Self {
        Self {
            options,
            output: <_>::default(),
            set_output: <_>::default(),
            field_bitfield: <_>::default(),
            is_extension_sequence: <_>::default(),
            extension_fields: <_>::default(),
            parent_output_length: <_>::default(),
        }
    }

    fn new_set_encoder<C: crate::types::Constructed>(&self) -> Self {
        let mut options = self.options;
        options.set_encoding = true;
        let mut encoder = Self::new(options);
        encoder.field_bitfield = C::FIELDS
            .canonised()
            .iter()
            .map(|field| (field.tag_tree.smallest_tag(), (field.presence, false)))
            .collect();
        encoder.parent_output_length = Some(self.output_length());
        encoder
    }

    fn new_sequence_encoder<C: crate::types::Constructed>(&self) -> Self {
        let mut encoder = Self::new(self.options.without_set_encoding());
        encoder.field_bitfield = C::FIELDS
            .iter()
            .map(|field| (field.tag_tree.smallest_tag(), (field.presence, false)))
            .collect();
        encoder.parent_output_length = Some(self.output_length());
        encoder
    }

    pub fn output(self) -> Vec<u8> {
        let mut output = self.bitstring_output();
        Self::force_pad_to_alignment(&mut output);
        super::to_vec(&output)
    }

    pub fn bitstring_output(self) -> BitString {
        self.options
            .set_encoding
            .then(|| self.set_output.values().flatten().collect::<BitString>())
            .unwrap_or(self.output)
    }

    pub fn set_bit(&mut self, tag: Tag, bit: bool) -> Result<()> {
        self.field_bitfield.entry(tag).and_modify(|(_, b)| *b = bit);
        Ok(())
    }

    fn output_length(&self) -> usize {
        let mut output_length = self.output.len();
        output_length += self.is_extension_sequence as usize;
        output_length += self
            .field_bitfield
            .values()
            .filter(|(presence, _)| presence.is_optional_or_default())
            .count();
        output_length += self.parent_output_length.unwrap_or_default();

        if self.options.set_encoding {
            output_length += self
                .set_output
                .values()
                .map(|output| output.len())
                .sum::<usize>();
        }

        output_length
    }

    fn pad_to_alignment(&self, buffer: &mut BitString) {
        if self.options.aligned {
            let mut output_length = self.output_length();
            output_length += buffer.len();
            if output_length % 8 != 0 {
                for _ in 0..(8 - output_length % 8) {
                    buffer.push(false);
                }
            }
        }
    }

    fn force_pad_to_alignment(buffer: &mut BitString) {
        const BYTE_WIDTH: usize = 8;
        if buffer.len() % BYTE_WIDTH != 0 {
            let mut string = BitString::new();
            for _ in 0..BYTE_WIDTH - (buffer.len() % 8) {
                string.push(false);
            }
            buffer.extend(string);
            debug_assert_eq!(0, buffer.len() % 8);
        }
    }

    fn encode_extensible_bit(
        &mut self,
        constraints: &Constraints,
        buffer: &mut BitString,
        extensible_condition: impl FnOnce() -> bool,
    ) -> bool {
        constraints
            .extensible()
            .then(|| {
                let is_in_constraints = !(extensible_condition)();
                buffer.push(is_in_constraints);
                is_in_constraints
            })
            .unwrap_or_default()
    }

    fn encode_known_multipler_string<S: StaticPermittedAlphabet>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &S,
    ) -> Result<()> {
        use crate::types::constraints::Bounded;
        let mut buffer = BitString::default();

        let is_large_string = if let Some(size) = constraints.size() {
            let width = match constraints.permitted_alphabet() {
                Some(alphabet) => {
                    self.character_width(super::log2(alphabet.constraint.len() as i128))
                }
                None => self.character_width(S::CHARACTER_WIDTH),
            };

            match *size.constraint {
                Bounded::Range {
                    start: Some(_),
                    end: Some(_),
                } if size.constraint.range().unwrap() * width as usize > 16 => true,
                Bounded::Single(max) if max * width as usize > 16 => {
                    self.pad_to_alignment(&mut buffer);
                    true
                }
                Bounded::Range {
                    start: None,
                    end: Some(max),
                } if max * width as usize > 16 => {
                    self.pad_to_alignment(&mut buffer);
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        match constraints.permitted_alphabet() {
            Some(alphabet)
                if S::CHARACTER_WIDTH
                    > self.character_width(super::log2(alphabet.constraint.len() as i128)) =>
            {
                let alphabet = &alphabet.constraint;
                let characters = &DynConstrainedCharacterString::from_bits(value.chars(), alphabet)
                    .map_err(Error::custom)?;

                self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
                    Ok(characters[range].to_bitvec())
                })?;
            }
            _ => {
                let char_length = value.len();
                let octet_aligned_value = self.options.aligned.then(|| {
                    if S::CHARACTER_WIDTH <= self.character_width(S::CHARACTER_WIDTH) {
                        value.to_octet_aligned_string()
                    } else {
                        value.to_octet_aligned_index_string()
                    }
                });
                let value = value.to_index_string();
                let octet_aligned_value = &octet_aligned_value;
                self.encode_string_length(
                    &mut buffer,
                    is_large_string,
                    char_length,
                    constraints.size(),
                    |range| {
                        Ok(match octet_aligned_value {
                            Some(value) => types::BitString::from_slice(&value[range]),
                            None => value[S::char_range_to_bit_range(range)].to_bitvec(),
                        })
                    },
                )?;
            }
        };

        self.extend(tag, &buffer);
        Ok(())
    }

    fn character_width(&self, width: u32) -> u32 {
        self.options
            .aligned
            .then(|| {
                width
                    .is_power_of_two()
                    .then_some(width)
                    .unwrap_or_else(|| width.next_power_of_two())
            })
            .unwrap_or(width)
    }

    fn encoded_extension_addition(extension_fields: &[Vec<u8>]) -> bool {
        !extension_fields.iter().all(|vec| vec.is_empty())
    }

    fn encode_constructed<C: crate::types::Constructed>(
        &mut self,
        tag: Tag,
        mut encoder: Self,
    ) -> Result<()> {
        self.set_bit(tag, true)?;
        let mut buffer = BitString::default();

        if C::EXTENDED_FIELDS.is_some() {
            buffer.push(Self::encoded_extension_addition(&encoder.extension_fields));
        }

        for bit in encoder
            .field_bitfield
            .values()
            .filter_map(|(presence, is_present)| {
                presence.is_optional_or_default().then_some(is_present)
            })
            .copied()
        {
            buffer.push(bit);
        }

        let extension_fields = core::mem::take(&mut encoder.extension_fields);

        if encoder.field_bitfield.values().any(|(_, b)| *b) {
            buffer.extend(encoder.bitstring_output());
        }

        if !Self::encoded_extension_addition(&extension_fields) {
            self.extend(tag, &buffer);
            return Ok(());
        }

        let bitfield_length = extension_fields.len();
        let mut extension_buffer = {
            let mut buffer = BitString::new();
            self.encode_normally_small_length(bitfield_length, &mut buffer)?;
            buffer
        };

        for field in &extension_fields {
            extension_buffer.push(!field.is_empty());
        }

        for field in extension_fields
            .into_iter()
            .filter(|field| !field.is_empty())
        {
            self.encode_length(
                &mut extension_buffer,
                field.len(),
                <_>::default(),
                |range| Ok(BitString::from_slice(&field[range])),
            )?;
        }

        buffer.extend_from_bitslice(&extension_buffer);
        self.extend(tag, &buffer);

        Ok(())
    }

    fn encode_normally_small_length(&mut self, value: usize, buffer: &mut BitString) -> Result<()> {
        debug_assert!(value >= 1);
        let value = if value >= 64 { value } else { value - 1 };
        self.encode_normally_small_integer(value, buffer)
    }

    fn encode_normally_small_integer(
        &mut self,
        value: usize,
        buffer: &mut BitString,
    ) -> Result<()> {
        let is_large = value >= 64;
        buffer.push(is_large);

        let size_constraints = if is_large {
            constraints::Value::new(constraints::Bounded::start_from(0)).into()
        } else {
            constraints::Value::new(constraints::Bounded::new(0, 63)).into()
        };

        self.encode_integer_into_buffer(
            Constraints::new(&[size_constraints]),
            &value.into(),
            buffer,
        )
    }

    fn encode_string_length(
        &self,
        buffer: &mut BitString,
        is_large_string: bool,
        length: usize,
        constraints: Option<&Extensible<constraints::Size>>,
        encode_fn: impl Fn(core::ops::Range<usize>) -> Result<BitString>,
    ) -> Result<()> {
        let Some(constraints) = constraints else {
            return self.encode_unconstrained_length(buffer, length, None, encode_fn);
        };

        if constraints.extensible.is_none() {
            Error::check_length(length, &constraints.constraint)?;
        } else if constraints.constraint.contains(&length) {
            buffer.push(false);
        } else {
            buffer.push(true);
        }

        let constraints = constraints.constraint;

        match constraints.start_and_end() {
            (Some(_), Some(_)) => {
                let range = constraints.range().unwrap();

                if range == 0 {
                    Ok(())
                } else if range == 1 {
                    buffer.extend((encode_fn)(0..length)?);
                    Ok(())
                } else if range < SIXTY_FOUR_K as usize {
                    let effective_length = constraints.effective_value(length).into_inner();
                    let range = (self.options.aligned && range > 256)
                        .then(|| {
                            let range = super::log2(range as i128);
                            super::range_from_bits(
                                range
                                    .is_power_of_two()
                                    .then_some(range)
                                    .unwrap_or_else(|| range.next_power_of_two()),
                            )
                        })
                        .unwrap_or(range as i128);
                    self.encode_non_negative_binary_integer(
                        buffer,
                        range,
                        &(effective_length as u32).to_be_bytes(),
                    );

                    if is_large_string {
                        self.pad_to_alignment(buffer);
                    }

                    buffer.extend((encode_fn)(0..length)?);
                    Ok(())
                } else {
                    self.encode_unconstrained_length(buffer, length, None, encode_fn)
                }
            }
            _ => self.encode_unconstrained_length(buffer, length, None, encode_fn),
        }
    }

    fn encode_length(
        &self,
        buffer: &mut BitString,
        length: usize,
        constraints: Option<&Extensible<constraints::Size>>,
        encode_fn: impl Fn(core::ops::Range<usize>) -> Result<BitString>,
    ) -> Result<()> {
        let Some(constraints) = constraints else {
            return self.encode_unconstrained_length(buffer, length, None, encode_fn);
        };

        if constraints.extensible.is_none() {
            Error::check_length(length, &constraints.constraint)?;
        } else if constraints.constraint.contains(&length) {
            buffer.push(false);
        } else {
            buffer.push(true);
        }

        let constraints = constraints.constraint;

        match constraints.start_and_end() {
            (Some(_), Some(_)) => {
                let range = constraints.range().unwrap();

                if range == 0 {
                    Ok(())
                } else if range == 1 {
                    buffer.extend((encode_fn)(0..length)?);
                    Ok(())
                } else if range < SIXTY_FOUR_K as usize {
                    let effective_length = constraints.effective_value(length).into_inner();
                    let range = (self.options.aligned && range > 256)
                        .then(|| {
                            let range = super::log2(range as i128);
                            super::range_from_bits(
                                range
                                    .is_power_of_two()
                                    .then_some(range)
                                    .unwrap_or_else(|| range.next_power_of_two()),
                            )
                        })
                        .unwrap_or(range as i128);
                    self.encode_non_negative_binary_integer(
                        buffer,
                        range,
                        &(effective_length as u32).to_be_bytes(),
                    );
                    // self.pad_to_alignment(buffer);
                    buffer.extend((encode_fn)(0..length)?);
                    Ok(())
                } else {
                    self.encode_unconstrained_length(buffer, length, None, encode_fn)
                }
            }
            _ => self.encode_unconstrained_length(buffer, length, None, encode_fn),
        }
    }

    fn encode_unconstrained_length(
        &self,
        buffer: &mut BitString,
        mut length: usize,
        min: Option<usize>,
        encode_fn: impl Fn(core::ops::Range<usize>) -> Result<BitString>,
    ) -> Result<()> {
        let mut min = min.unwrap_or_default();

        self.pad_to_alignment(&mut *buffer);
        if length <= 127 {
            buffer.extend((length as u8).to_be_bytes());
            buffer.extend((encode_fn)(0..length)?);
        } else if length < SIXTEEN_K.into() {
            const SIXTEENTH_BIT: u16 = 0x8000;
            buffer.extend((SIXTEENTH_BIT | length as u16).to_be_bytes());
            buffer.extend((encode_fn)(0..length)?);
        } else {
            loop {
                // Hack to get around no exclusive syntax.
                const K64: usize = SIXTY_FOUR_K as usize;
                const K48: usize = FOURTY_EIGHT_K as usize;
                const K32: usize = THIRTY_TWO_K as usize;
                const K16: usize = SIXTEEN_K as usize;
                const K64_MAX: usize = K64 - 1;
                const K48_MAX: usize = K48 - 1;
                const K32_MAX: usize = K32 - 1;
                let (fragment_index, amount) = match length {
                    K64..=usize::MAX => (4, K64),
                    K48..=K64_MAX => (3, K48),
                    K32..=K48_MAX => (2, K32),
                    K16..=K32_MAX => (1, K16),
                    _ => {
                        break self.encode_unconstrained_length(
                            buffer,
                            length,
                            Some(min),
                            encode_fn,
                        )?;
                    }
                };

                const FRAGMENT_MARKER: u8 = 0xC0;
                buffer.extend(&[FRAGMENT_MARKER | fragment_index]);

                buffer.extend((encode_fn)(min..min + amount)?);
                min += amount;

                if length == SIXTEEN_K as usize {
                    // Add final fragment in the frame.
                    buffer.extend(&[0]);
                    break;
                } else {
                    length = length.saturating_sub(amount);
                }
            }
        }

        Ok(())
    }

    fn extend<'input>(&mut self, tag: Tag, input: impl Into<Input<'input>>) {
        use bitvec::field::BitField;
        let mut set_buffer = <_>::default();
        let buffer = if self.options.set_encoding {
            &mut set_buffer
        } else {
            &mut self.output
        };

        match input.into() {
            Input::Bits(bits) => {
                buffer.extend_from_bitslice(bits);
            }
            Input::Bit(bit) => {
                buffer.push(bit);
            }
            Input::Byte(byte) => {
                buffer.store_be(byte);
            }
            Input::Bytes(bytes) => {
                buffer.extend(bytes);
            }
        }

        if self.options.set_encoding {
            self.set_output.insert(tag, set_buffer);
        }
    }

    fn encode_octet_string_into_buffer(
        &mut self,
        constraints: Constraints,
        value: &[u8],
        buffer: &mut BitString,
    ) -> Result<()> {
        let extensible_is_present = self.encode_extensible_bit(&constraints, buffer, || todo!());
        let Some(constraints) = constraints.size() else {
            return self.encode_length(buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            });
        };

        if extensible_is_present {
            self.encode_length(buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        } else if 0
            == constraints
                .constraint
                .effective_value(value.len())
                .into_inner()
        {
            // NO-OP
        } else {
            self.encode_length(buffer, value.len(), Some(constraints), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        }

        Ok(())
    }

    fn encode_integer_into_buffer(
        &mut self,
        constraints: Constraints,
        value: &num_bigint::BigInt,
        buffer: &mut BitString,
    ) -> Result<()> {
        self.encode_extensible_bit(&constraints, buffer, || {
            constraints.value().map_or(false, |value_range| {
                value_range.extensible.is_some() && value_range.constraint.bigint_contains(value)
            })
        });
        let Some(value_range) = constraints.value() else {
            let bytes = value.to_signed_bytes_be();
            self.encode_length(buffer, bytes.len(), constraints.size(), |range| {
                Ok(BitString::from_slice(&bytes[range]))
            })?;
            return Ok(());
        };

        let bytes = match value_range.constraint.effective_bigint_value(value.clone()) {
            either::Left(offset) => offset.to_biguint().unwrap().to_bytes_be(),
            either::Right(value) => value.to_signed_bytes_be(),
        };

        let effective_value: i128 = value_range
            .constraint
            .effective_value(value.try_into().map_err(Error::custom)?)
            .either_into();

        const K64: i128 = SIXTY_FOUR_K as i128;
        const OVER_K64: i128 = K64 + 1;

        if let Some(range) = value_range.constraint.range() {
            match (self.options.aligned, range) {
                (true, 256) => {
                    self.pad_to_alignment(buffer);
                    self.encode_non_negative_binary_integer(buffer, range, &bytes)
                }
                (true, 257..=K64) => {
                    self.pad_to_alignment(buffer);
                    self.encode_non_negative_binary_integer(buffer, K64, &bytes);
                }
                (true, OVER_K64..) => {
                    let range_len_in_bytes = num_integer::div_ceil(super::log2(range), 8) as i128;

                    if effective_value == 0 {
                        self.encode_non_negative_binary_integer(
                            &mut *buffer,
                            range_len_in_bytes,
                            &[0],
                        );
                        self.pad_to_alignment(&mut *buffer);
                        self.encode_non_negative_binary_integer(&mut *buffer, 255, &bytes);
                    } else {
                        let range_value_in_bytes =
                            num_integer::div_ceil(super::log2(effective_value + 1), 8) as i128;
                        self.encode_non_negative_binary_integer(
                            buffer,
                            range_len_in_bytes,
                            &(range_value_in_bytes - 1).to_be_bytes(),
                        );
                        self.pad_to_alignment(&mut *buffer);
                        self.encode_non_negative_binary_integer(
                            &mut *buffer,
                            super::range_from_bits(range_value_in_bytes as u32 * 8),
                            &bytes,
                        );
                    }
                }
                (false, OVER_K64..) => {
                    self.encode_length(buffer, bytes.len(), <_>::default(), |range| {
                        Ok(BitString::from_slice(&bytes[range]))
                    })?;
                }
                (_, _) => self.encode_non_negative_binary_integer(buffer, range, &bytes),
            }
        } else {
            self.encode_length(buffer, bytes.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&bytes[range]))
            })?;
        }

        Ok(())
    }

    fn encode_non_negative_binary_integer(
        &self,
        buffer: &mut BitString,
        range: i128,
        bytes: &[u8],
    ) {
        use core::cmp::Ordering;
        let total_bits = super::log2(range) as usize;
        let bits = BitVec::<u8, Msb0>::from_slice(bytes);
        let bits = match total_bits.cmp(&bits.len()) {
            Ordering::Greater => {
                let mut padding = types::BitString::repeat(false, total_bits - bits.len());
                padding.extend(bits);
                padding
            }
            Ordering::Less => bits[bits.len() - total_bits..].to_owned(),
            Ordering::Equal => bits,
        };

        // if !self.options.aligned && range >= super::TWO_FIFTY_SIX.into() {
        //     self.pad_to_alignment(buffer);
        // }

        buffer.extend(bits);
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = Error;

    fn encode_any(&mut self, tag: Tag, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(tag, <_>::default(), &value.contents)
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        let mut buffer = BitString::default();
        let extensible_is_present =
            self.encode_extensible_bit(&constraints, &mut buffer, || todo!());
        let size = constraints.size();

        if extensible_is_present || size.is_none() {
            self.encode_length(&mut buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        } else if size.map(|size| size.constraint.effective_value(value.len()).into_inner())
            == Some(0)
        {
            // NO-OP
        } else {
            self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.extend(tag, value);
        Ok(())
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        let mut buffer = BitString::default();
        let index = value.enumeration_index();
        if E::EXTENDED_VARIANTS.is_some() {
            buffer.push(value.is_extended_variant());
        }

        if value.is_extended_variant() {
            self.encode_normally_small_integer(index, &mut buffer)?;
        } else if core::mem::size_of::<usize>() == 4 {
            self.encode_non_negative_binary_integer(
                &mut buffer,
                E::variance() as i128,
                &u32::try_from(index).unwrap().to_be_bytes(),
            );
        } else if core::mem::size_of::<usize>() == 2 {
            self.encode_non_negative_binary_integer(
                &mut buffer,
                E::variance() as i128,
                &u16::try_from(index).unwrap().to_be_bytes(),
            );
        } else {
            self.encode_non_negative_binary_integer(
                &mut buffer,
                E::variance() as i128,
                &usize::to_be_bytes(index)[..],
            );
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        let mut buffer = BitString::new();
        self.encode_integer_into_buffer(constraints, value, &mut buffer)?;
        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        Ok(())
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        let der = crate::der::encode_scope(|encoder| encoder.encode_object_identifier(tag, oid))
            .context(error::DerSnafu)?;
        self.encode_octet_string(tag, <_>::default(), &der)
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        let mut buffer = BitString::default();
        self.encode_octet_string_into_buffer(constraints, value, &mut buffer)?;
        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(tag, <_>::default(), value)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(tag, <_>::default(), value)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(tag, <_>::default(), value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(
            tag,
            <_>::default(),
            &crate::der::encode(value).context(error::DerSnafu)?,
        )
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        self.encode_octet_string(
            tag,
            <_>::default(),
            &crate::der::encode(value).context(error::DerSnafu)?,
        )
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let options = self.options;
        self.set_bit(tag, true)?;

        self.encode_length(&mut buffer, values.len(), constraints.size(), |range| {
            let mut buffer = BitString::default();
            for value in &values[range] {
                let mut encoder = Self::new(options);
                E::encode(value, &mut encoder)?;
                buffer.extend(encoder.bitstring_output());
            }
            Ok(buffer)
        })?;

        self.extend(tag, &buffer);

        Ok(())
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &values.iter().collect::<Vec<_>>(), constraints)
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        if let Some((_, true)) = self.field_bitfield.get(&tag) {
            value.encode(self)
        } else if self.field_bitfield.get(&tag).is_none() {
            // There is no bitfield if none of the parent objects is struct/set
            // But we still need to handle nested choices explicitly
            value.encode(self)
        } else {
            self.set_bit(tag, true)?;
            value.encode_with_tag(self, tag)
        }
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        self.set_bit(E::TAG, true)?;
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        value.encode_with_tag(self, tag)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        value.encode_with_tag_and_constraints(self, tag, constraints)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.set_bit(E::TAG, false)?;
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, false)?;
        Ok(())
    }

    fn encode_sequence<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut encoder = self.new_sequence_encoder::<C>();
        (encoder_scope)(&mut encoder)?;
        self.encode_constructed::<C>(tag, encoder)
    }

    fn encode_set<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut set = self.new_set_encoder::<C>();

        (encoder_scope)(&mut set)?;

        self.encode_constructed::<C>(tag, set)
    }

    fn encode_choice<E: Encode + crate::types::Choice>(
        &mut self,
        constraints: Constraints,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::new();
        let mut choice_encoder = Self::new(self.options.without_set_encoding());
        let tag = (encode_fn)(&mut choice_encoder)?;
        let is_root_extension = crate::TagTree::tag_contains(&tag, E::VARIANTS);

        self.encode_extensible_bit(&constraints, &mut buffer, || is_root_extension);
        let variants = crate::types::variants::Variants::from_static(if is_root_extension {
            E::VARIANTS
        } else {
            E::EXTENDED_VARIANTS
        });

        let index = variants
            .iter()
            .enumerate()
            .find_map(|(i, &variant_tag)| (tag == variant_tag).then_some(i))
            .ok_or_else(|| Error::custom("variant not found in choice"))?;

        let bounds = if is_root_extension {
            let variance = variants.len();
            debug_assert!(variance > 0);

            if variance != 1 {
                Some(Some(variance))
            } else {
                None
            }
        } else {
            Some(None)
        };
        match (index, bounds) {
            (index, Some(Some(variance))) => {
                // https://github.com/XAMPPRocky/rasn/issues/168
                // Choice index starts from zero, so we need to reduce variance by one
                let choice_range = &[constraints::Value::new(constraints::Bounded::new(
                    0,
                    (variance - 1) as i128,
                ))
                .into()];
                self.encode_integer_into_buffer(
                    Constraints::from(choice_range),
                    &index.into(),
                    &mut buffer,
                )?;

                buffer.extend(choice_encoder.output);
            }
            (index, Some(None)) => {
                self.encode_normally_small_integer(index, &mut buffer)?;
                let mut output = choice_encoder.output();

                if output.is_empty() {
                    output.push(0);
                }
                self.encode_octet_string_into_buffer(<_>::default(), &output, &mut buffer)?;
            }
            (_, None) => {}
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error> {
        let mut encoder = Self::new(self.options.without_set_encoding());
        encoder.field_bitfield = <_>::from([(tag, (FieldPresence::Optional, false))]);
        E::encode_with_tag_and_constraints(&value, &mut encoder, tag, constraints)?;

        if encoder.field_bitfield.get(&tag).map_or(false, |(_, b)| *b) {
            self.set_bit(tag, true)?;
            self.extension_fields.push(encoder.output());
        } else {
            self.set_bit(tag, false)?;
            self.extension_fields.push(Vec::new());
        }

        Ok(())
    }

    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed,
    {
        let Some(value) = value else {
            self.set_bit(E::TAG, false)?;
            self.extension_fields.push(Vec::new());
            return Ok(());
        };

        self.set_bit(E::TAG, true)?;
        let mut encoder = self.new_sequence_encoder::<E>();
        encoder.is_extension_sequence = true;
        value.encode(&mut encoder)?;

        let output = encoder.output();
        self.extension_fields.push(output);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Input<'input> {
    Bit(bool),
    Byte(u8),
    Bits(&'input BitString),
    Bytes(&'input [u8]),
}

impl<'input> From<&'input BitString> for Input<'input> {
    fn from(value: &'input BitString) -> Self {
        Self::Bits(value)
    }
}

impl<'input> From<&'input [u8]> for Input<'input> {
    fn from(value: &'input [u8]) -> Self {
        Self::Bytes(value)
    }
}

impl<'input> From<&'input Vec<u8>> for Input<'input> {
    fn from(value: &'input Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl<'input> From<bool> for Input<'input> {
    fn from(value: bool) -> Self {
        Self::Bit(value)
    }
}

impl<'input> From<u8> for Input<'input> {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Encoder as _;

    #[derive(crate::AsnType, Default, crate::Encode, Clone, Copy)]
    #[rasn(crate_root = "crate")]
    struct Byte {
        one: bool,
        two: bool,
        three: bool,
        four: bool,
        five: bool,
        six: bool,
        seven: bool,
        eight: bool,
    }

    impl Byte {
        const MAX: Self = Self {
            one: true,
            two: true,
            three: true,
            four: true,
            five: true,
            six: true,
            seven: true,
            eight: true,
        };
    }

    #[test]
    fn length() {
        let encoder = Encoder::new(EncoderOptions::unaligned());
        let mut buffer = types::BitString::new();
        encoder
            .encode_length(
                &mut buffer,
                4,
                Some(&Extensible::new(constraints::Size::new(
                    constraints::Bounded::new(1, 64),
                ))),
                |_| Ok(<_>::default()),
            )
            .unwrap();
        assert_eq!(&[0xC], buffer.as_raw_slice());
    }

    #[test]
    fn sequence() {
        assert_eq!(&[0xff], &*crate::uper::encode(&Byte::MAX).unwrap());
    }

    #[test]
    fn constrained_integer() {
        assert_eq!(&[0xff], &*crate::uper::encode(&0xffu8).unwrap());
    }

    #[test]
    fn normally_small_integer() {
        let mut encoder = Encoder::new(EncoderOptions::unaligned());
        let mut buffer = types::BitString::new();
        encoder
            .encode_normally_small_integer(2, &mut buffer)
            .unwrap();
        assert_eq!(buffer.len(), 7);
        assert_eq!(bitvec::bits![0, 0, 0, 0, 0, 1, 0], buffer);
    }

    #[test]
    fn unconstrained_integer() {
        assert_eq!(
            &[0b00000010, 0b00010000, 0],
            &*crate::uper::encode(&types::Integer::from(4096)).unwrap()
        );
        struct CustomInt(i32);

        impl crate::AsnType for CustomInt {
            const TAG: Tag = Tag::INTEGER;
            const CONSTRAINTS: Constraints<'static> =
                Constraints::new(&[constraints::Constraint::Value(
                    constraints::Extensible::new(constraints::Value::new(
                        constraints::Bounded::up_to(65535),
                    )),
                )]);
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag_and_constraints<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer(tag, constraints, &self.0.into())
                    .map(drop)
            }
        }

        assert_eq!(
            &[0b00000001, 0b01111111],
            &*crate::uper::encode(&CustomInt(127)).unwrap()
        );
        assert_eq!(
            &[0b00000001, 0b10000000],
            &*crate::uper::encode(&CustomInt(-128)).unwrap()
        );
        assert_eq!(
            &[0b00000010, 0b00000000, 0b10000000],
            &*crate::uper::encode(&CustomInt(128)).unwrap()
        );
    }

    #[test]
    fn semi_constrained_integer() {
        let mut encoder = Encoder::new(EncoderOptions::unaligned());
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(&[constraints::Value::from(constraints::Bounded::start_from(
                    -1,
                ))
                .into()]),
                &4096.into(),
            )
            .unwrap();

        assert_eq!(&[2, 0b00010000, 1], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(&[
                    constraints::Value::from(constraints::Bounded::start_from(1)).into(),
                ]),
                &127.into(),
            )
            .unwrap();
        assert_eq!(&[1, 0b01111110], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(&[
                    constraints::Value::from(constraints::Bounded::start_from(0)).into(),
                ]),
                &128.into(),
            )
            .unwrap();
        assert_eq!(&[1, 0b10000000], &*encoder.output.into_vec());
    }

    #[track_caller]
    fn assert_encode<T: Encode>(options: EncoderOptions, value: T, expected: &[u8]) {
        let mut encoder = Encoder::new(options);
        T::encode(&value, &mut encoder).unwrap();
        let output = encoder.output.clone().into_vec();
        assert_eq!(
            expected
                .iter()
                .map(|ch| format!("{ch:08b}"))
                .collect::<Vec<_>>(),
            output
                .iter()
                .map(|ch| format!("{ch:08b}"))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn visible_string() {
        use crate::types::VisibleString;

        assert_encode(
            EncoderOptions::unaligned(),
            VisibleString::try_from("John").unwrap(),
            &[4, 0x95, 0xBF, 0x46, 0xE0],
        );
        assert_encode(
            EncoderOptions::aligned(),
            VisibleString::try_from("John").unwrap(),
            &[4, 0x4A, 0x6F, 0x68, 0x6E],
        );
    }

    #[test]
    fn sequence_of() {
        let make_buffer =
            |length| crate::uper::encode(&alloc::vec![Byte::default(); length]).unwrap();
        assert_eq!(&[5, 0, 0, 0, 0, 0], &*(make_buffer)(5));
        assert!((make_buffer)(130).starts_with(&[0b10000000u8, 0b10000010]));
        assert!((make_buffer)(16000).starts_with(&[0b10111110u8, 0b10000000]));
        let buffer = (make_buffer)(THIRTY_TWO_K as usize);
        assert_eq!(THIRTY_TWO_K as usize + 2, buffer.len());
        assert!(buffer.starts_with(&[0b11000010]));
        assert!(buffer.ends_with(&[0]));
        let buffer = (make_buffer)(99000);
        assert_eq!(99000 + 4, buffer.len());
        assert!(buffer.starts_with(&[0b11000100]));
        assert!(buffer[1 + SIXTY_FOUR_K as usize..].starts_with(&[0b11000010]));
        assert!(buffer[SIXTY_FOUR_K as usize + THIRTY_TWO_K as usize + 2..]
            .starts_with(&[0b10000010, 0b10111000]));
    }
}
