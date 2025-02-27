use crate::error::DecodeError;
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
///
/// IMPORTANT: While N describes the number of bits, also the internal size is for N amount of bytes.
/// When accessing the bits, use &array[..N] to get the correct bits, for example.
/// Also when setting the bits, you should use the big-endian ordering. See example.
/// Constraints are checked for N amount of bits and encoding operation will drop extra bits since the underlying container is larger.
///
/// # Example
/// ```rust
/// use rasn::prelude::*;
/// use bitvec::prelude::*;
///
/// let bool_array = [true, false, true];
/// let mut bit_array: FixedBitString<3> = BitArray::ZERO;
/// for (i, &value) in bool_array.iter().enumerate() {
///    bit_array.set(i, value);
/// }
/// // Also works: (note that the first byte can hold the whole bit array, since N = 3)
/// let second_array = FixedBitString::<3>::new([0b10100000, 0, 0]);
/// assert_eq!(bit_array, second_array);
/// ```
pub type FixedBitString<const N: usize> = bitvec::array::BitArray<[u8; N], bitvec::order::Msb0>;

///  A reference to a `BIT STRING` type.
pub type BitStr = bitvec::slice::BitSlice<u8, bitvec::order::Msb0>;

impl AsnType for BitString {
    const TAG: Tag = Tag::BIT_STRING;
    const IDENTIFIER: Option<&'static str> = Some("BIT_STRING");
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
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_bit_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl AsnType for BitStr {
    const TAG: Tag = Tag::BIT_STRING;
    const IDENTIFIER: Option<&'static str> = Some("BIT_STRING");
}

impl Encode for BitStr {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_bit_string(tag, constraints, self, identifier)
            .map(drop)
    }
}

impl<const N: usize> AsnType for FixedBitString<N> {
    const TAG: Tag = Tag::BIT_STRING;
    const CONSTRAINTS: Constraints = constraints!(size_constraint!(N));
    const IDENTIFIER: Option<&'static str> = Some("BIT_STRING");
}

impl<const N: usize> Decode for FixedBitString<N> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        let out = decoder.decode_bit_string(tag, constraints)?;
        if out.len() != N {
            return Err(D::Error::from(DecodeError::fixed_string_conversion_failed(
                Tag::BIT_STRING,
                out.len(),
                N,
                decoder.codec(),
            )));
        }
        let mut array = Self::ZERO;
        array[..out.len()].copy_from_bitslice(&out);
        Ok(array)
    }
}

impl<const N: usize> Encode for FixedBitString<N> {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Option<&'static str>,
    ) -> Result<(), E::Error> {
        encoder
            .encode_bit_string(tag, constraints, &self[..N], identifier)
            .map(drop)
    }
}
