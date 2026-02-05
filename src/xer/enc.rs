//! # Encoding XER.
use core::{fmt::Write, ops::Deref};

use crate::{
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    types::{
        fields::Fields, Any, BitStr, BmpString, Constraints, Date, Enumerated, GeneralString,
        GeneralizedTime, GraphicString, Ia5String, Identifier, IntegerType, NumericString,
        OctetString, Oid, PrintableString, RealType, SetOf, Tag, TeletexString, UtcTime,
        VisibleString,
    },
    AsnType,
};
use alloc::borrow::Cow;
use num_bigint::BigInt;
use xml_no_std::{
    attribute::Attribute, name::Name, namespace::Namespace, writer::XmlEvent, EventWriter,
    ParserConfig,
};

use crate::error::{EncodeError, XerEncodeErrorKind};

use super::{BOOLEAN_FALSE_TAG, BOOLEAN_TRUE_TAG, MINUS_INFINITY_TAG, NAN_TAG, PLUS_INFINITY_TAG};

macro_rules! wrap_in_tags {
    ($this:ident, $tag:expr, $inner:ident, $($args:expr)*) => {{
        let xml_tag = match $this.entering_list_item_type {
            true => $tag,
            false => $this.field_tag_stack.pop().unwrap_or($tag),
        };
        $this.write_start_element(&xml_tag)?;
        $this.$inner($($args),*)?;
        $this.write_end_element(&xml_tag)
    }};
}

macro_rules! try_wrap_in_tags {
    ($this:ident, $inner:ident, $($args:expr)*) => {{
        let xml_tag = $this.field_tag_stack
            .pop()
            .ok_or_else(|| XerEncodeErrorKind::MissingIdentifier)?;
        $this.write_start_element(&xml_tag)?;
        $this.$inner($($args),*)?;
        $this.write_end_element(&xml_tag)
    }};
}

/// Encoder for creating ASN.1 encodings using XML encoding rules (XER).
pub struct Encoder {
    field_tag_stack: Vec<Cow<'static, str>>,
    writer: EventWriter,
    end_index_of_first_tag: Option<usize>,
    start_index_of_last_tag: usize,
    entering_choice_value: bool,
    entering_list_item_type: bool,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    /// Creates a new XER encoder instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            writer: xml_no_std::EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(),
            field_tag_stack: Vec::new(),
            end_index_of_first_tag: None,
            start_index_of_last_tag: 0,
            entering_choice_value: false,
            entering_list_item_type: false,
        }
    }

    /// Returns the encoded XER value as UTF-8 bytes
    #[must_use]
    pub fn finish(self) -> Vec<u8> {
        self.writer.into_inner().into_bytes()
    }

    fn append(&mut self, other: &mut Encoder) {
        self.writer.inner_mut().push_str(other.writer.inner_mut());
    }

    fn write(&mut self, event: XmlEvent<'_>) -> Result<(), EncodeError> {
        self.start_index_of_last_tag = self.writer.inner_mut().len();
        self.writer.write(event).map_err(|e| {
            EncodeError::from(XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })
        })?;
        if self.end_index_of_first_tag.is_none() {
            self.end_index_of_first_tag = Some(self.writer.inner_mut().len());
        }
        Ok(())
    }

    fn write_start_element<S: AsRef<str>>(&mut self, value: S) -> Result<(), EncodeError> {
        if self.entering_choice_value {
            self.entering_choice_value = false;
        } else if self.entering_list_item_type {
            self.entering_list_item_type = false;
        }
        self.write(XmlEvent::StartElement {
            name: Name::local(value.as_ref()),
            attributes: Cow::Borrowed(&[]),
            namespace: Namespace::empty().borrow(),
        })
    }

    fn write_end_element<S: AsRef<str>>(&mut self, value: S) -> Result<(), EncodeError> {
        self.write(XmlEvent::EndElement {
            name: Some(Name::local(value.as_ref())),
        })
    }

    fn write_empty(&mut self) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(""))
    }

    fn erase_outer_tags(&mut self) {
        if let Some(end_index) = self.end_index_of_first_tag {
            let inner = self.writer.inner_mut();
            inner.drain(self.start_index_of_last_tag..);
            inner.drain(..=end_index);
        }
    }

    fn entering_choice_value(&mut self) {
        self.entering_choice_value = true;
    }

    fn set_entering_list_item_type(&mut self, value: bool) {
        self.entering_list_item_type = value;
    }
}

impl crate::Encoder<'_> for Encoder {
    type Ok = ();

    type Error = EncodeError;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder;

    fn codec(&self) -> crate::Codec {
        crate::Codec::Xer
    }

    fn encode_any(
        &mut self,
        __tag: Tag,
        value: &Any,
        _identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        try_wrap_in_tags!(self, write_any, value)
    }

    fn encode_bool(
        &mut self,
        _tag: Tag,
        value: bool,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if self.entering_list_item_type {
            self.write_bool(value)
        } else {
            wrap_in_tags!(
                self,
                Cow::Borrowed(identifier.or(bool::IDENTIFIER).unwrap()),
                write_bool,
                value
            )
        }
    }

    fn encode_bit_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &BitStr,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(BitStr::IDENTIFIER).unwrap()),
            write_bitstring,
            value
        )
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        _tag: Tag,
        value: &E,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if self.entering_list_item_type {
            self.write_enumerated(value)
        } else {
            wrap_in_tags!(
                self,
                Cow::Borrowed(
                    identifier
                        .or(E::IDENTIFIER)
                        .0
                        .ok_or(XerEncodeErrorKind::MissingIdentifier)?
                ),
                write_enumerated,
                value
            )
        }
    }

    fn encode_object_identifier(
        &mut self,
        _tag: Tag,
        value: &[u32],
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(Oid::IDENTIFIER).unwrap()),
            write_object_identifier,
            value
        )
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &I,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if let Some(as_bigint) = value.to_bigint() {
            wrap_in_tags!(
                self,
                Cow::Borrowed(identifier.or(u8::IDENTIFIER).unwrap()),
                write_integer,
                &as_bigint
            )
        } else {
            Err(XerEncodeErrorKind::UnsupportedIntegerValue.into())
        }
    }

    fn encode_null(&mut self, _tag: Tag, identifier: Identifier) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(<()>::IDENTIFIER).unwrap()),
            write_null,
        )
    }

    fn encode_octet_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &[u8],
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(OctetString::IDENTIFIER).unwrap()),
            write_octet_string,
            value
        )
    }

    fn encode_general_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &GeneralString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(GeneralString::IDENTIFIER).unwrap()),
            write_string_type,
            &String::from_utf8(value.deref().clone()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_utf8_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &str,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(str::IDENTIFIER).unwrap()),
            write_string_type,
            value
        )
    }

    fn encode_visible_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &VisibleString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(VisibleString::IDENTIFIER).unwrap()),
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_ia5_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &Ia5String,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(Ia5String::IDENTIFIER).unwrap()),
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_printable_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &PrintableString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(PrintableString::IDENTIFIER).unwrap()),
            write_string_type,
            &String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_numeric_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &NumericString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(NumericString::IDENTIFIER).unwrap()),
            write_string_type,
            &String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_teletex_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        _value: &TeletexString,
        _identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &BmpString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(BmpString::IDENTIFIER).unwrap()),
            write_string_type,
            &String::from_utf8(value.to_bytes()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_graphic_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &GraphicString,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(GraphicString::IDENTIFIER).unwrap()),
            write_string_type,
            &String::from_utf8(value.as_bytes().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_generalized_time(
        &mut self,
        _tag: Tag,
        value: &GeneralizedTime,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(GeneralizedTime::IDENTIFIER).unwrap()),
            write_generalized_time,
            value
        )
    }

    fn encode_utc_time(
        &mut self,
        _tag: Tag,
        value: &UtcTime,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(UtcTime::IDENTIFIER).unwrap()),
            write_utc_time,
            value
        )
    }

    fn encode_explicit_prefix<V: crate::Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if identifier.0.is_none() {
            return value.encode(self);
        }

        if tag != Tag::EOC {
            return value.encode_with_tag_and_constraints(self, V::TAG, V::CONSTRAINTS, identifier);
        }

        // Read current xml tag
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };

        if self.entering_list_item_type {
            // List items that are `CHOICE` delegate types are encoded without their outer tags
            // We use a new encoder to write the inner choice value of the delegate: <ChoiceType><option /></ChoiceType>
            let mut inner_encoder = Self::new();
            // Then we write an empty string to prompt the XML writer to close any uncloses start tags.
            self.write_empty()?;
            value.encode(&mut inner_encoder)?;
            // We then remove the outer tag pair: <option />
            inner_encoder.erase_outer_tags();
            // ... and append the output of the inner encoder: <option />
            self.append(&mut inner_encoder);
            Ok(())
        } else {
            // Special handling is needed for `CHOICE` delegate types
            // First we write the start tag of the delegate: <Delegate
            self.write_start_element(&xml_tag)?;
            // Then we write an empty string to prompt the XML writer to close the start tag: <Delegate>
            self.write_empty()?;
            // We use a new encoder to write the inner choice value: <ChoiceType><option /></ChoiceType>
            let mut inner_encoder = Self::new();
            value.encode(&mut inner_encoder)?;
            // We then remove the outer tag pair: <option />
            inner_encoder.erase_outer_tags();
            // ...append the output of the inner encoder: <Delegate><option />
            self.append(&mut inner_encoder);
            // ...and finally close the delegate: <Delegate><option /></Delegate>
            self.write_end_element(&xml_tag)
        }
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        __t: Tag,
        encoder_scope: F,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };
        self.write_start_element(&xml_tag)?;

        let mut ids = C::FIELDS
            .identifiers()
            .chain(
                C::EXTENDED_FIELDS
                    .as_ref()
                    .map(Fields::identifiers)
                    .into_iter()
                    .flatten(),
            )
            .map(|id| Cow::<str>::Owned(id.to_string()))
            .collect::<Vec<_>>();
        ids.reverse();
        ids.into_iter().for_each(|id| self.field_tag_stack.push(id));
        encoder_scope(self)?;

        self.write_end_element(xml_tag)
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _tag: Tag,
        value: &[E],
        _constraints: Constraints,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };
        self.write_start_element(&xml_tag)?;
        for elem in value {
            self.set_entering_list_item_type(true);
            elem.encode(self)?;
        }
        self.write_end_element(xml_tag)
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        _tag: Tag,
        value: F,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };
        self.write_start_element(&xml_tag)?;

        let mut ids = C::FIELDS
            .identifiers()
            .chain(
                C::EXTENDED_FIELDS
                    .as_ref()
                    .map(Fields::identifiers)
                    .into_iter()
                    .flatten(),
            )
            .map(|id| Cow::<str>::Owned(id.to_string()))
            .collect::<Vec<_>>();
        ids.reverse();
        ids.into_iter().for_each(|id| self.field_tag_stack.push(id));
        value(self)?;

        self.write_end_element(xml_tag)
    }

    fn encode_set_of<E: crate::Encode + Eq + core::hash::Hash>(
        &mut self,
        _tag: Tag,
        value: &SetOf<E>,
        _constraints: Constraints,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };
        self.write_start_element(&xml_tag)?;
        for elem in value.to_vec() {
            self.set_entering_list_item_type(true);
            elem.encode(self)?;
        }
        self.write_end_element(xml_tag)
    }

    fn encode_some<E: crate::Encode>(
        &mut self,
        value: &E,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag_and_constraints(self, E::TAG, E::CONSTRAINTS, identifier)
    }

    fn encode_some_with_tag_and_constraints<E: crate::Encode>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &E,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_some(value, identifier)
    }

    fn encode_none<E: crate::Encode>(
        &mut self,
        _identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.field_tag_stack.pop();
        Ok(())
    }

    fn encode_none_with_tag(
        &mut self,
        _tag: Tag,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if let Some(id) = self.entering_choice_value.then_some(()).and(identifier.0) {
            self.write_start_element(id)?;
            self.write_end_element(id)?;
        }
        self.field_tag_stack.pop();
        Ok(())
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _c: crate::types::Constraints,
        _tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = match self.entering_list_item_type {
            true => Cow::Borrowed(identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?),
            false => self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
                identifier.0.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
            )),
        };
        if self.entering_list_item_type {
            self.set_entering_list_item_type(false);
            encode_fn(self)?;
            Ok(())
        } else {
            self.write_start_element(&xml_tag)?;
            self.entering_choice_value();
            encode_fn(self)?;
            self.write_end_element(&xml_tag)
        }
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _t: Tag,
        _c: crate::types::Constraints,
        value: E,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag_and_constraints(self, E::TAG, E::CONSTRAINTS, identifier)
    }

    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        _tag: Tag,
        value: Option<&E>,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: crate::Encode + crate::types::Constructed<RL, EL>,
    {
        match value {
            Some(v) => v.encode_with_tag_and_constraints(self, E::TAG, E::CONSTRAINTS, identifier),
            None => self.encode_none::<E>(identifier),
        }
    }

    fn encode_real<R: crate::prelude::RealType>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
        value: &R,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(Identifier::REAL).unwrap()),
            write_real,
            value
        )
    }

    fn encode_date(
        &mut self,
        _tag: Tag,
        value: &crate::types::Date,
        identifier: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.or(Date::IDENTIFIER).unwrap()),
            write_date,
            value
        )
    }
}

impl Encoder {
    fn write_bool(&mut self, value: bool) -> Result<(), EncodeError> {
        if value {
            self.write_start_element(BOOLEAN_TRUE_TAG)?;
            self.write_end_element(BOOLEAN_TRUE_TAG)
        } else {
            self.write_start_element(BOOLEAN_FALSE_TAG)?;
            self.write_end_element(BOOLEAN_FALSE_TAG)
        }
    }

    fn write_date(&mut self, value: &Date) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(&value.format("%Y%m%d").to_string()))
    }

    fn write_bitstring(&mut self, value: &BitStr) -> Result<(), EncodeError> {
        if value.is_empty() {
            Ok(())
        } else {
            self.write(XmlEvent::Characters(
                value
                    .iter()
                    .map(|bit| if *bit { '1' } else { '0' })
                    .collect::<String>()
                    .as_str(),
            ))
        }
    }

    fn write_enumerated<E: Enumerated>(&mut self, value: &E) -> Result<(), EncodeError> {
        self.write_start_element(value.identifier())?;
        self.write_end_element(value.identifier())
    }

    fn write_integer(&mut self, value: &BigInt) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(&value.to_str_radix(10)))
    }

    fn write_real<R: RealType>(&mut self, value: &R) -> Result<(), EncodeError> {
        if value.is_infinity() {
            self.write_start_element(PLUS_INFINITY_TAG)?;
            self.write_end_element(PLUS_INFINITY_TAG)
        } else if value.is_neg_infinity() {
            self.write_start_element(MINUS_INFINITY_TAG)?;
            self.write_end_element(MINUS_INFINITY_TAG)
        } else if value.is_nan() {
            self.write_start_element(NAN_TAG)?;
            self.write_end_element(NAN_TAG)
        } else {
            self.write(XmlEvent::Characters(&value.to_string()))
        }
    }

    fn write_object_identifier(&mut self, value: &[u32]) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &value
                .iter()
                .map(|arc| arc.to_string())
                .collect::<Vec<String>>()
                .join("."),
        ))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn write_null(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    fn write_octet_string(&mut self, value: &[u8]) -> Result<(), EncodeError> {
        if value.is_empty() {
            Ok(())
        } else {
            self.write(XmlEvent::Characters(
                value
                    .iter()
                    .try_fold(String::new(), |mut acc, byte| {
                        write!(&mut acc, "{byte:02X?}").map(|_| acc)
                    })
                    .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                        upstream: e.to_string(),
                    })?
                    .as_str(),
            ))
        }
    }

    fn write_string_type(&mut self, value: &str) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(value))
    }

    fn write_generalized_time(&mut self, value: &GeneralizedTime) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
            )
            .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })?,
        ))
    }

    fn write_utc_time(&mut self, value: &UtcTime) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &String::from_utf8(
                crate::ber::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
            )
            .map_err(|e| XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })?,
        ))
    }

    fn write_any(&mut self, value: &Any) -> Result<(), EncodeError> {
        let mut reader = ParserConfig::default().create_reader(value.as_bytes().iter());
        while let Ok(evt) = reader.next() {
            match evt {
                xml_no_std::reader::XmlEvent::StartDocument { .. } => {
                    return Err(XerEncodeErrorKind::XmlEncodingError {
                        upstream: "Any-type values must not contain XML prolog!".to_string(),
                    }
                    .into())
                }
                xml_no_std::reader::XmlEvent::EndDocument => break,
                xml_no_std::reader::XmlEvent::ProcessingInstruction { name, data } => {
                    self.write(XmlEvent::ProcessingInstruction {
                        name: &name,
                        data: data.as_deref(),
                    })?;
                }
                xml_no_std::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    self.write(XmlEvent::StartElement {
                        name: name.borrow(),
                        namespace: namespace.borrow(),
                        attributes: attributes
                            .iter()
                            .map(|attr| Attribute::new(attr.name.borrow(), &attr.value))
                            .collect(),
                    })?;
                }
                xml_no_std::reader::XmlEvent::EndElement { name } => {
                    self.write(XmlEvent::EndElement {
                        name: Some(name.borrow()),
                    })?;
                }
                xml_no_std::reader::XmlEvent::CData(cdata) => {
                    self.write(XmlEvent::CData(&cdata))?;
                }
                xml_no_std::reader::XmlEvent::Comment(comment) => {
                    self.write(XmlEvent::Comment(&comment))?;
                }
                xml_no_std::reader::XmlEvent::Characters(characters) => {
                    self.write(XmlEvent::Characters(&characters))?;
                }
                xml_no_std::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        Ok(())
    }
}
