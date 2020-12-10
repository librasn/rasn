use super::{AsnType, Explicit, ObjectIdentifier};
use crate::tag::{Class, Tag};

/// An instance of a defined object class.
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceOf<T> {
    /// The OID identifying T's real type.
    pub type_id: ObjectIdentifier,
    /// The value identified by `type_id`.
    pub value: T,
}

impl<T> AsnType for InstanceOf<T> {
    const TAG: Tag = Tag::EXTERNAL;
}

struct C0;
const CONTEXT_0: Tag = Tag::new(Class::Context, 0);

impl AsnType for C0 {
    const TAG: Tag = CONTEXT_0;
}

impl<T: crate::Decode> crate::Decode for InstanceOf<T> {
    fn decode_with_tag<D: crate::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        let mut sequence = decoder.decode_sequence(tag)?;
        let type_id = ObjectIdentifier::decode(&mut sequence)?;
        let value = <Explicit<C0, T>>::decode(&mut sequence)?.value;

        Ok(Self { type_id, value })
    }
}

impl<T: crate::Encode> crate::Encode for InstanceOf<T> {
    fn encode_with_tag<D: crate::Encoder>(
        &self,
        encoder: &mut D,
        tag: Tag,
    ) -> Result<(), D::Error> {
        encoder.encode_sequence::<crate::constraints::Unconstrained, _>(tag, |sequence| {
            self.type_id.encode(sequence)?;
            sequence.encode_explicit_prefix(CONTEXT_0, &self.value)?;
            Ok(())
        })?;

        Ok(())
    }
}
