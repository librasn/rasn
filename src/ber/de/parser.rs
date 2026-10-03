//! Parsing of the octets that frame every X.690 encoding: the identifier
//! octets, the length octets and, for the indefinite form, the
//! end-of-contents octets (ITU-T X.690 (02/2021) §8.1).

use alloc::vec::Vec;

use bitvec::{order::Msb0, view::BitView};

use super::{
    BerDecodeErrorKind, CodecDecodeError, DecodeError, DecodeErrorKind, DecoderOptions,
    DerDecodeErrorKind, Result,
};
use crate::{
    Codec,
    ber::{EncodingRules, identifier::Identifier},
    de::{Error as _, Needed},
    types::{BitString, Class, Tag},
};

/// The end-of-contents octets that close the indefinite form (§8.1.5).
pub(super) const END_OF_CONTENTS: &[u8] = &[0, 0];

/// One encoded value (§8.1.1): its identifier, and its contents octets as
/// delimited by its length octets.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Value<'input> {
    pub(crate) identifier: Identifier,
    pub(crate) contents: Contents<'input>,
}

/// The contents octets of a value, as delimited by its length octets (§8.1.3).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Contents<'input> {
    /// The definite form (§8.1.3.3): the contents are exactly these octets.
    Definite(&'input [u8]),
    /// The indefinite form (§8.1.3.6), which only a constructed encoding may
    /// use: the contents extend to the end-of-contents octets, which stay in
    /// the input for the caller to parse after them.
    Indefinite,
}

/// A string type assembled from the segments of a constructed string
/// encoding (§8.6.4, §8.7.3, §8.21.3), or from the single segment of a
/// primitive one.
pub(crate) trait Segmented: Default {
    /// Appends the contents octets of one primitive segment.
    fn append_segment(&mut self, segment: &[u8], codec: Codec) -> Result<()>;
}

impl Segmented for Vec<u8> {
    fn append_segment(&mut self, segment: &[u8], _: Codec) -> Result<()> {
        self.extend_from_slice(segment);
        Ok(())
    }
}

impl Segmented for BitString {
    /// The first octet gives the number of unused bits in the last octet
    /// (§8.6.2.2), so a segment is never empty.
    fn append_segment(&mut self, segment: &[u8], codec: Codec) -> Result<()> {
        let (&unused_bits, octets) = segment
            .split_first()
            .ok_or_else(|| DecodeError::unexpected_empty_input(codec))?;
        let invalid = || DecodeError::invalid_bit_string(unused_bits, codec);
        if unused_bits > 7 {
            return Err(invalid());
        }
        let bit_length = (octets.len() * 8)
            .checked_sub(usize::from(unused_bits))
            .ok_or_else(invalid)?;
        crate::bits::extend_bitstring(self, &octets.view_bits::<Msb0>()[..bit_length]);
        Ok(())
    }
}

/// A cursor over the input that parses the framing of values and advances
/// past what it parses, including on failure. A caller that must not consume
/// input on failure parses through a copy and keeps the copy's position only
/// on success, as [`Decoder::parse`](super::Decoder::parse) does.
#[derive(Clone, Copy)]
pub(crate) struct Parser<'input> {
    rules: EncodingRules,
    /// The levels of nested constructed encodings that may still be entered.
    depth: usize,
    input: &'input [u8],
}

impl<'input> Parser<'input> {
    /// A parser over `input` with the rules and remaining depth of `options`.
    #[inline]
    pub(crate) fn new(options: DecoderOptions, input: &'input [u8]) -> Self {
        Self {
            rules: options.encoding_rules,
            depth: options.remaining_depth,
            input,
        }
    }

    /// The input that has not been parsed.
    #[inline]
    pub(crate) fn remaining(self) -> &'input [u8] {
        self.input
    }

    fn codec(&self) -> Codec {
        self.rules.codec()
    }

    /// Parses the identifier and length octets of one value, checking its tag
    /// against `expected` when given, and skips the contents octets of the
    /// definite form.
    ///
    /// Always inlined: returned out of line, the value and its error share a
    /// memory slot whose overlapping fields cost more to reassemble than the
    /// parsing itself.
    #[inline(always)]
    pub(crate) fn value(&mut self, expected: Option<Tag>) -> Result<Value<'input>> {
        let identifier = self.identifier()?;
        if let Some(expected) = expected
            && identifier.tag != expected
        {
            return Err(self.mismatched_tag(expected, identifier.tag));
        }
        let contents = self.contents(identifier)?;
        Ok(Value {
            identifier,
            contents,
        })
    }

    /// Parses the identifier octets (§8.1.2).
    #[inline(always)]
    pub(crate) fn identifier(&mut self) -> Result<Identifier> {
        let initial = self.octet()?;
        // Bits 8 and 7 hold the class (§8.1.2.2), bit 6 whether the encoding
        // is constructed (§8.1.2.5) and bits 5 to 1 the tag number, unless
        // they are all ones and the number follows in base 128 (§8.1.2.4).
        let class = match initial >> 6 {
            0 => Class::Universal,
            1 => Class::Application,
            2 => Class::Context,
            _ => Class::Private,
        };
        let is_constructed = initial & 0x20 != 0;
        let number = match initial & 0x1F {
            0x1F => self.tag_number()?,
            number => u32::from(number),
        };
        let identifier = Identifier {
            tag: Tag::new(class, number),
            is_constructed,
        };
        // End-of-contents octets look like an identifier of tag number zero,
        // and are never a value.
        if identifier.tag == Tag::EOC {
            return Err(self.ber_error(BerDecodeErrorKind::UnexpectedEndOfContents));
        }
        Ok(identifier)
    }

    /// Parses a tag number in the high form (§8.1.2.4).
    fn tag_number(&mut self) -> Result<u32> {
        self.base128_number(BerDecodeErrorKind::UnsupportedTagNumber)
    }

    /// Parses one arc of an object identifier (§8.19.2).
    #[inline]
    pub(crate) fn object_identifier_arc(&mut self) -> Result<u32> {
        self.base128_number(BerDecodeErrorKind::UnsupportedObjectIdentifierArc)
    }

    /// Parses a number in base 128, seven bits per octet, most significant
    /// octet first, with bit 8 set on every octet but the last (§8.1.2.4.2,
    /// §8.19.2), reporting `overflow` when it does not fit `u32`.
    #[inline]
    fn base128_number(&mut self, overflow: BerDecodeErrorKind) -> Result<u32> {
        let mut number = 0u32;
        loop {
            let octet = self.octet()?;
            if number > u32::MAX >> 7 {
                return Err(self.ber_error(overflow));
            }
            number = (number << 7) | u32::from(octet & 0x7F);
            if octet & 0x80 == 0 {
                return Ok(number);
            }
        }
    }

    /// Parses the length octets (§8.1.3) and, for the definite form, skips
    /// the contents octets they delimit.
    #[inline(always)]
    fn contents(&mut self, identifier: Identifier) -> Result<Contents<'input>> {
        let initial = self.octet()?;
        match initial {
            // Short form (§8.1.3.4): bits 7 to 1 are the length.
            0..=0x7F => self.octets(usize::from(initial)).map(Contents::Definite),
            // Indefinite form (§8.1.3.6): only for constructed encodings, and
            // never in DER (§10.1).
            0x80 if identifier.is_constructed() && self.rules.allows_indefinite() => {
                Ok(Contents::Indefinite)
            }
            0x80 => Err(self.ber_error(BerDecodeErrorKind::IndefiniteLengthNotAllowed)),
            // Reserved for possible future extension (§8.1.3.5 c).
            0xFF => Err(self.ber_error(BerDecodeErrorKind::ReservedLengthOctet)),
            // Long form (§8.1.3.5): bits 7 to 1 count the following octets,
            // which hold the length as an unsigned integer, most significant
            // octet first.
            _ => {
                let length = self.length(usize::from(initial & 0x7F))?;
                self.octets(length).map(Contents::Definite)
            }
        }
    }

    /// Parses the `octet_count` octets of a length in the long form.
    fn length(&mut self, octet_count: usize) -> Result<usize> {
        let mut length = 0usize;
        for &octet in self.octets(octet_count)? {
            length = length
                .checked_mul(256)
                .ok_or_else(|| self.length_exceeds_platform_width(octet_count))?
                | usize::from(octet);
        }
        Ok(length)
    }

    /// Parses the end-of-contents octets that close the indefinite form
    /// (§8.1.5).
    #[inline]
    pub(crate) fn end_of_contents(&mut self) -> Result<()> {
        match self.input.strip_prefix(END_OF_CONTENTS) {
            Some(rest) => {
                self.input = rest;
                Ok(())
            }
            None if END_OF_CONTENTS.starts_with(self.input) => {
                Err(self.incomplete(END_OF_CONTENTS.len() - self.input.len()))
            }
            None => Err(self.ber_error(BerDecodeErrorKind::MissingEndOfContents)),
        }
    }

    /// Parses past the contents of a value in the indefinite form and its
    /// end-of-contents octets, returning the contents. Nested values are
    /// parsed only far enough to find where they end, without recursion, so
    /// the nesting depth of the input cannot exhaust the stack.
    pub(crate) fn indefinite_contents(&mut self) -> Result<&'input [u8]> {
        let start = self.input;
        // The nested values in the indefinite form whose end-of-contents
        // octets have yet to be parsed.
        let mut open = 0usize;
        loop {
            match self.input.strip_prefix(END_OF_CONTENTS) {
                Some(rest) if open == 0 => {
                    let contents = &start[..start.len() - self.input.len()];
                    self.input = rest;
                    return Ok(contents);
                }
                Some(rest) => {
                    self.input = rest;
                    open -= 1;
                }
                None => {
                    if let Contents::Indefinite = self.value(None)?.contents {
                        open += 1;
                    }
                }
            }
        }
    }

    /// Parses a string value with the tag `tag`, whose constructed form
    /// (§8.6.4, §8.7.3, §8.21.3; BER and CER only) nests segments tagged
    /// `segment_tag` that are string values themselves.
    #[inline]
    pub(crate) fn string<T: Segmented>(&mut self, tag: Tag, segment_tag: Tag) -> Result<T> {
        let value = self.value(Some(tag))?;
        self.string_contents(value, segment_tag)
    }

    /// Gathers the string encoded by `value`, whose identifier and length
    /// octets have been parsed, from its segments in order.
    pub(crate) fn string_contents<T: Segmented>(
        &mut self,
        value: Value<'input>,
        segment_tag: Tag,
    ) -> Result<T> {
        let mut string = T::default();
        self.append_string(value, segment_tag, &mut string)?;
        Ok(string)
    }

    fn append_string<T: Segmented>(
        &mut self,
        value: Value<'input>,
        segment_tag: Tag,
        string: &mut T,
    ) -> Result<()> {
        match value.contents {
            Contents::Definite(octets) if value.identifier.is_primitive() => {
                string.append_segment(octets, self.codec())
            }
            _ if !self.rules.allows_constructed_strings() => {
                Err(DerDecodeErrorKind::ConstructedEncodingNotAllowed.into())
            }
            Contents::Definite(octets) => {
                let mut segments = self.nested(octets)?;
                segments.append_segments(segment_tag, string)?;
                segments.finish()
            }
            Contents::Indefinite => {
                let mut segments = self.nested(self.input)?;
                segments.append_segments(segment_tag, string)?;
                segments.end_of_contents()?;
                self.input = segments.input;
                Ok(())
            }
        }
    }

    /// Appends the segments of a constructed string until the input ends or
    /// reaches end-of-contents octets.
    fn append_segments<T: Segmented>(&mut self, segment_tag: Tag, string: &mut T) -> Result<()> {
        while !(self.input.is_empty() || self.input.starts_with(END_OF_CONTENTS)) {
            let segment = self.value(Some(segment_tag))?;
            self.append_string(segment, segment_tag, string)?;
        }
        Ok(())
    }

    /// A parser over `input`, one level deeper.
    fn nested(&self, input: &'input [u8]) -> Result<Self> {
        let depth = self
            .depth
            .checked_sub(1)
            .ok_or_else(|| self.exceeds_max_parse_depth())?;
        Ok(Self {
            rules: self.rules,
            depth,
            input,
        })
    }

    /// Requires the input to be exhausted.
    fn finish(self) -> Result<()> {
        if self.input.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::unexpected_extra_data(
                self.input.len(),
                self.codec(),
            ))
        }
    }

    #[inline]
    fn octet(&mut self) -> Result<u8> {
        let (&octet, rest) = self.input.split_first().ok_or_else(|| self.incomplete(1))?;
        self.input = rest;
        Ok(octet)
    }

    #[inline]
    fn octets(&mut self, count: usize) -> Result<&'input [u8]> {
        let (octets, rest) = self
            .input
            .split_at_checked(count)
            .ok_or_else(|| self.incomplete(count - self.input.len()))?;
        self.input = rest;
        Ok(octets)
    }

    #[cold]
    #[inline(never)]
    fn ber_error(&self, kind: BerDecodeErrorKind) -> DecodeError {
        DecodeError::from_kind(
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(kind),
            },
            self.codec(),
        )
    }

    #[cold]
    #[inline(never)]
    fn mismatched_tag(&self, expected: Tag, actual: Tag) -> DecodeError {
        self.ber_error(BerDecodeErrorKind::MismatchedTag { expected, actual })
    }

    #[cold]
    #[inline(never)]
    fn incomplete(&self, missing: usize) -> DecodeError {
        DecodeError::incomplete(Needed::new(missing), self.codec())
    }

    #[cold]
    #[inline(never)]
    fn length_exceeds_platform_width(&self, octet_count: usize) -> DecodeError {
        DecodeError::length_exceeds_platform_width(
            alloc::format!("{octet_count} length octets"),
            self.codec(),
        )
    }

    #[cold]
    #[inline(never)]
    fn exceeds_max_parse_depth(&self) -> DecodeError {
        DecodeError::from_kind(DecodeErrorKind::ExceedsMaxParseDepth, self.codec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BER_OPTIONS: DecoderOptions = DecoderOptions::ber();
    const CER_OPTIONS: DecoderOptions = DecoderOptions::cer();
    const DER_OPTIONS: DecoderOptions = DecoderOptions::der();

    fn value<'input>(options: DecoderOptions, input: &'input [u8]) -> Result<Value<'input>> {
        Parser::new(options, input).value(None)
    }

    fn definite_contents(options: DecoderOptions, input: &[u8]) -> &[u8] {
        match value(options, input).unwrap().contents {
            Contents::Definite(contents) => contents,
            Contents::Indefinite => panic!("expected the definite form"),
        }
    }

    #[test]
    fn long_tag() {
        let identifier = Parser::new(BER_OPTIONS, &[0xFF, 0x83, 0x7F])
            .identifier()
            .unwrap();
        assert!(identifier.is_constructed());
        assert_eq!(Tag::new(Class::Private, 511), identifier.tag);
    }

    #[test]
    fn numbers_wider_than_u32_are_rejected() {
        let mut parser = Parser::new(BER_OPTIONS, &[0x1F, 0x90, 0x80, 0x80, 0x80, 0x00]);
        assert!(matches!(
            *parser.identifier().unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::UnsupportedTagNumber)
            }
        ));
        let mut parser = Parser::new(BER_OPTIONS, &[0x1F, 0x8F, 0xFF, 0xFF, 0xFF, 0x7F]);
        assert_eq!(parser.identifier().unwrap().tag.value, u32::MAX);

        let mut parser = Parser::new(DER_OPTIONS, &[0x90, 0x80, 0x80, 0x80, 0x00]);
        let error = parser.object_identifier_arc().unwrap_err();
        assert!(matches!(
            *error.kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::UnsupportedObjectIdentifierArc)
            }
        ));
        assert_eq!(error.codec, Codec::Der);
    }

    #[test]
    fn value_long_length_form() {
        assert_eq!(
            definite_contents(BER_OPTIONS, &[0x1, 0x81, 0x2, 0xF0, 0xF0]),
            &[0xF0, 0xF0]
        );
        assert_eq!(
            definite_contents(BER_OPTIONS, &[0x1, 0x82, 0x0, 0x2, 0xF0, 0xF0]),
            &[0xF0, 0xF0]
        );
    }

    #[test]
    fn value_really_long_length_form() {
        let full_buffer = [0xff; 0x100];

        let mut encoding = alloc::vec![0x1, 0x82, 0x1, 0x0];
        encoding.extend_from_slice(&full_buffer);

        assert_eq!(definite_contents(BER_OPTIONS, &encoding), &full_buffer[..]);
    }

    #[test]
    fn length_wider_than_usize() {
        let mut encoding = alloc::vec![0x04, 0x89, 0x01];
        encoding.extend_from_slice(&[0; 8]);
        assert!(matches!(
            *value(BER_OPTIONS, &encoding).unwrap_err().kind,
            DecodeErrorKind::LengthExceedsPlatformWidth { .. }
        ));
    }

    #[test]
    fn reserved_length_octet() {
        assert!(matches!(
            *value(BER_OPTIONS, &[0x04, 0xFF]).unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::ReservedLengthOctet)
            }
        ));
    }

    #[test]
    fn value_indefinite_length_form() {
        let bytes = &[0x30, 0x80, 0xf0, 0xf0, 0xf0, 0xf0, 0, 0];
        assert!(matches!(
            value(BER_OPTIONS, bytes).unwrap().contents,
            Contents::Indefinite
        ));
        assert!(matches!(
            value(CER_OPTIONS, bytes).unwrap().contents,
            Contents::Indefinite
        ));
        assert!(matches!(
            *value(DER_OPTIONS, bytes).unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::IndefiniteLengthNotAllowed)
            }
        ));
        // A primitive encoding has no indefinite form.
        assert!(matches!(
            *value(BER_OPTIONS, &[0x04, 0x80, 0, 0]).unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::IndefiniteLengthNotAllowed)
            }
        ));
    }

    #[test]
    fn truncated_input_reports_the_missing_octets() {
        fn missing(input: &[u8]) -> Needed {
            match *value(BER_OPTIONS, input).unwrap_err().kind {
                DecodeErrorKind::Incomplete { needed } => needed,
                ref kind => panic!("expected an incomplete error, got {kind:?}"),
            }
        }
        assert_eq!(missing(&[]), Needed::new(1));
        assert_eq!(missing(&[0x04]), Needed::new(1));
        assert_eq!(missing(&[0x04, 0x05, 1, 2]), Needed::new(3));
        assert_eq!(missing(&[0x04, 0x82, 0x01]), Needed::new(1));
        assert_eq!(missing(&[0x1F, 0x81]), Needed::new(1));
    }

    #[test]
    fn end_of_contents_where_a_value_is_expected() {
        assert!(matches!(
            *value(BER_OPTIONS, &[0x00, 0x00]).unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::UnexpectedEndOfContents)
            }
        ));
    }

    #[test]
    fn end_of_contents() {
        let mut parser = Parser::new(BER_OPTIONS, &[0, 0, 0x05]);
        parser.end_of_contents().unwrap();
        assert_eq!(parser.remaining(), &[0x05]);

        let mut parser = Parser::new(BER_OPTIONS, &[0]);
        assert!(matches!(
            *parser.end_of_contents().unwrap_err().kind,
            DecodeErrorKind::Incomplete { needed } if needed == Needed::new(1)
        ));

        let mut parser = Parser::new(BER_OPTIONS, &[0x05, 0x00]);
        assert!(matches!(
            *parser.end_of_contents().unwrap_err().kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::MissingEndOfContents)
            }
        ));
    }

    #[test]
    fn indefinite_contents_span_nested_values() {
        // SEQUENCE { SEQUENCE { INTEGER 1 } OCTET STRING '00'H } followed by
        // an INTEGER that is not part of the value.
        let input = &[
            0x30, 0x80, 0x30, 0x80, 0x02, 0x01, 0x01, 0x00, 0x00, 0x04, 0x01, 0x00, 0x00, 0x00,
            0x02, 0x01, 0x02,
        ];
        let mut parser = Parser::new(BER_OPTIONS, input);
        assert!(matches!(
            parser.value(Some(Tag::SEQUENCE)).unwrap().contents,
            Contents::Indefinite
        ));
        assert_eq!(parser.indefinite_contents().unwrap(), &input[2..12]);
        assert_eq!(parser.remaining(), &input[14..]);
    }

    #[test]
    fn indefinite_contents_without_recursion() {
        // Deeper than any stack could take recursively.
        let depth = 1_000_000;
        let mut input = Vec::with_capacity(4 * depth);
        input.extend((0..depth).flat_map(|_| [0x30, 0x80]));
        input.extend((0..depth).flat_map(|_| [0x00, 0x00]));
        let mut parser = Parser::new(BER_OPTIONS, &input);
        parser.value(None).unwrap();
        assert_eq!(parser.indefinite_contents().unwrap().len(), 4 * depth - 4);
        assert!(parser.remaining().is_empty());
    }

    #[test]
    fn constructed_strings_concatenate_their_segments() {
        // Segments may be primitive or constructed, in either length form.
        let indefinite = &[
            0x24, 0x80, 0x04, 0x01, 0xA1, 0x24, 0x03, 0x04, 0x01, 0xB2, 0x24, 0x80, 0x04, 0x01,
            0xC3, 0x00, 0x00, 0x00, 0x00,
        ];
        let definite = &[
            0x24, 0x0F, 0x04, 0x01, 0xA1, 0x24, 0x03, 0x04, 0x01, 0xB2, 0x24, 0x80, 0x04, 0x01,
            0xC3, 0x00, 0x00,
        ];
        for input in [&indefinite[..], &definite[..]] {
            let mut parser = Parser::new(BER_OPTIONS, input);
            let string: Vec<u8> = parser.string(Tag::OCTET_STRING, Tag::OCTET_STRING).unwrap();
            assert_eq!(string, [0xA1, 0xB2, 0xC3]);
            assert!(parser.remaining().is_empty());
        }
        assert!(matches!(
            *Parser::new(DER_OPTIONS, definite)
                .string::<Vec<u8>>(Tag::OCTET_STRING, Tag::OCTET_STRING)
                .unwrap_err()
                .kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Der(DerDecodeErrorKind::ConstructedEncodingNotAllowed)
            }
        ));
    }

    #[test]
    fn constructed_string_segments_must_carry_the_segment_tag() {
        let input = &[0x24, 0x03, 0x0C, 0x01, 0xA1];
        assert!(matches!(
            *Parser::new(BER_OPTIONS, input)
                .string::<Vec<u8>>(Tag::OCTET_STRING, Tag::OCTET_STRING)
                .unwrap_err()
                .kind,
            DecodeErrorKind::CodecSpecific {
                inner: CodecDecodeError::Ber(BerDecodeErrorKind::MismatchedTag { .. })
            }
        ));
    }

    #[test]
    fn constructed_string_of_definite_length_must_be_exhausted_by_its_segments() {
        // End-of-contents octets inside definite-length contents are stray.
        let input = &[0x24, 0x05, 0x04, 0x01, 0xA1, 0x00, 0x00];
        assert!(matches!(
            *Parser::new(BER_OPTIONS, input)
                .string::<Vec<u8>>(Tag::OCTET_STRING, Tag::OCTET_STRING)
                .unwrap_err()
                .kind,
            DecodeErrorKind::UnexpectedExtraData { length: 2 }
        ));
    }

    #[test]
    fn constructed_string_nesting_is_bounded() {
        let depth = DecoderOptions::ber().remaining_depth + 1;
        let mut input = Vec::with_capacity(2 * depth);
        input.extend((0..depth).flat_map(|_| [0x24, 0x80]));
        input.extend((0..depth).flat_map(|_| [0x00, 0x00]));
        assert!(matches!(
            *Parser::new(BER_OPTIONS, &input)
                .string::<Vec<u8>>(Tag::OCTET_STRING, Tag::OCTET_STRING)
                .unwrap_err()
                .kind,
            DecodeErrorKind::ExceedsMaxParseDepth
        ));
    }

    #[test]
    fn bit_string_segments() {
        let mut string = BitString::new();
        string
            .append_segment(&[0, 0b1010_0000], Codec::Ber)
            .unwrap();
        string
            .append_segment(&[5, 0b1110_0000], Codec::Ber)
            .unwrap();
        assert_eq!(
            string,
            bitvec::bitvec![u8, Msb0; 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1]
        );
        assert_eq!(string.as_raw_slice(), [0b1010_0000, 0b1110_0000]);

        let mut empty = BitString::new();
        empty.append_segment(&[0], Codec::Ber).unwrap();
        assert!(empty.is_empty());

        for invalid in [&[][..], &[8, 0xFF], &[1]] {
            assert!(
                BitString::new()
                    .append_segment(invalid, Codec::Ber)
                    .is_err()
            );
        }
    }
}
