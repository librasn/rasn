mod oid;

use crate::tag::{self, Tag};

pub use alloc::string::String as Utf8String;
pub use bytes::Bytes as OctetString;
pub use num_bigint::BigInt as Integer;
pub use oid::ObjectIdentifier;

/// A reference to a `BIT STRING`.
pub type BitSlice = bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;
///  Alias for `bitvec::BitVec` mapped to ASN.1'a `BIT STRING`.
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
///  `IA5String` string alias that matches BER's encoding rules.
pub type IA5String = Implicit<tag::IA5_STRING, Utf8String>;
///  `PrintableString` string alias that matches BER's encoding rules.
pub type PrintableString = Implicit<tag::PRINTABLE_STRING, Utf8String>;
///  `VisibleString` string alias that matches BER's encoding rules.
pub type VisibleString = Implicit<tag::VISIBLE_STRING, Utf8String>;
///  `String` alias that matches `BmpString` BER's encoding rules.
pub type BmpString = Implicit<tag::BMP_STRING, Utf8String>;
///  `String` alias that matches BER's encoding rules.
pub type NumericString = Implicit<tag::NUMERIC_STRING, Utf8String>;
///  Alias to `Vec<T>`.
pub type SequenceOf<T> = alloc::vec::Vec<T>;
///  `UniversalString` string alias that matches BER's encoding rules.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;
///  Alias for `chrono::DateTime<Utc>`.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  Alias for `chrono::DateTime<FixedOffset>`.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;

pub use rasn_derive::AsnType;

/// A trait representing any type that can represented in ASN.1.
pub trait AsnType {
    /// The associated tag for the type.
    const TAG: Tag;
}

/// An "open" type representating any valid ASN.1 type.
#[derive(AsnType)]
#[rasn(crate_root = "crate")]
#[rasn(choice)]
pub enum Open {
    BitString(BitString),
    BmpString(BmpString),
    Bool(bool),
    GeneralizedTime(GeneralizedTime),
    IA5String(IA5String),
    Integer(Integer),
    Null,
    NumericString(NumericString),
    OctetString(OctetString),
    PrintableString(PrintableString),
    Sequence(alloc::collections::BTreeMap<Tag, Open>),
    SequenceOf(SequenceOf<Open>),
    UniversalString(UniversalString),
    UtcTime(UtcTime),
    VisibleString(VisibleString),
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
            Self::IA5String(_) => IA5String::TAG,
            Self::PrintableString(_) => PrintableString::TAG,
            Self::VisibleString(_) => VisibleString::TAG,
            Self::BmpString(_) => BmpString::TAG,
            Self::NumericString(_) => NumericString::TAG,
            Self::Sequence(_) => Tag::SEQUENCE,
            Self::SequenceOf(_) => <SequenceOf<Open>>::TAG,
            Self::UniversalString(_) => UniversalString::TAG,
            Self::UtcTime(_) => UtcTime::TAG,
            Self::GeneralizedTime(_) => GeneralizedTime::TAG,
            Self::Bool(_) => bool::TAG,
            Self::Integer(_) => Integer::TAG,
            Self::Null => <()>::TAG,
            Self::OctetString(_) => OctetString::TAG,
            Self::Unknown { tag, .. } => *tag,
        }
    }
}

impl crate::Decode for Open {
    fn decode_with_tag<D: crate::Decoder>(_: &mut D, _: Tag) -> Result<Self, D::Error> {
        Err(crate::error::Error::custom(
            "`CHOICE`-style enums cannot be implicitly tagged.",
        ))
    }
    fn decode<D: crate::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        const TAG_0: crate::Tag = <BitString>::TAG;
        const TAG_1: crate::Tag = <IA5String>::TAG;
        const TAG_2: crate::Tag = <PrintableString>::TAG;
        const TAG_3: crate::Tag = <VisibleString>::TAG;
        const TAG_4: crate::Tag = <BmpString>::TAG;
        const TAG_5: crate::Tag = <NumericString>::TAG;
        const TAG_6: crate::Tag = <SequenceOf<Open>>::TAG;
        const TAG_7: crate::Tag = <UniversalString>::TAG;
        const TAG_8: crate::Tag = <bool>::TAG;
        const TAG_9: crate::Tag = <Integer>::TAG;
        const TAG_10: crate::Tag = <()>::TAG;
        const TAG_11: crate::Tag = <OctetString>::TAG;

        let tag = decoder.peek_tag()?;
        Ok(match tag {
            TAG_0 => Open::BitString(<_>::decode(decoder)?),
            TAG_1 => Open::IA5String(<_>::decode(decoder)?),
            TAG_2 => Open::PrintableString(<_>::decode(decoder)?),
            TAG_3 => Open::VisibleString(<_>::decode(decoder)?),
            TAG_4 => Open::BmpString(<_>::decode(decoder)?),
            TAG_5 => Open::NumericString(<_>::decode(decoder)?),
            TAG_6 => Open::SequenceOf(<_>::decode(decoder)?),
            TAG_7 => Open::UniversalString(<_>::decode(decoder)?),
            TAG_8 => Open::Bool(<_>::decode(decoder)?),
            TAG_9 => Open::Integer(<_>::decode(decoder)?),
            TAG_10 => {
                decoder.decode_null(<()>::TAG)?;
                Open::Null
            }
            TAG_11 => Open::OctetString(<_>::decode(decoder)?),
            tag => Self::Unknown {
                tag,
                value: decoder.decode_octet_string(tag)?,
            },
        })
    }
}

impl crate::Encode for Open {
    fn encode_with_tag<EN: crate::Encoder>(&self, _: &mut EN, _: Tag) -> Result<EN::Ok, EN::Error> {
        Err(crate::error::Error::custom(
            "CHOICE-style enums do not allow implicit tagging.",
        ))
    }

    fn encode<E: crate::Encoder>(&self, encoder: &mut E) -> Result<E::Ok, E::Error> {
        match self {
            Open::BitString(value) => value.encode(encoder),
            Open::IA5String(value) => crate::Encode::encode(value, encoder),
            Open::PrintableString(value) => crate::Encode::encode(value, encoder),
            Open::VisibleString(value) => crate::Encode::encode(value, encoder),
            Open::BmpString(value) => crate::Encode::encode(value, encoder),
            Open::NumericString(value) => crate::Encode::encode(value, encoder),
            Open::Sequence(value) => crate::Encode::encode(value, encoder),
            Open::SequenceOf(value) => crate::Encode::encode(value, encoder),
            Open::UniversalString(value) => crate::Encode::encode(value, encoder),
            Open::UtcTime(value) => crate::Encode::encode(value, encoder),
            Open::GeneralizedTime(value) => crate::Encode::encode(value, encoder),
            Open::Bool(value) => crate::Encode::encode(value, encoder),
            Open::Integer(value) => crate::Encode::encode(value, encoder),
            Open::Null => encoder.encode_null(<()>::TAG),
            Open::OctetString(value) => crate::Encode::encode(value, encoder),
            Open::Unknown { tag, value } => encoder.encode_octet_string(*tag, value),
        }
    }
}

macro_rules! tag_kind {
    ($($name:ident),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name<T, V>{
                _tag: core::marker::PhantomData<T>,
                pub(crate) value: V,
            }

            impl<T, V> $name<T, V>{
                pub fn new(value: V) -> Self {
                    Self {
                        value,
                        _tag: core::marker::PhantomData,
                    }
                }
            }

            impl<T, V> From<V> for $name<T, V> {
                fn from(value: V) -> Self {
                    Self::new(value)
                }
            }

            impl<T, V> core::ops::Deref for $name<T, V> {
                type Target = V;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl<T, V> core::ops::DerefMut for $name<T, V> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }
        )+
    }
}

tag_kind!(Implicit, Explicit);

macro_rules! asn_type {
    ($($name:ty: $value:ident),+) => {
        $(
            impl AsnType for $name {
                const TAG: Tag = Tag::$value;
            }
        )+
    }
}

asn_type! {
    bool: BOOL,
    i8: INTEGER,
    i16: INTEGER,
    i32: INTEGER,
    i64: INTEGER,
    i128: INTEGER,
    isize: INTEGER,
    u8: INTEGER,
    u16: INTEGER,
    u32: INTEGER,
    u64: INTEGER,
    u128: INTEGER,
    usize: INTEGER,
    Integer: INTEGER,
    OctetString: OCTET_STRING,
    ObjectIdentifier: OBJECT_IDENTIFIER,
    BitString: BIT_STRING,
    Utf8String: UTF8_STRING,
    UtcTime: UTC_TIME,
    GeneralizedTime: GENERALIZED_TIME,
    (): NULL,
    &'_ str: UTF8_STRING,
    &'_ BitSlice: BIT_STRING

}

impl<T> AsnType for SequenceOf<T> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T: AsnType, V> AsnType for Implicit<T, V> {
    const TAG: Tag = T::TAG;
}

impl<T: AsnType, V> AsnType for Explicit<T, V> {
    const TAG: Tag = T::TAG;
}

impl<K, V> AsnType for alloc::collections::BTreeMap<K, V> {
    const TAG: Tag = Tag::SEQUENCE;
}
