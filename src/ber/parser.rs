use alloc::vec::Vec;

use nom::IResult;
use num_traits::ToPrimitive;
use snafu::OptionExt;

use super::{error, identifier::Identifier};
use crate::tag::{Class, Tag};
use crate::types::Integer;

pub(crate) fn parse_value(input: &[u8]) -> IResult<&[u8], (Identifier, &[u8])> {
    let (input, identifier) = parse_identifier_octet(input)?;
    let (input, contents) = parse_contents(input)?;

    Ok((input, (identifier, contents)))
}

pub(crate) fn parse_encoded_value<RV>(
    tag: Tag,
    slice: &[u8],
    primitive_callback: fn(&[u8]) -> super::Result<RV>,
) -> super::Result<(&[u8], RV)>
    where
        RV: Appendable,
{
    let (input, (identifier, contents)) = parse_value(slice).ok().context(error::Parser)?;
    error::assert_tag(tag, identifier.tag)?;

    if identifier.is_primitive() {
        Ok((input, (primitive_callback)(contents)?))
    } else {
        let mut container = RV::new();
        let mut contents = contents;

        while !contents.is_empty() {
            let (c, mut embedded_container) = parse_encoded_value(tag, contents, primitive_callback)?;
            contents = c;
            container.append(&mut embedded_container)
        }

        Ok((input, container))
    }
}


pub(crate) fn parse_identifier_octet(input: &[u8]) -> IResult<&[u8], Identifier> {
    let (input, identifier) = parse_initial_octet(input)?;

    let (input, tag) = if identifier.tag.value >= 0x1f {
        let (input, tag) = parse_encoded_number(input)?;

        (input, tag.to_u32().expect("Tag was larger than `u32`."))
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

pub(crate) fn parse_contents(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = nom::bytes::streaming::take(1usize)(input)?;
    take_contents(input, length[0])
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

fn take_contents(input: &[u8], length: u8) -> IResult<&[u8], &[u8]> {
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

    #[test]
    fn value_long_length_form() {
        let (_, (_, contents)) = parse_value([0x1, 0x81, 0x2, 0xF0, 0xF0][..].into()).unwrap();

        assert_eq!(contents, &[0xF0, 0xF0]);
    }

    #[test]
    fn value_really_long_length_form() {
        let full_buffer = [0xff; 0x100];

        let mut value = alloc::vec![0x1, 0x82, 0x1, 0x0];
        value.extend_from_slice(&full_buffer);

        let (_, (_, contents)) = parse_value((&*value).into()).unwrap();

        assert_eq!(contents, &full_buffer[..]);
    }

    #[test]
    fn value_indefinite_length_form() {
        let (_, (_, contents)) =
            parse_value([0x1, 0x80, 0xf0, 0xf0, 0xf0, 0xf0, 0, 0][..].into()).unwrap();

        assert_eq!(contents, &[0xf0, 0xf0, 0xf0, 0xf0]);
    }
}
