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

use crate::jer::enc::ValueMap;
use crate::{
    AsnType, Decode, Encode,
    de::{Decoder, Error as DecodeError},
    enc::Encoder,
    types::{
        Constraints, Identifier, Tag, TagTree,
        constraints::{Bounded, Constraint, Extensible, Value as ValueConstraint},
        fields::{Field, Fields},
    },
};
use serde_json::{Number, Value};

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

                if let Some(num) = s.parse::<f64>().ok().and_then(Number::from_f64) {
                    return Ok(Value::Number(num));
                }
                // Fallback: return as-is if parsing fails (shouldn't happen)
                Err(DecodeError::custom(
                    alloc::format!("Failed to parse decimal number: {s}"),
                    decoder.codec(),
                ))
            }
            Tag::UTF8_STRING => {
                let s = decoder.decode_utf8_string(Tag::UTF8_STRING, Constraints::default())?;
                Ok(Value::String(s))
            }
            t if t == TAG_ARRAY => {
                let arr: Vec<Value> =
                    decoder.decode_sequence_of(TAG_ARRAY, Constraints::default())?;
                Ok(Value::Array(arr))
            }
            t if t == TAG_OBJECT => {
                let map: ValueMap = ValueMap::decode_with_tag_and_constraints(
                    decoder,
                    TAG_OBJECT,
                    Constraints::default(),
                )?;
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
                        map.encode_with_tag_and_constraints(
                            enc,
                            TAG_OBJECT,
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

// ASN.1 trait implementations for ValueMap (Map<String, Value>)

impl AsnType for ValueMap {
    const TAG: Tag = TAG_OBJECT;
}

/// Helper type for encoding a single JSON object key-value entry.
/// Used internally by `ValueMap` encoding.
#[derive(Clone, Debug, PartialEq)]
struct ValueEntry<'a> {
    key: &'a str,
    value: &'a Value,
}

impl AsnType for ValueEntry<'_> {
    const TAG: Tag = Tag::SEQUENCE;
}

/// Owned version of ValueEntry for decoding.
#[derive(Clone, Debug, PartialEq)]
struct OwnedValueEntry {
    key: String,
    value: Value,
}

impl AsnType for OwnedValueEntry {
    const TAG: Tag = Tag::SEQUENCE;
}

impl crate::types::Constructed<2, 0> for OwnedValueEntry {
    const FIELDS: Fields<2> = Fields::from_static([
        Field::new_required(0, Tag::UTF8_STRING, TagTree::Leaf(Tag::UTF8_STRING), "key"),
        Field::new_required(1, Tag::EOC, Value::TAG_TREE, "value"),
    ]);
}

impl Encode for ValueEntry<'_> {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder.encode_sequence::<2, 0, OwnedValueEntry, _>(
            tag,
            |enc| {
                enc.encode_utf8_string(
                    Tag::UTF8_STRING,
                    Constraints::default(),
                    self.key,
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

impl Decode for OwnedValueEntry {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_sequence::<2, 0, Self, _, _>(tag, None::<fn() -> Self>, |dec| {
            let key = dec.decode_utf8_string(Tag::UTF8_STRING, Constraints::default())?;
            let value = Value::decode(dec)?;
            Ok(OwnedValueEntry { key, value })
        })
    }
}

impl Encode for ValueMap {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        let entries: Vec<ValueEntry<'_>> = self
            .iter()
            .map(|(k, v)| ValueEntry { key: k, value: v })
            .collect();
        encoder.encode_sequence_of(tag, &entries, constraints, identifier)?;
        Ok(())
    }
}

impl Decode for ValueMap {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        let entries: Vec<OwnedValueEntry> = decoder.decode_sequence_of(tag, constraints)?;
        Ok(entries.into_iter().map(|e| (e.key, e.value)).collect())
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
        round_trip_der(&json!("42"));
    }

    #[test]
    fn test_array() {
        round_trip_der(&json!([]));
        round_trip_der(&json!([1, 2, 3]));
        round_trip_der(&json!([null, true, "hello"]));
        round_trip_der(&json!([[1, 2], [3, 4]]));
        // Large array test
        let arr: Vec<i32> = (0..100).collect();
        let value = json!(arr);
        round_trip_der(&value);
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
    }
    // ============================================================
    // Test cases adapted from serde-json-canonicalizer https://github.com/evik42/serde-json-canonicalizer
    // ============================================================

    // Basic types tests (from basic_types.rs)
    #[test]
    fn test_literals() {
        // null, true, false
        round_trip_der(&json!(null));
        round_trip_der(&json!(true));
        round_trip_der(&json!(false));
    }

    #[test]
    fn test_number() {
        round_trip_der(&json!(42));
        round_trip_der(&json!(0));
        // JavaScript's MAX_SAFE_INTEGER: 2^53 - 1 = 9007199254740991
        round_trip_der(&json!(9007199254740991i64));
        round_trip_der(&json!(-9007199254740991i64));

        // u8 max
        round_trip_der(&json!(255));
        // i8 range
        round_trip_der(&json!(127));
        round_trip_der(&json!(-128));
        // u16 max
        round_trip_der(&json!(65535));
        // i16 range
        round_trip_der(&json!(32767));
        round_trip_der(&json!(-32768));
        // u32 max
        round_trip_der(&json!(4294967295u64));
        // i32 range
        round_trip_der(&json!(2147483647));
        round_trip_der(&json!(-2147483648i64));
    }

    #[test]
    fn test_basic_string_number() {}

    #[test]
    fn test_empty() {
        // empty array
        round_trip_der(&json!([]));
        // empty object
        round_trip_der(&json!({}));
        // empty key
        round_trip_der(&json!({"": "empty"}));
    }

    // Unicode and special character tests (adapted from weird.rs and unicode tests)
    #[test]
    fn test_unicode() {
        round_trip_der(&json!({"€": "Euro Sign"}));
        // unicode_control_char
        round_trip_der(&json!({"\u{0080}": "Control\u{007f}"}));
        round_trip_der(&json!({"😂": "Smiley"}));
        round_trip_der(&json!({"ö": "Latin Small Letter O With Diaeresis"}));
        round_trip_der(&json!({"\u{fb33}": "Hebrew Letter Dalet With Dagesh"}));
        // A with combining ring above (unnormalized form of Å)
        round_trip_der(&json!({"Unnormalized Unicode": "A\u{030a}"}));
        // String with various escape sequences
        round_trip_der(&json!({ "string": "€$\u{000f}\nA'B\"\\\\\"/", }));
    }

    #[test]
    fn test_separators() {
        round_trip_der(&json!({"\r": "Carriage Return"}));
        round_trip_der(&json!({"\n": "Newline"}));
        round_trip_der(&json!({"</script>": "Browser Challenge"}));
        round_trip_der(&json!("column1\tcolumn2"));
        round_trip_der(&json!("path\\to\\file"));
        round_trip_der(&json!("before\u{0000}after"));
    }

    // Array tests (from arrays.json)
    #[test]
    fn test_nested() {
        let value = json!([
            56,
            {
                "d": true,
                "10": null,
                "1": []
            }
        ]);
        round_trip_der(&value);
        let value = json!({
            "1": {"f": {"f": "hi", "F": 5}, "\n": 56},
            "10": {},
            "": "empty",
            "a": {},
            "111": [{"e": "yes", "E": "no"}],
            "A": {}
        });
        round_trip_der(&value);

        // deeply nested object
        let value = json!([[[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]]]);
        round_trip_der(&value);
        // deeply nested array
        let value = json!({
            "a": {
                "b": {
                    "c": {
                        "d": {
                            "e": "deep"
                        }
                    }
                }
            }
        });
        round_trip_der(&value);

        // Mixed content tests
        let value = json!({
            "null_val": null,
            "bool_true": true,
            "bool_false": false,
            "int_pos": 42,
            "int_neg": -17,
            "int_zero": 0,
            "string": "hello world",
            "string_empty": "",
            "string_unicode": "こんにちは",
            "array_empty": [],
            "array_mixed": [1, "two", true, null],
            "object_empty": {},
            "object_nested": {"inner": {"value": 123}}
        });
        round_trip_der(&value);
    }

    // Multiple codecs comprehensive test
    #[test]
    fn test_all_codecs() {
        let value = json!({
            "nested": {"key": "value", "array": [1, 2, 3], "a_key": "value"},
            "test": "value",
            "number": 42,
            "array": [1, 2, 3],
        });
        let value2 = json!({
            "array": [1, 2, 3],
            "nested": {"array": [1, 2, 3], "a_key": "value", "key": "value"},
            "number": 42,
            "test": "value",
        });
        let value3 = json!({
            "array": [1, 2, 3],
            "nested": {"a_key": "value", "array": [1, 2, 3], "key": "value"},
            "test": "value",
            "number": 42,
        });

        // DER canonical
        let der_enc = crate::der::encode(&value).unwrap();
        let der_enc2 = crate::der::encode(&value2).unwrap();
        let der_enc3 = crate::der::encode(&value3).unwrap();
        assert_eq!(der_enc, der_enc2);
        assert_eq!(der_enc, der_enc3);
        let der_dec: Value = crate::der::decode(&der_enc).expect("DER decode failed");
        assert_eq!(value, der_dec);

        // BER non canonical
        let ber_enc = crate::ber::encode(&value).unwrap();
        let ber_dec: Value = crate::ber::decode(&ber_enc).expect("BER decode failed");
        assert_eq!(value, ber_dec);

        // OER
        let oer_enc = crate::oer::encode(&value).unwrap();
        let oer_enc2 = crate::oer::encode(&value2).unwrap();
        let oer_enc3 = crate::oer::encode(&value3).unwrap();
        assert_eq!(oer_enc, oer_enc2);
        assert_eq!(oer_enc, oer_enc3);
        let oer_dec: Value = crate::oer::decode(&oer_enc).expect("OER decode failed");
        assert_eq!(value, oer_dec);

        // COER
        let coer_enc = crate::coer::encode(&value).unwrap();
        let coer_enc2 = crate::coer::encode(&value2).unwrap();
        let coer_enc3 = crate::coer::encode(&value3).unwrap();
        assert_eq!(coer_enc, coer_enc2);
        assert_eq!(coer_enc, coer_enc3);
        let coer_dec: Value = crate::coer::decode(&coer_enc).expect("COER decode failed");
        assert_eq!(value, coer_dec);

        // UPER
        let uper_enc = crate::uper::encode(&value).unwrap();
        let uper_enc2 = crate::uper::encode(&value2).unwrap();
        let uper_enc3 = crate::uper::encode(&value3).unwrap();
        assert_eq!(uper_enc, uper_enc2);
        assert_eq!(uper_enc, uper_enc3);
        let uper_dec: Value = crate::uper::decode(&uper_enc).expect("UPER decode failed");
        assert_eq!(value, uper_dec);

        // APER is not designed to be canonical
        let aper_enc = crate::aper::encode(&value).unwrap();
        let aper_dec: Value = crate::aper::decode(&aper_enc).expect("APER decode failed");
        assert_eq!(value, aper_dec);
    }
}
