use crate::{
    macros::{constraints, size_constraint},
    prelude::*,
};

use alloc::vec::Vec;

pub use bytes::Bytes as OctetString;

/// An `OCTET STRING` which has a fixed size range. This type uses const
/// generics to be able to place the octet string on the stack rather than the
/// heap.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedOctetString<const N: usize>([u8; N]);

impl<const N: usize> FixedOctetString<N> {
    /// Creates a new octet string from a given array.
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

impl<const N: usize> core::ops::Deref for FixedOctetString<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> core::ops::DerefMut for FixedOctetString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsnType for FixedOctetString<N> {
    const TAG: Tag = Tag::OCTET_STRING;
    const CONSTRAINTS: Constraints<'static> = constraints!(size_constraint!(N));
}

impl<const N: usize> Decode for FixedOctetString<N> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder
            .decode_octet_string(tag, constraints)?
            .try_into()
            .map(Self)
            .map_err(|vec| {
                D::Error::from(crate::error::DecodeError::fixed_string_conversion_failed(
                    Tag::OCTET_STRING,
                    vec.len(),
                    N,
                    decoder.codec(),
                ))
            })
    }
}

impl<const N: usize> Encode for FixedOctetString<N> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_octet_string(tag, constraints, &self.0)
            .map(drop)
    }
}
