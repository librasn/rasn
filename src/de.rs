use alloc::{collections::BTreeSet, vec::Vec};

use crate::tag::Tag;
use crate::types::{self, AsnType};

pub trait Decode: Sized + AsnType {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        Self::decode_with_tag(decoder, Self::TAG)
    }

    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error>;
}

pub trait Decoder: Sized {
    type Error: crate::error::Error;

    /// Returns whether the decoder's input is empty.
    fn is_empty(&self) -> bool;
    /// Peek at the next available tag.
    fn peek_tag(&self) -> Result<Tag, Self::Error>;

    fn decode_bit_string(&mut self, tag: Tag) -> Result<types::BitString, Self::Error>;
    fn decode_bool(&mut self, tag: Tag) -> Result<bool, Self::Error>;
    fn decode_enumerated(&mut self, tag: Tag) -> Result<types::Integer, Self::Error>;
    fn decode_integer(&mut self, tag: Tag) -> Result<types::Integer, Self::Error>;
    fn decode_null(&mut self, tag: Tag) -> Result<(), Self::Error>;
    fn decode_object_identifier(
        &mut self,
        tag: Tag,
    ) -> Result<types::ObjectIdentifier, Self::Error>;
    fn decode_octet_string(&mut self, tag: Tag) -> Result<types::OctetString, Self::Error>;
    fn decode_sequence(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    fn decode_sequence_of<D: Decode>(&mut self, tag: Tag) -> Result<Vec<D>, Self::Error>;
    fn decode_set(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    fn decode_set_of<D: Decode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>, Self::Error>;
    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String, Self::Error>;
    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error>;
}

impl Decode for () {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_null(tag)
    }
}

impl Decode for bool {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_bool(tag)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                core::convert::TryInto::try_into(decoder.decode_integer(tag)?)
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
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_integer(tag)
    }
}

impl Decode for types::OctetString {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_octet_string(tag)
    }
}

impl Decode for types::ObjectIdentifier {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_object_identifier(tag)
    }
}

impl Decode for types::BitString {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_bit_string(tag)
    }
}

impl Decode for types::Utf8String {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_utf8_string(tag)
    }
}

impl<T: Decode> Decode for alloc::vec::Vec<T> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_sequence_of(tag)
    }
}

impl<T: AsnType, V: Decode> Decode for types::Implicit<T, V> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        Ok(Self::new(V::decode_with_tag(decoder, tag)?))
    }
}

impl<T: AsnType, V: Decode> Decode for types::Explicit<T, V> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        Ok(Self::new(decoder.decode_explicit_prefix(tag)?))
    }
}
