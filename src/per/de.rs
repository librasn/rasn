mod error;

use alloc::{collections::BTreeSet, vec::Vec};

use crate::{Decode, Constraints, Tag, types};
pub use error::Error;
use super::Alignment;

type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Decoder<'input> {
    input: &'input [u8],
    alignment: Alignment,
}

impl<'input> Decoder<'input> {
    pub fn new(input: &'input [u8], alignment: Alignment) -> Self {
        Self {
            input,
            alignment,
        }
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = Error;

    fn peek_tag(&self) -> Result<Tag> {
        unreachable!("Tags are not encoded in PER.")
    }

    fn decode_any(&mut self, tag: Tag) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool> {
        todo!()
    }

    fn decode_enumerated(&mut self, tag: Tag) -> Result<types::Integer> {
        todo!()
    }

    fn decode_integer<C: Constraints>(&mut self, tag: Tag) -> Result<types::Integer> {
        todo!()
    }

    fn decode_octet_string(&mut self, tag: Tag) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_null(&mut self, tag: Tag) -> Result<()> {
        todo!()
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        todo!()
    }

    fn decode_bit_string(&mut self, tag: Tag) -> Result<types::BitString> {
        todo!()
    }

    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String> {
        todo!()
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        todo!()
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        todo!()
    }

    fn decode_sequence_of<D: Decode>(&mut self, tag: Tag) -> Result<Vec<D>> {
        todo!()
    }

    fn decode_set_of<D: Decode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>> {
        todo!()
    }

    fn decode_set(&mut self, tag: Tag) -> Result<Self> {
        todo!()
    }

    fn decode_sequence(&mut self, tag: Tag) -> Result<Self> {
        todo!()
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        todo!()
    }
}

