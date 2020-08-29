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

    fn encode_bit_string(&mut self, value: types::BitString) -> Result<Self::Ok, Self::Error>;
    fn encode_bool(&mut self, value: bool) -> Result<Self::Ok, Self::Error>;
    fn encode_integer(&mut self, value: types::Integer) -> Result<Self::Ok, Self::Error>;
    fn encode_null(&mut self, value: ()) -> Result<Self::Ok, Self::Error>;
    fn encode_object_identifier(
        &mut self,
        value: types::ObjectIdentifier,
    ) -> Result<Self::Ok, Self::Error>;
    fn encode_octet_string(&mut self, value: types::OctetString) -> Result<Self::Ok, Self::Error>;
    fn encode_utf8_string(&mut self, value: types::Utf8String) -> Result<Self::Ok, Self::Error>;
    // fn encode_sequence_of<D: Encode>(&mut self, tag: Tag) -> Result<Vec<D>, Self::Error>;
    // fn encode_sequence(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    // fn encode_set(&mut self, tag: Tag) -> Result<Self, Self::Error>;
    // fn encode_set_of<D: Encode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>, Self::Error>;
    // fn encode_explicit_prefix<D: Encode>(&mut self, tag: Tag) -> Result<D, Self::Error>;
}
