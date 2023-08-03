use crate::oer::enc::error::Error;
use crate::types::{constraints::*, BitString, Constraints, Integer};
use bitvec::prelude::*;

/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
mod config;
mod error;

pub const ITU_T_X696_OER_EDITION: f32 = 3.0;

// ## HELPER FUNCTIONS start which should be refactored elsewhere

/// A convenience type around results needing to return one or many bytes.
// enum ByteOrBytes {
//     Single(u8),
//     Many(Vec<u8>),
// }
//
// fn append_byte_or_bytes(output_vec: &mut Vec<u8>, bytes: ByteOrBytes) {
//     match bytes {
//         ByteOrBytes::Single(b) => output_vec.push(b),
//         ByteOrBytes::Many(mut bs) => output_vec.append(&mut bs),
//     }
// }
// HELPER FUNCTIONS end

/// COER encoder. A subset of OER to provide canonical and unique encoding.  
pub struct Encoder {
    options: config::EncoderOptions,
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
    pub fn new(options: config::EncoderOptions) -> Self {
        Self {
            options,
            output: <_>::default(),
        }
    }

    /// ITU-T X.696 9.
    /// False is encoded as a single zero octet. In COER, true is always encoded as 0xFF.
    /// In Basic-OER, any non-zero octet value represents true, but we support only canonical encoding.
    // fn encode_bool(&mut self, value: bool) {
    // self.output.push(if value { 0xffu8 } else { 0x00u8 });
    // append_byte_or_bytes(
    //     &mut self.output,
    //     if value {
    //         ByteOrBytes::Single(0xff)
    //     } else {
    //         ByteOrBytes::Single(0x00)
    //     },
    // );
    // }

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
            // Check if Integer is in constraint range
            // Constraint with extension leads ignoring the whole constraint in COER TODO decoding only?
            if !value.constraint.0.bigint_contains(value_to_enc) && value.extensible.is_none() {
                return Err(Error::IntegerOutOfRange {
                    value: value_to_enc.clone(),
                    expected: value.constraint.0,
                });
            }
            if let Bounded::Range { start, end } = value.constraint.0 {
                if let (Some(end), Some(start)) = (end, start) {
                    // Case a)
                    if start >= 0.into() {
                        let ranges: [i128; 5] = [
                            // encode as a fixed-size unsigned number in a one, two four or eight-octet word
                            // depending on the value of the upper bound
                            -1i128,
                            u8::MAX.into(),  // should be 1 octets
                            u16::MAX.into(), // should be 2 octets
                            u32::MAX.into(), // should be 4 octets
                            u64::MAX.into(), // should be 8 octets
                        ];
                        for (index, range) in ranges[0..(ranges.len() - 1)].iter().enumerate() {
                            if range < &end && end <= ranges[index + 1] {
                                let bytes = match value
                                    .constraint
                                    .0
                                    .effective_bigint_value(value_to_enc.clone())
                                {
                                    either::Left(offset) => {
                                        offset.to_biguint().unwrap().to_bytes_be()
                                    }
                                    either::Right(value) => value.to_signed_bytes_be(),
                                };
                                self.encode_non_negative_binary_integer(ranges[index + 1], &bytes)?;
                            }
                        }
                    }
                    // Case b)
                    else if start < 0.into() {
                        ();
                    }
                }

                // no lower bound
            }
        }
        Ok(())
    }

    /// When range constraints are present, the integer is encoded as a fixed-size unsigned number.
    /// This means that the zero padding is possible even with COER encoding.
    fn encode_non_negative_binary_integer(
        &mut self,
        octets: i128,
        bytes: &[u8],
    ) -> Result<(), Error> {
        use core::cmp::Ordering;
        let total_bits = crate::per::log2(octets) as usize;
        let bits = BitVec::<u8, Msb0>::from_slice(bytes);
        let bits = match total_bits.cmp(&bits.len()) {
            Ordering::Greater => {
                let mut padding = BitString::repeat(false, total_bits - bits.len());
                padding.extend(bits);
                padding
            }
            Ordering::Less => {
                return Err(Error::MoreOctetsThanExpected {
                    value: bits.len(),
                    expected: total_bits,
                })
            }
            Ordering::Equal => bits,
        };
        self.output.extend(bits);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        // let mut encoder = Encoder::new(super::config::EncoderOptions::coer());
        // encoder.encode_bool(true);
        // assert_eq!(encoder.output, vec![0xffu8]);
        // encoder.encode_bool(false);
        // assert_eq!(encoder.output, vec![0xffu8, 0x00u8]);
    }
    #[test]
    fn test_encode_integer_manual_setup() {
        let value_range = &[Constraint::Value(Extensible::new(Value::new(
            Bounded::Range {
                start: 0.into(),
                end: 255.into(),
            },
        )))];
        assert_eq!(
            0,
            *value_range[0]
                .as_value()
                .unwrap()
                .constraint
                .0
                .as_start()
                .unwrap()
        );
        assert_eq!(
            255,
            *value_range[0]
                .as_value()
                .unwrap()
                .constraint
                .0
                .as_end()
                .unwrap()
        );
        let consts = Constraints::new(value_range);
        let mut encoder = Encoder::new(config::EncoderOptions::coer());
        let result = encoder.encode_integer_with_constraints(&consts, &BigInt::from(244));
        assert!(!result.is_err());
        let v = vec![244u8];
        let bv = BitVec::<_, Msb0>::from_vec(v);
        assert_eq!(encoder.output, bv);
    }
}
