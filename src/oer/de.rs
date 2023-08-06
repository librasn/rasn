use crate::prelude::{
    Any, BitString, BmpString, Constraints, Constructed, DecodeChoice, Enumerated, GeneralString,
    GeneralizedTime, Ia5String, Integer, NumericString, ObjectIdentifier, PrintableString, SetOf,
    TeletexString, UtcTime, VisibleString,
};
use crate::{Decode, Tag};
use num_bigint::BigInt;

/// ITU-T X.696 (02/2021) version of OER decoding
// In OER, without knowledge of the type of the value encoded, it is not possible to determine
// the structure of the encoding. In particular, the end of the encoding cannot be determined from
// the encoding itself without knowledge of the type being encoded ITU-T X.696 (6.2).
mod error;
pub use error::Error;
type InputSlice<'input> = nom_bitvec::BSlice<'input, u8, bitvec::order::Msb0>;

#[derive(Clone, Copy, Debug)]
pub struct DecoderOptions {
    // pub(crate) encoding_rules: EncodingRules,
}

pub struct Decoder<'input> {
    input: InputSlice<'input>,
    options: DecoderOptions,
}

impl<'input> Decoder<'input> {
    pub fn new(input: &'input crate::types::BitStr, options: DecoderOptions) -> Self {
        Self {
            input: input.into(),
            options,
        }
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = error::Error;

    fn decode_any(&mut self) -> Result<Any, Self::Error> {
        todo!()
    }

    fn decode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<BitString, Self::Error> {
        todo!()
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool, Self::Error> {
        todo!()
    }

    fn decode_enumerated<E: Enumerated>(&mut self, tag: Tag) -> Result<E, Self::Error> {
        todo!()
    }

    fn decode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Integer, Self::Error> {
        todo!()
    }

    fn decode_null(&mut self, tag: Tag) -> Result<(), Self::Error> {
        todo!()
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<ObjectIdentifier, Self::Error> {
        todo!()
    }

    fn decode_sequence<D, F>(&mut self, tag: Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        D: Constructed,
        F: FnOnce(&mut Self) -> Result<D, Self::Error>,
    {
        todo!()
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Vec<D>, Self::Error> {
        todo!()
    }

    fn decode_set_of<D: Decode + Ord>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<SetOf<D>, Self::Error> {
        todo!()
    }

    fn decode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<String, Self::Error> {
        todo!()
    }

    fn decode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<VisibleString, Self::Error> {
        todo!()
    }

    fn decode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<GeneralString, Self::Error> {
        todo!()
    }

    fn decode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Ia5String, Self::Error> {
        todo!()
    }

    fn decode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<PrintableString, Self::Error> {
        todo!()
    }

    fn decode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<NumericString, Self::Error> {
        todo!()
    }

    fn decode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<TeletexString, Self::Error> {
        todo!()
    }

    fn decode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<BmpString, Self::Error> {
        todo!()
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error> {
        todo!()
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<UtcTime, Self::Error> {
        todo!()
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<GeneralizedTime, Self::Error> {
        todo!()
    }

    fn decode_set<FIELDS, SET, D, F>(
        &mut self,
        tag: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + Constructed,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        todo!()
    }

    fn decode_choice<D>(&mut self, constraints: Constraints) -> Result<D, Self::Error>
    where
        D: DecodeChoice,
    {
        todo!()
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error> {
        todo!()
    }

    fn decode_optional_with_tag<D: Decode>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error> {
        todo!()
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        todo!()
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error> {
        todo!()
    }

    fn decode_extension_addition<D>(&mut self) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        todo!()
    }

    fn decode_extension_addition_group<D: Decode + Constructed>(
        &mut self,
    ) -> Result<Option<D>, Self::Error> {
        todo!()
    }
}
