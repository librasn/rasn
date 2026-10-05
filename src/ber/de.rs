//! # Decoding BER

mod config;
pub(super) mod parser;

use crate::{
    Decode,
    types::{
        self, Constraints, Enumerated, Tag,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
    },
};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use parser::{Contents, END_OF_CONTENTS, Parser, Value};

use super::time;

pub use self::config::DecoderOptions;

pub use crate::error::DecodeError;
pub use crate::error::{BerDecodeErrorKind, CodecDecodeError, DecodeErrorKind, DerDecodeErrorKind};
type Result<T, E = DecodeError> = core::result::Result<T, E>;

/// A BER and variants decoder. Capable of decoding BER, CER, and DER.
pub struct Decoder<'input> {
    input: &'input [u8],
    config: DecoderOptions,
    initial_len: usize,
    /// Whether end-of-contents octets (§8.1.5) may close the input, as they
    /// close the contents of an indefinite-length value.
    closed_by_end_of_contents: bool,
}

impl<'input> Decoder<'input> {
    /// Return the current codec `Codec` variant
    #[must_use]
    pub fn codec(&self) -> crate::Codec {
        self.config.current_codec()
    }
    /// Returns reference to the remaining input data that has not been parsed.
    #[must_use]
    pub fn remaining(&self) -> &'input [u8] {
        self.input
    }
    /// Create a new [`Decoder`] from the given `input` and `config`.
    #[must_use]
    pub fn new(input: &'input [u8], config: DecoderOptions) -> Self {
        Self {
            input,
            config,
            initial_len: input.len(),
            closed_by_end_of_contents: config.encoding_rules.allows_indefinite(),
        }
    }

    /// Return a number of the decoded bytes by this decoder
    #[must_use]
    pub fn decoded_len(&self) -> usize {
        self.initial_len - self.input.len()
    }
    /// Peek the value of the next tag
    #[inline]
    pub fn peek_tag(&self) -> Result<Tag> {
        Ok(self.parser().identifier()?.tag)
    }
    /// Generic helper used by the optional decoders.
    /// The function will peek the upcoming tag and only invoke `f` when the tags match.
    /// If tags won't match or input is empty, will return `None`
    fn decode_optional_with_check<D, F>(&mut self, tag: Tag, f: F) -> Result<Option<D>, DecodeError>
    where
        F: FnOnce(&mut Self) -> Result<D, DecodeError>,
    {
        if self.at_end_of_contents() {
            return Ok(None);
        }
        if tag != Tag::EOC {
            let upcoming_tag = self.peek_tag()?;
            if tag != upcoming_tag {
                return Ok(None);
            }
        }
        Ok(Some(f(self)?))
    }

    /// Whether the contents being decoded have ended.
    #[inline]
    fn at_end_of_contents(&self) -> bool {
        self.input.is_empty()
            || (self.closed_by_end_of_contents && self.input.starts_with(END_OF_CONTENTS))
    }

    /// A parser over the remaining input.
    #[inline]
    fn parser(&self) -> Parser<'input> {
        Parser::new(self.config, self.input)
    }

    /// Parses from the remaining input, advancing past what `parse` parsed
    /// only when it succeeds.
    #[inline]
    fn parse<T>(&mut self, parse: impl FnOnce(&mut Parser<'input>) -> Result<T>) -> Result<T> {
        let mut parser = self.parser();
        let parsed = parse(&mut parser)?;
        self.input = parser.remaining();
        Ok(parsed)
    }

    fn parse_end_of_contents(&mut self) -> Result<()> {
        self.parse(Parser::end_of_contents)
    }

    #[inline]
    fn parse_value(&mut self, tag: Tag) -> Result<Value<'input>> {
        self.parse(|parser| parser.value(Some(tag)))
    }

    /// Parses a value that has no indefinite form, returning its contents.
    #[inline]
    fn parse_primitive_value(&mut self, tag: Tag) -> Result<&'input [u8]> {
        match self.parse_value(tag)?.contents {
            Contents::Definite(contents) => Ok(contents),
            Contents::Indefinite => Err(BerDecodeErrorKind::IndefiniteLengthNotAllowed.into()),
        }
    }

    #[inline]
    fn check_recursion_depth(&self) -> Result<()> {
        if self.config.remaining_depth == 0 {
            return Err(self.exceeds_max_parse_depth());
        }
        Ok(())
    }

    #[cold]
    #[inline(never)]
    fn exceeds_max_parse_depth(&self) -> DecodeError {
        DecodeError::from_kind(DecodeErrorKind::ExceedsMaxParseDepth, self.codec())
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
        self.check_recursion_depth()?;
        let Value {
            identifier,
            contents,
        } = self.parse_value(tag)?;

        if check_identifier && identifier.is_primitive() {
            return Err(BerDecodeErrorKind::InvalidConstructedIdentifier.into());
        }

        let (streaming, contents) = match contents {
            Contents::Definite(contents) => (false, contents),
            Contents::Indefinite => (true, self.input),
        };

        let mut inner = Self::new(contents, self.config);
        inner.closed_by_end_of_contents = streaming;
        inner.config.remaining_depth = inner.config.remaining_depth.saturating_sub(1);

        let result = (decode_fn)(&mut inner)?;

        if streaming {
            self.input = inner.input;
            self.parse_end_of_contents()?;
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
        let mut arcs = Parser::new(self.config, data);
        let root_octets = arcs.object_identifier_arc()?;
        let first: u32;
        let second: u32;
        const MAX_OID_THRESHOLD: u32 = MAX_OID_SECOND_OCTET + 1;
        if root_octets > MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD + MAX_OID_SECOND_OCTET {
            first = MAX_OID_FIRST_OCTET;
            second = root_octets - MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD;
        } else {
            second = root_octets % MAX_OID_THRESHOLD;
            first = (root_octets - second) / MAX_OID_THRESHOLD;
        }

        // preallocate some capacity for the OID arcs, maxing out at 16 elements
        // to prevent excessive preallocation from malformed or malicious
        // packets
        let mut buffer = Vec::with_capacity(core::cmp::min(arcs.remaining().len() + 2, 16));
        buffer.push(first);
        buffer.push(second);

        while !arcs.remaining().is_empty() {
            buffer.push(arcs.object_identifier_arc()?);
        }
        crate::types::ObjectIdentifier::new(buffer)
            .ok_or_else(|| BerDecodeErrorKind::InvalidObjectIdentifier.into())
    }
    /// Parses a GeneralizedTime in any of the forms X.680 allows, reading a
    /// value without a time zone as UTC.
    pub fn parse_any_generalized_time_string(
        string: alloc::string::String,
    ) -> Result<types::GeneralizedTime, DecodeError> {
        time::parse_generalized_time(string.as_bytes())
            .ok_or_else(|| BerDecodeErrorKind::invalid_date(string).into())
    }

    /// Parses a GeneralizedTime in the canonical form that X.690 §11.7
    /// requires of CER and DER.
    pub fn parse_canonical_generalized_time_string(
        string: alloc::string::String,
    ) -> Result<types::GeneralizedTime, DecodeError> {
        time::parse_canonical_generalized_time(string.as_bytes())
            .ok_or_else(|| BerDecodeErrorKind::invalid_date(string).into())
    }

    /// Parses a UTCTime in any of the forms X.680 allows.
    pub fn parse_any_utc_time_string(
        string: alloc::string::String,
    ) -> Result<types::UtcTime, DecodeError> {
        time::parse_utc_time(string.as_bytes())
            .ok_or_else(|| BerDecodeErrorKind::invalid_date(string).into())
    }

    /// Parses a UTCTime in the canonical form that X.690 §11.8 requires of
    /// CER and DER.
    pub fn parse_canonical_utc_time_string(string: &str) -> Result<types::UtcTime, DecodeError> {
        time::parse_canonical_utc_time(string.as_bytes())
            .ok_or_else(|| BerDecodeErrorKind::invalid_date(string.to_string()).into())
    }

    /// Parses a DATE, `YYYYMMDD` (X.690 §8.26.2).
    pub fn parse_date_string(string: &str) -> Result<types::Date, DecodeError> {
        time::parse_date(string.as_bytes())
            .ok_or_else(|| BerDecodeErrorKind::invalid_date(string.to_string()).into())
    }

    /// Decodes a time type, whose characters `parse` reads from the string
    /// value tagged `tag`.
    fn decode_time<T>(&mut self, tag: Tag, parse: fn(&[u8]) -> Option<T>) -> Result<T> {
        let characters =
            crate::Decoder::decode_octet_string::<Cow<[u8]>>(self, tag, &Constraints::default())?;
        parse(&characters).ok_or_else(|| {
            BerDecodeErrorKind::invalid_date(String::from_utf8_lossy(&characters).into_owned())
                .into()
        })
    }

    fn check_size_constraint(
        len: usize,
        constraints: &Constraints,
        codec: crate::Codec,
    ) -> Result<()> {
        if let Some(size) = constraints.size()
            && size.extensible.is_none()
        {
            size.constraint.contains_or_else(&len, || {
                DecodeError::size_constraint_not_satisfied(
                    Some(len),
                    size.constraint.to_string(),
                    codec,
                )
            })?;
        }
        Ok(())
    }
}

/// Drops the leading octets of a two's complement integer that only repeat
/// its sign: an octet of all ones or all zeros whose successor starts with
/// the same bit (X.690 §8.3.2).
fn without_redundant_sign_octets(contents: &[u8]) -> &[u8] {
    let sign = contents.first().map_or(0, |first| first & 0x80);
    let redundant = contents
        .windows(2)
        .take_while(|pair| pair[0] == if sign == 0 { 0x00 } else { 0xFF } && pair[1] & 0x80 == sign)
        .count();
    &contents[redundant..]
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Decoder<'input>;

    fn codec(&self) -> crate::Codec {
        Self::codec(self)
    }
    fn decode_any(&mut self, tag: Tag) -> Result<types::Any> {
        // Inside a SEQUENCE or SET the open type is wrapped in an explicit
        // tag, which is stripped; elsewhere the whole encoding is the value.
        let expected = (tag != Tag::EOC).then_some(tag);
        let input = self.input;
        let contents = self.parse(|parser| match parser.value(expected)?.contents {
            Contents::Definite(contents) => Ok(contents),
            Contents::Indefinite => parser.indefinite_contents(),
        })?;
        let encoding = &input[..input.len() - self.input.len()];
        let octets = if expected.is_some() {
            contents
        } else {
            encoding
        };
        Ok(types::Any::new(octets.to_vec()))
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool> {
        let contents = self.parse_primitive_value(tag)?;
        DecodeError::assert_length(1, contents.len(), self.codec())?;
        Ok(match contents[0] {
            0 => false,
            0xFF => true,
            _ if self.config.encoding_rules.is_ber() => true,
            _ => {
                return Err(DecodeError::from_kind(
                    DecodeErrorKind::InvalidBool { value: contents[0] },
                    self.codec(),
                ));
            }
        })
    }

    fn decode_enumerated<E: Enumerated>(&mut self, tag: Tag) -> Result<E> {
        let discriminant = self.decode_integer::<isize>(tag, &Constraints::default())?;

        E::from_discriminant(discriminant)
            .ok_or_else(|| DecodeError::discriminant_value_not_found(discriminant, self.codec()))
    }

    fn decode_integer<I: types::IntegerType>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<I> {
        let mut contents = self.parse_primitive_value(tag)?;
        // An encoding longer than the type may still fit it once the leading
        // octets that only repeat the sign are dropped (X.690 §8.3.2 forbids
        // them, but BER data in the wild carries them).
        if contents.len() > I::BYTE_WIDTH {
            contents = without_redundant_sign_octets(contents);
        }
        let result = match contents.first() {
            // Only a signed type can hold a negative value.
            Some(sign) if sign & 0x80 != 0 => I::try_from_signed_bytes(contents, self.codec())?,
            // A non-negative value may carry one octet more than the type
            // when that octet only keeps the sign bit clear.
            Some(0) if contents.len() == I::BYTE_WIDTH + 1 => {
                I::try_from_unsigned_bytes(&contents[1..], self.codec())?
            }
            _ => I::try_from_bytes(contents, self.codec())?,
        };

        if let Some(value) = constraints.value()
            && value.extensible.is_none()
            && !value.constraint.in_bound(&result)
        {
            return Err(DecodeError::value_constraint_not_satisfied(
                result.to_bigint().unwrap_or_default(),
                value.constraint.value,
                self.codec(),
            ));
        }

        Ok(result)
    }

    fn decode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<R, Self::Error> {
        Err(DecodeError::real_not_supported(self.codec()))
    }

    fn decode_octet_string<'b, T: From<&'b [u8]> + From<Vec<u8>>>(
        &'b mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<T> {
        let mut parser = self.parser();
        let value = parser.value(Some(tag))?;
        // A primitive encoding is borrowed; a constructed one is gathered
        // from its segments.
        if let Contents::Definite(octets) = value.contents
            && value.identifier.is_primitive()
        {
            self.input = parser.remaining();
            Self::check_size_constraint(octets.len(), constraints, self.codec())?;
            return Ok(T::from(octets));
        }
        let octets: Vec<u8> = parser.string_contents(value, Tag::OCTET_STRING)?;
        self.input = parser.remaining();
        Self::check_size_constraint(octets.len(), constraints, self.codec())?;
        Ok(T::from(octets))
    }

    fn decode_null(&mut self, tag: Tag) -> Result<()> {
        let contents = self.parse_primitive_value(tag)?;
        DecodeError::assert_length(0, contents.len(), self.codec())?;
        Ok(())
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        let contents = self.parse_primitive_value(tag)?;
        self.decode_object_identifier_from_bytes(contents)
    }

    fn decode_bit_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::BitString> {
        let string: types::BitString = self.parse(|parser| parser.string(tag, Tag::BIT_STRING))?;
        Self::check_size_constraint(string.len(), constraints, self.codec())?;
        Ok(string)
    }

    fn decode_visible_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::VisibleString, Self::Error> {
        types::VisibleString::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::Ia5String> {
        types::Ia5String::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_printable_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::PrintableString> {
        types::PrintableString::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::NumericString> {
        types::NumericString::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::TeletexString> {
        types::TeletexString::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::BmpString> {
        types::BmpString::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
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
        constraints: &Constraints,
    ) -> Result<types::GeneralString> {
        <types::GeneralString>::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::GraphicString> {
        <types::GraphicString>::try_from(
            self.decode_octet_string::<Cow<[u8]>>(tag, constraints)?
                .as_ref(),
        )
        .map_err(|e| DecodeError::permitted_alphabet_error(e, self.codec()))
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        let parse = if self.config.encoding_rules.is_ber() {
            time::parse_generalized_time
        } else {
            time::parse_canonical_generalized_time
        };
        self.decode_time(tag, parse)
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        let parse = if self.config.encoding_rules.is_ber() {
            time::parse_utc_time
        } else {
            time::parse_canonical_utc_time
        };
        self.decode_time(tag, parse)
    }

    fn decode_date(&mut self, tag: Tag) -> core::result::Result<types::Date, Self::Error> {
        self.decode_time(tag, time::parse_date)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        let items = self.parse_constructed_contents(tag, true, |decoder| {
            decoder.config.remaining_depth = decoder.config.remaining_depth.saturating_sub(1);
            let mut items = Vec::new();
            while !decoder.at_end_of_contents() {
                items.push(D::decode(decoder)?);
            }
            Ok(items)
        })?;
        Self::check_size_constraint(items.len(), constraints, self.codec())?;
        Ok(items)
    }

    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<types::SetOf<D>, Self::Error> {
        let items = self.parse_constructed_contents(tag, true, |decoder| {
            decoder.config.remaining_depth = decoder.config.remaining_depth.saturating_sub(1);
            let mut items = types::SetOf::new();
            while !decoder.at_end_of_contents() {
                items.insert(D::decode(decoder)?);
            }
            Ok(items)
        })?;
        Self::check_size_constraint(items.len(), constraints, self.codec())?;
        Ok(items)
    }

    fn decode_sequence<
        const RC: usize,
        const EC: usize,
        D: crate::types::Constructed<RC, EC>,
        DF: FnOnce() -> D,
        F: FnOnce(&mut Self) -> Result<D>,
    >(
        &mut self,
        tag: Tag,
        default_initializer_fn: Option<DF>,
        decode_fn: F,
    ) -> Result<D> {
        self.parse_constructed_contents(tag, true, |decoder| {
            // If there are no fields, or the input is empty and we know that
            // all fields are optional or default fields, we call the default
            // initializer and skip calling the decode function at all.
            if D::FIELDS.is_empty() && D::EXTENDED_FIELDS.is_none()
                || (D::FIELDS.len() == D::FIELDS.number_of_optional_and_default_fields()
                    && decoder.input.is_empty())
            {
                if let Some(default_initializer_fn) = default_initializer_fn {
                    return Ok((default_initializer_fn)());
                }
                return Err(DecodeError::from_kind(
                    DecodeErrorKind::UnexpectedEmptyInput,
                    decoder.codec(),
                ));
            }
            (decode_fn)(decoder)
        })
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        self.parse_constructed_contents(tag, false, D::decode)
    }
    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        tag: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_check(tag, |decoder| decoder.decode_explicit_prefix(tag))
    }

    fn decode_set<const RL: usize, const EL: usize, FIELDS, SET, D, F>(
        &mut self,
        tag: Tag,
        _decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed<RL, EL>,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        self.parse_constructed_contents(tag, true, |decoder| {
            let mut fields = Vec::new();
            while !decoder.at_end_of_contents() {
                fields.push(FIELDS::decode(decoder)?);
            }
            (field_fn)(fields)
        })
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_check(D::TAG, |decoder| D::decode(decoder))
    }

    /// Decode the optional value in a `SEQUENCE` or `SET` with `tag`.
    /// Passing the correct tag is required even when used with codecs where
    /// the tag is not present.
    fn decode_optional_with_tag<D: Decode>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_check(tag, |decoder| D::decode_with_tag(decoder, tag))
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        constraints: &Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_check(D::TAG, |decoder| {
            D::decode_with_constraints(decoder, constraints)
        })
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional_with_check(tag, |decoder| {
            D::decode_with_tag_and_constraints(decoder, tag, constraints)
        })
    }

    fn decode_choice<D>(&mut self, _: &Constraints) -> Result<D, Self::Error>
    where
        D: crate::types::DecodeChoice,
    {
        let tag = self.peek_tag()?;
        D::from_tag(self, tag)
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        _constraints: &Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_explicit_prefix(tag).map(Some)
    }

    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        // Constraints are irrelevant using BER
        _: &Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        <Option<D>>::decode_with_tag(self, tag)
    }

    fn decode_extension_addition_group<
        const RL: usize,
        const EL: usize,
        D: Decode + crate::types::Constructed<RL, EL>,
    >(
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

        let constructed_definite_encoded: types::BitString = decode(
            &[
                0x23, 0x0C, // TAG + LENGTH
                0x03, 0x03, 0x00, 0x0A, 0x3B, // Part 1
                0x03, 0x05, 0x04, 0x5F, 0x29, 0x1C, 0xD0, // Part 2
            ][..],
        )
        .unwrap();

        assert_eq!(bitstring, primitive_encoded);
        assert_eq!(bitstring, constructed_encoded);
        assert_eq!(bitstring, constructed_definite_encoded);

        let empty_bitstring_primitive_encoded: types::BitString =
            decode(&[0x03, 0x01, 0x00][..]).unwrap();
        assert_eq!(
            types::BitString::from_vec(vec![]),
            empty_bitstring_primitive_encoded
        );

        assert!(decode::<types::BitString>(&[0x03, 0x00][..]).is_err());
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

        impl types::Constructed<2, 0> for Foo {
            const FIELDS: types::fields::Fields<2> = types::fields::Fields::from_static([
                types::fields::Field::new_required(0, Ia5String::TAG, Ia5String::TAG_TREE, "name"),
                types::fields::Field::new_required(1, bool::TAG, bool::TAG_TREE, "ok"),
            ]);
        }

        impl types::AsnType for Foo {
            const TAG: Tag = Tag::SEQUENCE;
        }

        impl Decode for Foo {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                _: &Constraints,
            ) -> Result<Self, D::Error> {
                decoder.decode_sequence(tag, None::<fn() -> Self>, |sequence| {
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
    fn tagged_any_keeps_the_wrapped_encoding() {
        use crate::Decoder;

        // [1] EXPLICIT wrapping of OCTET STRING 'ABCD'H, in both length forms.
        let expected = Any::new(vec![0x04, 0x02, 0xAB, 0xCD]);
        for input in [
            &[0xA1, 0x04, 0x04, 0x02, 0xAB, 0xCD][..],
            &[0xA1, 0x80, 0x04, 0x02, 0xAB, 0xCD, 0x00, 0x00][..],
        ] {
            let mut decoder = super::Decoder::new(input, DecoderOptions::ber());
            assert_eq!(
                decoder.decode_any(Tag::new(Class::Context, 1)).unwrap(),
                expected
            );
            assert!(decoder.remaining().is_empty());
        }
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
    fn indefinite_sequence_of_ends_at_the_end_of_contents_octets() {
        let input = &[0x30, 0x80, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x00, 0x00];
        assert_eq!(decode::<Vec<i32>>(input).unwrap(), [1, 2]);
    }

    #[test]
    fn malformed_set_of_element_reports_its_own_error() {
        // The second element of a SET OF INTEGER is a BOOLEAN.
        let input = &[0x31, 0x06, 0x02, 0x01, 0x01, 0x01, 0x01, 0xFF];
        assert!(matches!(
            *decode::<SetOf<i32>>(input).unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::MismatchedTag { .. })
            }
        ));
    }

    #[test]
    fn malformed_optional_explicit_prefix_is_an_error() {
        use crate::Decoder as _;

        // A present [0] holds a BOOLEAN where an INTEGER is expected.
        let input = &[0xA0, 0x03, 0x01, 0x01, 0xFF];
        let mut decoder = Decoder::new(input, DecoderOptions::ber());
        assert!(
            decoder
                .decode_optional_with_explicit_prefix::<i32>(Tag::new(Class::Context, 0))
                .is_err()
        );
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

    #[test]
    fn octet_string_size_constraint() {
        // OCTET STRING (SIZE (3)) — tag 0x04, length, data
        let exact = &[0x04, 0x03, 0xAA, 0xBB, 0xCC];
        assert_eq!(
            *decode::<FixedOctetString<3>>(exact).unwrap(),
            [0xAA, 0xBB, 0xCC]
        );

        let too_short = &[0x04, 0x02, 0xAA, 0xBB];
        assert!(matches!(
            *decode::<FixedOctetString<3>>(too_short).unwrap_err().kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(2), .. }
        ));

        let too_long = &[0x04, 0x04, 0xAA, 0xBB, 0xCC, 0xDD];
        assert!(matches!(
            *decode::<FixedOctetString<3>>(too_long).unwrap_err().kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(4), .. }
        ));
    }

    #[test]
    fn bit_string_size_constraint() {
        use crate::Decoder as _;

        // SIZE(8) means exactly 8 bits.
        let constraints = &constraints!(size_constraint!(8));

        // 8-bit string: tag 0x03, len 2 (1 unused-bits byte + 1 data byte), unused=0, data=0xAA
        let exact = &[0x03, 0x02, 0x00, 0xAA];
        let mut dec = Decoder::new(exact, DecoderOptions::ber());
        assert!(dec.decode_bit_string(Tag::BIT_STRING, constraints).is_ok());

        // 0-bit string (too short): tag 0x03, len 1, unused=0
        let too_short = &[0x03, 0x01, 0x00];
        let mut dec = Decoder::new(too_short, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_bit_string(Tag::BIT_STRING, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(0), .. }
        ));

        // 16-bit string (too long): tag 0x03, len 3, unused=0, data=0xAA 0xBB
        let too_long = &[0x03, 0x03, 0x00, 0xAA, 0xBB];
        let mut dec = Decoder::new(too_long, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_bit_string(Tag::BIT_STRING, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(16), .. }
        ));
    }

    #[test]
    fn integer_value_constraint() {
        use crate::Decoder as _;

        // VALUE(0..100)
        let constraints = &constraints!(value_constraint!(0, 100));

        // 50 is in range: tag 0x02, len 1, value 0x32
        let in_range = &[0x02, 0x01, 0x32u8];
        let mut dec = Decoder::new(in_range, DecoderOptions::ber());
        assert!(
            dec.decode_integer::<Integer>(Tag::INTEGER, constraints)
                .is_ok()
        );

        // 200 is out of range (above 100): tag 0x02, len 2, value 0x00 0xC8
        let too_large = &[0x02, 0x02, 0x00, 0xC8u8];
        let mut dec = Decoder::new(too_large, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_integer::<Integer>(Tag::INTEGER, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::ValueConstraintNotSatisfied { .. }
        ));

        // -1 is out of range (below 0): tag 0x02, len 1, value 0xFF
        let negative = &[0x02, 0x01, 0xFFu8];
        let mut dec = Decoder::new(negative, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_integer::<Integer>(Tag::INTEGER, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::ValueConstraintNotSatisfied { .. }
        ));
    }

    #[test]
    fn sequence_of_size_constraint() {
        use crate::Decoder as _;

        // SIZE(1..3) — between 1 and 3 elements inclusive
        let constraints = &constraints!(size_constraint!(1, 3));

        // 2 elements: SEQUENCE tag 0x30, len 6, [INTEGER 1, INTEGER 2]
        let two = &[0x30u8, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02];
        let mut dec = Decoder::new(two, DecoderOptions::ber());
        let result = dec.decode_sequence_of::<i32>(Tag::SEQUENCE, constraints);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [1, 2]);

        // 0 elements (too few): SEQUENCE tag 0x30, len 0
        let zero = &[0x30u8, 0x00];
        let mut dec = Decoder::new(zero, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_sequence_of::<i32>(Tag::SEQUENCE, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(0), .. }
        ));

        // 4 elements (too many): SEQUENCE tag 0x30, len 12, [1, 2, 3, 4]
        let four = &[
            0x30u8, 0x0C, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01, 0x03, 0x02, 0x01, 0x04,
        ];
        let mut dec = Decoder::new(four, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_sequence_of::<i32>(Tag::SEQUENCE, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(4), .. }
        ));
    }

    #[test]
    fn set_of_size_constraint() {
        use crate::Decoder as _;

        // SIZE(1..3) — between 1 and 3 elements inclusive
        let constraints = &constraints!(size_constraint!(1, 3));

        // 2 elements: SET tag 0x31, len 6, [INTEGER 1, INTEGER 2]
        let two = &[0x31u8, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02];
        let mut dec = Decoder::new(two, DecoderOptions::ber());
        assert!(dec.decode_set_of::<i32>(Tag::SET, constraints).is_ok());

        // 0 elements (too few): SET tag 0x31, len 0
        let zero = &[0x31u8, 0x00];
        let mut dec = Decoder::new(zero, DecoderOptions::ber());
        assert!(matches!(
            *dec.decode_set_of::<i32>(Tag::SET, constraints)
                .unwrap_err()
                .kind,
            DecodeErrorKind::SizeConstraintNotSatisfied { size: Some(0), .. }
        ));
    }
}
