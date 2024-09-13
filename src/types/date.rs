use crate::prelude::Constraints;
use crate::types::Date;
use crate::{types::Tag, AsnType, Decode, Decoder, Encode, Encoder};

impl AsnType for Date {
    const TAG: Tag = Tag::DATE;
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
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_date(tag, self).map(drop)
    }
}
