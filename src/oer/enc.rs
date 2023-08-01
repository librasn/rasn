use crate::types::Constraints;

/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
mod config;
pub const ITU_T_X696_OER_EDITION: f32 = 3.0;

// ## HELPER FUNCTIONS start which should be refactored elsewhere

/// A convenience type around results needing to return one or many bytes.
enum ByteOrBytes {
    Single(u8),
    Many(Vec<u8>),
}

fn append_byte_or_bytes(output_vec: &mut Vec<u8>, bytes: ByteOrBytes) {
    match bytes {
        ByteOrBytes::Single(b) => output_vec.push(b),
        ByteOrBytes::Many(mut bs) => output_vec.append(&mut bs),
    }
}
// HELPER FUNCTIONS end

/// COER encoder. A subset of OER to provide canonical and unique encoding.  
pub struct Encoder {
    options: config::EncoderOptions,
    output: Vec<u8>,
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
    /// In OER any non-zero octet value represents true, but we support only canonical encoding.
    fn encode_bool(&mut self, value: bool) {
        self.output.push(if value { 0xffu8 } else { 0x00u8 });
        // append_byte_or_bytes(
        //     &mut self.output,
        //     if value {
        //         ByteOrBytes::Single(0xff)
        //     } else {
        //         ByteOrBytes::Single(0x00)
        //     },
        // );
    }
    fn encode_integer(&mut self, constraints: Constraints, value: i64) {}
}
