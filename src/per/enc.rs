mod error;

use alloc::vec::Vec;

use crate::{Encode, Tag, types, constraints::Constraints};
use super::Alignment;
pub use error::Error;

pub struct Encoder {
    pub output: bitvec::vec::BitVec<bitvec::order::Msb0, u8>,
    alignment: Alignment,
}

impl Encoder {
    pub fn new(alignment: Alignment) -> Self {
        Self {
            output: Default::default(),
            alignment,
        }
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = Error;

    fn encode_any(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        value: &types::BitString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        value: &types::Integer,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: Constraints,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        todo!()
    }
}

