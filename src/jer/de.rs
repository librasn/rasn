//! # Decoding JER

use jzon::JsonValue;

use crate::{
    de::Error,
    types::{fields::Fields, *},
    Decode,
};

macro_rules! decode_jer_value {
    ($decoder_fn:expr, $input:expr) => {
        $input
            .pop()
            .ok_or(error::Error::eoi())
            .and_then($decoder_fn)
    };
}

pub mod error;
pub struct Decoder {
    stack: alloc::vec::Vec<JsonValue>,
}

impl Decoder {
    pub fn new(input: &str) -> Result<Self, error::Error> {
        let root = jzon::parse(input)?;
        Ok(Self {
            stack: alloc::vec![root],
        })
    }
}

impl From<JsonValue> for Decoder {
    fn from(value: JsonValue) -> Self {
        Self {
            stack: alloc::vec![value],
        }
    }
}

impl crate::Decoder for Decoder {
    type Error = error::Error;

    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        decode_jer_value!(Self::any_from_value, self.stack)
    }

    fn decode_bit_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<BitString, Self::Error> {
        decode_jer_value!(Self::bit_string_from_value, self.stack)
    }

    fn decode_bool(&mut self, _t: crate::Tag) -> Result<bool, Self::Error> {
        decode_jer_value!(Self::boolean_from_value, self.stack)
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _t: crate::Tag) -> Result<E, Self::Error> {
        decode_jer_value!(Self::enumerated_from_value, self.stack)
    }

    fn decode_integer(&mut self, _t: crate::Tag, _c: Constraints) -> Result<Integer, Self::Error> {
        decode_jer_value!(Self::integer_from_value, self.stack)
    }

    fn decode_null(&mut self, _t: crate::Tag) -> Result<(), Self::Error> {
        decode_jer_value!(Self::null_from_value, self.stack)
    }

    fn decode_object_identifier(
        &mut self,
        _t: crate::Tag,
    ) -> Result<ObjectIdentifier, Self::Error> {
        decode_jer_value!(Self::object_identifier_from_value, self.stack)
    }

    fn decode_sequence<D, F>(&mut self, _: crate::Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        D: Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        let mut last = self.stack.pop().ok_or(error::Error::eoi())?;
        let value_map = last.as_object_mut().ok_or(error::Error::TypeMismatch {
            needed: "object",
            found: alloc::format!("unknown"),
        })?;
        let mut field_names = [D::FIELDS, D::EXTENDED_FIELDS.unwrap_or(Fields::empty())]
            .iter()
            .flat_map(|f| f.iter())
            .map(|f| f.name)
            .collect::<alloc::vec::Vec<&str>>();
        field_names.reverse();
        for name in field_names {
            self.stack
                .push(value_map.remove(name).unwrap_or(JsonValue::Null));
        }

        (decode_fn)(self)
    }

    fn decode_sequence_of<D: crate::Decode>(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<SequenceOf<D>, Self::Error> {
        decode_jer_value!(|v| self.sequence_of_from_value(v), self.stack)
    }

    fn decode_set_of<D: crate::Decode + Ord>(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        decode_jer_value!(|v| self.set_of_from_value(v), self.stack)
    }

    fn decode_octet_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        decode_jer_value!(Self::octet_string_from_value, self.stack)
    }

    fn decode_utf8_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<Utf8String, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)
    }

    fn decode_visible_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming VisibleString: {e:?}"))
            })
    }

    fn decode_general_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming GeneralString: {e:?}"))
            })
    }

    fn decode_ia5_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<Ia5String, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming IA5String: {e:?}"))
            })
    }

    fn decode_printable_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming PrintableString: {e:?}"))
            })
    }

    fn decode_numeric_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<NumericString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming NumericString: {e:?}"))
            })
    }

    fn decode_teletex_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<TeletexString, Self::Error> {
        todo!()
    }

    fn decode_bmp_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<BmpString, Self::Error> {
        decode_jer_value!(Self::string_from_value, self.stack)?
            .try_into()
            .map_err(|e| {
                error::Error::custom(&alloc::format!("Error transforming BMPString: {e:?}"))
            })
    }

    fn decode_explicit_prefix<D: crate::Decode>(
        &mut self,
        _t: crate::Tag,
    ) -> Result<D, Self::Error> {
        D::decode(self)
    }

    fn decode_utc_time(&mut self, _t: crate::Tag) -> Result<UtcTime, Self::Error> {
        decode_jer_value!(Self::utc_time_from_value, self.stack)
    }

    fn decode_generalized_time(&mut self, _t: crate::Tag) -> Result<GeneralizedTime, Self::Error> {
        decode_jer_value!(Self::general_time_from_value, self.stack)
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        _t: crate::Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: crate::Decode + Constructed,
        FIELDS: crate::Decode,
        D: Fn(&mut Self, usize, crate::Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(alloc::vec::Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        let mut last = self.stack.pop().ok_or(error::Error::eoi())?;
        let value_map = last.as_object_mut().ok_or(error::Error::TypeMismatch {
            needed: "object",
            found: alloc::format!("unknown"),
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
                .push(value_map.remove(field.name).unwrap_or(JsonValue::Null));
            fields.push((decode_fn)(self, index, field.tag)?);
        }

        for (index, field) in SET::EXTENDED_FIELDS
            .iter()
            .flat_map(|fields| fields.iter())
            .enumerate()
        {
            self.stack
                .push(value_map.remove(field.name).unwrap_or(JsonValue::Null));
            fields.push((decode_fn)(self, index + SET::FIELDS.len(), field.tag)?)
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
        let last = self.stack.pop().ok_or(error::Error::eoi())?;
        match last {
            JsonValue::Null => Ok(None),
            v => {
                self.stack.push(v);
                Some(D::decode(self)).transpose()
            }
        }
    }

    fn decode_optional_with_tag<D: crate::Decode>(
        &mut self,
        _: crate::Tag,
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
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_constraints<D>(
        &mut self,
        _: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: crate::Decode,
    {
        self.decode_optional()
    }

    fn decode_extension_addition_group<D: crate::Decode + Constructed>(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }
}

// -------------------------------------------------------------------
//
//                        HELPER METHODS
//
// -------------------------------------------------------------------

impl Decoder {
    fn any_from_value(value: JsonValue) -> Result<Any, error::Error> {
        Ok(Any::new(alloc::format!("{value}").as_bytes().to_vec()))
    }

    fn bit_string_from_value(value: JsonValue) -> Result<BitString, error::Error> {
        value
            .as_str()
            .ok_or(error::Error::TypeMismatch {
                needed: "bit string",
                found: alloc::format!("{value}"),
            })?
            .chars()
            .try_fold(BitString::new(), |mut acc, bit| {
                match bit {
                    '0' => acc.push(false),
                    '1' => acc.push(true),
                    c => {
                        return Err(error::Error::custom(&alloc::format!(
                            "Invalid character in bit string! Found {c}"
                        )))
                    }
                }
                Ok(acc)
            })
    }

    fn boolean_from_value(value: JsonValue) -> Result<bool, error::Error> {
        value.as_bool().ok_or(error::Error::TypeMismatch {
            needed: "boolean",
            found: alloc::format!("{value}"),
        })
    }

    fn enumerated_from_value<E: Enumerated>(value: JsonValue) -> Result<E, error::Error> {
        value
            .as_i64()
            .ok_or(error::Error::TypeMismatch {
                needed: "enumerable index",
                found: alloc::format!("{value}"),
            })?
            .try_into()
            .ok()
            .map(|i| E::from_discriminant(i))
            .flatten()
            .ok_or(error::Error::no_valid_variant(value))
    }

    fn integer_from_value(value: JsonValue) -> Result<Integer, error::Error> {
        value
            .as_i64()
            .ok_or(error::Error::TypeMismatch {
                needed: "number (supported range -2^63..2^63)",
                found: alloc::format!("{value}"),
            })
            .map(|n| n.into())
    }

    fn null_from_value(value: JsonValue) -> Result<(), error::Error> {
        value
            .is_null()
            .then(|| ())
            .ok_or(error::Error::TypeMismatch {
                needed: "null",
                found: alloc::format!("{value}"),
            })
    }

    fn object_identifier_from_value(value: JsonValue) -> Result<ObjectIdentifier, error::Error> {
        value
            .as_str()
            .ok_or(error::Error::TypeMismatch {
                needed: "number array",
                found: alloc::format!("{value}"),
            })?
            .split(".")
            .into_iter()
            .map(|arc| {
                u32::from_str_radix(arc, 10).map_err(|_| error::Error::TypeMismatch {
                    needed: "OID arc number",
                    found: alloc::format!("{arc}"),
                })
            })
            .collect::<Result<alloc::vec::Vec<u32>, _>>()
            .ok()
            .and_then(|arcs| Oid::new(&arcs).map(|oid| ObjectIdentifier::from(oid)))
            .ok_or(error::Error::custom(&alloc::format!(
                "Failed to construct OID from value {value}"
            )))
    }

    fn sequence_of_from_value<D: Decode>(
        &mut self,
        value: JsonValue,
    ) -> Result<SequenceOf<D>, error::Error> {
        value
            .as_array()
            .ok_or(error::Error::TypeMismatch {
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

    fn set_of_from_value<D: Decode + Ord>(
        &mut self,
        value: JsonValue,
    ) -> Result<SetOf<D>, error::Error> {
        value
            .as_array()
            .ok_or(error::Error::TypeMismatch {
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

    fn string_from_value(value: JsonValue) -> Result<alloc::string::String, error::Error> {
        value
            .as_str()
            .ok_or(error::Error::TypeMismatch {
                needed: "string",
                found: alloc::format!("{value}"),
            })
            .map(|n| n.into())
    }

    fn choice_from_value<D>(&mut self, value: JsonValue) -> Result<D, error::Error>
    where
        D: DecodeChoice,
    {
        let tag = value
            .as_object()
            .ok_or(error::Error::TypeMismatch {
                needed: "object",
                found: alloc::format!("{value}"),
            })?
            .iter()
            .next()
            .and_then(|(k, v)| {
                D::IDENTIFIERS
                    .iter()
                    .enumerate()
                    .find(|id| id.1.eq_ignore_ascii_case(&k))
                    .map(|(i, _)| (i, v))
            })
            .map_or(Tag::EOC, |(i, v)| {
                match variants::Variants::from_slice(&[D::VARIANTS, D::EXTENDED_VARIANTS].concat())
                    .get(i)
                {
                    Some(t) => {
                        self.stack.push(v.clone());
                        t.clone()
                    }
                    None => Tag::EOC,
                }
            });
        D::from_tag(self, tag)
    }

    fn octet_string_from_value(value: JsonValue) -> Result<alloc::vec::Vec<u8>, error::Error> {
        let octet_string = value.as_str().ok_or(error::Error::TypeMismatch {
            needed: "octet string",
            found: alloc::format!("{value}"),
        })?;
        (0..octet_string.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&octet_string[i..=i + 1], 16))
            .collect::<Result<alloc::vec::Vec<u8>, _>>()
            .map_err(|_| error::Error::custom("Failed to parse octet string."))
    }

    fn utc_time_from_value(
        value: JsonValue,
    ) -> Result<chrono::DateTime<chrono::Utc>, error::Error> {
        Ok(crate::ber::de::Decoder::parse_any_utc_time_string(
            value
                .as_str()
                .ok_or(error::Error::TypeMismatch {
                    needed: "time string",
                    found: alloc::format!("{value}"),
                })?
                .into(),
        )?)
    }

    fn general_time_from_value(
        value: JsonValue,
    ) -> Result<chrono::DateTime<chrono::FixedOffset>, error::Error> {
        Ok(crate::ber::de::Decoder::parse_any_generalized_time_string(
            value
                .as_str()
                .ok_or(error::Error::TypeMismatch {
                    needed: "time string",
                    found: alloc::format!("{value}"),
                })?
                .into(),
        )?)
    }
}
