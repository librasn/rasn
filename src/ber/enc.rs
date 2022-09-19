//! # Encoding BER.

mod config;
mod error;

use alloc::{collections::VecDeque, string::ToString, vec::Vec};

use super::Identifier;
use crate::{
    types::{
        self, constraints,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
        Constraints, Tag,
    },
    Encode,
};

pub use config::EncoderOptions;
pub use error::Error;

const START_OF_CONTENTS: u8 = 0x80;
const END_OF_CONTENTS: &[u8] = &[0, 0];

/// A BER and variants encoder. Capable of encoding to BER, CER, and DER.
pub struct Encoder {
    output: Vec<u8>,
    config: EncoderOptions,
    is_set_encoding: bool,
    set_buffer: alloc::collections::BTreeMap<Tag, Vec<u8>>,
}

/// A convenience type around results needing to return one or many bytes.
enum ByteOrBytes {
    Single(u8),
    Many(Vec<u8>),
}

impl Encoder {
    /// Creates a new instance from the given `config`.
    pub fn new(config: EncoderOptions) -> Self {
        Self {
            config,
            is_set_encoding: false,
            output: <_>::default(),
            set_buffer: <_>::default(),
        }
    }

    /// Creates a new instance from the given `config`, and uses SET encoding
    /// logic, ensuring that all messages are encoded in order by tag.
    pub fn new_set(config: EncoderOptions) -> Self {
        Self {
            config,
            is_set_encoding: true,
            output: <_>::default(),
            set_buffer: <_>::default(),
        }
    }

    /// Creates a new instance from the given `config` and a user-supplied
    /// `Vec<u8>` buffer. This allows reuse of an existing buffer instead of
    /// allocating a new encoding buffer each time an [`Encoder`] is created.
    /// The buffer will be cleared before use.
    pub fn new_with_buffer(config: EncoderOptions, mut buffer: Vec<u8>) -> Self {
        buffer.clear();
        Self { output: buffer, config, is_set_encoding: false, set_buffer: <_>::default() }
    }

    /// Consumes the encoder and returns the output of the encoding.
    pub fn output(self) -> Vec<u8> {
        if self.is_set_encoding {
            self.set_buffer
                .into_values()
                .fold(Vec::new(), |mut acc, mut field| {
                    acc.append(&mut field);
                    acc
                })
        } else {
            self.output
        }
    }

    fn append_byte_or_bytes(&mut self, bytes: ByteOrBytes) {
        match bytes {
            ByteOrBytes::Single(b) => self.output.push(b),
            ByteOrBytes::Many(mut bs) => self.output.append(&mut bs),
        }
    }

    pub(super) fn encode_as_base128(&self, number: u32, buffer: &mut Vec<u8>) {
        const WIDTH: u8 = 7;
        const SEVEN_BITS: u8 = 0x7F;
        const EIGHTH_BIT: u8 = 0x80;

        if number < EIGHTH_BIT as u32 {
            buffer.push(number as u8);
        } else {
            let mut n: u8;
            let mut bits_left = 35;
            let mut cont = false;
            while bits_left > 0 {
                bits_left -= WIDTH;
                n = ((number >> bits_left) as u8) & SEVEN_BITS;
                if n > 0 || cont {
                    buffer.push(if bits_left > 0 { EIGHTH_BIT } else { 0 } | (n & SEVEN_BITS));
                    cont = true;
                }
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
            self.encode_as_base128(tag_number, &mut buffer);
            ByteOrBytes::Many(buffer)
        } else {
            tag_byte |= tag_number as u8;
            ByteOrBytes::Single(tag_byte)
        }
    }

    fn encode_length(&mut self, identifier: Identifier, value: &[u8]) {
        if identifier.is_primitive() || !self.config.encoding_rules.is_cer() {
            let len_bytes = self.encode_definite_length(value.len());
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

    fn encode_octet_string_(&mut self, tag: Tag, value: &[u8]) -> Result<(), Error> {
        self.encode_string(tag, Tag::OCTET_STRING, value)
    }

    /// "STRING" types in ASN.1 BER (OCTET STRING, UTF8 STRING) are either
    /// primitive encoded, or in certain variants like CER they are constructed
    /// encoded containing primitive encoded chunks.
    fn encode_string(&mut self, tag: Tag, nested_tag: Tag, value: &[u8]) -> Result<(), Error> {
        let max_string_length = self.config.encoding_rules.max_string_length();

        if value.len() > max_string_length {
            let ident_bytes = self.encode_identifier(Identifier::from_tag(tag, true));
            self.append_byte_or_bytes(ident_bytes);

            self.output.push(START_OF_CONTENTS);

            for chunk in value.chunks(max_string_length) {
                self.encode_primitive(nested_tag, chunk);
            }

            self.output.extend_from_slice(END_OF_CONTENTS);
            self.encode_to_set(tag);
        } else {
            self.encode_primitive(tag, value);
        }

        Ok(())
    }

    fn encode_primitive(&mut self, tag: Tag, value: &[u8]) {
        self.encode_value(Identifier::from_tag(tag, false), value);
    }

    fn encode_constructed(&mut self, tag: Tag, value: &[u8]) {
        self.encode_value(Identifier::from_tag(tag, true), value);
    }

    /// Encodes a given ASN.1 BER value with the `identifier`.
    fn encode_value(&mut self, identifier: Identifier, value: &[u8]) {
        let ident_bytes = self.encode_identifier(identifier);
        self.append_byte_or_bytes(ident_bytes);
        self.encode_length(identifier, value);
        self.encode_to_set(identifier.tag);
    }

    /// Runs at the end of a complete value encoding to decide whether to sort
    /// the output by the tag of each value.
    fn encode_to_set(&mut self, tag: Tag) {
        if self.is_set_encoding {
            self.set_buffer
                .insert(tag, core::mem::take(&mut self.output));
        }
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = error::Error;

    fn encode_any(&mut self, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        if self.is_set_encoding {
            return Err(crate::enc::Error::custom(
                "Cannot encode `ANY` types in `SET` fields.",
            ));
        }

        self.output.extend_from_slice(&value.contents);

        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BitString,
    ) -> Result<Self::Ok, Self::Error> {
        if value.not_any() {
            self.encode_primitive(tag, &[]);
            Ok(())
        } else {
            let bit_length = value.len();
            let bytes = value.clone().into_vec();
            let mut deque = VecDeque::from(bytes);
            while deque.back().map_or(false, |i| *i == 0) {
                deque.pop_back();
            }

            deque.push_front((deque.len() * 8).saturating_sub(bit_length) as u8);
            self.encode_string(tag, Tag::BIT_STRING, &Vec::from(deque))
        }
    }

    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, &[if value { 0xff } else { 0x00 }]);
        Ok(())
    }

    fn encode_enumerated(
        &mut self,
        tag: Tag,
        variance: usize,
        value: isize,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_integer(
            tag,
            Constraints::from(&[constraints::Size::new(constraints::Range::up_to(variance)).into()]),
            &(value.into()),
        )
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        _constraints: Constraints,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, &value.to_signed_bytes_be());
        Ok(())
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(tag, &[]);
        Ok(())
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        if oid.len() < 2 {
            return Err(error::Error::InvalidObjectIdentifier);
        }
        let mut bytes = Vec::new();

        let first = oid[0];
        let second = oid[1];

        if first > MAX_OID_FIRST_OCTET {
            return Err(error::Error::InvalidObjectIdentifier);
        }

        self.encode_as_base128((first * (MAX_OID_SECOND_OCTET + 1)) + second, &mut bytes);

        for component in oid.iter().skip(2) {
            self.encode_as_base128(*component, &mut bytes);
        }

        self.encode_primitive(tag, &bytes);

        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string_(tag, value)
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string_(tag, &value.to_iso646_bytes())
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string_(tag, value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(
            tag,
            value
                .naive_utc()
                .format("%y%m%d%H%M%SZ")
                .to_string()
                .as_bytes(),
        );

        Ok(())
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_primitive(
            tag,
            value
                .naive_utc()
                .format("%Y%m%d%H%M%SZ")
                .to_string()
                .as_bytes(),
        );

        Ok(())
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag(self, tag)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        let mut sequence_encoder = Self::new(self.config);

        for value in values {
            value.encode(&mut sequence_encoder)?;
        }

        self.encode_constructed(tag, &sequence_encoder.output);

        Ok(())
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        _constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        let mut sequence_encoder = Self::new(self.config);

        for value in values {
            value.encode(&mut sequence_encoder)?;
        }

        self.encode_constructed(tag, &sequence_encoder.output);

        Ok(())
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        let mut encoder = Self::new(self.config);
        value.encode(&mut encoder)?;
        self.encode_constructed(tag, &encoder.output);
        Ok(())
    }

    fn encode_sequence<C, F>(
        &mut self,
        tag: Tag,
        _constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut encoder = Self::new(self.config);

        (encoder_scope)(&mut encoder)?;

        self.encode_constructed(tag, &encoder.output);

        Ok(())
    }

    fn encode_set<C, F>(
        &mut self,
        tag: Tag,
        _constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut encoder = Self::new_set(self.config);

        (encoder_scope)(&mut encoder)?;

        self.encode_constructed(tag, &encoder.output());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;
    use alloc::vec;

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct C0;
    impl crate::AsnType for C0 {
        const TAG: Tag = Tag::new(crate::types::Class::Context, 0);
    }

    #[test]
    fn bit_string() {
        let bitstring =
            types::BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

        let primitive_encoded = &[0x03, 0x07, 0x00, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];

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
                Tag::new(crate::types::Class::Private, 127),
                true,
            ))
        );
    }

    #[test]
    fn encoding_oid() {
        fn oid_to_bytes(oid: &[u32]) -> Vec<u8> {
            use crate::Encoder;
            let mut enc = self::Encoder::new(EncoderOptions::ber());
            enc.encode_object_identifier(Tag::OBJECT_IDENTIFIER, oid)
                .unwrap();
            enc.output
        }

        // example from https://stackoverflow.com/questions/5929050/how-does-asn-1-encode-an-object-identifier
        assert_eq!(
            &vec![0x06, 0x08, 0x2b, 0x06, 0x01, 0x05, 0x05, 0x07, 0x30, 0x01],
            &oid_to_bytes(&[1, 3, 6, 1, 5, 5, 7, 48, 1])
        );

        // example from https://docs.microsoft.com/en-us/windows/win32/seccertenroll/about-object-identifier
        assert_eq!(
            &vec![0x06, 0x09, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x15, 0x14],
            &oid_to_bytes(&[1, 3, 6, 1, 4, 1, 311, 21, 20])
        );

        // commonName (X.520 DN component)
        assert_eq!(
            &vec![0x06, 0x03, 0x55, 0x04, 0x03],
            &oid_to_bytes(&[2, 5, 4, 3])
        );

        // example oid
        assert_eq!(
            &vec![0x06, 0x03, 0x88, 0x37, 0x01],
            &oid_to_bytes(&[2, 999, 1])
        );
    }

    #[test]
    fn base128_test() {
        fn encode(n: u32) -> Vec<u8> {
            let enc = self::Encoder::new(EncoderOptions::ber());
            let mut buffer: Vec<u8> = vec![];
            enc.encode_as_base128(n, &mut buffer);
            buffer
        }

        assert_eq!(&vec![0x0], &encode(0x0));
        assert_eq!(&vec![0x7F], &encode(0x7F));
        assert_eq!(&vec![0x81, 0x00], &encode(0x80));
        assert_eq!(&vec![0xC0, 0x00], &encode(0x2000));
        assert_eq!(&vec![0xFF, 0x7F], &encode(0x3FFF));
        assert_eq!(&vec![0x81, 0x80, 0x00], &encode(0x4000));
        assert_eq!(&vec![0xFF, 0xFF, 0x7F], &encode(0x001FFFFF));
        assert_eq!(&vec![0x81, 0x80, 0x80, 0x00], &encode(0x00200000));
        assert_eq!(&vec![0xC0, 0x80, 0x80, 0x00], &encode(0x08000000));
        assert_eq!(&vec![0xFF, 0xFF, 0xFF, 0x7F], &encode(0x0FFFFFFF));
    }

    #[test]
    fn any() {
        let bitstring =
            types::BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

        let primitive_encoded = &[0x03, 0x07, 0x00, 0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..];
        let any = types::Any {
            contents: primitive_encoded.into(),
        };

        assert_eq!(primitive_encoded, super::super::encode(&bitstring).unwrap());
        assert_eq!(
            super::super::encode(&bitstring).unwrap(),
            super::super::encode(&any).unwrap()
        );
    }

    #[test]
    fn set() {
        use crate::{
            types::{AsnType, Implicit},
            Encoder as _,
        };

        struct C0;
        struct C1;
        struct C2;

        impl AsnType for C0 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 0);
        }

        impl AsnType for C1 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 1);
        }

        impl AsnType for C2 {
            const TAG: Tag = Tag::new(crate::types::Class::Context, 2);
        }

        type Field1 = Implicit<C0, u32>;
        type Field2 = Implicit<C1, u32>;
        type Field3 = Implicit<C2, u32>;

        let field1: Field1 = 1.into();
        let field2: Field2 = 2.into();
        let field3: Field3 = 3.into();

        struct Set;

        impl crate::types::Constructed for Set {
            const FIELDS: &'static [crate::types::Field] = &[
                crate::types::Field::new_required(C0::TAG),
                crate::types::Field::new_required(C1::TAG),
                crate::types::Field::new_required(C2::TAG),
            ];
        }

        let output = {
            let mut encoder = Encoder::new_set(EncoderOptions::ber());
            encoder
                .encode_set::<Set, _>(Tag::SET, <_>::default(), |encoder| {
                    field3.encode(encoder)?;
                    field2.encode(encoder)?;
                    field1.encode(encoder)?;
                    Ok(())
                })
                .unwrap();

            encoder.output()
        };

        assert_eq!(
            vec![0x31, 0x9, 0x80, 0x1, 0x1, 0x81, 0x1, 0x2, 0x82, 0x1, 0x3],
            output,
        );
    }
}
