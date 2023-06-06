//! # Decoding BER

mod config;
mod error;
pub(super) mod parser;

use alloc::{borrow::ToOwned, vec::Vec};

use snafu::*;

use super::identifier::Identifier;
use crate::{
    de::Error as _,
    types::{
        self,
        oid::{MAX_OID_FIRST_OCTET, MAX_OID_SECOND_OCTET},
        Tag,
    },
    Decode,
};

pub use self::{config::DecoderOptions, error::Error};

type Result<T, E = Error> = core::result::Result<T, E>;

const EOC: &[u8] = &[0, 0];

/// A BER and variants decoder. Capable of decoding BER, CER, and DER.
pub struct Decoder<'input> {
    input: &'input [u8],
    config: DecoderOptions,
    initial_len: usize,
}

impl<'input> Decoder<'input> {
    /// Create a new [`Decoder`] from the given `input` and `config`.
    pub fn new(input: &'input [u8], config: DecoderOptions) -> Self {
        Self {
            input,
            config,
            initial_len: input.len(),
        }
    }

    /// Return a number of the decoded bytes by this decoder
    pub fn decoded_len(&self) -> usize {
        self.initial_len - self.input.len()
    }

    fn parse_eoc(&mut self) -> Result<()> {
        let (i, _) = nom::bytes::streaming::tag(EOC)(self.input).map_err(error::map_nom_err)?;
        self.input = i;
        Ok(())
    }

    pub(crate) fn parse_value(&mut self, tag: Tag) -> Result<(Identifier, Option<&'input [u8]>)> {
        let (input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, Some(tag))?;
        self.input = input;
        Ok((identifier, contents))
    }

    pub(crate) fn parse_primitive_value(&mut self, tag: Tag) -> Result<(Identifier, &'input [u8])> {
        let (input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, Some(tag))?;
        self.input = input;
        match contents {
            Some(contents) => Ok((identifier, contents)),
            None => error::IndefiniteLengthNotAllowedSnafu.fail(),
        }
    }

    /// Parses a constructed ASN.1 value, checking the `tag`, and optionally
    /// checking if the identifier is marked as encoded. This should be true
    /// in all cases except explicit prefixes.
    fn parse_constructed_contents<D, F>(
        &mut self,
        tag: Tag,
        check_identifier: bool,
        decode_fn: F,
    ) -> Result<D>
    where
        F: FnOnce(&mut Self) -> Result<D>,
    {
        let (identifier, contents) = self.parse_value(tag)?;

        error::assert_tag(tag, identifier.tag)?;

        if check_identifier && identifier.is_primitive() {
            return Err(Error::custom("Invalid constructed identifier"));
        }

        let (streaming, contents) = match contents {
            Some(contents) => (false, contents),
            None => (true, self.input),
        };

        let mut inner = Self::new(contents, self.config);

        let result = (decode_fn)(&mut inner)?;

        if streaming {
            self.input = inner.input;
            self.parse_eoc()?;
        } else if !inner.input.is_empty() {
            return Err(Error::UnexpectedExtraData {
                length: inner.input.len(),
            });
        }

        Ok(result)
    }
}

impl<'input> crate::Decoder for Decoder<'input> {
    type Error = Error;

    fn decode_any(&mut self) -> Result<types::Any> {
        let (mut input, (identifier, contents)) =
            self::parser::parse_value(&self.config, self.input, None)?;

        if contents.is_none() {
            let (i, _) = self::parser::parse_encoded_value(
                &self.config,
                self.input,
                identifier.tag,
                |input| Ok(alloc::vec::Vec::from(input)),
            )?;
            input = i;
        }
        let diff = self.input.len() - input.len();
        let contents = &self.input[..diff];
        self.input = input;

        Ok(types::Any {
            contents: contents.to_vec(),
        })
    }

    fn decode_bool(&mut self, tag: Tag) -> Result<bool> {
        let (_, contents) = self.parse_primitive_value(tag)?;
        error::assert_length(1, contents.len())?;
        Ok(match contents[0] {
            0 => false,
            0xFF => true,
            _ if self.config.encoding_rules.is_ber() => true,
            _ => return Err(error::Error::InvalidBool),
        })
    }

    fn decode_enumerated(&mut self, tag: Tag) -> Result<types::Integer> {
        self.decode_integer(tag)
    }

    fn decode_integer(&mut self, tag: Tag) -> Result<types::Integer> {
        Ok(types::Integer::from_signed_bytes_be(
            self.parse_primitive_value(tag)?.1,
        ))
    }

    fn decode_octet_string(&mut self, tag: Tag) -> Result<Vec<u8>> {
        let (identifier, contents) = self.parse_value(tag)?;

        if identifier.is_primitive() {
            match contents {
                Some(c) => Ok(c.to_vec()),
                None => error::IndefiniteLengthNotAllowedSnafu.fail(),
            }
        } else if identifier.is_constructed() && self.config.encoding_rules.is_der() {
            error::ConstructedEncodingNotAllowedSnafu.fail()
        } else {
            let mut buffer = Vec::new();

            match contents {
                Some(mut contents) => {
                    while !contents.is_empty() {
                        let (c, mut vec) = self::parser::parse_encoded_value(
                            &self.config,
                            contents,
                            Tag::OCTET_STRING,
                            |input| Ok(alloc::vec::Vec::from(input)),
                        )?;
                        contents = c;

                        buffer.append(&mut vec);
                    }
                }
                None => {
                    while !self.input.starts_with(EOC) {
                        let (c, mut vec) = self::parser::parse_encoded_value(
                            &self.config,
                            self.input,
                            Tag::OCTET_STRING,
                            |input| Ok(alloc::vec::Vec::from(input)),
                        )?;
                        self.input = c;

                        buffer.append(&mut vec);
                    }

                    self.parse_eoc()?;
                }
            }

            Ok(buffer)
        }
    }

    fn decode_null(&mut self, tag: Tag) -> Result<()> {
        let (_, contents) = self.parse_primitive_value(tag)?;
        error::assert_length(0, contents.len())?;
        Ok(())
    }

    fn decode_object_identifier(&mut self, tag: Tag) -> Result<crate::types::ObjectIdentifier> {
        use num_traits::ToPrimitive;
        let contents = self.parse_primitive_value(tag)?.1;
        let (mut contents, root_octets) =
            parser::parse_base128_number(contents).map_err(error::map_nom_err)?;
        let the_number = root_octets
            .to_u32()
            .context(error::IntegerOverflowSnafu { max_width: 32u32 })?;
        let first: u32;
        let second: u32;
        const MAX_OID_THRESHOLD: u32 = MAX_OID_SECOND_OCTET + 1;
        if the_number > MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD + MAX_OID_SECOND_OCTET {
            first = MAX_OID_FIRST_OCTET;
            second = the_number - MAX_OID_FIRST_OCTET * MAX_OID_THRESHOLD;
        } else {
            second = the_number % MAX_OID_THRESHOLD;
            first = (the_number - second) / MAX_OID_THRESHOLD;
        }
        let mut buffer = alloc::vec![first, second];

        while !contents.is_empty() {
            let (c, number) = parser::parse_base128_number(contents).map_err(error::map_nom_err)?;
            contents = c;
            buffer.push(
                number
                    .to_u32()
                    .context(error::IntegerOverflowSnafu { max_width: 32u32 })?,
            );
        }

        crate::types::ObjectIdentifier::new(buffer).context(error::InvalidObjectIdentifierSnafu)
    }

    fn decode_bit_string(&mut self, tag: Tag) -> Result<types::BitString> {
        let (input, bs) =
            self::parser::parse_encoded_value(&self.config, self.input, tag, |input| {
                let unused_bits = if let Some(bits) = input.get(0).copied() {
                    bits
                } else {
                    return Ok(types::BitString::new());
                };

                match unused_bits {
                    // TODO: https://github.com/myrrlyn/bitvec/issues/72
                    bits @ 0..=7 => {
                        let mut buffer = input[1..].to_owned();
                        if let Some(last) = buffer.last_mut() {
                            *last &= !((1 << bits) - 1);
                        }
                        if buffer.last().map_or(false, |i| *i == 0) {
                            buffer.pop();
                        }

                        let string = types::BitString::from_vec(buffer);

                        if string.not_any() {
                            Ok(types::BitString::new())
                        } else {
                            Ok(string)
                        }
                    }
                    _ => Err(Error::InvalidBitString { bits: unused_bits }),
                }
            })?;

        self.input = input;
        if let Some((i, _)) = bs
            .as_raw_slice()
            .iter()
            .enumerate()
            .rev()
            .find(|(_, v)| **v != 0)
        {
            Ok(types::BitString::from_vec(bs.as_raw_slice()[..=i].to_vec()))
        } else {
            Ok(bs)
        }
    }

    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String> {
        let vec = self.decode_octet_string(tag)?;
        types::Utf8String::from_utf8(vec)
            .ok()
            .context(error::InvalidUtf8Snafu)
    }

    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime> {
        let string = self.decode_utf8_string(tag)?;
        // Reference https://obj-sys.com/asn1tutorial/node14.html
        // If data contains ., 3 decimal places of seconds are expected
        // If data contains explict Z, result is UTC
        // If data contains + or -, explicit timezone is given
        // If neither Z nor + nor -, purely local time is implied
        // FIXME for future, NaiveDatetime is right in last case. Others want DateTime<UTC>
        // or DateTime<FixedOffset> respectively
        // FIXME, supposedly, minutes and seconds are optional, and would have to be handled
        // in the no decimal point cases
        let format = if string.contains('Z') {
            if string.contains('.') {
                "%Y%m%d%H%M%S%.3fZ"
            } else {
                "%Y%m%d%H%M%SZ"
            }
        } else if string.contains('+') || string.contains('-') {
            if string.contains('.') {
                "%Y%m%d%H%M%S%.3f%z"
            } else {
                "%Y%m%d%H%M%S%z"
            }
        } else {
            if string.contains('.') {
                "%Y%m%d%H%M%S%.3fZ"
            } else {
                "%Y%m%d%H%M%SZ"
            }
        };
        chrono::NaiveDateTime::parse_from_str(&string, format)
            .ok()
            .context(error::InvalidDateSnafu)
            .map(|date| types::GeneralizedTime::from_utc(date, chrono::FixedOffset::east_opt(0).unwrap()))
    }

    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime> {
        // Reference https://obj-sys.com/asn1tutorial/node15.html
        // FIXME - handle optional seconds
        // FIXME - should this be DateTime<UTC> rather than NaiveDateTime ?
        let string = self.decode_utf8_string(tag)?;
        let format = if string.contains('Z') {
            "%y%m%d%H%M%SZ"
        } else {
            "%y%m%d%H%M%S%z"
        };
        chrono::NaiveDateTime::parse_from_str(&string, format)
            .ok()
            .context(error::InvalidDateSnafu)
            .map(|date| types::UtcTime::from_utc(date, chrono::Utc))
    }

    fn decode_sequence_of<D: Decode>(&mut self, tag: Tag) -> Result<Vec<D>, Self::Error> {
        self.decode_sequence(tag, |decoder| {
            let mut items = Vec::new();

            while let Ok(item) = D::decode(decoder) {
                items.push(item);
            }

            Ok(items)
        })
    }

    fn decode_set_of<D: Decode + Ord>(&mut self, tag: Tag) -> Result<types::SetOf<D>, Self::Error> {
        self.decode_sequence(tag, |decoder| {
            let mut items = types::SetOf::new();

            while let Ok(item) = D::decode(decoder) {
                items.insert(item);
            }

            Ok(items)
        })
    }

    fn decode_sequence<D, F: FnOnce(&mut Self) -> Result<D>>(
        &mut self,
        tag: Tag,
        decode_fn: F,
    ) -> Result<D> {
        self.parse_constructed_contents(tag, true, decode_fn)
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D> {
        self.parse_constructed_contents(tag, false, D::decode)
    }

    fn decode_set<FIELDS, SET, F>(&mut self, tag: Tag, decode_fn: F) -> Result<SET, Self::Error>
    where
        SET: Decode,
        FIELDS: Decode,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>,
    {
        self.decode_sequence(tag, |decoder| {
            let mut fields = Vec::new();

            while let Ok(value) = FIELDS::decode(decoder) {
                fields.push(value);
            }

            (decode_fn)(fields)
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct C2;
    impl AsnType for C2 {
        const TAG: Tag = Tag::new(Class::Context, 2);
    }

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct A3;
    impl AsnType for A3 {
        const TAG: Tag = Tag::new(Class::Application, 3);
    }

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct A7;
    impl AsnType for A7 {
        const TAG: Tag = Tag::new(Class::Application, 7);
    }

    use super::*;
    use crate::types::*;

    fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, self::Error> {
        let mut decoder = self::Decoder::new(input, self::DecoderOptions::ber());
        match T::decode(&mut decoder) {
            Ok(result) => {
                assert_eq!(decoder.decoded_len(), input.len());
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    #[test]
    fn boolean() {
        assert_eq!(true, decode::<bool>(&[0x01, 0x01, 0xff]).unwrap());
        assert_eq!(false, decode::<bool>(&[0x01, 0x01, 0x00]).unwrap());
    }

    #[test]
    fn tagged_boolean() {
        assert_eq!(
            Explicit::<C2, _>::new(true),
            decode(&[0xa2, 0x03, 0x01, 0x01, 0xff]).unwrap()
        );
    }

    #[test]
    fn integer() {
        assert_eq!(
            32768,
            decode::<i32>(&[0x02, 0x03, 0x00, 0x80, 0x00,]).unwrap()
        );
        assert_eq!(32767, decode::<i32>(&[0x02, 0x02, 0x7f, 0xff]).unwrap());
        assert_eq!(256, decode::<i16>(&[0x02, 0x02, 0x01, 0x00]).unwrap());
        assert_eq!(255, decode::<i16>(&[0x02, 0x02, 0x00, 0xff]).unwrap());
        assert_eq!(128, decode::<i16>(&[0x02, 0x02, 0x00, 0x80]).unwrap());
        assert_eq!(127, decode::<i8>(&[0x02, 0x01, 0x7f]).unwrap());
        assert_eq!(1, decode::<i8>(&[0x02, 0x01, 0x01]).unwrap());
        assert_eq!(0, decode::<i8>(&[0x02, 0x01, 0x00]).unwrap());
        assert_eq!(-1, decode::<i8>(&[0x02, 0x01, 0xff]).unwrap());
        assert_eq!(-128, decode::<i16>(&[0x02, 0x01, 0x80]).unwrap());
        assert_eq!(-129i16, decode::<i16>(&[0x02, 0x02, 0xff, 0x7f]).unwrap());
        assert_eq!(-256i16, decode::<i16>(&[0x02, 0x02, 0xff, 0x00]).unwrap());
        assert_eq!(-32768i32, decode::<i32>(&[0x02, 0x02, 0x80, 0x00]).unwrap());
        assert_eq!(
            -32769i32,
            decode::<i32>(&[0x02, 0x03, 0xff, 0x7f, 0xff]).unwrap()
        );

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
        let bitstring =
            types::BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());

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
    fn generalized_time() {
        let time = crate::types::GeneralizedTime::parse_from_str(
            "20001231205959.999+0000",
            "%Y%m%d%H%M%S%.3f%z",
        )
        .unwrap();
        let has_z = &[
            0x18, 0x13, 0x32, 0x30, 0x30, 0x30, 0x31, 0x32, 0x33, 0x31, 0x32, 0x30, 0x35, 0x39,
            0x35, 0x39, 0x2E, 0x39, 0x39, 0x39, 0x5A,
        ];
        assert_eq!(
            time,
            decode::<chrono::DateTime::<chrono::FixedOffset>>(has_z).unwrap()
        );
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
        use types::Ia5String;
        // Taken from examples in 8.9 of X.690.
        #[derive(Debug, PartialEq)]
        struct Foo {
            name: Ia5String,
            ok: bool,
        }

        impl types::AsnType for Foo {
            const TAG: Tag = Tag::SEQUENCE;
        }

        impl Decode for Foo {
            fn decode_with_tag<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
            ) -> Result<Self, D::Error> {
                decoder.decode_sequence(tag, |sequence| {
                    let name: Ia5String = Ia5String::decode(sequence)?;
                    let ok: bool = bool::decode(sequence)?;
                    Ok(Self { name, ok })
                })
            }
        }

        let foo = Foo {
            name: String::from("Smith").into(),
            ok: true,
        };
        let bytes = &[
            0x30, 0x0A, // TAG + LENGTH
            0x16, 0x05, 0x53, 0x6d, 0x69, 0x74, 0x68, // Ia5String "Smith"
            0x01, 0x01, 0xff, // BOOL True
        ];

        assert_eq!(foo, decode(bytes).unwrap());
    }

    #[test]
    fn tagging() {
        type Type1 = VisibleString;
        type Type2 = Implicit<A3, Type1>;
        type Type3 = Explicit<C2, Type2>;
        type Type4 = Implicit<A7, Type3>;
        type Type5 = Implicit<C2, Type2>;

        let jones = String::from("Jones");
        let jones1 = Type1::from(jones);
        let jones2 = Type2::from(jones1.clone());
        let jones3 = Type3::from(jones2.clone());
        let jones4 = Type4::from(jones3.clone());
        let jones5 = Type5::from(jones2.clone());

        assert_eq!(
            jones1,
            decode(&[0x1A, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones2,
            decode(&[0x43, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones3,
            decode(&[0xa2, 0x07, 0x43, 0x5, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones4,
            decode(&[0x67, 0x07, 0x43, 0x5, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
        assert_eq!(
            jones5,
            decode(&[0x82, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73]).unwrap()
        );
    }

    #[test]
    fn flip1() {
        let _ = decode::<Open>(&[
            0x10, 0x10, 0x23, 0x00, 0xfe, 0x7f, 0x10, 0x03, 0x00, 0xff, 0xe4, 0x04, 0x50, 0x10,
            0x50, 0x10, 0x10, 0x10,
        ]);
    }

    #[test]
    fn any() {
        let expected = &[0x1A, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73];
        assert_eq!(
            Any {
                contents: expected.to_vec()
            },
            decode(expected).unwrap()
        );
    }

    #[test]
    fn any_indefinite() {
        let any = &[
            0x30, 0x80, 0x2C, 0x80, 0x04, 0x03, 0x4A, 0x6F, 0x6E, 0x04, 0x02, 0x65, 0x73, 0x00,
            0x00, 0x00, 0x00,
        ];
        assert_eq!(
            Any {
                contents: any.to_vec()
            },
            decode(any).unwrap(),
        );
    }

    #[test]
    fn any_indefinite_fail_no_eoc() {
        let any = &[
            0x30, 0x80, 0x2C, 0x80, 0x04, 0x03, 0x4A, 0x6F, 0x6E, 0x04, 0x02, 0x65, 0x73, 0x00,
            0x00,
        ];
        assert!(decode::<Any>(any).is_err());
    }

    #[test]
    fn decoding_oid() {
        use crate::Decoder;

        let mut decoder =
            super::Decoder::new(&[0x06, 0x03, 0x88, 0x37, 0x01], DecoderOptions::der());
        let oid = decoder.decode_object_identifier(Tag::OBJECT_IDENTIFIER);
        assert!(oid.is_ok());
        let oid = oid.unwrap();
        assert_eq!(ObjectIdentifier::new([2, 999, 1].to_vec()).unwrap(), oid);
    }
}
