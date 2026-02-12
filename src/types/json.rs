//! Implementation of ASN.1 traits for `serde_json::Value`.
//!
//! This module provides `AsnType`, `Encode`, `Decode`, `Choice`, and `DecodeChoice`
//! implementations for `serde_json::Value`, treating it as an ASN.1 CHOICE type.
//!
//! The mapping is:
//! - `Value::Null` -> NULL (tag 5)
//! - `Value::Bool` -> BOOLEAN (tag 1)
//! - `Value::Number` -> INTEGER (tag 2) for integers, or UTF8String for decimals
//! - `Value::String` -> UTF8String (tag 12)
//! - `Value::Array` -> SEQUENCE OF (context tag 1) containing recursive Values
//! - `Value::Object` -> SEQUENCE OF (context tag 0) key-value pairs

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::{
    de::{Decoder, Error as DecodeError},
    enc::Encoder,
    types::{
        constraints::{Bounded, Constraint, Extensible, Value as ValueConstraint},
        fields::{Field, Fields},
        Constraints, Identifier, Tag, TagTree,
    },
    AsnType, Decode, Encode,
};
use serde_json::{Map, Number, Value};

// Context tags for distinguishing variants that would otherwise share tags
const TAG_OBJECT: Tag = Tag::new(crate::types::Class::Context, 0);
const TAG_ARRAY: Tag = Tag::new(crate::types::Class::Context, 1);
const TAG_DECIMAL: Tag = Tag::new(crate::types::Class::Context, 2); // For non-integer numbers
const CHOICE_LIST: [TagTree; 7] = [
    TagTree::Leaf(Tag::NULL),        // Null
    TagTree::Leaf(Tag::BOOL),        // Bool
    TagTree::Leaf(Tag::INTEGER),     // Number (integer)
    TagTree::Leaf(TAG_DECIMAL),      // Number (decimal/float)
    TagTree::Leaf(Tag::UTF8_STRING), // String
    TagTree::Leaf(TAG_ARRAY),        // Array
    TagTree::Leaf(TAG_OBJECT),       // Object
];
/// Number of variants in the Value CHOICE type (Null, Bool, Integer, Decimal, String, Array, Object)
const VARIANT_COUNT: usize = 7;

impl AsnType for Value {
    const TAG: Tag = Tag::EOC; // CHOICE types use EOC
    const TAG_TREE: TagTree = TagTree::Choice(&CHOICE_LIST);
    const IS_CHOICE: bool = true;
}

impl crate::types::Choice for Value {
    const VARIANTS: &'static [TagTree] = &CHOICE_LIST;

    const VARIANCE_CONSTRAINT: Constraints =
        Constraints::new(&[Constraint::Value(Extensible::new(ValueConstraint::new(
            Bounded::const_new(0, (VARIANT_COUNT - 1) as i128),
        )))]);

    const IDENTIFIERS: &'static [&'static str] = &[
        "null", "bool", "integer", "decimal", "string", "array", "object",
    ];
}

impl crate::types::DecodeChoice for Value {
    fn from_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        match tag {
            Tag::NULL => {
                decoder.decode_null(Tag::NULL)?;
                Ok(Value::Null)
            }
            Tag::BOOL => {
                let b = decoder.decode_bool(Tag::BOOL)?;
                Ok(Value::Bool(b))
            }
            Tag::INTEGER => {
                let i: i64 = decoder.decode_integer(Tag::INTEGER, Constraints::default())?;
                Ok(Value::Number(Number::from(i)))
            }
            t if t == TAG_DECIMAL => {
                // Decimal numbers are encoded as UTF8String with context tag
                let s = decoder.decode_utf8_string(TAG_DECIMAL, Constraints::default())?;
                if let Ok(n) = s.parse::<f64>() {
                    if let Some(num) = Number::from_f64(n) {
                        return Ok(Value::Number(num));
                    }
                }
                // Fallback: return as-is if parsing fails (shouldn't happen)
                Err(DecodeError::custom(
                    alloc::format!("Failed to parse decimal number: {s}"),
                    decoder.codec(),
                ))
            }
            Tag::UTF8_STRING => {
                let s = decoder.decode_utf8_string(Tag::UTF8_STRING, Constraints::default())?;
                Ok(Value::String(s.into()))
            }
            t if t == TAG_ARRAY => {
                let arr: Vec<Value> =
                    decoder.decode_sequence_of(TAG_ARRAY, Constraints::default())?;
                Ok(Value::Array(arr))
            }
            t if t == TAG_OBJECT => {
                let pairs: Vec<JsonKeyValue> =
                    decoder.decode_sequence_of(TAG_OBJECT, Constraints::default())?;
                let map: Map<String, Value> =
                    pairs.into_iter().map(|kv| (kv.key, kv.value)).collect();
                Ok(Value::Object(map))
            }
            _ => Err(DecodeError::no_valid_choice(
                "serde_json::Value",
                decoder.codec(),
            )),
        }
    }
}

impl Encode for Value {
    fn encode<'b, E: Encoder<'b>>(&self, encoder: &mut E) -> Result<(), E::Error> {
        // For CHOICE types, we use encode_choice
        let tag = value_to_tag(self);
        encoder.encode_choice::<Self>(
            Self::CONSTRAINTS,
            tag,
            |enc| {
                match self {
                    Value::Null => {
                        enc.encode_null(Tag::NULL, Identifier::EMPTY)?;
                    }
                    Value::Bool(b) => {
                        enc.encode_bool(Tag::BOOL, *b, Identifier::EMPTY)?;
                    }
                    Value::Number(n) => {
                        // Try to encode as integer if possible
                        if let Some(i) = n.as_i64() {
                            enc.encode_integer(
                                Tag::INTEGER,
                                Constraints::default(),
                                &i,
                                Identifier::EMPTY,
                            )?;
                        } else {
                            // For floats and large numbers, encode as UTF8String with TAG_DECIMAL
                            let s = n.to_string();
                            enc.encode_utf8_string(
                                TAG_DECIMAL,
                                Constraints::default(),
                                &s,
                                Identifier::EMPTY,
                            )?;
                        }
                    }
                    Value::String(s) => {
                        enc.encode_utf8_string(
                            Tag::UTF8_STRING,
                            Constraints::default(),
                            s,
                            Identifier::EMPTY,
                        )?;
                    }
                    Value::Array(arr) => {
                        enc.encode_sequence_of(
                            TAG_ARRAY,
                            arr,
                            Constraints::default(),
                            Identifier::EMPTY,
                        )?;
                    }
                    Value::Object(map) => {
                        let pairs: Vec<JsonKeyValue> = map
                            .iter()
                            .map(|(k, v)| JsonKeyValue {
                                key: k.clone(),
                                value: v.clone(),
                            })
                            .collect();
                        enc.encode_sequence_of(
                            TAG_OBJECT,
                            &pairs,
                            Constraints::default(),
                            Identifier::EMPTY,
                        )?;
                    }
                }
                Ok(tag)
            },
            Self::IDENTIFIER,
        )?;
        Ok(())
    }

    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        _tag: Tag,
        _constraints: Constraints,
        _identifier: Identifier,
    ) -> Result<(), E::Error> {
        // CHOICE types ignore the outer tag - they use their variant's tag
        self.encode(encoder)
    }
}

impl Decode for Value {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.decode_choice::<Self>(Constraints::default())
    }

    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<Self, D::Error> {
        // CHOICE types ignore the outer tag
        Self::decode(decoder)
    }
}

/// Helper function to determine the tag for a Value variant
fn value_to_tag(value: &Value) -> Tag {
    match value {
        Value::Null => Tag::NULL,
        Value::Bool(_) => Tag::BOOL,
        Value::Number(n) => {
            if n.as_i64().is_some() {
                Tag::INTEGER
            } else {
                TAG_DECIMAL
            }
        }
        Value::String(_) => Tag::UTF8_STRING,
        Value::Array(_) => TAG_ARRAY,
        Value::Object(_) => TAG_OBJECT,
    }
}

/// Helper type for encoding JSON object key-value pairs
#[derive(Clone, Debug, PartialEq)]
struct JsonKeyValue {
    key: String,
    value: Value,
}

impl AsnType for JsonKeyValue {
    const TAG: Tag = Tag::SEQUENCE;
}

impl crate::types::Constructed<2, 0> for JsonKeyValue {
    const FIELDS: Fields<2> = Fields::from_static([
        Field::new_required(0, Tag::UTF8_STRING, TagTree::Leaf(Tag::UTF8_STRING), "key"),
        Field::new_required(1, Tag::EOC, Value::TAG_TREE, "value"),
    ]);
}

impl Encode for JsonKeyValue {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder.encode_sequence::<2, 0, Self, _>(
            tag,
            |enc| {
                enc.encode_utf8_string(
                    Tag::UTF8_STRING,
                    Constraints::default(),
                    &self.key,
                    Identifier::EMPTY,
                )?;
                self.value.encode(enc)?;
                Ok(())
            },
            identifier,
        )?;
        Ok(())
    }
}

impl Decode for JsonKeyValue {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_sequence::<2, 0, Self, _, _>(tag, None::<fn() -> Self>, |dec| {
            let key = dec.decode_utf8_string(Tag::UTF8_STRING, Constraints::default())?;
            let value = Value::decode(dec)?;
            Ok(JsonKeyValue {
                key: key.into(),
                value,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn round_trip_der(value: &Value) {
        let encoded = crate::der::encode(value).expect("DER encode failed");
        let decoded: Value = crate::der::decode(&encoded).expect("DER decode failed");
        assert_eq!(value, &decoded);
    }

    fn round_trip_ber(value: &Value) {
        let encoded = crate::ber::encode(value).expect("BER encode failed");
        let decoded: Value = crate::ber::decode(&encoded).expect("BER decode failed");
        assert_eq!(value, &decoded);
    }

    #[test]
    fn test_null() {
        let value = json!(null);
        round_trip_der(&value);
        round_trip_ber(&value);
    }

    #[test]
    fn test_bool() {
        round_trip_der(&json!(true));
        round_trip_der(&json!(false));
        round_trip_ber(&json!(true));
        round_trip_ber(&json!(false));
    }

    #[test]
    fn test_integer() {
        round_trip_der(&json!(0));
        round_trip_der(&json!(42));
        round_trip_der(&json!(-17));
        round_trip_der(&json!(i64::MAX));
        round_trip_der(&json!(i64::MIN));
    }

    #[test]
    fn test_string() {
        round_trip_der(&json!(""));
        round_trip_der(&json!("hello"));
        round_trip_der(&json!("with \"quotes\""));
        round_trip_der(&json!("unicode: 你好"));
    }

    #[test]
    fn test_array() {
        round_trip_der(&json!([]));
        round_trip_der(&json!([1, 2, 3]));
        round_trip_der(&json!([null, true, "hello"]));
        round_trip_der(&json!([[1, 2], [3, 4]]));
    }

    #[test]
    fn test_object() {
        round_trip_der(&json!({}));
        round_trip_der(&json!({"key": "value"}));
        round_trip_der(&json!({"a": 1, "b": 2}));
        round_trip_der(&json!({"nested": {"inner": true}}));
    }

    #[test]
    fn test_complex_structure() {
        let value = json!({
            "users": [
                {"id": 1, "name": "Alice", "active": true},
                {"id": 2, "name": "Bob", "active": false}
            ],
            "metadata": {
                "version": "1.0",
                "count": 2
            },
            "tags": ["important", "reviewed"],
            "notes": null
        });
        round_trip_der(&value);
        round_trip_ber(&value);
    }

    #[test]
    fn test_oer() {
        let value = json!({"name": "test", "value": 42});
        let encoded = crate::oer::encode(&value).expect("OER encode failed");
        let decoded: Value = crate::oer::decode(&encoded).expect("OER decode failed");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_uper() {
        let value = json!({"name": "test", "value": 42});
        let encoded = crate::uper::encode(&value).expect("UPER encode failed");
        let decoded: Value = crate::uper::decode(&encoded).expect("UPER decode failed");
        assert_eq!(value, decoded);
    }
}
