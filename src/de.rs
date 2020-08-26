use alloc::vec::Vec;

use crate::tag::Tag;
use crate::types;

pub trait Decode: Sized {
    const TAG: Tag;
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error>;
}

pub trait Decoder {
    type Error: crate::error::Error;

    fn is_empty(&self) -> bool;

    fn decode_bool(self, tag: Tag) -> Result<bool, Self::Error>;
    fn decode_integer(self, tag: Tag) -> Result<types::Integer, Self::Error>;
    fn decode_octet_string(self, tag: Tag) -> Result<types::OctetString, Self::Error>;
    fn decode_null(self, tag: Tag) -> Result<(), Self::Error>;
    fn decode_object_identifier(self, tag: Tag) -> Result<types::ObjectIdentifier, Self::Error>;
    fn decode_bit_string(self, tag: Tag) -> Result<types::BitString, Self::Error>;
    fn decode_utf8_string(self, tag: Tag) -> Result<types::Utf8String, Self::Error>;
    fn decode_sequence_of<D: Decode>(self, tag: Tag) -> Result<Vec<D>, Self::Error>;
}

impl Decode for bool {
    const TAG: Tag = Tag::BOOL;
    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_bool(Self::TAG)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            const TAG: Tag = Tag::INTEGER;

            fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
                core::convert::TryInto::try_into(decoder.decode_integer(Self::TAG)?)
                    .map_err(crate::error::Error::custom)
            }
        }
        )+
    }
}

impl_integers! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
}

impl Decode for types::Integer {
    const TAG: Tag = Tag::INTEGER;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_integer(Self::TAG)
    }
}

impl Decode for types::OctetString {
    const TAG: Tag = Tag::OCTET_STRING;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_octet_string(Self::TAG)
    }
}

impl Decode for types::ObjectIdentifier {
    const TAG: Tag = Tag::OBJECT_IDENTIFIER;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_object_identifier(Self::TAG)
    }
}

impl Decode for types::BitString {
    const TAG: Tag = Tag::BIT_STRING;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_bit_string(Self::TAG)
    }
}

impl Decode for types::Utf8String {
    const TAG: Tag = Tag::UTF8_STRING;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_utf8_string(Self::TAG)
    }
}

impl<T: Decode> Decode for alloc::vec::Vec<T> {
    const TAG: Tag = Tag::SEQUENCE;

    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
        decoder.decode_sequence_of(Self::TAG)
    }
}

struct Foo {
    b: Option<bool>,
    i: Option<u32>,
}

//impl Decode for Foo {
//    const TAG: Tag = Tag::SEQUENCE;
//
//    fn decode<D: Decoder>(decoder: D) -> Result<Self, D::Error> {
//        let b = None;
//        let i = None;
//
//        while !decoder.is_empty() {
//            // match decoder.decode_tag()? {
//            //     Tag::BOOL =>
//            // }
//        }
//
//        Ok(Self { b, i })
//    }
//}
