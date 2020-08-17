use alloc::vec::Vec;

use nom::IResult;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::identifier::{Class, Identifier};

pub(crate) fn parse_value(input: &[u8]) -> IResult<&[u8], (Identifier, &[u8])> {
    let (input, identifier) = parse_identifier_octet(input)?;
    let (input, contents) = parse_contents(input)?;

    Ok((input, (identifier, contents)))
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

pub(crate) fn parse_encoded_number(input: &[u8]) -> IResult<&[u8], BigInt> {
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

/// Concatenates a series of 7 bit numbers delimited by `1`'s and
/// ended by a `0` in the 8th bit.
fn concat_number(body: &[u8], end: u8) -> BigInt {
    let mut number = BigInt::new(num_bigint::Sign::NoSign, Vec::new());

    for byte in body {
        number <<= 7usize;
        number |= BigInt::from(byte & 0x7F);
    }

    // end doesn't need to be bitmasked as we know the MSB is `0`
    // (X.690 8.1.2.4.2.a).
    number <<= 7usize;
    number |= BigInt::from(end);

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
