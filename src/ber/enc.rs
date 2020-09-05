mod config;
mod error;

use alloc::{borrow::ToOwned, collections::VecDeque, vec::Vec};

use super::Identifier;
use crate::{tag::Tag, types, Encode};

pub use config::EncoderOptions;
pub use error::Error;

const START_OF_CONTENTS: u8 = 0x80;
const END_OF_CONTENTS: &[u8] = &[0, 0];

pub struct Encoder {
    pub(crate) output: Vec<u8>,
    config: EncoderOptions,
}

impl Encoder {
    pub fn new(config: EncoderOptions) -> Self {
        Self {
            output: Vec::new(),
            config,
        }
    }
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

    pub(super) fn encode_seven_bit_integer(&self, mut number: u32, buffer: &mut Vec<u8>) {
        const WIDTH: u8 = 7;
        const SEVEN_BITS: u8 = (1 << 7) - 1;
        const EIGHTH_BIT: u8 = 0x80;

        if number == 0 {
            buffer.push(0);
        } else {
            while number > 0 {
                let seven_bits = number as u8 & SEVEN_BITS;
                let encoded = seven_bits | EIGHTH_BIT;
                buffer.push(encoded);
                number >>= WIDTH;
            }

            if let Some(last) = buffer.last_mut() {
                *last &= 0x7f;
            }
        }
    }

    /// Encodes the identifier of a type in BER/CER/DER. An identifier consists
    /// of a "class", encoding bit, and tag number. If our tag number is
    /// greater than 30 we to encode the number as stream of a 7 bit integers
    /// in big endian delimited by the leading bit of each byte.
    ///
    /// ```text
    /// ---------------------------------
    /// | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    /// ---------------------------------
    /// | class | E |        Tag        |
    /// ---------------------------------
    /// ```
    fn encode_identifier(
        &mut self,
        Identifier {
            tag,
            is_constructed,
        }: Identifier,
    ) -> ByteOrBytes {
        const FIVE_BITS: u32 = (1 << 5) - 1;
        let mut tag_byte = tag.class as u8;
        let tag_number = tag.value;

        // Constructed is a single bit.
        tag_byte <<= 1;
        tag_byte |= match tag {
            Tag::EXTERNAL | Tag::SEQUENCE | Tag::SET => 1,
            _ if is_constructed => 1,
            _ => 0,
        };

        tag_byte <<= 5;

        if tag_number >= FIVE_BITS {
            let mut buffer = alloc::vec![tag_byte | FIVE_BITS as u8];
            self.encode_seven_bit_integer(tag_number, &mut buffer);
            ByteOrBytes::Many(buffer)
        } else {
            tag_byte |= tag_number as u8;
            ByteOrBytes::Single(tag_byte)
        }
    }

    fn encode_length(&mut self, identifier: Identifier, len: usize, value: &[u8]) {
        if identifier.is_primitive() || !self.config.encoding_rules.is_cer() {
            let len_bytes = self.encode_definite_length(len);
            self.append_byte_or_bytes(len_bytes);
            self.output.extend_from_slice(value);
        } else {
            self.output.push(START_OF_CONTENTS);
            self.output.extend_from_slice(value);
            self.output.extend_from_slice(END_OF_CONTENTS);
        }
    }

    fn encode_definite_length(&mut self, len: usize) -> ByteOrBytes {
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

    fn encode_string(&mut self, tag: Tag, value: &[u8]) -> Result<(), Error> {
        let max_string_length = self.config.encoding_rules.max_string_length();

        if value.len() > max_string_length {
            let ident_bytes = self.encode_identifier(Identifier::from_tag(tag, true));
            self.append_byte_or_bytes(ident_bytes);

            self.output.push(START_OF_CONTENTS);

            for chunk in value.chunks(max_string_length) {
                self.encode_value(tag, chunk);
            }

            self.output.extend_from_slice(END_OF_CONTENTS);
        } else {
            self.encode_value(tag, value);
        }

        Ok(())
    }

    fn encode_value(&mut self, tag: Tag, value: &[u8]) {
        let ident_bytes = self.encode_identifier(Identifier::from_tag(tag, false));
        self.append_byte_or_bytes(ident_bytes);
        self.encode_length(Identifier::from_tag(tag, false), value.len(), value);
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = error::Error;

    fn encode_any(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, value))
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        value: &types::BitString,
    ) -> Result<Self::Ok, Self::Error> {
        if value.not_any() {
            Ok(self.encode_value(tag, &[]))
        } else {
            let bytes = value.as_slice().to_owned();
            let mut deque = VecDeque::from(bytes);
            while deque.back().map_or(false, |i| *i == 0) {
                deque.pop_back();
            }
            deque.push_front(deque.back().map(|i| i.trailing_zeros() as u8).unwrap_or(0));
            Ok(self.encode_value(tag, &Vec::from(deque)))
        }
    }
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &[if value { 0xff } else { 0x00 }]))
    }

    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error> {
        self.encode_integer(tag, &(value.into()))
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        value: &types::Integer,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &value.to_signed_bytes_be()))
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, &[]))
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        if oid.len() < 2 {
            return Err(error::Error::InvalidObjectIdentifier);
        }
        let mut bytes = Vec::new();

        let first = oid[0];
        let second = oid[1];

        if first > 1 {
            return Err(error::Error::InvalidObjectIdentifier);
        }

        self.encode_seven_bit_integer((first * 40) + second, &mut bytes);

        for component in oid.iter().skip(2) {
            self.encode_seven_bit_integer(*component, &mut bytes);
        }

        Ok(self.encode_value(tag, &bytes))
    }

    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.encode_string(tag, value)
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        self.encode_string(tag, value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, value.to_rfc2822().as_bytes()))
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(self.encode_value(tag, value.to_rfc3339().as_bytes()))
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
    ) -> Result<Self::Ok, Self::Error> {
        let mut sequence_encoder = Self::new(self.config);

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
        self.encode_sequence(tag, |sequence| {
            value.encode(sequence)?;
            Ok(())
        })
    }

    fn encode_sequence<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut encoder = Self::new(self.config);

        (encoder_scope)(&mut encoder)?;

        Ok(self.encode_value(tag, &encoder.output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_string() {
        let bitstring =
            types::BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

        let primitive_encoded = &[0x03, 0x07, 0x04, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];

        assert_eq!(primitive_encoded, super::super::encode(&bitstring).unwrap());
    }

    #[test]
    fn identifier() {
        fn ident_to_bytes(ident: Identifier) -> Vec<u8> {
            let mut enc = Encoder::new(EncoderOptions::ber());
            let bytes = enc.encode_identifier(ident);
            enc.append_byte_or_bytes(bytes);
            enc.output
        }

        assert_eq!(
            &[0xFF, 0x7F,][..],
            ident_to_bytes(Identifier::from_tag(
                Tag::new(crate::tag::Class::Private, 127),
                true,
            ))
        );
        assert_eq!(
            &[0b1101_1111, 0xFF, 0xFF, 0x3][..],
            ident_to_bytes(Identifier::from_tag(
                Tag::new(crate::tag::Class::Private, 65535),
                false,
            ))
        );
    }

    #[test]
    fn explicit_empty_tag() {
        use crate::{tag::Class, types::Explicit, AsnType, Tag};

        struct C0;
        impl AsnType for C0 {
            const TAG: Tag = Tag::new(Class::Context, 0);
        }

        assert_eq!(
            &[0x80, 0],
            &*crate::ber::encode(&<Explicit<C0, _>>::new(None::<()>)).unwrap()
        );
    }
}
