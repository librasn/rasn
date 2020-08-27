mod parser;

use alloc::{collections::BTreeSet, vec::Vec};

use snafu::*;

use crate::{tag::Tag, types, Decode, Decoder};
use super::{identifier::Identifier, error::{self, Error}, Result};

pub(crate) struct Parser<'input> {
    input: &'input [u8],
}

impl<'input> Parser<'input> {
    pub(crate) fn new(input: &'input [u8]) -> Self {
        Self { input }
    }

    pub(crate) fn parse_value(&mut self, tag: Tag) -> Result<(Identifier, &'input [u8])> {
        let (input, (identifier, contents)) = self::parser::parse_value(self.input, tag)?;
        self.input = input;
        Ok((identifier, contents))
    }
}

impl<'input> Decoder for Parser<'input> {
    type Error = Error;

    fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool> {
        let (_, contents) = self.parse_value(tag)?;
        error::assert_length(1, contents.len())?;
        Ok(contents[0] != 0)
    }

    fn decode_integer(&mut self, tag: Tag) -> Result<types::Integer> {
        Ok(types::Integer::from_signed_bytes_be(
            self.parse_value(tag)?.1,
        ))
    }

    fn decode_octet_string(&mut self, tag: Tag) -> Result<types::OctetString> {
        let (identifier, mut contents) = self.parse_value(tag)?;

        if identifier.is_primitive() {
            Ok(contents.to_vec().into())
        } else {
            let mut buffer = alloc::vec![];
            while !contents.is_empty() {
                let (c, mut vec) =
                    self::parser::parse_encoded_value(contents, Tag::OCTET_STRING, |input| {
                        Ok(alloc::vec::Vec::from(input))
                    })?;
                contents = c;

                buffer.append(&mut vec);
            }

            Ok(buffer.into())
        }
    }

    fn decode_null(&mut self, tag: Tag) -> Result<()> {
        let (_, contents) = self.parse_value(tag)?;
        error::assert_length(0, contents.len())?;
        Ok(())
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        use num_traits::ToPrimitive;
        let contents = self.parse_value(tag)?.1;
        let (mut contents, root_octets) = parser::parse_encoded_number(contents)
            .map_err(error::map_nom_err)?;
        let second = (&root_octets % 40u8)
            .to_u32()
            .expect("Second root component greater than `u32`");
        let first = ((root_octets - second) / 40u8)
            .to_u32()
            .expect("first root component greater than `u32`");
        let mut buffer = alloc::vec![first, second];

        while !contents.is_empty() {
            let (c, number) = parser::parse_encoded_number(contents)
                .map_err(error::map_nom_err)?;
            contents = c;
            buffer.push(number.to_u32().expect("sub component greater than `u32`"));
        }

        Ok(crate::types::ObjectIdentifier::new(buffer))
    }

    fn decode_bit_string(&mut self, tag: Tag) -> Result<types::BitString> {
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
        })
        .map(|(_, rv)| rv)
    }

    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String> {
        let vec = self.decode_octet_string(tag)?.to_vec();
        types::Utf8String::from_utf8(vec)
            .ok()
            .context(error::InvalidUtf8)
    }

    fn decode_sequence_of<D: Decode>(&mut self, tag: Tag) -> Result<Vec<D>> {
        let contents = self.parse_value(tag)?.1;
        let mut vec = Vec::new();

        let mut sequence_parser = Parser { input: contents };

        while !Decoder::is_empty(&sequence_parser) {
            let value = D::decode(&mut sequence_parser)?;
            vec.push(value);
        }

        Ok(vec)
    }

    fn decode_set_of<D: Decode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>> {
        let contents = self.parse_value(tag)?.1;
        let mut vec = BTreeSet::new();

        let mut set_parser = Parser { input: contents };

        while !Decoder::is_empty(&set_parser) {
            let value = D::decode(&mut set_parser)?;
            vec.insert(value);
        }

        Ok(vec)
    }

    fn decode_set(&mut self, tag: Tag) -> Result<Self> {
        self.decode_sequence(tag)
    }

    fn decode_sequence(&mut self, tag: Tag) -> Result<Self> {
        let contents = self.parse_value(tag)?.1;

        Ok(Parser { input: contents })
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        D::decode(&mut self.decode_sequence(tag)?)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::{ber::decode, tag};
    use super::*;

    #[test]
    fn boolean() {
        assert_eq!(true, decode(&[0x01, 0x01, 0xff]).unwrap());
        assert_eq!(false, decode(&[0x01, 0x01, 0x00]).unwrap());
    }

    #[test]
    fn tagged_boolean() {
        #[derive(Debug, PartialEq, Eq)]
        struct A2;
        impl types::AsnType for A2 {
            const TAG: Tag = Tag::new(tag::Class::Context, 2);
        }

        assert_eq!(
            tag::Explicit::<A2, _>::new(true),
            decode(&[0xa2, 0x03, 0x01, 0x01, 0xff]).unwrap()
        );
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
        let name = String::from("Jones");
        let primitive = &[0x0C, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73];
        let definite_constructed = &[
            0x2C, 0x09, // TAG + LENGTH
            0x04, 0x03, // PART 1 TLV
            0x4A, 0x6F, 0x6E, 0x04, 0x02, // PART 2 TLV
            0x65, 0x73,
        ];
        let indefinite_constructed = &[
            0x2C, 0x80, // TAG + LENGTH
            0x04, 0x03, // PART 1 TLV
            0x4A, 0x6F, 0x6E, 0x04, 0x02, // PART 2 TLV
            0x65, 0x73, 0x00, 0x00,
        ];

        assert_eq!(name, decode::<String>(primitive).unwrap());
        assert_eq!(name, decode::<String>(definite_constructed).unwrap());
        assert_eq!(name, decode::<String>(indefinite_constructed).unwrap());
    }

    #[test]
    fn sequence_of() {
        let vec = alloc::vec!["Jon", "es"];
        let from_raw: Vec<String> = decode(
            &[
                0x30, 0x9, 0x0C, 0x03, 0x4A, 0x6F, 0x6E, 0x0C, 0x02, 0x65, 0x73,
            ][..],
        )
        .unwrap();

        assert_eq!(vec, from_raw);
    }

    #[test]
    fn sequence() {
        use types::IA5String;
        // Taken from examples in 8.9 of X.690.
        #[derive(Debug, PartialEq)]
        struct Foo {
            name: IA5String,
            ok: bool,
        }

        impl Decode for Foo {
            const TAG: Tag = Tag::SEQUENCE;

            fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                let mut field_decoder = decoder.decode_sequence(tag)?;

                let name: IA5String = IA5String::decode(&mut field_decoder)?;
                let ok: bool = bool::decode(&mut field_decoder)?;

                Ok(Self { name, ok })
            }
        }

        let foo = Foo {
            name: String::from("Smith").into(),
            ok: true,
        };
        let bytes = &[
            0x30, 0x0A, // TAG + LENGTH
                0x16, 0x05, 0x53, 0x6d, 0x69, 0x74, 0x68, // IA5String "Smith"
                0x01, 0x01, 0xff, // BOOL True
        ];

        assert_eq!(foo, decode(bytes).unwrap());
    }
}

