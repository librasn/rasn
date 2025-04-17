//! Encoding Rust structures into JSON Encoding Rules data.

use alloc::string::ToString;

use serde_json::{Map, Value};

type ValueMap = Map<alloc::string::String, Value>;

use crate::{
    error::{EncodeError, JerEncodeErrorKind},
    types::{Constraints, Identifier, IntegerType, Tag},
};

use crate::types::RealType;

/// Encodes Rust structures into JSON Encoding Rules data.
pub struct Encoder {
    stack: alloc::vec::Vec<&'static str>,
    constructed_stack: alloc::vec::Vec<ValueMap>,
    root_value: Option<Value>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// Creates new default encoder.
    pub fn new() -> Self {
        Self {
            stack: alloc::vec![],
            constructed_stack: alloc::vec![],
            root_value: None,
        }
    }

    /// Returns the complete encoded JSON value, consuming the encoder.
    pub fn to_json(self) -> Result<Value, EncodeError> {
        Ok(self
            .root_value
            .ok_or(JerEncodeErrorKind::NoRootValueFound)?)
    }

    /// Returns the complete encoded JSON value formatted to a string, consuming the encoder.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> alloc::string::String {
        self.root_value.map_or(<_>::default(), |v| v.to_string())
    }

    fn update_root_or_constructed(&mut self, value: Value) -> Result<(), EncodeError> {
        match self.stack.pop() {
            Some(id) => {
                self.constructed_stack
                    .last_mut()
                    .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?
                    .insert(id.to_string(), value);
            }
            None => {
                self.root_value = Some(value);
            }
        };
        Ok(())
    }
}

impl crate::Encoder<'_> for Encoder {
    type Ok = ();

    type Error = EncodeError;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder;

    fn encode_any(
        &mut self,
        t: crate::types::Tag,
        value: &crate::types::Any,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            t,
            Constraints::default(),
            &value.contents,
            Identifier::EMPTY,
        )
    }

    fn encode_bool(&mut self, _: Tag, value: bool, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::Bool(value))
    }

    fn encode_bit_string(
        &mut self,
        _t: Tag,
        constraints: crate::types::Constraints,
        value: &crate::types::BitStr,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut bitvec = value.to_bitvec();
        bitvec.force_align();
        let bytes = bitvec
            .into_vec()
            .iter()
            .fold(alloc::string::String::new(), |mut acc, bit| {
                acc.push_str(&alloc::format!("{bit:02X?}"));
                acc
            });
        let json_value = if constraints.size().is_some_and(|s| s.constraint.is_fixed()) {
            Value::String(bytes)
        } else {
            let mut value_map = ValueMap::new();
            value_map.insert("value".into(), bytes.into());
            value_map.insert("length".into(), Value::Number(value.len().into()));
            Value::Object(value_map)
        };
        self.update_root_or_constructed(json_value)
    }

    fn encode_enumerated<E: crate::types::Enumerated>(
        &mut self,
        _: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(alloc::string::String::from(
            value.identifier(),
        )))
    }

    fn encode_object_identifier(
        &mut self,
        _t: Tag,
        value: &[u32],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            value
                .iter()
                .map(|arc| alloc::format!("{arc}"))
                .collect::<alloc::vec::Vec<alloc::string::String>>()
                .join("."),
        ))
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &I,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if let Some(as_i64) = value.to_i64() {
            self.update_root_or_constructed(Value::Number(as_i64.into()))
        } else {
            Err(JerEncodeErrorKind::ExceedsSupportedIntSize {
                value: value.to_bigint().unwrap_or_default(),
            }
            .into())
        }
    }

    fn encode_real<R: RealType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &R,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        use num_traits::{float::FloatCore, ToPrimitive, Zero};

        let as_float = value
            .try_to_float()
            .ok_or(JerEncodeErrorKind::ExceedsSupportedRealRange)?;

        if as_float.is_infinite() {
            if as_float.is_sign_positive() {
                self.update_root_or_constructed(Value::String("INF".into()))
            } else {
                self.update_root_or_constructed(Value::String("-INF".into()))
            }
        } else if as_float.is_nan() {
            self.update_root_or_constructed(Value::String("NAN".into()))
        } else if as_float.is_zero() && as_float.is_sign_negative() {
            self.update_root_or_constructed(Value::String("-0".into()))
        } else if let Some(number) = as_float.to_f64().and_then(serde_json::Number::from_f64) {
            self.update_root_or_constructed(number.into())
        } else {
            Err(JerEncodeErrorKind::ExceedsSupportedRealRange.into())
        }
    }

    fn encode_null(&mut self, _: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::Null)
    }

    fn encode_octet_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &[u8],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(value.iter().fold(
            alloc::string::String::new(),
            |mut acc, bit| {
                acc.push_str(&alloc::format!("{bit:02X?}"));
                acc
            },
        )))
    }

    fn encode_general_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::GeneralString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_graphic_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::GraphicString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_utf8_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &str,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(value.into()))
    }

    fn encode_visible_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::VisibleString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_ia5_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::Ia5String,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_printable_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::PrintableString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.as_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_numeric_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::NumericString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.as_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_teletex_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        _value: &crate::types::TeletexString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &crate::types::BmpString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(value.to_bytes())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_generalized_time(
        &mut self,
        _t: Tag,
        value: &crate::types::GeneralizedTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
            )
            .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_utc_time(
        &mut self,
        _t: Tag,
        value: &crate::types::UtcTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
            )
            .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_date(
        &mut self,
        _t: Tag,
        value: &crate::types::Date,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::String(
            alloc::string::String::from_utf8(crate::ber::enc::Encoder::naivedate_to_date_bytes(
                value,
            ))
            .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_explicit_prefix<V: crate::Encode>(
        &mut self,
        _: Tag,
        value: &V,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        __t: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let mut field_names = C::FIELDS
            .iter()
            .map(|f| f.name)
            .collect::<alloc::vec::Vec<&str>>();
        if let Some(extended_fields) = C::EXTENDED_FIELDS {
            field_names.extend(extended_fields.iter().map(|f| f.name));
        }
        field_names.reverse();
        for name in field_names {
            self.stack.push(name);
        }
        self.constructed_stack.push(ValueMap::new());
        (encoder_scope)(self)?;
        let value_map =
            self.constructed_stack
                .pop()
                .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                    msg: "Internal stack mismatch!".into(),
                })?;
        self.update_root_or_constructed(Value::Object(value_map))
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _t: Tag,
        value: &[E],
        _c: crate::types::Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::Array(value.iter().try_fold(
            alloc::vec![],
            |mut acc, v| {
                let mut item_encoder = Self::new();
                v.encode(&mut item_encoder)
                    .and(item_encoder.to_json().map(|rv| acc.push(rv)).map(|_| acc))
            },
        )?))
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        value: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        self.encode_sequence::<RL, EL, C, F>(tag, value, Identifier::EMPTY)
    }

    fn encode_set_of<E: crate::Encode + Eq + core::hash::Hash>(
        &mut self,
        _t: Tag,
        value: &crate::types::SetOf<E>,
        _c: crate::types::Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(Value::Array(value.to_vec().iter().try_fold(
            alloc::vec![],
            |mut acc, v| {
                let mut item_encoder = Self::new();
                v.encode(&mut item_encoder)
                    .and(item_encoder.to_json().map(|rv| acc.push(rv)).map(|_| acc))
            },
        )?))
    }

    fn encode_some<E: crate::Encode>(
        &mut self,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: crate::Encode>(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_none<E: crate::Encode>(&mut self, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.stack.pop();
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _t: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.stack.pop();
        Ok(())
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _c: crate::types::Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let variants = &[E::VARIANTS, E::EXTENDED_VARIANTS.unwrap_or(&[])].concat();

        let identifier = variants
            .iter()
            .enumerate()
            .find_map(|(i, &variant_tag)| {
                (tag == variant_tag)
                    .then_some(E::IDENTIFIERS.get(i))
                    .flatten()
            })
            .ok_or_else(|| crate::error::EncodeError::variant_not_in_choice(self.codec()))?;

        if variants.is_empty() {
            self.update_root_or_constructed(Value::Object(ValueMap::new()))
        } else {
            self.constructed_stack.push(ValueMap::new());
            self.stack.push(identifier);
            (encode_fn)(self)?;
            let value_map =
                self.constructed_stack
                    .pop()
                    .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?;
            self.update_root_or_constructed(Value::Object(value_map))
        }
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        value: Option<&E>,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: crate::Encode + crate::types::Constructed<RL, EL>,
    {
        match value {
            Some(v) => v.encode(self),
            None => self.encode_none::<E>(Identifier::EMPTY),
        }
    }

    fn codec(&self) -> crate::Codec {
        crate::Codec::Jer
    }
}
