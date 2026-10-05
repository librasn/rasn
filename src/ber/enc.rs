//! Encoding Rust structures into Basic Encoding Rules data.

mod config;

use alloc::{
    borrow::{Cow, ToOwned},
    vec::Vec,
};

use super::{Identifier, time};
use crate::{
    Codec, Encode,
    types::{
        self, Constraints, Enumerated, IntegerType, Tag,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
    },
};

pub use crate::error::{BerEncodeErrorKind, EncodeError, EncodeErrorKind};
pub use config::EncoderOptions;

/// The initial length octet of the indefinite form (X.690 §8.1.3.6).
const INDEFINITE_LENGTH: u8 = 0x80;
/// The output capacity reserved before encoding, so that a message up to
/// this size is written without the buffer growing.
const INITIAL_CAPACITY: usize = 1024;
/// The length of a canonical UTCTime, `YYMMDDHHMMSSZ`.
const UTC_TIME_LENGTH: usize = 13;
/// The length of a DATE, `YYYYMMDD`.
const DATE_LENGTH: usize = 8;
/// Room for a canonical GeneralizedTime with a fraction of a second,
/// `YYYYMMDDHHMMSS.FFFFFFFFFZ`.
const GENERALIZED_TIME_CAPACITY: usize = 25;
/// The end-of-contents octets that close the indefinite form (§8.1.5).
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
    write_arc((first * (MAX_OID_SECOND_OCTET + 1)) + second, buffer);
    for component in oid.iter().skip(2) {
        write_arc(*component, buffer);
    }
    Ok(())
}

/// Appends an arc (§8.19.2): one octet when it is below 128, the base 128
/// form otherwise.
#[inline]
fn write_arc(arc: u32, buffer: &mut Vec<u8>) {
    if arc < 0x80 {
        buffer.push(arc as u8);
    } else {
        encode_as_base128(arc, buffer);
    }
}

/// Appends `number` in base 128, seven bits per octet, most significant
/// group first, with bit 8 set on every octet but the last (X.690
/// §8.1.2.4.2, §8.19.2).
///
/// Kept out of line so that the identifier writer, which only needs it for
/// tag numbers of 31 and over, stays small enough to inline.
#[inline(never)]
pub(super) fn encode_as_base128(number: u32, buffer: &mut Vec<u8>) {
    let groups = (u32::BITS - number.leading_zeros()).div_ceil(7).max(1);
    for group in (1..groups).rev() {
        buffer.push(0x80 | ((number >> (7 * group)) & 0x7F) as u8);
    }
    buffer.push((number & 0x7F) as u8);
}

/// Encodes Rust structures into Basic Encoding Rules data.
///
/// Every value is written straight into one output buffer. A value whose
/// contents length is not known in advance reserves one length octet, which
/// is completed once the contents are written; when the long form is needed,
/// the contents are shifted along to make room. The components of a SET and
/// the elements of a SET OF are encoded in sequence and reordered in place.
pub struct Encoder {
    config: EncoderOptions,
    output: Vec<u8>,
    /// The number of constructed values whose contents are being written.
    depth: usize,
    /// The innermost SET whose components are being written.
    open_set: Option<OpenSet>,
    /// The components of the open SETs, outermost first, followed by the
    /// elements of the SET OF being written, if any.
    components: Vec<Component>,
    /// Space for reordering components.
    scratch: Vec<u8>,
}

/// A SET whose components are being written.
#[derive(Clone, Copy)]
struct OpenSet {
    /// The depth at which the components of the SET are written.
    depth: usize,
    /// The index in `Encoder::components` of the first component.
    first_component: usize,
}

/// A value written as a direct component of a SET or element of a SET OF,
/// occupying `start..end` of the output.
struct Component {
    tag: Tag,
    start: usize,
    end: usize,
}

/// A value whose identifier and length octets are written and whose
/// contents are being written.
struct OpenValue {
    identifier: Identifier,
    /// Where the value starts in the output.
    start: usize,
    /// Where the reserved length octet is in the output.
    length_position: usize,
}

impl OpenValue {
    /// Where the contents octets start in the output.
    fn contents_start(&self) -> usize {
        self.length_position + 1
    }
}

/// The number of octets that `length`, at least 128, takes in the long form.
fn long_form_octets(length: usize) -> usize {
    (usize::BITS - length.leading_zeros()).div_ceil(8) as usize
}

impl Encoder {
    /// Creates a new instance from the given `config`.
    #[must_use]
    pub fn new(config: EncoderOptions) -> Self {
        Self::new_with_buffer(config, Vec::new())
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
        let mut encoder = Self::new(config);
        encoder.open_set = Some(OpenSet {
            depth: 0,
            first_component: 0,
        });
        encoder
    }

    /// Creates a new instance from the given `config` and a user-supplied
    /// `Vec<u8>` buffer. This allows reuse of an existing buffer instead of
    /// allocating a new encoding buffer each time an [`Encoder`] is created.
    /// The buffer will be cleared before use.
    #[must_use]
    pub fn new_with_buffer(config: EncoderOptions, mut buffer: Vec<u8>) -> Self {
        buffer.clear();
        buffer.reserve(INITIAL_CAPACITY);
        Self {
            config,
            output: buffer,
            depth: 0,
            open_set: None,
            components: Vec::new(),
            scratch: Vec::new(),
        }
    }

    /// Consumes the encoder and returns the output of the encoding.
    #[must_use]
    pub fn output(mut self) -> Vec<u8> {
        if let Some(set) = self.open_set.take() {
            self.order_set_components(0, set.first_component);
        }
        self.output
    }

    /// Writes the identifier octets (§8.1.2). An identifier consists of a
    /// class, an encoding bit and a tag number; a tag number of 31 or more
    /// follows the initial octet in base 128, most significant group first,
    /// with bit 8 set on every octet but the last.
    ///
    /// ```text
    /// ---------------------------------
    /// | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    /// ---------------------------------
    /// | class | E |        Tag        |
    /// ---------------------------------
    /// ```
    fn write_identifier(
        &mut self,
        Identifier {
            tag,
            is_constructed,
        }: Identifier,
    ) {
        const HIGH_TAG_NUMBER: u8 = 0x1F;
        let class = (tag.class as u8) << 6;
        // SEQUENCE, SET and EXTERNAL only have constructed encodings.
        let encoding = if is_constructed || matches!(tag, Tag::EXTERNAL | Tag::SEQUENCE | Tag::SET)
        {
            0x20
        } else {
            0
        };
        if tag.value < u32::from(HIGH_TAG_NUMBER) {
            self.output.push(class | encoding | tag.value as u8);
        } else {
            self.output.push(class | encoding | HIGH_TAG_NUMBER);
            encode_as_base128(tag.value, &mut self.output);
        }
    }

    /// Writes the length octets of the definite form (§8.1.3.3).
    fn write_length(&mut self, length: usize) {
        if length < 0x80 {
            self.output.push(length as u8);
        } else {
            let octets = long_form_octets(length);
            self.output.push(0x80 | octets as u8);
            self.output
                .extend_from_slice(&length.to_be_bytes()[size_of::<usize>() - octets..]);
        }
    }

    /// Writes a primitive value with the given contents octets.
    fn write_primitive(&mut self, tag: Tag, contents: &[u8]) {
        self.write_primitive_with(tag, contents.len(), |output| {
            output.extend_from_slice(contents);
        });
    }

    /// Writes a primitive value whose `length` contents octets `write` appends.
    fn write_primitive_with(&mut self, tag: Tag, length: usize, write: impl FnOnce(&mut Vec<u8>)) {
        let start = self.output.len();
        self.write_identifier(Identifier::from_tag(tag, false));
        self.write_length(length);
        write(&mut self.output);
        self.finish_value(tag, start);
    }

    /// Starts a value whose contents length is not known in advance. The
    /// contents are written next, and [`Self::end_value`] completes the
    /// length octets.
    fn begin_value(&mut self, identifier: Identifier) -> OpenValue {
        let start = self.output.len();
        self.write_identifier(identifier);
        let length_position = self.output.len();
        if identifier.is_constructed() {
            self.depth += 1;
        }
        // The indefinite form (§8.1.3.6) needs no completion; the definite
        // form gets its short form octet patched, or the long form inserted.
        self.output.push(if self.is_indefinite(identifier) {
            INDEFINITE_LENGTH
        } else {
            0
        });
        OpenValue {
            identifier,
            start,
            length_position,
        }
    }

    /// Completes a value started by [`Self::begin_value`].
    fn end_value(&mut self, value: OpenValue) {
        if value.identifier.is_constructed() {
            self.depth -= 1;
        }
        if self.is_indefinite(value.identifier) {
            self.output.extend_from_slice(END_OF_CONTENTS);
        } else {
            self.complete_length(value.length_position);
        }
        self.finish_value(value.identifier.tag, value.start);
    }

    /// Whether a value with `identifier` uses the indefinite length form:
    /// CER requires it for every constructed encoding (§9.1).
    fn is_indefinite(&self, identifier: Identifier) -> bool {
        identifier.is_constructed() && self.config.encoding_rules.is_cer()
    }

    /// Completes the length octet reserved at `length_position` for the
    /// contents that follow it and extend to the end of the output.
    fn complete_length(&mut self, length_position: usize) {
        let contents_start = length_position + 1;
        let length = self.output.len() - contents_start;
        if length < 0x80 {
            self.output[length_position] = length as u8;
            return;
        }
        // The long form (§8.1.3.5): the reserved octet counts the length
        // octets, which take the place of the contents, shifted along.
        let octets = long_form_octets(length);
        let end = self.output.len();
        self.output.resize(end + octets, 0);
        self.output
            .copy_within(contents_start..end, contents_start + octets);
        self.output[length_position] = 0x80 | octets as u8;
        self.output[contents_start..contents_start + octets]
            .copy_from_slice(&length.to_be_bytes()[size_of::<usize>() - octets..]);
    }

    /// Records the value written from `start` to the end of the output when
    /// it is a component of the open SET.
    #[inline]
    fn finish_value(&mut self, tag: Tag, start: usize) {
        if let Some(set) = self.open_set
            && set.depth == self.depth
        {
            self.components.push(Component {
                tag,
                start,
                end: self.output.len(),
            });
        }
    }

    /// Whether a value written now is a component of the open SET.
    fn in_open_set(&self) -> bool {
        self.open_set.is_some_and(|set| set.depth == self.depth)
    }

    /// Orders the components of a SET, recorded from `first_component` and
    /// written in sequence from `contents_start`, by their tags (§10.3).
    fn order_set_components(&mut self, contents_start: usize, first_component: usize) {
        self.components[first_component..].sort_by_key(|component| component.tag);
        self.reorder(contents_start, first_component);
    }

    /// Orders the elements of a SET OF, recorded from `first_element` and
    /// written in sequence from `contents_start`, by their encodings (§11.6).
    fn order_set_of_elements(&mut self, contents_start: usize, first_element: usize) {
        let output = &self.output;
        self.components[first_element..]
            .sort_by(|a, b| output[a.start..a.end].cmp(&output[b.start..b.end]));
        self.reorder(contents_start, first_element);
    }

    /// Rewrites the output from `contents_start` so that the components
    /// recorded from `first_component`, written there in sequence, appear in
    /// the order they are now in, and forgets them.
    fn reorder(&mut self, contents_start: usize, first_component: usize) {
        let components = &self.components[first_component..];
        if !components.is_sorted_by_key(|component| component.start) {
            self.scratch.clear();
            self.scratch
                .extend_from_slice(&self.output[contents_start..]);
            let mut position = contents_start;
            for component in components {
                let encoding =
                    &self.scratch[component.start - contents_start..component.end - contents_start];
                self.output[position..position + encoding.len()].copy_from_slice(encoding);
                position += encoding.len();
            }
        }
        self.components.truncate(first_component);
    }

    /// Writes a string value: primitive, unless its contents exceed the
    /// segment size of the encoding rules (1000 octets in CER, §9.2), in
    /// which case it is constructed from primitive segments tagged
    /// `segment_tag`.
    fn write_string(&mut self, tag: Tag, segment_tag: Tag, contents: &[u8]) {
        let segment_size = self.config.encoding_rules.max_string_length();
        if contents.len() <= segment_size {
            self.write_primitive(tag, contents);
        } else {
            let string = self.begin_value(Identifier::from_tag(tag, true));
            for segment in contents.chunks(segment_size) {
                self.write_primitive(segment_tag, segment);
            }
            self.end_value(string);
        }
    }

    /// Writes a BIT STRING value (§8.6), whose contents start with the
    /// number of unused bits in their last octet. A constructed encoding
    /// (§9.2) nests segments that each start with their own count, which is
    /// zero for all but the last.
    fn write_bit_string(&mut self, tag: Tag, unused_bits: u8, octets: &[u8]) {
        let segment_size = self.config.encoding_rules.max_string_length();
        if octets.len() < segment_size {
            self.write_primitive_with(tag, octets.len() + 1, |output| {
                output.push(unused_bits);
                output.extend_from_slice(octets);
            });
        } else {
            let string = self.begin_value(Identifier::from_tag(tag, true));
            let segments = octets.chunks(segment_size - 1);
            let last = segments.len() - 1;
            for (index, segment) in segments.enumerate() {
                let unused_bits = if index == last { unused_bits } else { 0 };
                self.write_primitive_with(Tag::BIT_STRING, segment.len() + 1, |output| {
                    output.push(unused_bits);
                    output.extend_from_slice(segment);
                });
            }
            self.end_value(string);
        }
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER as defined in X.690 section 11.7.
    /// Also used for BER on this crate.
    pub fn datetime_to_canonical_generalized_time_bytes(
        value: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(GENERALIZED_TIME_CAPACITY);
        time::write_generalized_time(&mut bytes, value);
        bytes
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER UTCTime as defined in X.690 section 11.8.
    /// Also used for BER on this crate.
    pub fn datetime_to_canonical_utc_time_bytes(value: &chrono::DateTime<chrono::Utc>) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(UTC_TIME_LENGTH);
        time::write_utc_time(&mut bytes, value);
        bytes
    }

    #[must_use]
    /// Canonical byte presentation for CER/DER DATE as defined in X.690 section 8.26.2
    /// Also used for BER on this crate.
    pub fn naivedate_to_date_bytes(value: &chrono::NaiveDate) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(DATE_LENGTH);
        time::write_date(&mut bytes, value);
        bytes
    }

    fn check_encode_size_constraint(
        len: usize,
        constraints: &Constraints,
        codec: Codec,
    ) -> Result<(), EncodeError> {
        Self::check_encode_size_constraint_with(|| len, constraints, codec)
    }

    /// Checks the size constraint against `len`, which is only computed when
    /// there is a constraint to check.
    fn check_encode_size_constraint_with(
        len: impl FnOnce() -> usize,
        constraints: &Constraints,
        codec: Codec,
    ) -> Result<(), EncodeError> {
        if let Some(size) = constraints.size()
            && size.extensible.is_none()
        {
            let len = len();
            if !size.constraint.contains(&len) {
                return Err(EncodeError::size_constraint_not_satisfied(
                    len,
                    &size.constraint,
                    codec,
                ));
            }
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
        let encoding = value.as_bytes();
        if encoding.is_empty() {
            return Ok(());
        }
        if tag != Tag::EOC {
            // A tagged open type is wrapped in a constructed encoding, as
            // X.680 §31.2.7 and §31.2.9 only allow explicit tagging of open
            // types and X.690 §8.14.3 requires explicit tags to be constructed.
            let open_type = self.begin_value(Identifier::from_tag(tag, true));
            self.output.extend_from_slice(encoding);
            self.end_value(open_type);
        } else if self.in_open_set() {
            // Without a tag there is nothing to order the component by.
            return Err(BerEncodeErrorKind::AnyInSet.into());
        } else {
            self.output.extend_from_slice(encoding);
        }
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::BitStr,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), constraints, self.codec())?;
        let unused_bits = (value.len().next_multiple_of(8) - value.len()) as u8;
        let octets = match crate::bits::aligned_octets(value) {
            Some(octets) => Cow::Borrowed(octets),
            None => {
                let mut bits = value.to_bitvec();
                bits.force_align();
                bits.set_uninitialized(false);
                Cow::Owned(bits.into_vec())
            }
        };
        self.write_bit_string(tag, unused_bits, &octets);
        Ok(())
    }

    fn encode_bool(
        &mut self,
        tag: Tag,
        value: bool,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(tag, &[if value { 0xFF } else { 0x00 }]);
        Ok(())
    }

    fn encode_choice<E: Encode>(
        &mut self,
        _: &Constraints,
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
            &Constraints::default(),
            &value,
            crate::types::Identifier::EMPTY,
        )
    }

    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &I,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_value_constraint(value, constraints, self.codec())?;
        let (bytes, needed) = value.to_signed_bytes_be();
        self.write_primitive(tag, &bytes.as_ref()[..needed]);
        Ok(())
    }

    fn encode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: &Constraints,
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
        self.write_primitive(tag, &[]);
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        oid: &[u32],
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let object_identifier = self.begin_value(Identifier::from_tag(tag, false));
        object_identifier_as_bytes(oid, &mut self.output)?;
        self.end_value(object_identifier);
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &[u8],
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, value);
        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::VisibleString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_iso646_bytes();
        Self::check_encode_size_constraint(bytes.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, bytes);
        Ok(())
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::Ia5String,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_iso646_bytes();
        Self::check_encode_size_constraint(bytes.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, bytes);
        Ok(())
    }

    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::GeneralString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, value);
        Ok(())
    }

    fn encode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::GraphicString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(value.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, value);
        Ok(())
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::PrintableString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_bytes();
        Self::check_encode_size_constraint(bytes.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, bytes);
        Ok(())
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::NumericString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.as_bytes();
        Self::check_encode_size_constraint(bytes.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, bytes);
        Ok(())
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::TeletexString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let octets: &[u8] = value;
        Self::check_encode_size_constraint(octets.len(), constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, octets);
        Ok(())
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &types::BmpString,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = value.to_bytes();
        // BmpString SIZE constraint is in characters; each character is 2 bytes.
        Self::check_encode_size_constraint(bytes.len() / 2, constraints, self.codec())?;
        self.write_string(tag, Tag::OCTET_STRING, &bytes);
        Ok(())
    }

    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
        value: &str,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        // UTF8String SIZE constraint is in Unicode characters, not bytes.
        Self::check_encode_size_constraint_with(
            || value.chars().count(),
            constraints,
            self.codec(),
        )?;
        self.write_string(tag, Tag::OCTET_STRING, value.as_bytes());
        Ok(())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_primitive_with(tag, UTC_TIME_LENGTH, |output| {
            time::write_utc_time(output, value);
        });
        Ok(())
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        let generalized_time = self.begin_value(Identifier::from_tag(tag, false));
        time::write_generalized_time(&mut self.output, value);
        self.end_value(generalized_time);
        Ok(())
    }

    fn encode_date(
        &mut self,
        tag: Tag,
        value: &types::Date,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_primitive_with(tag, DATE_LENGTH, |output| {
            time::write_date(output, value);
        });
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
        constraints: &Constraints,
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
        constraints: &Constraints,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(values.len(), constraints, self.codec())?;
        let sequence = self.begin_value(Identifier::from_tag(tag, true));
        for value in values {
            value.encode(self)?;
        }
        self.end_value(sequence);
        Ok(())
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: &Constraints,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        Self::check_encode_size_constraint(values.len(), constraints, self.codec())?;
        let set = self.begin_value(Identifier::from_tag(tag, true));
        let first_element = self.components.len();
        for value in values.iter() {
            let start = self.output.len();
            value.encode(self)?;
            self.components.push(Component {
                tag: E::TAG,
                start,
                end: self.output.len(),
            });
        }
        self.order_set_of_elements(set.contents_start(), first_element);
        self.end_value(set);
        Ok(())
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
        _: crate::types::Identifier,
    ) -> Result<Self::Ok, Self::Error> {
        if value.is_present() {
            let prefix = self.begin_value(Identifier::from_tag(tag, true));
            value.encode(self)?;
            self.end_value(prefix);
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
        let sequence = self.begin_value(Identifier::from_tag(tag, true));
        (encoder_scope)(self)?;
        self.end_value(sequence);
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
        let set = self.begin_value(Identifier::from_tag(tag, true));
        let first_component = self.components.len();
        let enclosing = self.open_set.replace(OpenSet {
            depth: self.depth,
            first_component,
        });
        let components = (encoder_scope)(self);
        self.open_set = enclosing;
        components?;
        self.order_set_components(set.contents_start(), first_component);
        self.end_value(set);
        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: &Constraints,
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

        // Bits that do not fill the last octet, and a slice that does not
        // start on an octet boundary.
        fn encode(bits: &BitStr) -> Vec<u8> {
            use crate::Encoder as _;
            let mut enc = Encoder::new(EncoderOptions::ber());
            enc.encode_bit_string(
                Tag::BIT_STRING,
                &Constraints::default(),
                bits,
                Identifier::EMPTY,
            )
            .unwrap();
            enc.output
        }
        let bits = bitvec::bitvec![u8, bitvec::order::Msb0; 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1];
        assert_eq!(&[0x03, 0x03, 0x05, 0xD5, 0xA0][..], encode(&bits));
        assert_eq!(&[0x03, 0x03, 0x06, 0xAB, 0x40][..], encode(&bits[1..]));
    }

    #[test]
    fn identifier() {
        fn ident_to_bytes(ident: crate::ber::Identifier) -> Vec<u8> {
            let mut enc = Encoder::new(EncoderOptions::ber());
            enc.write_identifier(ident);
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
    fn long_form_lengths() {
        // Lengths of 128 and 256 octets need two and three length octets,
        // inserted after the contents are written.
        let octets = OctetString::from(vec![0xAB; 128]);
        let encoded = super::super::encode(&octets).unwrap();
        assert_eq!(&encoded[..3], &[0x04, 0x81, 0x80]);
        assert_eq!(&encoded[3..], &octets[..]);

        let nested: Vec<Vec<OctetString>> = vec![vec![OctetString::from(vec![0xCD; 256])]];
        let encoded = super::super::encode(&nested).unwrap();
        assert_eq!(
            &encoded[..12],
            &[
                0x30, 0x82, 0x01, 0x08, 0x30, 0x82, 0x01, 0x04, 0x04, 0x82, 0x01, 0x00
            ]
        );
        assert_eq!(&encoded[12..], &[0xCD; 256][..]);
        assert_eq!(
            super::super::decode::<Vec<Vec<OctetString>>>(&encoded).unwrap(),
            nested
        );
    }

    #[test]
    fn cer_segments_long_strings() {
        let octets = OctetString::from(vec![0x55; 1500]);
        let encoded = crate::cer::encode(&octets).unwrap();
        assert_eq!(&encoded[..5], &[0x24, 0x80, 0x04, 0x82, 0x03]);
        assert_eq!(encoded[5], 0xE8);
        assert_eq!(&encoded[1006..1008], &[0x04, 0x82]);
        assert_eq!(&encoded[1008..1010], &[0x01, 0xF4]);
        assert_eq!(&encoded[encoded.len() - 2..], &[0x00, 0x00]);
        assert_eq!(crate::cer::decode::<OctetString>(&encoded).unwrap(), octets);

        // Each BIT STRING segment holds 999 octets after its own count of
        // unused bits, which only the last segment can set.
        let mut bits = BitString::repeat(true, 1200 * 8 + 3);
        bits.set(0, false);
        let encoded = crate::cer::encode(&bits).unwrap();
        assert_eq!(&encoded[..6], &[0x23, 0x80, 0x03, 0x82, 0x03, 0xE8]);
        assert_eq!(encoded[6], 0);
        let last_segment = 2 + 1004;
        assert_eq!(
            &encoded[last_segment..last_segment + 5],
            &[0x03, 0x81, 0xCB, 0x05, 0xFF]
        );
        assert_eq!(crate::cer::decode::<BitString>(&encoded).unwrap(), bits);
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
    fn set_components_are_ordered_by_their_outermost_tag() {
        use crate as rasn;
        use rasn::prelude::*;

        #[derive(AsnType, Encode, Decode, Debug, PartialEq)]
        #[rasn(set)]
        struct Inner {
            #[rasn(tag(1))]
            b: u8,
            #[rasn(tag(0))]
            a: u8,
        }

        #[derive(AsnType, Encode, Decode, Debug, PartialEq)]
        #[rasn(set)]
        struct Outer {
            #[rasn(tag(explicit(3)))]
            explicit: Vec<u8>,
            inner: Inner,
            #[rasn(tag(2))]
            open: Any,
            #[rasn(tag(application, 0))]
            application: u8,
        }

        let value = Outer {
            explicit: vec![9],
            inner: Inner { b: 2, a: 1 },
            open: Any::new(vec![0x05, 0x00]),
            application: 7,
        };
        let encoded = super::super::encode(&value).unwrap();
        assert_eq!(
            encoded,
            [
                0x31, 0x16, // SET
                0x31, 0x06, 0x80, 0x01, 0x01, 0x81, 0x01, 0x02, // inner, components reordered
                0x40, 0x01, 0x07, // [APPLICATION 0]
                0xA2, 0x02, 0x05, 0x00, // [2] open type
                0xA3, 0x05, 0x30, 0x03, 0x02, 0x01, 0x09, // [3] EXPLICIT
            ]
        );
        assert_eq!(super::super::decode::<Outer>(&encoded).unwrap(), value);
    }

    #[test]
    fn untagged_open_type_in_set() {
        use crate::{AsnType, Encoder as _};

        #[derive(AsnType)]
        #[rasn(crate_root = "crate")]
        struct Set;

        impl crate::types::Constructed<1, 0> for Set {
            const FIELDS: crate::types::fields::Fields<1> =
                crate::types::fields::Fields::from_static([
                    crate::types::fields::Field::new_required(0, Any::TAG, Any::TAG_TREE, "open"),
                ]);
        }

        let mut encoder = Encoder::new(EncoderOptions::ber());
        let error = encoder
            .encode_set::<1, 0, Set, _>(
                Tag::SET,
                |encoder| Any::new(vec![0x05, 0x00]).encode(encoder),
                crate::types::Identifier::EMPTY,
            )
            .unwrap_err();
        assert!(matches!(
            *error.kind,
            crate::error::EncodeErrorKind::CodecSpecific {
                inner: crate::error::CodecEncodeError::Ber(super::BerEncodeErrorKind::AnyInSet)
            }
        ));
    }

    #[test]
    fn set_of_elements_are_ordered_by_their_encodings() {
        let set: SetOf<OctetString> = SetOf::from_vec(vec![
            OctetString::from(vec![0x02, 0x01]),
            OctetString::from(vec![0x01]),
            OctetString::from(vec![0x01, 0xFF]),
            OctetString::from(vec![0x02]),
        ]);
        let encoded = super::super::encode(&set).unwrap();
        assert_eq!(
            encoded,
            [
                0x31, 0x0E, 0x04, 0x01, 0x01, 0x04, 0x01, 0x02, 0x04, 0x02, 0x01, 0xFF, 0x04, 0x02,
                0x02, 0x01
            ]
        );
        assert_eq!(crate::cer::encode(&set).unwrap()[..2], [0x31, 0x80]);
        assert_eq!(
            super::super::decode::<SetOf<OctetString>>(&encoded).unwrap(),
            set
        );
    }

    #[test]
    fn octet_string_size_constraint() {
        use crate::Encoder as _;
        use crate::error::EncodeErrorKind;

        let constraints = &constraints!(size_constraint!(3));

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
        let constraints = &constraints!(size_constraint!(8));
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
        let constraints = &constraints!(value_constraint!(0, 100));

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
        let constraints = &constraints!(size_constraint!(1, 3));

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
        let constraints = &constraints!(size_constraint!(1, 3));
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
