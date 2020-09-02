mod error;

use alloc::{borrow::ToOwned, collections::VecDeque, vec::Vec};

use crate::{tag::Tag, types, Encode};

pub use error::Error;

#[derive(Default)]
pub(crate) struct Encoder {
    pub(crate) output: Vec<u8>,
}

enum ByteOrBytes {
    Single(u8),
    Many(Vec<u8>),
}

impl Encoder {
    fn append_byte_or_bytes(&mut self, bytes: ByteOrBytes) {
        match bytes {
            ByteOrBytes::Single(b) => self.output.push(b),
            ByteOrBytes::Many(mut bs) => self.output.append(&mut bs),
        }
    }

    fn encode_identifier(&mut self, tag: Tag) -> ByteOrBytes {
        let mut tag_byte = tag.class as u8;
        let mut tag_number = tag.value;

        // Constructed is a single bit.
        tag_byte <<= 1;
        tag_byte |= match tag {
            Tag::EXTERNAL | Tag::SEQUENCE | Tag::SET => 1,
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

    fn encode_length(&mut self, len: usize) -> ByteOrBytes {
        if len <= 127 {
            ByteOrBytes::Single(len as u8)
        } else {
            let mut length = len;
            let mut length_buffer = VecDeque::new();

            while length != 0 {
                length_buffer.push_front((length & 0xff) as u8);
                length >>= 8;
            }

            length_buffer.push_front(length_buffer.len() as u8 | 0x80);

            ByteOrBytes::Many(length_buffer.into())
        }
    }

    fn encode_value(&mut self, tag: Tag, value: &[u8]) {
        let ident_bytes = self.encode_identifier(tag);
        let len_bytes = self.encode_length(value.len());
        self.append_byte_or_bytes(ident_bytes);
        self.append_byte_or_bytes(len_bytes);
        self.output.extend_from_slice(value);
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = error::Error;

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        value: &types::BitSlice,
    ) -> Result<Self::Ok, Self::Error> {
        let mut deque = VecDeque::from(value.as_slice().to_owned());
        deque.push_front(deque.back().map(|i| i.trailing_zeros() as u8).unwrap_or(0));
        Ok(self.encode_value(tag, &Vec::from(deque)))
    }
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &[if value { 0xff } else { 0x00 }]))
    }

    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error> {
        self.encode_integer(tag, value.into())
    }

    fn encode_integer(&mut self, tag: Tag, value: types::Integer) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &value.to_signed_bytes_be()))
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &[]))
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        if oid.len() < 2 {
            return Err(error::Error::InvalidObjectIdentifier);
        }

        fn encode_component(mut v: u32, writer: &mut Vec<u8>) {
            let mut bytes: Vec<u8> = Vec::new();

            while v != 0 {
                bytes.push((v & 0x7f) as u8);
                v >>= 7;
            }

            for byte in bytes.iter().skip(1).rev() {
                let octet = (0x80 | byte) as u8;
                writer.push(octet);
            }

            let final_octet = bytes[0] as u8;
            writer.push(final_octet);
        }

        let mut bytes = Vec::new();

        let first = oid[0];
        let second = oid[1];

        encode_component((first * 40) + second, &mut bytes);

        for component in oid.iter().skip(2) {
            encode_component(*component, &mut bytes);
        }

        Ok(self.encode_value(tag, &bytes))
    }

    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, value))
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, value.as_bytes()))
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
    ) -> Result<Self::Ok, Self::Error> {
        let mut sequence_encoder = Self::default();

        for value in values {
            value.encode(&mut sequence_encoder)?;
        }

        Ok(self.encode_value(tag, &sequence_encoder.output))
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        let bytes = {
            let mut encoder = Self::default();
            value.encode(&mut encoder)?;
            encoder.output
        };

        Ok(self.encode_value(tag, &bytes))
    }

    fn encode_sequence<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut encoder = Self::default();

        (encoder_scope)(&mut encoder)?;

        Ok(self.encode_value(tag, &encoder.output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_string() {
        let bitstring = types::BitString::from_slice(&[0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0]);

        let primitive_encoded = &[0x03, 0x07, 0x04, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];

        assert_eq!(primitive_encoded, super::super::encode(&bitstring).unwrap());
    }
}
