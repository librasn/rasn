//! Decoding ASN.1 Value Notation (AVN) text into Rust structures.
//!
//! 1. **Parse** the entire input string into an [`AvnValue`] tree using nom combinators.
//! 2. **Type-directed decode** pops values from a stack and dispatches.

#![deny(clippy::all)]

use alloc::collections::BTreeMap;
use core::str::FromStr;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    sequence::{pair, preceded},
};
use num_bigint::BigInt;

use crate::{
    Decode,
    de::Error,
    error::{AvnDecodeErrorKind, DecodeError},
    types::{
        Any, BitString, BmpString, Constraints, Constructed, Date, DecodeChoice, Enumerated,
        GeneralString, GeneralizedTime, GraphicString, Ia5String, NumericString, ObjectIdentifier,
        Oid, PrintableString, SetOf, Tag, TeletexString, UtcTime, Utf8String, VisibleString,
        variants,
    },
};

use super::value::AvnValue;

// ---------------------------------------------------------------------------
// nom-based parser
// ---------------------------------------------------------------------------

fn parse_identifier_str(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_ascii_alphabetic()),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
    ))(input)
}

fn parse_hex_or_bin_string(input: &str) -> IResult<&str, AvnValue> {
    let (input, _) = char('\'')(input)?;
    let (input, raw) = take_while(|c: char| c != '\'')(input)?;
    let (input, _) = char('\'')(input)?;
    let content: alloc::string::String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    let (input, suffix) = alt((char('H'), char('B')))(input)?;
    if suffix == 'H' {
        parse_hex_bytes(&content)
            .map(|bytes| (input, AvnValue::OctetString(bytes)))
            .ok_or_else(|| {
                nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::HexDigit,
                ))
            })
    } else {
        parse_bin_bits(&content)
            .map(|(bytes, bit_length)| (input, AvnValue::BitString { bytes, bit_length }))
            .ok_or_else(|| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })
    }
}

fn parse_quoted_string(input: &str) -> IResult<&str, AvnValue> {
    let (input, _) = char('"')(input)?;
    let mut s = alloc::string::String::new();
    let mut rest = input;
    loop {
        match rest.chars().next() {
            None => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Eof,
                )));
            }
            Some('"') => {
                rest = &rest[1..];
                if rest.starts_with('"') {
                    s.push('"');
                    rest = &rest[1..];
                } else {
                    break;
                }
            }
            Some(c) => {
                s.push(c);
                rest = &rest[c.len_utf8()..];
            }
        }
    }
    Ok((rest, AvnValue::CharString(s)))
}

fn parse_number(input: &str) -> IResult<&str, AvnValue> {
    map(
        recognize(pair(
            opt(char('-')),
            pair(digit1, opt(preceded(char('.'), digit1))),
        )),
        |s: &str| {
            if s.contains('.') {
                AvnValue::Real(s.into())
            } else {
                AvnValue::Integer(s.into())
            }
        },
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, AvnValue> {
    parse_value_depth(input, 0)
}

fn parse_value_depth(input: &str, depth: usize) -> IResult<&str, AvnValue> {
    if depth > 64 {
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }
    let (input, _) = multispace0(input)?;
    alt((
        parse_hex_or_bin_string,
        parse_quoted_string,
        parse_number,
        move |i| parse_brace(i, depth + 1),
        move |i| parse_identifier_or_choice(i, depth),
    ))(input)
}

fn parse_identifier_or_choice(input: &str, depth: usize) -> IResult<&str, AvnValue> {
    let (after_id, id) = parse_identifier_str(input)?;
    let (after_ws, _) = multispace0(after_id)?;
    if let Some(rest) = after_ws.strip_prefix(':') {
        let (rest, inner) = parse_value_depth(rest, depth + 1)?;
        return Ok((
            rest,
            AvnValue::Choice {
                identifier: id.into(),
                value: alloc::boxed::Box::new(inner),
            },
        ));
    }
    let val = match id {
        "TRUE" => AvnValue::Boolean(true),
        "FALSE" => AvnValue::Boolean(false),
        "NULL" => AvnValue::Null,
        "PLUS-INFINITY" | "MINUS-INFINITY" | "NOT-A-NUMBER" => AvnValue::Real(id.into()),
        _ => AvnValue::Enumerated(id.into()),
    };
    Ok((after_id, val))
}

fn parse_brace(input: &str, depth: usize) -> IResult<&str, AvnValue> {
    let (input, _) = char('{')(input)?;
    let (input, _) = multispace0(input)?;

    if let Ok((rest, _)) = char::<_, nom::error::Error<&str>>('}')(input) {
        return Ok((rest, AvnValue::SequenceOf(alloc::vec![])));
    }

    enum BraceItem {
        Named(alloc::string::String, AvnValue),
        Bare(AvnValue),
    }

    let mut items: alloc::vec::Vec<BraceItem> = alloc::vec![];
    let mut current = input;

    loop {
        let (after_ws, _) = multispace0(current)?;
        current = after_ws;

        if let Ok((after_id, id)) = parse_identifier_str(current) {
            let (after_ws2, _) = multispace0(after_id)?;
            if let Some(after_colon) = after_ws2.strip_prefix(':') {
                // CHOICE element inside braces: id : value
                let (rest, val) = parse_value_depth(after_colon, depth)?;
                items.push(BraceItem::Bare(AvnValue::Choice {
                    identifier: id.into(),
                    value: alloc::boxed::Box::new(val),
                }));
                current = rest;
            } else if after_ws2.starts_with(',') || after_ws2.starts_with('}') {
                // Bare identifier: enumerated or real keyword
                let val = match id {
                    "PLUS-INFINITY" | "MINUS-INFINITY" | "NOT-A-NUMBER" => {
                        AvnValue::Real(id.into())
                    }
                    _ => AvnValue::Enumerated(id.into()),
                };
                items.push(BraceItem::Bare(val));
                current = after_id;
            } else {
                // Named SEQUENCE field: id value
                let (rest, val) = parse_value_depth(after_ws2, depth)?;
                items.push(BraceItem::Named(id.into(), val));
                current = rest;
            }
        } else {
            // Non-identifier bare value (number, hex/bin string, nested brace, …)
            let (rest, val) = parse_value_depth(current, depth)?;
            items.push(BraceItem::Bare(val));
            current = rest;
        }

        let (after_sep, _) = multispace0(current)?;
        current = after_sep;

        if current.starts_with('}') {
            current = &current[1..];
            break;
        } else if current.starts_with(',') {
            current = &current[1..];
        } else if current
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit() || c == '-')
            && items
                .iter()
                .all(|i| matches!(i, BraceItem::Bare(AvnValue::Integer(_))))
        {
            // OID-style space-separated arcs — no comma needed, continue
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                current,
                nom::error::ErrorKind::Char,
            )));
        }
    }

    let has_named = items.iter().any(|i| matches!(i, BraceItem::Named(..)));
    let has_bare = items.iter().any(|i| matches!(i, BraceItem::Bare(..)));

    if has_named && has_bare {
        return Err(nom::Err::Error(nom::error::Error::new(
            current,
            nom::error::ErrorKind::Verify,
        )));
    }

    if has_named {
        Ok((
            current,
            AvnValue::Sequence(
                items
                    .into_iter()
                    .map(|item| {
                        if let BraceItem::Named(name, val) = item {
                            (name, Some(val))
                        } else {
                            unreachable!()
                        }
                    })
                    .collect(),
            ),
        ))
    } else {
        Ok((
            current,
            AvnValue::SequenceOf(
                items
                    .into_iter()
                    .map(|item| {
                        if let BraceItem::Bare(val) = item {
                            val
                        } else {
                            unreachable!()
                        }
                    })
                    .collect(),
            ),
        ))
    }
}

// ---------------------------------------------------------------------------
// Low-level string helpers
// ---------------------------------------------------------------------------

fn parse_hex_bytes(hex: &str) -> Option<alloc::vec::Vec<u8>> {
    if hex.is_empty() {
        return Some(alloc::vec![]);
    }
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let b = hex.as_bytes();
    let mut out = alloc::vec::Vec::with_capacity(hex.len() / 2);
    for chunk in b.chunks(2) {
        let hi = nibble(chunk[0])?;
        let lo = nibble(chunk[1])?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn parse_bin_bits(bin: &str) -> Option<(alloc::vec::Vec<u8>, usize)> {
    if bin.is_empty() {
        return Some((alloc::vec![], 0));
    }
    let bit_length = bin.len();
    let mut bytes: alloc::vec::Vec<u8> = alloc::vec![];
    let mut current: u8 = 0;
    for (i, c) in bin.chars().enumerate() {
        let bit: u8 = match c {
            '0' => 0,
            '1' => 1,
            _ => return None,
        };
        current |= bit << (7 - (i % 8));
        if i % 8 == 7 {
            bytes.push(current);
            current = 0;
        }
    }
    if !bit_length.is_multiple_of(8) {
        bytes.push(current);
    }
    Some((bytes, bit_length))
}

// ---------------------------------------------------------------------------
// Decoder struct
// ---------------------------------------------------------------------------

macro_rules! decode_avn_value {
    ($decoder_fn:expr, $stack:expr) => {
        $stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))
            .and_then(|v| v.ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi())))
            .and_then($decoder_fn)
    };
}

/// Decodes ASN.1 Value Notation text into Rust structures.
pub struct Decoder {
    stack: alloc::vec::Vec<Option<AvnValue>>,
}

impl Decoder {
    /// Create a new decoder by parsing the entire input string.
    pub fn new(input: &str) -> Result<Self, DecodeError> {
        let (rest, root) = parse_value(input).map_err(|e| match e {
            nom::Err::Incomplete(_) => DecodeError::from(AvnDecodeErrorKind::AvnEndOfInput {}),
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                if e.code == nom::error::ErrorKind::TooLarge {
                    DecodeError::from_kind(
                        crate::error::DecodeErrorKind::ExceedsMaxParseDepth,
                        crate::Codec::Avn,
                    )
                } else {
                    DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                        found: e.input.chars().take(20).collect(),
                    })
                }
            }
        })?;
        if !rest.trim().is_empty() {
            return Err(DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                found: rest.into(),
            }));
        }
        Ok(Self {
            stack: alloc::vec![Some(root)],
        })
    }
}

impl From<AvnValue> for Decoder {
    fn from(value: AvnValue) -> Self {
        Self {
            stack: alloc::vec![Some(value)],
        }
    }
}

impl crate::Decoder for Decoder {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Self;

    fn codec(&self) -> crate::Codec {
        crate::Codec::Avn
    }

    fn decode_any(&mut self, _tag: Tag) -> Result<Any, Self::Error> {
        let value = self
            .stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;
        Ok(Any::new(alloc::format!("{value}").into_bytes()))
    }

    fn decode_bool(&mut self, _t: Tag) -> Result<bool, Self::Error> {
        decode_avn_value!(Self::boolean_from_value, self.stack)
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _t: Tag) -> Result<E, Self::Error> {
        decode_avn_value!(Self::enumerated_from_value::<E>, self.stack)
    }

    fn decode_integer<I: crate::types::IntegerType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<I, Self::Error> {
        decode_avn_value!(Self::integer_from_value::<I>, self.stack)
    }

    fn decode_real<R: crate::types::RealType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<R, Self::Error> {
        decode_avn_value!(Self::real_from_value::<R>, self.stack)
    }

    fn decode_null(&mut self, _t: Tag) -> Result<(), Self::Error> {
        decode_avn_value!(Self::null_from_value, self.stack)
    }

    fn decode_object_identifier(&mut self, _t: Tag) -> Result<ObjectIdentifier, Self::Error> {
        decode_avn_value!(Self::oid_from_value, self.stack)
    }

    fn decode_bit_string(&mut self, _t: Tag, _c: Constraints) -> Result<BitString, Self::Error> {
        decode_avn_value!(Self::bit_string_from_value, self.stack)
    }

    fn decode_octet_string<'buf, T>(
        &'buf mut self,
        _: Tag,
        _c: Constraints,
    ) -> Result<T, Self::Error>
    where
        T: From<alloc::vec::Vec<u8>> + From<&'buf [u8]>,
    {
        decode_avn_value!(Self::octet_string_from_value, self.stack).map(T::from)
    }

    fn decode_utf8_string(&mut self, _t: Tag, _c: Constraints) -> Result<Utf8String, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)
    }

    fn decode_visible_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::VISIBLE_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_general_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GENERAL_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_graphic_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<GraphicString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GRAPHIC_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_ia5_string(&mut self, _t: Tag, _c: Constraints) -> Result<Ia5String, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::IA5_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_printable_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::PRINTABLE_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_numeric_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<NumericString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::NUMERIC_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_teletex_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<TeletexString, Self::Error> {
        todo!()
    }

    fn decode_bmp_string(&mut self, _t: Tag, _c: Constraints) -> Result<BmpString, Self::Error> {
        decode_avn_value!(Self::char_string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::BMP_STRING,
                    alloc::format!("{e:?}"),
                    crate::Codec::Avn,
                )
            })
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _t: Tag) -> Result<D, Self::Error> {
        D::decode(self)
    }

    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        _: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_utc_time(&mut self, _t: Tag) -> Result<UtcTime, Self::Error> {
        decode_avn_value!(Self::utc_time_from_value, self.stack)
    }

    fn decode_generalized_time(&mut self, _t: Tag) -> Result<GeneralizedTime, Self::Error> {
        decode_avn_value!(Self::generalized_time_from_value, self.stack)
    }

    fn decode_date(&mut self, _t: Tag) -> Result<Date, Self::Error> {
        decode_avn_value!(Self::date_from_value, self.stack)
    }

    fn decode_sequence<const RC: usize, const EC: usize, D, DF, F>(
        &mut self,
        _: Tag,
        _: Option<DF>,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: Constructed<RC, EC>,
        DF: FnOnce() -> D,
        F: FnOnce(&mut Self::AnyDecoder<RC, EC>) -> Result<D, Self::Error>,
    {
        let last = self
            .stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;

        let mut field_map: BTreeMap<alloc::string::String, AvnValue> = match last {
            AvnValue::Sequence(fields) => fields
                .into_iter()
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect(),
            AvnValue::SequenceOf(items) if items.is_empty() => BTreeMap::new(),
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "sequence",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };

        // Push field values in reverse order so pop() yields them in declaration order.
        let mut field_names: alloc::vec::Vec<&str> = D::FIELDS.iter().map(|f| f.name).collect();
        if let Some(extended_fields) = D::EXTENDED_FIELDS {
            field_names.extend(extended_fields.iter().map(|f| f.name));
        }
        field_names.reverse();
        for name in field_names {
            self.stack.push(field_map.remove(name));
        }
        (decode_fn)(self)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<alloc::vec::Vec<D>, Self::Error> {
        decode_avn_value!(|v| self.sequence_of_from_value(v), self.stack)
    }

    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        decode_avn_value!(|v| self.set_of_from_value(v), self.stack)
    }

    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        _t: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + Constructed<RC, EC>,
        FIELDS: Decode,
        D: Fn(&mut Self::AnyDecoder<RC, EC>, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(alloc::vec::Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let last = self
            .stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;

        let mut field_map: BTreeMap<alloc::string::String, AvnValue> = match last {
            AvnValue::Sequence(fields) => fields
                .into_iter()
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect(),
            AvnValue::SequenceOf(items) if items.is_empty() => BTreeMap::new(),
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "set",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };

        let mut fields_out = alloc::vec![];

        let mut field_indices: alloc::vec::Vec<(usize, _)> =
            SET::FIELDS.iter().enumerate().collect();
        field_indices
            .sort_by(|(_, a), (_, b)| a.tag_tree.smallest_tag().cmp(&b.tag_tree.smallest_tag()));
        for (index, field) in field_indices {
            self.stack.push(field_map.remove(field.name));
            fields_out.push((decode_fn)(self, index, field.tag)?);
        }
        for (index, field) in SET::EXTENDED_FIELDS
            .iter()
            .flat_map(|fields| fields.iter())
            .enumerate()
        {
            self.stack.push(field_map.remove(field.name));
            fields_out.push((decode_fn)(self, index + SET::FIELDS.len(), field.tag)?);
        }
        (field_fn)(fields_out)
    }

    fn decode_choice<D>(&mut self, _c: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        decode_avn_value!(|v| self.choice_from_value::<D>(v), self.stack)
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        match self
            .stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?
        {
            None => Ok(None),
            Some(v) => {
                self.stack.push(Some(v));
                Some(D::decode(self)).transpose()
            }
        }
    }

    fn decode_optional_with_tag<D: Decode>(&mut self, _: Tag) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        _: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_extension_addition_with_tag_and_constraints::<D>(tag, constraints)
    }

    fn decode_extension_addition_with_tag_and_constraints<D: Decode>(
        &mut self,
        _: Tag,
        _: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: Decode + Constructed<RC, EC>,
    >(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }
}

// ---------------------------------------------------------------------------
// Helper methods on Decoder
// ---------------------------------------------------------------------------

impl Decoder {
    fn boolean_from_value(value: AvnValue) -> Result<bool, DecodeError> {
        match value {
            AvnValue::Boolean(b) => Ok(b),
            other => Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                needed: "boolean",
                found: alloc::format!("{other:?}"),
            })),
        }
    }

    fn enumerated_from_value<E: Enumerated>(value: AvnValue) -> Result<E, DecodeError> {
        let id = match value {
            AvnValue::Enumerated(id) => id,
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "enumerated identifier",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        E::from_identifier(&id).ok_or_else(|| {
            DecodeError::from(AvnDecodeErrorKind::AvnInvalidEnumDiscriminant { discriminant: id })
        })
    }

    fn integer_from_value<I: crate::types::IntegerType>(value: AvnValue) -> Result<I, DecodeError> {
        let s = match value {
            AvnValue::Integer(s) => s,
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "integer",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        let bigint = BigInt::from_str(&s).map_err(|e| {
            DecodeError::from(AvnDecodeErrorKind::IntegerParseError {
                msg: alloc::format!("{e}"),
            })
        })?;
        I::try_from(bigint).map_err(|_| DecodeError::integer_overflow(I::WIDTH, crate::Codec::Avn))
    }

    fn real_from_value<R: crate::types::RealType>(value: AvnValue) -> Result<R, DecodeError> {
        let s: alloc::string::String = match value {
            AvnValue::Real(s) => s,
            AvnValue::Integer(s) => s, // handles "-0" which lexes as Number("-0")
            AvnValue::Enumerated(s) => s, // handles PLUS-INFINITY etc. if missed as Identifier
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "real",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };

        let real_err = || {
            DecodeError::from_kind(
                crate::error::DecodeErrorKind::InvalidRealEncoding,
                crate::Codec::Avn,
            )
        };

        match s.as_str() {
            "PLUS-INFINITY" => {
                return R::try_from_float(f64::INFINITY).ok_or_else(real_err);
            }
            "MINUS-INFINITY" => {
                return R::try_from_float(f64::NEG_INFINITY).ok_or_else(real_err);
            }
            "NOT-A-NUMBER" => {
                return R::try_from_float(f64::NAN).ok_or_else(real_err);
            }
            "-0" => {
                return R::try_from_float(-0.0_f64).ok_or_else(real_err);
            }
            _ => {}
        }

        let f: f64 = s.parse().map_err(|_| real_err())?;
        R::try_from_float(f).ok_or_else(real_err)
    }

    fn null_from_value(value: AvnValue) -> Result<(), DecodeError> {
        match value {
            AvnValue::Null => Ok(()),
            other => Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                needed: "null",
                found: alloc::format!("{other:?}"),
            })),
        }
    }

    fn oid_from_value(value: AvnValue) -> Result<ObjectIdentifier, DecodeError> {
        // OID is encoded as { arc1 arc2 ... } which parses as SequenceOf([Integer, ...])
        let arcs: alloc::vec::Vec<u32> = match value {
            AvnValue::Oid(arcs) => arcs,
            AvnValue::SequenceOf(items) => items
                .into_iter()
                .map(|item| match item {
                    AvnValue::Integer(s) => s.parse::<u32>().map_err(|_| {
                        DecodeError::from(AvnDecodeErrorKind::InvalidOid { value: s })
                    }),
                    other => Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                        needed: "OID arc (integer)",
                        found: alloc::format!("{other:?}"),
                    })),
                })
                .collect::<Result<_, _>>()?,
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "OID as { arc... }",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        Oid::new(&arcs).map(ObjectIdentifier::from).ok_or_else(|| {
            DecodeError::from(AvnDecodeErrorKind::InvalidOid {
                value: alloc::format!("{arcs:?}"),
            })
        })
    }

    fn bit_string_from_value(value: AvnValue) -> Result<BitString, DecodeError> {
        let (bytes, bit_length) = match value {
            AvnValue::BitString { bytes, bit_length } => (bytes, bit_length),
            // Hex notation '...'H also used for byte-aligned bit strings
            AvnValue::OctetString(bytes) => {
                let bl = bytes.len() * 8;
                (bytes, bl)
            }
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "bit string ('...'H or '...'B)",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        // Build BitVec from raw bytes and trim to exact bit_length
        let mut bv = BitString::try_from_vec(bytes).map_err(|e| {
            DecodeError::custom(
                alloc::format!("Failed to create BitString: {e:02x?}"),
                crate::Codec::Avn,
            )
        })?;
        while bv.len() > bit_length {
            bv.pop();
        }
        Ok(bv)
    }

    fn octet_string_from_value(value: AvnValue) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        match value {
            AvnValue::OctetString(bytes) => Ok(bytes),
            other => Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                needed: "octet string ('...'H)",
                found: alloc::format!("{other:?}"),
            })),
        }
    }

    fn char_string_from_value(value: AvnValue) -> Result<alloc::string::String, DecodeError> {
        match value {
            AvnValue::CharString(s) => Ok(s),
            AvnValue::OctetString(bytes) => alloc::string::String::from_utf8(bytes).map_err(|e| {
                DecodeError::custom(
                    alloc::format!("Invalid UTF-8 in AVN string: {e}"),
                    crate::Codec::Avn,
                )
            }),
            other => Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                needed: "character string",
                found: alloc::format!("{other:?}"),
            })),
        }
    }

    fn sequence_of_from_value<D: Decode>(
        &mut self,
        value: AvnValue,
    ) -> Result<alloc::vec::Vec<D>, DecodeError> {
        let items = match value {
            AvnValue::SequenceOf(items) => items,
            AvnValue::Sequence(fields) if fields.is_empty() => alloc::vec![],
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "sequence of",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        items
            .into_iter()
            .map(|v| {
                self.stack.push(Some(v));
                D::decode(self)
            })
            .collect()
    }

    fn set_of_from_value<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        value: AvnValue,
    ) -> Result<SetOf<D>, DecodeError> {
        let items = match value {
            AvnValue::SequenceOf(items) => items,
            AvnValue::Sequence(fields) if fields.is_empty() => alloc::vec![],
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "set of",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };
        items.into_iter().try_fold(SetOf::new(), |mut acc, v| {
            self.stack.push(Some(v));
            acc.insert(D::decode(self)?);
            Ok(acc)
        })
    }

    fn choice_from_value<D: DecodeChoice>(&mut self, value: AvnValue) -> Result<D, DecodeError> {
        let (identifier, inner_value) = match value {
            AvnValue::Choice { identifier, value } => (identifier, value),
            other => {
                return Err(DecodeError::from(AvnDecodeErrorKind::AvnTypeMismatch {
                    needed: "choice (identifier : value)",
                    found: alloc::format!("{other:?}"),
                }));
            }
        };

        let all_variants = variants::Variants::from_slice(
            &[D::VARIANTS, D::EXTENDED_VARIANTS.unwrap_or(&[])].concat(),
        );
        let tag = all_variants
            .iter()
            .enumerate()
            .find_map(|(i, t)| {
                D::IDENTIFIERS
                    .get(i)
                    .filter(|&&id| id.eq_ignore_ascii_case(&identifier))
                    .map(|_| *t)
            })
            .unwrap_or(Tag::EOC);

        if tag != Tag::EOC {
            self.stack.push(Some(*inner_value));
        }
        D::from_tag(self, tag)
    }

    fn utc_time_from_value(value: AvnValue) -> Result<UtcTime, DecodeError> {
        crate::ber::de::Decoder::parse_any_utc_time_string(Self::char_string_from_value(value)?)
    }

    fn generalized_time_from_value(value: AvnValue) -> Result<GeneralizedTime, DecodeError> {
        crate::ber::de::Decoder::parse_any_generalized_time_string(Self::char_string_from_value(
            value,
        )?)
    }

    fn date_from_value(value: AvnValue) -> Result<Date, DecodeError> {
        crate::ber::de::Decoder::parse_date_string(&Self::char_string_from_value(value)?)
    }
}
