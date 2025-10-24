//! # Decoding XER
extern crate alloc;

use crate::alloc::string::ToString;
use core::{borrow::Borrow, f64};

use xml_no_std::{
    attribute::Attribute, common::XmlVersion, name::OwnedName, namespace::Namespace,
    reader::XmlEvent, ParserConfig,
};

use crate::{error::*, types::*, xer::BOOLEAN_TRUE_TAG, Decode};

use self::fields::Field;

use super::{
    BOOLEAN_FALSE_TAG, MINUS_INFINITY_TAG, MINUS_INFINITY_VALUE, NAN_TAG, NAN_VALUE,
    PLUS_INFINITY_TAG, PLUS_INFINITY_VALUE,
};

const OPTIONAL_ITEM_NOT_PRESENT: &str = "§_NOT_PRESENT_§";

macro_rules! error {
    ($kind:ident, $($arg:tt)*) => {
        DecodeError::from(XerDecodeErrorKind::$kind {
            details: alloc::format!($($arg)*)
        })
    };
    ($kind:ident) => {
        DecodeError::from(XerDecodeErrorKind::$kind { })
    };
}

macro_rules! tag {
    ($event:ident, $this:ident, $tag:expr) => {
        match $this.next_element() {
            Some(XmlEvent::$event { name, .. }) => {
                if name.local_name.as_str() == $tag {
                    Ok(())
                } else {
                    Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: $tag,
                        found: alloc::format!("{name:?}"),
                    }))
                }
            }
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: $tag,
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        }
    };
    ($event:ident, $this:ident) => {
        match $this.next_element() {
            Some(XmlEvent::$event { .. }) => Ok(()),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "StartElement or EndElement",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        }
    };
}

macro_rules! decode_string {
    ($this:ident, $tryfrom:path, $tag:path, $needed:literal) => {{
        tag!(StartElement, $this)?;
        let value = match $this.next_element() {
            Some(XmlEvent::Characters(value)) => $tryfrom(value).map_err(|e| {
                DecodeError::string_conversion_failed(
                    $tag,
                    alloc::format!("Error transforming string: {e:?}"),
                    crate::Codec::Xer,
                )
            }),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: $needed,
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, $this)?;
        value
    }};
}

macro_rules! decode_time {
    ($this:ident, $decode_fn:path) => {{
        tag!(StartElement, $this)?;
        let value = match $this.next_element() {
            Some(XmlEvent::Characters(value)) => $decode_fn(value),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "Time value",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, $this)?;
        value
    }};
}

macro_rules! value_or_empty {
    ($this:ident, $parser:ident, $expected:expr) => {{
        let value = match $this.peek() {
            Some(XmlEvent::Characters(s)) => $parser(s),
            Some(XmlEvent::EndElement { .. }) => return Ok(<_>::default()),
            Some(elem) => {
                return Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                    needed: $expected,
                    found: alloc::format!("{elem:?}"),
                }))
            }
            _ => return Err(DecodeError::from(XerDecodeErrorKind::EndOfXmlInput {})),
        };
        let _ = $this.next_element();
        value
    }};
}

#[derive(Debug)]
struct XerElement {
    events: alloc::collections::VecDeque<XmlEvent>,
}

impl XerElement {
    pub fn next(&mut self) -> Option<XmlEvent> {
        self.events.pop_front()
    }

    pub fn peek(&self) -> Option<&XmlEvent> {
        self.events.front()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

impl<I: IntoIterator<Item = XmlEvent>> From<I> for XerElement {
    fn from(value: I) -> Self {
        XerElement {
            events: value.into_iter().collect(),
        }
    }
}

/// Decoder for decoding XER-conforming ASN.1 data
pub struct Decoder {
    stack: alloc::vec::Vec<XerElement>,
    in_list: bool,
}

impl Decoder {
    /// Creates a new Decoder from the given input
    pub fn new(input: &[u8]) -> Result<Self, <Decoder as crate::de::Decoder>::Error> {
        let mut reader = ParserConfig::default().create_reader(input.iter());
        let next = reader.next().map_err(|e| error!(XmlParser, "{e:?}"))?;
        check_prolog(&next)?;
        let mut elements = alloc::collections::VecDeque::new();
        'read_xml: loop {
            let next = reader.next().map_err(|e| error!(XmlParser, "{e:?}"))?;
            if next == XmlEvent::EndDocument {
                break 'read_xml;
            }
            elements.push_back(next);
        }
        elements.try_into()
    }

    fn next_element(&mut self) -> Option<XmlEvent> {
        if let Some(mut elem) = self.stack.pop() {
            let event = elem.next();
            if !elem.events.is_empty() {
                self.stack.push(elem);
            }
            event
        } else {
            None
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.stack.iter().fold(0, |acc, evt| acc + evt.len())
    }

    fn peek(&self) -> Option<&XmlEvent> {
        self.stack.last().and_then(XerElement::peek)
    }

    fn sort_by_field_tag_order(
        &mut self,
        field_indices: &[(usize, Field)],
    ) -> Result<(), DecodeError> {
        let field_names = field_indices.iter().map(|(_, f)| f.name).collect();
        self.sort_by_field_name_order(field_names)
    }

    fn sort_by_field_name_order(
        &mut self,
        field_names: alloc::vec::Vec<&str>,
    ) -> Result<(), DecodeError> {
        let stack = core::mem::take(&mut self.stack);
        let mut reordered = stack.into_iter().try_fold(
            alloc::collections::BTreeMap::<usize, XerElement>::new(),
            |mut acc, elem| {
                let index = match elem.peek() {
                    Some(XmlEvent::StartElement { name, .. }) => field_names
                        .iter()
                        .enumerate()
                        .find_map(|(i, f)| (*f == name.local_name.as_str()).then_some(i))
                        .ok_or_else(|| {
                            XerDecodeErrorKind::XmlTag {
                                needed: name.local_name.clone(),
                                found: "nothing".into(),
                            }
                            .into()
                        }),
                    e => Err(error!(XmlParser, "Expected opening tag, found {e:?}")),
                };
                index.map(|i| {
                    acc.insert(i, elem);
                    acc
                })
            },
        )?;
        for i in (0..field_names.len()).rev() {
            self.stack.push(reordered.remove(&i).unwrap_or(XerElement {
                events: alloc::vec![XmlEvent::Characters(OPTIONAL_ITEM_NOT_PRESENT.into())].into(),
            }));
        }
        Ok(())
    }

    fn into_list_decoder(mut self) -> Self {
        self.in_list = true;
        self
    }

    fn from_stack_elems<E: IntoIterator<Item = XmlEvent>, I: IntoIterator<Item = E>>(
        elems: I,
    ) -> Self {
        Decoder {
            stack: elems.into_iter().map(|i| XerElement::from(i)).collect(),
            in_list: false,
        }
    }
}

impl TryFrom<alloc::collections::VecDeque<XmlEvent>> for Decoder {
    type Error = DecodeError;
    fn try_from(value: alloc::collections::VecDeque<XmlEvent>) -> Result<Self, Self::Error> {
        let (mut stack, mut events, mut tag) =
            (alloc::vec![], alloc::collections::VecDeque::new(), None);
        let mut level_of_nested_items = 0;
        'xml_elements: for evt in value {
            match (&tag, evt) {
                (_, XmlEvent::Whitespace(_)) => continue 'xml_elements,
                (
                    None,
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    },
                ) => {
                    tag = Some(name.clone());
                    events.push_back(XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    });
                }
                (None, _) => {
                    continue 'xml_elements;
                }
                (
                    Some(t),
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    },
                ) => {
                    if &name == t {
                        level_of_nested_items += 1;
                    }
                    events.push_back(XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    });
                }
                (Some(t), XmlEvent::EndElement { name }) => {
                    if &name == t && level_of_nested_items != 0 {
                        level_of_nested_items -= 1;
                        events.push_back(XmlEvent::EndElement { name });
                    } else if &name == t {
                        events.push_back(XmlEvent::EndElement { name });
                        let collected_events: alloc::collections::VecDeque<XmlEvent> =
                            core::mem::take(&mut events);
                        stack.push(XerElement {
                            events: collected_events,
                        });
                        tag = None;
                    } else {
                        events.push_back(XmlEvent::EndElement { name });
                    }
                }
                (Some(_), XmlEvent::EndDocument) => return Err(error!(EndOfXmlInput)),
                (Some(_), event) => events.push_back(event),
            }
        }
        Ok(Self {
            stack,
            in_list: false,
        })
    }
}

fn check_prolog(prolog: &XmlEvent) -> Result<(), DecodeError> {
    if let XmlEvent::StartDocument {
        version, encoding, ..
    } = prolog
    {
        if version != &XmlVersion::Version10 || encoding != &alloc::string::String::from("UTF-8") {
            Err(error!(
                SpecViolation,
                r#"§8.2 The XML prolog shall either be empty; or shall consist of [...] <?xml
                version="1.0"
                encoding="UTF-8"?>"#
            ))
        } else {
            Ok(())
        }
    } else {
        Err(error!(XmlParser, "Expected XML prolog, found {:?}", prolog))
    }
}

impl crate::Decoder for Decoder {
    type Ok = ();

    type AnyDecoder<const R: usize, const E: usize> = Decoder;
    type Error = DecodeError;

    fn codec(&self) -> crate::Codec {
        crate::Codec::Xer
    }

    fn decode_any(&mut self, _tag: Tag) -> Result<crate::types::Any, Self::Error> {
        tag!(StartElement, self)?;
        let mut events = self
            .stack
            .pop()
            .ok_or_else(|| error!(EndOfXmlInput))?
            .events;
        events.pop_back();
        let mut xml_writer = xml_no_std::EmitterConfig::new()
            .write_document_declaration(false)
            .create_writer();

        for reader_event in events {
            match reader_event {
                XmlEvent::EndDocument => return Err(XerDecodeErrorKind::EndOfXmlInput {}.into()),
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let event = xml_no_std::writer::XmlEvent::StartElement {
                        name: name.borrow(),
                        namespace: namespace.borrow(),
                        attributes: attributes
                            .iter()
                            .map(|attr| Attribute::new(attr.name.borrow(), &attr.value))
                            .collect(),
                    };
                    xml_writer
                        .write(event)
                        .map_err(|e| XerDecodeErrorKind::InvalidOpenType { inner_err: e })?;
                }
                XmlEvent::Characters(text) => {
                    let text = text.borrow();
                    let event = xml_no_std::writer::XmlEvent::Characters(text);
                    xml_writer
                        .write(event)
                        .map_err(|e| XerDecodeErrorKind::InvalidOpenType { inner_err: e })?;
                }
                XmlEvent::Comment(text) => {
                    let text = text.borrow();
                    let event = xml_no_std::writer::XmlEvent::Comment(text);
                    xml_writer
                        .write(event)
                        .map_err(|e| XerDecodeErrorKind::InvalidOpenType { inner_err: e })?;
                }
                other => {
                    if let Some(writer_event) = other.as_writer_event() {
                        xml_writer
                            .write(writer_event)
                            .map_err(|e| XerDecodeErrorKind::InvalidOpenType { inner_err: e })?;
                    }
                }
            }
        }
        Ok(Any::new(xml_writer.into_inner().into_bytes()))
    }

    fn decode_bit_string(
        &mut self,
        __tag: Tag,
        __constraints: Constraints,
    ) -> Result<crate::types::BitString, Self::Error> {
        tag!(StartElement, self)?;
        let value = value_or_empty!(self, parse_bitstring_value, "`1` or `0`");
        tag!(EndElement, self)?;
        value
    }

    fn decode_bool(&mut self, __tag: Tag) -> Result<bool, Self::Error> {
        if !self.in_list {
            tag!(StartElement, self)?;
        }
        let value = match self.next_element() {
            Some(XmlEvent::StartElement { name, .. }) => {
                if name.local_name.as_str() == BOOLEAN_TRUE_TAG {
                    tag!(EndElement, self, BOOLEAN_TRUE_TAG).map(|_| true)
                } else if name.local_name.as_str() == BOOLEAN_FALSE_TAG {
                    tag!(EndElement, self, BOOLEAN_FALSE_TAG).map(|_| false)
                } else if self.in_list {
                    let boolean = match self.next_element() {
                        Some(XmlEvent::Characters(c)) if c == "0" => false,
                        Some(XmlEvent::Characters(c)) if c == "1" => true,
                        Some(XmlEvent::Characters(c)) if c == "false" => false,
                        Some(XmlEvent::Characters(c)) if c == "true" => true,
                        _ => {
                            return Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                                needed: "`<true/>` or `<false/>`",
                                found: alloc::format!("{name:?}"),
                            }))
                        }
                    };
                    tag!(EndElement, self)?;
                    Ok(boolean)
                } else {
                    Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: "`<true/>` or `<false/>`",
                        found: alloc::format!("{name:?}"),
                    }))
                }
            }
            Some(XmlEvent::Characters(c)) if c == "0" => Ok(false),
            Some(XmlEvent::Characters(c)) if c == "1" => Ok(true),
            Some(XmlEvent::Characters(c)) if c == "false" => Ok(false),
            Some(XmlEvent::Characters(c)) if c == "true" => Ok(true),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: bool::IDENTIFIER.0.unwrap(),
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        if !self.in_list {
            tag!(EndElement, self)?;
        }
        value
    }

    fn decode_enumerated<E: Enumerated>(&mut self, __tag: Tag) -> Result<E, Self::Error> {
        if !self.in_list {
            tag!(StartElement, self)?;
        }
        let value = match self.next_element() {
            Some(XmlEvent::StartElement {
                name: OwnedName { local_name, .. },
                ..
            }) => {
                if let Some(e) = E::from_identifier(&local_name) {
                    tag!(EndElement, self).map(|_| e)
                } else {
                    Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: "enumerated value",
                        found: local_name,
                    }))
                }
            }
            Some(XmlEvent::Characters(c)) => E::from_identifier(&c).ok_or(DecodeError::from(
                XerDecodeErrorKind::XmlTypeMismatch {
                    needed: "enumerated value",
                    found: c,
                },
            )),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "enumerated value",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        if !self.in_list {
            tag!(EndElement, self)?;
        }
        value
    }

    fn decode_integer<I: crate::types::IntegerType>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<I, Self::Error> {
        tag!(StartElement, self)?;
        let value = match self.next_element() {
            Some(XmlEvent::Characters(value)) => {
                if let Ok(int) = value.parse::<i128>() {
                    int.try_into()
                        .map_err(|_| DecodeError::integer_overflow(I::WIDTH, crate::Codec::Jer))
                } else {
                    Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: "integer value",
                        found: value,
                    }))
                }
            }
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "integer value",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, self)?;
        value
    }

    fn decode_null(&mut self, _tag: Tag) -> Result<(), Self::Error> {
        tag!(StartElement, self)?;
        tag!(EndElement, self)?;
        Ok(())
    }

    fn decode_object_identifier(
        &mut self,
        _tag: Tag,
    ) -> Result<crate::types::ObjectIdentifier, Self::Error> {
        tag!(StartElement, self)?;
        let value = match self.next_element() {
            Some(XmlEvent::Characters(value)) => parse_object_identifier(&value),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "'.'-separated numeric object identifier arcs",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, self)?;
        value
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
        tag!(StartElement, self)?;
        let mut field_names = D::FIELDS
            .iter()
            .map(|f| f.name)
            .collect::<alloc::vec::Vec<&str>>();
        if let Some(extended_fields) = D::EXTENDED_FIELDS {
            field_names.extend(extended_fields.iter().map(|f| f.name));
        }
        let events = self
            .stack
            .pop()
            .ok_or_else(|| error!(EndOfXmlInput))?
            .events;
        let mut sequence_decoder = Decoder::try_from(events)?;
        sequence_decoder.sort_by_field_name_order(field_names)?;
        (decode_fn)(&mut sequence_decoder)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<alloc::vec::Vec<D>, Self::Error> {
        decode_sequence_or_set_items(self)
    }

    fn decode_set_of<D: crate::Decode + Eq + core::hash::Hash>(
        &mut self,
        _t: Tag,
        _c: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        let items = decode_sequence_or_set_items::<D>(self)?;
        Ok(SetOf::from_vec(items))
    }

    fn decode_octet_string<'b, T: From<alloc::vec::Vec<u8>> + From<&'b [u8]>>(
        &'b mut self,
        _: Tag,
        _c: Constraints,
    ) -> Result<T, Self::Error> {
        tag!(StartElement, self)?;
        let value = match self.peek() {
            Some(XmlEvent::Characters(s)) => parse_octetstring_value(s),
            Some(XmlEvent::EndElement { .. }) => return Ok(<T as From<&'b [u8]>>::from(&[])),
            Some(elem) => {
                return Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                    needed: "hexadecimal characters",
                    found: alloc::format!("{elem:?}"),
                }))
            }
            _ => return Err(DecodeError::from(XerDecodeErrorKind::EndOfXmlInput {})),
        };
        let _ = self.next_element();
        tag!(EndElement, self)?;
        value.map(T::from)
    }

    fn decode_utf8_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::Utf8String, Self::Error> {
        tag!(StartElement, self)?;
        let value = match self.next_element() {
            Some(XmlEvent::Characters(value)) => Ok(value),
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "UTF8 string value",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, self)?;
        value
    }

    fn decode_visible_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::VisibleString, Self::Error> {
        decode_string!(
            self,
            crate::types::VisibleString::try_from,
            Tag::VISIBLE_STRING,
            "VisibleString value"
        )
    }

    fn decode_general_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::GeneralString, Self::Error> {
        decode_string!(
            self,
            crate::types::GeneralString::try_from,
            Tag::GENERAL_STRING,
            "GeneralString value"
        )
    }

    fn decode_ia5_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::Ia5String, Self::Error> {
        decode_string!(
            self,
            crate::types::Ia5String::try_from,
            Tag::IA5_STRING,
            "IA5String value"
        )
    }

    fn decode_graphic_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::GraphicString, Self::Error> {
        decode_string!(
            self,
            crate::types::GraphicString::try_from,
            Tag::GRAPHIC_STRING,
            "GraphicString value"
        )
    }

    fn decode_printable_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::PrintableString, Self::Error> {
        decode_string!(
            self,
            crate::types::PrintableString::try_from,
            Tag::PRINTABLE_STRING,
            "PrintableString value"
        )
    }

    fn decode_numeric_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::NumericString, Self::Error> {
        decode_string!(
            self,
            crate::types::NumericString::try_from,
            Tag::NUMERIC_STRING,
            "NumericString value"
        )
    }

    fn decode_teletex_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::TeletexString, Self::Error> {
        todo!()
    }

    fn decode_bmp_string(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<crate::types::BmpString, Self::Error> {
        decode_string!(
            self,
            crate::types::BmpString::try_from,
            Tag::BMP_STRING,
            "BMP String value"
        )
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _tag: Tag) -> Result<D, Self::Error> {
        D::decode(self)
    }

    fn decode_utc_time(&mut self, _tag: Tag) -> Result<crate::types::UtcTime, Self::Error> {
        decode_time!(self, crate::ber::de::Decoder::parse_any_utc_time_string)
    }

    fn decode_generalized_time(
        &mut self,
        _tag: Tag,
    ) -> Result<crate::types::GeneralizedTime, Self::Error> {
        decode_time!(
            self,
            crate::ber::de::Decoder::parse_any_generalized_time_string
        )
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
        tag!(StartElement, self)?;
        let events = self
            .stack
            .pop()
            .ok_or_else(|| error!(EndOfXmlInput))?
            .events;
        let mut field_indices = SET::FIELDS
            .iter()
            .enumerate()
            .collect::<alloc::vec::Vec<_>>();
        let mut fields = alloc::vec![];
        field_indices
            .sort_by(|(_, a), (_, b)| a.tag_tree.smallest_tag().cmp(&b.tag_tree.smallest_tag()));
        let mut sequence_decoder = Decoder::try_from(events)?;
        sequence_decoder.sort_by_field_tag_order(&field_indices)?;
        for (index, field) in field_indices {
            fields.push((decode_fn)(&mut sequence_decoder, index, field.tag)?);
        }

        for (index, field) in SET::EXTENDED_FIELDS
            .iter()
            .flat_map(|fields| fields.iter())
            .enumerate()
        {
            fields.push((decode_fn)(
                &mut sequence_decoder,
                index + SET::FIELDS.len(),
                field.tag,
            )?);
        }

        (field_fn)(fields)
    }

    fn decode_choice<D>(&mut self, _constraints: Constraints) -> Result<D, Self::Error>
    where
        D: crate::types::DecodeChoice,
    {
        if !self.in_list {
            tag!(StartElement, self)?;
        }
        match self.peek() {
            Some(XmlEvent::StartElement { name, .. }) => {
                let tag = D::IDENTIFIERS
                    .iter()
                    .enumerate()
                    .find(|(_, id)| id.eq_ignore_ascii_case(&name.local_name))
                    .and_then(|(i, _)| {
                        [D::VARIANTS, D::EXTENDED_VARIANTS.unwrap_or(&[])]
                            .concat()
                            .get(i)
                            .copied()
                    })
                    .unwrap_or(Tag::EOC);
                let events = self
                    .stack
                    .pop()
                    .ok_or_else(|| error!(EndOfXmlInput))?
                    .events;
                let mut variant_decoder = Decoder::try_from(events)?;
                D::from_tag(&mut variant_decoder, tag)
            }
            elem => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "Start element of choice option",
                found: alloc::format!("{elem:?}"),
            })),
        }
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        match self.peek() {
            Some(XmlEvent::Characters(c)) if c == OPTIONAL_ITEM_NOT_PRESENT => {
                let _ = self.next_element();
                return Ok(None);
            }
            _ => (),
        }
        D::decode(self).map(Some)
    }

    fn decode_optional_with_tag<D: Decode>(&mut self, _tag: Tag) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        _constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_constraints<D>(
        &mut self,
        _constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
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

    fn decode_real<R: crate::types::RealType>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<R, Self::Error> {
        tag!(StartElement, self)?;
        let value = match self.next_element() {
            Some(XmlEvent::Characters(value)) => match value.as_str().parse::<f64>() {
                Ok(real) => R::try_from_float(real).ok_or_else(|| {
                    DecodeError::integer_overflow(R::BYTE_WIDTH as u32, crate::Codec::Xer)
                }),
                Err(_) if value.as_str() == PLUS_INFINITY_VALUE => Ok(R::INFINITY),
                Err(_) if value.as_str() == MINUS_INFINITY_VALUE => Ok(R::NEG_INFINITY),
                Err(_) if value.as_str() == NAN_VALUE => Ok(R::NAN),
                _ => {
                    return Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: "integer value",
                        found: value,
                    }))
                }
            },
            Some(XmlEvent::StartElement { name, .. }) => {
                tag!(EndElement, self)?;
                match name.local_name.as_str() {
                    PLUS_INFINITY_TAG => Ok(R::INFINITY),
                    MINUS_INFINITY_TAG => Ok(R::NEG_INFINITY),
                    NAN_TAG => Ok(R::NAN),
                    _ => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                        needed: "PLUS-INFINITY, MINUS-INFINITY or NOT-A-NUMBER",
                        found: name.local_name,
                    })),
                }
            }
            Some(elem) => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
                needed: "integer value",
                found: alloc::format!("{elem:?}"),
            })),
            None => Err(error!(EndOfXmlInput)),
        };
        tag!(EndElement, self)?;
        value
    }

    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        _tag: Tag,
    ) -> Result<Option<D>, Self::Error> {
        self.decode_optional()
    }

    fn decode_date(&mut self, _tag: Tag) -> Result<Date, Self::Error> {
        decode_time!(
            self,
            crate::ber::de::Decoder::parse_any_generalized_time_string
        )
        .map(|dt| dt.date_naive())
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition()
    }

    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        _tag: Tag,
        _constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition()
    }
}

fn parse_bitstring_value(val: &str) -> Result<BitString, DecodeError> {
    // TODO: Add support for X.680 §22.9 XMLIdentifierLists
    if !val
        .chars()
        .all(|c| c == '1' || c == '0' || c.is_whitespace())
    {
        return Err(error!(
            SpecViolation,
            r#"§12.11 An "xmlbstring" shall consist of an arbitrary number (possibly zero) of zeros, ones or white-space"#
        ));
    }
    Ok(BitString::from_iter(val.chars().filter_map(|c| {
        (c == '1').then_some(true).or((c == '0').then_some(false))
    })))
}

fn parse_octetstring_value(val: &str) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect::<Result<alloc::vec::Vec<_>, _>>()
        .map_err(|e| XerDecodeErrorKind::InvalidXerOctetstring { parse_int_err: e }.into())
}

fn parse_object_identifier(val: &str) -> Result<ObjectIdentifier, DecodeError> {
    let arcs = val
        .split('.')
        .try_fold(alloc::vec::Vec::<u32>::new(), |mut acc, curr| {
            curr.parse()
                .map(|i| {
                    acc.push(i);
                    acc
                })
                .map_err(|_| {
                    DecodeError::from(XerDecodeErrorKind::InvalidInput {
                        details: "Invalid Object Identifier value.",
                    })
                })
        })?;
    ObjectIdentifier::new(arcs).ok_or_else(|| {
        XerDecodeErrorKind::InvalidInput {
            details: "Invalid Object Identifier value.",
        }
        .into()
    })
}

fn decode_sequence_or_set_items<D: Decode>(
    decoder: &mut Decoder,
) -> Result<alloc::vec::Vec<D>, DecodeError> {
    let identifier = match decoder.next_element() {
        Some(XmlEvent::StartElement { name, .. }) => Ok(name),
        elem => Err(DecodeError::from(XerDecodeErrorKind::XmlTypeMismatch {
            needed: "StartElement of SEQUENCE OF",
            found: alloc::format!("{elem:?}"),
        })),
    }?;

    let mut inner_decoder: Decoder = if let Some(XmlEvent::Characters(c)) = decoder.peek() {
        let mut elems = alloc::vec![alloc::vec![XmlEvent::EndElement {
            name: identifier.clone()
        }]];
        elems.extend(c.split_ascii_whitespace().map(|item| {
            alloc::vec![
                XmlEvent::StartElement {
                    name: OwnedName {
                        local_name: D::IDENTIFIER.0.unwrap_or("dummy").to_string(),
                        namespace: None,
                        prefix: None,
                    },
                    attributes: alloc::vec::Vec::new(),
                    namespace: Namespace::empty(),
                },
                XmlEvent::Characters(item.to_string()),
                XmlEvent::EndElement {
                    name: OwnedName {
                        local_name: D::IDENTIFIER.0.unwrap_or("dummy").to_string(),
                        namespace: None,
                        prefix: None,
                    },
                },
            ]
        }));
        let _ = decoder.stack.pop();
        Decoder::from_stack_elems(elems)
    } else {
        decoder
            .stack
            .pop()
            .ok_or_else(|| error!(EndOfXmlInput))?
            .events
            .try_into()?
    }
    .into_list_decoder();

    let mut items = alloc::vec::Vec::new();
    let mut level_of_nesting = 0;
    loop {
        match inner_decoder.peek() {
            Some(XmlEvent::StartElement { name, .. }) if name == &identifier => {
                level_of_nesting += 1;
                items.push(D::decode(&mut inner_decoder)?)
            }
            Some(XmlEvent::EndElement { name }) if name == &identifier && level_of_nesting == 0 => {
                break
            }
            Some(XmlEvent::EndElement { name }) if name == &identifier => {
                level_of_nesting -= 1;
                inner_decoder.next_element();
            }
            None => break,
            _ => items.push(D::decode(&mut inner_decoder)?),
        }
    }
    items.reverse();

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::Decoder;
    use crate::{types::*, AsnType, Decode, Decoder as _};
    use bitvec::order::Msb0;

    macro_rules! decode_test_1 {
        ($suite:ident, $method:ident, $xml:literal, $expected:expr) => {
            #[test]
            fn $suite() {
                let mut decoder = Decoder::new($xml.as_bytes()).unwrap();
                assert_eq!(
                    crate::Decoder::$method(&mut decoder, Tag::CHOICE).unwrap(),
                    $expected
                )
            }
        };
    }

    macro_rules! decode_test_2 {
        ($suite:ident, $method:path, $xml:literal, $expected:expr) => {
            #[test]
            fn $suite() {
                let mut decoder = Decoder::new($xml.as_bytes()).unwrap();
                assert_eq!(
                    $method(&mut decoder, Tag::CHOICE, Constraints::NONE).unwrap(),
                    $expected
                )
            }
        };
    }

    #[test]
    fn creates_decoder() {
        Decoder::new(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <Actual>
          <errorCode>
            <local>1</local>
          </errorCode>
          <parameter>
            <BOOLEAN><false/></BOOLEAN>
          </parameter>
        </Actual>"#
                .as_bytes(),
        )
        .unwrap();
    }

    decode_test_1!(
        boolean_true,
        decode_bool,
        "<BOOLEAN><true/></BOOLEAN>",
        true
    );
    decode_test_1!(
        boolean_false,
        decode_bool,
        "<BOOLEAN><false/></BOOLEAN>",
        false
    );
    decode_test_2!(
        bit_string,
        crate::Decoder::decode_bit_string,
        "<BIT_STRING>1010</BIT_STRING>",
        bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
    );
    decode_test_2!(
        bit_string_ws,
        crate::Decoder::decode_bit_string,
        "<BIT_STRING>  1   0  1  0  </BIT_STRING>",
        bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
    );
    decode_test_2!(
        bit_string_empty,
        crate::Decoder::decode_bit_string,
        "<BIT_STRING/>",
        bitvec::bitvec![u8, Msb0;]
    );

    #[derive(AsnType, Decode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct TestTypeA {
        wine: bool,
        grappa: BitString,
    }

    #[derive(AsnType, Decode, Debug, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct TestTypeParent {
        sinmin: bool,
        nested: TestTypeA,
    }

    #[test]
    fn sequence() {
        let mut decoder = Decoder::new(
            "<TestTypeA><grappa>1010</grappa><wine><false/></wine></TestTypeA>".as_bytes(),
        )
        .unwrap();
        assert_eq!(
            TestTypeA::decode(&mut decoder).unwrap(),
            TestTypeA {
                wine: false,
                grappa: bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
            }
        )
    }

    #[test]
    fn sequence_nested() {
        let mut decoder = Decoder::new(
            "<TestTypeParent><nested><grappa>1 11 1 </grappa><wine><false/></wine></nested><sinmin><true/></sinmin></TestTypeParent>".as_bytes(),
        )
        .unwrap();
        assert_eq!(
            TestTypeParent::decode(&mut decoder).unwrap(),
            TestTypeParent {
                nested: TestTypeA {
                    wine: false,
                    grappa: bitvec::bitvec![u8, Msb0; 1, 1, 1, 1]
                },
                sinmin: true
            }
        )
    }

    #[derive(AsnType, Clone, Debug, Decode, PartialEq)]
    #[rasn(crate_root = "crate", automatic_tags, set)]
    struct TestSetA {
        wine: bool,
        grappa: BitString,
    }

    #[test]
    fn set() {
        let mut decoder = Decoder::new(
            "<TestTypeA><grappa>1010</grappa><wine><false/></wine></TestTypeA>".as_bytes(),
        )
        .unwrap();
        assert_eq!(
            TestSetA::decode(&mut decoder).unwrap(),
            TestSetA {
                wine: false,
                grappa: bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
            }
        )
    }

    decode_test_2!(
        positive_int,
        crate::Decoder::decode_integer::<i128>,
        "<INTEGER>1283749501626451264</INTEGER>",
        1283749501626451264_i128
    );

    decode_test_2!(
        negative_int,
        crate::Decoder::decode_integer::<i32>,
        "<INTEGER>-124142</INTEGER>",
        -124142_i32
    );

    #[derive(AsnType, Decode, Debug, PartialEq, Clone, Copy)]
    #[rasn(enumerated)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum TestEnum {
        #[rasn(identifier = "option-A")]
        OptionA,
        #[rasn(identifier = "option-B")]
        OptionB,
    }

    #[test]
    fn enumerated() {
        let mut decoder = Decoder::new("<TestEnum><option-B/></TestEnum>".as_bytes()).unwrap();
        assert_eq!(TestEnum::decode(&mut decoder).unwrap(), TestEnum::OptionB);
        assert_eq!(decoder.len(), 0);
        let mut decoder = Decoder::new("<TestEnum>option-B</TestEnum>".as_bytes()).unwrap();
        assert_eq!(TestEnum::decode(&mut decoder).unwrap(), TestEnum::OptionB);
        assert_eq!(decoder.len(), 0);
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    #[rasn(crate_root = "crate")]
    struct SeqOfType(SequenceOf<Integer>);

    #[test]
    fn sequence_of() {
        let mut decoder = Decoder::new(
            r#"<SeqOfType>
            <INTEGER>-1</INTEGER>
            <INTEGER>-5</INTEGER>
            <INTEGER>0</INTEGER>
            <INTEGER>55</INTEGER>
            <INTEGER>212424214</INTEGER>
          </SeqOfType>"#
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            SeqOfType::decode(&mut decoder).unwrap(),
            SeqOfType(vec![
                Integer::from(-1),
                Integer::from(-5),
                Integer::from(0),
                Integer::from(55),
                Integer::from(212424214)
            ])
        )
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    #[rasn(crate_root = "crate")]
    struct NestedSeqOf(SequenceOf<SeqOfType>);

    #[test]
    fn nested_sequence_of() {
        let mut decoder = Decoder::new(
            r#"<NestedSeqOf>
                <SeqOfType>
                    <INTEGER>-1</INTEGER>
                    <INTEGER>-5</INTEGER>
                    <INTEGER>0</INTEGER>
                    <INTEGER>55</INTEGER>
                    <INTEGER>212424214</INTEGER>
                </SeqOfType>
                <SeqOfType>
                    <INTEGER>1</INTEGER>
                    <INTEGER>5</INTEGER>
                    <INTEGER>0</INTEGER>
                    <INTEGER>55</INTEGER>
                    <INTEGER>212424214</INTEGER>
                </SeqOfType>
            </NestedSeqOf>"#
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            NestedSeqOf::decode(&mut decoder).unwrap(),
            NestedSeqOf(vec![
                SeqOfType(vec![
                    Integer::from(-1),
                    Integer::from(-5),
                    Integer::from(0),
                    Integer::from(55),
                    Integer::from(212424214)
                ]),
                SeqOfType(vec![
                    Integer::from(1),
                    Integer::from(5),
                    Integer::from(0),
                    Integer::from(55),
                    Integer::from(212424214)
                ])
            ])
        )
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    #[rasn(crate_root = "crate")]
    struct SetOfType(crate::types::SetOf<Integer>);

    #[test]
    fn set_of() {
        let mut decoder = Decoder::new(
            r#"<SeqOfType>
            <INTEGER>-1</INTEGER>
            <INTEGER>-5</INTEGER>
            <INTEGER>0</INTEGER>
            <INTEGER>55</INTEGER>
            <INTEGER>212424214</INTEGER>
          </SeqOfType>"#
                .as_bytes(),
        )
        .unwrap();
        let expected = SetOf::from_vec(
            [
                Integer::from(-1),
                Integer::from(-5),
                Integer::from(0),
                Integer::from(55),
                Integer::from(212424214),
            ]
            .into_iter()
            .collect::<Vec<_>>(),
        );

        assert_eq!(
            SetOfType::decode(&mut decoder).unwrap(),
            SetOfType(expected)
        )
    }

    #[test]
    fn generalized_time() {
        let mut decoder =
            Decoder::new(r#"<TimeType>20001231235959.999+0000</TimeType>"#.as_bytes()).unwrap();

        assert_eq!(
            crate::types::GeneralizedTime::decode(&mut decoder).unwrap(),
            GeneralizedTime::from(
                chrono::NaiveDate::from_ymd_opt(2000, 12, 31)
                    .unwrap()
                    .and_hms_milli_opt(23, 59, 59, 999)
                    .unwrap()
                    .and_utc()
            )
        )
    }

    #[test]
    fn utc_time() {
        let mut decoder = Decoder::new(r#"<TimeType>991231235900Z</TimeType>"#.as_bytes()).unwrap();

        assert_eq!(
            crate::types::UtcTime::decode(&mut decoder).unwrap(),
            UtcTime::from(
                chrono::NaiveDate::from_ymd_opt(1999, 12, 31)
                    .unwrap()
                    .and_hms_opt(23, 59, 0)
                    .unwrap()
                    .and_utc()
            )
        )
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(choice)]
    #[rasn(crate_root = "crate")]
    enum ChoiceType {
        #[rasn(identifier = "int")]
        Int(u8),
        #[rasn(identifier = "bool")]
        Bool(bool),
    }

    #[test]
    fn choice() {
        let mut decoder = Decoder::new(
            r#"<ChoiceType>
            <int>5</int>
          </ChoiceType>"#
                .as_bytes(),
        )
        .unwrap();

        assert_eq!(
            ChoiceType::decode(&mut decoder).unwrap(),
            ChoiceType::Int(5)
        )
    }

    #[test]
    fn sequence_of_choices() {
        let mut decoder = Decoder::new(
            r#"
        <SEQUENCE_OF>
            <int>5</int>
            <bool><false/></bool>
        </SEQUENCE_OF>"#
                .as_bytes(),
        )
        .unwrap();

        assert_eq!(
            SequenceOf::<ChoiceType>::decode(&mut decoder).unwrap(),
            vec![ChoiceType::Int(5), ChoiceType::Bool(false)]
        )
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct OptionalTest {
        wine: Option<bool>,
        grappa: BitString,
    }

    #[test]
    fn optional_present() {
        let mut decoder = Decoder::new(
            "<TestTypeA><grappa>1010</grappa><wine><false/></wine></TestTypeA>".as_bytes(),
        )
        .unwrap();
        assert_eq!(
            OptionalTest::decode(&mut decoder).unwrap(),
            OptionalTest {
                wine: Some(false),
                grappa: bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
            }
        );
    }

    #[test]
    fn optional_absent() {
        let mut decoder =
            Decoder::new("<TestTypeA><grappa>1010</grappa></TestTypeA>".as_bytes()).unwrap();
        assert_eq!(
            OptionalTest::decode(&mut decoder).unwrap(),
            OptionalTest {
                wine: None,
                grappa: bitvec::bitvec![u8, Msb0; 1, 0, 1, 0]
            }
        );
    }

    #[derive(AsnType, Debug, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct AnyTest {
        grappa: Any,
    }

    #[test]
    fn decodes_any() {
        let mut decoder = Decoder::new(
            "<AnyTest><grappa><Actual><Hello>7</Hello><Text>Text</Text></Actual></grappa></AnyTest>".as_bytes(),
        )
        .unwrap();
        assert_eq!(
            "<Actual><Hello>7</Hello><Text>Text</Text></Actual>".as_bytes(),
            AnyTest::decode(&mut decoder).unwrap().grappa.as_bytes()
        )
    }

    #[test]
    fn decodes_object_identifier() {
        let mut decoder =
            Decoder::new("<OBJECT_IDENTIFIER>1.0.8571.2.1</OBJECT_IDENTIFIER>".as_bytes()).unwrap();
        assert_eq!(
            ObjectIdentifier::decode(&mut decoder).unwrap(),
            ObjectIdentifier::new(&[1, 0, 8571, 2, 1]).unwrap()
        )
    }

    #[test]
    fn mapem() {
        use crate::Encode;
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, crate_root = "crate", size("1..=63"))]
        pub struct DescriptiveName(pub Ia5String);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags, crate_root = "crate")]
        #[non_exhaustive]
        pub struct IntersectionGeometry {
            pub name: Option<DescriptiveName>,
        }
        impl IntersectionGeometry {
            pub fn new(name: Option<DescriptiveName>) -> Self {
                Self { name }
            }
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, crate_root = "crate", size("1..=32"))]
        pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags, crate_root = "crate")]
        #[allow(clippy::upper_case_acronyms)]
        pub struct MAPEM {
            pub map: MapData,
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags, crate_root = "crate")]
        #[non_exhaustive]
        pub struct MapData {
            pub intersections: Option<IntersectionGeometryList>,
        }
        impl MapData {
            pub fn new(intersections: Option<IntersectionGeometryList>) -> Self {
                Self { intersections }
            }
        }

        let encoded = r#"<?xml version="1.0"?><MAPEM><map><intersections><IntersectionGeometry><name>MAP_ITS_00\19\19.3</name></IntersectionGeometry></intersections></map></MAPEM>"#;
        assert_eq!(
            MAPEM {
                map: MapData::new(Some(IntersectionGeometryList(vec![
                    IntersectionGeometry::new(Some(DescriptiveName(
                        Ia5String::from_iso646_bytes(r#"MAP_ITS_00\19\19.3"#.as_bytes()).unwrap()
                    )))
                ])))
            },
            crate::xer::decode::<MAPEM>(encoded.as_bytes()).unwrap()
        );
    }

    #[derive(AsnType, Debug, Clone, Decode, PartialEq)]
    #[rasn(automatic_tags, crate_root = "crate")]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ZoneIds(pub SequenceOf<Zid>);

    #[derive(AsnType, Debug, Clone, Decode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags, crate_root = "crate")]
    #[rasn(delegate, value("1..=32", extensible))]
    pub struct Zid(pub Integer);

    #[derive(AsnType, Debug, Clone, Decode, PartialEq)]
    #[rasn(automatic_tags, crate_root = "crate")]
    #[non_exhaustive]
    pub struct GicPart {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: Option<ZoneIds>,
        pub direction: Option<u32>,
    }

    #[test]
    fn simple_type_sequence_of() {
        let mut encoded = r#"<ZoneIds>2 5</ZoneIds>"#;
        assert_eq!(
            ZoneIds(vec![Zid(2.into()), Zid(5.into())]),
            crate::xer::decode::<ZoneIds>(encoded.as_bytes()).unwrap()
        );
        encoded = r#"<ZoneIds>2</ZoneIds>"#;
        assert_eq!(
            ZoneIds(vec![Zid(2.into())]),
            crate::xer::decode::<ZoneIds>(encoded.as_bytes()).unwrap()
        );
        encoded = r#"<GicPart>
                        <detectionZoneIds>2</detectionZoneIds>
                        <relevanceZoneIds>1</relevanceZoneIds>
                        <direction>0</direction>
                    </GicPart>"#;
        assert_eq!(
            GicPart {
                detection_zone_ids: Some(ZoneIds(alloc::vec![Zid(2.into())])),
                relevance_zone_ids: Some(ZoneIds(alloc::vec![Zid(1.into())])),
                direction: Some(0),
            },
            crate::xer::decode::<GicPart>(encoded.as_bytes()).unwrap()
        );
    }

    #[test]
    fn decodes_with_and_without_default() {
        #[derive(AsnType, Debug, Decode, PartialEq)]
        #[rasn(automatic_tags)]
        #[rasn(crate_root = "crate")]
        struct DefaultSequence {
            #[rasn(identifier = "bool-df", default = "bool_default")]
            bool_with_default: bool,
            recursion: Vec<DefaultSequence>,
        }

        fn bool_default() -> bool {
            bool::default()
        }

        assert_eq!(
            crate::xer::decode::<DefaultSequence>(
                r#"
                <DefaultSequence>
                    <recursion>
                        <DefaultSequence>
                            <bool-df>
                                <false />
                            </bool-df>
                            <recursion />
                        </DefaultSequence>
                        <DefaultSequence>
                            <recursion />
                        </DefaultSequence>
                    </recursion>
                </DefaultSequence>
            "#
                .as_bytes()
            )
            .unwrap(),
            DefaultSequence {
                bool_with_default: false,
                recursion: vec![
                    DefaultSequence {
                        bool_with_default: false,
                        recursion: vec![]
                    },
                    DefaultSequence {
                        bool_with_default: false,
                        recursion: vec![]
                    }
                ]
            }
        )
    }

    #[test]
    fn decodes_extended_boolean_notation() {
        assert_eq!(
            crate::xer::decode::<SequenceOf<bool>>(
                r#"
                <SEQUENCE_OF>
                    <true />
                    <false />
                    <BOOLEAN>0</BOOLEAN>
                    <BOOLEAN>1</BOOLEAN>
                    <BOOLEAN>true</BOOLEAN>
                    <BOOLEAN>false</BOOLEAN>
                </SEQUENCE_OF>
            "#
                .as_bytes()
            )
            .unwrap(),
            vec![true, false, false, true, true, false]
        )
    }

    #[test]
    fn decodes_nested_extended_boolean_notation() {
        #[derive(AsnType, Debug, Decode, PartialEq)]
        #[rasn(automatic_tags)]
        #[rasn(crate_root = "crate")]
        struct Boolean {
            val: bool,
        }

        assert_eq!(
            crate::xer::decode::<SequenceOf<Boolean>>(
                r#"
                <SEQUENCE_OF>
                    <Boolean><val><true /></val></Boolean>
                    <Boolean><val><false /></val></Boolean>
                    <Boolean><val>0</val></Boolean>
                    <Boolean><val>1</val></Boolean>
                    <Boolean><val>true</val></Boolean>
                    <Boolean><val>false</val></Boolean>
                </SEQUENCE_OF>
            "#
                .as_bytes()
            )
            .unwrap(),
            vec![
                Boolean { val: true },
                Boolean { val: false },
                Boolean { val: false },
                Boolean { val: true },
                Boolean { val: true },
                Boolean { val: false }
            ]
        )
    }
}
