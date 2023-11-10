use alloc::{string::ToString, vec::Vec};

use crate::oer::helpers;
use crate::prelude::{
    Any, BitStr, BmpString, Choice, Constructed, Enumerated, GeneralString, GeneralizedTime,
    Ia5String, NumericString, PrintableString, SetOf, TeletexString, UtcTime, VisibleString,
};
use crate::Codec;
use bitvec::prelude::*;
use num_traits::{Signed, ToPrimitive};

use crate::types::{fields::FieldPresence, BitString, Constraints, Integer};
use crate::{Encode, Tag};

/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
use crate::error::{CoerEncodeErrorKind, EncodeError, EncodeErrorKind};

// pub type Result<T, E = EncodeError> = core::result::Result<T, E>;

pub const ITU_T_X696_OER_EDITION: f32 = 3.0;
const MAX_LENGTH_IN_BYTES: usize = usize::MAX / 8;

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
    output: BitString,
    set_output: alloc::collections::BTreeMap<Tag, BitString>,
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
            output: <BitString>::default(),
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
        // TODO, move from per to utility module?
        let output = self.bitstring_output();
        crate::per::to_vec(&output)
    }
    #[must_use]
    pub fn bitstring_output(self) -> BitString {
        self.options
            .set_encoding
            .then(|| self.set_output.values().flatten().collect::<BitString>())
            .unwrap_or(self.output)
    }
    pub fn set_bit(&mut self, tag: Tag, bit: bool) {
        self.field_bitfield.entry(tag).and_modify(|(_, b)| *b = bit);
    }
    fn extend(&mut self, tag: Tag, bytes: BitString) -> Result<(), EncodeError> {
        // TODO bound check
        if self.options.set_encoding {
            self.set_output.insert(tag, bytes);
        } else {
            self.output.extend(bytes);
        }
        Ok(())
    }
    /// Encode a tag as specified in ITU-T X.696 8.7
    ///
    /// Encoding of the tag is only required when encoding a choice type.
    fn encode_tag(tag: Tag) -> BitVec {
        use crate::types::Class;
        let mut bv = BitVec::new();
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
        bv
    }

    /// Encode the length of the value to output.
    /// `Length` of the data should be provided as bits.
    ///
    /// COER tries to use the shortest possible encoding and avoids leading zeros.
    /// `forced_long_form` is used for cases when length < 128 but we want to force long form. E.g. when encoding a enumerated.
    fn encode_length(
        &mut self,
        buffer: &mut BitString,
        length: usize,
        signed: bool,
        forced_long_form: bool,
    ) -> Result<(), EncodeError> {
        // Bits to byte length
        let length = if length % 8 == 0 {
            length / 8
        } else {
            return Err(CoerEncodeErrorKind::LengthNotAsBitLength {
                length,
                remainder: length % 8,
            }
            .into());
        };
        // On some cases we want to present length also as signed integer
        // E.g. length of a enumerated value
        //  ITU-T X.696 (02/2021) 11.4 ???? Seems like it is not needed
        let bytes =
            helpers::integer_to_bitvec_bytes(&Integer::from(length), signed).ok_or_else(|| {
                EncodeError::integer_type_conversion_failed(
                    "Negative integer value has been provided to be converted into unsigned bytes"
                        .to_string(),
                    self.codec(),
                )
            })?;

        if length < 128 && !forced_long_form {
            // First bit should be always zero when below 128: ITU-T X.696 8.6.4
            buffer.extend(&bytes);
            return Ok(());
        }
        if length < 128 && forced_long_form {
            // We must swap the first bit to show long form
            let mut bytes = bytes;
            _ = bytes.remove(0);
            bytes.insert(0, true);
            buffer.extend(&bytes);
            return Ok(());
        }
        let length_of_length = u8::try_from(bytes.len() / 8);
        if length_of_length.is_ok() && length_of_length.unwrap() > 127 {
            return Err(CoerEncodeErrorKind::TooLongValue {
                length: length as u128,
            }
            .into());
        } else if length_of_length.is_ok() {
            buffer.extend(length_of_length.unwrap().to_be_bytes());
            // We must swap the first bit to show long form
            // It is always zero by default with u8 type when value being < 128
            _ = buffer.remove(0);
            buffer.insert(0, true);
            buffer.extend(bytes);
        } else {
            return Err(EncodeError::integer_type_conversion_failed(
                length_of_length.err().unwrap().to_string(),
                self.codec(),
            ));
        }
        Ok(())
    }
    /// Encode integer `value_to_enc` with length determinant
    /// Either as signed or unsigned, set by `signed`
    fn encode_unconstrained_integer(
        &mut self,
        value_to_enc: &Integer,
        signed: bool,
        long_form_short_length: bool,
    ) -> Result<BitString, EncodeError> {
        helpers::integer_to_bitvec_bytes(value_to_enc, signed).ok_or_else(|| {
            EncodeError::integer_type_conversion_failed(
                "Negative integer value has been provided to be converted into unsigned bytes"
                    .to_string(),
                self.codec(),
            )
        })
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
        let mut buffer = BitString::default();

        if let Some(value) = constraints.value() {
            if !value.constraint.0.bigint_contains(value_to_enc) && value.extensible.is_none() {
                return Err(EncodeError::value_constraint_not_satisfied(
                    value_to_enc.to_owned(),
                    &value.constraint.0,
                    self.codec(),
                ));
            }
            helpers::determine_integer_size_and_sign(
                &value,
                value_to_enc,
                |value_to_enc, sign, octets| -> Result<(), EncodeError> {
                    let bytes: BitString;
                    if let Some(octets) = octets {
                        bytes = self.encode_constrained_integer_with_padding(
                            i128::from(octets),
                            value_to_enc,
                            sign,
                        )?;
                    } else {
                        bytes = self.encode_unconstrained_integer(value_to_enc, sign, false)?;
                        self.encode_length(&mut buffer, bytes.len(), false, false)?;
                    }
                    buffer.extend(bytes);
                    // self.extend(tag, buffer)?;
                    Ok(())
                },
            )?;
        } else {
            let bytes = self.encode_unconstrained_integer(value_to_enc, true, false)?;
            if bytes.len() > MAX_LENGTH_IN_BYTES {
                return Err(CoerEncodeErrorKind::TooLongValue {
                    length: buffer.len() as u128,
                }
                .into());
            }
            self.encode_length(&mut buffer, bytes.len(), false, false)?;
            buffer.extend(bytes);
        }
        self.extend(tag, buffer)?;
        Ok(())
    }

    /// When range constraints are present, the integer is encoded as a fixed-size number.
    /// This means that the zero padding is possible even with COER encoding.
    fn encode_constrained_integer_with_padding(
        &mut self,
        octets: i128,
        value: &Integer,
        signed: bool,
    ) -> Result<BitString, EncodeError> {
        use core::cmp::Ordering;
        if octets > 8 {
            return Err(CoerEncodeErrorKind::InvalidConstrainedIntegerOctetSize.into());
        }
        let bytes = if signed {
            value.to_signed_bytes_be()
        } else {
            value.to_biguint().unwrap().to_bytes_be()
        };
        let bits = BitVec::<u8, Msb0>::from_slice(&bytes);
        // octets * 8 never > 64, safe conversion and multiplication
        #[allow(clippy::cast_sign_loss)]
        let octets_as_bits: usize = octets as usize * 8;
        let bits = match octets_as_bits.cmp(&bits.len()) {
            Ordering::Greater => {
                let mut padding: BitVec<u8, Msb0>;
                if signed && value.is_negative() {
                    // 2's complement
                    padding = BitString::repeat(true, octets_as_bits - bits.len());
                } else {
                    padding = BitString::repeat(false, octets_as_bits - bits.len());
                }
                padding.extend(bits);
                padding
            }
            Ordering::Less => {
                return Err(EncodeError::from_kind(
                    EncodeErrorKind::MoreBytesThanExpected {
                        value: bits.len(),
                        expected: octets_as_bits,
                    },
                    self.codec(),
                ));
            }
            Ordering::Equal => bits,
        };
        // self.output.extend(bits);
        Ok(bits)
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
    /// Encode a string with a known multiplier.
    ///
    /// We rely on rasn to provide the correct allowed characters for each type.
    ///
    /// Note: following common constraints are not OER-visible:
    /// * Permitted alphabet constraints;
    /// * Pattern constraints;
    fn encode_known_multiplier_string<T: crate::types::strings::StaticPermittedAlphabet>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &T,
    ) -> Result<(), EncodeError> {
        let mut buffer = BitString::default();
        let fixed_size_encode = |value: &&T| {
            buffer.extend(value.to_octet_aligned_string());
            Ok(())
        };
        if !self.check_fixed_size_constraint(&value, value.len(), constraints, fixed_size_encode)? {
            // Use length determinant on other cases
            // Save multiplication, overflow checked earlier
            self.encode_length(&mut buffer, value.len() * 8, false, false)?;
            buffer.extend(value.to_bit_string());
        }
        self.extend(tag, value.to_bit_string())?;
        Ok(())
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
            output_length += self
                .set_output
                .values()
                .map(bitvec::vec::BitVec::len)
                .sum::<usize>();
        }
        output_length
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
    fn encoded_extension_addition(extension_fields: &[Vec<u8>]) -> bool {
        !extension_fields.iter().all(alloc::vec::Vec::is_empty)
    }
    fn encode_constructed<C: crate::types::Constructed>(
        &mut self,
        tag: Tag,
        mut encoder: Self,
    ) -> Result<(), EncodeError> {
        self.set_bit(tag, true);
        let mut buffer = BitString::default();
        // ### PREAMBLE ###
        // Section 16.2.2
        let extensions_defined = C::EXTENDED_FIELDS.is_some();
        let mut extensions_present = false;
        if extensions_defined {
            extensions_present = Self::encoded_extension_addition(&encoder.extension_fields);
            buffer.push(extensions_present);
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
                buffer.push(bit);
            }
        }
        // 16.2.4 - fill missing bits from full octet with zeros
        if buffer.len() % 8 != 0 {
            buffer.extend(BitString::repeat(false, 8 - buffer.len() % 8));
        }
        // Section 16.3 ### Encodings of the components in the extension root ###
        // Must copy before move...
        let extension_fields = core::mem::take(&mut encoder.extension_fields);
        if encoder.field_bitfield.values().any(|(a, b)| *b) {
            buffer.extend(encoder.bitstring_output());
        }
        if !extensions_defined || !extensions_present {
            debug_assert!(self.output.len() % 8 == 0);
            self.extend(tag, buffer)?;
            return Ok(());
        }
        // Section 16.4 ### Extension addition presence bitmap ###
        let bitfield_length = extension_fields.len();
        // let mut extension_buffer = BitString::new();
        // TODO overflow check
        let missing_bits: u8 = if bitfield_length > 0 {
            8u8 - (bitfield_length % 8usize) as u8
        } else {
            0
        };
        debug_assert!((bitfield_length + 8 + missing_bits as usize) % 8 == 0);
        self.encode_length(
            &mut buffer,
            8 + bitfield_length + missing_bits as usize,
            false,
            false,
        )?;
        buffer.extend(missing_bits.to_be_bytes());
        for field in &extension_fields {
            buffer.push(!field.is_empty());
        }
        buffer.extend(BitString::repeat(false, missing_bits as usize));
        // Section 16.5 # Encodings of the components in the extension addition group, as open type
        for field in extension_fields
            .into_iter()
            .filter(|field| !field.is_empty())
        {
            self.encode_length(&mut buffer, 8 * field.len(), false, false)?;
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
        self.extend(
            tag,
            BitVec::<u8, Msb0>::from_slice(&[if value { 0xffu8 } else { 0x00u8 }]),
        )?;
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
        let mut buffer = BitString::default();

        let fixed_size_encode = |value: &BitStr| {
            let missing_bits: usize = 8 - value.len() % 8;
            let trailing = BitVec::<u8, Msb0>::repeat(false, missing_bits);
            if missing_bits > 0 {
                buffer.extend(value);
                buffer.extend(trailing);
            } else {
                buffer.extend(value);
            }
            Ok(())
        };
        if !self.check_fixed_size_constraint(value, value.len(), &constraints, fixed_size_encode)? {
            // With length determinant
            let missing_bits: usize = (8 - value.len() % 8) % 8;
            if missing_bits < 8 {}
            let trailing = BitVec::<u8, Msb0>::repeat(false, missing_bits);
            let mut bit_string = BitVec::<u8, Msb0>::new();
            // missing bits never > 8
            bit_string.extend(missing_bits.to_u8().unwrap().to_be_bytes());
            bit_string.extend(value);
            bit_string.extend(trailing);
            self.encode_length(&mut buffer, bit_string.len(), false, false)?;
            buffer.extend(bit_string);
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
        // TODO max size for enumerated value is currently only isize MIN/MAX
        // Spec allows between –2^1015 and 2^1015 – 1
        // TODO negative discriminant values are not currently possibly
        self.set_bit(tag, true);
        let number = value.discriminant();
        let mut buffer = BitString::default();
        if 0isize <= number && number <= i8::MAX.into() {
            let bytes = self.encode_constrained_integer_with_padding(1, &number.into(), false)?;
            buffer.extend(bytes);
        } else {
            //Value is signed here as defined in section 11.4
            // Long form
            let bytes = self.encode_unconstrained_integer(&number.into(), true, true)?;
            self.encode_length(&mut buffer, bytes.len(), true, true)?;
            buffer.extend(bytes);
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
        if value.len() > MAX_LENGTH_IN_BYTES {
            return Err(CoerEncodeErrorKind::TooLongValue {
                length: value.len() as u128,
            }
            .into());
        }
        let mut buffer = BitString::default();
        self.encode_length(&mut buffer, octets.len() * 8, false, false)?;
        buffer.extend(BitVec::<u8, Msb0>::from_slice(&octets));
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
        let mut buffer = BitString::default();
        let fixed_size_encode = |value: &[u8]| {
            buffer.extend(value);
            Ok(())
        };
        if !self.check_fixed_size_constraint(value, value.len(), &constraints, fixed_size_encode)? {
            // Use length determinant on other cases
            self.encode_length(&mut buffer, value.len() * 8, false, false)?;
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
        // TODO check additional conditions from teletex fn
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
        // Are there some escape sequences that we should add?
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
        self.encode_known_multiplier_string(tag, &constraints, value)
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
        } else if self.field_bitfield.get(&tag).is_none() {
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
        (encoder_scope)(&mut encoder)?;
        self.encode_constructed::<C>(tag, encoder)
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        // TODO, it seems that constraints here are not C/OER visible? No mention in standard...
        self.set_bit(tag, true);
        let mut buffer = BitString::default();
        let value_len_bytes =
            self.encode_unconstrained_integer(&value.len().into(), false, false)?;
        self.encode_length(
            &mut buffer,
            if value_len_bytes.is_empty() {
                1
            } else {
                value_len_bytes.len()
            },
            false,
            false,
        )?;
        buffer.extend(value_len_bytes);
        for one in value {
            let mut encoder = Self::new(self.options);
            E::encode(one, &mut encoder)?;
            buffer.extend(encoder.bitstring_output());
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
        (encoder_scope)(&mut set)?;
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
        constraints: Constraints,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let mut choice_encoder = Self::new(self.options.without_set_encoding());
        let tag = (encode_fn)(&mut choice_encoder)?;
        let is_root_extension = crate::TagTree::tag_contains(&tag, E::VARIANTS);
        let tag_bytes = Self::encode_tag(tag);
        let mut buffer = BitString::default();
        buffer.extend(tag_bytes);
        if is_root_extension {
            buffer.extend(choice_encoder.output);
        } else {
            self.encode_length(&mut buffer, choice_encoder.output.len(), false, false)?;
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
        E: Encode + crate::types::Constructed,
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
    use num_bigint::BigInt;

    use super::*;
    use crate::prelude::{AsnType, Decode, Encode};
    use crate::types::constraints::{Bounded, Constraint, Constraints, Extensible, Value};

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
        let range_bound = Bounded::<i128>::Range {
            start: 0.into(),
            end: 255.into(),
        };
        let value_range = &[Constraint::Value(Extensible::new(Value::new(range_bound)))];
        let consts = Constraints::new(value_range);
        let mut encoder = Encoder::default();
        let result =
            encoder.encode_integer_with_constraints(Tag::INTEGER, &consts, &BigInt::from(244));
        assert!(result.is_ok());
        let v = vec![244u8];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
        encoder.output.clear();
        let value = BigInt::from(256);
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &consts, &value);
        assert!(matches!(result, Err(encode_error)));
    }
    #[test]
    fn test_integer_with_length_determinant() {
        // Using defaults, no limits
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();
        let result =
            encoder.encode_integer_with_constraints(Tag::INTEGER, &constraints, &BigInt::from(244));
        assert!(result.is_ok());
        let v = vec![2u8, 0, 244];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
        encoder.output.clear();
        let result = encoder.encode_integer_with_constraints(
            Tag::INTEGER,
            &constraints,
            &BigInt::from(-1_234_567),
        );
        assert!(result.is_ok());
        let v = vec![0x03u8, 0xED, 0x29, 0x79];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
    }
    #[test]
    fn test_large_lengths() {
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();

        // Signed integer with byte length of 128
        // Needs long form to represent
        let number = BigInt::from(256).pow(127) - 1;
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
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();

        let choice = Choice::Normal(333.into());
        choice.encode(&mut encoder).unwrap();

        assert_eq!(encoder.output(), &[128, 2, 1, 77]);
        // let result = encoder.encode_choice(constraints, |encoder| encoder.encode_bool(true));
    }
}
