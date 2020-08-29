mod error;

use alloc::{vec::Vec, collections::VecDeque};

use crate::{tag::Tag, types,};

type Result<T, E = error::Error> = core::result::Result<T, E>;

#[derive(Default)]
pub(crate) struct Encoder {
    output: VecDeque<u8>,
}

enum ByteOrBytes {
    Single(u8),
    Many(Vec<u8>)
}

impl Encoder {
    fn encode_identifier(&mut self, tag: Tag) -> ByteOrBytes {
        let mut tag_byte = tag.class as u8;
        let mut tag_number = tag.value;

        // Constructed is a single bit.
        tag_byte <<= 1;
        tag_byte |= match tag {
            Tag::EXTERNAL |
            Tag::SEQUENCE |
            Tag::SET => 1,
            _ => 0,
        };

        // Identifier number is five bits
        tag_byte <<= 5;

        if tag_number >= 0x1f {
            let mut buffer = Vec::with_capacity(1);
            tag_byte |= 0x1f;
            buffer.push(tag_byte);

            while tag_number != 0 {
                let mut encoded_number: u8 = (tag_number & 0x7f) as u8;
                tag_number >>= 7;

                // Fill the last bit unless we're at the last bit.
                if tag_number != 0 {
                    encoded_number |= 0x80;
                }

                buffer.push(encoded_number);
            }

            ByteOrBytes::Many(buffer)
        } else {
            tag_byte |= tag_number as u8;
            ByteOrBytes::Single(tag_byte)
        }
    }

    fn encode_length(&mut self, len: usize) -> Option<ByteOrBytes> {
        if len <= 127 {
            Some(ByteOrBytes::Single(len as u8))
        } else {
            let mut length = len;
            let mut length_buffer = VecDeque::new();

            while length != 0 {
                length_buffer.push_front((length & 0xff) as u8);
                length >>= 8;
            }

            let length_of_length = length_buffer.len();

            if length_of_length >= 0xff {
                None
            } else {
                length_buffer.push_front(length_of_length as u8 | 0x80);
                Some(ByteOrBytes::Many(length_buffer.into()))
            }
        }
    }

    fn encode_value(&mut self, tag: Tag, value: &[u8]) {
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = error::Error;

    fn encode_bit_string(&mut self, value: types::BitString) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_bool(&mut self, value: bool) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_integer(&mut self, value: types::Integer) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_null(&mut self, value: ()) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_object_identifier( &mut self, value: types::ObjectIdentifier,) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_octet_string(&mut self, value: types::OctetString) -> Result<Self::Ok, Self::Error> { todo!() }
    fn encode_utf8_string(&mut self, value: types::Utf8String) -> Result<Self::Ok, Self::Error> { todo!() }
}

