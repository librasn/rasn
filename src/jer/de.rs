//! Decoding JSON Encoding Rules data into Rust structures.

use serde_json::Value;

use crate::{
    de::Error,
    error::{DecodeError, JerDecodeErrorKind},
    types::{
        variants, Any, BitString, BmpString, Constraints, Constructed, Date, DecodeChoice,
        Enumerated, GeneralString, GeneralizedTime, GraphicString, Ia5String, NumericString,
        ObjectIdentifier, Oid, PrintableString, SequenceOf, SetOf, Tag, TeletexString, UtcTime,
        Utf8String, VisibleString,
    },
    Decode,
};

macro_rules! decode_jer_value {
    ($decoder_fn:expr, $input:expr) => {
        $input
            .pop()
            .ok_or_else(|| DecodeError::from(JerDecodeErrorKind::eoi()))
            .and_then($decoder_fn)
    };
}

/// Decodes JSON Encoding Rules data into Rust structures.
pub struct Decoder {
    stack: alloc::vec::Vec<Value>,
}

impl Decoder {
    /// Creates new default decoder from the given input.
    pub fn new(input: &str) -> Result<Self, <Decoder as crate::de::Decoder>::Error> {
        let root = serde_json::from_str(input).map_err(|e| {
            DecodeError::parser_fail(
                alloc::format!("Error parsing JER JSON {e:?}"),
                crate::Codec::Jer,
            )
        })?;
        Ok(Self {
            stack: alloc::vec![root],
        })
    }
}

impl From<Value> for Decoder {
    fn from(value: Value) -> Self {
        Self {
            stack: alloc::vec![value],
        }
    }
}

impl crate::Decoder for Decoder {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Self;

    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        decode_jer_value!(Self::any_from_value, self.stack)
    }

    fn decode_bit_string(
        &mut self,
        _t: Tag,
        constraints: Constraints,
    ) -> Result<BitString, Self::Error> {
        let (mut padded, bitstring_length) = if let Some(size) = constraints
            .size()
            .and_then(|s| s.constraint.is_fixed().then_some(s.constraint.as_start()))
            .flatten()
        {
            let value = BitString::try_from_vec(decode_jer_value!(
                Self::octet_string_from_value,
                self.stack
            )?)
            .map_err(|e| {
                DecodeError::custom(
                    alloc::format!("Failed to create BitString from bytes: {e:02x?}"),
                    self.codec(),
                )
            })?;
            (value, *size)
        } else {
            let last = self.stack.pop().ok_or_else(JerDecodeErrorKind::eoi)?;
            let value_map = last
                .as_object()
                .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                    needed: "object",
                    found: "unknown".into(),
                })?;
            let (value, length) = value_map
                .get("value")
                .and_then(|v| v.as_str())
                .zip(
                    value_map
                        .get("length")
                        .and_then(|l| l.as_number())
                        .and_then(|i| i.as_u64())
                        .map(|i| i as usize),
                )
                .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                    needed: "JSON object containing 'value' and 'length' properties.",
                    found: alloc::format!("{value_map:#?}"),
                })?;

            let value = bytes_from_hexstring(value).ok_or(DecodeError::custom(
                alloc::format!("Failed to create BitString from bytes: {value:02x?}"),
                self.codec(),
            ))?;
            let value = BitString::try_from_vec(value).map_err(|e| {
                DecodeError::custom(
                    alloc::format!("Failed to create BitString from bytes: {e:02x?}"),
                    self.codec(),
                )
            })?;

            (value, length)
        };
        let padding_length = if bitstring_length % 8 == 0 {
            0
        } else {
            8 - (bitstring_length % 8)
        };
        for _ in 0..padding_length {
            padded.pop();
        }

        if bitstring_length == padded.len() {
            Ok(padded)
        } else {
            Err(DecodeError::custom(
                alloc::format!("Failed to create BitString from bytes: invalid value length (was: {}, expected: {})", padded.len(), bitstring_length),
                self.codec(),
            ))
        }
    }

    fn decode_bool(&mut self, _t: Tag) -> Result<bool, Self::Error> {
        decode_jer_value!(Self::boolean_from_value, self.stack)
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _t: Tag) -> Result<E, Self::Error> {
        decode_jer_value!(Self::enumerated_from_value, self.stack)
    }

    fn decode_integer<I: crate::types::IntegerType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<I, Self::Error> {
        decode_jer_value!(Self::integer_from_value::<I>, self.stack)
    }

    fn decode_real<R: crate::types::RealType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<R, Self::Error> {
        decode_jer_value!(Self::real_from_value::<R>, self.stack)
    }

    fn decode_null(&mut self, _t: Tag) -> Result<(), Self::Error> {
        decode_jer_value!(Self::null_from_value, self.stack)
    }

    fn decode_object_identifier(&mut self, _t: Tag) -> Result<ObjectIdentifier, Self::Error> {
        decode_jer_value!(Self::object_identifier_from_value, self.stack)
    }

    fn decode_sequence<const RC: usize, const EC: usize, D, DF, F>(
        &mut self,
        _: Tag,
        _: Option<DF>,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: Constructed<RC, EC>,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        let mut last = self.stack.pop().ok_or_else(JerDecodeErrorKind::eoi)?;
        let value_map = last
            .as_object_mut()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "object",
                found: "unknown".into(),
            })?;
        let mut field_names = D::FIELDS
            .iter()
            .map(|f| f.name)
            .collect::<alloc::vec::Vec<&str>>();
        if let Some(extended_fields) = D::EXTENDED_FIELDS {
            field_names.extend(extended_fields.iter().map(|f| f.name));
        }
        field_names.reverse();
        for name in field_names {
            self.stack
                .push(value_map.remove(name).unwrap_or(Value::Null));
        }

        (decode_fn)(self)
    }

    fn decode_sequence_of<D: crate::Decode>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<SequenceOf<D>, Self::Error> {
        decode_jer_value!(|v| self.sequence_of_from_value(v), self.stack)
    }

    fn decode_set_of<D: crate::Decode + Eq + core::hash::Hash>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        decode_jer_value!(|v| self.set_of_from_value(v), self.stack)
    }

    fn decode_octet_string<'b, T: From<alloc::vec::Vec<u8>> + From<&'b [u8]>>(
        &'b mut self,
        _: Tag,
        _c: Constraints,
    ) -> Result<T, Self::Error> {
        decode_jer_value!(Self::octet_string_from_value, self.stack).map(T::from)
    }

    fn decode_utf8_string(&mut self, _t: Tag, _c: Constraints) -> Result<Utf8String, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)
    }

    fn decode_visible_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::VISIBLE_STRING,
                    alloc::format!("Error transforming VisibleString: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }

    fn decode_general_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GENERAL_STRING,
                    alloc::format!("Error transforming GeneralString: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }

    fn decode_graphic_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<GraphicString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::GRAPHIC_STRING,
                    alloc::format!("Error transforming GeneralString: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }

    fn decode_ia5_string(&mut self, _t: Tag, _c: Constraints) -> Result<Ia5String, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::IA5_STRING,
                    alloc::format!("Error transforming IA5String: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }

    fn decode_printable_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::PRINTABLE_STRING,
                    alloc::format!("Error transforming PrintableString: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }

    fn decode_numeric_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<NumericString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::NUMERIC_STRING,
                    alloc::format!("Error transforming NumericString: {e:?}"),
                    crate::Codec::Jer,
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
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                DecodeError::string_conversion_failed(
                    Tag::BMP_STRING,
                    alloc::format!("Error transforming BMPString: {e:?}"),
                    crate::Codec::Jer,
                )
            })
    }
    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        _: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_explicit_prefix<D: crate::Decode>(&mut self, _t: Tag) -> Result<D, Self::Error> {
        D::decode(self)
    }

    fn decode_utc_time(&mut self, _t: Tag) -> Result<UtcTime, Self::Error> {
        decode_jer_value!(Self::utc_time_from_value, self.stack)
    }

    fn decode_generalized_time(&mut self, _t: Tag) -> Result<GeneralizedTime, Self::Error> {
        decode_jer_value!(Self::general_time_from_value, self.stack)
    }

    fn decode_date(&mut self, _t: Tag) -> Result<Date, Self::Error> {
        decode_jer_value!(Self::date_from_value, self.stack)
    }

    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        _t: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: crate::Decode + Constructed<RC, EC>,
        FIELDS: crate::Decode,
        D: Fn(&mut Self::AnyDecoder<RC, EC>, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(alloc::vec::Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let mut last = self.stack.pop().ok_or_else(JerDecodeErrorKind::eoi)?;
        let value_map = last
            .as_object_mut()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "object",
                found: "unknown".into(),
            })?;
        let mut field_indices = SET::FIELDS
            .iter()
            .enumerate()
            .collect::<alloc::vec::Vec<_>>();
        let mut fields = alloc::vec![];
        field_indices
            .sort_by(|(_, a), (_, b)| a.tag_tree.smallest_tag().cmp(&b.tag_tree.smallest_tag()));
        for (index, field) in field_indices.into_iter() {
            self.stack
                .push(value_map.remove(field.name).unwrap_or(Value::Null));
            fields.push((decode_fn)(self, index, field.tag)?);
        }

        for (index, field) in SET::EXTENDED_FIELDS
            .iter()
            .flat_map(|fields| fields.iter())
            .enumerate()
        {
            self.stack
                .push(value_map.remove(field.name).unwrap_or(Value::Null));
            fields.push((decode_fn)(self, index + SET::FIELDS.len(), field.tag)?);
        }

        (field_fn)(fields)
    }

    fn decode_choice<D>(&mut self, _c: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        decode_jer_value!(|v| self.choice_from_value::<D>(v), self.stack)
    }

    fn decode_optional<D: crate::Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        let last = self.stack.pop().ok_or_else(JerDecodeErrorKind::eoi)?;
        match last {
            Value::Null => Ok(None),
            v => {
                self.stack.push(v);
                Some(D::decode(self)).transpose()
            }
        }
    }

    fn decode_optional_with_tag<D: crate::Decode>(
        &mut self,
        _: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_constraints<D: crate::Decode>(
        &mut self,
        _: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_tag_and_constraints<D: crate::Decode>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> core::result::Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_tag_and_constraints::<D>(tag, constraints)
    }

    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        _: Tag,
        _: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: crate::Decode,
    {
        self.decode_optional()
    }

    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: crate::Decode + Constructed<RC, EC>,
    >(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn codec(&self) -> crate::Codec {
        crate::Codec::Jer
    }
}

// -------------------------------------------------------------------
//
//                        HELPER METHODS
//
// -------------------------------------------------------------------

impl Decoder {
    fn any_from_value(value: Value) -> Result<Any, <Self as crate::de::Decoder>::Error> {
        Ok(Any::new(alloc::format!("{value}").as_bytes().to_vec()))
    }

    fn boolean_from_value(value: Value) -> Result<bool, DecodeError> {
        Ok(value
            .as_bool()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "boolean",
                found: alloc::format!("{value}"),
            })?)
    }

    fn enumerated_from_value<E: Enumerated>(value: Value) -> Result<E, DecodeError> {
        let identifier = value
            .as_str()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "enumerated item as string",
                found: alloc::format!("{value}"),
            })?;
        Ok(E::from_identifier(identifier).ok_or_else(|| {
            JerDecodeErrorKind::InvalidEnumDiscriminant {
                discriminant: alloc::string::String::from(identifier),
            }
        })?)
    }

    fn integer_from_value<I: crate::types::IntegerType>(value: Value) -> Result<I, DecodeError> {
        value
            .as_i64()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "number (supported range -2^63..2^63)",
                found: alloc::format!("{value}"),
            })?
            .try_into()
            .map_err(|_| DecodeError::integer_overflow(I::WIDTH, crate::Codec::Jer))
    }

    fn real_from_value<R: crate::types::RealType>(value: Value) -> Result<R, DecodeError> {
        if let Some(as_f64) = value.as_f64() {
            return R::try_from_float(as_f64).ok_or_else(|| {
                JerDecodeErrorKind::TypeMismatch {
                    needed: "number (double precision floating point)",
                    found: alloc::format!("{value}"),
                }
                .into()
            });
        }

        value
            .as_str()
            .and_then(|s| match s {
                "-0" => R::try_from_float(-0.0),
                "INF" => R::try_from_float(f64::INFINITY),
                "-INF" => R::try_from_float(f64::NEG_INFINITY),
                "NAN" => R::try_from_float(f64::NAN),
                _ => None,
            })
            .ok_or_else(|| {
                JerDecodeErrorKind::TypeMismatch {
                    needed: "number (double precision floating point)",
                    found: alloc::format!("{value}"),
                }
                .into()
            })
    }

    fn null_from_value(value: Value) -> Result<(), DecodeError> {
        Ok(value
            .is_null()
            .then_some(())
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "null",
                found: alloc::format!("{value}"),
            })?)
    }

    fn object_identifier_from_value(value: Value) -> Result<ObjectIdentifier, DecodeError> {
        // For performance reasons, sometimes it is better to use lazy one
        #[allow(clippy::unnecessary_lazy_evaluations)]
        Ok(value
            .as_str()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "number array",
                found: alloc::format!("{value}"),
            })?
            .split('.')
            .map(|arc| {
                arc.parse::<u32>()
                    .map_err(|_| JerDecodeErrorKind::TypeMismatch {
                        needed: "OID arc number",
                        found: arc.into(),
                    })
            })
            .collect::<Result<alloc::vec::Vec<u32>, _>>()
            .ok()
            .and_then(|arcs| Oid::new(&arcs).map(ObjectIdentifier::from))
            .ok_or_else(|| JerDecodeErrorKind::InvalidOIDString { value })?)
    }

    fn sequence_of_from_value<D: Decode>(
        &mut self,
        value: Value,
    ) -> Result<SequenceOf<D>, DecodeError> {
        value
            .as_array()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "array",
                found: alloc::format!("{value}"),
            })?
            .clone()
            .into_iter()
            .map(|v| {
                self.stack.push(v);
                D::decode(self)
            })
            .collect()
    }

    fn set_of_from_value<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        value: Value,
    ) -> Result<SetOf<D>, DecodeError> {
        value
            .as_array()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "array",
                found: alloc::format!("{value}"),
            })?
            .clone()
            .into_iter()
            .try_fold(SetOf::new(), |mut acc, v| {
                self.stack.push(v);
                acc.insert(D::decode(self)?);
                Ok(acc)
            })
    }

    fn string_from_value(value: Value) -> Result<alloc::string::String, DecodeError> {
        Ok(value
            .as_str()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "string",
                found: alloc::format!("{value}"),
            })
            .map(|n| n.into())?)
    }

    fn choice_from_value<D>(&mut self, value: Value) -> Result<D, DecodeError>
    where
        D: DecodeChoice,
    {
        let tag = value
            .as_object()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "object",
                found: alloc::format!("{value}"),
            })?
            .iter()
            .next()
            .and_then(|(k, v)| {
                D::IDENTIFIERS
                    .iter()
                    .enumerate()
                    .find(|id| id.1.eq_ignore_ascii_case(k))
                    .map(|(i, _)| (i, v))
            })
            .map_or(Tag::EOC, |(i, v)| {
                match variants::Variants::from_slice(
                    &[D::VARIANTS, D::EXTENDED_VARIANTS.unwrap_or(&[])].concat(),
                )
                .get(i)
                {
                    Some(t) => {
                        self.stack.push(v.clone());
                        *t
                    }
                    None => Tag::EOC,
                }
            });
        D::from_tag(self, tag)
    }

    fn octet_string_from_value(value: Value) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        let octet_string = value
            .as_str()
            .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                needed: "hex string",
                found: alloc::format!("{value}"),
            })?;
        bytes_from_hexstring(octet_string)
            .ok_or(JerDecodeErrorKind::InvalidJerOctetString {}.into())
    }

    fn utc_time_from_value(value: Value) -> Result<chrono::DateTime<chrono::Utc>, DecodeError> {
        crate::ber::de::Decoder::parse_any_utc_time_string(
            value
                .as_str()
                .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                    needed: "time string",
                    found: alloc::format!("{value}"),
                })?
                .into(),
        )
    }

    fn general_time_from_value(
        value: Value,
    ) -> Result<chrono::DateTime<chrono::FixedOffset>, DecodeError> {
        crate::ber::de::Decoder::parse_any_generalized_time_string(
            value
                .as_str()
                .ok_or_else(|| JerDecodeErrorKind::TypeMismatch {
                    needed: "time string",
                    found: alloc::format!("{value}"),
                })?
                .into(),
        )
    }

    fn date_from_value(value: Value) -> Result<chrono::NaiveDate, DecodeError> {
        crate::ber::de::Decoder::parse_date_string(value.as_str().ok_or_else(|| {
            JerDecodeErrorKind::TypeMismatch {
                needed: "date string",
                found: alloc::format!("{value}"),
            }
        })?)
    }
}

/// Parses a hex string into bytes.
fn bytes_from_hexstring(hex_string: &str) -> Option<alloc::vec::Vec<u8>> {
    if !hex_string.len().is_multiple_of(2) {
        return None;
    }
    let mut bytes = alloc::vec::Vec::<u8>::with_capacity(hex_string.len() / 2);
    for (i, c) in hex_string.char_indices() {
        let n = nibble_from_hexdigit(c)?;
        if i % 2 == 0 {
            bytes.push(n << 4);
        } else {
            bytes[i / 2] |= n;
        }
    }
    Some(bytes)
}

/// Parses a hexdigit character into a nibble (four bits).
fn nibble_from_hexdigit(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 0xA),
        'A'..='F' => Some(c as u8 - b'A' + 0xA),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_from_hexstring() {
        assert_eq!(bytes_from_hexstring(""), Some(vec![]));
        assert_eq!(bytes_from_hexstring("00"), Some(vec![0]));
        assert_eq!(bytes_from_hexstring("FF"), Some(vec![0xFF]));
        assert_eq!(bytes_from_hexstring("0000"), Some(vec![0, 0]));
        assert_eq!(bytes_from_hexstring("FFFF"), Some(vec![0xFF, 0xFF]));

        assert_eq!(bytes_from_hexstring(" "), None);
        assert_eq!(bytes_from_hexstring("!"), None);
        assert_eq!(bytes_from_hexstring("0"), None);
        assert_eq!(bytes_from_hexstring(" 0"), None);
        assert_eq!(bytes_from_hexstring("0 "), None);
        assert_eq!(bytes_from_hexstring("0!"), None);
        assert_eq!(bytes_from_hexstring("  "), None);
        assert_eq!(bytes_from_hexstring("00 "), None);
        assert_eq!(bytes_from_hexstring(" 00"), None);
        assert_eq!(bytes_from_hexstring("000"), None);
        assert_eq!(bytes_from_hexstring("Œ"), None);
        assert_eq!(bytes_from_hexstring("ŒŒ"), None);
        assert_eq!(bytes_from_hexstring("ŒŒŒ"), None);
        assert_eq!(bytes_from_hexstring("ABCDEFG"), None);
        assert_eq!(bytes_from_hexstring(" ABCDEF"), None);
        assert_eq!(bytes_from_hexstring("\u{0000}"), None);
        assert_eq!(bytes_from_hexstring("\u{FFFF}"), None);
        assert_eq!(bytes_from_hexstring("\u{0123}"), None);
        assert_eq!(bytes_from_hexstring("\u{30}"), None);
        assert_eq!(bytes_from_hexstring("\\u0030"), None);
        assert_eq!(bytes_from_hexstring("\\u202E\\u0030\\u0030"), None);
        assert_eq!(bytes_from_hexstring("⣐⡄"), None);
        assert_eq!(bytes_from_hexstring("😎"), None);
        assert_eq!(bytes_from_hexstring("🙈🙉🙊"), None);
    }

    #[test]
    fn test_nibble_from_hexdigit() {
        for c in '\u{0}'..'\u{1024}' {
            match c {
                '0' => assert_eq!(Some(0x00), nibble_from_hexdigit(c)),
                '1' => assert_eq!(Some(0x01), nibble_from_hexdigit(c)),
                '2' => assert_eq!(Some(0x02), nibble_from_hexdigit(c)),
                '3' => assert_eq!(Some(0x03), nibble_from_hexdigit(c)),
                '4' => assert_eq!(Some(0x04), nibble_from_hexdigit(c)),
                '5' => assert_eq!(Some(0x05), nibble_from_hexdigit(c)),
                '6' => assert_eq!(Some(0x06), nibble_from_hexdigit(c)),
                '7' => assert_eq!(Some(0x07), nibble_from_hexdigit(c)),
                '8' => assert_eq!(Some(0x08), nibble_from_hexdigit(c)),
                '9' => assert_eq!(Some(0x09), nibble_from_hexdigit(c)),
                'A' => assert_eq!(Some(0x0A), nibble_from_hexdigit(c)),
                'B' => assert_eq!(Some(0x0B), nibble_from_hexdigit(c)),
                'C' => assert_eq!(Some(0x0C), nibble_from_hexdigit(c)),
                'D' => assert_eq!(Some(0x0D), nibble_from_hexdigit(c)),
                'E' => assert_eq!(Some(0x0E), nibble_from_hexdigit(c)),
                'F' => assert_eq!(Some(0x0F), nibble_from_hexdigit(c)),
                'a' => assert_eq!(Some(0x0A), nibble_from_hexdigit(c)),
                'b' => assert_eq!(Some(0x0B), nibble_from_hexdigit(c)),
                'c' => assert_eq!(Some(0x0C), nibble_from_hexdigit(c)),
                'd' => assert_eq!(Some(0x0D), nibble_from_hexdigit(c)),
                'e' => assert_eq!(Some(0x0E), nibble_from_hexdigit(c)),
                'f' => assert_eq!(Some(0x0F), nibble_from_hexdigit(c)),
                _ => assert_eq!(None, nibble_from_hexdigit(c)),
            }
        }
    }
}
