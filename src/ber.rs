mod error;
mod identifier;
mod parser;

use snafu::*;

use crate::{tag::Tag, types, Decode, Decoder};

pub use self::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn decode<T: Decode>(slice: &[u8]) -> Result<T> {
    T::decode(Ber, slice)
}

// pub fn encode<T, W>(writer: &mut W, value: &T) -> Result<T> {
//     todo!()
// }

struct Ber;

impl Decoder for Ber {
    type Error = Error;

    fn decode_bool(&self, slice: &[u8]) -> Result<bool> {
        let (_, (identifier, contents)) = self::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        error::assert_tag(Tag::BOOL, identifier.tag)?;
        error::assert_length(1, contents.len())?;
        Ok(contents[0] != 0)
    }

    fn decode_integer(&self, slice: &[u8]) -> Result<types::Integer> {
        let (_, (identifier, contents)) = self::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        error::assert_tag(Tag::INTEGER, identifier.tag)?;
        Ok(types::Integer::from_signed_bytes_be(contents))
    }

    fn decode_octet_string(&self, tag: Tag, slice: &[u8]) -> Result<types::OctetString> {
        let (_, (identifier, mut contents)) = self::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        error::assert_tag(tag, identifier.tag)?;

        if identifier.is_primitive() {
            Ok(contents.to_vec().into())
        } else {
            let mut buffer = alloc::vec![];
            while !contents.is_empty() {
                let (c, mut vec) = self::parser::parse_encoded_value(Tag::OCTET_STRING, contents, |input| Ok(alloc::vec::Vec::from(input)))?;
                contents = c;

                buffer.append(&mut vec);
            }

            Ok(buffer.into())
        }

    }

    fn decode_null(&self, slice: &[u8]) -> Result<()> {
        let (_, (identifier, contents)) = self::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        error::assert_tag(Tag::NULL, identifier.tag)?;
        error::assert_length(0, contents.len())
    }

    fn decode_object_identifier(&self, slice: &[u8]) -> Result<crate::types::ObjectIdentifier> {
        use num_traits::ToPrimitive;
        let (_, (identifier, contents)) = self::parser::parse_value(slice)
            .ok()
            .context(error::Parser)?;
        error::assert_tag(Tag::OBJECT_IDENTIFIER, identifier.tag)?;
        let (input, root_octets) = parser::parse_encoded_number(contents)
            .ok()
            .context(error::Parser)?;
        let second = (&root_octets % 40u8)
            .to_u32()
            .expect("Second root component greater than `u32`");
        let first = ((root_octets - second) / 40u8)
            .to_u32()
            .expect("first root component greater than `u32`");
        let mut buffer = alloc::vec![first, second];

        let mut input = input;
        while !input.is_empty() {
            let (new_input, number) = parser::parse_encoded_number(input)
                .ok()
                .context(error::Parser)?;
            input = new_input;
            buffer.push(number.to_u32().expect("sub component greater than `u32`"));
        }

        Ok(crate::types::ObjectIdentifier::new(buffer))
    }

    fn decode_bit_string(&self, slice: &[u8]) -> Result<types::BitString> {
        self::parser::parse_encoded_value(Tag::BIT_STRING, slice, |input| {
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
        }).map(|(_, bs)| bs)
    }

    fn decode_utf8_string(&self, slice: &[u8]) -> Result<types::Utf8String> {
        self.decode_octet_string(Tag::UTF8_STRING, slice)
            .and_then(|bytes| types::Utf8String::from_utf8(bytes.to_vec()).ok().context(error::InvalidUtf8))
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
}
