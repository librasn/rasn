use crate::prelude::Constraints;
use crate::types::Date;
use crate::{types::Tag, AsnType, Decode, Decoder, Encode, Encoder};

impl AsnType for Date {
    const TAG: Tag = Tag::DATE;
    const IDENTIFIER: Option<&'static str> = Some("DATE");
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
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder.encode_date(tag, self, identifier).map(drop)
    }
}
