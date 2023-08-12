use crate::oer::utils;
use crate::prelude::{
    Any, BmpString, Choice, Constructed, Enumerated, GeneralString, GeneralizedTime, Ia5String,
    NumericString, PrintableString, SetOf, TeletexString, UtcTime, VisibleString,
};
use crate::types::{BitString, Constraints, Integer};
use crate::{Encode, Tag};
use alloc::{string::ToString, vec::Vec};
use bitvec::prelude::*;
/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
mod config;
pub use config::EncoderOptions;
mod error;
pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub const ITU_T_X696_OER_EDITION: f32 = 3.0;

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}
/// COER encoder. A subset of OER to provide canonical and unique encoding.  
pub struct Encoder {
    // options: EncoderOptions,
    output: BitString,
}
// ITU-T X.696 8.2.1 Only the following constraints are OER-visible:
// a) non-extensible single value constraints and value range constraints on integer types;
// b) non-extensible single value constraints on real types where the single value is either plus zero or minus zero or
// one of the special real values PLUS-INFINITY, MINUS-INFINITY and NOT-A-NUMBER;
// c) non-extensible size constraints on known-multiplier character string types, octetstring types, and bitstring
// types;
// d) non-extensible property settings constraints on the time type or on the useful and defined time types;
// e) inner type constraints applying OER-visible constraints to real types when used to restrict the mantissa, base,
// or exponent;
// f) inner type constraints applied to CHARACTER STRING or EMBEDDED-PDV types when used to restrict
// the value of the syntaxes component to a single value, or when used to restrict identification to the fixed
// alternative;
// g) contained subtype constraints in which the constraining type carries an OER-visible constraint.

// Tags are encoded only as part of the encoding of a choice type, where the tag indicates
// which alternative of the choice type is the chosen alternative (see 20.1).
impl Encoder {
    // pub fn new(options: EncoderOptions) -> Self {
    #[must_use]
    pub fn new() -> Self {
        Self {
            output: <BitString>::default(),
        }
    }

    #[must_use]
    pub fn output(&self) -> Vec<u8> {
        // TODO, move from per to utility module?
        crate::per::to_vec(&self.output)
    }
    /// ITU-T X.696 9.
    /// False is encoded as a single zero octet. In COER, true is always encoded as 0xFF.
    /// In Basic-OER, any non-zero octet value represents true, but we support only canonical encoding.
    fn encode_bool(&mut self, value: bool) {
        self.output
            .extend(BitVec::<u8, Msb0>::from_slice(&[if value {
                0xffu8
            } else {
                0x00u8
            }]));
    }

    /// Encode the length of the value to output.
    /// Length of the data `length` should not be provided as full bytes.
    /// In COER we try to use the shortest possible encoding, hence convert to the smallest integer type
    /// to avoid leading zeros.
    fn encode_length(&mut self, length: usize) -> Result<(), Error> {
        // Bits to byte length
        let length = if length % 8 == 0 {
            length / 8
        } else {
            Err(Error::LengthNotAsBitLength {
                value: length,
                remainder: length % 8,
            })?
        };
        let bytes: BitVec<u8, Msb0> = match length {
            v if u8::try_from(v).is_ok() => {
                BitVec::<u8, Msb0>::from_slice(&(length as u8).to_be_bytes())
            }
            v if u16::try_from(v).is_ok() => {
                BitVec::<u8, Msb0>::from_slice(&(length as u16).to_be_bytes())
            }
            v if u32::try_from(v).is_ok() => {
                BitVec::<u8, Msb0>::from_slice(&(length as u32).to_be_bytes())
            }
            v if u64::try_from(v).is_ok() => {
                BitVec::<u8, Msb0>::from_slice(&(length as u64).to_be_bytes())
            }
            _ => BitVec::<u8, Msb0>::from_slice(&(length as u128).to_be_bytes()),
        };
        if length < 128 {
            // First bit should be always zero when below 128: ITU-T X.696 8.6.4
            self.output.extend(&bytes);
            return Ok(());
        }
        let length_of_length = u8::try_from(bytes.len() / 8);
        if length_of_length.is_ok() && length_of_length.unwrap() > 127 {
            return Err(Error::TooLongValue {
                length: length as u128,
            });
        } else if length_of_length.is_ok() {
            self.output.extend(length_of_length.unwrap().to_be_bytes());
            // We must swap the first bit to show long form
            // It is always zero by default with u8 type when value being < 128
            _ = self.output.remove(0);
            self.output.insert(0, true);
            self.output.extend(bytes);
        } else {
            return Err(Error::Propagated {
                msg: length_of_length.err().unwrap().to_string(),
            });
        }
        Ok(())
    }
    /// Encode integer `value_to_enc` with length determinant
    /// Either as signed or unsigned, set by `signed`
    fn encode_unconstrained_integer(
        &mut self,
        value_to_enc: &Integer,
        signed: bool,
    ) -> Result<(), Error> {
        let bytes = if signed {
            BitVec::<u8, Msb0>::from_slice(&value_to_enc.to_signed_bytes_be())
        } else {
            BitVec::<u8, Msb0>::from_slice(&value_to_enc.to_biguint().unwrap().to_bytes_be())
        };
        self.encode_length(bytes.len())?;
        self.output.extend(bytes);
        Ok(())
    }

    /// Encode an integer value with constraints.
    ///
    /// Encoding depends on the range constraint, and has two scenarios.
    /// a) The effective value constraint has a lower bound, and that lower bound is zero or positive.
    /// b) The effective value constraint has either a negative lower bound or no lower bound.
    /// Other integer constraints are OER-invisible.
    /// Unlike PER, OER does not add an extension bit at the beginning of the encoding of an integer
    /// type with an extensible OER-visible constraint. Such a type is encoded as an integer type with no bounds.
    ///
    /// If the Integer is not bound or outside of range, we encode with the smallest number of octets possible.
    fn encode_integer_with_constraints(
        &mut self,
        constraints: &Constraints,
        value_to_enc: &Integer,
    ) -> Result<(), Error> {
        if let Some(value) = constraints.value() {
            if !value.constraint.0.bigint_contains(value_to_enc) && value.extensible.is_none() {
                return Err(Error::IntegerOutOfRange {
                    value: value_to_enc.clone(),
                    expected: value.constraint.0,
                });
            }
            return utils::determine_integer_size_and_sign(
                &value,
                value_to_enc,
                |value_to_enc, sign, octets| {
                    let bytes = if sign {
                        value_to_enc.to_signed_bytes_be()
                    } else {
                        value_to_enc.to_biguint().unwrap().to_bytes_be()
                    };
                    self.encode_integer_with_padding(octets.map(i128::from).unwrap(), &bytes)
                },
            );
        }
        self.encode_unconstrained_integer(value_to_enc, true)
    }

    /// When range constraints are present, the integer is encoded as a fixed-size unsigned number.
    /// This means that the zero padding is possible even with COER encoding.
    fn encode_integer_with_padding(&mut self, octets: i128, bytes: &[u8]) -> Result<(), Error> {
        use core::cmp::Ordering;
        if octets > 8 {
            return Err(Error::Custom {
                msg: alloc::format!("Unexpected constrained integer byte size: {octets}"),
            });
        }
        let bits = BitVec::<u8, Msb0>::from_slice(bytes);
        let bits = match (octets as usize * 8).cmp(&bits.len()) {
            Ordering::Greater => {
                let mut padding = BitString::repeat(false, octets as usize - bits.len());
                padding.extend(bits);
                padding
            }
            Ordering::Less => {
                return Err(Error::MoreBytesThanExpected {
                    value: bits.len(),
                    expected: octets as usize,
                })
            }
            Ordering::Equal => bits,
        };
        self.output.extend(bits);
        Ok(())
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = Error;

    fn encode_any(&mut self, _tag: Tag, value: &Any) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, _tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.encode_bool(value);
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        _tag: Tag,
        constraints: Constraints,
        value: &BitString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_object_identifier(&mut self, _: Tag, value: &[u32]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_integer(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &Integer,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_integer_with_constraints(&constraints, value)
    }

    fn encode_null(&mut self, _: Tag) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_general_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &GeneralString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utf8_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_visible_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_ia5_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_printable_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_numeric_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_teletex_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bmp_string(
        &mut self,
        _: Tag,
        constraints: Constraints,
        value: &BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_generalized_time(
        &mut self,
        _: Tag,
        value: &GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utc_time(&mut self, _tag: Tag, value: &UtcTime) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        _: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence<C, F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        todo!()
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_set<C, F>(&mut self, _tag: Tag, value: F) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed,
        F: FnOnce(&mut Self) -> Result<(), Self::Error>,
    {
        todo!()
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_choice<E: Encode + Choice>(
        &mut self,
        constraints: Constraints,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_extension_addition_group<E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + Constructed,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::constraints::{Bounded, Constraint, Constraints, Extensible, Value};
    use num_bigint::BigInt;

    // const ALPHABETS: &[u32] = &{
    //     let mut array = [0; 26];
    //     let mut i = 0;
    //     let mut start = 'a' as u32;
    //     let end = 'z' as u32;
    //     loop {
    //         array[i] = start;
    //         start += 1;
    //         i += 1;
    //
    //         if start > end {
    //             break;
    //         }
    //     }
    //
    //     array
    // };
    #[test]
    fn test_encode_bool() {
        let mut encoder = Encoder::new();
        encoder.encode_bool(true);
        let mut bv = BitVec::<u8, Msb0>::from_slice(&[0xffu8]);
        assert_eq!(encoder.output, bv);
        encoder.encode_bool(false);
        bv.append(&mut BitVec::<u8, Msb0>::from_slice(&[0x00u8]));
        assert_eq!(encoder.output, bv);
        assert_eq!(encoder.output.as_raw_slice(), &[0xffu8, 0]);
        // Use higher abstraction
        let decoded = crate::oer::encode(&true).unwrap();
        assert_eq!(decoded, &[0xffu8]);
        let decoded = crate::oer::encode(&false).unwrap();
        assert_eq!(decoded, &[0x0]);
    }
    #[test]
    fn test_encode_integer_manual_setup() {
        let range_bound = Bounded::<i128>::Range {
            start: 0.into(),
            end: 255.into(),
        };
        let value_range = &[Constraint::Value(Extensible::new(Value::new(range_bound)))];
        let consts = Constraints::new(value_range);
        let mut encoder = Encoder::default();
        let result = encoder.encode_integer_with_constraints(&consts, &BigInt::from(244));
        assert!(result.is_ok());
        let v = vec![244u8];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
        encoder.output.clear();
        let value = BigInt::from(256);
        let result = encoder.encode_integer_with_constraints(&consts, &value);
        // dbg!(result.as_ref().err());
        assert!(matches!(
            result,
            Err(Error::IntegerOutOfRange {
                value,
                expected: bound
            })
        ));
    }
    #[test]
    fn test_integer_with_length_determinant() {
        // Using defaults, no limits
        let constraints = Constraints::default();
        let mut encoder = Encoder::default();
        let result = encoder.encode_integer_with_constraints(&constraints, &BigInt::from(244));
        // dbg!(&result.err());
        assert!(result.is_ok());
        let v = vec![2u8, 0, 244];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
        encoder.output.clear();
        let result =
            encoder.encode_integer_with_constraints(&constraints, &BigInt::from(-1_234_567));
        assert!(result.is_ok());
        let v = vec![0x03u8, 0xED, 0x29, 0x79];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
    }
}
