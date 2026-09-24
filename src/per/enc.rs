//! Encoding Rust structures into Packed Encoding Rules data.

use alloc::{string::ToString, vec::Vec};

use super::{
    FOURTY_EIGHT_K, LARGE_UNSIGNED_CONSTRAINT, SIXTEEN_K, SIXTY_FOUR_K, SMALL_UNSIGNED_CONSTRAINT,
    THIRTY_TWO_K,
};
use crate::{
    Encode,
    types::{
        self, BitString, Constraints, Enumerated, Identifier, IntegerType, Tag,
        constraints::{self, Extensible, Size},
        strings::{BitStr, CharacterAlphabet, StaticPermittedAlphabet},
    },
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
    /// Scratch buffer reused across encode_* calls to avoid repeated heap allocations.
    /// Each method takes ownership via `mem::take`, clears it, uses it, then puts it back.
    work: BitString,
    /// Preamble bits already present in `output` before this encoder's own field data begins.
    /// Set when a parent encoder moves its buffer into this child to avoid a separate allocation.
    /// Subtracted from `number_optional_default_fields` in `output_length` to avoid double-counting.
    preamble_pre_reserved: usize,
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
            work: BitString::new(),
            preamble_pre_reserved: 0,
            set_output: <_>::default(),
            number_optional_default_fields: 0,
            root_bitfield: (0, [(false, Tag::new_private(0)); RCL]),
            extension_bitfield: (0, [false; ECL]),
            is_extension_sequence: <_>::default(),
            extension_fields: [(); ECL].map(|_| None),
            parent_output_length: <_>::default(),
        }
    }

    /// Constructs a new encoder from the provided options, reusing the given
    /// bit buffer's allocation. The buffer is cleared before encoding begins.
    pub fn new_with_output(options: EncoderOptions, mut output: BitString) -> Self {
        output.clear();
        Self {
            options,
            output,
            work: BitString::new(),
            preamble_pre_reserved: 0,
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

    /// Consumes the encoder and returns the octet-aligned output, reusing the
    /// internal buffer's allocation instead of copying it.
    pub fn output_into_vec(mut self) -> Vec<u8> {
        let mut output = self.bitstring_output();
        Self::force_pad_to_alignment(&mut output);
        output.into_vec()
    }

    /// Returns the bit level output for the encoder.
    fn bitstring_output(&mut self) -> BitString {
        if self.options.set_encoding {
            let mut output = BitString::new();
            for value in self.set_output.values() {
                crate::bits::extend_bitstring(&mut output, value);
            }
            output
        } else {
            core::mem::take(&mut self.output)
        }
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
        output_length += usize::from(self.is_extension_sequence);
        // When the parent's buffer was moved into this encoder, preamble bits are already
        // present in self.output. Subtract to avoid double-counting with number_optional_default_fields.
        output_length += self
            .number_optional_default_fields
            .saturating_sub(self.preamble_pre_reserved);
        output_length += self.parent_output_length.unwrap_or_default();

        if self.options.set_encoding {
            output_length += self
                .set_output
                .values()
                .map(bitvec::vec::BitVec::len)
                .sum::<usize>();
        }

        output_length
    }

    fn pad_to_alignment(&self, buffer: &mut BitString) {
        if self.options.aligned {
            let mut output_length = self.output_length();
            output_length += buffer.len();
            if !output_length.is_multiple_of(8) {
                for _ in 0..(8 - output_length % 8) {
                    buffer.push(false);
                }
            }
        }
    }

    fn force_pad_to_alignment(buffer: &mut BitString) {
        const BYTE_WIDTH: usize = 8;
        if !buffer.len().is_multiple_of(BYTE_WIDTH) {
            buffer.resize(buffer.len().next_multiple_of(BYTE_WIDTH), false);
            debug_assert_eq!(0, buffer.len() % 8);
        }
    }

    fn encode_extensible_bit(
        &mut self,
        constraints: &Constraints,
        buffer: &mut BitString,
        extensible_condition: impl FnOnce() -> bool,
    ) -> bool {
        if constraints.extensible() {
            let is_in_constraints = !(extensible_condition)();
            buffer.push(is_in_constraints);
            is_in_constraints
        } else {
            <_>::default()
        }
    }

    #[allow(clippy::too_many_lines)]
    fn encode_known_multiplier_string<S: StaticPermittedAlphabet>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &S,
    ) -> Result<()> {
        use crate::types::constraints::Bounded;
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        let string_length = value.len();

        let is_extended_value = self.encode_extensible_bit(constraints, &mut work, || {
            constraints.size().is_some_and(|size_constraint| {
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
                } if size.constraint.range().unwrap() * width > 16 => true,
                Bounded::Single(max) if max * width > 16 => {
                    self.pad_to_alignment(&mut work);
                    true
                }
                Bounded::Range {
                    start: None,
                    end: Some(max),
                } if max * width > 16 => {
                    self.pad_to_alignment(&mut work);
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        // ITU-T X.691 (02/2021) §30.5: each character is written as a fixed-width
        // value or alphabet index, straight into the buffer.
        let alphabet = CharacterAlphabet::new::<S>(
            constraints
                .permitted_alphabet()
                .map(|alphabet| alphabet.constraint.as_inner()),
            self.options.aligned,
        );
        let width = alphabet.width();
        let codec = self.codec();
        self.encode_string_length(
            &mut work,
            is_large_string,
            string_length,
            is_extended_value
                .then(|| -> Extensible<Size> { <_>::default() })
                .as_ref()
                .or(constraints.size()),
            |buf, range| {
                let mut appender = crate::bits::BitAppender::new(buf, range.len() * width);
                for ch in value.chars().skip(range.start).take(range.len()) {
                    let code = alphabet
                        .encode(ch)
                        .map_err(|e| Error::alphabet_constraint_not_satisfied(e, codec))?;
                    appender.push(code, width);
                }
                Ok(())
            },
        )?;

        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn character_width(&self, width: usize) -> usize {
        if self.options.aligned {
            {
                if width.is_power_of_two() {
                    width
                } else {
                    width.next_power_of_two()
                }
            }
        } else {
            width
        }
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
        let extensions_present =
            C::IS_EXTENSIBLE && encoder.extension_fields.iter().any(Option::is_some);
        let (needed, option_bitfield) = if encoder.options.set_encoding {
            // In set encoding, tags must be unique so we sort them to canonical order for preamble
            encoder.root_bitfield.1.sort_by_key(|(_, tag1)| *tag1);
            encoder.root_bitfield
        } else {
            encoder.root_bitfield
        };
        debug_assert!(C::FIELDS.number_of_optional_and_default_fields() == needed);

        // Fast path: non-SET parent, no extensions present.
        // Pre-reserve preamble bits directly in self.output, then append child output.
        if !self.options.set_encoding && !extensions_present {
            let preamble_bits = (C::IS_EXTENSIBLE as usize) + needed;
            let preamble_start = self.output.len();
            if preamble_bits > 0 {
                // All bits initialise to false; only set the true presence bits.
                self.output.resize(preamble_start + preamble_bits, false);
                // extensibility bit (index 0 when IS_EXTENSIBLE) stays false (not present).
                let presence_start = preamble_start + (C::IS_EXTENSIBLE as usize);
                for (i, (bit, _)) in option_bitfield[..needed].iter().enumerate() {
                    if *bit {
                        self.output.set(presence_start + i, true);
                    }
                }
            }
            let out = encoder.bitstring_output();
            crate::bits::extend_bitstring(&mut self.output, &out);
            return Ok(());
        }

        // Slow path: SET encoding or extensions present — use an intermediate buffer.
        let required_present = C::FIELDS.has_required_field();
        let mut buffer = BitString::with_capacity(core::mem::size_of::<C>());
        if C::IS_EXTENSIBLE {
            buffer.push(extensions_present);
        }
        if needed > 0 || C::IS_EXTENSIBLE {
            for (bit, _tag) in &option_bitfield[..needed] {
                buffer.push(*bit);
            }
        }
        if option_bitfield[..needed].iter().any(|(bit, _tag)| *bit) || required_present {
            let out = encoder.bitstring_output();
            crate::bits::extend_bitstring(&mut buffer, &out);
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
            self.encode_length(&mut buffer, field.len(), <_>::default(), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &field[range]);
                Ok(())
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

    /// Encodes the length determinant for `length` items and then the items
    /// themselves. `encode_fn` writes the items in `range` straight into the
    /// buffer it is handed; it is called once per fragment for lengths of 16K
    /// and above.
    fn encode_string_length(
        &self,
        buffer: &mut BitString,
        is_large_string: bool,
        length: usize,
        constraints: Option<&Extensible<constraints::Size>>,
        mut encode_fn: impl FnMut(&mut BitString, core::ops::Range<usize>) -> Result<()>,
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
                    (encode_fn)(buffer, 0..length)
                } else if range <= SIXTY_FOUR_K as usize {
                    let effective_length = constraints.effective_value(length).into_inner();
                    let range = if self.options.aligned && range > 256 {
                        {
                            let range = crate::num::log2(range as i128);
                            crate::bits::range_from_len(if range.is_power_of_two() {
                                range
                            } else {
                                range.next_power_of_two()
                            })
                        }
                    } else {
                        range as i128
                    };
                    self.encode_non_negative_binary_integer(
                        buffer,
                        range,
                        effective_length as u128,
                    );
                    if is_large_string {
                        self.pad_to_alignment(buffer);
                    }

                    (encode_fn)(buffer, 0..length)
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
        encode_fn: impl FnMut(&mut BitString, core::ops::Range<usize>) -> Result<()>,
    ) -> Result<()> {
        self.encode_string_length(buffer, false, length, constraints, encode_fn)
    }

    fn encode_unconstrained_length(
        &self,
        buffer: &mut BitString,
        mut length: usize,
        min: Option<usize>,
        mut encode_fn: impl FnMut(&mut BitString, core::ops::Range<usize>) -> Result<()>,
    ) -> Result<()> {
        let mut min = min.unwrap_or_default();

        self.pad_to_alignment(&mut *buffer);
        if length <= 127 {
            crate::bits::extend_bitstring_from_bytes(buffer, &[length as u8]);
            (encode_fn)(buffer, min..min + length)?;
        } else if length < SIXTEEN_K.into() {
            const SIXTEENTH_BIT: u16 = 0x8000;
            crate::bits::extend_bitstring_from_bytes(
                buffer,
                &(SIXTEENTH_BIT | length as u16).to_be_bytes(),
            );
            (encode_fn)(buffer, min..min + length)?;
        } else {
            // ITU-T X.691 (02/2021) §11.9.3.8: a length of 16K or more is encoded as a
            // series of fragments, each a multiple of 16K items, and is always terminated
            // by a final length determinant (possibly zero) for the remaining items.
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
                // Every length determinant is octet-aligned in the ALIGNED variant; a
                // no-op for the first fragment, which was aligned above.
                self.pad_to_alignment(&mut *buffer);
                crate::bits::extend_bitstring_from_bytes(
                    buffer,
                    &[FRAGMENT_MARKER | fragment_index],
                );

                (encode_fn)(buffer, min..min + amount)?;
                min += amount;
                // When the fragments consume the whole value, the next iteration
                // emits the mandatory zero-length terminator through the `_` arm.
                length -= amount;
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
                crate::bits::extend_bitstring(buffer, bits);
            }
            Input::Bit(bit) => {
                buffer.push(bit);
            }
            Input::Byte(byte) => {
                buffer.store_be(byte);
            }
            Input::Bytes(bytes) => {
                crate::bits::extend_bitstring_from_bytes(buffer, bytes);
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
            constraints.size().is_some_and(|size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&octet_string_length)
            })
        });
        let Some(size) = constraints.size() else {
            return self.encode_length(buffer, value.len(), <_>::default(), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &value[range]);
                Ok(())
            });
        };

        if extensible_is_present {
            self.encode_length(buffer, value.len(), <_>::default(), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &value[range]);
                Ok(())
            })?;
        } else if Some(0) == size.constraint.range() {
            // ITU-T X.691 (02/2021) §11.9.3.3: If "n" is zero there shall be no further addition to the field-list.
        } else if size.constraint.range() == Some(1) && size.constraint.as_start() <= Some(&2) {
            // ITU-T X.691 (02/2021) §17 NOTE: Octet strings of fixed length less than or equal to two octets are not octet-aligned.
            // All other octet strings are octet-aligned in the ALIGNED variant.
            self.encode_length(buffer, value.len(), Some(size), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &value[range]);
                Ok(())
            })?;
        } else {
            if size.constraint.range() == Some(1) {
                self.pad_to_alignment(buffer);
            }
            self.encode_string_length(buffer, true, value.len(), Some(size), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &value[range]);
                Ok(())
            })?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn encode_integer_into_buffer<I: IntegerType>(
        &mut self,
        constraints: Constraints,
        value: &I,
        buffer: &mut BitString,
    ) -> Result<()> {
        let is_extended_value = self.encode_extensible_bit(&constraints, buffer, || {
            constraints.value().is_some_and(|value_range| {
                value_range.extensible.is_some() && value_range.constraint.in_bound(value)
            })
        });

        let value_range = if is_extended_value || constraints.value().is_none() {
            let (bytes, needed) = value.to_signed_bytes_be();
            self.encode_length(buffer, needed, constraints.size(), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &bytes.as_ref()[..needed][range]);
                Ok(())
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

        let value_i128 = value.to_i128().ok_or_else(|| {
            Error::integer_type_conversion_failed(
                "Value too large for i128 type - outside of type constraint".to_string(),
                self.codec(),
            )
        })?;
        let effective_range = value_range.constraint.effective_value(value_i128);

        const K64: i128 = SIXTY_FOUR_K as i128;
        const OVER_K64: i128 = K64 + 1;

        let Some(range) = value_range.constraint.range() else {
            // Semi-constrained: a length-prefixed octet string of the offset
            // from the lower bound, or of the value itself without one.
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
            return self.encode_length(buffer, needed, <_>::default(), |buf, range| {
                crate::bits::extend_bitstring_from_bytes(buf, &bytes[..needed][range]);
                Ok(())
            });
        };

        // Both bounds are known, so the encoding is the non-negative offset
        // from the lower bound (or nothing at all for a single value).
        let offset: i128 = effective_range.either_into();
        let offset = offset as u128;
        match (self.options.aligned, range) {
            (true, 256) => {
                self.pad_to_alignment(buffer);
                crate::bits::push_bits(buffer, offset, 8);
            }
            (true, 257..=K64) => {
                self.pad_to_alignment(buffer);
                crate::bits::push_bits(buffer, offset, 16);
            }
            (true, OVER_K64..) => {
                // A length determinant for the octets of the offset, then the
                // octet-aligned offset itself in the fewest whole octets.
                let range_octets = crate::num::log2(range).div_ceil(8);
                let offset_octets = if offset == 0 {
                    1
                } else {
                    crate::num::log2(offset as i128 + 1).div_ceil(8)
                };
                self.encode_non_negative_binary_integer(
                    buffer,
                    i128::from(range_octets),
                    u128::from(offset_octets - 1),
                );
                self.pad_to_alignment(buffer);
                crate::bits::push_bits(buffer, offset, offset_octets as usize * 8);
            }
            (_, _) => {
                let bits = if I::WIDTH <= 16 && range == (1i128 << I::WIDTH) {
                    I::WIDTH as usize
                } else {
                    crate::num::log2(range) as usize
                };
                crate::bits::push_bits(buffer, offset, bits);
            }
        }

        Ok(())
    }

    /// Writes `value` as a constrained whole number with the given `range`
    /// (ITU-T X.691 §11.5), which takes `log2(range)` bits.
    fn encode_non_negative_binary_integer(&self, buffer: &mut BitString, range: i128, value: u128) {
        crate::bits::push_bits(buffer, value, crate::num::log2(range) as usize);
    }
}

impl<const RFC: usize, const EFC: usize> crate::Encoder<'_> for Encoder<RFC, EFC> {
    type Ok = ();
    type Error = Error;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder<R, E>;

    fn codec(&self) -> crate::Codec {
        Self::codec(self)
    }

    fn encode_any(
        &mut self,
        tag: Tag,
        value: &types::Any,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            value.as_bytes(),
            Identifier::EMPTY,
        )
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitStr,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        let bit_string_length = value.len();
        let extensible_is_present = self.encode_extensible_bit(&constraints, &mut work, || {
            constraints.size().is_some_and(|size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&bit_string_length)
            })
        });
        let size = constraints.size();

        if extensible_is_present || size.is_none() {
            self.encode_length(&mut work, value.len(), <_>::default(), |buf, range| {
                crate::bits::extend_bitstring(buf, &value[range]);
                Ok(())
            })?;
        } else if size.and_then(|size| size.constraint.range()) == Some(0) {
            // NO-OP
        } else if size.is_some_and(|size| {
            size.constraint.range() == Some(1) && size.constraint.as_start() <= Some(&16)
        }) {
            // ITU-T X.691 (02/2021) §16: Bitstrings constrained to a fixed length less than or equal to 16 bits
            // do not cause octet alignment. Larger bitstrings are octet-aligned in the ALIGNED variant.
            self.encode_length(&mut work, value.len(), constraints.size(), |buf, range| {
                crate::bits::extend_bitstring(buf, &value[range]);
                Ok(())
            })?;
        } else {
            if size.and_then(|size| size.constraint.range()) == Some(1) {
                self.pad_to_alignment(&mut work);
            }
            self.encode_string_length(
                &mut work,
                true,
                value.len(),
                constraints.size(),
                |buf, range| {
                    crate::bits::extend_bitstring(buf, &value[range]);
                    Ok(())
                },
            )?;
        }

        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_bool(
        &mut self,
        tag: Tag,
        value: bool,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.extend(tag, value);
        Ok(())
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        let index = value.enumeration_index();
        if E::EXTENDED_VARIANTS.is_some() {
            work.push(value.is_extended_variant());
        }

        if value.is_extended_variant() {
            self.encode_normally_small_integer(index, &mut work)?;
        } else {
            self.encode_non_negative_binary_integer(
                &mut work,
                E::variance() as i128,
                index as u128,
            );
        }

        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &I,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        self.encode_integer_into_buffer(constraints, value, &mut work)?;
        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: Constraints,
        _: &R,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::real_not_supported(self.codec()))
    }

    fn encode_null(&mut self, _tag: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        oid: &[u32],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buf = Vec::new();
        crate::ber::enc::object_identifier_as_bytes(oid, &mut buf)?;
        self.encode_octet_string(tag, Constraints::default(), &buf, Identifier::EMPTY)
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        self.encode_octet_string_into_buffer(constraints, value, &mut work)?;
        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::GeneralString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), value, Identifier::EMPTY)
    }

    fn encode_graphic_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::GraphicString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, Constraints::default(), value, Identifier::EMPTY)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::TeletexString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multiplier_string(tag, &constraints, value)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &str,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            value.as_bytes(),
            Identifier::EMPTY,
        )
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::encode(value)?,
            Identifier::EMPTY,
        )
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::encode(value)?,
            Identifier::EMPTY,
        )
    }

    fn encode_date(
        &mut self,
        tag: Tag,
        value: &types::Date,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::encode(value)?,
            Identifier::EMPTY,
        )
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();
        let options = self.options.without_set_encoding();
        // Absolute bit position at which `work` will be appended, so that the
        // elements align relative to the start of the whole encoding.
        let parent_output_length = Some(self.output_length());

        self.encode_extensible_bit(&constraints, &mut work, || {
            constraints.size().is_some_and(|size_constraint| {
                size_constraint.extensible.is_some()
                    && size_constraint.constraint.contains(&values.len())
            })
        });

        let mut element_work = BitString::new();
        self.encode_length(
            &mut work,
            values.len(),
            constraints.size(),
            |buffer, range| {
                // Lend `buffer` to a child encoder so the elements are written straight
                // into it, rather than materialised in an intermediate bit string.
                let mut encoder = Encoder::<0, 0> {
                    options,
                    output: core::mem::take(buffer),
                    work: core::mem::take(&mut element_work),
                    preamble_pre_reserved: 0,
                    set_output: <_>::default(),
                    number_optional_default_fields: 0,
                    root_bitfield: (0, []),
                    extension_bitfield: (0, []),
                    is_extension_sequence: false,
                    extension_fields: [],
                    parent_output_length,
                };
                for value in &values[range] {
                    E::encode(value, &mut encoder)?;
                }
                element_work = encoder.work;
                *buffer = encoder.output;
                Ok(())
            },
        )?;

        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &values.to_vec(), constraints, Identifier::EMPTY)
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if V::IS_CHOICE {
            value.encode(self)
        } else {
            value.encode_with_tag(self, tag)
        }
    }

    fn encode_some<E: Encode>(
        &mut self,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, true);
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, true);
        value.encode_with_tag(self, tag)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, true);
        value.encode_with_tag_and_constraints(self, tag, constraints, Identifier::EMPTY)
    }

    fn encode_none<E: Encode>(&mut self, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, false);
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, false);
        Ok(())
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        // Fast path: non-extensible, non-SET sequences.
        // Move self.output into the child encoder so its fields are written directly into the
        // parent's buffer, avoiding a separate allocation and the subsequent bit-copy.
        if !self.options.set_encoding && !C::IS_EXTENSIBLE {
            let needed = C::FIELDS.number_of_optional_and_default_fields();
            let preamble_start = self.output.len();
            if needed > 0 {
                self.output.resize(preamble_start + needed, false);
            }
            let mut child = Encoder::<RL, EL> {
                options: self.options,
                output: core::mem::take(&mut self.output),
                work: core::mem::take(&mut self.work),
                preamble_pre_reserved: needed,
                set_output: <_>::default(),
                number_optional_default_fields: needed,
                root_bitfield: (0, [(false, Tag::new_private(0)); RL]),
                extension_bitfield: (0, [false; EL]),
                is_extension_sequence: false,
                extension_fields: [(); EL].map(|_| None),
                parent_output_length: self.parent_output_length,
            };
            (encoder_scope)(&mut child)?;
            // Move the buffers back; reclaim any grown work allocation from the child.
            self.work = core::mem::take(&mut child.work);
            self.output = core::mem::take(&mut child.output);
            for (i, (bit, _)) in child.root_bitfield.1[..needed].iter().enumerate() {
                if *bit {
                    self.output.set(preamble_start + i, true);
                }
            }
            return Ok(());
        }

        let mut encoder = self.new_sequence_encoder::<RL, EL, C>();
        (encoder_scope)(&mut encoder)?;
        self.encode_constructed::<RL, EL, C>(tag, encoder)
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: Identifier,
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
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut work = core::mem::take(&mut self.work);
        work.clear();

        let is_root_extension = crate::types::TagTree::tag_contains(&tag, E::VARIANTS);
        self.encode_extensible_bit(&constraints, &mut work, || is_root_extension);
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

        let preceding_bits = self.output_length();
        choice_encoder.parent_output_length = Some(preceding_bits + choice_bits_len);
        let _tag = (encode_fn)(&mut choice_encoder)?;

        match (index, bounds) {
            (index, Some(Some(_))) => {
                self.encode_integer_into_buffer::<usize>(
                    E::VARIANCE_CONSTRAINT,
                    &index,
                    &mut work,
                )?;

                crate::bits::extend_bitstring(&mut work, &choice_encoder.output);
            }
            (index, Some(None)) => {
                self.encode_normally_small_integer(index, &mut work)?;
                let mut output = choice_encoder.output_into_vec();

                if output.is_empty() {
                    output.push(0);
                }
                self.encode_octet_string_into_buffer(Constraints::default(), &output, &mut work)?;
            }
            (_, None) => {
                crate::bits::extend_bitstring(&mut work, &choice_encoder.output);
            }
        }

        self.extend(tag, &work);
        self.work = work;
        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if value.is_present() {
            // Lend the scratch buffer to the child so the extension does not need
            // its own; the encoded bytes are kept until the preamble is known.
            let mut encoder = Encoder::<0, 0>::new(self.options.without_set_encoding());
            encoder.work = core::mem::take(&mut self.work);
            let result = E::encode_with_tag_and_constraints(
                &value,
                &mut encoder,
                tag,
                constraints,
                Identifier::EMPTY,
            );
            self.work = core::mem::take(&mut encoder.work);
            result?;
            self.extension_fields[self.extension_bitfield.0] = Some(encoder.output_into_vec());
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
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed<RL, EL>,
    {
        let Some(value) = value else {
            self.extension_fields[self.extension_bitfield.0] = None;
            self.set_extension_presence(false);
            return Ok(());
        };
        // Must use an owned-buffer encoder here — we need to capture the output as
        // Vec<u8> for storage in extension_fields. Never use the ext fast path.
        let mut encoder = Encoder::<RL, EL>::new(self.options.without_set_encoding());
        encoder.is_extension_sequence = true;
        encoder.number_optional_default_fields = E::FIELDS.number_of_optional_and_default_fields();
        encoder.parent_output_length = Some(self.output_length());
        value.encode(&mut encoder)?;
        let (present_count, presence) = encoder.root_bitfield;
        let out = encoder.output_into_vec();

        let all_absent = if E::FIELDS.has_required_field() {
            false
        } else if present_count > 0 {
            presence[..present_count]
                .iter()
                .all(|(present, _)| !present)
        } else {
            out.iter().all(|&b| b == 0)
        };

        if all_absent {
            self.extension_fields[self.extension_bitfield.0] = None;
            self.set_extension_presence(false);
        } else {
            self.extension_fields[self.extension_bitfield.0] = Some(out);
            self.set_extension_presence(true);
        }
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

    use bitvec::prelude::*;

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
                |_, _| Ok(()),
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
                _: Identifier,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer::<i128>(tag, constraints, &self.0.into(), Identifier::EMPTY)
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
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_1, &4096.into(), Identifier::EMPTY)
            .unwrap();

        assert_eq!(&[2, 0b00010000, 1], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        const CONSTRAINT_2: Constraints = constraints!(value_constraint!(start: 1));
        encoder
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_2, &127.into(), Identifier::EMPTY)
            .unwrap();
        assert_eq!(&[1, 0b01111110], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        const CONSTRAINT_3: Constraints = constraints!(value_constraint!(start: 0));
        encoder
            .encode_integer::<i128>(Tag::INTEGER, CONSTRAINT_3, &128.into(), Identifier::EMPTY)
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
        use crate::{AsnType, types::VisibleString};

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
        use crate::{AsnType, types::OctetString};

        #[derive(AsnType, Encode, Clone, PartialEq)]
        #[rasn(delegate, size("1..=3", extensible))]
        #[rasn(crate_root = "crate")]
        struct ExtSizeRangeOctetStr(pub OctetString);

        // Extensible OCTET STRING with size range constraint
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeOctetStr(OctetString::from_static(&[1, 2])),
            &[32, 32, 64],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeOctetStr(OctetString::from_static(&[1, 2])),
            &[32, 1, 2],
        );
        assert_encode(
            EncoderOptions::unaligned(),
            ExtSizeRangeOctetStr(OctetString::from_static(&[1, 2, 3, 4])),
            &[130, 0, 129, 1, 130, 0],
        );
        assert_encode(
            EncoderOptions::aligned(),
            ExtSizeRangeOctetStr(OctetString::from_static(&[1, 2, 3, 4])),
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
        assert!(
            buffer[SIXTY_FOUR_K as usize + THIRTY_TWO_K as usize + 2..]
                .starts_with(&[0b10000010, 0b10111000])
        );
    }
}
