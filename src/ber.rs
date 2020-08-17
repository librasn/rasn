use num_bigint::BigInt;
use snafu::OptionExt;

use crate::{
    error::{self, Error},
    identifier::Tag,
    Decode, Decoder, Result,
};

pub fn decode<T: Decode>(slice: &[u8]) -> Result<T> {
    T::decode(Ber, slice)
}

pub fn encode<T, W>(writer: &mut W, value: &T) -> Result<T> {
    todo!()
}

struct Ber;

fn assert_tag(expected: Tag, actual: Tag) -> Result<()> {
    if expected != actual {
        Err(Error::MismatchedTag { expected, actual })
    } else {
        Ok(())
    }
}

fn assert_length(expected: usize, actual: usize) -> Result<()> {
    if expected != actual {
        Err(Error::MismatchedLength { expected, actual })
    } else {
        Ok(())
    }
}

impl Decoder for Ber {
    fn decode_bool(&self, slice: &[u8]) -> Result<bool> {
        let (_, (identifier, contents)) = crate::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        assert_tag(Tag::BOOL, identifier.tag)?;
        assert_length(1, contents.len())?;
        Ok(contents[0] != 0)
    }

    fn decode_integer(&self, slice: &[u8]) -> Result<BigInt> {
        let (_, (identifier, contents)) = crate::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        assert_tag(Tag::INTEGER, identifier.tag)?;
        Ok(BigInt::from_signed_bytes_be(contents))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean() {
        assert_eq!(true, decode(&[0x01, 0x01, 0xff]).unwrap());
        assert_eq!(false, decode(&[0x01, 0x01, 0x00]).unwrap());
    }

    #[test]
    fn integer() {
        assert_eq!(32768, decode(&[0x02, 0x03, 0x00, 0x80, 0x00,]).unwrap());
        assert_eq!(32767, decode(&[0x02, 0x02, 0x7f, 0xff]).unwrap());
        assert_eq!(256, decode(&[0x02, 0x02, 0x01, 0x00]).unwrap());
        assert_eq!(255, decode(&[0x02, 0x02, 0x00, 0xff]).unwrap());
        assert_eq!(128, decode(&[0x02, 0x02, 0x00, 0x80]).unwrap());
        assert_eq!(127, decode(&[0x02, 0x01, 0x7f]).unwrap());
        assert_eq!(1, decode(&[0x02, 0x01, 0x01]).unwrap());
        assert_eq!(0, decode(&[0x02, 0x01, 0x00]).unwrap());
        assert_eq!(-1, decode(&[0x02, 0x01, 0xff]).unwrap());
        assert_eq!(-128, decode(&[0x02, 0x01, 0x80]).unwrap());
        assert_eq!(-129, decode(&[0x02, 0x02, 0xff, 0x7f]).unwrap());
        assert_eq!(-256, decode(&[0x02, 0x02, 0xff, 0x00]).unwrap());
        assert_eq!(-32768, decode(&[0x02, 0x02, 0x80, 0x00]).unwrap());
        assert_eq!(-32769, decode(&[0x02, 0x03, 0xff, 0x7f, 0xff]).unwrap());

        let mut data = [0u8; 261];
        data[0] = 0x02;
        data[1] = 0x82;
        data[2] = 0x01;
        data[3] = 0x01;
        data[4] = 0x01;
        let mut bigint = BigInt::from(1);
        bigint <<= 2048;
        assert_eq!(bigint, decode(&data).unwrap());
    }
}
