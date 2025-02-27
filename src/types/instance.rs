use super::{AsnType, Class, Constraints, ObjectIdentifier, Tag};
use crate::{
    de::Decoder,
    enc::Encoder,
    types::fields::{Field, FieldPresence, Fields},
};

/// An instance of a defined object class.
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct InstanceOf<T> {
    /// The OID identifying T's real type.
    pub type_id: ObjectIdentifier,
    /// The value identified by `type_id`.
    pub value: T,
}

impl<T> AsnType for InstanceOf<T> {
    const TAG: Tag = Tag::EXTERNAL;
}

impl<T: crate::Decode> crate::Decode for InstanceOf<T> {
    fn decode_with_tag_and_constraints<D: crate::Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_sequence(tag, None::<fn() -> Self>, |sequence| {
            let type_id = ObjectIdentifier::decode(sequence)?;
            let value = sequence.decode_explicit_prefix(Tag::new(Class::Context, 0))?;

            Ok(Self { type_id, value })
        })
    }
}

impl<T: crate::Encode> crate::Encode for InstanceOf<T> {
    fn encode_with_tag_and_constraints<'b, EN: crate::Encoder<'b>>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        _: Constraints,
        identifier: Option<&'static str>,
    ) -> core::result::Result<(), EN::Error> {
        encoder.encode_sequence::<2, 0, Self, _>(
            tag,
            |sequence| {
                self.type_id.encode(sequence)?;
                sequence.encode_explicit_prefix(
                    Tag::new(Class::Context, 0),
                    &self.value,
                    identifier,
                )?;
                Ok(())
            },
            identifier,
        )?;

        Ok(())
    }
}

impl<T: AsnType> crate::types::Constructed<2, 0> for InstanceOf<T> {
    const FIELDS: Fields<2> = Fields::from_static([
        Field {
            index: 0,
            tag: ObjectIdentifier::TAG,
            tag_tree: ObjectIdentifier::TAG_TREE,
            presence: FieldPresence::Required,
            name: "type_id",
        },
        Field {
            index: 1,
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Required,
            name: "value",
        },
    ]);
}
