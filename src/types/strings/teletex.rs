use super::*;

/// A string, which contains the characters defined in T.61 standard.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(Vec<u8>);

impl TeletexString {
    pub fn new(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<Vec<u8>> for TeletexString {
    fn from(vec: Vec<u8>) -> Self {
        Self::new(vec)
    }
}

impl core::ops::Deref for TeletexString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsnType for TeletexString {
    const TAG: Tag = Tag::TELETEX_STRING;
}

impl Encode for TeletexString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_teletex_string(tag, constraints, self)
            .map(drop)
    }
}

impl Decode for TeletexString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_teletex_string(tag, constraints)
    }
}
