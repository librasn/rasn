mod error;
mod identifier;
mod parser;

use alloc::vec::Vec;

use snafu::*;

use crate::{tag::Tag, types, Decode, Decoder};

pub use self::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn decode<T: Decode>(input: &[u8]) -> Result<T> {
    T::decode(&mut Parser { input })
}

// pub fn encode<T, W>(writer: &mut W, value: &T) -> Result<T> {
//     todo!()
// }

struct Parser<'input> {
    input: &'input [u8],
}

impl<'input> Parser<'input> {
    fn parse_value(&mut self, tag: Tag) -> Result<(self::identifier::Identifier, &'input [u8])> {
        let (input, (identifier, contents)) = self::parser::parse_value(self.input, tag)
            .ok()
            .context(error::Parser)?;
        self.input = input;
        Ok((identifier, contents))
    }
}

impl<'input> Decoder for &mut Parser<'input> {
    type Error = Error;

    fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn decode_bool(self, tag: Tag) -> Result<bool> {
        let (_, contents) = self.parse_value(tag)?;
        error::assert_length(1, contents.len())?;
        Ok(contents[0] != 0)
    }

    fn decode_integer(self, tag: Tag) -> Result<types::Integer> {
        Ok(types::Integer::from_signed_bytes_be(self.parse_value(tag)?.1))
    }

    fn decode_octet_string(self, tag: Tag) -> Result<types::OctetString> {
        let (identifier, mut contents) = self.parse_value(tag)?;

        if identifier.is_primitive() {
            Ok(contents.to_vec().into())
        } else {
            let mut buffer = alloc::vec![];
            while !contents.is_empty() {
                let (c, mut vec) = self::parser::parse_encoded_value(contents, Tag::OCTET_STRING, |input| Ok(alloc::vec::Vec::from(input)))?;
                contents = c;

                buffer.append(&mut vec);
            }

            Ok(buffer.into())
        }

    }

    fn decode_null(self, tag: Tag) -> Result<()> {
        let (_, contents) = self.parse_value(tag)?;
        error::assert_length(0, contents.len())?;
        Ok(())
    }

    fn decode_object_identifier(self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        use num_traits::ToPrimitive;
        let contents = self.parse_value(tag)?.1;
        let (mut contents, root_octets) = parser::parse_encoded_number(contents)
            .ok()
            .context(error::Parser)?;
        let second = (&root_octets % 40u8)
            .to_u32()
            .expect("Second root component greater than `u32`");
        let first = ((root_octets - second) / 40u8)
            .to_u32()
            .expect("first root component greater than `u32`");
        let mut buffer = alloc::vec![first, second];

        while !contents.is_empty() {
            let (c, number) = parser::parse_encoded_number(contents)
                .ok()
                .context(error::Parser)?;
            contents = c;
            buffer.push(number.to_u32().expect("sub component greater than `u32`"));
        }

        Ok(crate::types::ObjectIdentifier::new(buffer))
    }

    fn decode_bit_string(self, tag: Tag) -> Result<types::BitString> {
        self::parser::parse_encoded_value(self.input, tag, |input| {
            let unused_bits = input[0];

            match unused_bits {
                0..=7 => {
                    let mut buffer = types::BitString::from(&input[1..]);

                    for _ in 0..unused_bits {
                        buffer.pop();
                    }

                    Ok(buffer)
                }
                _ => return Err(Error::InvalidBitString { bits: unused_bits }),
            }
        }).map(|(_, rv)| rv)
    }

    fn decode_utf8_string(self, tag: Tag) -> Result<types::Utf8String> {
        let vec = self.decode_octet_string(tag)?.to_vec();
        types::Utf8String::from_utf8(vec).ok().context(error::InvalidUtf8)
    }


    fn decode_sequence_of<D: Decode>(self, tag: Tag) -> Result<Vec<D>> {
        let contents = self.parse_value(tag)?.1;
        let mut vec = Vec::new();

        let mut sequence_parser = Parser { input: contents };

        while !Decoder::is_empty(&&mut sequence_parser) {
            let value = D::decode(&mut sequence_parser)?;
            vec.push(value);
        }

        Ok(vec)
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
        let mut bigint = types::Integer::from(1);
        bigint <<= 2048;
        assert_eq!(bigint, decode(&data).unwrap());
    }

    #[test]
    fn oid_from_bytes() {
        let oid = types::ObjectIdentifier::new(alloc::vec![1, 2, 840, 113549]);
        let from_raw = decode(&[0x6, 0x6, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d][..]).unwrap();

        assert_eq!(oid, from_raw);
    }

    #[test]
    fn octet_string() {
        let octet_string = types::OctetString::from(alloc::vec![1, 2, 3, 4, 5, 6]);
        let primitive_encoded = &[0x4, 0x6, 1, 2, 3, 4, 5, 6];
        let constructed_encoded = &[0x24, 0x80, 0x4, 0x4, 1, 2, 3, 4, 0x4, 0x2, 5, 6, 0x0, 0x0];

        assert_eq!(
            octet_string,
            decode::<types::OctetString>(primitive_encoded).unwrap()
        );
        assert_eq!(
            octet_string,
            decode::<types::OctetString>(constructed_encoded).unwrap()
        );
    }

    #[test]
    fn bit_string() {
        let bitstring = {
            let mut b = types::BitString::from_vec(alloc::vec![0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0]);

            for _ in 0..4 {
                b.pop();
            }

            b
        };

        let primitive_encoded: types::BitString =
            decode(&[0x03, 0x07, 0x04, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..]).unwrap();

        let constructed_encoded: types::BitString = decode(
            &[
                0x23, 0x80, // TAG + LENGTH
                0x03, 0x03, 0x00, 0x0A, 0x3B, // Part 1
                0x03, 0x05, 0x04, 0x5F, 0x29, 0x1C, 0xD0, // Part 2
                0x00, 0x00, // EOC
            ][..],
        )
        .map_err(|e| panic!("{}", e))
        .unwrap();

        assert_eq!(bitstring, primitive_encoded);
        assert_eq!(bitstring, constructed_encoded);
    }

    #[test]
    fn utf8_string() {
        use alloc::string::String;
        let name = String::from("Jones");
        let primitive = &[
            0x0C, 0x05,
            0x4A, 0x6F, 0x6E, 0x65, 0x73
        ];
        let definite_constructed = &[
            0x2C, 0x09, // TAG + LENGTH
                0x04, 0x03, // PART 1 TLV
                0x4A, 0x6F, 0x6E,
                0x04, 0x02, // PART 2 TLV
                0x65, 0x73
        ];
        let indefinite_constructed = &[
            0x2C, 0x80, // TAG + LENGTH
                0x04, 0x03, // PART 1 TLV
                0x4A, 0x6F, 0x6E,
                0x04, 0x02, // PART 2 TLV
                0x65, 0x73,
            0x00, 0x00,
        ];

        assert_eq!(name, decode::<String>(primitive).unwrap());
        assert_eq!(name, decode::<String>(definite_constructed).unwrap());
        assert_eq!(name, decode::<String>(indefinite_constructed).unwrap());
    }

    #[test]
    fn sequence_of() {
        let vec = alloc::vec!["Jon", "es"];
        let from_raw: Vec<alloc::string::String> = decode(&[
            0x30, 0x9,
            0x0C, 0x03, 0x4A, 0x6F, 0x6E,
            0x0C, 0x02, 0x65, 0x73
        ][..]).unwrap();

        assert_eq!(vec, from_raw);
    }
}
