//! Encoding Rust structures into Octet Encoding Rules data.

use alloc::vec::Vec;
use bitvec::prelude::*;
use num_traits::ToPrimitive;

use crate::{
    oer::EncodingRules,
    types::{
        Any, BitStr, BmpString, Choice, Constraints, Constructed, Date, Enumerated, GeneralString,
        GeneralizedTime, GraphicString, Ia5String, Identifier, IntegerType, NumericString,
        PrintableString, RealType, SetOf, Tag, TeletexString, UtcTime, VisibleString,
    },
    Codec, Encode,
};

/// ITU-T X.696 (02/2021) version of (C)OER encoding
/// On this crate, only canonical version will be used to provide unique and reproducible encodings.
/// Basic-OER is not supported and it might be that never will.
use crate::error::{CoerEncodeErrorKind, EncodeError, EncodeErrorKind};

/// The current supported edition of the ITU X.696 standard.
pub const ITU_T_X696_OER_EDITION: f32 = 3.0;

/// Options for configuring the [`Encoder`].
#[derive(Clone, Copy, Debug)]
pub struct EncoderOptions {
    encoding_rules: EncodingRules,
    set_encoding: bool,
}

impl EncoderOptions {
    /// Returns the default encoding rules options for [`EncodingRules::Coer`].
    #[must_use]
    pub const fn coer() -> Self {
        Self {
            encoding_rules: EncodingRules::Coer,
            set_encoding: false,
        }
    }
    fn without_set_encoding(mut self) -> Self {
        self.set_encoding = false;
        self
    }
    #[must_use]
    fn current_codec(self) -> Codec {
        match self.encoding_rules {
            EncodingRules::Oer => Codec::Oer,
            EncodingRules::Coer => Codec::Coer,
        }
    }
}

impl Default for EncoderOptions {
    fn default() -> Self {
        Self::coer()
    }
}

// Meta information about the current encoding state.
// Particularly related to encoding the presense of optional fields and extensions, and positioning
// encoded bytes in the buffer with a minimal amount of allocations.
#[derive(Debug, Clone, Copy)]
struct ConstructedCursor<const RC: usize, const EC: usize> {
    number_optional_default: usize,
    preamble_width: usize,
    preamble_cursor: usize,
    extension_missing_bits: u8,
    extension_bitfield_width: usize,
    extension_bitmap_width: usize,
    extension_bitmap_total_width: usize,
    extension_bitmap_cursor: usize,
}

impl<const RC: usize, const EC: usize> ConstructedCursor<RC, EC> {
    // See Section 16 in ITU-T X.696 (02/2021)
    const fn new(number_optional_default: usize, is_extensible: bool) -> Self {
        let preamble_missing_bits =
            (8 - ((is_extensible as usize + number_optional_default) & 7)) & 7;
        debug_assert!(
            (preamble_missing_bits + is_extensible as usize + number_optional_default)
                .is_multiple_of(8)
        );
        let preamble_width =
            (number_optional_default + is_extensible as usize + preamble_missing_bits) / 8;
        let extension_missing_bits: u8 =
            ((EC > 0) as u8).wrapping_neg() & ((8 - (EC & 7) as u8) & 7);
        debug_assert!((EC + extension_missing_bits as usize).is_multiple_of(8));
        let extension_bitfield_width = (EC + extension_missing_bits as usize) / 8;
        let extension_bitmap_width = 1 + extension_bitfield_width;
        let extension_bitmap_width_length = if extension_bitmap_width <= 127 {
            1 // Short form
        } else {
            1 + (extension_bitmap_width as u32).ilog2().div_ceil(8) as usize // Long form
        };
        let extension_bitmap_total_width = extension_bitmap_width + extension_bitmap_width_length;
        Self {
            number_optional_default,
            preamble_width,
            preamble_cursor: 0,
            extension_missing_bits,
            extension_bitfield_width,
            extension_bitmap_width,
            extension_bitmap_total_width,
            extension_bitmap_cursor: 0,
        }
    }
    fn set_extension_bitmap_cursor(&mut self, cursor: usize) {
        self.extension_bitmap_cursor = cursor;
    }
    fn set_preamble_cursor(&mut self, cursor: usize) {
        self.preamble_cursor = cursor;
    }
    const fn default() -> Self {
        Self {
            number_optional_default: 0,
            preamble_width: 0,
            preamble_cursor: 0,
            extension_bitfield_width: 0,
            extension_bitmap_width: 0,
            extension_missing_bits: 0,
            extension_bitmap_total_width: 0,
            extension_bitmap_cursor: 0,
        }
    }
}

/// COER encoder. A subset of OER to provide canonical and unique encoding.
///
/// Const `RCL` is the count of root components in the root component list of a sequence or set.
/// Const `ECL` is the count of extension additions in the extension addition component type list in a sequence or set.
#[derive(Debug)]
pub struct Encoder<'buffer, const RCL: usize = 0, const ECL: usize = 0> {
    options: EncoderOptions,
    output: &'buffer mut Vec<u8>,
    set_output: alloc::collections::BTreeMap<Tag, Vec<u8>>,
    is_extension_sequence: bool,
    root_bitfield: (usize, [(bool, Tag); RCL]),
    extension_bitfield: (usize, [bool; ECL]),
    // Tracks the position in the output buffer where the preamble and extension fields should/are encoded.
    cursor: ConstructedCursor<RCL, ECL>,
    // Sometimes we need to encode data into separate buffer before length can be calculated.
    // Using a separate buffer comes with a trade-off of reduced allocation count vs. peak memory usage.
    worker: &'buffer mut Vec<u8>,
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
impl<'buffer, const RCL: usize, const ECL: usize> Encoder<'buffer, RCL, ECL> {
    #[must_use]
    /// Constructs a new encoder from options and existing `buffer` and worker buffer.
    pub fn from_buffer(
        options: EncoderOptions,
        output: &'buffer mut Vec<u8>,
        worker: &'buffer mut Vec<u8>,
    ) -> Self {
        Self {
            options,
            output,
            set_output: <_>::default(),
            root_bitfield: (0, [(false, Tag::new_private(0)); RCL]),
            extension_bitfield: (0, [false; ECL]),
            cursor: ConstructedCursor::default(),
            is_extension_sequence: bool::default(),
            worker,
        }
    }

    fn codec(&self) -> Codec {
        self.options.current_codec()
    }

    /// Takes and returns the current output buffer, clearing the internal storage.
    #[must_use]
    pub fn output(&mut self) -> Vec<u8> {
        core::mem::take(self.output)
    }

    // `BTreeMap` is used to maintain the order of the fields in [SET], relying on the `Ord` trait of the [Tag] type.
    fn collect_set(&mut self) {
        self.output.append(
            self.set_output
                .values()
                .flatten()
                .copied()
                .collect::<Vec<u8>>()
                .as_mut(),
        );
    }

    /// Sets the presence of a `OPTIONAL` or `DEFAULT` field in the bitfield.
    /// The presence is ordered based on the field appearance order in the schema.
    fn set_presence(&mut self, tag: Tag, bit: bool) {
        // Applies only for SEQUENCE and SET types (RCL > 0)
        // Compiler should optimize this out otherwise
        if RCL > 0 {
            if self.cursor.number_optional_default < self.root_bitfield.0 + 1 {
                // Fields should be encoded in order
                // When the presence of optional extension field is set, we end up here
                // However, we don't need that information
                return;
            }
            self.root_bitfield.1[self.root_bitfield.0] = (bit, tag);
            self.root_bitfield.0 += 1;
        }
    }
    fn set_extension_presence(&mut self, bit: bool) {
        // Applies only for SEQUENCE and SET types (ECL > 0)
        // Compiler should optimize this out when not present
        if ECL > 0 {
            self.extension_bitfield.1[self.extension_bitfield.0] = bit;
            self.extension_bitfield.0 += 1;
        }
    }
    fn extend(&mut self, tag: Tag) {
        if self.options.set_encoding {
            // If not using mem::take here, remember to call output.clear() after encoding
            self.set_output.insert(tag, core::mem::take(self.output));
        }
    }
    /// Encode a tag as specified in ITU-T X.696 8.7
    ///
    /// Encoding of the tag is only required when encoding a choice type.
    fn encode_tag(&self, tag: Tag, bv: &mut BitSlice<u8, Msb0>) -> usize {
        use crate::types::Class;
        // Encode the tag class
        let mut index = 0;
        match tag.class {
            Class::Universal => {
                bv.set(index, false);
                bv.set(index + 1, false);
                index += 2;
            }
            Class::Application => {
                bv.set(index, false);
                bv.set(index + 1, true);
                index += 2;
            }
            Class::Context => {
                bv.set(index, true);
                bv.set(index + 1, false);
                index += 2;
            }
            Class::Private => {
                bv.set(index, true);
                bv.set(index + 1, true);
                index += 2;
            }
        }
        let mut tag_number = tag.value;
        // Encode the tag number
        if tag_number < 63 {
            for i in (0..6).rev() {
                bv.set(index, tag_number & (1 << i) != 0);
                index += 1;
            }
        } else {
            for i in 0..6 {
                bv.set(index + i, true);
            }
            index += 6;
            // Generate the bits for the tag number
            let mut tag_number_bits = 0;
            let mut temp_tag_number = tag_number;
            while temp_tag_number > 0 {
                temp_tag_number >>= 1;
                tag_number_bits += 1;
            }
            let mut remainer = 7 - tag_number_bits % 7;
            // Encode the bits in the "big-endian" format, with continuation bits
            bv.set(index, true);
            index += 1;
            // First, add leading zeros if needed to make length a multiple of 7
            for _ in 0..7 {
                if remainer != 0 {
                    bv.set(index, false);
                    index += 1;
                    remainer -= 1;
                    continue;
                }
                bv.set(index, tag_number & 1 != 0);
                tag_number >>= 1;
                index += 1;
            }
            while tag_number > 0 {
                // 8th bit is continuation marker; true for all but the last octet
                bv.set(index, true);
                index += 1;
                for _ in 0..7 {
                    bv.set(index, tag_number & 1 != 0);
                    tag_number >>= 1;
                    index += 1;
                }
            }
            // Correct the 8th bit of the last octet to be false
            let bv_last_8bit = &bv[..index].len() - 8;
            bv.replace(bv_last_8bit, false);
            debug_assert!(&bv[2..8].all());
            debug_assert!(&bv[9..16].any());
        }
        index
    }

    fn encode_unconstrained_enum_index(&mut self, value: isize) -> Result<(), EncodeError> {
        let (bytes, needed) = value.to_signed_bytes_be();
        let mut length = u8::try_from(needed).map_err(|err| {
            EncodeError::integer_type_conversion_failed(
                alloc::format!(
                    "Length of length conversion failed when encoding enumerated index.\
                value likely too large: {err}"
                ),
                self.codec(),
            )
        })?;
        if length > 127 {
            // There seems to be an error in standard. It states that enumerated index can be
            // between –2^1015 and 2^1015 – 1, but then it limits the amount of subsequent bytes to 127
            return Err(CoerEncodeErrorKind::TooLongValue {
                length: needed as u128,
            }
            .into());
        }
        // We must swap the first bit to show long form
        // It is always zero by default with u8 type when value being < 128
        length |= 0b_1000_0000;
        self.output.extend_from_slice(&length.to_be_bytes());
        self.output.extend_from_slice(&bytes.as_ref()[..needed]);
        Ok(())
    }
    /// Encode the length of the value to output.
    /// `Length` of the data should be provided as full bytes.
    ///
    /// COER tries to use the shortest possible encoding and avoids leading zeros.
    fn encode_length(buffer: &mut Vec<u8>, length: usize) -> Result<(), EncodeError> {
        let (bytes, needed) = length.to_unsigned_bytes_be();
        if length < 128 {
            // First bit should be always zero when below 128: ITU-T X.696 8.6.4
            buffer.extend_from_slice(&bytes.as_ref()[..needed]);
            return Ok(());
        }
        let mut length_of_length = u8::try_from(needed).map_err(|err| {
            EncodeError::integer_type_conversion_failed(
                alloc::format!("Length of length conversion failed: {err}"),
                Codec::Coer,
            )
        })?;
        if length_of_length > 127 {
            return Err(CoerEncodeErrorKind::TooLongValue {
                length: length as u128,
            }
            .into());
        }
        // We must swap the first bit to show long form
        // It is always zero by default with u8 type when value being < 128
        length_of_length |= 0b_1000_0000;
        buffer.extend_from_slice(&length_of_length.to_be_bytes());
        buffer.extend_from_slice(&bytes.as_ref()[..needed]);
        Ok(())
    }
    /// Encode integer `value_to_enc` with length determinant
    /// Either as signed or unsigned bytes, defined by `signed`
    fn encode_unconstrained_integer<I: IntegerType>(
        &mut self,
        value_to_enc: &I,
        signed: bool,
    ) -> Result<(), EncodeError> {
        if signed {
            let (bytes, needed) = value_to_enc.to_signed_bytes_be();
            Self::encode_length(self.output, needed)?;
            self.output.extend_from_slice(&bytes.as_ref()[..needed]);
        } else {
            let (bytes, needed) = value_to_enc.to_unsigned_bytes_be();
            Self::encode_length(self.output, needed)?;
            self.output.extend_from_slice(&bytes.as_ref()[..needed]);
        };
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
    fn encode_integer_with_constraints<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value_to_enc: &I,
    ) -> Result<(), EncodeError> {
        if let Some(value) = constraints.value() {
            if !value.constraint.value.in_bound(value_to_enc) && value.extensible.is_none() {
                return Err(EncodeError::value_constraint_not_satisfied(
                    value_to_enc.to_bigint().unwrap_or_default(),
                    &value.constraint.value,
                    self.codec(),
                ));
            }
            let (signed, octets) = if value.extensible.is_some() {
                (true, None)
            } else {
                (value.constraint.get_sign(), value.constraint.get_range())
            };
            if let Some(octets) = octets {
                self.encode_constrained_integer_with_padding(
                    usize::from(octets),
                    value_to_enc,
                    signed,
                )?;
            } else {
                self.encode_unconstrained_integer(value_to_enc, signed)?;
            }
        } else {
            self.encode_unconstrained_integer(value_to_enc, true)?;
        }
        self.extend(tag);
        Ok(())
    }

    /// When range constraints are present, the integer is encoded as a fixed-size number.
    /// This means that the zero padding is possible even with COER encoding.
    fn encode_constrained_integer_with_padding<I: IntegerType>(
        &mut self,
        octets: usize,
        value: &I,
        signed: bool,
    ) -> Result<(), EncodeError> {
        use core::cmp::Ordering;
        if octets > 8 {
            return Err(CoerEncodeErrorKind::InvalidConstrainedIntegerOctetSize.into());
        }
        let signed_ref;
        let unsigned_ref;
        let needed: usize;
        let bytes = if signed {
            (signed_ref, needed) = value.to_signed_bytes_be();
            signed_ref.as_ref()
        } else {
            (unsigned_ref, needed) = value.to_unsigned_bytes_be();
            unsigned_ref.as_ref()
        };

        match octets.cmp(&needed) {
            Ordering::Greater => {
                const PADDED_BYTES_NEG: [u8; 8] = [0xffu8; 8];
                const PADDED_BYTES_POS: [u8; 8] = [0x00u8; 8];
                // Branchless selection using array indexing
                let idx = usize::from(signed && value.is_negative());
                let padded_bytes = [&PADDED_BYTES_POS, &PADDED_BYTES_NEG][idx];
                self.output
                    .extend_from_slice(&padded_bytes[..octets - needed]);
            }
            Ordering::Less => {
                return Err(EncodeError::from_kind(
                    EncodeErrorKind::MoreBytesThanExpected {
                        value: needed,
                        expected: octets,
                    },
                    self.codec(),
                ));
            }
            // As is
            Ordering::Equal => {}
        };
        self.output.extend_from_slice(&bytes[..needed]);
        Ok(())
    }
    fn check_fixed_size_constraint(
        &self,
        length: usize,
        constraints: &Constraints,
    ) -> Result<bool, EncodeError> {
        if let Some(size) = constraints.size() {
            if !size.constraint.contains(&length) && size.extensible.is_none() {
                return Err(EncodeError::size_constraint_not_satisfied(
                    length,
                    &size.constraint,
                    self.codec(),
                ));
            }
            // Encode without length determinant
            if size.constraint.is_fixed() && size.extensible.is_none() {
                return Ok(true);
            }
        }
        // Prior checks before encoding with length determinant
        const MAX_PERMITTED_LENGTH: usize = usize::MAX / 8;
        if length > MAX_PERMITTED_LENGTH {
            return Err(EncodeError::length_exceeds_platform_size(self.codec()));
        }
        Ok(false)
    }

    // Reserve byte space for extension presence bitmap
    // if we do it early, we avoid most extra allocations
    fn extension_bitmap_reserve(&mut self) {
        self.cursor.set_extension_bitmap_cursor(self.output.len());
        self.output.extend(core::iter::repeat_n(
            0,
            self.cursor.extension_bitmap_total_width,
        ));
    }

    /// Encode a constructed type.`RC` is the number root components, `EC` is the number of extension components.
    /// `encoder` is the encoder for the constructed type that already includes the encoded values.
    fn encode_constructed<const RC: usize, const EC: usize, C: Constructed<RC, EC>>(
        &mut self,
        tag: Tag,
        set_output: Option<&mut alloc::collections::BTreeMap<Tag, Vec<u8>>>,
    ) -> Result<(), EncodeError> {
        // ### PREAMBLE ###
        // Section 16.2.2
        let mut preamble = BitArray::<[u8; RC], Msb0>::default();
        let mut preamble_index = 0;
        let mut extensions_present = false;
        if C::IS_EXTENSIBLE {
            extensions_present = self.extension_bitfield.1.iter().any(|b| *b);
            // In case we have no any components in the root component list, we need to set extension present bit with other means later on
            if RC > 0 {
                preamble.set(0, extensions_present);
                preamble_index += 1;
            }
        }
        // Section 16.2.3
        let (needed, option_bitfield) = if self.options.set_encoding {
            // In set encoding, tags must be unique so we just sort them to be in canonical order for preamble
            self.root_bitfield
                .1
                .sort_by(|(_, tag1), (_, tag2)| tag1.const_cmp(tag2));
            self.root_bitfield
        } else {
            self.root_bitfield
        };
        debug_assert!(C::FIELDS.number_of_optional_and_default_fields() == needed);
        for (bit, _tag) in &option_bitfield[..needed] {
            preamble.set(preamble_index, *bit);
            preamble_index += 1;
        }
        // 16.2.4 - fill missing bits from full octet with zeros
        // Whether we need preamble
        if needed > 0 || C::IS_EXTENSIBLE {
            // `.as_raw_slice` seems to be faster than `BitSlice::domain()`
            if RC == 0 && C::IS_EXTENSIBLE {
                self.output.push(u8::from(extensions_present) << 7);
            } else {
                // replace reserved preamble position with correct values, starting from preamble_start index
                if self.options.set_encoding {
                    self.output
                        .extend_from_slice(&preamble.as_raw_slice()[..self.cursor.preamble_width]);
                } else {
                    self.output[self.cursor.preamble_cursor
                        ..self.cursor.preamble_cursor + self.cursor.preamble_width]
                        .copy_from_slice(&preamble.as_raw_slice()[..self.cursor.preamble_width]);
                }
            }
        }
        // Section 16.3 ### Encodings of the components in the extension root ###
        if !C::IS_EXTENSIBLE || !extensions_present {
            if let Some(set_output) = set_output {
                set_output.insert(tag, core::mem::take(self.output));
            }
            return Ok(());
        }

        // Section 16.4 ### Extension addition presence bitmap ###

        // Extension cursor cannot be 0 - preamble byte takes at least 1 byte if extensions are defined
        debug_assert_ne!(self.cursor.extension_bitmap_cursor, 0);
        // We have pre-reserved space for the extension bitmap
        // Replace bytes
        let mut extension_bitmap_buffer: BitArray<[u8; EC], Msb0> = BitArray::default();
        Self::encode_length(self.worker, self.cursor.extension_bitmap_width)?;
        let mut cursor = self.cursor.extension_bitmap_cursor + self.worker.len();
        self.output[self.cursor.extension_bitmap_cursor..cursor].copy_from_slice(self.worker);
        self.worker.clear();
        self.output[cursor..=cursor]
            .copy_from_slice(&self.cursor.extension_missing_bits.to_be_bytes());
        cursor += 1;
        for (i, bit) in self.extension_bitfield.1.iter().enumerate() {
            extension_bitmap_buffer.set(i, *bit);
        }
        // The size of EC is always at least 1 byte if extensions present, so full octet will always fit
        self.output[cursor..cursor + self.cursor.extension_bitfield_width].copy_from_slice(
            &extension_bitmap_buffer.as_raw_slice()[..self.cursor.extension_bitfield_width],
        );

        // NOTE: Length for open type has been already added when encoding extension additions
        // NOTE: Extension data is already in the buffer in correct place

        // Encoding inside of set...
        if let Some(set_output) = set_output {
            set_output.insert(tag, core::mem::take(self.output));
        }
        Ok(())
    }
}

impl<'buffer, const RFC: usize, const EFC: usize> crate::Encoder<'buffer>
    for Encoder<'buffer, RFC, EFC>
{
    type Ok = ();
    type Error = EncodeError;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder<'this, R, E>;

    fn codec(&self) -> Codec {
        self.options.current_codec()
    }

    fn encode_any(
        &mut self,
        tag: Tag,
        value: &Any,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            <Constraints>::default(),
            &value.contents,
            Identifier::EMPTY,
        )
    }

    /// ITU-T X.696 9.
    /// False is encoded as a single zero octet. In COER, true is always encoded as 0xFF.
    /// In Basic-OER, any non-zero octet value represents true, but we support only canonical encoding.
    fn encode_bool(
        &mut self,
        tag: Tag,
        value: bool,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.output
            .extend_from_slice(if value { &[0xffu8] } else { &[0x00u8] });
        self.extend(tag);
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitStr,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // TODO When Rec. ITU-T X.680 | ISO/IEC 8824-1, 22.7 applies (i.e., the bitstring type is defined with a
        // "NamedBitList"), the bitstring value shall be encoded with trailing 0 bits added or removed as necessary to satisfy the
        // effective size constraint.
        // Rasn does not currently support NamedBitList
        let mut bit_string_encoding = BitVec::<u8, Msb0>::with_capacity(value.len());

        if let Some(size) = constraints.size() {
            // Constraints apply only if the lower and upper bounds
            // of the effective size constraint are identical (13.1)
            if size.constraint.is_fixed() && size.extensible.is_none() {
                //  Series of octets will be empty, allowed as 13.2.3,
                // but this seems impossible to implement for decoder so not supported
                // Can't say if this is an error in standard or not when not handling NamedBitList
                // if value.is_empty() {
                // } else
                if size.constraint.contains(&value.len()) {
                    let missing_bits: usize = (8 - (value.len() & 7)) & 7;
                    bit_string_encoding.extend(value);
                    if missing_bits > 0 {
                        bit_string_encoding.extend(core::iter::repeat_n(false, missing_bits));
                    }
                    self.output
                        .extend_from_slice(bit_string_encoding.as_raw_slice());
                } else {
                    return Err(EncodeError::size_constraint_not_satisfied(
                        value.len(),
                        &size.constraint,
                        self.codec(),
                    ));
                }
                self.extend(tag);
                return Ok(());
            }
        }

        // If the BitString is empty, length is one and initial octet is zero
        if value.is_empty() {
            Self::encode_length(self.output, 1)?;
            self.output.extend_from_slice(&[0x00u8]);
        } else {
            // TODO 22.7 X.680, NamedBitString and COER
            // if self.options.encoding_rules.is_coer()
            //     && value.trailing_zeros() > 7
            // {
            //     if value.first_one().is_some() {
            //         // In COER, we strip trailing zeros if they take full octets
            //         let trimmed_value = &value[..(value.len() - value.trailing_zeros() - 1)];
            //     }
            //     else {  }
            // }
            // With length determinant
            let missing_bits: usize = (8 - value.len() % 8) % 8;
            let trailing = [false; 8];
            let trailing = &trailing[..missing_bits];
            // missing bits never > 8
            bit_string_encoding.extend(missing_bits.to_u8().unwrap_or(0).to_be_bytes());
            bit_string_encoding.extend(value);
            bit_string_encoding.extend(trailing);
            Self::encode_length(self.output, bit_string_encoding.len() / 8)?;
            self.output
                .extend_from_slice(bit_string_encoding.as_raw_slice());
        }
        self.extend(tag);
        Ok(())
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // 11.5 The presence of an extension marker in the definition of an enumerated type does not affect the encoding of
        // the values of the enumerated type.
        // max size for enumerated value is currently only isize MIN/MAX
        // Spec allows between –2^1015 and 2^1015 – 1
        let number = value.discriminant();
        if 0isize <= number && number <= i8::MAX.into() {
            self.encode_constrained_integer_with_padding(1, &number, false)?;
        } else {
            // Value is signed here as defined in section 11.4
            // Long form but different from regular length determinant encoding
            self.encode_unconstrained_enum_index(number)?;
        }
        self.extend(tag);
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        value: &[u32],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::ber());
        let mut octets = enc.object_identifier_as_bytes(value)?;
        Self::encode_length(self.output, octets.len())?;
        self.output.append(&mut octets);
        self.extend(tag);
        Ok(())
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &I,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_integer_with_constraints(tag, &constraints, value)
    }

    fn encode_real<R: RealType>(
        &mut self,
        tag: Tag,
        _constraints: Constraints,
        value: &R,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let (bytes, len) = value.to_ieee754_bytes();
        self.output.extend_from_slice(&bytes.as_ref()[..len]);
        self.extend(tag);

        Ok(())
    }

    fn encode_null(&mut self, _tag: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if self.check_fixed_size_constraint(value.len(), &constraints)? {
            self.output.extend_from_slice(value);
        } else {
            // Use length determinant on other cases
            Self::encode_length(self.output, value.len())?;
            self.output.extend_from_slice(value);
        }
        self.extend(tag);
        Ok(())
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &GeneralString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // Seems like it can be encoded as it is...
        self.encode_octet_string(tag, constraints, value, Identifier::EMPTY)
    }

    fn encode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &GraphicString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // Seems like it can be encoded as it is...
        self.encode_octet_string(tag, constraints, value, Identifier::EMPTY)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &str,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, value.as_bytes(), Identifier::EMPTY)
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &VisibleString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, value.as_iso646_bytes(), Identifier::EMPTY)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &Ia5String,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, value.as_iso646_bytes(), Identifier::EMPTY)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &PrintableString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, value.as_bytes(), Identifier::EMPTY)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &NumericString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, value.as_bytes(), Identifier::EMPTY)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &TeletexString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // X.690 8.23.5
        // TODO the octets specified in ISO/IEC 2022 for encodings in an 8-bit environment, using
        // the escape sequence and character codings registered in accordance with ISO/IEC 2375.
        self.encode_octet_string(tag, constraints, &value.to_bytes(), Identifier::EMPTY)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BmpString,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, constraints, &value.to_bytes(), Identifier::EMPTY)
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &GeneralizedTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::enc::Encoder::datetime_to_canonical_generalized_time_bytes(value),
            Identifier::EMPTY,
        )
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &UtcTime,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::enc::Encoder::datetime_to_canonical_utc_time_bytes(value),
            Identifier::EMPTY,
        )
    }

    fn encode_date(
        &mut self,
        tag: Tag,
        value: &Date,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(
            tag,
            Constraints::default(),
            &crate::der::enc::Encoder::naivedate_to_date_bytes(value),
            Identifier::EMPTY,
        )
    }
    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // Whether we have a choice type being encoded
        if V::IS_CHOICE {
            value.encode(self)
        } else {
            value.encode_with_tag(self, tag)
        }
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let mut encoder = Encoder::<'_, RL, EL>::from_buffer(
            self.options.without_set_encoding(),
            self.output,
            self.worker,
        );
        let mut cursor = ConstructedCursor::<RL, EL>::new(
            C::FIELDS.number_of_optional_and_default_fields(),
            C::IS_EXTENSIBLE,
        );
        cursor.set_preamble_cursor(encoder.output.len());
        // reserve bytes for preamble
        for _ in 0..cursor.preamble_width {
            encoder.output.push(0);
        }

        encoder.cursor = cursor;
        encoder_scope(&mut encoder)?;
        if self.options.set_encoding {
            encoder.encode_constructed::<RL, EL, C>(tag, Some(&mut self.set_output))?;
        } else {
            encoder.encode_constructed::<RL, EL, C>(tag, None)?;
        }
        Ok(())
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        _: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // It seems that constraints here are not C/OER visible? No mention in standard...
        self.encode_unconstrained_integer(&value.len(), false)?;
        self.output.reserve(core::mem::size_of_val(value));

        let mut encoder = Encoder::<0>::from_buffer(self.options, self.output, self.worker);
        {
            for one in value {
                E::encode(one, &mut encoder)?;
            }
        }
        self.extend(tag);
        Ok(())
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), Self::Error>,
    {
        let mut options = self.options;
        options.set_encoding = true;
        let mut encoder = Encoder::<RL, EL>::from_buffer(options, self.output, self.worker);
        let cursor = ConstructedCursor::<RL, EL>::new(
            C::FIELDS.number_of_optional_and_default_fields(),
            C::IS_EXTENSIBLE,
        );
        encoder.cursor = cursor;
        encoder_scope(&mut encoder)?;
        if self.options.set_encoding {
            encoder.encode_constructed::<RL, EL, C>(tag, Some(&mut self.set_output))?;
        } else {
            encoder.encode_constructed::<RL, EL, C>(tag, None)?;
        }
        encoder.collect_set();
        Ok(())
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        value: &SetOf<E>,
        constraints: Constraints,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &value.to_vec(), constraints, Identifier::EMPTY)
    }

    fn encode_some<E: Encode>(
        &mut self,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, true);
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, true);
        value.encode_with_tag_and_constraints(self, tag, constraints, Identifier::EMPTY)
    }

    fn encode_none<E: Encode>(&mut self, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.set_presence(E::TAG, false);
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag, _: Identifier) -> Result<Self::Ok, Self::Error> {
        self.set_presence(tag, false);
        Ok(())
    }

    fn encode_choice<E: Encode + Choice>(
        &mut self,
        _: Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // Encode tag
        let mut tag_buffer: BitArray<[u8; core::mem::size_of::<Tag>() + 1], Msb0> =
            BitArray::default();
        let needed = self.encode_tag(tag, tag_buffer.as_mut_bitslice());
        self.output
            .extend_from_slice(&tag_buffer.as_raw_slice()[..(needed / 8)]);

        let buffer_end = self.output.len();
        // Encode the value
        let _tag = encode_fn(self)?;
        debug_assert_eq!(_tag, tag);
        let is_root_extension = crate::types::TagTree::tag_contains(&tag, E::VARIANTS);
        if is_root_extension {
            // all good, correct data in the buffer already
        } else {
            // Extension with length determinant
            // Unfortunatelly, we really cannot avoid extra allocating here
            // We don't know the length of the data length before encoding
            self.worker.append(&mut self.output.split_off(buffer_end));
            Self::encode_length(self.output, self.worker.len())?;
            self.output.append(self.worker);
        }
        self.extend(tag);
        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // let buffer_end = self.output.len();
        if value.is_present() {
            if self.cursor.extension_bitmap_cursor == 0 {
                self.extension_bitmap_reserve();
            }
            let cursor = self.output.len();
            // Since we need to calculate the unknown length of the data length, data must be encoded at first
            // Swap the buffers here to avoid extra alloactions
            // Also helps us on playing with mutability checks...
            let mut encoder = Encoder::<0>::from_buffer(self.options, self.worker, self.output);
            E::encode_with_tag_and_constraints(
                &value,
                &mut encoder,
                tag,
                constraints,
                Identifier::EMPTY,
            )?;
            // Truncate the actual output buffer to the original state
            encoder.worker.truncate(cursor);
            Self::encode_length(encoder.worker, encoder.output.len())?;
            self.output.append(self.worker);
            self.set_extension_presence(true);
        } else {
            self.set_extension_presence(false);
        }
        Ok(())
    }
    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        value: Option<&E>,
        _: Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + Constructed<RL, EL>,
    {
        let Some(value) = value else {
            self.set_extension_presence(false);
            return Ok(());
        };
        if self.cursor.extension_bitmap_cursor == 0 {
            self.extension_bitmap_reserve();
        }
        self.is_extension_sequence = true;
        // Since we need to calculate the unknown length of the data length, data must be encoded at first
        // Swap the buffers here to avoid extra alloactions
        // Also helps us on playing with mutability checks...
        let cursor = self.output.len();
        let mut encoder = Encoder::<0>::from_buffer(self.options, self.worker, self.output);
        value.encode(&mut encoder)?;
        // Truncate the actual output buffer to the original state
        encoder.worker.truncate(cursor);
        self.is_extension_sequence = false;
        Self::encode_length(encoder.worker, encoder.output.len())?;
        self.output.append(self.worker);
        self.set_extension_presence(true);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use super::*;
    use crate::macros::{constraints, value_constraint};
    use crate::prelude::{AsnType, Decode, Encode};

    #[test]
    fn test_encode_bool() {
        let output = crate::coer::encode(&true).unwrap();
        let bv = BitVec::<u8, Msb0>::from_slice(&[0xffu8]);
        assert_eq!(output, bv.as_raw_slice());
        let output = crate::coer::encode(&false).unwrap();
        let bv = BitVec::<u8, Msb0>::from_slice(&[0x00u8]);
        assert_eq!(output, bv.as_raw_slice());
        let decoded = crate::coer::encode(&true).unwrap();
        assert_eq!(decoded, &[0xffu8]);
        let decoded = crate::coer::encode(&false).unwrap();
        assert_eq!(decoded, &[0x0]);
    }
    #[test]
    fn test_encode_integer_manual_setup() {
        const CONSTRAINT_1: Constraints = constraints!(value_constraint!(0, 255));
        let mut buffer = vec![];
        let mut wb = vec![];
        let mut encoder =
            Encoder::<0, 0>::from_buffer(EncoderOptions::coer(), &mut buffer, &mut wb);
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &CONSTRAINT_1, &244);
        assert!(result.is_ok());
        let v = vec![244u8];
        assert_eq!(encoder.output.clone(), v);
        encoder.output.clear();
        let value = BigInt::from(256);
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &CONSTRAINT_1, &value);
        assert!(result.is_err());
    }
    #[test]
    fn test_integer_with_length_determinant() {
        // Using defaults, no limits
        let constraints = Constraints::default();
        let mut buffer = vec![];
        let mut wb = vec![];
        let mut encoder =
            Encoder::<0, 0>::from_buffer(EncoderOptions::coer(), &mut buffer, &mut wb);
        let result =
            encoder.encode_integer_with_constraints(Tag::INTEGER, &constraints, &BigInt::from(244));
        assert!(result.is_ok());
        let v = vec![2u8, 0, 244];
        assert_eq!(encoder.output.to_vec(), v);
        encoder.output.clear();
        let result = encoder.encode_integer_with_constraints(
            Tag::INTEGER,
            &constraints,
            &BigInt::from(-1_234_567),
        );
        assert!(result.is_ok());
        let v = vec![0x03u8, 0xED, 0x29, 0x79];
        assert_eq!(encoder.output.to_vec(), v);
    }
    #[test]
    fn test_large_lengths() {
        let constraints = Constraints::default();
        let mut buffer = vec![];
        let mut wb = vec![];
        let mut encoder =
            Encoder::<0, 0>::from_buffer(EncoderOptions::coer(), &mut buffer, &mut wb);

        // Signed integer with byte length of 128
        // Needs long form to represent
        let number: BigInt = BigInt::from(256).pow(127) - 1;
        let result = encoder.encode_integer_with_constraints(Tag::INTEGER, &constraints, &number);
        assert!(result.is_ok());
        let vc = [
            0x81, 0x80, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ];
        assert_eq!(encoder.output(), vc);
    }
    #[test]
    fn test_choice() {
        use crate as rasn;
        use crate::types::Integer;
        #[derive(AsnType, Decode, Debug, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        enum Choice {
            Normal(Integer),
            High(Integer),
            #[rasn(extension_addition)]
            Medium(Integer),
        }
        let mut buffer = vec![];
        let mut wb = vec![];
        let mut encoder =
            Encoder::<0, 0>::from_buffer(EncoderOptions::coer(), &mut buffer, &mut wb);

        let choice = Choice::Normal(333.into());
        choice.encode(&mut encoder).unwrap();

        assert_eq!(encoder.output(), &[128, 2, 1, 77]);
    }
}
