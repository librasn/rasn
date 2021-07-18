use super::*;

/// An "open" type representing any valid ASN.1 type.
#[derive(Debug, Clone, PartialEq)]
pub enum Open {
    BitString(BitString),
    BmpString(BmpString),
    Bool(bool),
    GeneralizedTime(GeneralizedTime),
    IA5String(IA5String),
    Integer(Integer),
    Null,
    OctetString(OctetString),
    PrintableString(PrintableString),
    UniversalString(UniversalString),
    UtcTime(UtcTime),
    VisibleString(VisibleString),
    InstanceOf(alloc::boxed::Box<InstanceOf<Open>>),
    Unknown {
        tag: Tag,
        value: alloc::vec::Vec<u8>,
    },
}

impl Open {
    /// Returns the tag of the variant.
    pub fn tag(&self) -> Tag {
        match self {
            Self::BitString(_) => BitString::TAG,
            Self::BmpString(_) => BmpString::TAG,
            Self::Bool(_) => bool::TAG,
            Self::GeneralizedTime(_) => GeneralizedTime::TAG,
            Self::IA5String(_) => IA5String::TAG,
            Self::InstanceOf(_) => <InstanceOf<Open>>::TAG,
            Self::Integer(_) => Integer::TAG,
            Self::Null => <()>::TAG,
            Self::OctetString(_) => OctetString::TAG,
            Self::PrintableString(_) => PrintableString::TAG,
            Self::UniversalString(_) => UniversalString::TAG,
            Self::UtcTime(_) => UtcTime::TAG,
            Self::VisibleString(_) => VisibleString::TAG,
            Self::Unknown { tag, .. } => *tag,
        }
    }
}

impl crate::AsnType for Open {
    const TAG: Tag = Tag::EOC;
}

impl crate::Decode for Open {
    fn decode_with_tag<D: crate::Decoder>(_: &mut D, _: Tag) -> Result<Self, D::Error> {
        Err(crate::de::Error::custom(
            "`CHOICE`-style enums cannot be implicitly tagged.",
        ))
    }
    fn decode<D: crate::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(match decoder.peek_tag()? {
            Tag::EOC => return Err(crate::de::Error::custom("Invalid ASN.1 Type")),
            Tag::BIT_STRING => Open::BitString(<_>::decode(decoder)?),
            Tag::BMP_STRING => Open::BmpString(<_>::decode(decoder)?),
            Tag::BOOL => Open::Bool(<_>::decode(decoder)?),
            Tag::IA5_STRING => Open::IA5String(<_>::decode(decoder)?),
            Tag::INTEGER => Open::Integer(<_>::decode(decoder)?),
            Tag::OCTET_STRING => Open::OctetString(<_>::decode(decoder)?),
            Tag::PRINTABLE_STRING => Open::PrintableString(<_>::decode(decoder)?),
            Tag::UNIVERSAL_STRING => Open::UniversalString(<_>::decode(decoder)?),
            Tag::VISIBLE_STRING => Open::VisibleString(<_>::decode(decoder)?),
            Tag::UTC_TIME => Open::UtcTime(<_>::decode(decoder)?),
            Tag::EXTERNAL => Open::InstanceOf(alloc::boxed::Box::new(<_>::decode(decoder)?)),
            Tag::GENERALIZED_TIME => Open::GeneralizedTime(<_>::decode(decoder)?),
            Tag::NULL => {
                decoder.decode_null(<()>::TAG)?;
                Open::Null
            }
            tag => Open::Unknown {
                tag,
                value: decoder.decode_any(tag)?,
            },
        })
    }
}

impl crate::Encode for Open {
    fn encode_with_tag<EN: crate::Encoder>(&self, _: &mut EN, _: Tag) -> Result<(), EN::Error> {
        Err(crate::enc::Error::custom(
            "CHOICE-style enums do not allow implicit tagging.",
        ))
    }

    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            Open::BitString(value) => value.encode(encoder),
            Open::BmpString(value) => crate::Encode::encode(value, encoder),
            Open::Bool(value) => crate::Encode::encode(value, encoder),
            Open::GeneralizedTime(value) => crate::Encode::encode(value, encoder),
            Open::IA5String(value) => crate::Encode::encode(value, encoder),
            Open::InstanceOf(value) => crate::Encode::encode(&**value, encoder),
            Open::Integer(value) => crate::Encode::encode(value, encoder),
            Open::Null => ().encode(encoder),
            Open::OctetString(value) => crate::Encode::encode(value, encoder),
            Open::PrintableString(value) => crate::Encode::encode(value, encoder),
            Open::UniversalString(value) => crate::Encode::encode(value, encoder),
            Open::UtcTime(value) => crate::Encode::encode(value, encoder),
            Open::VisibleString(value) => crate::Encode::encode(value, encoder),
            Open::Unknown { tag, value } => encoder.encode_any(*tag, value).map(drop),
        }
        .map(drop)
    }
}
