use super::{AsnType, Class, ObjectIdentifier, Tag, Constraints};
use crate::types::fields::{Field, Fields, FieldPresence};

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
    fn decode_with_tag_and_constraints<'constraints, D: crate::Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
        decoder.decode_sequence(tag, constraints, |sequence| {
            let type_id = ObjectIdentifier::decode(sequence)?;
            let value = sequence.decode_explicit_prefix(Tag::new(Class::Context, 0))?;

            Ok(Self { type_id, value })
        })
    }
}

impl<T: crate::Encode> crate::Encode for InstanceOf<T> {
    fn encode_with_tag_and_constraints<'constraints, EN: crate::Encoder>(&self, encoder: &mut EN, tag: Tag, constraints: Constraints<'constraints>) -> core::result::Result<(), EN::Error> {
        encoder.encode_sequence::<Self, _>(tag, constraints, |sequence| {
            self.type_id.encode(sequence)?;
            sequence.encode_explicit_prefix(Tag::new(Class::Context, 0), &self.value)?;
            Ok(())
        })?;

        Ok(())
    }
}

impl<T: AsnType> crate::types::Constructed for InstanceOf<T> {
    const FIELDS: Fields = Fields::from_static(&[
        Field {
            tag: ObjectIdentifier::TAG_TREE,
            presence: FieldPresence::Required,
        },
        Field {
            tag: T::TAG_TREE,
            presence: FieldPresence::Required,
        },
    ]);
}
