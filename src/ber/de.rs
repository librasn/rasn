//! # Decoding BER

mod config;
pub(super) mod parser;

use super::identifier::Identifier;
use crate::{
    types::{
        self,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
        Constraints, Enumerated, Tag,
    },
    Decode,
};
use alloc::{borrow::ToOwned, string::ToString, vec::Vec};
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};

pub use self::config::DecoderOptions;

pub use crate::error::DecodeError;
pub use crate::error::{BerDecodeErrorKind, CodecDecodeError, DecodeErrorKind, DerDecodeErrorKind};
type Result<T, E = DecodeError> = core::result::Result<T, E>;

const EOC: &[u8] = &[0, 0];

/// A BER and variants decoder. Capable of decoding BER, CER, and DER.
pub struct Decoder<'input> {
    input: &'input [u8],
    config: DecoderOptions,
    initial_len: usize,
}

impl<'input> Decoder<'input> {
    /// Return the current codec `Codec` variant
    #[must_use]
    pub fn codec(&self) -> crate::Codec {
        self.config.current_codec()
    }
    /// Create a new [`Decoder`] from the given `input` and `config`.
    #[must_use]
    pub fn new(input: &'input [u8], config: DecoderOptions) -> Self {
        Self {
            input,
            config,
            initial_len: input.len(),
        }
    }

    /// Return a number of the decoded bytes by this decoder
    #[must_use]
    pub fn decoded_len(&self) -> usize {
        self.initial_len - self.input.len()
    }

    fn parse_eoc(&mut self) -> Result<()> {
        let (i, _) = nom::bytes::streaming::tag(EOC)(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        self.input = i;
        Ok(())
    }

    pub(crate) fn parse_value(&mut self, tag: Tag) -> Result<(Identifier, Option<&'input [u8]>)> {
        let (input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, Some(tag))?;
        self.input = input;
        Ok((identifier, contents))
    }

    pub(crate) fn parse_primitive_value(&mut self, tag: Tag) -> Result<(Identifier, &'input [u8])> {
        let (input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, Some(tag))?;
        self.input = input;
        match contents {
            Some(contents) => Ok((identifier, contents)),
            None => Err(BerDecodeErrorKind::IndefiniteLengthNotAllowed.into()),
        }
    }

    /// Parses a constructed ASN.1 value, checking the `tag`, and optionally
    /// checking if the identifier is marked as encoded. This should be true
    /// in all cases except explicit prefixes.
    fn parse_constructed_contents<D, F>(
        &mut self,
        tag: Tag,
        check_identifier: bool,
        decode_fn: F,
    ) -> Result<D>
    where
        F: FnOnce(&mut Self) -> Result<D>,
    {
        let (identifier, contents) = self.parse_value(tag)?;

        BerDecodeErrorKind::assert_tag(tag, identifier.tag)?;

        if check_identifier && identifier.is_primitive() {
            return Err(BerDecodeErrorKind::InvalidConstructedIdentifier.into());
        }

        let (streaming, contents) = match contents {
            Some(contents) => (false, contents),
            None => (true, self.input),
        };

        let mut inner = Self::new(contents, self.config);

        let result = (decode_fn)(&mut inner)?;

        if streaming {
            self.input = inner.input;
            self.parse_eoc()?;
        } else if !inner.input.is_empty() {
            return Err(DecodeError::unexpected_extra_data(
                inner.input.len(),
                self.codec(),
            ));
        }

        Ok(result)
    }
    /// Decode an object identifier from a byte slice in BER format.
    /// Function is public to be used by other codecs.
    pub fn decode_object_identifier_from_bytes(
        &self,
        data: &[u8],
    ) -> Result<crate::types::ObjectIdentifier, DecodeError> {
        use num_traits::ToPrimitive;
        let (mut contents, root_octets) = parser::parse_base128_number(data)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        let the_number = root_octets
            .to_u32()
            .ok_or_else(|| DecodeError::integer_overflow(32u32, self.codec()))?;
        let first: u32;
        let second: u32;
        const MAX_OID_THRESHOLD: u32 = MAX_OID_SECOND_OCTET + 1;
        if the_number > MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD + MAX_OID_SECOND_OCTET {
            first = MAX_OID_FIRST_OCTET;
            second = the_number - MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD;
        } else {
            second = the_number % MAX_OID_THRESHOLD;
            first = (the_number - second) / MAX_OID_THRESHOLD;
        }
        let mut buffer = alloc::vec![first, second];

        while !contents.is_empty() {
            let (c, number) = parser::parse_base128_number(contents)
                .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
            contents = c;
            buffer.push(
                number
                    .to_u32()
                    .ok_or_else(|| DecodeError::integer_overflow(32u32, self.codec()))?,
            );
        }
        crate::types::ObjectIdentifier::new(buffer)
            .ok_or_else(|| BerDecodeErrorKind::InvalidObjectIdentifier.into())
    }
    /// Parse any GeneralizedTime string, allowing for any from ASN.1 definition
    /// TODO, move to type itself?
    pub fn parse_any_generalized_time_string(
        string: alloc::string::String,
    ) -> Result<types::GeneralizedTime, DecodeError> {
        // Reference https://obj-sys.com/asn1tutorial/node14.html
        // If data contains ., 3 decimal places of seconds are expected
        // If data contains explict Z, result is UTC
        // If data contains + or -, explicit timezone is given
        // If neither Z nor + nor -, purely local time is implied
        let len = string.len();
        // Helper function to deal with fractions and without timezone
        let parse_without_timezone = |string: &str| -> Result<NaiveDateTime, DecodeError> {
            // Handle both decimal cases (dot . and comma , )
            let string: &str = &string.replace(",", ".");
            if string.contains('.') {
                // Use chrono to parse the string every time, since we don't the know the number of decimals places
                NaiveDateTime::parse_from_str(string, "%Y%m%d%H%.f")
                    .or_else(|_| NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M%.f"))
                    .or_else(|_| NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M%S%.f"))
                    .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()).into())
            } else {
                let fmt_string = match string.len() {
                    8 => "%Y%m%d",
                    10 => "%Y%m%d%H",
                    12 => "%Y%m%d%H%M",
                    14 => "%Y%m%d%H%M%S",
                    _ => "",
                };
                match fmt_string.len() {
                    l if l > 0 => NaiveDateTime::parse_from_str(string, fmt_string)
                        .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()).into()),
                    _ => Err(BerDecodeErrorKind::invalid_date(string.to_string()).into()),
                }
            }
        };
        if string.ends_with('Z') {
            let naive = parse_without_timezone(&string[..len - 1])?;
            return Ok(naive.and_utc().into());
        }
        // Check for timezone offset
        if len > 5
            && string
                .chars()
                .nth(len - 5)
                .map_or(false, |c| c == '+' || c == '-')
        {
            let naive = parse_without_timezone(&string[..len - 5])?;
            let sign = match string.chars().nth(len - 5) {
                Some('+') => 1,
                Some('-') => -1,
                _ => {
                    return Err(BerDecodeErrorKind::invalid_date(string.to_string()).into());
                }
            };
            let offset_hours = string
                .chars()
                .skip(len - 4)
                .take(2)
                .collect::<alloc::string::String>()
                .parse::<i32>()
                .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()))?;
            let offset_minutes = string
                .chars()
                .skip(len - 2)
                .take(2)
                .collect::<alloc::string::String>()
                .parse::<i32>()
                .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()))?;
            if offset_hours > 23 || offset_minutes > 59 {
                return Err(BerDecodeErrorKind::invalid_date(string.to_string()).into());
            }
            let offset = FixedOffset::east_opt(sign * (offset_hours * 3600 + offset_minutes * 60))
                .ok_or_else(|| BerDecodeErrorKind::invalid_date(string.to_string()))?;
            return Ok(TimeZone::from_local_datetime(&offset, &naive)
                .single()
                .ok_or_else(|| BerDecodeErrorKind::invalid_date(string.to_string()))?);
        }

        // Parse without timezone details
        let naive = parse_without_timezone(&string)?;
        Ok(naive.and_utc().into())
    }
    /// Enforce CER/DER restrictions defined in Section 11.7, strictly raise error on non-compliant
    pub fn parse_canonical_generalized_time_string(
        string: alloc::string::String,
    ) -> Result<types::GeneralizedTime, DecodeError> {
        let len = string.len();
        // Helper function to deal with fractions of seconds and without timezone
        let parse_without_timezone =
            |string: &str| -> core::result::Result<NaiveDateTime, DecodeError> {
                let len = string.len();
                if string.contains('.') {
                    // https://github.com/chronotope/chrono/issues/238#issuecomment-378737786
                    NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M%S%.f")
                        .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()).into())
                } else if len == 14 {
                    NaiveDateTime::parse_from_str(string, "%Y%m%d%H%M%S")
                        .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()).into())
                } else {
                    // CER/DER encoding rules don't allow for timezone offset +/
                    // Or missing seconds/minutes/hours
                    // Or comma , instead of dot .
                    // Or local time without timezone
                    Err(BerDecodeErrorKind::invalid_date(string.to_string()).into())
                }
            };
        if string.ends_with('Z') {
            let naive = parse_without_timezone(&string[..len - 1])?;
            Ok(naive.and_utc().into())
        } else {
            Err(BerDecodeErrorKind::invalid_date(string.to_string()).into())
        }
    }
    /// Parse any UTCTime string, can be any from ASN.1 definition
    /// TODO, move to type itself?
    pub fn parse_any_utc_time_string(
        string: alloc::string::String,
    ) -> Result<types::UtcTime, DecodeError> {
        // When compared to GeneralizedTime, UTC time has no fractions.
        let len = string.len();
        // Largest string, e.g. "820102070000-0500".len() == 17
        if len > 17 {
            return Err(BerDecodeErrorKind::invalid_date(string.to_string()).into());
        }
        let format = if string.contains('Z') {
            if len == 11 {
                "%y%m%d%H%MZ"
            } else {
                "%y%m%d%H%M%SZ"
            }
        } else if len == 15 {
            "%y%m%d%H%M%z"
        } else {
            "%y%m%d%H%M%S%z"
        };
        match len {
            11 | 13 => {
                let naive = NaiveDateTime::parse_from_str(&string, format)
                    .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()))?;
                Ok(naive.and_utc())
            }
            15 | 17 => Ok(DateTime::parse_from_str(&string, format)
                .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()))?
                .into()),
            _ => Err(BerDecodeErrorKind::invalid_date(string.to_string()).into()),
        }
    }

    /// Enforce CER/DER restrictions defined in Section 11.8, strictly raise error on non-compliant
    pub fn parse_canonical_utc_time_string(string: &str) -> Result<types::UtcTime, DecodeError> {
        let len = string.len();
        if string.ends_with('Z') {
            let naive = match len {
                13 => NaiveDateTime::parse_from_str(string, "%y%m%d%H%M%SZ")
                    .map_err(|_| BerDecodeErrorKind::invalid_date(string.to_string()))?,
                _ => Err(BerDecodeErrorKind::invalid_date(string.to_string()))?,
            };
            Ok(naive.and_utc())
        } else {
            Err(BerDecodeErrorKind::invalid_date(string.to_string()).into())
        }
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = DecodeError;

    fn codec(&self) -> crate::Codec {
        Self::codec(self)
    }
    fn decode_any(&mut self) -> Result<types::Any> {
        let (mut input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, None)?;

        if contents.is_none() {
            let (i, _) = self::parser::parse_encoded_value(
                &self.config,
                self.input,
                identifier.tag,
                |input, _| Ok(alloc::vec::Vec::from(input)),
            )?;
            input = i;
        }
        let diff = self.input.len() - input.len();
        let contents = &self.input[..diff];
        self.input = input;

        Ok(types::Any {
            contents: contents.to_vec(),
        })
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool> {
        let (_, contents) = self.parse_primitive_value(tag)?;
        DecodeError::assert_length(1, contents.len(), self.codec())?;
        Ok(match contents[0] {
            0 => false,
            0xFF => true,
            _ if self.config.encoding_rules.is_ber() => true,
            _ => {
                return Err(DecodeError::from_kind(
                    DecodeErrorKind::InvalidBool { value: contents[0] },
                    self.codec(),
                ))
            }
        })
    }

    fn decode_enumerated<E: Enumerated>(&mut self, tag: Tag) -> Result<E> {
        let discriminant = self
            .decode_integer(tag, <_>::default())?
            .try_into()
            .map_err(|e: num_bigint::TryFromBigIntError<types::Integer>| {
                DecodeError::integer_type_conversion_failed(e.to_string(), self.codec())
            })?;

        E::from_discriminant(discriminant)
            .ok_or_else(|| BerDecodeErrorKind::DiscriminantValueNotFound { discriminant }.into())
    }

    fn decode_integer(&mut self, tag: Tag, _: Constraints) -> Result<types::Integer> {
        Ok(types::Integer::from_signed_bytes_be(
            self.parse_primitive_value(tag)?.1,
        ))
    }

    fn decode_octet_string(&mut self, tag: Tag, _: Constraints) -> Result<Vec<u8>> {
        let (identifier, contents) = self.parse_value(tag)?;

        if identifier.is_primitive() {
            match contents {
                Some(c) => Ok(c.to_vec()),
                None => Err(BerDecodeErrorKind::IndefiniteLengthNotAllowed.into()),
            }
        } else if identifier.is_constructed() && self.config.encoding_rules.is_der() {
            Err(DerDecodeErrorKind::ConstructedEncodingNotAllowed.into())
        } else {
            let mut buffer = Vec::new();

            if let Some(mut contents) = contents {
                while !contents.is_empty() {
                    let (c, mut vec) = self::parser::parse_encoded_value(
                        &self.config,
                        contents,
                        Tag::OCTET_STRING,
                        |input, _| Ok(alloc::vec::Vec::from(input)),
                    )?;
                    contents = c;

                    buffer.append(&mut vec);
                }
            } else {
                while !self.input.starts_with(EOC) {
                    let (c, mut vec) = self::parser::parse_encoded_value(
                        &self.config,
                        self.input,
                        Tag::OCTET_STRING,
                        |input, _| Ok(alloc::vec::Vec::from(input)),
                    )?;
                    self.input = c;

                    buffer.append(&mut vec);
                }

                self.parse_eoc()?;
            }

            Ok(buffer)
        }
    }

    fn decode_null(&mut self, tag: Tag) -> Result<()> {
        let (_, contents) = self.parse_primitive_value(tag)?;
        DecodeError::assert_length(0, contents.len(), self.codec())?;
        Ok(())
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        let contents = self.parse_primitive_value(tag)?.1;
        self.decode_object_identifier_from_bytes(contents)
    }

    fn decode_bit_string(&mut self, tag: Tag, _: Constraints) -> Result<types::BitString> {
        let (input, bs) =
            self::parser::parse_encoded_value(&self.config, self.input, tag, |input, codec| {
                let Some(unused_bits) = input.first().copied() else {
                    return Ok(types::BitString::new());
                };

                match unused_bits {
                    // TODO: https://github.com/myrrlyn/bitvec/issues/72
                    bits @ 0..=7 => {
                        let mut buffer = input[1..].to_owned();
                        if let Some(last) = buffer.last_mut() {
                            *last &= !((1 << bits) - 1);
                        }

                        let mut string = types::BitString::from_vec(buffer);
                        let bit_length = string
                            .len()
                            .checked_sub(bits as usize)
                            .ok_or_else(|| DecodeError::invalid_bit_string(unused_bits, codec))?;
                        string.truncate(bit_length);

                        Ok(string)
                    }
                    _ => Err(DecodeError::invalid_bit_string(unused_bits, codec)),
                }
            })?;

        self.input = input;
        Ok(bs)
    }

    fn decode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::VisibleString, Self::Error> {
        types::VisibleString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::VISIBLE_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::Ia5String> {
        types::Ia5String::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::IA5_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::PrintableString> {
        types::PrintableString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::PRINTABLE_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::NumericString> {
        types::NumericString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::NUMERIC_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::TeletexString> {
        types::TeletexString::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::TELETEX_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_bmp_string(&mut self, _: Tag, _constraints: Constraints) -> Result<types::BmpString> {
        todo!()
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::Utf8String> {
        let vec = self.decode_octet_string(tag, constraints)?;
        types::Utf8String::from_utf8(vec).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::UTF8_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GeneralString> {
        <types::GeneralString>::try_from(self.decode_octet_string(tag, constraints)?).map_err(|e| {
            DecodeError::string_conversion_failed(
                types::Tag::GENERAL_STRING,
                e.to_string(),
                self.codec(),
            )
        })
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        let string = self.decode_utf8_string(tag, <_>::default())?;
        if self.config.encoding_rules.is_ber() {
            Self::parse_any_generalized_time_string(string)
        } else {
            Self::parse_canonical_generalized_time_string(string)
        }
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        // Reference https://obj-sys.com/asn1tutorial/node15.html
        let string = self.decode_utf8_string(tag, <_>::default())?;
        if self.config.encoding_rules.is_ber() {
            Self::parse_any_utc_time_string(string)
        } else {
            Self::parse_canonical_utc_time_string(&string)
        }
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        tag: Tag,
        _: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        self.parse_constructed_contents(tag, true, |decoder| {
            let mut items = Vec::new();

            while let Ok(item) = D::decode(decoder) {
                items.push(item);
            }

            Ok(items)
        })
    }

    fn decode_set_of<D: Decode + Ord>(
        &mut self,
        tag: Tag,
        _: Constraints,
    ) -> Result<types::SetOf<D>, Self::Error> {
        self.parse_constructed_contents(tag, true, |decoder| {
            let mut items = types::SetOf::new();

            while let Ok(item) = D::decode(decoder) {
                items.insert(item);
            }

            Ok(items)
        })
    }

    fn decode_sequence<D, F: FnOnce(&mut Self) -> Result<D>>(
        &mut self,
        tag: Tag,
        decode_fn: F,
    ) -> Result<D> {
        self.parse_constructed_contents(tag, true, decode_fn)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        self.parse_constructed_contents(tag, false, D::decode)
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        tag: Tag,
        _decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        self.parse_constructed_contents(tag, true, |decoder| {
            let mut fields = Vec::new();

            while let Ok(value) = FIELDS::decode(decoder) {
                fields.push(value);
            }

            (field_fn)(fields)
        })
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        if D::TAG == Tag::EOC {
            Ok(D::decode(self).ok())
        } else {
            self.decode_optional_with_tag(D::TAG)
        }
    }

    /// Decode an the optional value in a `SEQUENCE` or `SET` with `tag`.
    /// Passing the correct tag is required even when used with codecs where
    /// the tag is not present.
    fn decode_optional_with_tag<D: Decode>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error> {
        Ok(D::decode_with_tag(self, tag).ok())
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        Ok(D::decode_with_constraints(self, constraints).ok())
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        Ok(D::decode_with_tag_and_constraints(self, tag, constraints).ok())
    }

    fn decode_choice<D>(&mut self, _: Constraints) -> Result<D, Self::Error>
    where
        D: crate::types::DecodeChoice,
    {
        let (_, identifier) = parser::parse_identifier_octet(self.input)
            .map_err(|e| DecodeError::map_nom_err(e, self.codec()))?;
        D::from_tag(self, identifier.tag)
    }

    fn decode_extension_addition_with_constraints<D>(
        &mut self,
        // Constraints are irrelevant using BER
        _: Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        <Option<D>>::decode(self)
    }

    fn decode_extension_addition_group<D: Decode + crate::types::Constructed>(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        <Option<D>>::decode(self)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct C2;
    impl AsnType for C2 {
        const TAG: Tag = Tag::new(Class::Context, 2);
    }

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct A3;
    impl AsnType for A3 {
        const TAG: Tag = Tag::new(Class::Application, 3);
    }

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct A7;
    impl AsnType for A7 {
        const TAG: Tag = Tag::new(Class::Application, 7);
    }

    use super::*;
    use crate::types::*;

    fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, DecodeError> {
        let mut decoder = self::Decoder::new(input, self::DecoderOptions::ber());
        match T::decode(&mut decoder) {
            Ok(result) => {
                assert_eq!(decoder.decoded_len(), input.len());
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    #[test]
    fn boolean() {
        assert!(decode::<bool>(&[0x01, 0x01, 0xff]).unwrap());
        assert!(!decode::<bool>(&[0x01, 0x01, 0x00]).unwrap());
    }

    #[test]
    fn tagged_boolean() {
        assert_eq!(
            Explicit::<C2, _>::new(true),
            decode(&[0xa2, 0x03, 0x01, 0x01, 0xff]).unwrap()
        );
    }

    #[test]
    fn integer() {
        assert_eq!(
            32768,
            decode::<i32>(&[0x02, 0x03, 0x00, 0x80, 0x00,]).unwrap()
        );
        assert_eq!(32767, decode::<i32>(&[0x02, 0x02, 0x7f, 0xff]).unwrap());
        assert_eq!(256, decode::<i16>(&[0x02, 0x02, 0x01, 0x00]).unwrap());
        assert_eq!(255, decode::<i16>(&[0x02, 0x02, 0x00, 0xff]).unwrap());
        assert_eq!(128, decode::<i16>(&[0x02, 0x02, 0x00, 0x80]).unwrap());
        assert_eq!(127, decode::<i8>(&[0x02, 0x01, 0x7f]).unwrap());
        assert_eq!(1, decode::<i8>(&[0x02, 0x01, 0x01]).unwrap());
        assert_eq!(0, decode::<i8>(&[0x02, 0x01, 0x00]).unwrap());
        assert_eq!(-1, decode::<i8>(&[0x02, 0x01, 0xff]).unwrap());
        assert_eq!(-128, decode::<i16>(&[0x02, 0x01, 0x80]).unwrap());
        assert_eq!(-129i16, decode::<i16>(&[0x02, 0x02, 0xff, 0x7f]).unwrap());
        assert_eq!(-256i16, decode::<i16>(&[0x02, 0x02, 0xff, 0x00]).unwrap());
        assert_eq!(-32768i32, decode::<i32>(&[0x02, 0x02, 0x80, 0x00]).unwrap());
        assert_eq!(
            -32769i32,
            decode::<i32>(&[0x02, 0x03, 0xff, 0x7f, 0xff]).unwrap()
        );

        let mut data = [0u8; 261];
        data[0] = 0x02;
        data[1] = 0x82;
        data[2] = 0x01;
        data[3] = 0x01;
        data[4] = 0x01;
        let mut bigint = num_bigint::BigInt::from(1);
        bigint <<= 2048;
        assert_eq!(bigint, decode::<num_bigint::BigInt>(&data).unwrap());
    }

    #[test]
    fn octet_string() {
        let octet_string = types::OctetString::from(alloc::vec![1, 2, 3, 4, 5, 6]);
        let primitive_encoded = &[0x4, 0x6, 1, 2, 3, 4, 5, 6];
        let constructed_encoded = &[0x24, 0x80, 0x4, 0x4, 1, 2, 3, 4, 0x4, 0x2, 5, 6, 0x0, 0x0];

        assert_eq!(
            octet_string,
            decode::<types::OctetString>(primitive_encoded).unwrap()
        );
        assert_eq!(
            octet_string,
            decode::<types::OctetString>(constructed_encoded).unwrap()
        );
    }

    #[test]
    fn bit_string() {
        let mut bitstring =
            types::BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());
        bitstring.truncate(bitstring.len() - 4);

        let primitive_encoded: types::BitString =
            decode(&[0x03, 0x07, 0x04, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..]).unwrap();

        let constructed_encoded: types::BitString = decode(
            &[
                0x23, 0x80, // TAG + LENGTH
                0x03, 0x03, 0x00, 0x0A, 0x3B, // Part 1
                0x03, 0x05, 0x04, 0x5F, 0x29, 0x1C, 0xD0, // Part 2
                0x00, 0x00, // EOC
            ][..],
        )
        .unwrap();

        assert_eq!(bitstring, primitive_encoded);
        assert_eq!(bitstring, constructed_encoded);
    }

    #[test]
    fn utf8_string() {
        let name = String::from("Jones");
        let primitive = &[0x0C, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73];
        let definite_constructed = &[
            0x2C, 0x09, // TAG + LENGTH
            0x04, 0x03, // PART 1 TLV
            0x4A, 0x6F, 0x6E, 0x04, 0x02, // PART 2 TLV
            0x65, 0x73,
        ];
        let indefinite_constructed = &[
            0x2C, 0x80, // TAG + LENGTH
            0x04, 0x03, // PART 1 TLV
            0x4A, 0x6F, 0x6E, 0x04, 0x02, // PART 2 TLV
            0x65, 0x73, 0x00, 0x00,
        ];

        assert_eq!(name, decode::<String>(primitive).unwrap());
        assert_eq!(name, decode::<String>(definite_constructed).unwrap());
        assert_eq!(name, decode::<String>(indefinite_constructed).unwrap());
    }

    #[test]
    fn utc_time() {
        let time =
            crate::types::GeneralizedTime::parse_from_str("991231235959+0000", "%y%m%d%H%M%S%z")
                .unwrap();
        // 991231235959Z
        let has_z = &[
            0x17, 0x0D, 0x39, 0x39, 0x31, 0x32, 0x33, 0x31, 0x32, 0x33, 0x35, 0x39, 0x35, 0x39,
            0x5A,
        ];
        // 991231235959+0000
        let has_noz = &[
            0x17, 0x11, 0x39, 0x39, 0x31, 0x32, 0x33, 0x31, 0x32, 0x33, 0x35, 0x39, 0x35, 0x39,
            0x2B, 0x30, 0x30, 0x30, 0x30,
        ];
        assert_eq!(
            time,
            decode::<chrono::DateTime::<chrono::Utc>>(has_z).unwrap()
        );

        assert_eq!(
            time,
            crate::der::decode::<crate::types::UtcTime>(has_z).unwrap()
        );

        assert_eq!(
            time,
            decode::<chrono::DateTime::<chrono::Utc>>(has_noz).unwrap()
        );
        assert!(crate::der::decode::<crate::types::UtcTime>(has_noz).is_err());
    }

    #[test]
    fn generalized_time() {
        let time = crate::types::GeneralizedTime::parse_from_str(
            "20001231205959.999+0000",
            "%Y%m%d%H%M%S%.3f%z",
        )
        .unwrap();
        let has_z = &[
            0x18, 0x13, 0x32, 0x30, 0x30, 0x30, 0x31, 0x32, 0x33, 0x31, 0x32, 0x30, 0x35, 0x39,
            0x35, 0x39, 0x2E, 0x39, 0x39, 0x39, 0x5A,
        ];
        assert_eq!(
            time,
            decode::<chrono::DateTime::<chrono::FixedOffset>>(has_z).unwrap()
        );
    }

    #[test]
    fn sequence_of() {
        let vec = alloc::vec!["Jon", "es"];
        let from_raw: Vec<String> = decode(
            &[
                0x30, 0x9, 0x0C, 0x03, 0x4A, 0x6F, 0x6E, 0x0C, 0x02, 0x65, 0x73,
            ][..],
        )
        .unwrap();

        assert_eq!(vec, from_raw);
    }

    #[test]
    fn sequence() {
        use types::Ia5String;
        // Taken from examples in 8.9 of X.690.
        #[derive(Debug, PartialEq)]
        struct Foo {
            name: Ia5String,
            ok: bool,
        }

        impl types::Constructed for Foo {
            const FIELDS: types::fields::Fields = types::fields::Fields::from_static(&[
                types::fields::Field::new_required(Ia5String::TAG, Ia5String::TAG_TREE),
                types::fields::Field::new_required(bool::TAG, bool::TAG_TREE),
            ]);
        }

        impl types::AsnType for Foo {
            const TAG: Tag = Tag::SEQUENCE;
        }

        impl Decode for Foo {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                _: Constraints,
            ) -> Result<Self, D::Error> {
                decoder.decode_sequence(tag, |sequence| {
                    let name: Ia5String = Ia5String::decode(sequence)?;
                    let ok: bool = bool::decode(sequence)?;
                    Ok(Self { name, ok })
                })
            }
        }

        let foo = Foo {
            name: String::from("Smith").try_into().unwrap(),
            ok: true,
        };
        let bytes = &[
            0x30, 0x0A, // TAG + LENGTH
            0x16, 0x05, 0x53, 0x6d, 0x69, 0x74, 0x68, // Ia5String "Smith"
            0x01, 0x01, 0xff, // BOOL True
        ];

        assert_eq!(foo, decode(bytes).unwrap());
    }

    #[test]
    fn tagging() {
        type Type1 = VisibleString;
        type Type2 = Implicit<A3, Type1>;
        type Type3 = Explicit<C2, Type2>;
        type Type4 = Implicit<A7, Type3>;
        type Type5 = Implicit<C2, Type2>;

        let jones = String::from("Jones");
        let jones1 = Type1::try_from(jones).unwrap();
        let jones2 = Type2::from(jones1.clone());
        let jones3 = Type3::from(jones2.clone());
        let jones4 = Type4::from(jones3.clone());
        let jones5 = Type5::from(jones2.clone());

        assert_eq!(
            jones1,
            decode(&[0x1A, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones2,
            decode(&[0x43, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones3,
            decode(&[0xa2, 0x07, 0x43, 0x5, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones4,
            decode(&[0x67, 0x07, 0x43, 0x5, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones5,
            decode(&[0x82, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
    }

    #[test]
    fn flip1() {
        let _ = decode::<Open>(&[
            0x10, 0x10, 0x23, 0x00, 0xfe, 0x7f, 0x10, 0x03, 0x00, 0xff, 0xe4, 0x04, 0x50, 0x10,
            0x50, 0x10, 0x10, 0x10,
        ]);
    }

    #[test]
    fn any() {
        let expected = &[0x1A, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73];
        assert_eq!(
            Any {
                contents: expected.to_vec()
            },
            decode(expected).unwrap()
        );
    }

    #[test]
    fn any_indefinite() {
        let any = &[
            0x30, 0x80, 0x2C, 0x80, 0x04, 0x03, 0x4A, 0x6F, 0x6E, 0x04, 0x02, 0x65, 0x73, 0x00,
            0x00, 0x00, 0x00,
        ];
        assert_eq!(
            Any {
                contents: any.to_vec()
            },
            decode(any).unwrap(),
        );
    }

    #[test]
    fn any_indefinite_fail_no_eoc() {
        let any = &[
            0x30, 0x80, 0x2C, 0x80, 0x04, 0x03, 0x4A, 0x6F, 0x6E, 0x04, 0x02, 0x65, 0x73, 0x00,
            0x00,
        ];
        assert!(decode::<Any>(any).is_err());
    }

    #[test]
    fn decoding_oid() {
        use crate::Decoder;

        let mut decoder =
            super::Decoder::new(&[0x06, 0x03, 0x88, 0x37, 0x01], DecoderOptions::der());
        let oid = decoder.decode_object_identifier(Tag::OBJECT_IDENTIFIER);
        assert!(oid.is_ok());
        let oid = oid.unwrap();
        assert_eq!(ObjectIdentifier::new([2, 999, 1].to_vec()).unwrap(), oid);
    }
}
