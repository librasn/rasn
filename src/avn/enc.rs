//! Encoding Rust structures into ASN.1 Value Notation (AVN) text format.

use alloc::string::ToString;

use super::value::AvnValue;
use crate::types::RealType;
use crate::{
    error::{AvnEncodeErrorKind, EncodeError},
    types::{Constraints, Identifier, IntegerType, Tag, variants},
};

/// Encodes Rust structures into ASN.1 Value Notation text.
pub struct Encoder {
    /// Stack of field names waiting to be consumed by the next `update_root_or_constructed` call.
    stack: alloc::vec::Vec<&'static str>,
    /// Stack of in-progress SEQUENCE / SET / CHOICE constructed values.
    constructed_stack: alloc::vec::Vec<alloc::vec::Vec<(alloc::string::String, AvnValue)>>,
    /// The completed root value (set once encoding is done).
    root_value: Option<AvnValue>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// Creates a new default encoder.
    pub fn new() -> Self {
        Self {
            stack: alloc::vec![],
            constructed_stack: alloc::vec![],
            root_value: None,
        }
    }

    /// Returns the complete encoded AVN value, consuming the encoder.
    pub fn to_avn(self) -> Result<AvnValue, EncodeError> {
        self.root_value
            .ok_or_else(|| AvnEncodeErrorKind::AvnNoRootValueFound.into())
    }

    /// Returns the encoded AVN value formatted as a string, consuming the encoder.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> alloc::string::String {
        self.to_avn()
            .ok()
            .map(|v| v.to_string())
            .unwrap_or_default()
    }

    /// Places `value` into the top of the constructed stack (keyed by the
    /// name popped from `self.stack`), or sets the root value when the stack
    /// is empty.
    fn update_root_or_constructed(&mut self, value: AvnValue) -> Result<(), EncodeError> {
        match self.stack.pop() {
            Some(id) => {
                self.constructed_stack
                    .last_mut()
                    .ok_or_else(|| AvnEncodeErrorKind::AvnEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?
                    .push((id.to_string(), value));
            }
            None => {
                self.root_value = Some(value);
            }
        }
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
            value.as_bytes(),
            Identifier::EMPTY,
        )
    }

    fn encode_bool(&mut self, _: Tag, value: bool, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::Boolean(value))
    }

    fn encode_bit_string(
        &mut self,
        _t: Tag,
        _constraints: Constraints,
        value: &crate::types::BitStr,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bit_length = value.len();
        let mut bitvec = value.to_bitvec();
        // Force-align so that `into_vec()` gives us whole bytes.
        bitvec.force_align();
        let bytes = bitvec.into_vec();
        self.update_root_or_constructed(AvnValue::BitString { bytes, bit_length })
    }

    fn encode_enumerated<E: crate::types::Enumerated>(
        &mut self,
        _: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::Enumerated(alloc::string::String::from(
            value.identifier(),
        )))
    }

    fn encode_object_identifier(
        &mut self,
        _t: Tag,
        value: &[u32],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::Oid(value.to_vec()))
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &I,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // BigInt always works — no i64 limitation unlike JER.
        let s = value.to_bigint().unwrap_or_default().to_string();
        self.update_root_or_constructed(AvnValue::Integer(s))
    }

    fn encode_real<R: RealType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &R,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        use num_traits::{ToPrimitive, Zero, float::FloatCore};

        let as_float = value
            .try_to_float()
            .ok_or(AvnEncodeErrorKind::AvnExceedsSupportedRealRange)?;

        let avn_val = if as_float.is_infinite() {
            if as_float.is_sign_positive() {
                AvnValue::Real("PLUS-INFINITY".into())
            } else {
                AvnValue::Real("MINUS-INFINITY".into())
            }
        } else if as_float.is_nan() {
            AvnValue::Real("NOT-A-NUMBER".into())
        } else if as_float.is_zero() && as_float.is_sign_negative() {
            AvnValue::Real("-0".into())
        } else {
            let f64_val = as_float
                .to_f64()
                .ok_or(AvnEncodeErrorKind::AvnExceedsSupportedRealRange)?;
            AvnValue::Real(alloc::format!("{f64_val}"))
        };
        self.update_root_or_constructed(avn_val)
    }

    fn encode_null(&mut self, _: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::Null)
    }

    fn encode_octet_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &[u8],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::OctetString(value.to_vec()))
    }

    fn encode_general_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::GeneralString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(value.to_vec()).map_err(|e| {
            AvnEncodeErrorKind::AvnEncoder {
                msg: alloc::format!("Invalid UTF-8: {e}"),
            }
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_graphic_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::GraphicString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(value.to_vec()).map_err(|e| {
            AvnEncodeErrorKind::AvnEncoder {
                msg: alloc::format!("Invalid UTF-8: {e}"),
            }
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_utf8_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &str,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.update_root_or_constructed(AvnValue::CharString(value.into()))
    }

    fn encode_visible_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::VisibleString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s =
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec()).map_err(|e| {
                AvnEncodeErrorKind::AvnEncoder {
                    msg: alloc::format!("Invalid UTF-8: {e}"),
                }
            })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_ia5_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::Ia5String,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s =
            alloc::string::String::from_utf8(value.as_iso646_bytes().to_vec()).map_err(|e| {
                AvnEncodeErrorKind::AvnEncoder {
                    msg: alloc::format!("Invalid UTF-8: {e}"),
                }
            })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_printable_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::PrintableString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
            AvnEncodeErrorKind::AvnEncoder {
                msg: alloc::format!("Invalid UTF-8: {e}"),
            }
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_numeric_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::NumericString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
            AvnEncodeErrorKind::AvnEncoder {
                msg: alloc::format!("Invalid UTF-8: {e}"),
            }
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_teletex_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        _value: &crate::types::TeletexString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _t: Tag,
        _c: Constraints,
        value: &crate::types::BmpString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(value.to_bytes()).map_err(|e| {
            AvnEncodeErrorKind::AvnEncoder {
                msg: alloc::format!("Invalid UTF-8: {e}"),
            }
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_generalized_time(
        &mut self,
        _t: Tag,
        value: &crate::types::GeneralizedTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(
            crate::ber::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
        )
        .map_err(|e| AvnEncodeErrorKind::AvnEncoder {
            msg: alloc::format!("Invalid UTF-8: {e}"),
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_utc_time(
        &mut self,
        _t: Tag,
        value: &crate::types::UtcTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(
            crate::ber::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
        )
        .map_err(|e| AvnEncodeErrorKind::AvnEncoder {
            msg: alloc::format!("Invalid UTF-8: {e}"),
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
    }

    fn encode_date(
        &mut self,
        _t: Tag,
        value: &crate::types::Date,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let s = alloc::string::String::from_utf8(
            crate::ber::enc::Encoder::naivedate_to_date_bytes(value),
        )
        .map_err(|e| AvnEncodeErrorKind::AvnEncoder {
            msg: alloc::format!("Invalid UTF-8: {e}"),
        })?;
        self.update_root_or_constructed(AvnValue::CharString(s))
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
        _t: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        // Push field names in reverse so that `pop()` yields them in definition order.
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
        self.constructed_stack.push(alloc::vec![]);
        (encoder_scope)(self)?;
        let fields =
            self.constructed_stack
                .pop()
                .ok_or_else(|| AvnEncodeErrorKind::AvnEncoder {
                    msg: "Internal stack mismatch!".into(),
                })?;
        self.update_root_or_constructed(AvnValue::Sequence(fields))
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _t: Tag,
        value: &[E],
        _c: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let items = value
            .iter()
            .try_fold(alloc::vec::Vec::<AvnValue>::new(), |mut acc, v| {
                let mut item_encoder = Self::new();
                v.encode(&mut item_encoder)?;
                acc.push(item_encoder.to_avn()?);
                Ok::<_, EncodeError>(acc)
            })?;
        self.update_root_or_constructed(AvnValue::SequenceOf(items))
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
        // Treat SET identically to SEQUENCE in AVN output.
        self.encode_sequence::<RL, EL, C, F>(tag, value, Identifier::EMPTY)
    }

    fn encode_set_of<E: crate::Encode + Eq + core::hash::Hash>(
        &mut self,
        _t: Tag,
        value: &crate::types::SetOf<E>,
        _c: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let items =
            value
                .to_vec()
                .iter()
                .try_fold(alloc::vec::Vec::<AvnValue>::new(), |mut acc, v| {
                    let mut item_encoder = Self::new();
                    v.encode(&mut item_encoder)?;
                    acc.push(item_encoder.to_avn()?);
                    Ok::<_, EncodeError>(acc)
                })?;
        self.update_root_or_constructed(AvnValue::SequenceOf(items))
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
        _c: Constraints,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_none<E: crate::Encode>(&mut self, _: Identifier) -> Result<Self::Ok, Self::Error> {
        // Pop the field name from the stack and push an Absent sentinel so the
        // SEQUENCE encoder can filter it out.
        if let (Some(id), Some(top)) = (self.stack.pop(), self.constructed_stack.last_mut()) {
            top.push((id.to_string(), AvnValue::Absent));
        }
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _t: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        if let (Some(id), Some(top)) = (self.stack.pop(), self.constructed_stack.last_mut()) {
            top.push((id.to_string(), AvnValue::Absent));
        }
        Ok(())
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _c: Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        _: Identifier,
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
            self.update_root_or_constructed(AvnValue::Null)
        } else {
            // Push an inner constructed frame and the identifier name so the
            // inner encode_fn deposits the value there.
            self.constructed_stack.push(alloc::vec![]);
            self.stack.push(identifier);
            (encode_fn)(self)?;
            let fields =
                self.constructed_stack
                    .pop()
                    .ok_or_else(|| AvnEncodeErrorKind::AvnEncoder {
                        msg: "Internal stack mismatch!".into(),
                    })?;
            // Retrieve the single inner value that was deposited.
            let inner_value = fields
                .into_iter()
                .next()
                .map(|(_, v)| v)
                .unwrap_or(AvnValue::Null);
            self.update_root_or_constructed(AvnValue::Choice {
                identifier: identifier.to_string(),
                value: alloc::boxed::Box::new(inner_value),
            })
        }
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _t: Tag,
        _c: Constraints,
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
        crate::Codec::Avn
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder as AvnEncoder;
    use crate::enc::Encoder as EncoderTrait;
    use crate::prelude::*;

    #[test]
    fn bool_encodes_correctly() {
        let mut enc = AvnEncoder::new();
        enc.encode_bool(Tag::BOOL, true, Identifier::EMPTY).unwrap();
        assert_eq!(enc.to_string(), "TRUE");

        let mut enc = AvnEncoder::new();
        enc.encode_bool(Tag::BOOL, false, Identifier::EMPTY)
            .unwrap();
        assert_eq!(enc.to_string(), "FALSE");
    }

    #[test]
    fn null_encodes_correctly() {
        let mut enc = AvnEncoder::new();
        enc.encode_null(Tag::NULL, Identifier::EMPTY).unwrap();
        assert_eq!(enc.to_string(), "NULL");
    }

    #[test]
    fn integer_encodes_correctly() {
        let mut enc = AvnEncoder::new();
        enc.encode_integer(
            Tag::INTEGER,
            Constraints::default(),
            &42_i64,
            Identifier::EMPTY,
        )
        .unwrap();
        assert_eq!(enc.to_string(), "42");
    }

    #[test]
    fn octet_string_encodes_correctly() {
        let mut enc = AvnEncoder::new();
        enc.encode_octet_string(
            Tag::OCTET_STRING,
            Constraints::default(),
            &[0x01, 0xFF, 0xAB],
            Identifier::EMPTY,
        )
        .unwrap();
        assert_eq!(enc.to_string(), "'01FFAB'H");
    }
}
