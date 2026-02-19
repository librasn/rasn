use crate::prelude::Constraints;
use crate::types::Date;
use crate::{AsnType, Decode, Decoder, Encode, Encoder, types::Tag};

use super::Identifier;

impl AsnType for Date {
    const TAG: Tag = Tag::DATE;
    const IDENTIFIER: Identifier = Identifier::DATE;
}

impl Decode for Date {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_date(tag)
    }
}

impl Encode for Date {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder.encode_date(tag, self, identifier).map(drop)
    }
}
