//! # Decoding JER

use core::iter::Peekable;

use crate::{
    de::Error,
    types::{fields::Fields, *},
};
use serde_json::{de::StrRead, Deserializer, StreamDeserializer, Value};

macro_rules! decode_jer_value {
    ($decoder_fn:expr, $input:expr) => {
        $input
            .next()
            .ok_or(error::Error::eoi())?
            .map_err(|e| error::Error::Parser {
                msg: alloc::format!("{e}"),
            })
            .and_then($decoder_fn)
    };
}

pub mod error;
pub struct Decoder<'i> {
    input: Peekable<StreamDeserializer<'i, StrRead<'i>, Value>>,
}

impl<'input> Decoder<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input: Deserializer::from_str(input).into_iter::<Value>().peekable(),
        }
    }
}

impl<'i> crate::Decoder for Decoder<'i> {
    type Error = error::Error;

    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        decode_jer_value!(Self::any_from_value, self.input)
    }

    fn decode_bit_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<BitString, Self::Error> {
        decode_jer_value!(Self::bit_string_from_value, self.input)
    }

    fn decode_bool(&mut self, _t: crate::Tag) -> Result<bool, Self::Error> {
        decode_jer_value!(Self::boolean_from_value, self.input)
    }

    fn decode_enumerated<E: Enumerated>(&mut self, _t: crate::Tag) -> Result<E, Self::Error> {
        decode_jer_value!(Self::enumerated_from_value, self.input)
    }

    fn decode_integer(&mut self, _t: crate::Tag, _c: Constraints) -> Result<Integer, Self::Error> {
        decode_jer_value!(Self::integer_from_value, self.input)
    }

    fn decode_null(&mut self, _t: crate::Tag) -> Result<(), Self::Error> {
        decode_jer_value!(Self::null_from_value, self.input)
    }

    fn decode_object_identifier(
        &mut self,
        _t: crate::Tag,
    ) -> Result<ObjectIdentifier, Self::Error> {
        decode_jer_value!(Self::object_identifier_from_value, self.input)
    }

    fn decode_sequence<D, F>(&mut self, _: crate::Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        D: Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        decode_jer_value!(|v| Self::sequence_from_value(self, v, decode_fn), self.input)
    }

    fn decode_sequence_of<D: crate::Decode>(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<SequenceOf<D>, Self::Error> {
        todo!()
    }

    fn decode_set_of<D: crate::Decode + Ord>(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        todo!()
    }

    fn decode_octet_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        todo!()
    }

    fn decode_utf8_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<Utf8String, Self::Error> {
        todo!()
    }

    fn decode_visible_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        todo!()
    }

    fn decode_general_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        todo!()
    }

    fn decode_ia5_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<Ia5String, Self::Error> {
        todo!()
    }

    fn decode_printable_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        todo!()
    }

    fn decode_numeric_string(
        &mut self,
        _t: crate::Tag,
        _c: Constraints,
    ) -> Result<NumericString, Self::Error> {
        todo!()
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
        todo!()
    }

    fn decode_explicit_prefix<D: crate::Decode>(
        &mut self,
        _t: crate::Tag,
    ) -> Result<D, Self::Error> {
        todo!()
    }

    fn decode_utc_time(&mut self, _t: crate::Tag) -> Result<UtcTime, Self::Error> {
        todo!()
    }

    fn decode_generalized_time(&mut self, _t: crate::Tag) -> Result<GeneralizedTime, Self::Error> {
        todo!()
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
        todo!()
    }

    fn decode_choice<D>(&mut self, _c: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        todo!()
    }

    fn decode_optional<D: crate::Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        match self
            .input
            .peek()
            .ok_or(error::Error::eoi())?
            .as_ref()
            .map_err(|e| error::Error::Parser {
                msg: alloc::format!("{e}"),
            })? {
            Value::Null => {
                self.input.next();
                Ok(None)
            },
            _ => {
                D::decode(self).map(|d| Some(d))
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

/// -------------------------------------------------------------------
///
///                        HELPER METHODS
///
/// -------------------------------------------------------------------

impl<'i> Decoder<'i> {
    fn any_from_value(value: Value) -> Result<Any, error::Error> {
        Ok(Any::new(alloc::format!("{value}").as_bytes().to_vec()))
    }

    fn bit_string_from_value(value: Value) -> Result<BitString, error::Error> {
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

    fn boolean_from_value(value: Value) -> Result<bool, error::Error> {
        value.as_bool().ok_or(error::Error::TypeMismatch {
            needed: "boolean",
            found: alloc::format!("{value}"),
        })
    }

    fn enumerated_from_value<E: Enumerated>(value: Value) -> Result<E, error::Error> {
        value
            .as_u64()
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

    fn integer_from_value(value: Value) -> Result<Integer, error::Error> {
        value
            .as_i64()
            .ok_or(error::Error::TypeMismatch {
                needed: "number (supported range -2^63..2^63)",
                found: alloc::format!("{value}"),
            })
            .map(|n| n.into())
    }

    fn null_from_value(value: Value) -> Result<(), error::Error> {
        value.as_null().ok_or(error::Error::TypeMismatch {
            needed: "null",
            found: alloc::format!("{value}"),
        })
    }

    fn object_identifier_from_value(value: Value) -> Result<ObjectIdentifier, error::Error> {
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

    fn sequence_from_value<'a, D, F>(&mut self, value: Value, decode_fn: F) -> Result<D, error::Error>
    where
        D: Constructed,
        F: FnOnce(&mut Self) -> Result<D, error::Error>,
    {
        self.input.
        // let object = value.as_object().ok_or(error::Error::TypeMismatch {
        //     needed: "object",
        //     found: alloc::format!("{value}"),
        // })?;
        // let sequence_values = [D::FIELDS, D::EXTENDED_FIELDS.unwrap_or(Fields::empty())]
        //     .iter()
        //     .flat_map(|f| f.iter())
        //     .fold(alloc::string::String::new(), |mut acc, field| {
        //         let val = object
        //             .get(field.name)
        //             .map_or(alloc::string::String::from("null "), |v| {
        //                 alloc::format!("{v} ")
        //             });
        //         acc.push_str(&val);
        //         acc
        //     });
        // let mut sequence_decoder = Decoder::<'a>::new(&sequence_values);
        // (decode_fn)(&mut sequence_decoder)
    }
}
