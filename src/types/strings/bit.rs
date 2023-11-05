use crate::prelude::*;

///  The `BIT STRING` type.
/// /// ## Usage
/// ASN1 declaration such as ...
/// ```asn
/// Test-type-a ::= BIT STRING
/// ```
/// ... can be represented using `rasn` as ...
/// ```rust
/// use rasn::prelude::*;
///
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate)]
/// struct TestTypeA(pub BitString);
/// ```
pub type BitString = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
///  A fixed length `BIT STRING` type.
pub type FixedBitString<const N: usize> = bitvec::array::BitArray<[u8; N], bitvec::order::Msb0>;
///  A reference to a `BIT STRING` type.
pub type BitStr = bitvec::slice::BitSlice<u8, bitvec::order::Msb0>;

impl AsnType for BitString {
    const TAG: Tag = Tag::BIT_STRING;
}

impl Decode for BitString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_bit_string(tag, constraints)
    }
}

impl Encode for BitString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bit_string(tag, constraints, self).map(drop)
    }
}

impl AsnType for BitStr {
    const TAG: Tag = Tag::BIT_STRING;
}

impl Encode for BitStr {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bit_string(tag, constraints, self).map(drop)
    }
}

impl<const N: usize> AsnType for FixedBitString<N> {
    const TAG: Tag = Tag::BIT_STRING;
}

impl<const N: usize> Decode for FixedBitString<N> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder
            .decode_bit_string(tag, constraints)?
            .as_bitslice()
            .try_into()
            .map_err(crate::de::Error::custom)
    }
}

impl<const N: usize> Encode for FixedBitString<N> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bit_string(tag, constraints, self).map(drop)
    }
}
