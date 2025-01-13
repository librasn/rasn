//! Decoding Octet Encoding Rules data into Rust structures.

// ITU-T X.696 (02/2021) version of OER decoding
// In OER, without knowledge of the type of the value encoded, it is not possible to determine
// the structure of the encoding. In particular, the end of the encoding cannot be determined from
// the encoding itself without knowledge of the type being encoded ITU-T X.696 (6.2).

use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};

use core::num::NonZeroUsize;
use nom::Needed;

use crate::{
    de::{Decode, Error as _},
    oer::EncodingRules,
    types::{
        self,
        fields::{Field, Fields},
        Any, BitString, BmpString, Constraints, Constructed, DecodeChoice, Enumerated,
        GeneralString, GeneralizedTime, Ia5String, IntegerType, NumericString, ObjectIdentifier,
        PrintableString, SetOf, Tag, TeletexString, UtcTime, VisibleString,
    },
    Codec,
};

use bitvec::{order::Msb0, view::BitView};

use crate::error::{CoerDecodeErrorKind, DecodeError, DecodeErrorKind, OerDecodeErrorKind};

/// Options for configuring the [`Decoder`].
#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    encoding_rules: EncodingRules, // default COER
}

impl DecoderOptions {
    /// Returns the default decoding rules options for [EncodingRules::Oer].
    #[must_use]
    pub const fn oer() -> Self {
        Self {
            encoding_rules: EncodingRules::Oer,
        }
    }

    /// Returns the default decoding rules options for [EncodingRules::Coer].
    #[must_use]
    pub const fn coer() -> Self {
        Self {
            encoding_rules: EncodingRules::Coer,
        }
    }

    #[must_use]
    fn current_codec(self) -> Codec {
        match self.encoding_rules {
            EncodingRules::Oer => Codec::Oer,
            EncodingRules::Coer => Codec::Coer,
        }
    }
}

/// Decodes Octet Encoding Rules (OER) data into Rust data structures.
pub struct Decoder<'input, const RFC: usize = 0, const EFC: usize = 0> {
    input: &'input [u8],
    options: DecoderOptions,
    fields: ([Option<Field>; RFC], usize),
    extension_fields: Option<Fields<EFC>>,
    extensions_present: Option<Option<([Option<Field>; EFC], usize)>>,
}

impl<'input, const RFC: usize, const EFC: usize> Decoder<'input, RFC, EFC> {
    /// Creates a new Decoder from the given input and options.
    #[must_use]
    pub fn new(input: &'input [u8], options: DecoderOptions) -> Self {
        Self {
            input,
            options,
            fields: ([None; RFC], 0),
            extension_fields: <_>::default(),
            extensions_present: <_>::default(),
        }
    }

    #[must_use]
    fn codec(&self) -> Codec {
        self.options.current_codec()
    }
    /// Returns reference to the remaining input data that has not been parsed.
    #[must_use]
    pub fn remaining(&self) -> &'input [u8] {
        self.input
    }

    fn parse_one_byte(&mut self) -> Result<u8, DecodeError> {
        let (first, rest) = self.input.split_first().ok_or_else(|| {
            DecodeError::parser_fail(
                "Unexpected end of data when parsing single byte from &[u8]".to_string(),
                self.codec(),
            )
        })?;
        self.input = rest;
        Ok(*first)
    }

    fn parse_tag(&mut self) -> Result<Tag, DecodeError> {
        // Seems like tag number
        use crate::types::Class;
        let first_byte = self.parse_one_byte()?;
        let class = match first_byte >> 6 {
            0b00 => Class::Universal,
            0b01 => Class::Application,
            0b10 => Class::Context,
            0b11 => Class::Private,
            class => return Err(OerDecodeErrorKind::InvalidTagClassOnChoice { class }.into()),
        };
        let tag_number = first_byte & 0b0011_1111;
        if tag_number == 0b11_1111 {
            // Long form
            let mut tag_number = 0u32;
            let mut next_byte = self.parse_one_byte()?;
            // The first octet cannot have last 7 bits set to 0
            if next_byte & 0b0111_1111 == 0 || next_byte == 0 {
                return Err(OerDecodeErrorKind::invalid_tag_number_on_choice(u32::from(
                    next_byte & 0b1000_0000,
                )));
            }
            loop {
                // Constructs tag number from multiple 7-bit sized chunks
                tag_number = tag_number
                    .checked_shl(7)
                    .ok_or(OerDecodeErrorKind::invalid_tag_number_on_choice(tag_number))?
                    | u32::from(next_byte & 0b0111_1111);
                // The zero-first-bit marks the octet as last
                if next_byte & 0b1000_0000 == 0 {
                    break;
                }
                next_byte = self.parse_one_byte()?;
            }
            Ok(Tag::new(class, tag_number))
        } else {
            Ok(Tag::new(class, u32::from(tag_number)))
        }
    }

    /// There is a short form and long form for length determinant in OER encoding.
    /// In short form one octet is used and the leftmost bit is always zero; length is less than 128
    /// Max length for data type could be 2^1016 - 1 octets, however on this implementation it is limited to `usize::MAX`
    fn decode_length(&mut self) -> Result<usize, DecodeError> {
        let possible_length = self.parse_one_byte()?;
        if possible_length < 128 {
            Ok(usize::from(possible_length))
        } else {
            // We have the length of the length, mask and extract only 7 bis
            let length = possible_length & 0x7fu8;
            // Length of length cannot be zero
            if length == 0 {
                return Err(DecodeError::from_kind(
                    DecodeErrorKind::ZeroLengthOfLength,
                    self.codec(),
                ));
            }
            let (data, rest) = self
                .input
                .split_at_checked(length as usize)
                .ok_or_else(|| {
                    DecodeError::parser_fail(
                        alloc::format!("Unexpected end of data when parsing length by length of length {} from &[u8]", length
                            ),
                        self.codec(),
                    )
                })?;
            self.input = rest;

            if self.options.encoding_rules.is_coer() && data.first() == Some(&0) {
                return Err(CoerDecodeErrorKind::NotValidCanonicalEncoding {
                    msg: "Length value should not have leading zeroes in COER".to_string(),
                }
                .into());
            }
            let length = usize::try_from_unsigned_bytes(data, self.codec())?;
            if length < 128 && self.options.encoding_rules.is_coer() {
                return Err(CoerDecodeErrorKind::NotValidCanonicalEncoding {
                    msg: "Length determinant could have been encoded in short form.".to_string(),
                }
                .into());
            }
            Ok(length)
        }
    }

    /// Extracts data from input by length and updates the input
    /// 'length' is the length of the data in bytes (octets)
    /// Returns the data
    fn extract_data_by_length(&mut self, length: usize) -> Result<&'input [u8], DecodeError> {
        if length == 0 {
            return Ok(&[]);
        }
        let (data, rest) = self.input.split_at_checked(length).ok_or_else(|| {
            DecodeError::incomplete(
                Needed::Size(NonZeroUsize::new(length - self.input.len()).unwrap()),
                self.codec(),
            )
        })?;
        self.input = rest;
        Ok(data)
    }

    fn decode_integer_from_bytes<I: crate::types::IntegerType>(
        &mut self,
        signed: bool,
        length: Option<usize>,
    ) -> Result<I, DecodeError> {
        let final_length = match length {
            Some(l) => l,
            None => self.decode_length()?,
        };

        let codec = self.codec();
        let coer = self.options.encoding_rules.is_coer();
        let data = self.extract_data_by_length(final_length)?;
        // // Constrained data can correctly include leading zeros, unconstrained not
        if coer && !signed && data.first() == Some(&0) && length.is_none() {
            return Err(CoerDecodeErrorKind::NotValidCanonicalEncoding {
                msg: "Leading zeros are not allowed on unsigned Integer value in COER.".to_string(),
            }
            .into());
        }
        if signed {
            Ok(I::try_from_signed_bytes(data, codec)?)
        } else {
            Ok(I::try_from_unsigned_bytes(data, codec)?)
        }
    }

    fn decode_integer_with_constraints<I: crate::types::IntegerType>(
        &mut self,
        constraints: &Constraints,
    ) -> Result<I, DecodeError> {
        // Only 'value' constraint is OER visible for integer
        if let Some(value) = constraints.value() {
            let (signed, octets) = if value.extensible.is_some() {
                (true, None)
            } else {
                (value.constraint.get_sign(), value.constraint.get_range())
            };
            let integer = self.decode_integer_from_bytes::<I>(signed, octets.map(usize::from))?;
            // if the value is too large for a i128, the constraint isn't satisfied
            if let Some(constraint_integer) = integer.to_i128() {
                if value.constraint.contains(&constraint_integer) {
                    Ok(integer)
                } else {
                    Err(DecodeError::value_constraint_not_satisfied(
                        integer.to_bigint().unwrap_or_default(),
                        value.constraint.value,
                        self.codec(),
                    ))
                }
            } else {
                Err(DecodeError::value_constraint_not_satisfied(
                    integer.to_bigint().unwrap_or_default(),
                    value.constraint.value,
                    self.codec(),
                ))
            }
            // })
        } else {
            // No constraints
            self.decode_integer_from_bytes::<I>(true, None)
        }
    }

    fn parse_bit_string(&mut self, constraints: &Constraints) -> Result<BitString, DecodeError> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let length = size.constraint.as_start().ok_or_else(|| {
                    Err(DecodeError::size_constraint_not_satisfied(
                        None,
                        "Fixed size constraint should have value".to_string(),
                        self.codec(),
                    ))
                });
                return match length {
                    Ok(length) => {
                        let bytes_required = (*length + 7) / 8;
                        let data = &self
                            .extract_data_by_length(bytes_required)?
                            .view_bits::<Msb0>()[..*length];
                        Ok(data.into())
                    }
                    Err(e) => e,
                };
            }
        }
        let length = self.decode_length()?;
        if length == 0 {
            return Ok(BitString::new());
        }

        let num_unused_bits = self.parse_one_byte()?;
        if length == 1 && num_unused_bits > 0 {
            return Err(OerDecodeErrorKind::invalid_bit_string(
                "Length includes only initial octet. There cannot be unused bits on the subsequent octects as there isn't any.".to_string(),
            ));
        }
        if num_unused_bits > 7 {
            return Err(OerDecodeErrorKind::invalid_bit_string(
                "Marked number of unused bits should be less than 8 when decoding OER".to_string(),
            ));
        }
        // Remove one from length as one describes trailing zeros...
        let data_bit_length: usize = (&length - 1usize).checked_mul(8).ok_or_else(|| {
            DecodeError::length_exceeds_platform_width(
                "Total length exceeds BitSlice max usize when decoding BitString".to_string(),
                self.codec(),
            )
        })?;
        let data = &self.extract_data_by_length(length - 1)?.view_bits::<Msb0>()
            [..(data_bit_length - num_unused_bits as usize)];
        Ok(data.into())
    }

    fn parse_known_multiplier_string<
        T: crate::types::strings::StaticPermittedAlphabet
            + crate::types::AsnType
            + for<'a> TryFrom<&'a [u8], Error = crate::error::strings::PermittedAlphabetError>,
    >(
        &mut self,
        constraints: &Constraints,
    ) -> Result<T, DecodeError> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let data = self.extract_data_by_length(*size.constraint.as_start().unwrap())?;
                return T::try_from(data)
                    .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()));
            }
        }
        let length = self.decode_length()?;
        T::try_from(self.extract_data_by_length(length)?)
            .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    #[track_caller]
    fn require_field(&mut self, tag: Tag) -> Result<bool, DecodeError> {
        let (fields, index) = &mut self.fields;
        let Some(field) = fields.get(*index) else {
            return Err(DecodeError::missing_tag_class_or_value_in_sequence_or_set(
                tag.class,
                tag.value,
                self.codec(),
            ));
        };

        *index += 1;
        match field {
            Some(field) if field.tag_tree.smallest_tag() == tag => Ok(true),
            None => Ok(false),
            _ => Err(DecodeError::missing_tag_class_or_value_in_sequence_or_set(
                tag.class,
                tag.value,
                self.codec(),
            )),
        }
    }

    fn extension_is_present(&mut self) -> Result<Option<&Field>, DecodeError> {
        let codec = self.codec();
        let Some(Some((fields, index))) = self.extensions_present.as_mut() else {
            return Err(DecodeError::type_not_extensible(codec));
        };

        let field = fields
            .get(*index)
            .ok_or_else(|| DecodeError::type_not_extensible(codec))?;

        *index += 1;
        Ok(field.as_ref())
    }

    fn parse_extension_header(&mut self) -> Result<bool, DecodeError> {
        match self.extensions_present {
            Some(Some(_)) => return Ok(true),
            Some(None) => (),
            None => return Ok(false),
        }
        let extensions_length = self.decode_length()?;
        // If length is 0, then there is only initial octet
        if extensions_length < 1u8.into() {
            return Err(OerDecodeErrorKind::invalid_extension_header(
                "Extension length should be at least 1 byte".to_string(),
            ));
        }
        let extension_fields = self
            .extension_fields
            .ok_or_else(|| DecodeError::type_not_extensible(self.codec()))?;
        // Must be at least 8 bits at this point or error is already raised
        let bitfield_bytes = self.extract_data_by_length(extensions_length)?;
        let (first_byte, bitfield) = bitfield_bytes.split_first().ok_or_else(|| {
            OerDecodeErrorKind::invalid_extension_header("Missing initial octet".to_string())
        })?;
        let unused_bits = *first_byte as usize;

        if unused_bits > 7 || unused_bits > bitfield.len() * 8 {
            return Err(OerDecodeErrorKind::invalid_extension_header(
                "Invalid extension bitfield initial octet".to_string(),
            ));
        }
        let mut fields: [Option<Field>; EFC] = [None; EFC];
        for (i, field) in extension_fields.iter().enumerate() {
            let byte_idx = i / 8;
            let bit_idx = 7 - (i & 7); // This gives us MSB0 ordering within each byte
            let is_set = byte_idx < bitfield.len() && (bitfield[byte_idx] & (1 << bit_idx)) != 0;

            if field.is_not_optional_or_default() && !is_set {
                return Err(DecodeError::required_extension_not_present(
                    field.tag,
                    self.codec(),
                ));
            } else if is_set {
                fields[i] = Some(field);
            }
        }

        self.extensions_present = Some(Some((fields, 0)));
        Ok(true)
    }

    fn parse_preamble<const RC: usize, const EC: usize, D>(
        &mut self,
    ) -> Result<([bool; RC], bool), DecodeError>
    where
        D: Constructed<RC, EC>,
    {
        let is_extensible = D::IS_EXTENSIBLE;
        let preamble_width =
            D::FIELDS.number_of_optional_and_default_fields() + is_extensible as usize;
        let bytes = self.extract_data_by_length((preamble_width + 7) / 8)?;

        let mut result = [false; RC];
        let mut extensible_present = false;

        // Process each preamble bit we need
        for i in 0..preamble_width {
            let byte_idx = i / 8;
            let bit_idx = 7 - (i & 7);
            let is_set: bool = (bytes[byte_idx] & (1 << bit_idx)) != 0;

            if i == 0 && is_extensible {
                extensible_present = is_set;
            } else if i - (is_extensible as usize) < RC {
                result[i - is_extensible as usize] = is_set;
            }
        }

        // Check that remaining bits are zero
        let remaining_bits_start = preamble_width;
        for i in remaining_bits_start..bytes.len() * 8 {
            let byte_idx = i / 8;
            let bit_idx = 7 - (i & 7);
            if (bytes[byte_idx] & (1 << bit_idx)) != 0 {
                return Err(OerDecodeErrorKind::invalid_preamble(
                    "Preamble unused bits should be all zero.".to_string(),
                ));
            }
        }

        Ok((result, extensible_present))
    }
}
impl<'input, const RFC: usize, const EFC: usize> crate::Decoder for Decoder<'input, RFC, EFC> {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Decoder<'input, R, E>;

    fn codec(&self) -> Codec {
        self.codec()
    }

    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        panic!("Not every type can be decoded as Any in OER.")
    }

    fn decode_bit_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<BitString, Self::Error> {
        self.parse_bit_string(&constraints)
    }

    /// One octet is used to present bool, false is 0x0 and true is value up to 0xff
    /// In COER, only 0x0 and 0xff are valid values
    fn decode_bool(&mut self, _: Tag) -> Result<bool, Self::Error> {
        let byte = self.parse_one_byte()?;
        Ok(match byte {
            0 => false,
            0xFF => true,
            _ if self.options.encoding_rules.is_oer() => true,
            _ => {
                return Err(DecodeError::from_kind(
                    DecodeErrorKind::InvalidBool { value: byte },
                    self.codec(),
                ))
            }
        })
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _: Tag) -> Result<E, Self::Error> {
        let byte = self.parse_one_byte()?;
        if byte < 128 {
            // Short form, use value directly as unsigned integer
            E::from_discriminant(isize::from(byte))
                .ok_or_else(|| DecodeError::discriminant_value_not_found(byte.into(), self.codec()))
        } else {
            // Long form, value as signed integer. Previous byte is length of the subsequent octets
            let length = byte & 0x7fu8;
            let discriminant: isize = self
                .decode_integer_from_bytes(true, Some(length.into()))
                .map_err(|e| {
                    if matches!(&*e.kind, DecodeErrorKind::IntegerOverflow { .. }) {
                        DecodeError::length_exceeds_platform_width(
                            "Enumerated discriminant value too large for this platform."
                                .to_string(),
                            self.codec(),
                        )
                    } else {
                        e
                    }
                })?;

            if (0..128).contains(&discriminant) && self.options.encoding_rules.is_coer() {
                return Err(CoerDecodeErrorKind::NotValidCanonicalEncoding {
                    msg: "Enumerated discriminant should have been encoded in short form."
                        .to_string(),
                }
                .into());
            }
            E::from_discriminant(discriminant).ok_or_else(|| {
                DecodeError::discriminant_value_not_found(discriminant, self.codec())
            })
        }
    }

    fn decode_integer<I: crate::types::IntegerType>(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<I, Self::Error> {
        self.decode_integer_with_constraints::<I>(&constraints)
    }

    fn decode_real<R: crate::types::RealType>(
        &mut self,
        _: Tag,
        _: Constraints,
    ) -> Result<R, Self::Error> {
        let octets = self.extract_data_by_length(R::BYTE_WIDTH)?;
        R::try_from_ieee754_bytes(octets)
            .map_err(|_| DecodeError::from_kind(DecodeErrorKind::InvalidRealEncoding, self.codec()))
    }

    /// Null contains no data, so we just skip
    fn decode_null(&mut self, _: Tag) -> Result<(), Self::Error> {
        Ok(())
    }

    fn decode_object_identifier(&mut self, _: Tag) -> Result<ObjectIdentifier, Self::Error> {
        let length = self.decode_length()?;
        let ber_decoder = crate::ber::de::Decoder::new(&[], crate::ber::de::DecoderOptions::ber());
        ber_decoder.decode_object_identifier_from_bytes(self.extract_data_by_length(length)?)
    }

    fn decode_sequence<const RC: usize, const EC: usize, D, DF: FnOnce() -> D, F>(
        &mut self,
        _: Tag,
        default_initializer_fn: Option<DF>,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: Constructed<RC, EC>,
        F: FnOnce(&mut Self::AnyDecoder<RC, EC>) -> Result<D, Self::Error>,
    {
        // If there are no fields then the sequence is empty
        // Or if all fields are optional and default and there is no data
        if D::FIELDS.is_empty()
            || D::FIELDS.len() == D::FIELDS.number_of_optional_and_default_fields()
                && self.input.is_empty()
        {
            if let Some(default_initializer_fn) = default_initializer_fn {
                return Ok((default_initializer_fn)());
            }
            return Err(DecodeError::from_kind(
                DecodeErrorKind::UnexpectedEmptyInput,
                self.codec(),
            ));
        }
        // ### PREAMBLE ###
        let (bitmap, extensible_present) = self.parse_preamble::<RC, EC, D>()?;
        // ### ENDS
        let mut fields = ([None; RC], 0);
        D::FIELDS
            .optional_and_default_fields()
            .zip(bitmap)
            .enumerate()
            .for_each(|(i, (field, is_present))| {
                if is_present {
                    fields.0[i] = Some(field);
                } else {
                    fields.0[i] = None;
                }
            });

        let value = {
            let mut sequence_decoder = Decoder::new(self.input, self.options);
            sequence_decoder.extension_fields = D::EXTENDED_FIELDS;
            sequence_decoder.extensions_present = extensible_present.then_some(None);
            sequence_decoder.fields = fields;
            let value = decode_fn(&mut sequence_decoder)?;

            self.input = sequence_decoder.input;
            value
        };

        Ok(value)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _: Tag,
        _: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        let length_of_quantity = self.decode_length()?;
        let coer = self.options.encoding_rules.is_coer();
        let length_bytes = self.extract_data_by_length(length_of_quantity)?;
        if coer && length_bytes.first() == Some(&0) && length_bytes.len() > 1 {
            return Err(CoerDecodeErrorKind::NotValidCanonicalEncoding {
                msg: "Quantity value in 'sequence/set of' should not have leading zeroes in COER"
                    .to_string(),
            }
            .into());
        }
        let length = usize::try_from_unsigned_bytes(length_bytes, self.codec())?;
        let mut sequence_of: Vec<D> = Vec::with_capacity(length);
        let mut decoder = Self::new(self.input, self.options);
        for _ in 0..length {
            let value = D::decode(&mut decoder)?;
            self.input = decoder.input;
            sequence_of.push(value);
        }
        Ok(sequence_of)
    }

    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        self.decode_sequence_of(tag, constraints)
            .map(|seq| SetOf::from_vec(seq))
    }

    fn decode_octet_string<'b, T: From<&'b [u8]> + From<Vec<u8>>>(
        &'b mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<T, Self::Error> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let data =
                    self.extract_data_by_length(*size.constraint.as_start().ok_or_else(|| {
                        DecodeError::size_constraint_not_satisfied(
                            None,
                            "Fixed size constraint should have value when decoding Octet String"
                                .to_string(),
                            self.codec(),
                        )
                    })?)?;
                return Ok(T::from(data));
            }
        }
        let length = self.decode_length()?;
        let data = self.extract_data_by_length(length)?;
        Ok(T::from(data))
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<String, Self::Error> {
        self.decode_octet_string(tag, constraints)
            .and_then(|bytes| {
                String::from_utf8(bytes).map_err(|e| {
                    DecodeError::string_conversion_failed(
                        Tag::UTF8_STRING,
                        e.to_string(),
                        self.codec(),
                    )
                })
            })
    }

    fn decode_visible_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_general_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_ia5_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<Ia5String, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_printable_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_numeric_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<NumericString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_teletex_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<TeletexString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_bmp_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<BmpString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }
    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        tag: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_tag(tag)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error> {
        // Whether we have a choice here
        if D::TAG == Tag::EOC {
            D::decode(self)
        } else {
            D::decode_with_tag(self, tag)
        }
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<UtcTime, Self::Error> {
        let string = String::from_utf8(self.decode_octet_string(tag, Constraints::default())?)
            .map_err(|_| {
                DecodeError::string_conversion_failed(
                    Tag::UTF8_STRING,
                    "UTCTime should be UTF8 encoded.".to_string(),
                    self.codec(),
                )
            })?;
        crate::der::de::Decoder::parse_canonical_utc_time_string(&string)
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<GeneralizedTime, Self::Error> {
        let string = String::from_utf8(self.decode_octet_string(tag, Constraints::default())?)
            .map_err(|_| {
                DecodeError::string_conversion_failed(
                    Tag::UTF8_STRING,
                    "GeneralizedTime should be UTF8 encoded".to_string(),
                    self.codec(),
                )
            })?;
        crate::der::de::Decoder::parse_canonical_generalized_time_string(string)
    }

    fn decode_date(&mut self, tag: Tag) -> Result<types::Date, Self::Error> {
        let string = String::from_utf8(self.decode_octet_string(tag, Constraints::default())?)
            .map_err(|_| {
                DecodeError::string_conversion_failed(
                    Tag::UTF8_STRING,
                    "DATE should be UTF8 encoded".to_string(),
                    self.codec(),
                )
            })?;
        crate::der::de::Decoder::parse_date_string(&string)
    }

    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        _: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + Constructed<RC, EC>,
        FIELDS: Decode,
        D: Fn(&mut Self::AnyDecoder<RC, EC>, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let (bitmap, extensible_present) = self.parse_preamble::<RC, EC, SET>()?;

        let mut field_map: ([Option<Field>; RC], usize) = ([None; RC], 0);
        for (i, (field, is_present)) in SET::FIELDS
            .canonised()
            .optional_and_default_fields()
            .zip(bitmap)
            .enumerate()
        {
            if is_present {
                field_map.0[i] = Some(field);
            } else {
                field_map.0[i] = None;
            }
        }

        let fields = {
            let extended_fields_len = SET::EXTENDED_FIELDS.map_or(0, |fields| fields.len());
            let mut fields = Vec::with_capacity(SET::FIELDS.len() + extended_fields_len);
            let mut set_decoder = Decoder::new(self.input, self.options);
            set_decoder.extension_fields = SET::EXTENDED_FIELDS;
            set_decoder.extensions_present = extensible_present.then_some(None);
            set_decoder.fields = field_map;

            let mut opt_index = 0;
            for field in SET::FIELDS.canonised().iter() {
                if field.is_optional_or_default() {
                    // Safe unwrap, we just created the field_map
                    if field_map.0.get(opt_index).unwrap().is_some() {
                        fields.push(decode_fn(&mut set_decoder, field.index, field.tag)?);
                    }
                    opt_index += 1;
                } else {
                    fields.push(decode_fn(&mut set_decoder, field.index, field.tag)?);
                }
            }
            for (indice, field) in SET::EXTENDED_FIELDS
                .iter()
                .flat_map(Fields::iter)
                .enumerate()
            {
                fields.push(decode_fn(
                    &mut set_decoder,
                    indice + SET::FIELDS.len(),
                    field.tag,
                )?);
            }

            self.input = set_decoder.input;
            fields
        };

        field_fn(fields)
    }

    fn decode_choice<D>(&mut self, constraints: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        let is_extensible = constraints.extensible();
        let tag: Tag = self.parse_tag()?;
        let is_root_extension = crate::types::TagTree::tag_contains(&tag, D::VARIANTS);
        let is_extended_extension =
            crate::types::TagTree::tag_contains(&tag, D::EXTENDED_VARIANTS.unwrap_or(&[]));
        if is_root_extension {
            D::from_tag(self, tag)
        } else if is_extensible && is_extended_extension {
            let options = self.options;
            let length = self.decode_length()?;
            let bytes = self.extract_data_by_length(length)?;
            let mut decoder = Decoder::<0, 0>::new(bytes, options);
            D::from_tag(&mut decoder, tag)
        } else {
            return Err(OerDecodeErrorKind::invalid_tag_variant_on_choice(
                tag,
                is_extensible,
            ));
        }
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_tag(D::TAG)
    }

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
    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_tag_and_constraints::<D>(tag, constraints)
    }

    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        _tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        if !self.parse_extension_header()? {
            return Ok(None);
        }

        let extension_is_present = self.extension_is_present()?.is_some();

        if !extension_is_present {
            return Ok(None);
        }

        // Values of the extensions are only left, encoded as Open type
        let options = self.options;
        let bytes: Cow<[u8]> =
            self.decode_octet_string(Tag::OCTET_STRING, Constraints::default())?;
        let mut decoder = Decoder::<0, 0>::new(&bytes, options);
        D::decode_with_constraints(&mut decoder, constraints).map(Some)
    }

    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: Decode + Constructed<RC, EC>,
    >(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        if !self.parse_extension_header()? {
            return Ok(None);
        }

        let extension_is_present = self.extension_is_present()?.is_some();

        if !extension_is_present {
            return Ok(None);
        }

        // Values of the extensions are only left, inner type encoded as Open type
        let options = self.options;
        let bytes: Cow<[u8]> =
            self.decode_octet_string(Tag::OCTET_STRING, Constraints::default())?;
        let mut decoder = Decoder::<0, 0>::new(&bytes, options);
        D::decode(&mut decoder).map(Some)
    }
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use num_bigint::BigUint;
    // Max length for data type can be 2^1016, below presented as byte array of unsigned int
    const MAX_LENGTH: [u8; 127] = [0xff; 127];
    const MAX_LENGTH_LENGTH: usize = MAX_LENGTH.len();
    use super::*;
    use crate::macros::{constraints, value_constraint};
    use crate::types::constraints::Constraints;
    use bitvec::prelude::BitSlice;
    use num_bigint::BigInt;

    #[test]
    fn test_decode_bool() {
        let decoded: bool = crate::oer::decode(&[0xffu8]).unwrap();
        assert!(decoded);
        let decoded: bool = crate::oer::decode(&[0u8]).unwrap();
        assert!(!decoded);
        let decoded: bool = crate::oer::decode(&[0xffu8, 0xff]).unwrap();
        assert!(decoded);
        let decoded: bool = crate::oer::decode(&[0x33u8, 0x0]).unwrap();
        assert!(decoded);
    }

    #[test]
    fn test_decode_length_invalid() {
        let data = &[0xffu8];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        // Length determinant is > 127 without subsequent bytes
        assert!(decoder.decode_length().is_err());
        // Still missing some data
        let data = &[0xffu8, 0xff];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        // Length determinant is > 127 without subsequent bytes
        assert!(decoder.decode_length().is_err());
    }

    #[test]
    fn test_decode_length_valid() {
        // Max length
        let max_length: BigUint = BigUint::from(2u8).pow(1016u32) - BigUint::from(1u8);
        assert_eq!(max_length.to_bytes_be(), MAX_LENGTH);
        assert_eq!(max_length.to_bytes_be().len(), MAX_LENGTH_LENGTH);
        // Unfortunately we cannot support lengths > 2^64 - 1 at the moment
        // Nor larger than BitSlice::<usize>::MAX_BITS
        assert!(max_length > usize::MAX.into());
        assert!(usize::MAX > BitSlice::<usize>::MAX_BITS);

        // # SHORT FORM
        let data = &[0x01u8, 0xff];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), 1);
        let data = &[0x03u8, 0xff, 0xff, 0xfe];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), 3);
        // Max for short form
        let mut data: [u8; 0x80] = [0xffu8; 0x80];
        data[0] = 0x7f; // length determinant
        let data = &data;
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), 127);

        // # LONG FORM
        // Length of the length should be 2 octets, 0x7f - 0x82 = 2, length is 258 octets
        let length: [u8; 1] = [0x82u8]; // first bit is 1, remaining tells length of the length
        let length_determinant: [u8; 0x02] = [0x01u8, 0x02];
        let data: [u8; 258] = [0xffu8; 258];
        let mut combined: [u8; 261] = [0x0; 261];
        combined[..1].copy_from_slice(&length);
        combined[1..=2].copy_from_slice(&length_determinant);
        combined[3..].copy_from_slice(&data);

        let data = &combined;
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), 258usize);
    }
    #[test]
    fn test_long_form_length_decode() {
        let vc = &[
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
        let mut decoder = Decoder::<0, 0>::new(vc, DecoderOptions::oer());
        let number = BigInt::from(256).pow(127) - 1;
        let constraints = Constraints::default();
        let new_number: BigInt = decoder
            .decode_integer_with_constraints(&constraints)
            .unwrap();
        assert_eq!(new_number, number);

        // Test maximum possible length
        let length: [u8; 1] = [0x80u8 + u8::try_from(usize::BITS / 8u32).unwrap()]; // first bit is 1, remaining tells length of the length
        let length_determinant: [u8; (usize::BITS / 8u32) as usize] =
            [0xff; (usize::BITS / 8u32) as usize];
        let mut combined: [u8; 1 + (usize::BITS / 8u32) as usize] =
            [0x0; 1 + (usize::BITS / 8u32) as usize];
        combined[..1].copy_from_slice(&length);
        combined[1..=(usize::BITS / 8u32) as usize].copy_from_slice(&length_determinant);
        let data = &combined;
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        let new_length = decoder.decode_length().unwrap();
        assert_eq!(new_length, usize::MAX);
        // Test length > usize::MAX
        let length: [u8; 1] = [0x80u8 + u8::try_from(usize::BITS / 8u32).unwrap() + 1]; // first bit is 1, remaining tells length of the length
        let length_determinant: [u8; (usize::BITS / 8u32) as usize + 1] =
            [0xff; (usize::BITS / 8u32) as usize + 1];
        let mut combined: [u8; 1 + (usize::BITS / 8u32) as usize + 1] =
            [0x0; 1 + (usize::BITS / 8u32) as usize + 1];
        combined[..1].copy_from_slice(&length);
        combined[1..=(usize::BITS / 8u32 + 1) as usize].copy_from_slice(&length_determinant);
        let data = &combined;
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        let new_length = decoder.decode_length();
        assert!(new_length.is_err());
    }
    #[test]
    fn test_integer_decode_with_constraints() {
        const CONSTRAINT_1: Constraints = constraints!(value_constraint!(0, 255));
        let data = &[0x01u8];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        let decoded_int: i32 = decoder
            .decode_integer_with_constraints(&CONSTRAINT_1)
            .unwrap();
        assert_eq!(decoded_int, 1);

        let data = &[0xffu8];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        let decoded_int: i64 = decoder
            .decode_integer_with_constraints(&CONSTRAINT_1)
            .unwrap();
        assert_eq!(decoded_int, 255);

        let data = &[0xffu8, 0xff];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        let decoded_int: BigInt = decoder
            .decode_integer_with_constraints(&CONSTRAINT_1)
            .unwrap();
        assert_eq!(decoded_int, 255.into());

        let data = &[0x02u8, 0xff, 0x01];
        let mut decoder = Decoder::<0, 0>::new(data, DecoderOptions::oer());
        const CONSTRAINT_2: Constraints = Constraints::default();
        let decoded_int: BigInt = decoder
            .decode_integer_with_constraints(&CONSTRAINT_2)
            .unwrap();
        assert_eq!(decoded_int, BigInt::from(-255));
    }
}
