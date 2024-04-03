//! # Encoding JER.

use jzon::{object::Object, JsonValue};

use crate::{
    error::{EncodeError, JerEncodeErrorKind},
    types::{fields::Fields, variants},
};

pub struct Encoder {
    stack: alloc::vec::Vec<alloc::string::String>,
    constructed_stack: alloc::vec::Vec<Object>,
    root_value: Option<JsonValue>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            stack: alloc::vec![],
            constructed_stack: alloc::vec![],
            root_value: None,
        }
    }

    pub fn root_value(self) -> Result<JsonValue, EncodeError> {
        Ok(self
            .root_value
            .ok_or(JerEncodeErrorKind::NoRootValueFound)?)
    }

    pub fn to_json(self) -> alloc::string::String {
        self.root_value.map_or(<_>::default(), |v| v.dump())
    }

    fn update_root_or_constructed(&mut self, value: JsonValue) -> Result<(), EncodeError> {
        match self.stack.pop() {
            Some(id) => {
                self.constructed_stack
                    .last_mut()
                    .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?
                    .insert(&id, value);
            }
            None => {
                self.root_value = Some(value);
            }
        };
        Ok(())
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();

    type Error = EncodeError;

    fn encode_any(
        &mut self,
        t: crate::Tag,
        value: &crate::types::Any,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(t, <_>::default(), &value.contents)
    }

    fn encode_bool(&mut self, _: crate::Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::Boolean(value))
    }

    fn encode_bit_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(value.iter().fold(
            alloc::string::String::new(),
            |mut acc, bit| {
                acc.push(if *bit { '1' } else { '0' });
                acc
            },
        )))
    }

    fn encode_enumerated<E: crate::types::Enumerated>(
        &mut self,
        _: crate::Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from(value.identifier())
        ))
    }

    fn encode_object_identifier(
        &mut self,
        _t: crate::Tag,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            value
                .iter()
                .map(|arc| alloc::format!("{arc}"))
                .collect::<alloc::vec::Vec<alloc::string::String>>()
                .join("."),
        ))
    }

    fn encode_integer(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        let as_i64: i64 =
            value
                .try_into()
                .map_err(|_| JerEncodeErrorKind::ExceedsSupportedIntSize {
                    value: value.clone(),
                })?;
        self.update_root_or_constructed(JsonValue::Number(as_i64.into()))
    }

    fn encode_null(&mut self, _: crate::Tag) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::Null)
    }

    fn encode_octet_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(value.iter().fold(
            alloc::string::String::new(),
            |mut acc, bit| {
                acc.push_str(&alloc::format!("{bit:02X?}"));
                acc
            },
        )))
    }

    fn encode_general_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_utf8_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(value.into()))
    }

    fn encode_visible_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_ia5_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_printable_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.as_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_numeric_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.as_bytes().to_vec())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_teletex_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        _value: &crate::types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &crate::types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(value.to_bytes())
                .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_generalized_time(
        &mut self,
        _t: crate::Tag,
        value: &crate::types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
            )
            .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_utc_time(
        &mut self,
        _t: crate::Tag,
        value: &crate::types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::String(
            alloc::string::String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
            )
            .map_err(|e| JerEncodeErrorKind::InvalidCharacter { error: e })?,
        ))
    }

    fn encode_explicit_prefix<V: crate::Encode>(
        &mut self,
        _: crate::Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_sequence<C, F>(
        &mut self,
        __t: crate::Tag,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        let mut field_names = [C::FIELDS, C::EXTENDED_FIELDS.unwrap_or(Fields::empty())]
            .iter()
            .flat_map(|f| f.iter())
            .map(|f| f.name)
            .collect::<alloc::vec::Vec<&str>>();
        field_names.reverse();
        for name in field_names {
            self.stack.push(name.replace('-', "_"));
        }
        self.constructed_stack.push(Object::new());
        (encoder_scope)(self)?;
        let value_map =
            self.constructed_stack
                .pop()
                .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                    msg: "Internal stack mismatch!".into(),
                })?;
        self.update_root_or_constructed(JsonValue::Object(value_map))
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _t: crate::Tag,
        value: &[E],
        _c: crate::types::Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::Array(value.iter().try_fold(
            alloc::vec![],
            |mut acc, v| {
                let mut item_encoder = Self::new();
                v.encode(&mut item_encoder).and(
                    item_encoder
                        .root_value()
                        .map(|rv| acc.push(rv))
                        .map(|_| acc),
                )
            },
        )?))
    }

    fn encode_set<C, F>(&mut self, tag: crate::Tag, value: F) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        self.encode_sequence::<C, F>(tag, value)
    }

    fn encode_set_of<E: crate::Encode>(
        &mut self,
        _t: crate::Tag,
        value: &crate::types::SetOf<E>,
        _c: crate::types::Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(JsonValue::Array(value.iter().try_fold(
            alloc::vec![],
            |mut acc, v| {
                let mut item_encoder = Self::new();
                v.encode(&mut item_encoder).and(
                    item_encoder
                        .root_value()
                        .map(|rv| acc.push(rv))
                        .map(|_| acc),
                )
            },
        )?))
    }

    fn encode_some<E: crate::Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: crate::Encode>(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_none<E: crate::Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.stack.pop();
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _t: crate::Tag) -> Result<Self::Ok, Self::Error> {
        self.stack.pop();
        Ok(())
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _c: crate::types::Constraints,
        tag: crate::types::Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<crate::Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let variants = variants::Variants::from_slice(
            &[E::VARIANTS, E::EXTENDED_VARIANTS.unwrap_or(&[])].concat(),
        );

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
            self.update_root_or_constructed(JsonValue::Object(Object::new()))
        } else {
            self.constructed_stack.push(Object::new());
            self.stack.push(identifier.replace('-', "_"));
            (encode_fn)(self)?;
            let value_map =
                self.constructed_stack
                    .pop()
                    .ok_or_else(|| JerEncodeErrorKind::JsonEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?;
            self.update_root_or_constructed(JsonValue::Object(value_map))
        }
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _t: crate::Tag,
        _c: crate::types::Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: crate::Encode + crate::types::Constructed,
    {
        match value {
            Some(v) => v.encode(self),
            None => self.encode_none::<E>(),
        }
    }

    fn codec(&self) -> crate::Codec {
        crate::Codec::Jer
    }
}
