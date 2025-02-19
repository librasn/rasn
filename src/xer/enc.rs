//! # Encoding XER.
use core::{borrow::Borrow, fmt::Write, ops::Deref};

use crate::{
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    types::{
        fields::Fields, variants::Variants, Any, BitStr, Enumerated, GeneralizedTime, UtcTime,
    },
    xer::{
        BIT_STRING_TYPE_TAG, BMP_STRING_TYPE_TAG, GENERALIZED_TIME_TYPE_TAG,
        GENERAL_STRING_TYPE_TAG, IA5_STRING_TYPE_TAG, INTEGER_TYPE_TAG, NULL_TYPE_TAG,
        NUMERIC_STRING_TYPE_TAG, OBJECT_IDENTIFIER_TYPE_TAG, OCTET_STRING_TYPE_TAG,
        PRINTABLE_STRING_TYPE_TAG, UTC_TIME_TYPE_TAG, UTF8_STRING_TYPE_TAG,
        VISIBLE_STRING_TYPE_TAG,
    },
};
use alloc::borrow::Cow;
use num_bigint::BigInt;
use xml_no_std::{
    attribute::Attribute, name::Name, namespace::Namespace, writer::XmlEvent, EventWriter,
    ParserConfig,
};

use crate::error::{EncodeError, XerEncodeErrorKind};

use super::{BOOLEAN_FALSE_TAG, BOOLEAN_TRUE_TAG, BOOLEAN_TYPE_TAG};

macro_rules! wrap_in_tags {
    ($this:ident, $tag:expr, $inner:ident, $($args:expr)*) => {{
        let xml_tag = $this.field_tag_stack.pop().unwrap_or($tag);
        $this.write_start_element(&xml_tag)?;
        $this.$inner($($args),*)?;
        $this.write_end_element(&xml_tag)
    }};
}

macro_rules! try_wrap_in_tags {
    ($this:ident, $inner:ident, $($args:expr)*) => {{
        let xml_tag = $this.field_tag_stack
            .pop()
            .ok_or_else(|| XerEncodeErrorKind::FieldName)?;
        $this.write_start_element(&xml_tag)?;
        $this.$inner($($args),*)?;
        $this.write_end_element(&xml_tag)
    }};
}

pub struct Encoder {
    field_tag_stack: Vec<Cow<'static, str>>,
    writer: EventWriter,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            writer: xml_no_std::EmitterConfig::new()
                .write_document_declaration(false)
                .create_writer(),
            field_tag_stack: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.writer.into_inner().into_bytes()
    }

    fn write(&mut self, event: XmlEvent<'_>) -> Result<(), EncodeError> {
        self.writer.write(event).map_err(|e| {
            EncodeError::from(XerEncodeErrorKind::XmlEncodingError {
                upstream: e.to_string(),
            })
        })
    }

    fn write_start_element<S: AsRef<str>>(&mut self, value: S) -> Result<(), EncodeError> {
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
}

impl crate::Encoder for Encoder {
    type Ok = ();

    type Error = EncodeError;

    fn codec(&self) -> crate::Codec {
        crate::Codec::Xer
    }

    fn encode_any(
        &mut self,
        __tag: crate::Tag,
        value: &crate::types::Any,
    ) -> Result<Self::Ok, Self::Error> {
        try_wrap_in_tags!(self, write_any, value)
    }

    fn encode_bool(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: bool,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(BOOLEAN_TYPE_TAG)),
            write_bool,
            value
        )
    }

    fn encode_bit_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::BitStr,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(BIT_STRING_TYPE_TAG)),
            write_bitstring,
            value
        )
    }

    fn encode_enumerated<E: crate::types::Enumerated>(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(
                identifier
                    .or(E::IDENTIFIER)
                    .ok_or(XerEncodeErrorKind::MissingIdentifier)?
            ),
            write_enumerated,
            value
        )
    }

    fn encode_object_identifier(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(OBJECT_IDENTIFIER_TYPE_TAG)),
            write_object_identifier,
            value
        )
    }

    fn encode_integer(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(INTEGER_TYPE_TAG)),
            write_integer,
            value
        )
    }

    fn encode_null(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(NULL_TYPE_TAG)),
            write_null,
        )
    }

    fn encode_octet_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(OCTET_STRING_TYPE_TAG)),
            write_octet_string,
            value
        )
    }

    fn encode_general_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(GENERAL_STRING_TYPE_TAG)),
            write_string_type,
            &String::from_utf8(value.deref().to_vec()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_utf8_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(UTF8_STRING_TYPE_TAG)),
            write_string_type,
            value
        )
    }

    fn encode_visible_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(VISIBLE_STRING_TYPE_TAG)),
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_ia5_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(IA5_STRING_TYPE_TAG)),
            write_string_type,
            &value.to_string()
        )
    }

    fn encode_printable_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(PRINTABLE_STRING_TYPE_TAG)),
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
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(NUMERIC_STRING_TYPE_TAG)),
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
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        _identifier: Option<&'static str>,
        _value: &crate::types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
        value: &crate::types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(BMP_STRING_TYPE_TAG)),
            write_string_type,
            &String::from_utf8(value.to_bytes()).map_err(|e| {
                XerEncodeErrorKind::XmlEncodingError {
                    upstream: e.to_string(),
                }
            })?
        )
    }

    fn encode_generalized_time(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: &crate::types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(GENERALIZED_TIME_TYPE_TAG)),
            write_generalized_time,
            value
        )
    }

    fn encode_utc_time(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: &crate::types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        wrap_in_tags!(
            self,
            Cow::Borrowed(identifier.unwrap_or(UTC_TIME_TYPE_TAG)),
            write_utc_time,
            value
        )
    }

    fn encode_explicit_prefix<V: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self, identifier)
    }

    fn encode_sequence<C, F>(
        &mut self,
        _tag: crate::Tag,
        identifier: Option<&'static str>,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        let xml_tag = self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
            identifier
                .or(C::IDENTIFIER)
                .ok_or(XerEncodeErrorKind::MissingIdentifier)?,
        ));
        self.write_start_element(&xml_tag)?;

        let mut ids = C::FIELDS
            .identifiers()
            .chain(C::EXTENDED_FIELDS.unwrap_or(Fields::empty()).identifiers())
            .map(|id| Cow::<str>::Owned(id.to_string()))
            .collect::<Vec<_>>();
        ids.reverse();
        ids.into_iter().for_each(|id| self.field_tag_stack.push(id));
        encoder_scope(self)?;

        self.write_end_element(xml_tag)
    }

    fn encode_sequence_of<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        value: &[E],
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
            identifier.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
        ));
        self.write_start_element(&xml_tag)?;
        for elem in value {
            elem.encode(self, E::IDENTIFIER)?;
        }
        self.write_end_element(xml_tag)
    }

    fn encode_set<C, F>(
        &mut self,
        _tag: crate::Tag,
        value: F,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        let xml_tag = self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
            identifier
                .or(C::IDENTIFIER)
                .ok_or(XerEncodeErrorKind::MissingIdentifier)?,
        ));
        self.write_start_element(&xml_tag)?;

        let mut ids = C::FIELDS
            .identifiers()
            .chain(C::EXTENDED_FIELDS.unwrap_or(Fields::empty()).identifiers())
            .map(|id| Cow::<str>::Owned(id.to_string()))
            .collect::<Vec<_>>();
        ids.reverse();
        ids.into_iter().for_each(|id| self.field_tag_stack.push(id));
        value(self)?;

        self.write_end_element(xml_tag)
    }

    fn encode_set_of<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        value: &crate::types::SetOf<E>,
        _constraints: crate::types::Constraints,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
            identifier.ok_or(XerEncodeErrorKind::MissingIdentifier)?,
        ));
        self.write_start_element(&xml_tag)?;
        for elem in value {
            elem.encode(self, E::IDENTIFIER)?;
        }
        self.write_end_element(xml_tag)
    }

    fn encode_some<E: crate::Encode>(
        &mut self,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self, identifier)
    }

    fn encode_some_with_tag_and_constraints<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: &E,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_some(value, identifier)
    }

    fn encode_none<E: crate::Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.field_tag_stack.pop();
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _tag: crate::Tag) -> Result<Self::Ok, Self::Error> {
        self.field_tag_stack.pop();
        Ok(())
    }

    fn encode_choice<E: crate::Encode + crate::types::Choice>(
        &mut self,
        _constraints: crate::types::Constraints,
        tag: crate::Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<crate::Tag, Self::Error>,
        identifier: Option<&'static str>,
    ) -> Result<Self::Ok, Self::Error> {
        let xml_tag = self.field_tag_stack.pop().unwrap_or(Cow::Borrowed(
            identifier
                .or(E::IDENTIFIER)
                .ok_or(XerEncodeErrorKind::MissingIdentifier)?,
        ));
        self.write_start_element(&xml_tag)?;

        let variants =
            Variants::from_slice(&[E::VARIANTS, E::EXTENDED_VARIANTS.unwrap_or(&[])].concat());

        let identifier = variants
            .iter()
            .enumerate()
            .find_map(|(i, &variant_tag)| {
                (tag == variant_tag)
                    .then_some(E::IDENTIFIERS.get(i))
                    .flatten()
            })
            .ok_or_else(|| crate::error::EncodeError::variant_not_in_choice(self.codec()))?;

        self.write_start_element(identifier)?;
        encode_fn(self)?;
        self.write_end_element(identifier)?;

        self.write_end_element(&xml_tag)
    }

    fn encode_extension_addition<E: crate::Encode>(
        &mut self,
        _tag: crate::Tag,
        _constraints: crate::types::Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self, None)
    }

    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: crate::Encode + crate::types::Constructed,
    {
        match value {
            Some(v) => v.encode(self, None),
            None => self.encode_none::<E>(),
        }
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

    fn write_object_identifier(&mut self, value: &[u32]) -> Result<(), EncodeError> {
        self.write(XmlEvent::Characters(
            &value
                .iter()
                .map(|arc| arc.to_string())
                .collect::<Vec<String>>()
                .join("."),
        ))
    }

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
        let mut reader = ParserConfig::default().create_reader(value.contents.iter());
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
                xml_no_std::reader::XmlEvent::Whitespace(_) => {
                    continue;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bitvec::bitvec;
    use bitvec::order::Msb0;

    use crate::prelude::*;
    use crate::xer::encode;

    #[derive(AsnType, Debug, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    struct NestedTestA {
        wine: bool,
        grappa: OctetString,
        inner: InnerTestA,
        #[rasn(extension_addition)]
        oid: Option<ObjectIdentifier>,
    }

    #[derive(AsnType, Debug, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct InnerTestA {
        hidden: Option<bool>,
    }

    #[test]
    fn encodes_nested_extensible_sequence() {
        assert_eq!(
            String::from_utf8(encode(&NestedTestA {
                wine: true,
                grappa: vec![0, 1, 2, 3].into(),
                inner: InnerTestA { hidden: Some(false) },
                oid: Some(ObjectIdentifier::from(Oid::const_new(&[1, 8270, 4, 1])))
            })
            .unwrap()).unwrap(),
            "<NestedTestA><wine><true /></wine><grappa>00010203</grappa><inner><hidden><false /></hidden></inner><oid>1.8270.4.1</oid></NestedTestA>"
        );
        assert_eq!(
            String::from_utf8(encode(&NestedTestA {
                wine: true,
                grappa: vec![0, 1, 2, 3].into(),
                inner: InnerTestA { hidden: None },
                oid: Some(ObjectIdentifier::from(Oid::const_new(&[1, 8270, 4, 1])))
            })
            .unwrap()).unwrap(),
            "<NestedTestA><wine><true /></wine><grappa>00010203</grappa><inner /><oid>1.8270.4.1</oid></NestedTestA>"
        );
        assert_eq!(
            String::from_utf8(
                encode(&NestedTestA {
                    wine: true,
                    grappa: vec![0, 1, 2, 3].into(),
                    inner: InnerTestA { hidden: None },
                    oid: None
                })
                .unwrap()
            )
            .unwrap(),
            "<NestedTestA><wine><true /></wine><grappa>00010203</grappa><inner /></NestedTestA>"
        )
    }

    macro_rules! basic_types {
        ($test_name:ident, $type:ident, $value:expr, $expected_ty:literal, $expected_val:literal) => {
            #[test]
            fn $test_name() {
                #[derive(AsnType, Debug, Encode, PartialEq)]
                #[rasn(automatic_tags, delegate)]
                #[rasn(crate_root = "crate")]
                struct DelegateType(pub $type);

                #[derive(AsnType, Debug, Encode, PartialEq)]
                #[rasn(automatic_tags, delegate)]
                #[rasn(crate_root = "crate")]
                struct NestedDelegateType(pub DelegateType);

                #[derive(AsnType, Debug, Encode, PartialEq)]
                #[rasn(automatic_tags, delegate, identifier = "Alias")]
                #[rasn(crate_root = "crate")]
                struct AliasDelegateType(pub $type);

                assert_eq!(
                    String::from_utf8(encode(&$value).unwrap()).unwrap(),
                    String::from("<")
                        + $expected_ty
                        + ">"
                        + $expected_val
                        + "</"
                        + $expected_ty
                        + ">"
                );
                assert_eq!(
                    String::from_utf8(encode(&DelegateType($value)).unwrap()).unwrap(),
                    String::from("<DelegateType>") + $expected_val + "</DelegateType>"
                );
                assert_eq!(
                    String::from_utf8(encode(&NestedDelegateType(DelegateType($value))).unwrap())
                        .unwrap(),
                    String::from("<NestedDelegateType>") + $expected_val + "</NestedDelegateType>"
                );
                assert_eq!(
                    String::from_utf8(encode(&AliasDelegateType($value)).unwrap()).unwrap(),
                    String::from("<Alias>") + $expected_val + "</Alias>"
                );
            }
        };
    }

    #[derive(AsnType, Debug, Encode, PartialEq, Copy, Clone)]
    #[rasn(enumerated, automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum EnumType {
        #[rasn(identifier = "eins")]
        First,
        #[rasn(identifier = "zwei")]
        Second,
    }

    #[derive(AsnType, Debug, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum ChoiceType {
        #[rasn(identifier = "enum")]
        EnumVariant(EnumType),
        nested(InnerTestA),
    }

    basic_types!(boolean_true, bool, true, "BOOLEAN", "<true />");
    basic_types!(boolean_false, bool, false, "BOOLEAN", "<false />");
    basic_types!(integer_sml, Integer, Integer::from(1), "INTEGER", "1");
    basic_types!(integer_neg, Integer, Integer::from(-2), "INTEGER", "-2");
    basic_types!(integer_u8, u8, 212, "INTEGER", "212");
    basic_types!(
        integer_i64,
        i64,
        -2141247653269i64,
        "INTEGER",
        "-2141247653269"
    );
    basic_types!(
        bit_string,
        BitString,
        bitvec![u8, Msb0; 1,0,1,1,0,0,1],
        "BIT_STRING",
        "1011001"
    );
    basic_types!(
        octet_string,
        OctetString,
        OctetString::from([255u8, 0, 8, 10].to_vec()),
        "OCTET_STRING",
        "FF00080A"
    );
    basic_types!(
        ia5_string,
        Ia5String,
        Ia5String::from_iso646_bytes(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35]).unwrap(),
        "IA5String",
        "012345"
    );
    basic_types!(
        numeric_string,
        NumericString,
        NumericString::from_bytes(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35]).unwrap(),
        "NumericString",
        "012345"
    );
    basic_types!(
        utf8_string,
        Utf8String,
        "012345".to_string(),
        "UTF8String",
        "012345"
    );
    basic_types!(
        object_identifier,
        ObjectIdentifier,
        ObjectIdentifier::from(Oid::const_new(&[1, 654, 2, 1])),
        "OBJECT_IDENTIFIER",
        "1.654.2.1"
    );
    basic_types!(
        sequence,
        InnerTestA,
        InnerTestA {
            hidden: Some(false)
        },
        "InnerTestA",
        "<hidden><false /></hidden>"
    );
    basic_types!(
        enumerated,
        EnumType,
        EnumType::First,
        "EnumType",
        "<eins />"
    );
    basic_types!(
        choice,
        ChoiceType,
        ChoiceType::nested(InnerTestA { hidden: None }),
        "ChoiceType",
        "<nested><InnerTestA /></nested>"
    );
}
