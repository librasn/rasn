use alloc::{string::ToString, vec::Vec};
use core::cmp::Ordering;

use crate::oer::ranges;
use crate::prelude::{
    Any, BitStr, BmpString, Choice, Constructed, Enumerated, GeneralString, GeneralizedTime,
    Ia5String, NumericString, PrintableString, SetOf, TeletexString, UtcTime, VisibleString,
};
use crate::Codec;
use bitvec::prelude::*;

use crate::types::{
    fields::FieldPresence, BitString, Class, Constraints, Integer, PrimIntBytes, PrimitiveInteger,
    PRIMITIVE_BYTE_WIDTH,
};
use crate::{Encode, Tag};

/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
use crate::error::{CoerEncodeErrorKind, EncodeError, EncodeErrorKind};

pub const ITU_T_X696_OER_EDITION: f32 = 3.0;

/// Options for configuring the [`Encoder`][super::Encoder].
#[derive(Clone, Copy, Debug)]
pub struct EncoderOptions {
    encoding_rules: EncodingRules,
    set_encoding: bool,
}

impl EncoderOptions {
    // Return the default configuration for COER.
    // We reserve the possibility to use OER in the future by using the rules.
    #[must_use]
    pub const fn coer() -> Self {
        Self {
            encoding_rules: EncodingRules::Coer,
            set_encoding: false,
        }
    }
    fn without_set_encoding(mut self) -> Self {
        self.set_encoding = false;
        self
    }
    #[must_use]
    fn current_codec(self) -> Codec {
        match self.encoding_rules {
            EncodingRules::Oer => Codec::Oer,
            EncodingRules::Coer => Codec::Coer,
        }
    }
}
impl Default for EncoderOptions {
    fn default() -> Self {
        Self::coer()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingRules {
    Oer,
    Coer,
}

impl EncodingRules {
    #[must_use]
    pub fn is_coer(self) -> bool {
        matches!(self, Self::Coer)
    }
    #[must_use]
    pub fn is_oer(self) -> bool {
        matches!(self, Self::Oer)
    }
}
impl Default for Encoder {
    fn default() -> Self {
        Self::new(EncoderOptions::coer())
    }
}
/// COER encoder. A subset of OER to provide canonical and unique encoding.  
#[derive(Debug)]
pub struct Encoder {
    options: EncoderOptions,
    output: Vec<u8>,
    set_output: alloc::collections::BTreeMap<Tag, Vec<u8>>,
    field_bitfield: alloc::collections::BTreeMap<Tag, (FieldPresence, bool)>,
    extension_fields: Vec<Vec<u8>>,
    is_extension_sequence: bool,
    parent_output_length: Option<usize>,
}
// ITU-T X.696 8.2.1 Only the following constraints are OER-visible:
// a) non-extensible single value constraints and value range constraints on integer types;
// b) non-extensible single value constraints on real types where the single value is either plus zero or minus zero or
// one of the special real values PLUS-INFINITY, MINUS-INFINITY and NOT-A-NUMBER;
// c) non-extensible size constraints on known-multiplier character string types, octetstring types, and bitstring
// types;
// d) non-extensible property settings constraints on the time type or on the useful and defined time types;
// e) inner type constraints applying OER-visible constraints to real types when used to restrict the mantissa, base,
// or exponent;
// f) inner type constraints applied to CHARACTER STRING or EMBEDDED-PDV types when used to restrict
// the value of the syntaxes component to a single value, or when used to restrict identification to the fixed
// alternative;
// g) contained subtype constraints in which the constraining type carries an OER-visible constraint.

// Tags are encoded only as part of the encoding of a choice type, where the tag indicates
// which alternative of the choice type is the chosen alternative (see 20.1).
impl Encoder {
    // pub fn new(options: EncoderOptions) -> Self {
    #[must_use]
    pub fn new(options: EncoderOptions) -> Self {
        Self {
            options,
            output: <_>::default(),
            set_output: <_>::default(),
            field_bitfield: <_>::default(),
            extension_fields: <_>::default(),
            is_extension_sequence: bool::default(),
            parent_output_length: <_>::default(),
        }
    }
    fn codec(&self) -> Codec {
        self.options.current_codec()
    }

    #[must_use]
    pub fn output(self) -> Vec<u8> {
        self.options
            .set_encoding
            .then(|| {
                self.set_output
                    .values()
                    .flatten()
                    .copied()
                    .collect::<Vec<u8>>()
            })
            .unwrap_or(self.output)
    }
    pub fn set_bit(&mut self, tag: Tag, bit: bool) {
        self.field_bitfield.entry(tag).and_modify(|(_, b)| *b = bit);
    }
    fn extend(&mut self, tag: Tag, mut bytes: Vec<u8>) -> Result<(), EncodeError> {
        match self.output.len().checked_add(bytes.len()) {
            Some(_) => {
                if self.options.set_encoding {
                    self.set_output.insert(tag, bytes);
                } else {
                    self.output.append(&mut bytes);
                }
            }
            _ => Err(EncodeError::length_exceeds_platform_size(self.codec()))?,
        }
        Ok(())
    }
    /// Encode a tag as specified in ITU-T X.696 8.7
    ///
    /// Encoding of the tag is only required when encoding a choice type.
    fn encode_tag(tag: Tag) -> Vec<u8> {
        let mut bv: BitVec<u8, Msb0> = BitVec::with_capacity(8);
        // Encode the tag class
        match tag.class {
            Class::Universal => bv.extend(&[false, false]),
            Class::Application => bv.extend(&[false, true]),
            Class::Context => bv.extend(&[true, false]),
            Class::Private => bv.extend(&[true, true]),
        }
        let mut tag_number = tag.value;
        // Encode the tag number
        if tag_number < 63 {
            for i in (0..6).rev() {
                bv.push(tag_number & (1 << i) != 0);
            }
        } else {
            bv.extend([true; 6].iter());
            // Generate the bits for the tag number
            let mut tag_bits = BitVec::<u8, Msb0>::new();
            while tag_number > 0 {
                tag_bits.push(tag_number & 1 != 0);
                tag_number >>= 1;
            }
            // Add leading zeros if needed to make length a multiple of 7
            while tag_bits.len() % 7 != 0 {
                tag_bits.push(false);
            }
            // Encode the bits in the "big-endian" format, with continuation bits
            for chunk in tag_bits.chunks(7).rev() {
                // 8th bit is continuation marker; true for all but the last octet
                bv.push(true);
                bv.extend(chunk);
            }
            // Correct the 8th bit of the last octet to be false
            let bv_last_8bit = bv.len() - 8;
            bv.replace(bv_last_8bit, false);
            debug_assert!(&bv[2..8].all());
            debug_assert!(&bv[9..16].any());
        }
        bv.into_vec()
    }

    fn encode_unconstrained_enum_index(
        &mut self,
        buffer: &mut Vec<u8>,
        value: isize,
    ) -> Result<(), EncodeError> {
        // Truncated value here with `as` ?
        let (bytes, needed) =
            (value as PrimitiveInteger).needed_as_be_bytes::<PRIMITIVE_BYTE_WIDTH>(true);
        let mut length = u8::try_from(needed).map_err(|err| {
            EncodeError::integer_type_conversion_failed(
                alloc::format!(
                    "Length of length conversion failed when encoding enumerated index.\
                value likely too large: {err}"
                ),
                self.codec(),
            )
        })?;
        if length > 127 {
            // There seems to be an error in standard. It states that enumerated index can be
            // between –2^1015 and 2^1015 – 1, but then it limits the amount of subsequent bytes to 127
            return Err(CoerEncodeErrorKind::TooLongValue {
                length: bytes.len(),
            }
            .into());
        }
        // We must swap the first bit to show long form
        // It is always zero by default with u8 type when value being < 128
        length |= 0b_1000_0000;
        buffer.extend(&length.to_be_bytes());
        buffer.extend(&bytes[..needed]);
        Ok(())
    }
    /// Encode the length of the value to output.
    /// `Length` of the data should be provided as full bytes.
    ///
    /// COER tries to use the shortest possible encoding and avoids leading zeros.
    fn encode_length(&mut self, buffer: &mut Vec<u8>, length: usize) -> Result<(), EncodeError> {
        let (bytes, needed) =
            (length as PrimitiveInteger).needed_as_be_bytes::<PRIMITIVE_BYTE_WIDTH>(false);

        if length < 128 {
            // First bit should be always zero when below 128: ITU-T X.696 8.6.4
            buffer.extend_from_slice(&bytes[..needed]);
            return Ok(());
        }
        let mut length_of_length = u8::try_from(needed).map_err(|err| {
            EncodeError::integer_type_conversion_failed(
                alloc::format!("Length of length conversion failed: {err}"),
                self.codec(),
            )
        })?;
        if length_of_length > 127 {
            return Err(CoerEncodeErrorKind::TooLongValue { length }.into());
        }
        // We must swap the first bit to show long form
        // It is always zero by default with u8 type when value being < 128
        length_of_length |= 0b_1000_0000;
        buffer.extend(&length_of_length.to_be_bytes());
        buffer.extend(&bytes[..needed]);
        Ok(())
    }
    /// Encode integer `value_to_enc` with length determinant
    /// Either as signed or unsigned, set by `signed`
    fn encode_unconstrained_integer(
        &mut self,
        value_to_enc: &Integer,
        signed: bool,
    ) -> Result<Vec<u8>, EncodeError> {
        // For primitives byte_length + 1 is enough without additional allocations
        let mut buffer = Vec::with_capacity(value_to_enc.byte_length() + 1);
        if let Integer::Primitive(value) = value_to_enc {
            let (slice, needed) = value.needed_as_be_bytes::<PRIMITIVE_BYTE_WIDTH>(true);
            self.encode_length(&mut buffer, needed)?;
            buffer.extend_from_slice(&slice[..needed]);
        } else {
            let bytes = crate::bits::integer_to_bytes(value_to_enc, signed).ok_or_else(|| {
                EncodeError::integer_type_conversion_failed(
                    "Negative integer value has been provided to be converted into unsigned bytes"
                        .to_string(),
                    self.codec(),
                )
            })?;
            self.encode_length(&mut buffer, bytes.len())?;
            buffer.extend(bytes);
        }

        Ok(buffer)
    }

    /// Encode an integer value with constraints.
    ///
    /// Encoding depends on the range constraint, and has two scenarios.
    /// a) The effective value constraint has a lower bound, and that lower bound is zero or positive.
    /// b) The effective value constraint has either a negative lower bound or no lower bound.
    /// Other integer constraints are OER-invisible.
    /// Unlike PER, OER does not add an extension bit at the beginning of the encoding of an integer
    /// type with an extensible OER-visible constraint. Such a type is encoded as an integer type with no bounds.
    ///
    /// If the Integer is not bound or outside of range, we encode with the smallest number of octets possible.
    fn encode_integer_with_constraints(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value_to_enc: &Integer,
    ) -> Result<(), EncodeError> {
        let mut buffer = Vec::with_capacity(value_to_enc.byte_length());

        if let Some(value) = constraints.value() {
            if !value.constraint.0.bigint_contains(value_to_enc) && value.extensible.is_none() {
                return Err(EncodeError::value_constraint_not_satisfied(
                    value_to_enc.clone(),
                    &value.constraint.0,
                    self.codec(),
                ));
            }
            ranges::determine_integer_size_and_sign(
                &value,
                value_to_enc,
                |value_to_enc, sign, octets| -> Result<(), EncodeError> {
                    let bytes: Vec<u8>;
                    if let Some(octets) = octets {
                        bytes = self.encode_constrained_integer_with_padding(
                            usize::from(octets),
                            value_to_enc,
                            sign,
                        )?;
                    } else {
                        bytes = self.encode_unconstrained_integer(value_to_enc, sign)?;
                    }
                    buffer.extend(bytes);
                    Ok(())
                },
            )?;
        } else {
            let bytes = self.encode_unconstrained_integer(value_to_enc, true)?;
            buffer.extend(bytes);
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    /// When range constraints are present, the integer is encoded as a fixed-size number.
    /// This means that the zero padding is possible even with COER encoding.
    fn encode_constrained_integer_with_padding(
        &mut self,
        octets: usize,
        value: &Integer,
        signed: bool,
    ) -> Result<Vec<u8>, EncodeError> {
        if octets > 8 {
            return Err(CoerEncodeErrorKind::InvalidConstrainedIntegerOctetSize.into());
        }
        let mut buffer: Vec<u8> = Vec::with_capacity(octets);
        let fixed_size_bytes: [u8; PRIMITIVE_BYTE_WIDTH];
        let len: usize;
        let bytes = if signed {
            match value {
                // For efficiency, access primitive ints with slice reference
                Integer::Primitive(value) => {
                    (fixed_size_bytes, len) = value.needed_as_be_bytes(true);
                    &fixed_size_bytes[..len]
                }
                Integer::Big(_) => &value.to_be_bytes(),
            }
        } else {
            match value {
                Integer::Primitive(value) => {
                    (fixed_size_bytes, len) = value.needed_as_be_bytes(false);
                    &fixed_size_bytes[..len]
                }

                Integer::Big(_) => &value.to_unsigned_be_bytes().ok_or_else(|| {
                    EncodeError::integer_type_conversion_failed(
                    "Negative integer value has been provided to be converted into unsigned bytes"
                        .to_string(),
                    self.codec(),
                )
                })?,
            }
        };
        let needed = bytes.len();
        match octets.cmp(&needed) {
            Ordering::Greater => {
                if signed && value.is_negative() {
                    // 2's complement
                    buffer.extend(core::iter::repeat(0xff).take(octets - bytes.len()));
                } else {
                    buffer.extend(core::iter::repeat(0x00).take(octets - bytes.len()));
                }
            }
            Ordering::Less => {
                return Err(EncodeError::from_kind(
                    EncodeErrorKind::MoreBytesThanExpected {
                        value: bytes.len(),
                        expected: octets,
                    },
                    self.codec(),
                ));
            }
            // As is
            Ordering::Equal => {}
        };
        buffer.extend(bytes);
        Ok(buffer)
    }
    fn check_fixed_size_constraint<T>(
        &self,
        value: T,
        length: usize,
        constraints: &Constraints,
        mut is_fixed_fn: impl FnMut(T) -> Result<(), EncodeError>,
    ) -> Result<bool, EncodeError> {
        if let Some(size) = constraints.size() {
            if !size.constraint.contains(&length) && size.extensible.is_none() {
                return Err(EncodeError::size_constraint_not_satisfied(
                    length,
                    &size.constraint,
                    self.codec(),
                ));
            }
            // Encode without length determinant
            if size.constraint.is_fixed() && size.extensible.is_none() {
                return match is_fixed_fn(value) {
                    Ok(()) => Ok(true),
                    Err(err) => Err(err),
                };
            }
        }
        // Prior checks before encoding with length determinant
        let max_permitted_length = usize::MAX / 8; // In compile time, no performance penalty?
        if length > max_permitted_length {
            return Err(EncodeError::length_exceeds_platform_size(self.codec()));
        }
        Ok(false)
    }

    fn output_length(&self) -> usize {
        let mut output_length = self.output.len();
        output_length += usize::from(self.is_extension_sequence);
        output_length += self
            .field_bitfield
            .values()
            .filter(|(presence, _)| presence.is_optional_or_default())
            .count();
        output_length += self.parent_output_length.unwrap_or_default();

        if self.options.set_encoding {
            output_length += self.set_output.values().map(Vec::len).sum::<usize>();
        }
        output_length
    }
    fn new_set_encoder<C: Constructed>(&self) -> Self {
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

    fn new_sequence_encoder<C: Constructed>(&self) -> Self {
        let mut encoder = Self::new(self.options.without_set_encoding());
        encoder.field_bitfield = C::FIELDS
            .iter()
            .map(|field| (field.tag_tree.smallest_tag(), (field.presence, false)))
            .collect();
        encoder.parent_output_length = Some(self.output_length());
        encoder
    }
    fn encoded_extension_addition(extension_fields: &[Vec<u8>]) -> bool {
        !extension_fields.iter().all(Vec::is_empty)
    }
    fn encode_constructed<C: Constructed>(
        &mut self,
        tag: Tag,
        mut encoder: Self,
    ) -> Result<(), EncodeError> {
        self.set_bit(tag, true);
        let mut buffer = Vec::new();
        let mut preamble = BitString::with_capacity(8);
        // ### PREAMBLE ###
        // Section 16.2.2
        let extensions_defined = C::EXTENDED_FIELDS.is_some();
        let mut extensions_present = false;
        if extensions_defined {
            extensions_present = Self::encoded_extension_addition(&encoder.extension_fields);
            preamble.push(extensions_present);
        }
        // Section 16.2.3
        if C::FIELDS.number_of_optional_and_default_fields() > 0 {
            for bit in encoder
                .field_bitfield
                .values()
                .filter_map(|(presence, is_present)| {
                    presence.is_optional_or_default().then_some(is_present)
                })
                .copied()
            {
                preamble.push(bit);
            }
        }
        // 16.2.4 - fill missing bits from full octet with zeros
        if preamble.len() % 8 != 0 {
            preamble.extend(BitString::repeat(false, 8 - preamble.len() % 8));
        }
        debug_assert!(preamble.len() % 8 == 0);
        buffer.extend(crate::bits::to_vec(&preamble));
        // Section 16.3 ### Encodings of the components in the extension root ###
        // Must copy before move...
        let extension_fields = core::mem::take(&mut encoder.extension_fields);
        if encoder.field_bitfield.values().any(|(_, b)| *b) {
            buffer.extend(encoder.output());
        }
        if !extensions_defined || !extensions_present {
            self.extend(tag, buffer)?;
            return Ok(());
        }
        // Section 16.4 ### Extension addition presence bitmap ###
        let bitfield_length = extension_fields.len();
        let mut extension_bitmap_buffer = BitString::with_capacity(8);
        #[allow(clippy::cast_possible_truncation)]
        let missing_bits: u8 = if bitfield_length > 0 {
            (8u8 - (bitfield_length % 8) as u8) % 8
        } else {
            0
        };
        debug_assert!((bitfield_length + 8 + missing_bits as usize) % 8 == 0);
        self.encode_length(
            &mut buffer,
            (8 + bitfield_length + missing_bits as usize) / 8,
        )?;
        extension_bitmap_buffer.extend(missing_bits.to_be_bytes());
        for field in &extension_fields {
            extension_bitmap_buffer.push(!field.is_empty());
        }
        extension_bitmap_buffer.extend(BitString::repeat(false, missing_bits as usize));
        debug_assert!(extension_bitmap_buffer.len() % 8 == 0);
        buffer.extend(crate::bits::to_vec(&extension_bitmap_buffer));
        // Section 16.5 # Encodings of the components in the extension addition group, as open type
        for field in extension_fields
            .into_iter()
            .filter(|field| !field.is_empty())
        {
            self.encode_length(&mut buffer, field.len())?;
            buffer.extend(field);
        }
        self.extend(tag, buffer)?;
        Ok(())
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = EncodeError;

    fn codec(&self) -> Codec {
        self.options.current_codec()
    }

    fn encode_any(&mut self, tag: Tag, value: &Any) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, <Constraints>::default(), &value.contents)
    }

    /// ITU-T X.696 9.
    /// False is encoded as a single zero octet. In COER, true is always encoded as 0xFF.
    /// In Basic-OER, any non-zero octet value represents true, but we support only canonical encoding.
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.extend(tag, alloc::vec![if value { 0xffu8 } else { 0x00u8 }])?;
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        // TODO When Rec. ITU-T X.680 | ISO/IEC 8824-1, 22.7 applies (i.e., the bitstring type is defined with a
        // "NamedBitList"), the bitstring value shall be encoded with trailing 0 bits added or removed as necessary to satisfy the
        // effective size constraint.
        // Rasn does not currently support NamedBitList
        self.set_bit(tag, true);
        let mut buffer: Vec<u8> = Vec::with_capacity(2);
        let mut bit_string_encoding = BitVec::<u8, Msb0>::with_capacity(8);

        if let Some(size) = constraints.size() {
            // Constraints apply only if the lower and upper bounds
            // of the effective size constraint are identical (13.1)
            if size.constraint.is_fixed() && size.extensible.is_none() {
                //  Series of octets will be empty, allowed as 13.2.3,
                // but this seems impossible to implement for decoder so not supported
                // Can't say if this is an error in standard or not when not handling NamedBitList
                // if value.is_empty() {
                // } else
                if size.constraint.contains(&value.len()) {
                    let missing_bits: usize = (8 - value.len() % 8) % 8;
                    let trailing = BitVec::<u8, Msb0>::repeat(false, missing_bits);
                    if missing_bits > 0 {
                        bit_string_encoding.extend(value);
                        bit_string_encoding.extend(trailing);
                    } else {
                        bit_string_encoding.extend(value);
                    }
                    buffer.extend(crate::bits::to_vec(&bit_string_encoding));
                } else {
                    return Err(EncodeError::size_constraint_not_satisfied(
                        value.len(),
                        &size.constraint,
                        self.codec(),
                    ));
                }
                self.extend(tag, buffer)?;
                return Ok(());
            }
        }

        // If the BitString is empty, length is one and initial octet is zero
        if value.is_empty() {
            self.encode_length(&mut buffer, 1)?;
            buffer.extend(&[0x00u8]);
        } else {
            // TODO 22.7 X.680, NamedBitString and COER
            // if self.options.encoding_rules.is_coer()
            //     && value.trailing_zeros() > 7
            // {
            //     if value.first_one().is_some() {
            //         // In COER, we strip trailing zeros if they take full octets
            //         let trimmed_value = &value[..(value.len() - value.trailing_zeros() - 1)];
            //     }
            //     else {  }
            // }
            // With length determinant
            let missing_bits: usize = (8 - value.len() % 8) % 8;
            let trailing = BitVec::<u8, Msb0>::repeat(false, missing_bits);
            // missing bits never > 8
            bit_string_encoding.extend(u8::try_from(missing_bits).unwrap_or(0).to_be_bytes());
            bit_string_encoding.extend(value);
            bit_string_encoding.extend(trailing);
            self.encode_length(&mut buffer, bit_string_encoding.len() / 8)?;
            buffer.extend(crate::bits::to_vec(&bit_string_encoding));
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        // 11.5 The presence of an extension marker in the definition of an enumerated type does not affect the encoding of
        // the values of the enumerated type.
        // max size for enumerated value is currently only isize MIN/MAX
        // Spec allows between –2^1015 and 2^1015 – 1
        self.set_bit(tag, true);
        let number = value.discriminant();
        let mut buffer = Vec::with_capacity(2);
        if 0isize <= number && number <= i8::MAX.into() {
            let bytes = self.encode_constrained_integer_with_padding(1, &number.into(), false)?;
            buffer.extend(bytes);
        } else {
            // Value is signed here as defined in section 11.4
            // Long form but different from regular length determinant encoding
            self.encode_unconstrained_enum_index(&mut buffer, number)?;
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::ber());
        let octets = enc.object_identifier_as_bytes(value)?;
        let mut buffer = Vec::with_capacity(2 + octets.len());
        self.encode_length(&mut buffer, octets.len())?;
        buffer.extend(&octets);
        self.extend(tag, buffer)?;
        Ok(())
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &Integer,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_integer_with_constraints(tag, &constraints, value)
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        let mut buffer = Vec::with_capacity(value.len());
        let fixed_size_encode = |value: &[u8]| {
            buffer.extend(value);
            Ok(())
        };
        if !self.check_fixed_size_constraint(value, value.len(), &constraints, fixed_size_encode)? {
            // Use length determinant on other cases
            self.encode_length(&mut buffer, value.len())?;
            buffer.extend(value);
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        // Seems like it can be encoded as it is...
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value.as_bytes())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value.as_iso646_bytes())
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value.as_iso646_bytes())
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value.as_bytes())
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value.as_bytes())
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        // X.690 8.23.5
        // TODO the octets specified in ISO/IEC 2022 for encodings in an 8-bit environment, using
        // the escape sequence and character codings registered in accordance with ISO/IEC 2375.
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, value)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(tag, constraints, &value.to_bytes())
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
        )
    }

    fn encode_utc_time(&mut self, tag: Tag, value: &UtcTime) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
        )
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        if let Some((_, true)) = self.field_bitfield.get(&tag) {
            value.encode(self)
        } else if !self.field_bitfield.contains_key(&tag) {
            value.encode(self)
        } else {
            self.set_bit(tag, true);
            value.encode_with_tag(self, tag)
        }
    }

    fn encode_sequence<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        let mut encoder = self.new_sequence_encoder::<C>();
        encoder_scope(&mut encoder)?;
        self.encode_constructed::<C>(tag, encoder)
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        _: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        // It seems that constraints here are not C/OER visible? No mention in standard...
        self.set_bit(tag, true);
        let mut buffer = Vec::new();
        let value_len_bytes = self.encode_unconstrained_integer(&value.len().into(), false)?;
        buffer.extend(value_len_bytes);
        for one in value {
            let mut encoder = Self::new(self.options);
            E::encode(one, &mut encoder)?;
            buffer.extend(encoder.output());
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    fn encode_set<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        let mut set = self.new_set_encoder::<C>();
        encoder_scope(&mut set)?;
        self.encode_constructed::<C>(tag, set)
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &value.iter().collect::<Vec<_>>(), constraints)
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        self.set_bit(E::TAG, true);
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true);
        value.encode_with_tag_and_constraints(self, tag, constraints)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.set_bit(E::TAG, false);
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, false);
        Ok(())
    }

    fn encode_choice<E: Encode + Choice>(
        &mut self,
        _: Constraints,
        _tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let mut choice_encoder = Self::new(self.options.without_set_encoding());
        let tag = encode_fn(&mut choice_encoder)?;
        let is_root_extension = crate::TagTree::tag_contains(&tag, E::VARIANTS);
        let tag_bytes: Vec<u8> = Self::encode_tag(tag);
        let mut buffer = Vec::new();
        buffer.extend(tag_bytes);
        if is_root_extension {
            buffer.extend(choice_encoder.output);
        } else {
            self.encode_length(&mut buffer, choice_encoder.output.len())?;
            buffer.extend(choice_encoder.output);
        }
        self.extend(tag, buffer)?;
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
            self.set_bit(tag, true);
            self.extension_fields.push(encoder.output());
        } else {
            self.set_bit(tag, false);
            self.extension_fields.push(Vec::new());
        }

        Ok(())
    }
    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + Constructed,
    {
        let Some(value) = value else {
            self.set_bit(E::TAG, false);
            self.extension_fields.push(Vec::new());
            return Ok(());
        };
        self.set_bit(E::TAG, true);
        let mut encoder = self.new_sequence_encoder::<E>();
        encoder.is_extension_sequence = true;
        value.encode(&mut encoder)?;

        let output = encoder.output();
        self.extension_fields.push(output);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Pow;

    use super::*;
    use crate::prelude::{AsnType, Decode, Encode};
    use crate::types::constraints::{Bounded, Constraint, Extensible, Value};

    #[test]
    fn test_encode_bool() {
        let output = crate::coer::encode(&true).unwrap();
        let bv = BitVec::<u8, Msb0>::from_slice(&[0xffu8]);
        assert_eq!(output, bv.as_raw_slice());
        let output = crate::coer::encode(&false).unwrap();
        let bv = BitVec::<u8, Msb0>::from_slice(&[0x00u8]);
        assert_eq!(output, bv.as_raw_slice());
        let decoded = crate::coer::encode(&true).unwrap();
        assert_eq!(decoded, &[0xffu8]);
        let decoded = crate::coer::encode(&false).unwrap();
        assert_eq!(decoded, &[0x0]);
    }
    #[test]
    fn test_encode_integer_manual_setup() {
        let range_bound = Bounded::<PrimitiveInteger>::Range {
            start: 0.into(),
            end: 255.into(),
        };
        let value_range = &[Constraint::Value(Extensible::new(Value::new(range_bound)))];
        let consts = Constraints::new(value_range);
        let mut encoder = Encoder::default();
        let result =
            encoder.encode_integer_with_constraints(Tag::INTEGER, &consts, &Integer::from(244));
        assert!(result.is_ok());
        let v = vec![244u8];
        assert_eq!(encoder.output, v);
        encoder.output.clear();
        let value = Integer::from(256);
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &consts, &value);
        assert!(result.is_err());
    }
    #[test]
    fn test_integer_with_length_determinant() {
        // Using defaults, no limits
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();
        let result = encoder.encode_integer_with_constraints(
            Tag::INTEGER,
            &constraints,
            &Integer::from(244),
        );
        assert!(result.is_ok());
        let v = vec![2u8, 0, 244];
        assert_eq!(encoder.output, v);
        encoder.output.clear();
        let result = encoder.encode_integer_with_constraints(
            Tag::INTEGER,
            &constraints,
            &Integer::from(-1_234_567),
        );
        assert!(result.is_ok());
        let v = vec![0x03u8, 0xED, 0x29, 0x79];
        assert_eq!(encoder.output, v);
    }
    #[test]
    fn test_large_lengths() {
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();

        // Signed integer with byte length of 128
        // Needs long form to represent
        let number = Integer::from(256).pow(127.into()) - 1.into();
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &constraints, &number);
        assert!(result.is_ok());
        let vc = [
            0x81, 0x80, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ];
        assert_eq!(encoder.output(), vc);
    }
    #[test]
    fn test_choice() {
        use crate as rasn;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum Choice {
            Normal(Integer),
            High(Integer),
            #[rasn(extension_addition)]
            Medium(Integer),
        }
        let mut encoder = Encoder::default();

        let choice = Choice::Normal(333.into());
        choice.encode(&mut encoder).unwrap();

        assert_eq!(encoder.output(), &[128, 2, 1, 77]);
    }
}
