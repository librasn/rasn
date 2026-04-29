//! Encoding Rust structures into Basic Encoding Rules data.

mod config;

use alloc::{borrow::ToOwned, collections::VecDeque, string::ToString, vec::Vec};
use chrono::Timelike;

use super::Identifier;
use crate::{
    Codec, Encode,
    types::{
        self, Constraints, Enumerated, IntegerType, Tag,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
    },
};

pub use crate::error::{BerEncodeErrorKind, EncodeError, EncodeErrorKind};
pub use config::EncoderOptions;

const START_OF_CONTENTS: u8 = 0x80;
const END_OF_CONTENTS: &[u8] = &[0, 0];

/// Encodes an object identifier into `buffer` in BER format.
/// Reusable by other codecs without constructing an [`Encoder`].
pub fn object_identifier_as_bytes(oid: &[u32], buffer: &mut Vec<u8>) -> Result<(), EncodeError> {
    if oid.len() < 2 {
        return Err(BerEncodeErrorKind::invalid_object_identifier(oid.to_owned()).into());
    }

    let first = oid[0];
    let second = oid[1];

    if first > MAX_OID_FIRST_OCTET {
        return Err(BerEncodeErrorKind::invalid_object_identifier(oid.to_owned()).into());
    }
    encode_as_base128((first * (MAX_OID_SECOND_OCTET + 1)) + second, buffer);
    for component in oid.iter().skip(2) {
        encode_as_base128(*component, buffer);
    }
    Ok(())
}

pub(super) fn encode_as_base128(number: u32, buffer: &mut Vec<u8>) {
    const WIDTH: u8 = 7;
    const SEVEN_BITS: u8 = 0x7F;
    const EIGHTH_BIT: u8 = 0x80;

    if number < EIGHTH_BIT as u32 {
        buffer.push(number as u8);
    } else {
        let mut n: u8;
        let mut bits_left = 35;
        let mut cont = false;
        while bits_left > 0 {
            bits_left -= WIDTH;
            n = ((number >> bits_left) as u8) & SEVEN_BITS;
            if n > 0 || cont {
                buffer.push(if bits_left > 0 { EIGHTH_BIT } else { 0 } | (n & SEVEN_BITS));
                cont = true;
            }
        }
    }
}

/// Encodes Rust structures into Basic Encoding Rules data.
pub struct Encoder {
    output: Vec<u8>,
    config: EncoderOptions,
    is_set_encoding: bool,
    set_buffer: alloc::collections::BTreeMap<Tag, Vec<u8>>,
    /// Spare allocation recycled from the most-recently-completed child encoder,
    /// so that sibling fields at the same nesting level reuse one heap allocation
    /// rather than allocating a fresh `Vec<u8>` per field.
    worker: Vec<u8>,
}

/// A convenience type around results needing to return one or many bytes.
enum ByteOrBytes {
    Single(u8),
    Many(Vec<u8>),
}

impl Encoder {
    /// Creates a new instance from the given `config`.
    #[must_use]
    pub fn new(config: EncoderOptions) -> Self {
        Self {
            config,
            is_set_encoding: false,
            output: <_>::default(),
            set_buffer: <_>::default(),
            worker: <_>::default(),
        }
    }

    /// Returns the currently selected codec.
    #[must_use]
    pub fn codec(&self) -> crate::Codec {
        self.config.current_codec()
    }

    /// Creates a new instance from the given `config`, and uses SET encoding
    /// logic, ensuring that all messages are encoded in order by tag.
    #[must_use]
    pub fn new_set(config: EncoderOptions) -> Self {
        Self {
            config,
            is_set_encoding: true,
            output: <_>::default(),
            set_buffer: <_>::default(),
            worker: <_>::default(),
        }
    }

    /// Creates a new instance from the given `config` and a user-supplied
    /// `Vec<u8>` buffer. This allows reuse of an existing buffer instead of
    /// allocating a new encoding buffer each time an [`Encoder`] is created.
    /// The buffer will be cleared before use.
    #[must_use]
    pub fn new_with_buffer(config: EncoderOptions, mut buffer: Vec<u8>) -> Self {
        buffer.clear();
        Self {
            output: buffer,
            config,
            is_set_encoding: false,
            set_buffer: <_>::default(),
            worker: <_>::default(),
        }
    }

    /// Consumes the encoder and returns the output of the encoding.
    #[must_use]
    pub fn output(self) -> Vec<u8> {
        if self.is_set_encoding {
            self.set_buffer
                .into_values()
                .fold(Vec::new(), |mut acc, mut field| {
                    acc.append(&mut field);
                    acc
                })
        } else {
            self.output
        }
    }

    fn append_byte_or_bytes(&mut self, bytes: ByteOrBytes) {
        match bytes {
            ByteOrBytes::Single(b) => self.output.push(b),
            ByteOrBytes::Many(mut bs) => self.output.append(&mut bs),
        }
    }

    /// Encodes the identifier of a type in BER/CER/DER. An identifier consists
    /// of a "class", encoding bit, and tag number. If our tag number is
    /// greater than 30 we to encode the number as stream of a 7 bit integers
    /// in big endian delimited by the leading bit of each byte.
    ///
    /// ```text
    /// ---------------------------------
    /// | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    /// ---------------------------------
    /// | class | E |        Tag        |
    /// ---------------------------------
    /// ```
    fn encode_identifier(
        &mut self,
        Identifier {
            tag,
            is_constructed,
        }: Identifier,
    ) -> ByteOrBytes {
        const FIVE_BITS: u32 = (1 << 5) - 1;
        let mut tag_byte = tag.class as u8;
        let tag_number = tag.value;

        // Constructed is a single bit.
        tag_byte <<= 1;
        tag_byte |= match tag {
            Tag::EXTERNAL | Tag::SEQUENCE | Tag::SET => 1,
            _ if is_constructed => 1,
            _ => 0,
        };

        tag_byte <<= 5;

        if tag_number >= FIVE_BITS {
            let mut buffer = alloc::vec![tag_byte | FIVE_BITS as u8];
            encode_as_base128(tag_number, &mut buffer);
            ByteOrBytes::Many(buffer)
        } else {
            tag_byte |= tag_number as u8;
            ByteOrBytes::Single(tag_byte)
        }
    }

    fn encode_length(&mut self, identifier: Identifier, value: &[u8]) {
        if identifier.is_primitive() || !self.config.encoding_rules.is_cer() {
            let len_bytes = self.encode_definite_length(value.len());
            self.append_byte_or_bytes(len_bytes);
            self.output.extend_from_slice(value);
        } else {
            self.output.push(START_OF_CONTENTS);
            self.output.extend_from_slice(value);
            self.output.extend_from_slice(END_OF_CONTENTS);
        }
    }

    fn encode_definite_length(&mut self, len: usize) -> ByteOrBytes {
        if len <= 127 {
            #[allow(clippy::cast_possible_truncation)]
            ByteOrBytes::Single(len as u8)
        } else {
            let mut length = len;
            let mut length_buffer = VecDeque::new();

            while length != 0 {
                length_buffer.push_front((length & 0xff) as u8);
                length >>= 8;
            }

            length_buffer.push_front(length_buffer.len() as u8 | 0x80);

            ByteOrBytes::Many(length_buffer.into())
        }
    }

    fn encode_octet_string_(&mut self, tag: Tag, value: &[u8]) -> Result<(), EncodeError> {
        self.encode_string(tag, Tag::OCTET_STRING, value)
    }

    /// "STRING" types in ASN.1 BER (OCTET STRING, UTF8 STRING) are either
    /// primitive encoded, or in certain variants like CER they are constructed
    /// encoded containing primitive encoded chunks.
    fn encode_string(
        &mut self,
        tag: Tag,
        nested_tag: Tag,
        value: &[u8],
    ) -> Result<(), EncodeError> {
        let max_string_length = self.config.encoding_rules.max_string_length();

        if value.len() > max_string_length {
            let ident_bytes = self.encode_identifier(Identifier::from_tag(tag, true));
            self.append_byte_or_bytes(ident_bytes);

            self.output.push(START_OF_CONTENTS);

            for chunk in value.chunks(max_string_length) {
                self.encode_primitive(nested_tag, chunk);
            }

            self.output.extend_from_slice(END_OF_CONTENTS);
            self.encode_to_set(tag);
        } else {
            self.encode_primitive(tag, value);
        }

        Ok(())
    }

    fn encode_primitive(&mut self, tag: Tag, value: &[u8]) {
        self.encode_value(Identifier::from_tag(tag, false), value);
    }

    fn encode_constructed(&mut self, tag: Tag, value: &[u8]) {
        self.encode_value(Identifier::from_tag(tag, true), value);
    }

    /// Encodes a given ASN.1 BER value with the `identifier`.
    fn encode_value(&mut self, identifier: Identifier, value: &[u8]) {
        let ident_bytes = self.encode_identifier(identifier);
        self.append_byte_or_bytes(ident_bytes);
        self.encode_length(identifier, value);
        self.encode_to_set(identifier.tag);
    }

    /// Runs at the end of a complete value encoding to decide whether to sort
    /// the output by the tag of each value.
    fn encode_to_set(&mut self, tag: Tag) {
        if self.is_set_encoding {
            self.set_buffer
                .insert(tag, core::mem::take(&mut self.output));
        }
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER as defined in X.690 section 11.7.
    /// Also used for BER on this crate.
    pub fn datetime_to_canonical_generalized_time_bytes(
        value: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Vec<u8> {
        let mut string;
        // Convert to UTC so we can always append Z.
        let value = value.naive_utc();
        if value.nanosecond() > 0 {
            string = value.format("%Y%m%d%H%M%S.%f").to_string();
            // No trailing zeros with fractions
            while string.ends_with('0') {
                string.pop();
            }
        } else {
            string = value.format("%Y%m%d%H%M%S").to_string();
        }
        string.push('Z');
        string.into_bytes()
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER UTCTime as defined in X.690 section 11.8.
    /// Also used for BER on this crate.
    pub fn datetime_to_canonical_utc_time_bytes(value: &chrono::DateTime<chrono::Utc>) -> Vec<u8> {
        value
            .naive_utc()
            .format("%y%m%d%H%M%SZ")
            .to_string()
            .into_bytes()
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER DATE as defined in X.690 section 8.26.2
    /// Also used for BER on this crate.
    pub fn naivedate_to_date_bytes(value: &chrono::NaiveDate) -> Vec<u8> {
        value.format("%Y%m%d").to_string().into_bytes()
    }

    fn check_encode_size_constraint(
        len: usize,
        constraints: &Constraints,
        codec: Codec,
    ) -> Result<(), EncodeError> {
        if let Some(size) = constraints.size()
            && size.extensible.is_none()
            && !size.constraint.contains(&len)
        {
            return Err(EncodeError::size_constraint_not_satisfied(
                len,
                &size.constraint,
                codec,
            ));
        }
        Ok(())
    }

    fn check_encode_value_constraint<I: IntegerType>(
        value: &I,
        constraints: &Constraints,
        codec: Codec,
    ) -> Result<(), EncodeError> {
        if let Some(value_c) = constraints.value()
            && value_c.extensible.is_none()
            && !value_c.constraint.in_bound(value)
        {
            return Err(EncodeError::value_constraint_not_satisfied(
                value.to_bigint().unwrap_or_default(),
                &value_c.constraint.value,
                codec,
            ));
        }
        Ok(())
    }

    /// Creates a child encoder that uses the parent's spare `worker` allocation
    /// as its output buffer, avoiding a fresh heap allocation for sibling fields.
    fn take_child_encoder(&mut self) -> Self {
        Self::new_with_buffer(self.config, core::mem::take(&mut self.worker))
    }

    /// Extracts the child's output buffer and stores it back as the parent's
    /// `worker`, so the next sibling field reuses the same heap allocation.
    /// Returns the encoded bytes for the caller to write to the parent output.
    fn reclaim_worker(&mut self, mut child: Encoder) -> Vec<u8> {
        let output = core::mem::take(&mut child.output);
        // Keep the larger capacity as the next worker; drop the smaller one.
        if child.worker.capacity() > self.worker.capacity() {
            self.worker = core::mem::take(&mut child.worker);
        }
        output
    }
}

impl crate::Encoder<'_> for Encoder {
    type Ok = ();
    type Error = EncodeError;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder;

    fn codec(&self) -> Codec {
        Self::codec(self)
    }

    fn encode_any(
        &mut self,
        tag: Tag,
        value: &types::Any,
        _identifier: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if self.is_set_encoding {
            return Err(BerEncodeErrorKind::AnyInSet.into());
        }

        let inner = value.as_bytes();
        if inner.is_empty() {
            return Ok(());
        }
        // If we have Any type or Choice type directly, don't encode the tag
        if tag != Tag::EOC {
            let inner_constructed = (inner[0] & 0x20) != 0;
            let ident = Identifier::from_tag(tag, inner_constructed);
            let ident_bytes = self.encode_identifier(ident);
            self.append_byte_or_bytes(ident_bytes);
            self.encode_length(ident, inner);
            self.encode_to_set(ident.tag);
        } else {
            self.output.extend_from_slice(inner);
        }

        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BitStr,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bit_length = value.len();
        Self::check_encode_size_constraint(bit_length, &constraints, self.codec())?;
        let vec = value.to_bitvec();
        let bytes = vec.as_raw_slice();
        let unused_bits: u8 = ((bytes.len() * 8) - bit_length).try_into().map_err(|err| {
            EncodeError::from_kind(
                EncodeErrorKind::FailedBitStringUnusedBitsToU8 { err },
                self.codec(),
            )
        })?;
        let mut encoded = Vec::with_capacity(bytes.len() + 1);
        encoded.push(unused_bits);
        encoded.extend(bytes);

        self.encode_string(tag, Tag::BIT_STRING, &encoded)
    }

    fn encode_bool(
        &mut self,
        tag: Tag,
        value: bool,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, &[if value { 0xff } else { 0x00 }]);
        Ok(())
    }

    fn encode_choice<E: Encode>(
        &mut self,
        _: Constraints,
        _t: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        (encode_fn)(self).map(drop)
    }

    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let value = E::discriminant(value);
        self.encode_integer(
            tag,
            Constraints::default(),
            &value,
            crate::types::Identifier::EMPTY,
        )
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &I,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_value_constraint(value, &constraints, self.codec())?;
        let (bytes, needed) = value.to_signed_bytes_be();
        self.encode_primitive(tag, &bytes.as_ref()[..needed]);
        Ok(())
    }

    fn encode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: Constraints,
        _: &R,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Err(EncodeError::real_not_supported(self.codec()))
    }

    fn encode_null(
        &mut self,
        tag: Tag,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, &[]);
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        oid: &[u32],
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = Vec::new();
        object_identifier_as_bytes(oid, &mut buffer)?;
        self.encode_primitive(tag, &buffer);
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, value)
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_iso646_bytes();
        Self::check_encode_size_constraint(bytes.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, bytes)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_iso646_bytes();
        Self::check_encode_size_constraint(bytes.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, bytes)
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::GeneralString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, value)
    }

    fn encode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::GraphicString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, value)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_bytes();
        Self::check_encode_size_constraint(bytes.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, bytes)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_bytes();
        Self::check_encode_size_constraint(bytes.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, bytes)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::TeletexString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.to_bytes();
        Self::check_encode_size_constraint(bytes.len(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, &bytes)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.to_bytes();
        // BmpString SIZE constraint is in characters; each character is 2 bytes.
        Self::check_encode_size_constraint(bytes.len() / 2, &constraints, self.codec())?;
        self.encode_octet_string_(tag, &bytes)
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &str,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // UTF8String SIZE constraint is in Unicode characters, not bytes.
        Self::check_encode_size_constraint(value.chars().count(), &constraints, self.codec())?;
        self.encode_octet_string_(tag, value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(
            tag,
            Self::datetime_to_canonical_utc_time_bytes(value).as_slice(),
        );

        Ok(())
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(
            tag,
            Self::datetime_to_canonical_generalized_time_bytes(value).as_slice(),
        );

        Ok(())
    }

    fn encode_date(
        &mut self,
        tag: Tag,
        value: &types::Date,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, Self::naivedate_to_date_bytes(value).as_slice());

        Ok(())
    }

    fn encode_some<E: Encode>(
        &mut self,
        value: &E,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag(self, tag)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag_and_constraints(
            self,
            tag,
            constraints,
            crate::types::Identifier::EMPTY,
        )
    }

    fn encode_none<E: Encode>(
        &mut self,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_none_with_tag(E::TAG, crate::types::Identifier::EMPTY)
    }

    fn encode_none_with_tag(
        &mut self,
        _: Tag,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(values.len(), &constraints, self.codec())?;
        let mut sequence_encoder = self.take_child_encoder();

        for value in values {
            value.encode(&mut sequence_encoder)?;
        }

        let child_output = self.reclaim_worker(sequence_encoder);
        self.encode_constructed(tag, &child_output);
        self.worker = {
            let mut w = child_output;
            w.clear();
            w
        };

        Ok(())
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: Constraints,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(values.len(), &constraints, self.codec())?;
        // Encode every element sequentially into one buffer, recording each
        // element's byte range so we can sort without extra allocations.
        let mut combined = core::mem::take(&mut self.worker);
        combined.clear();
        let mut elem_enc = Self::new(self.config);
        let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(values.len());

        for val in values.to_vec() {
            elem_enc.output.clear();
            val.encode(&mut elem_enc)?;
            let start = combined.len();
            combined.extend_from_slice(&elem_enc.output);
            ranges.push((start, combined.len()));
        }

        // The encodings of the component values of a set-of value shall appear
        // in ascending order, the encodings being compared as octet strings.
        ranges.sort_by(|(as_, ae), (bs, be)| combined[*as_..*ae].cmp(&combined[*bs..*be]));

        // Total content length is invariant under sorting, so write the outer
        // tag + definite length before streaming the sorted elements directly
        // into self.output — no intermediate sorted-concatenation buffer needed.
        let ident = Identifier::from_tag(tag, true);
        let ident_bytes = self.encode_identifier(ident);
        self.append_byte_or_bytes(ident_bytes);
        let len_bytes = self.encode_definite_length(combined.len());
        self.append_byte_or_bytes(len_bytes);
        for (start, end) in ranges {
            self.output.extend_from_slice(&combined[start..end]);
        }
        self.encode_to_set(tag);

        combined.clear();
        self.worker = combined;

        Ok(())
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if value.is_present() {
            let mut encoder = self.take_child_encoder();
            value.encode(&mut encoder)?;
            let child_output = self.reclaim_worker(encoder);
            self.encode_constructed(tag, &child_output);
            self.worker = {
                let mut w = child_output;
                w.clear();
                w
            };
        }
        Ok(())
    }

    fn encode_sequence<'b, const RC: usize, const EC: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RC, EC>,
        F: FnOnce(&mut Self::AnyEncoder<'b, 0, 0>) -> Result<(), Self::Error>,
    {
        let mut encoder = self.take_child_encoder();

        (encoder_scope)(&mut encoder)?;

        let child_output = self.reclaim_worker(encoder);
        self.encode_constructed(tag, &child_output);
        self.worker = {
            let mut w = child_output;
            w.clear();
            w
        };

        Ok(())
    }

    fn encode_set<'b, const RC: usize, const EC: usize, C, F>(
        &'b mut self,
        tag: Tag,
        encoder_scope: F,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<RC, EC>,
        F: FnOnce(&mut Self::AnyEncoder<'b, 0, 0>) -> Result<(), Self::Error>,
    {
        let mut encoder = Self::new_set(self.config);
        encoder.worker = core::mem::take(&mut self.worker);

        (encoder_scope)(&mut encoder)?;

        // output() merges the set_buffer entries in tag order into a fresh Vec.
        // We recycle that Vec as the new worker after copying into self.output.
        let merged = encoder.output();
        self.encode_constructed(tag, &merged);
        self.worker = {
            let mut w = merged;
            w.clear();
            w
        };

        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag_and_constraints(
            self,
            tag,
            constraints,
            crate::types::Identifier::EMPTY,
        )
    }

    /// Encode a extension addition group value.
    fn encode_extension_addition_group<const RC: usize, const EC: usize, E>(
        &mut self,
        value: Option<&E>,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed<RC, EC>,
    {
        value.encode(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::ber::enc::{Encoder, EncoderOptions};
    use crate::{Encode, types::*};
    use alloc::borrow::ToOwned;
    use alloc::vec;

    #[test]
    fn bit_string() {
        let bitstring = BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

        let primitive_encoded = &[0x03, 0x07, 0x00, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];

        assert_eq!(primitive_encoded, super::super::encode(&bitstring).unwrap());

        let empty_bitstring = BitString::from_vec(vec![]);
        let empty_bitstring_encoded = &[0x03, 0x01, 0x00][..];
        assert_eq!(
            empty_bitstring_encoded,
            super::super::encode(&empty_bitstring).unwrap()
        );
    }

    #[test]
    fn identifier() {
        fn ident_to_bytes(ident: crate::ber::Identifier) -> Vec<u8> {
            let mut enc = Encoder::new(EncoderOptions::ber());
            let bytes = enc.encode_identifier(ident);
            enc.append_byte_or_bytes(bytes);
            enc.output
        }

        assert_eq!(
            &[0xFF, 0x7F,][..],
            ident_to_bytes(crate::ber::Identifier::from_tag(
                Tag::new(crate::types::Class::Private, 127),
                true,
            ))
        );

        // DATE Tag Rec. ITU-T X.680 (02/2021) section 8 Table 1
        assert_eq!(
            &[0x1F, 0x1F,][..],
            ident_to_bytes(crate::ber::Identifier::from_tag(Tag::DATE, false,))
        );
    }

    #[test]
    fn encoding_oid() {
        fn oid_to_bytes(oid: &[u32]) -> Vec<u8> {
            use crate::Encoder;
            let mut enc = self::Encoder::new(EncoderOptions::ber());
            enc.encode_object_identifier(Tag::OBJECT_IDENTIFIER, oid, Identifier::EMPTY)
                .unwrap();
            enc.output
        }

        // example from https://stackoverflow.com/questions/5929050/how-does-asn-1-encode-an-object-identifier
        assert_eq!(
            &vec![0x06, 0x08, 0x2b, 0x06, 0x01, 0x05, 0x05, 0x07, 0x30, 0x01],
            &oid_to_bytes(&[1, 3, 6, 1, 5, 5, 7, 48, 1])
        );

        // example from https://docs.microsoft.com/en-us/windows/win32/seccertenroll/about-object-identifier
        assert_eq!(
            &vec![
                0x06, 0x09, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x15, 0x14
            ],
            &oid_to_bytes(&[1, 3, 6, 1, 4, 1, 311, 21, 20])
        );

        // commonName (X.520 DN component)
        assert_eq!(
            &vec![0x06, 0x03, 0x55, 0x04, 0x03],
            &oid_to_bytes(&[2, 5, 4, 3])
        );

        // example oid
        assert_eq!(
            &vec![0x06, 0x03, 0x88, 0x37, 0x01],
            &oid_to_bytes(&[2, 999, 1])
        );
    }

    #[test]
    fn base128_test() {
        fn encode(n: u32) -> Vec<u8> {
            let mut buffer: Vec<u8> = vec![];
            super::encode_as_base128(n, &mut buffer);
            buffer
        }

        assert_eq!(&vec![0x0], &encode(0x0));
        assert_eq!(&vec![0x7F], &encode(0x7F));
        assert_eq!(&vec![0x81, 0x00], &encode(0x80));
        assert_eq!(&vec![0xC0, 0x00], &encode(0x2000));
        assert_eq!(&vec![0xFF, 0x7F], &encode(0x3FFF));
        assert_eq!(&vec![0x81, 0x80, 0x00], &encode(0x4000));
        assert_eq!(&vec![0xFF, 0xFF, 0x7F], &encode(0x001FFFFF));
        assert_eq!(&vec![0x81, 0x80, 0x80, 0x00], &encode(0x00200000));
        assert_eq!(&vec![0xC0, 0x80, 0x80, 0x00], &encode(0x08000000));
        assert_eq!(&vec![0xFF, 0xFF, 0xFF, 0x7F], &encode(0x0FFFFFFF));
    }

    #[test]
    fn any() {
        let bitstring = BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

        let primitive_encoded = &[0x03, 0x07, 0x00, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];
        let any = Any {
            contents: primitive_encoded.into(),
        };

        assert_eq!(primitive_encoded, super::super::encode(&bitstring).unwrap());
        assert_eq!(
            super::super::encode(&bitstring).unwrap(),
            super::super::encode(&any).unwrap()
        );
    }

    #[test]
    fn set() {
        use crate::{
            Encoder as _,
            types::{AsnType, Implicit},
        };

        struct C0;
        struct C1;
        struct C2;

        impl AsnType for C0 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 0);
        }

        impl AsnType for C1 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 1);
        }

        impl AsnType for C2 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 2);
        }

        type Field1 = Implicit<C0, u32>;
        type Field2 = Implicit<C1, u32>;
        type Field3 = Implicit<C2, u32>;

        let field1: Field1 = 1.into();
        let field2: Field2 = 2.into();
        let field3: Field3 = 3.into();

        #[derive(AsnType)]
        #[rasn(crate_root = "crate")]
        struct Set;

        impl crate::types::Constructed<3, 0> for Set {
            const FIELDS: crate::types::fields::Fields<3> =
                crate::types::fields::Fields::from_static([
                    crate::types::fields::Field::new_required(0, C0::TAG, C0::TAG_TREE, "field1"),
                    crate::types::fields::Field::new_required(1, C1::TAG, C1::TAG_TREE, "field2"),
                    crate::types::fields::Field::new_required(2, C2::TAG, C2::TAG_TREE, "field3"),
                ]);
        }

        let output = {
            let mut encoder = Encoder::new_set(EncoderOptions::ber());
            encoder
                .encode_set::<3, 0, Set, _>(
                    Tag::SET,
                    |encoder| {
                        field3.encode(encoder)?;
                        field2.encode(encoder)?;
                        field1.encode(encoder)?;
                        Ok(())
                    },
                    crate::types::Identifier::EMPTY,
                )
                .unwrap();

            encoder.output()
        };

        assert_eq!(
            vec![0x31, 0x9, 0x80, 0x1, 0x1, 0x81, 0x1, 0x2, 0x82, 0x1, 0x3],
            output,
        );
    }

    #[test]
    fn octet_string_size_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        let constraints = constraints!(size_constraint!(3));

        let mut enc = Encoder::new(EncoderOptions::ber());
        enc.encode_octet_string(
            Tag::OCTET_STRING,
            constraints,
            &[0x01, 0x02, 0x03],
            Identifier::EMPTY,
        )
        .unwrap();

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_octet_string(
                Tag::OCTET_STRING,
                constraints,
                &[0x01, 0x02],
                Identifier::EMPTY,
            )
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 2, .. }
        ));

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_octet_string(
                Tag::OCTET_STRING,
                constraints,
                &[0x01, 0x02, 0x03, 0x04],
                Identifier::EMPTY,
            )
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 4, .. }
        ));
    }

    #[test]
    fn bit_string_size_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        // SIZE(8) means exactly 8 bits
        let constraints = constraints!(size_constraint!(8));
        let eight_bits = BitString::from_vec(alloc::vec![0xAA]);
        let sixteen_bits = BitString::from_vec(alloc::vec![0xAA, 0xBB]);

        let mut enc = Encoder::new(EncoderOptions::ber());
        enc.encode_bit_string(Tag::BIT_STRING, constraints, &eight_bits, Identifier::EMPTY)
            .unwrap();

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_bit_string(
                Tag::BIT_STRING,
                constraints,
                &sixteen_bits,
                Identifier::EMPTY,
            )
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 16, .. }
        ));
    }

    #[test]
    fn integer_value_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        // VALUE(0..100)
        let constraints = constraints!(value_constraint!(0, 100));

        let mut enc = Encoder::new(EncoderOptions::ber());
        enc.encode_integer(Tag::INTEGER, constraints, &50i32, Identifier::EMPTY)
            .unwrap();

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_integer(Tag::INTEGER, constraints, &200i32, Identifier::EMPTY)
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::ValueConstraintNotSatisfied { .. }
        ));

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_integer(Tag::INTEGER, constraints, &-1i32, Identifier::EMPTY)
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::ValueConstraintNotSatisfied { .. }
        ));
    }

    #[test]
    fn sequence_of_size_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        // SIZE(1..3)
        let constraints = constraints!(size_constraint!(1, 3));

        let mut enc = Encoder::new(EncoderOptions::ber());
        enc.encode_sequence_of(Tag::SEQUENCE, &[1i32, 2], constraints, Identifier::EMPTY)
            .unwrap();

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_sequence_of(Tag::SEQUENCE, &[] as &[i32], constraints, Identifier::EMPTY)
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 0, .. }
        ));

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_sequence_of(
                Tag::SEQUENCE,
                &[1i32, 2, 3, 4],
                constraints,
                Identifier::EMPTY,
            )
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 4, .. }
        ));
    }

    #[test]
    fn set_of_size_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        // SIZE(1..3)
        let constraints = constraints!(size_constraint!(1, 3));
        let two: SetOf<i32> = SetOf::from_vec(vec![1, 2]);
        let empty: SetOf<i32> = SetOf::new();
        let four: SetOf<i32> = SetOf::from_vec(vec![1, 2, 3, 4]);

        let mut enc = Encoder::new(EncoderOptions::ber());
        enc.encode_set_of(Tag::SET, &two, constraints, Identifier::EMPTY)
            .unwrap();

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_set_of(Tag::SET, &empty, constraints, Identifier::EMPTY)
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 0, .. }
        ));

        let mut enc = Encoder::new(EncoderOptions::ber());
        let err = enc
            .encode_set_of(Tag::SET, &four, constraints, Identifier::EMPTY)
            .unwrap_err();
        assert!(matches!(
            *err.kind,
            EncodeErrorKind::SizeConstraintNotSatisfied { size: 4, .. }
        ));
    }
}
