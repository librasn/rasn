use crate::{tag::Tag, types};

pub trait Encode: types::AsnType {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<E::Ok, E::Error> {
        self.encode_with_tag(encoder, Self::TAG)
    }

    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error>;
}

pub trait Encoder {
    type Ok;
    type Error;

    fn encode_explicit_prefix<V: Encode>(&mut self, tag: Tag, value: &V) -> Result<Self::Ok, Self::Error>;
    fn encode_bit_string(&mut self, tag: Tag, value: &types::BitSlice) -> Result<Self::Ok, Self::Error>;
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error>;
    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error>;
    fn encode_integer(&mut self, tag: Tag, value: types::Integer) -> Result<Self::Ok, Self::Error>;
    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error>;
    fn encode_object_identifier(&mut self, tag: Tag, value: &[u32]) -> Result<Self::Ok, Self::Error>;
    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error>;
    fn encode_sequence_of<E: Encode>(&mut self, tag: Tag, value: &[E]) -> Result<Self::Ok, Self::Error>;
    fn encode_sequence<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
        where F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>;
    // fn encode_sequence(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    // fn encode_set(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    // fn encode_set_of<D: Encode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>, Self::Error>;
    // fn encode_explicit_prefix<D: Encode>(&mut self, tag: Tag) -> Result<D, Self::Error>;
}

impl Encode for () {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_null(tag)
    }
}

impl Encode for bool {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_bool(tag, *self)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+) => {
        $(
            impl Encode for $int {
                fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
                    encoder.encode_integer(tag, (*self).into())
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
    usize
}

impl Encode for types::OctetString {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_octet_string(tag, self)
    }
}

impl Encode for types::Utf8String {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_utf8_string(tag, self)
    }
}

impl Encode for &'_ str {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_utf8_string(tag, self)
    }
}

impl Encode for types::BitString {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_bit_string(tag, self)
    }
}

impl Encode for &'_ types::BitSlice {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_bit_string(tag, self)
    }
}

impl Encode for types::ObjectIdentifier {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<E::Ok, E::Error> {
        encoder.encode_object_identifier(tag, self)
    }
}

impl<E: Encode> Encode for types::SequenceOf<E> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<EN::Ok, EN::Error> {
        encoder.encode_sequence_of(tag, self)
    }
}

impl<T: crate::types::AsnType, V: Encode> Encode for crate::tag::Implicit<T, V> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<EN::Ok, EN::Error> {
        V::encode_with_tag(&self.value, encoder, tag)
    }
}

impl<T: crate::types::AsnType, V: Encode> Encode for crate::tag::Explicit<T, V> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<EN::Ok, EN::Error> {
        encoder.encode_explicit_prefix(tag, &self.value)
    }
}
