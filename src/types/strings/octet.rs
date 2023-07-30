use crate::prelude::*;

pub use bytes::Bytes as OctetString;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedOctetString<const N: usize>([u8; N]);

impl<const N: usize> FixedOctetString<N> {
    pub fn new(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<[u8; N]> for FixedOctetString<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for FixedOctetString<N> {
    type Error = <[u8; N] as TryFrom<Vec<u8>>>::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl<const N: usize> TryFrom<&[u8]> for FixedOctetString<N> {
    type Error = <[u8; N] as TryFrom<&'static [u8]>>::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl<const N: usize> TryFrom<OctetString> for FixedOctetString<N> {
    type Error = <[u8; N] as TryFrom<&'static [u8]>>::Error;

    fn try_from(value: OctetString) -> Result<Self, Self::Error> {
        (&*value).try_into().map(Self)
    }
}

impl<const N: usize> std::ops::Deref for FixedOctetString<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> std::ops::DerefMut for FixedOctetString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsnType for FixedOctetString<N> {
    const TAG: Tag = Tag::OCTET_STRING;
    const CONSTRAINTS: Constraints<'static> = Constraints::new(&[
        Constraint::Size(Extensible::new(constraints::Size::fixed(N))),
    ]);
}

impl<const N: usize> Decode for FixedOctetString<N> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_octet_string(tag, constraints)?.try_into().map(Self).map_err(|vec| crate::de::Error::custom(format!("length mismatch, expected `{N}`, actual `{}`", vec.len())))
    }
}

impl<const N: usize> Encode for FixedOctetString<N> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_octet_string(tag, constraints, &self.0).map(drop)
    }
}

