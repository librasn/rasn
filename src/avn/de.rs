//! Decoding ASN.1 Value Notation (AVN) text into Rust structures.
//!
//! Uses a two-phase approach:
//! 1. **Lex + parse** the entire input string into an [`AvnValue`] tree.
//! 2. **Type-directed decode** pops values from a stack and dispatches.

use alloc::collections::BTreeMap;
use core::str::FromStr;
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
// Lexer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Token {
    LBrace,
    RBrace,
    Colon,
    Comma,
    Identifier(alloc::string::String),
    Number(alloc::string::String),
    /// Content of `'...'H` (between the quotes, before the `H`)
    HexString(alloc::string::String),
    /// Content of `'...'B` (between the quotes, before the `B`)
    BinString(alloc::string::String),
    /// Already-unescaped content of `"..."` string
    QuotedString(alloc::string::String),
    True,
    False,
    Null,
}

fn lex(input: &str) -> Result<alloc::vec::Vec<Token>, DecodeError> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut tokens = alloc::vec::Vec::new();

    while i < len {
        match bytes[i] {
            // skip whitespace
            b' ' | b'\t' | b'\n' | b'\r' => i += 1,

            b'{' => {
                tokens.push(Token::LBrace);
                i += 1;
            }
            b'}' => {
                tokens.push(Token::RBrace);
                i += 1;
            }
            b':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            b',' => {
                tokens.push(Token::Comma);
                i += 1;
            }

            // Quoted string "..." with "" → " escaping
            b'"' => {
                i += 1; // skip opening "
                let mut s = alloc::string::String::new();
                loop {
                    if i >= len {
                        return Err(DecodeError::from(AvnDecodeErrorKind::UnterminatedString));
                    }
                    if bytes[i] == b'"' {
                        i += 1;
                        if i < len && bytes[i] == b'"' {
                            s.push('"');
                            i += 1;
                        } else {
                            break; // end of string
                        }
                    } else {
                        // For simplicity, treat each byte as a char (ASCII-safe for AVN identifiers)
                        s.push(bytes[i] as char);
                        i += 1;
                    }
                }
                tokens.push(Token::QuotedString(s));
            }

            // Hex or binary string: '...'H or '...'B
            b'\'' => {
                i += 1; // skip opening '
                let start = i;
                while i < len && bytes[i] != b'\'' {
                    i += 1;
                }
                if i >= len {
                    return Err(DecodeError::from(AvnDecodeErrorKind::UnterminatedString));
                }
                // Collect content (strip internal whitespace allowed in AVN hex strings)
                let raw = &input[start..i];
                let content: alloc::string::String =
                    raw.chars().filter(|c| !c.is_whitespace()).collect();
                i += 1; // skip closing '
                // Check for H or B suffix
                if i < len && bytes[i] == b'H' {
                    tokens.push(Token::HexString(content));
                    i += 1;
                } else if i < len && bytes[i] == b'B' {
                    tokens.push(Token::BinString(content));
                    i += 1;
                } else {
                    return Err(DecodeError::from(AvnDecodeErrorKind::InvalidHexString));
                }
            }

            // Number (possibly negative)
            b'-' | b'0'..=b'9' => {
                let start = i;
                if bytes[i] == b'-' {
                    i += 1;
                }
                while i < len && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i < len && bytes[i] == b'.' {
                    i += 1;
                    while i < len && bytes[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                let s =
                    alloc::string::String::from_utf8(bytes[start..i].to_vec()).unwrap_or_default();
                tokens.push(Token::Number(s));
            }

            // Identifier or keyword: [a-zA-Z][a-zA-Z0-9-_]*
            b if b.is_ascii_alphabetic() => {
                let start = i;
                while i < len
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-' || bytes[i] == b'_')
                {
                    i += 1;
                }
                let s =
                    alloc::string::String::from_utf8(bytes[start..i].to_vec()).unwrap_or_default();
                match s.as_str() {
                    "TRUE" => tokens.push(Token::True),
                    "FALSE" => tokens.push(Token::False),
                    "NULL" => tokens.push(Token::Null),
                    _ => tokens.push(Token::Identifier(s)),
                }
            }

            b => {
                return Err(DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                    found: alloc::format!("byte 0x{b:02X} at position {i}"),
                }));
            }
        }
    }
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser {
    tokens: alloc::vec::Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: alloc::vec::Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn parse_value(&mut self, depth: usize) -> Result<AvnValue, DecodeError> {
        if depth > 64 {
            return Err(DecodeError::from_kind(
                crate::error::DecodeErrorKind::ExceedsMaxParseDepth,
                crate::Codec::Avn,
            ));
        }
        match self.peek() {
            None => Err(DecodeError::from(AvnDecodeErrorKind::AvnEndOfInput {})),

            Some(Token::True) => {
                self.consume();
                Ok(AvnValue::Boolean(true))
            }
            Some(Token::False) => {
                self.consume();
                Ok(AvnValue::Boolean(false))
            }
            Some(Token::Null) => {
                self.consume();
                Ok(AvnValue::Null)
            }

            Some(Token::Number(_)) => {
                if let Some(Token::Number(s)) = self.consume() {
                    if s.contains('.') {
                        Ok(AvnValue::Real(s))
                    } else {
                        Ok(AvnValue::Integer(s))
                    }
                } else {
                    unreachable!()
                }
            }

            Some(Token::HexString(_)) => {
                if let Some(Token::HexString(hex)) = self.consume() {
                    parse_hex_bytes(&hex)
                        .map(AvnValue::OctetString)
                        .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::InvalidHexString))
                } else {
                    unreachable!()
                }
            }

            Some(Token::BinString(_)) => {
                if let Some(Token::BinString(bin)) = self.consume() {
                    parse_bin_bits(&bin)
                        .map(|(bytes, bit_length)| AvnValue::BitString { bytes, bit_length })
                        .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::InvalidBinString))
                } else {
                    unreachable!()
                }
            }

            Some(Token::QuotedString(_)) => {
                if let Some(Token::QuotedString(s)) = self.consume() {
                    Ok(AvnValue::CharString(s))
                } else {
                    unreachable!()
                }
            }

            Some(Token::LBrace) => {
                self.consume(); // consume '{'
                self.parse_brace_contents(depth + 1)
            }

            Some(Token::Identifier(_)) => {
                if let Some(Token::Identifier(id)) = self.consume() {
                    // Special real keywords encoded as identifiers by the encoder
                    match id.as_str() {
                        "PLUS-INFINITY" | "MINUS-INFINITY" | "NOT-A-NUMBER" => {
                            return Ok(AvnValue::Real(id));
                        }
                        _ => {}
                    }
                    // Check for CHOICE: identifier : value
                    if matches!(self.peek(), Some(Token::Colon)) {
                        self.consume(); // consume ':'
                        let inner = self.parse_value(depth + 1)?;
                        Ok(AvnValue::Choice {
                            identifier: id,
                            value: alloc::boxed::Box::new(inner),
                        })
                    } else {
                        Ok(AvnValue::Enumerated(id))
                    }
                } else {
                    unreachable!()
                }
            }

            Some(t) => Err(DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                found: alloc::format!("{t:?}"),
            })),
        }
    }

    /// Parse contents after `{` has been consumed.
    /// Returns `AvnValue::Sequence`, `AvnValue::SequenceOf`, or empty `SequenceOf([])`.
    fn parse_brace_contents(&mut self, depth: usize) -> Result<AvnValue, DecodeError> {
        // Empty braces
        if matches!(self.peek(), Some(Token::RBrace)) {
            self.consume();
            return Ok(AvnValue::SequenceOf(alloc::vec![]));
        }

        enum BraceItem {
            Named(alloc::string::String, AvnValue),
            Bare(AvnValue),
        }

        let mut items: alloc::vec::Vec<BraceItem> = alloc::vec![];

        loop {
            match self.peek() {
                None => return Err(DecodeError::from(AvnDecodeErrorKind::AvnEndOfInput {})),
                Some(Token::RBrace) => {
                    self.consume();
                    break;
                }
                Some(Token::Identifier(_)) => {
                    let id = if let Some(Token::Identifier(s)) = self.consume() {
                        s
                    } else {
                        unreachable!()
                    };

                    match self.peek() {
                        // id : value → CHOICE element (bare)
                        Some(Token::Colon) => {
                            self.consume();
                            let val = self.parse_value(depth)?;
                            items.push(BraceItem::Bare(AvnValue::Choice {
                                identifier: id,
                                value: alloc::boxed::Box::new(val),
                            }));
                        }
                        // id alone (comma or closing brace) → bare enumerated or real keyword
                        Some(Token::Comma) | Some(Token::RBrace) => match id.as_str() {
                            "PLUS-INFINITY" | "MINUS-INFINITY" | "NOT-A-NUMBER" => {
                                items.push(BraceItem::Bare(AvnValue::Real(id)));
                            }
                            _ => items.push(BraceItem::Bare(AvnValue::Enumerated(id))),
                        },
                        // id value → named SEQUENCE field
                        _ => {
                            let val = self.parse_value(depth)?;
                            items.push(BraceItem::Named(id, val));
                        }
                    }
                }
                _ => {
                    // Non-identifier: bare SEQUENCE OF element
                    let val = self.parse_value(depth)?;
                    items.push(BraceItem::Bare(val));
                }
            }

            // Consume separator: comma, or allow space-separated integers (OID notation).
            match self.peek() {
                Some(Token::Comma) => {
                    self.consume();
                }
                Some(Token::RBrace) => {}
                None => return Err(DecodeError::from(AvnDecodeErrorKind::AvnEndOfInput {})),
                // OID-style space-separated arcs: all items so far must be bare integers
                Some(Token::Number(_))
                    if items
                        .iter()
                        .all(|i| matches!(i, BraceItem::Bare(AvnValue::Integer(_)))) =>
                {
                    // No comma needed — continue to parse next arc in next loop iteration
                }
                Some(t) => {
                    return Err(DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                        found: alloc::format!("{t:?}"),
                    }));
                }
            }
        }

        let has_named = items.iter().any(|i| matches!(i, BraceItem::Named(..)));
        let has_bare = items.iter().any(|i| matches!(i, BraceItem::Bare(..)));

        if has_named && has_bare {
            return Err(DecodeError::from(AvnDecodeErrorKind::UnexpectedToken {
                found: "mixed named and bare items inside braces".into(),
            }));
        }

        if has_named {
            Ok(AvnValue::Sequence(
                items
                    .into_iter()
                    .map(|item| {
                        if let BraceItem::Named(name, val) = item {
                            (name, val)
                        } else {
                            unreachable!()
                        }
                    })
                    .collect(),
            ))
        } else {
            Ok(AvnValue::SequenceOf(
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
            ))
        }
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
            .and_then($decoder_fn)
    };
}

/// Decodes ASN.1 Value Notation text into Rust structures.
pub struct Decoder {
    stack: alloc::vec::Vec<AvnValue>,
}

impl Decoder {
    /// Create a new decoder by parsing the entire input string.
    pub fn new(input: &str) -> Result<Self, DecodeError> {
        let tokens = lex(input)?;
        let mut parser = Parser::new(tokens);
        let root = parser.parse_value(0)?;
        Ok(Self {
            stack: alloc::vec![root],
        })
    }
}

impl From<AvnValue> for Decoder {
    fn from(value: AvnValue) -> Self {
        Self {
            stack: alloc::vec![value],
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
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;

        let mut field_map: BTreeMap<alloc::string::String, AvnValue> = match last {
            AvnValue::Sequence(fields) => fields.into_iter().collect(),
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
            self.stack
                .push(field_map.remove(name).unwrap_or(AvnValue::Absent));
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
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;

        let mut field_map: BTreeMap<alloc::string::String, AvnValue> = match last {
            AvnValue::Sequence(fields) => fields.into_iter().collect(),
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
            self.stack
                .push(field_map.remove(field.name).unwrap_or(AvnValue::Absent));
            fields_out.push((decode_fn)(self, index, field.tag)?);
        }
        for (index, field) in SET::EXTENDED_FIELDS
            .iter()
            .flat_map(|fields| fields.iter())
            .enumerate()
        {
            self.stack
                .push(field_map.remove(field.name).unwrap_or(AvnValue::Absent));
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
        let last = self
            .stack
            .pop()
            .ok_or_else(|| DecodeError::from(AvnDecodeErrorKind::eoi()))?;
        match last {
            AvnValue::Absent => Ok(None),
            v => {
                self.stack.push(v);
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
                self.stack.push(v);
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
            self.stack.push(v);
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
            self.stack.push(*inner_value);
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
