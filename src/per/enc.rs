//! Encoding Rust structures into Packed Encoding Rules data.

use alloc::{borrow::ToOwned, string::ToString, vec::Vec};

use bitvec::prelude::*;

use super::{
    FOURTY_EIGHT_K, LARGE_UNSIGNED_CONSTRAINT, SIXTEEN_K, SIXTY_FOUR_K, SMALL_UNSIGNED_CONSTRAINT,
    THIRTY_TWO_K,
};
use crate::{
    types::{
        self,
        constraints::{self, Extensible, Size},
        strings::{
            should_be_indexed, BitStr, DynConstrainedCharacterString, StaticPermittedAlphabet,
        },
        BitString, Constraints, Enumerated, IntegerType, Tag,
    },
    Encode,
};

pub use crate::error::EncodeError as Error;
type Result<T, E = Error> = core::result::Result<T, E>;

/// Options for configuring the [`Encoder`].
#[derive(Debug, Clone, Copy, Default)]
pub struct EncoderOptions {
    aligned: bool,
    set_encoding: bool,
}

impl EncoderOptions {
    /// Returns the default encoder options for Aligned Packed Encoding Rules.
    #[must_use]
    pub fn aligned() -> Self {
        Self {
            aligned: true,
            ..<_>::default()
        }
    }

    /// Returns the default encoder options for Unaligned Packed Encoding Rules.
    #[must_use]
    pub fn unaligned() -> Self {
        Self {
            aligned: false,
            ..<_>::default()
        }
    }

    #[must_use]
    fn without_set_encoding(mut self) -> Self {
        self.set_encoding = false;
        self
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

/// Encodes Rust data structures into Canonical Packed Encoding Rules (CPER) data.
///
/// Const `RCL` is the count of root components in the root component list of a sequence or set.
/// Const `ECL` is the count of extension additions in the extension addition component type list in a sequence or set.
#[derive(Debug)]
pub struct Encoder<const RCL: usize = 0, const ECL: usize = 0> {
    options: EncoderOptions,
    output: BitString,
    set_output: alloc::collections::BTreeMap<Tag, BitString>,
    number_optional_default_fields: usize,
    root_bitfield: (usize, [(bool, Tag); RCL]),
    extension_bitfield: (usize, [bool; ECL]),
    extension_fields: [Option<Vec<u8>>; ECL],
    is_extension_sequence: bool,
    parent_output_length: Option<usize>,
}

impl<const RCL: usize, const ECL: usize> Encoder<RCL, ECL> {
    /// Constructs a new encoder from the provided options.
    pub fn new(options: EncoderOptions) -> Self {
        Self {
            options,
            output: <_>::default(),
            set_output: <_>::default(),
            number_optional_default_fields: 0,
            root_bitfield: (0, [(false, Tag::new_private(0)); RCL]),
            extension_bitfield: (0, [false; ECL]),
            is_extension_sequence: <_>::default(),
            extension_fields: [(); ECL].map(|_| None),
            parent_output_length: <_>::default(),
        }
    }
    fn codec(&self) -> crate::Codec {
        self.options.current_codec()
    }
    fn new_set_encoder<const RL: usize, const EL: usize, C: crate::types::Constructed<RL, EL>>(
        &self,
    ) -> Encoder<RL, EL> {
        let mut options = self.options;
        options.set_encoding = true;
        let mut encoder = Encoder::<RL, EL>::new(options);
        encoder.number_optional_default_fields = C::FIELDS.number_of_optional_and_default_fields();
        encoder.is_extension_sequence = C::IS_EXTENSIBLE;
        encoder.parent_output_length = Some(self.output_length());
        encoder
    }

    fn new_sequence_encoder<
        const RL: usize,
        const EL: usize,
        C: crate::types::Constructed<RL, EL>,
    >(
        &self,
    ) -> Encoder<RL, EL> {
        let mut encoder = Encoder::<RL, EL>::new(self.options.without_set_encoding());
        encoder.number_optional_default_fields = C::FIELDS.number_of_optional_and_default_fields();
        encoder.is_extension_sequence = C::IS_EXTENSIBLE;
        encoder.parent_output_length = Some(self.output_length());
        encoder
    }

    /// Returns the octet aligned output for the encoder.
    pub fn output(&mut self) -> Vec<u8> {
        let mut output = self.bitstring_output();
        Self::force_pad_to_alignment(&mut output);
        output.as_raw_slice().to_vec()
    }

    /// Returns the bit level output for the encoder.
    fn bitstring_output(&mut self) -> BitString {
        self.options
            .set_encoding
            .then(|| self.set_output.values().flatten().collect::<BitString>())
            .unwrap_or(core::mem::take(&mut *self.output.as_mut()))
    }

    /// Sets the presence of a `OPTIONAL` or `DEFAULT` field in the bitfield.
    /// The presence is ordered based on the field index.
    fn set_presence(&mut self, tag: Tag, bit: bool) {
        // Applies only for SEQUENCE and SET types (RCL > 0)
        // Compiler should optimize this out
        if RCL > 0 {
            if self.number_optional_default_fields < self.root_bitfield.0 + 1 {
                // Fields should be encoded in order
                // When the presence of optional extension field is set, we end up here
                // However, we don't need that information
                return;
            }
            self.root_bitfield.1[self.root_bitfield.0] = (bit, tag);
            self.root_bitfield.0 += 1;
        }
    }
    fn set_extension_presence(&mut self, bit: bool) {
        // Applies only for SEQUENCE and SET types (ECL > 0)
        // Compiler should optimize this out when not present
        if ECL > 0 {
            self.extension_bitfield.1[self.extension_bitfield.0] = bit;
            self.extension_bitfield.0 += 1;
        }
    }

    fn output_length(&self) -> usize {
        let mut output_length = self.output.len();
        output_length += self.is_extension_sequence as usize;
        output_length += self.number_optional_default_fields;
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

    fn encode_known_multiplier_string<S: StaticPermittedAlphabet>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &S,
    ) -> Result<()> {
        use crate::types::constraints::Bounded;
        let mut buffer = BitString::default();
        let string_length = value.len();

        let is_extended_value = self.encode_extensible_bit(constraints, &mut buffer, || {
            constraints.size().map_or(false, |size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&string_length)
            })
        });

        let is_large_string = if let Some(size) = constraints.size() {
            let width = match constraints.permitted_alphabet() {
                Some(alphabet) => self
                    .character_width(crate::num::log2(alphabet.constraint.len() as i128) as usize),
                None => self.character_width(S::CHARACTER_SET_WIDTH),
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

        match (
            constraints.permitted_alphabet(),
            should_be_indexed(S::CHARACTER_SET_WIDTH as u32, S::CHARACTER_SET),
            constraints.permitted_alphabet().map(|alphabet| {
                S::CHARACTER_SET_WIDTH
                    > self.character_width(
                        crate::num::log2(alphabet.constraint.len() as i128) as usize
                    )
            }),
        ) {
            (Some(alphabet), _, Some(true)) | (Some(alphabet), true, _) => {
                let alphabet = &alphabet.constraint;
                let characters = &DynConstrainedCharacterString::from_bits(value.chars(), alphabet)
                    .map_err(|e| Error::alphabet_constraint_not_satisfied(e, self.codec()))?;

                self.encode_length(
                    &mut buffer,
                    value.len(),
                    is_extended_value
                        .then(|| -> Extensible<Size> { <_>::default() })
                        .as_ref()
                        .or(constraints.size()),
                    |range| Ok(characters[range].to_bitvec()),
                )?;
            }
            (None, true, _) => {
                let characters =
                    &DynConstrainedCharacterString::from_bits(value.chars(), S::CHARACTER_SET)
                        .map_err(|e| Error::alphabet_constraint_not_satisfied(e, self.codec()))?;

                self.encode_length(
                    &mut buffer,
                    value.len(),
                    is_extended_value
                        .then(|| -> Extensible<Size> { <_>::default() })
                        .as_ref()
                        .or(constraints.size()),
                    |range| Ok(characters[range].to_bitvec()),
                )?;
            }
            _ => {
                let char_length = value.len();
                let octet_aligned_value = self.options.aligned.then(|| {
                    if S::CHARACTER_SET_WIDTH <= self.character_width(S::CHARACTER_SET_WIDTH) {
                        value.to_octet_aligned_string()
                    } else {
                        value.to_octet_aligned_index_string()
                    }
                });
                // 30.5.4 Rec. ITU-T X.691 (02/2021)
                let value = value.to_index_or_value_bitstring();

                let octet_aligned_value = &octet_aligned_value;
                self.encode_string_length(
                    &mut buffer,
                    is_large_string,
                    char_length,
                    is_extended_value
                        .then(|| -> Extensible<Size> { <_>::default() })
                        .as_ref()
                        .or(constraints.size()),
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

    fn character_width(&self, width: usize) -> usize {
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

    fn encode_constructed<
        const RL: usize,
        const EL: usize,
        C: crate::types::Constructed<RL, EL>,
    >(
        &mut self,
        tag: Tag,
        mut encoder: Encoder<RL, EL>,
    ) -> Result<()> {
        let mut buffer = BitString::with_capacity(core::mem::size_of::<C>());
        let mut extensions_present = false;
        if C::IS_EXTENSIBLE {
            extensions_present = encoder.extension_fields.iter().any(Option::is_some);
            buffer.push(extensions_present);
        }
        let required_present = C::FIELDS.has_required_field();
        let (needed, option_bitfield) = if encoder.options.set_encoding {
            // In set encoding, tags must be unique so we just sort them to be in canonical order for preamble
            encoder
                .root_bitfield
                .1
                .sort_by(|(_, tag1), (_, tag2)| tag1.cmp(tag2));
            encoder.root_bitfield
        } else {
            encoder.root_bitfield
        };
        debug_assert!(C::FIELDS.number_of_optional_and_default_fields() == needed);
        if needed > 0 || C::IS_EXTENSIBLE {
            for (bit, _tag) in option_bitfield[..needed].iter() {
                buffer.push(*bit);
            }
        }
        if option_bitfield[..needed].iter().any(|(bit, _tag)| *bit) || required_present {
            let mut out = encoder.bitstring_output();
            buffer.append(&mut out);
        }

        if !C::IS_EXTENSIBLE || !extensions_present {
            self.extend(tag, &buffer);
            return Ok(());
        }
        self.encode_normally_small_length(EL, &mut buffer)?;
        for bit in encoder.extension_fields.iter() {
            buffer.push(bit.is_some());
        }

        for field in encoder.extension_fields.iter().filter_map(Option::as_ref) {
            self.encode_length(&mut buffer, field.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&field[range]))
            })?;
        }
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

        if is_large {
            self.encode_integer_into_buffer::<usize>(LARGE_UNSIGNED_CONSTRAINT, &value, buffer)
        } else {
            self.encode_integer_into_buffer::<usize>(SMALL_UNSIGNED_CONSTRAINT, &value, buffer)
        }
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
            Error::check_length(length, &constraints.constraint, self.codec())?;
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
                            let range = crate::num::log2(range as i128);
                            crate::bits::range_from_len(
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
        self.encode_string_length(buffer, false, length, constraints, encode_fn)
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
        let octet_string_length = value.len();
        let extensible_is_present = self.encode_extensible_bit(&constraints, buffer, || {
            constraints.size().map_or(false, |size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&octet_string_length)
            })
        });
        let Some(size) = constraints.size() else {
            return self.encode_length(buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            });
        };

        if extensible_is_present {
            self.encode_length(buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        } else if 0 == size.constraint.effective_value(value.len()).into_inner() {
            // NO-OP
        } else if size.constraint.range() == Some(1) && size.constraint.as_start() <= Some(&2) {
            // ITU-T X.691 (02/2021) §17 NOTE: Octet strings of fixed length less than or equal to two octets are not octet-aligned.
            // All other octet strings are octet-aligned in the ALIGNED variant.
            self.encode_length(buffer, value.len(), Some(size), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        } else {
            if size.constraint.range() == Some(1) {
                self.pad_to_alignment(buffer);
            }
            self.encode_string_length(buffer, true, value.len(), Some(size), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        }

        Ok(())
    }

    fn encode_integer_into_buffer<I: IntegerType>(
        &mut self,
        constraints: Constraints,
        value: &I,
        buffer: &mut BitString,
    ) -> Result<()> {
        let is_extended_value = self.encode_extensible_bit(&constraints, buffer, || {
            constraints.value().map_or(false, |value_range| {
                value_range.extensible.is_some() && value_range.constraint.in_bound(value)
            })
        });

        let value_range = if is_extended_value || constraints.value().is_none() {
            let (bytes, needed) = value.to_signed_bytes_be();
            self.encode_length(buffer, needed, constraints.size(), |range| {
                Ok(BitString::from_slice(&bytes.as_ref()[..needed][range]))
            })?;
            return Ok(());
        } else {
            // Safe to unwrap because we checked for None above
            constraints.value().unwrap()
        };

        if !value_range.constraint.in_bound(value) && !is_extended_value {
            return Err(Error::value_constraint_not_satisfied(
                value.to_bigint().unwrap_or_default(),
                &value_range.constraint,
                self.codec(),
            ));
        }

        let effective_range = value_range
            .constraint
            .effective_value(value.to_i128().ok_or_else(|| {
                Error::integer_type_conversion_failed(
                    "Value too large for i128 type - outside of type constraint".to_string(),
                    self.codec(),
                )
            })?);
        let unsigned_ref;
        let signed_ref;
        let needed: usize;
        let bytes = match &effective_range {
            either::Left(offset) => {
                (unsigned_ref, needed) = offset.to_unsigned_bytes_be();
                unsigned_ref.as_ref()
            }
            either::Right(value) => {
                (signed_ref, needed) = value.to_signed_bytes_be();
                signed_ref.as_ref()
            }
        };

        let effective_value: i128 = value_range
            .constraint
            .effective_value(value.to_i128().ok_or_else(|| {
                Error::integer_type_conversion_failed(
                    "Value too large for i128 type - outside of type constraint".to_string(),
                    self.codec(),
                )
            })?)
            .either_into();

        const K64: i128 = SIXTY_FOUR_K as i128;
        const OVER_K64: i128 = K64 + 1;

        if let Some(range) = value_range.constraint.range() {
            match (self.options.aligned, range) {
                (true, 256) => {
                    self.pad_to_alignment(buffer);
                    self.encode_non_negative_binary_integer(buffer, range, &bytes[..needed])
                }
                (true, 257..=K64) => {
                    self.pad_to_alignment(buffer);
                    self.encode_non_negative_binary_integer(buffer, K64, &bytes[..needed]);
                }
                (true, OVER_K64..) => {
                    let range_len_in_bytes =
                        num_integer::div_ceil(crate::num::log2(range), 8) as i128;

                    if effective_value == 0 {
                        self.encode_non_negative_binary_integer(
                            &mut *buffer,
                            range_len_in_bytes,
                            &[0],
                        );
                        self.pad_to_alignment(&mut *buffer);
                        self.encode_non_negative_binary_integer(
                            &mut *buffer,
                            255,
                            &bytes[..needed],
                        );
                    } else {
                        let range_value_in_bytes =
                            num_integer::div_ceil(crate::num::log2(effective_value + 1), 8) as i128;
                        self.encode_non_negative_binary_integer(
                            buffer,
                            range_len_in_bytes,
                            &(range_value_in_bytes - 1).to_be_bytes(),
                        );
                        self.pad_to_alignment(&mut *buffer);
                        self.encode_non_negative_binary_integer(
                            &mut *buffer,
                            crate::bits::range_from_len(range_value_in_bytes as u32 * 8),
                            &bytes[..needed],
                        );
                    }
                }
                (_, _) => self.encode_non_negative_binary_integer(buffer, range, &bytes[..needed]),
            }
        } else {
            self.encode_length(buffer, needed, <_>::default(), |range| {
                Ok(BitString::from_slice(&bytes[..needed][range]))
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
        let total_bits = crate::num::log2(range) as usize;
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

impl<const RFC: usize, const EFC: usize> crate::Encoder<'_> for Encoder<RFC, EFC> {
    type Ok = ();
    type Error = Error;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder<R, E>;

    fn codec(&self) -> crate::Codec {
        Self::codec(self)
    }

    fn encode_any(&mut self, tag: Tag, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), &value.contents)
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let bit_string_length = value.len();
        let extensible_is_present = self.encode_extensible_bit(&constraints, &mut buffer, || {
            constraints.size().map_or(false, |size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&bit_string_length)
            })
        });
        let size = constraints.size();

        if extensible_is_present || size.is_none() {
            self.encode_length(&mut buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        } else if size.and_then(|size| size.constraint.range()) == Some(0) {
            // NO-OP
        } else if size.map_or(false, |size| {
            size.constraint.range() == Some(1) && size.constraint.as_start() <= Some(&16)
        }) {
            // ITU-T X.691 (02/2021) §16: Bitstrings constrained to a fixed length less than or equal to 16 bits
            // do not cause octet alignment. Larger bitstrings are octet-aligned in the ALIGNED variant.
            self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        } else {
            if size.and_then(|size| size.constraint.range()) == Some(1) {
                self.pad_to_alignment(&mut buffer);
            }
            self.encode_string_length(
                &mut buffer,
                true,
                value.len(),
                constraints.size(),
                |range| Ok(BitString::from(&value[range])),
            )?;
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.extend(tag, value);
        Ok(())
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
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

    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &I,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::new();
        self.encode_integer_into_buffer(constraints, value, &mut buffer)?;
        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_null(&mut self, _tag: Tag) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        let mut encoder = crate::der::enc::Encoder::new(crate::der::enc::EncoderOptions::der());
        let der = encoder.object_identifier_as_bytes(oid)?;
        self.encode_octet_string(tag, Constraints::default(), &der)
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
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
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), value)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), &crate::der::encode(value)?)
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), &crate::der::encode(value)?)
    }

    fn encode_date(&mut self, tag: Tag, value: &types::Date) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), &crate::der::encode(value)?)
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let options = self.options;

        self.encode_extensible_bit(&constraints, &mut buffer, || {
            constraints.size().map_or(false, |size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&values.len())
            })
        });
        let extension_bits = buffer.clone();

        self.encode_length(&mut buffer, values.len(), constraints.size(), |range| {
            let mut buffer = BitString::default();
            let mut first_round = true;
            for value in &values[range] {
                let mut encoder = Self::new(options);
                if first_round {
                    encoder.parent_output_length = Some(extension_bits.len());
                    first_round = false;
                }
                E::encode(value, &mut encoder)?;
                buffer.extend(encoder.bitstring_output());
            }
            Ok(buffer)
        })?;

        self.extend(tag, &buffer);

        Ok(())
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &values.to_vec(), constraints)
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        if V::TAG == Tag::EOC {
            value.encode(self)
        } else {
            value.encode_with_tag(self, tag)
        }
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, true);
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, true);
        value.encode_with_tag(self, tag)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, true);
        value.encode_with_tag_and_constraints(self, tag, constraints)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, false);
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, false);
        Ok(())
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let mut encoder = self.new_sequence_encoder::<RL, EL, C>();
        (encoder_scope)(&mut encoder)?;
        self.encode_constructed::<RL, EL, C>(tag, encoder)
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let mut set = self.new_set_encoder::<RL, EL, C>();

        (encoder_scope)(&mut set)?;

        self.encode_constructed::<RL, EL, C>(tag, set)
    }

    fn encode_choice<E: Encode + crate::types::Choice>(
        &mut self,
        constraints: Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::new();

        let is_root_extension = crate::types::TagTree::tag_contains(&tag, E::VARIANTS);
        self.encode_extensible_bit(&constraints, &mut buffer, || is_root_extension);
        let variants = crate::types::variants::Variants::from_static(if is_root_extension {
            E::VARIANTS
        } else {
            E::EXTENDED_VARIANTS.unwrap_or(&[])
        });

        let index = variants
            .iter()
            .enumerate()
            .find_map(|(i, &variant_tag)| (tag == variant_tag).then_some(i))
            .ok_or_else(|| Error::variant_not_in_choice(self.codec()))?;

        let bounds = if is_root_extension {
            let variance = variants.len();
            debug_assert!(variance > 0);
            if variance == 1 {
                None
            } else {
                Some(Some(variance))
            }
        } else {
            Some(None)
        };

        let mut choice_encoder = Self::new(self.options.without_set_encoding());
        // Extensibility and index encoding size must be noted for byte alignment
        let mut choice_bits_len = 0;
        if E::EXTENDED_VARIANTS.is_some() && self.options.aligned {
            choice_bits_len += 1;
        }
        choice_bits_len += if let Some(Some(variance)) = bounds {
            crate::num::log2(variance as i128) as usize
        } else {
            0
        };

        choice_encoder.parent_output_length = Some(choice_bits_len);
        let _tag = (encode_fn)(&mut choice_encoder)?;

        match (index, bounds) {
            (index, Some(Some(_))) => {
                self.encode_integer_into_buffer::<usize>(
                    E::VARIANCE_CONSTRAINT,
                    &index,
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
                self.encode_octet_string_into_buffer(Constraints::default(), &output, &mut buffer)?;
            }
            (_, None) => {
                buffer.extend(choice_encoder.output);
            }
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
        if value.is_present() {
            E::encode_with_tag_and_constraints(&value, &mut encoder, tag, constraints)?;
            self.extension_fields[self.extension_bitfield.0] = Some(encoder.output());
            self.set_extension_presence(true);
        } else {
            self.extension_fields[self.extension_bitfield.0] = None;
            self.set_extension_presence(false);
        }
        Ok(())
    }

    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed<RL, EL>,
    {
        let Some(value) = value else {
            self.extension_fields[self.extension_bitfield.0] = None;
            self.set_extension_presence(false);
            return Ok(());
        };
        let mut encoder = self.new_sequence_encoder::<RL, EL, E>();
        encoder.is_extension_sequence = true;
        encoder.number_optional_default_fields = E::FIELDS.number_of_optional_and_default_fields();
        value.encode(&mut encoder)?;
        let output = encoder.output();

        self.extension_fields[self.extension_bitfield.0] = Some(output);
        self.set_extension_presence(true);
        Ok(())
    }
}

#[derive(Debug)]
enum Input<'input> {
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

impl From<bool> for Input<'_> {
    fn from(value: bool) -> Self {
        Self::Bit(value)
    }
}

impl From<u8> for Input<'_> {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::macros::{constraints, value_constraint};
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
        let encoder = Encoder::<0, 0>::new(EncoderOptions::unaligned());
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
        let mut encoder = Encoder::<0, 0>::new(EncoderOptions::unaligned());
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
            const CONSTRAINTS: Constraints = constraints!(value_constraint!(end: 65535));
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag_and_constraints<'b, E: crate::Encoder<'b>>(
                &self,
                encoder: &mut E,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer::<i128>(tag, constraints, &self.0.into())
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
        let mut encoder = Encoder::<0, 0>::new(EncoderOptions::unaligned());
        const CONSTRAINT_1: Constraints = constraints!(value_constraint!(start: -1));
        encoder
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_1, &4096.into())
            .unwrap();

        assert_eq!(&[2, 0b00010000, 1], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        const CONSTRAINT_2: Constraints = constraints!(value_constraint!(start: 1));
        encoder
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_2, &127.into())
            .unwrap();
        assert_eq!(&[1, 0b01111110], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        const CONSTRAINT_3: Constraints = constraints!(value_constraint!(start: 0));
        encoder
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_3, &128.into())
            .unwrap();
        assert_eq!(&[1, 0b10000000], &*encoder.output.into_vec());
    }

    #[track_caller]
    fn assert_encode<T: Encode>(options: EncoderOptions, value: T, expected: &[u8]) {
        let mut encoder = Encoder::<0, 0>::new(options);
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
    fn constrained_visible_string() {
        use crate::{types::VisibleString, AsnType};

        #[derive(AsnType, Encode, Clone, PartialEq)]
        #[rasn(delegate, size("1..=3", extensible))]
        #[rasn(crate_root = "crate")]
        struct ExtSizeRangeString(pub VisibleString);

        // Extensible VisibleString with size range constraint
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeString(VisibleString::try_from("abc").unwrap()),
            &[88, 113, 99],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeString(VisibleString::try_from("abc").unwrap()),
            &[64, 97, 98, 99],
        );
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeString(VisibleString::try_from("abcd").unwrap()),
            &[130, 97, 197, 143, 32],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeString(VisibleString::try_from("abcd").unwrap()),
            &[128, 4, 97, 98, 99, 100],
        );
    }

    #[test]
    fn constrained_bit_string() {
        use crate::AsnType;

        #[derive(AsnType, Encode, Clone, PartialEq)]
        #[rasn(delegate, size("1..=4", extensible))]
        #[rasn(crate_root = "crate")]
        struct ExtSizeRangeBitStr(pub BitString);

        #[derive(AsnType, Encode, Clone, PartialEq)]
        #[rasn(delegate, size("2", extensible))]
        #[rasn(crate_root = "crate")]
        struct ExtStrictSizeBitStr(pub BitString);

        // Extensible BIT STRING with size range constraint
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeBitStr(BitString::from_iter([true].iter())),
            &[16],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeBitStr(BitString::from_iter([true].iter())),
            &[0, 128],
        );
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeBitStr(BitString::from_iter(
                [true, false, true, false, true, true].iter(),
            )),
            &[131, 86],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeBitStr(BitString::from_iter(
                [true, false, true, false, true, true].iter(),
            )),
            &[128, 6, 172],
        );
        // Edge case ITU-T X.691 (02/2021) §16 Note: strictly sized BIT STRINGs shorter than 17 bits
        assert_encode(
            EncoderOptions::unaligned(),
            ExtStrictSizeBitStr(BitString::from_iter([true, true].iter())),
            &[96],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtStrictSizeBitStr(BitString::from_iter([true, true].iter())),
            &[96],
        );
        assert_encode(
            EncoderOptions::unaligned(),
            ExtStrictSizeBitStr(BitString::from_iter([true, true, true].iter())),
            &[129, 240],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtStrictSizeBitStr(BitString::from_iter([true, true, true].iter())),
            &[128, 3, 224],
        );
    }

    #[test]
    fn constrained_octet_string() {
        use crate::{types::OctetString, AsnType};

        #[derive(AsnType, Encode, Clone, PartialEq)]
        #[rasn(delegate, size("1..=3", extensible))]
        #[rasn(crate_root = "crate")]
        struct ExtSizeRangeOctetStr(pub OctetString);

        // Extensible OCTET STRING with size range constraint
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeOctetStr(OctetString::copy_from_slice(&[1, 2])),
            &[32, 32, 64],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeOctetStr(OctetString::copy_from_slice(&[1, 2])),
            &[32, 1, 2],
        );
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeOctetStr(OctetString::copy_from_slice(&[1, 2, 3, 4])),
            &[130, 0, 129, 1, 130, 0],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeOctetStr(OctetString::copy_from_slice(&[1, 2, 3, 4])),
            &[128, 4, 1, 2, 3, 4],
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
