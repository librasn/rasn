use alloc::vec::Vec;

use nom::IResult;
use num_traits::ToPrimitive;

use super::{error, DecoderOptions};
use crate::{
    ber::identifier::Identifier,
    tag::{Class, Tag},
    types::Integer,
};

pub(crate) fn parse_value<'config, 'input>(
    config: &'config DecoderOptions,
    input: &'input [u8],
    tag: Tag,
) -> super::Result<(&'input [u8], (Identifier, &'input [u8]))> {
    let (input, identifier) = parse_identifier_octet(input).map_err(error::map_nom_err)?;
    error::assert_tag(tag, identifier.tag)?;
    let (input, contents) =
        parse_contents(config, identifier, input).map_err(error::map_nom_err)?;

    Ok((input, (identifier, contents)))
}

pub(crate) fn parse_encoded_value<'config, 'input, RV>(
    config: &'config DecoderOptions,
    slice: &'input [u8],
    tag: Tag,
    primitive_callback: fn(&'input [u8]) -> super::Result<RV>,
) -> super::Result<(&'input [u8], RV)>
where
    RV: Appendable,
{
    let (input, (identifier, contents)) = parse_value(config, slice, tag)?;

    if identifier.is_primitive() {
        Ok((input, (primitive_callback)(contents)?))
    } else if config.encoding_rules.allows_constructed_strings() {
        let mut container = RV::new();
        let mut contents = contents;

        while !contents.is_empty() {
            let (c, mut embedded_container) =
                parse_encoded_value(config, contents, tag, primitive_callback)?;
            contents = c;
            container.append(&mut embedded_container)
        }

        Ok((input, container))
    } else {
        Err(error::Error::ConstructedEncodingNotAllowed)
    }
}

pub(crate) fn parse_identifier_octet(input: &[u8]) -> IResult<&[u8], Identifier> {
    let (input, identifier) = parse_initial_octet(input)?;

    let (input, tag) = if identifier.tag.value >= 0x1f {
        let (input, tag) = parse_encoded_number(input)?;

        match tag.to_u32() {
            Some(value) => (input, value),
            None => {
                use nom::error::ParseError;
                return Err(nom::Err::Failure(<_>::from_error_kind(
                    input,
                    nom::error::ErrorKind::TooLarge,
                )));
            }
        }
    } else {
        (input, identifier.tag.value)
    };

    Ok((input, identifier.tag(tag)))
}

pub(crate) fn parse_encoded_number(input: &[u8]) -> IResult<&[u8], Integer> {
    let (input, body) = nom::bytes::streaming::take_while(|i| i & 0x80 != 0)(input)?;
    let (input, end) = nom::bytes::streaming::take(1usize)(input)?;

    Ok((input, concat_number(body, end[0])))
}

fn parse_initial_octet(input: &[u8]) -> IResult<&[u8], Identifier> {
    let (input, octet) = nom::bytes::streaming::take(1usize)(input)?;
    let initial_octet = octet[0];

    let class_bits = (initial_octet & 0xC0) >> 6;
    let class = Class::from_u8(class_bits);
    let constructed = (initial_octet & 0x20) != 0;
    let tag = (initial_octet & 0x1f) as u32;

    Ok((input, Identifier::new(class, constructed, tag)))
}

pub(crate) fn parse_contents<'input, 'config>(
    config: &'config DecoderOptions,
    identifier: Identifier,
    input: &'input [u8],
) -> IResult<&'input [u8], &'input [u8]> {
    let (input, length) = nom::bytes::streaming::take(1usize)(input)?;
    take_contents(config, input, identifier, length[0])
}

pub(crate) trait Appendable: Sized {
    fn new() -> Self;
    fn append(&mut self, other: &mut Self);
}

impl Appendable for Vec<u8> {
    fn new() -> Self {
        Self::new()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }
}

impl Appendable for crate::types::BitString {
    fn new() -> Self {
        Self::new()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }
}

/// Concatenates a series of 7 bit numbers delimited by `1`'s and
/// ended by a `0` in the 8th bit.
fn concat_number(body: &[u8], end: u8) -> Integer {
    let mut number = Integer::new(num_bigint::Sign::NoSign, Vec::new());

    for byte in body {
        number <<= 7usize;
        number |= Integer::from(byte & 0x7F);
    }

    // end doesn't need to be bitmasked as we know the MSB is `0`
    // (X.690 8.1.2.4.2.a).
    number <<= 7usize;
    number |= Integer::from(end);

    number
}

fn concat_bits(body: &[u8], width: u8) -> usize {
    let mut result: usize = 0;

    for byte in body {
        result <<= width;
        result |= *byte as usize;
    }

    result
}

fn take_contents<'config, 'input>(
    config: &'config DecoderOptions,
    input: &'input [u8],
    identifier: Identifier,
    length: u8,
) -> IResult<&'input [u8], &'input [u8]> {
    if length == 0x80 && (identifier.is_primitive() || !config.encoding_rules.allows_indefinite()) {
        return nom::error::context("Indefinite length not allowed", |_| {
            Err(nom::Err::Failure((input, nom::error::ErrorKind::Tag)))
        })(input);
    }

    if length == 0x80 {
        const EOC_OCTET: &[u8] = &[0, 0];
        let (input, contents) = nom::bytes::streaming::take_until(EOC_OCTET)(input)?;
        let (input, _) = nom::bytes::streaming::tag(EOC_OCTET)(input)?;

        Ok((input, contents))
    } else if length >= 0x7f {
        let length = length ^ 0x80;
        let (input, length_slice) = nom::bytes::streaming::take(length)(input)?;
        let length = concat_bits(&length_slice, 8);
        nom::bytes::streaming::take(length)(input)
    } else if length == 0 {
        Ok((input, &[]))
    } else {
        nom::bytes::streaming::take(length)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BER_OPTIONS: DecoderOptions = DecoderOptions::ber();
    const CER_OPTIONS: DecoderOptions = DecoderOptions::cer();
    const DER_OPTIONS: DecoderOptions = DecoderOptions::der();

    #[test]
    fn value_long_length_form() {
        let (_, (_, contents)) = parse_value(
            &BER_OPTIONS,
            [0x1, 0x81, 0x2, 0xF0, 0xF0][..].into(),
            Tag::BOOL,
        )
        .unwrap();

        assert_eq!(contents, &[0xF0, 0xF0]);
    }

    #[test]
    fn value_really_long_length_form() {
        let full_buffer = [0xff; 0x100];

        let mut value = alloc::vec![0x1, 0x82, 0x1, 0x0];
        value.extend_from_slice(&full_buffer);

        let (_, (_, contents)) = parse_value(&BER_OPTIONS, (&*value).into(), Tag::BOOL).unwrap();

        assert_eq!(contents, &full_buffer[..]);
    }

    #[test]
    fn value_indefinite_length_form() {
        let bytes = [0x30, 0x80, 0xf0, 0xf0, 0xf0, 0xf0, 0, 0][..].into();
        assert!(parse_value(&BER_OPTIONS, bytes, Tag::SEQUENCE).is_ok());
        assert!(parse_value(&DER_OPTIONS, bytes, Tag::SEQUENCE).is_err());
        assert!(parse_value(&CER_OPTIONS, bytes, Tag::SEQUENCE).is_ok());
    }
}
