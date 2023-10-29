// ITU-T X.696 (02/2021) version of OER decoding
// In OER, without knowledge of the type of the value encoded, it is not possible to determine
// the structure of the encoding. In particular, the end of the encoding cannot be determined from
// the encoding itself without knowledge of the type being encoded ITU-T X.696 (6.2).
use super::enc::EncodingRules;
use crate::oer::helpers;
use crate::prelude::{
    Any, BitString, BmpString, Constraints, Constructed, DecodeChoice, Enumerated, GeneralString,
    GeneralizedTime, Ia5String, Integer, NumericString, ObjectIdentifier, PrintableString, SetOf,
    TeletexString, UtcTime, VisibleString,
};
use crate::types::fields::{Field, Fields};
use crate::{de::Error as _, Codec, Decode, Tag};
use alloc::{
    collections::VecDeque,
    string::{String, ToString},
    vec::Vec,
};
use bitvec::field::BitField;
use bitvec::macros::internal::funty::Fundamental;
use nom::{AsBytes, Slice};
use num_bigint::{BigUint, Sign};
use num_integer::div_ceil;
use num_traits::{One, ToPrimitive};

// Max length for data type can be 2^1016, below presented as byte array of unsigned int
const MAX_LENGTH: [u8; 127] = [0xff; 127];
const MAX_LENGTH_LENGTH: usize = MAX_LENGTH.len();
use crate::error::{DecodeError, OerDecodeErrorKind};

type InputSlice<'input> = nom_bitvec::BSlice<'input, u8, bitvec::order::Msb0>;

#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    encoding_rules: EncodingRules, // default COER
    // Strict mode raises error when there are bytes left in buffer after parsing
    strict_mode: bool, // default true
}
impl DecoderOptions {
    #[must_use]
    pub const fn oer() -> Self {
        Self {
            encoding_rules: EncodingRules::Oer,
            strict_mode: false,
        }
    }
    #[must_use]
    pub const fn coer() -> Self {
        Self {
            encoding_rules: EncodingRules::Coer,
            strict_mode: false,
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

pub struct Decoder<'input> {
    input: InputSlice<'input>,
    options: DecoderOptions,
    fields: VecDeque<(Field, bool)>,
    extension_fields: Option<Fields>,
    extensions_present: Option<Option<VecDeque<(Field, bool)>>>,
}

impl<'input> Decoder<'input> {
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
    #[must_use]
    fn codec(&self) -> Codec {
        self.options.current_codec()
    }
    fn parse_one_bit(&mut self) -> Result<bool, DecodeError> {
        let (input, boolean) = nom::bytes::streaming::take(1u8)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = input;
        Ok(boolean[0])
    }
    fn parse_one_byte(&mut self) -> Result<u8, DecodeError> {
        let (input, byte) = nom::bytes::streaming::take(8u8)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

        self.input = input;
        Ok(byte.as_bytes()[0])
    }
    fn drop_bits(&mut self, num: usize) -> Result<(), DecodeError> {
        let (input, _) = nom::bytes::streaming::take(num)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = input;
        Ok(())
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
            if next_byte & 0b1000_0000 > 0 {
                return Err(DecodeError::custom("Invalid tag number, first subsequent byte should not be full zeros after leading bit", self.codec()));
            }
            loop {
                // Constructs tag number from multiple 7-bit sized chunks
                tag_number = tag_number.checked_shl(7).ok_or(DecodeError::custom(
                    "Tag size exceeds platform width",
                    self.codec(),
                ))? | u32::from(next_byte & 0b0111_1111);
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
    /// Max length for data type can be 2^1016 - 1 octets
    fn decode_length(&mut self) -> Result<BigUint, DecodeError> {
        // In OER decoding, there might be cases when there are multiple zero octets as padding
        // or the length is encoded in more than one octet.
        let mut only_zeros: bool = false;
        let mut possible_length: u8 = 0;
        // Parse leading zeros but also note if there are only zeros
        while let Ok(data) = self.parse_one_byte() {
            if data != 0 {
                possible_length = data;
                only_zeros = false;
                break;
            }
            only_zeros = true;
        }
        if only_zeros {
            // Only zeros, length is zero
            return Ok(BigUint::from(0u8));
        }
        if possible_length < 128 {
            Ok(BigUint::from(possible_length))
        } else {
            // We have the length of the length, mask and extract only 7 bis
            let length = possible_length & 0x7fu8;
            // Should not overflow, max size 8 x 127 = 1016 < u16::MAX
            let result: Result<(InputSlice, InputSlice), DecodeError> =
                nom::bytes::streaming::take(length.as_u16() * 8)(self.input)
                    .map_err(|e| DecodeError::map_nom_err(e, self.codec()));

            match result {
                Ok((input, data)) => {
                    self.input = input;
                    let length = BigUint::from_bytes_be(data.as_bytes());
                    if length > usize::MAX.into() {
                        Err(DecodeError::custom(alloc::format!("Length of the incoming data is either incorrect or your device is up by miracle: {length}"), self.codec()))
                    } else {
                        Ok(length)
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
    /// Extracts data from input by length and updates the input
    /// Since we rely on memory and `BitSlice`, we cannot handle larger data length than `0x1fff_ffff_ffff_ffff`
    /// 'length' is the length of the data in bytes (octets)
    /// Returns the data
    fn extract_data_by_length(&mut self, length: BigUint) -> Result<InputSlice, DecodeError> {
        if length == BigUint::from(0u8) {
            return Ok(InputSlice::from(bitvec::slice::BitSlice::from_slice(&[])));
        }
        if &length * 8u8 > BigUint::from(bitvec::slice::BitSlice::<usize>::MAX_BITS) {
            return Err(DecodeError::exceeds_max_length(length, self.codec()));
        }
        let (input, data) = nom::bytes::streaming::take(length.to_usize().unwrap() * 8)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

        self.input = input;
        Ok(data)
    }
    fn decode_integer_from_bytes(
        &mut self,
        signed: bool,
        length: Option<BigUint>,
    ) -> Result<Integer, DecodeError> {
        if let Some(length) = length {
            let data = self.extract_data_by_length(length)?;
            if signed {
                Ok(Integer::from_signed_bytes_be(data.as_bytes()))
            } else {
                Ok(Integer::from_bytes_be(Sign::Plus, data.as_bytes()))
            }
        } else {
            let length = self.decode_length()?;
            let data = self.extract_data_by_length(length)?;
            if signed {
                Ok(Integer::from_signed_bytes_be(data.as_bytes()))
            } else {
                Ok(Integer::from_bytes_be(Sign::Plus, data.as_bytes()))
            }
        }
    }
    fn decode_integer_with_constraints(
        &mut self,
        constraints: &Constraints,
    ) -> Result<Integer, DecodeError> {
        // Only 'value' constraint is OER visible for integer
        if let Some(value) = constraints.value() {
            helpers::determine_integer_size_and_sign(&value, self.input, |_, sign, octets| {
                self.decode_integer_from_bytes(sign, octets.map(BigUint::from))
            })
        } else {
            // No constraints
            self.decode_integer_from_bytes(true, None)
        }
    }
    fn parse_bit_string(&mut self, constraints: &Constraints) -> Result<BitString, DecodeError> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let length = size.constraint.as_start().ok_or_else(|| {
                    Err(DecodeError::custom(
                        "Fixed size constraint should have value",
                        self.codec(),
                    ))
                });
                return match length {
                    Ok(length) => {
                        let bytes_required = div_ceil(*length, 8);
                        let data = self
                            .extract_data_by_length(BigUint::from(bytes_required))?
                            .slice(..*length);
                        Ok(BitString::from_bitslice(&data))
                    }
                    Err(e) => e,
                };
            }
        }
        let length = self.decode_length()?;
        let num_unused_bits = self.parse_one_byte()?;
        if num_unused_bits > 7 {
            return Err(DecodeError::custom(
                "Marked number of unused bits should be less than 8 when decoding BitString",
                self.codec(),
            ));
        }
        // Remove one from length as one describes trailing zeros...
        let data_bit_length: usize = ((&length - 1u8) * 8u8).try_into().map_err(|_| {
            DecodeError::custom(
                "Total length exceeds BitSlice max usize when decoding BitString",
                self.codec(),
            )
        })?;
        let data = self
            .extract_data_by_length(length.clone() - BigUint::from(1u8))?
            .slice(..(data_bit_length - num_unused_bits as usize));
        Ok(BitString::from_bitslice(data.into()))
    }
    fn parse_known_multiplier_string<T: crate::types::strings::StaticPermittedAlphabet>(
        &mut self,
        constraints: &Constraints,
    ) -> Result<T, DecodeError> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let data = self
                    .extract_data_by_length(BigUint::from(*size.constraint.as_start().unwrap()))
                    .map(|data| data.to_bitvec())?;
                return T::try_from_bits(data, 8).map_err(|_| {
                    DecodeError::custom(
                        "Invalid data when decoding known multiplier string and constructing the type",
                    self.codec())
                });
            }
        }
        let length = self.decode_length()?;
        T::try_from_bits(self.extract_data_by_length(length)?.to_bitvec(), 8).map_err(|_| {
            DecodeError::custom(
                "Invalid data when decoding known multiplier string and constructing the type",
                self.codec(),
            )
        })
    }
    fn parse_optional_and_default_field_bitmap(
        &mut self,
        fields: &Fields,
    ) -> Result<InputSlice<'input>, DecodeError> {
        let (input, bitset) =
            nom::bytes::streaming::take(fields.number_of_optional_and_default_fields())(self.input)
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;

        self.input = input;
        Ok(bitset)
    }
    #[track_caller]
    fn require_field(&mut self, tag: Tag) -> Result<bool, DecodeError> {
        if self
            .fields
            .front()
            .map(|field| field.0.tag_tree.smallest_tag() == tag)
            .unwrap_or_default()
        {
            Ok(self.fields.pop_front().unwrap().1)
        } else {
            Err(DecodeError::custom(
                alloc::format!(
                    "expected class: {}, value: {} in sequence or set",
                    tag.class,
                    tag.value
                ),
                self.codec(),
            ))
        }
    }
    fn extension_is_present(&mut self) -> Result<Option<(Field, bool)>, DecodeError> {
        let codec = self.codec();
        Ok(self
            .extensions_present
            .as_mut()
            .ok_or_else(|| DecodeError::type_not_extensible(codec))?
            .as_mut()
            .ok_or_else(|| DecodeError::type_not_extensible(codec))?
            .pop_front())
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
            return Err(DecodeError::custom(
                "Extension length should be at least 1 byte",
                self.codec(),
            ));
        }
        // Must be at least 8 bits at this point or error is already raised
        let bitfield = self.extract_data_by_length(extensions_length)?.to_bitvec();
        // let mut missing_bits: bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
        // Initial octet
        let (unused_bits, bitfield) = bitfield.split_at(8);
        let unused_bits: usize = unused_bits.load();

        if unused_bits > bitfield.len() || unused_bits > 7 {
            return Err(DecodeError::custom(
                "Invalid extension bitfield initial octet",
                self.codec(),
            ));
        }
        let (bitfield, _) = bitfield.split_at(bitfield.len() - unused_bits);

        let extensions_present: VecDeque<_> = self
            .extension_fields
            .as_ref()
            .unwrap()
            .iter()
            .zip(bitfield.iter().map(|b| *b))
            .collect();

        for (field, is_present) in &extensions_present {
            if field.is_not_optional_or_default() && !*is_present {
                return Err(DecodeError::required_extension_not_present(
                    field.tag,
                    self.codec(),
                ));
            }
        }
        self.extensions_present = Some(Some(extensions_present));

        Ok(true)
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = DecodeError;

    fn codec(&self) -> Codec {
        self.codec()
    }
    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        todo!()
    }

    fn decode_bit_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<BitString, Self::Error> {
        self.parse_bit_string(&constraints)
    }
    /// One octet is used to present bool, false is 0x0 and true is value up to 0xff
    fn decode_bool(&mut self, _: Tag) -> Result<bool, Self::Error> {
        Ok(self.parse_one_byte()? > 0)
    }

    fn decode_enumerated<E: Enumerated>(&mut self, tag: Tag) -> Result<E, Self::Error> {
        let byte = self.parse_one_byte()?;
        if byte < 128 {
            // Short form
            E::from_discriminant(isize::from(byte))
                .ok_or_else(|| DecodeError::discriminant_value_not_found(byte.into(), self.codec()))
        } else {
            // Long form, value as signed integer
            let length = byte & 0x7fu8;
            let data = self.decode_integer_from_bytes(true, Some(length.into()))?;
            E::from_discriminant(data.to_isize().ok_or_else(|| {
                DecodeError::choice_index_exceeds_platform_width(
                    u32::MAX,
                    data.to_u64().unwrap(),
                    self.codec(),
                )
            })?)
            .ok_or_else(|| {
                DecodeError::discriminant_value_not_found(data.to_isize().unwrap(), self.codec())
            })
        }
    }

    fn decode_integer(&mut self, _: Tag, constraints: Constraints) -> Result<Integer, Self::Error> {
        self.decode_integer_with_constraints(&constraints)
    }

    /// Null contains no data, so we just skip
    fn decode_null(&mut self, _: Tag) -> Result<(), Self::Error> {
        Ok(())
    }

    fn decode_object_identifier(&mut self, _: Tag) -> Result<ObjectIdentifier, Self::Error> {
        let length = self.decode_length()?;
        let ber_decoder = crate::ber::de::Decoder::new(&[], crate::ber::de::DecoderOptions::ber());
        ber_decoder
            .decode_object_identifier_from_bytes(self.extract_data_by_length(length)?.as_bytes())
    }

    fn decode_sequence<D, F>(&mut self, _: Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        D: Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        // ### PREAMBLE ###
        let is_extensible = D::EXTENDED_FIELDS.is_some();
        let extensible_present = if is_extensible {
            self.parse_one_bit()?
        } else {
            false
        };
        let num_opt_default_fields = D::FIELDS.number_of_optional_and_default_fields();
        let mut bitmap: InputSlice = InputSlice::from(bitvec::slice::BitSlice::from_slice(&[]));
        if num_opt_default_fields > 0 {
            bitmap = self.parse_optional_and_default_field_bitmap(&D::FIELDS)?;
            let preamble_length = if is_extensible {
                1usize + bitmap.len()
            } else {
                bitmap.len()
            };
            self.drop_bits(8 - preamble_length % 8)?;
        } else if is_extensible {
            // Rest of the preamble are unused bits
            self.drop_bits(7usize)?;
        }
        debug_assert_eq!(self.input.len() % 8, 0);

        let value = {
            let mut sequence_decoder = Self::new(self.input.0, self.options);
            sequence_decoder.extension_fields = D::EXTENDED_FIELDS;
            sequence_decoder.extensions_present = extensible_present.then_some(None);
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

    fn decode_sequence_of<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        let mut sequence_of = Vec::new();
        let length_of_quantity = self.decode_length()?;
        let length =
            BigUint::from_bytes_be(self.extract_data_by_length(length_of_quantity)?.as_bytes());

        let mut start = BigUint::one();
        while start <= length {
            let mut decoder = Self::new(self.input.0, self.options);
            let value = D::decode(&mut decoder)?;
            self.input = decoder.input;
            sequence_of.push(value);
            start += BigUint::one();
        }
        Ok(sequence_of)
    }

    fn decode_set_of<D: Decode + Ord>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        todo!()
    }

    fn decode_octet_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<Vec<u8>, Self::Error> {
        if let Some(size) = constraints.size() {
            // Fixed size, only data is included
            if size.constraint.is_fixed() && size.extensible.is_none() {
                let data = self
                    .extract_data_by_length(BigUint::from(*size.constraint.as_start().unwrap()))
                    .map(|data| data.as_bytes().to_vec());
                return data;
            }
        }
        let length = self.decode_length()?;
        self.extract_data_by_length(length)
            .map(|data| data.as_bytes().to_vec())
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<String, Self::Error> {
        self.decode_octet_string(tag, constraints)
            .and_then(|bytes| {
                String::from_utf8(bytes)
                    .map_err(|_| DecodeError::custom("Invalid UTF-8 string", self.codec()))
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
        tag: Tag,
        constraints: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        GeneralString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|_| {
            DecodeError::custom(
                "Invalid data when decoding general string and constructing the type",
                self.codec(),
            )
        })
    }

    fn decode_ia5_string(
        &mut self,
        tag: Tag,
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
        tag: Tag,
        constraints: Constraints,
    ) -> Result<TeletexString, Self::Error> {
        TeletexString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|_| {
            DecodeError::custom(
                "Invalid data when decoding teletext string and constructing the type",
                self.codec(),
            )
        })
    }

    fn decode_bmp_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
    ) -> Result<BmpString, Self::Error> {
        self.parse_known_multiplier_string(&constraints)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _tag: Tag) -> Result<D, Self::Error> {
        D::decode(self)
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<UtcTime, Self::Error> {
        let string = String::from_utf8(self.decode_octet_string(tag, Constraints::default())?)
            .map_err(|_| DecodeError::custom("Invalid UTF-8 string", self.codec()))?;
        crate::der::de::Decoder::parse_canonical_utc_time_string(&string)
            .map_err(|err| DecodeError::custom(err.to_string(), self.codec()))
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<GeneralizedTime, Self::Error> {
        let string = String::from_utf8(self.decode_octet_string(tag, Constraints::default())?)
            .map_err(|_| DecodeError::custom("Invalid UTF-8 string", self.codec()))?;
        crate::der::de::Decoder::parse_canonical_generalized_time_string(string)
            .map_err(|err| DecodeError::custom(err.to_string(), self.codec()))
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        tag: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + Constructed,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        // TODO make generic preamble for sequence/set
        // let is_extensible = SET::EXTENDED_FIELDS
        //     .is_some()
        //     .then(|| self.parse_one_bit())
        //     .transpose()?
        //     .unwrap_or_default();
        //
        // let bitmap = self.parse_optional_and_default_field_bitmap(&SET::FIELDS)?;
        // let field_map = SET::FIELDS
        //     .optional_and_default_fields()
        //     .zip(bitmap.into_iter().map(|b| *b))
        //     .collect::<alloc::collections::BTreeMap<_, _>>();
        // ### PREAMBLE ###
        let is_extensible = SET::EXTENDED_FIELDS.is_some();
        let extensible_present = if is_extensible {
            self.parse_one_bit()?
        } else {
            false
        };
        let num_opt_default_fields = SET::FIELDS.number_of_optional_and_default_fields();
        let mut bitmap: InputSlice = InputSlice::from(bitvec::slice::BitSlice::from_slice(&[]));
        if num_opt_default_fields > 0 {
            bitmap = self.parse_optional_and_default_field_bitmap(&SET::FIELDS)?;
            let preamble_length = if is_extensible {
                1usize + bitmap.len()
            } else {
                bitmap.len()
            };
            self.drop_bits(8 - preamble_length % 8)?;
        } else if is_extensible {
            // Rest of the preamble are unused bits
            self.drop_bits(7usize)?;
        }
        debug_assert_eq!(self.input.len() % 8, 0);
        let field_map = SET::FIELDS
            .optional_and_default_fields()
            .zip(bitmap.into_iter().map(|b| *b))
            .collect::<alloc::collections::BTreeMap<_, _>>();

        let fields = {
            let mut fields = Vec::new();
            let mut set_decoder = Self::new(self.input.0, self.options);
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
            for (indice, field) in field_indices {
                match field_map.get(&field).copied() {
                    Some(true) | None => {
                        fields.push((decode_fn)(&mut set_decoder, indice, field.tag)?);
                    }
                    Some(false) => {}
                }
            }

            for (indice, field) in SET::EXTENDED_FIELDS
                .iter()
                .flat_map(Fields::iter)
                .enumerate()
            {
                fields.push((decode_fn)(
                    &mut set_decoder,
                    indice + SET::FIELDS.len(),
                    field.tag,
                )?);
            }

            self.input = set_decoder.input;
            fields
        };

        (field_fn)(fields)
    }

    fn decode_choice<D>(&mut self, constraints: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        let is_extensible = constraints.extensible();
        let tag: Tag = self.parse_tag()?;
        let is_root_extension = crate::TagTree::tag_contains(&tag, D::VARIANTS);
        let is_extended_extension = crate::TagTree::tag_contains(&tag, D::EXTENDED_VARIANTS);
        if is_root_extension {
            D::from_tag(self, tag)
        } else if is_extensible && is_extended_extension {
            let options = self.options;
            let length = self.decode_length()?;
            let bytes = self.extract_data_by_length(length)?;
            let mut decoder = Decoder::new(&bytes, options);
            D::from_tag(&mut decoder, tag)
        } else {
            return Err(DecodeError::custom(
                alloc::format!("Tag not found from the variants of the platform. Extensible status: {is_extensible}"), self.codec()),
            );
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
        todo!()
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        todo!()
    }

    fn decode_extension_addition_with_constraints<D>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
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

        // Values of the extensions are only left
        let bytes = self.input;
        let mut decoder = Decoder::new(&bytes, self.options);

        D::decode_with_constraints(&mut decoder, constraints).map(Some)
    }

    fn decode_extension_addition_group<D: Decode + Constructed>(
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

        // Values of the extensions are only left
        let bytes = self.input;
        let mut decoder = Decoder::new(&bytes, self.options);

        D::decode(&mut decoder).map(Some)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assertions_on_constants)]

    use super::*;
    use crate::types::constraints::{Bounded, Constraint, Constraints, Extensible, Size, Value};
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
        let data: BitString = BitString::from_slice(&[0xffu8]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        // Length determinant is > 127 without subsequent bytes
        assert!(decoder.decode_length().is_err());
        // Still missing some data
        let data: BitString = BitString::from_slice(&[0xffu8, 0xff]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
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
        let data: BitString = BitString::from_slice(&[0x01u8, 0xff]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), BigUint::from(1u8));
        let data: BitString = BitString::from_slice(&[0x03u8, 0xff, 0xff, 0xfe]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), BigUint::from(3u8));
        // Max for short form
        let mut data: [u8; 0x80] = [0xffu8; 0x80];
        data[0] = 0x7f; // length determinant
        let data: BitString = BitString::from_slice(&data);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), BigUint::from(127u8));

        // # LONG FORM
        // Length of the length should be 2 octets, 0x7f - 0x82 = 2, length is 258 octets
        let length: [u8; 1] = [0x82u8]; // first bit is 1, remaining tells length of the length
        let length_determinant: [u8; 0x02] = [0x01u8, 0x02];
        let data: [u8; 258] = [0xffu8; 258];
        let mut combined: [u8; 261] = [0x0; 261];
        combined[..1].copy_from_slice(&length);
        combined[1..=2].copy_from_slice(&length_determinant);
        combined[3..].copy_from_slice(&data);

        let data: BitString = BitString::from_slice(&combined);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        assert_eq!(decoder.decode_length().unwrap(), BigUint::from(258u16));
    }
    #[test]
    fn test_long_form_length_decode() {
        let vc = BitString::from_slice(&[
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
        ]);
        let mut decoder = Decoder::new(&vc, DecoderOptions::oer());
        let number = BigInt::from(256).pow(127) - 1;
        let constraints = Constraints::default();
        let new_number = decoder
            .decode_integer_with_constraints(&constraints)
            .unwrap();
        assert_eq!(new_number, number);
    }
    #[test]
    fn test_integer_decode_with_constraints() {
        let range_bound = Bounded::<i128>::Range {
            start: 0.into(),
            end: 255.into(),
        };
        let value_range = &[Constraint::Value(Extensible::new(Value::new(range_bound)))];
        let consts = Constraints::new(value_range);
        let data = BitString::from_slice(&[0x01u8]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        let decoded_int = decoder.decode_integer_with_constraints(&consts).unwrap();
        assert_eq!(decoded_int, 1.into());

        let data = BitString::from_slice(&[0xffu8]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        let decoded_int = decoder.decode_integer_with_constraints(&consts).unwrap();
        assert_eq!(decoded_int, 255.into());

        let data = BitString::from_slice(&[0xffu8, 0xff]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        let decoded_int = decoder.decode_integer_with_constraints(&consts).unwrap();
        assert_eq!(decoded_int, 255.into());

        let data = BitString::from_slice(&[0x02u8, 0xff, 0x01]);
        let mut decoder = Decoder::new(&data, DecoderOptions::oer());
        let decoded_int = decoder
            .decode_integer_with_constraints(&Constraints::new(&[Constraint::Size(
                Size::new(Bounded::None).into(),
            )]))
            .unwrap();
        assert_eq!(decoded_int, BigInt::from(-255));
    }
}
