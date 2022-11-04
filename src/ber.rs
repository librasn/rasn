//! # Basic Encoding Rules

pub mod de;
pub mod enc;
mod identifier;
mod rules;

pub use identifier::Identifier;
pub(crate) use rules::EncodingRules;

/// Attempts to decode `T` from `input` using BER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, de::Error> {
    T::decode(&mut de::Decoder::new(input, de::DecoderOptions::ber()))
}

/// Attempts to encode `value` to BER.
pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, enc::Error> {
    let mut enc = enc::Encoder::new(enc::EncoderOptions::ber());

    value.encode(&mut enc)?;

    Ok(enc.output())
}

#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned;
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::types::*;

    use super::*;

    #[derive(Clone, Copy, Hash, Debug, PartialEq)]
    struct C0;
    impl AsnType for C0 {
        const TAG: Tag = Tag::new(Class::Context, 0);
    }

    #[test]
    fn bit_string() {
        const DATA: &[u8] = &[0, 0xD0];
        let small = BitString::from_vec(DATA.to_owned());
        let bits = BitString::from_vec([0x0A, 0x3B, 0x5F, 0x29, 0x1C, 0xD0][..].to_owned());
        let padding_test = BitString::from_element(0x42);
        let padding_expected: &[u8] = &[0x03, 02, 0x00, 0x42];
        let trailing_test = bitvec::bitvec![u8, bitvec::prelude::Msb0; 1, 0, 0, 0, 0, 1, 1, 0];
        let trailing_expected: &[u8] = &[0x03, 02, 0x00, 0x86];

        assert_eq!(
            small,
            decode::<BitString>(&encode(&small).unwrap()).unwrap()
        );
        assert_eq!(bits, decode::<BitString>(&encode(&bits).unwrap()).unwrap());
        assert_eq!(padding_expected, encode(&padding_test).unwrap());
        assert_eq!(trailing_expected, encode(&trailing_test).unwrap());
    }

    #[test]
    fn implicit_prefix() {
        type MyInteger = Implicit<C0, u64>;

        let new_int = MyInteger::new(5);

        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }

    #[test]
    fn explicit_prefix() {
        type MyInteger = Explicit<C0, u64>;

        let new_int = MyInteger::new(5);
        let data = &[0xA0, 3, 0x2, 0x1, 5][..];

        assert_eq!(data, &encode(&new_int).unwrap());
        assert_eq!(new_int, decode(&encode(&new_int).unwrap()).unwrap());
    }

    #[test]
    fn implicit_tagged_constructed() {
        type ImpVec = Implicit<C0, Vec<i32>>;

        let value = ImpVec::new(vec![1, 2]);
        let data = &[0xA0, 6, 2, 1, 1, 2, 1, 2][..];

        assert_eq!(data, &*crate::ber::encode(&value).unwrap());
        assert_eq!(value, crate::ber::decode::<ImpVec>(data).unwrap());
    }

    #[test]
    fn explicit_empty_tag() {
        type EmptyTag = Explicit<C0, Option<()>>;

        let value = EmptyTag::new(None);
        let data = &[0xA0, 0][..];

        assert_eq!(data, &*crate::ber::encode(&value).unwrap());
        assert_eq!(value, crate::ber::decode::<EmptyTag>(data).unwrap());
    }

    #[test]
    fn set() {
        #[derive(Debug, PartialEq)]
        struct Set {
            age: u32,
            name: Utf8String,
        }

        impl AsnType for Set {
            const TAG: Tag = Tag::SET;
        }

        impl crate::types::Constructed for Set {
            const FIELDS: crate::types::fields::Fields = crate::types::fields::Fields::from_static(&[
                crate::types::fields::Field::new_required(u32::TAG, u32::TAG_TREE),
                crate::types::fields::Field::new_required(Utf8String::TAG, Utf8String::TAG_TREE),
            ]);
        }

        let example = Set {
            age: 1,
            name: "Jane".into(),
        };
        let age_then_name = [0x31, 0x9, 0x2, 0x1, 0x1, 0xC, 0x4, 0x4a, 0x61, 0x6e, 0x65];
        let name_then_age = [0x31, 0x9, 0xC, 0x4, 0x4a, 0x61, 0x6e, 0x65, 0x2, 0x1, 0x1];

        assert_eq!(&age_then_name[..], crate::ber::encode(&example).unwrap());

        assert_eq!(
            crate::ber::decode::<Set>(&age_then_name).unwrap(),
            crate::ber::decode::<Set>(&name_then_age).unwrap()
        );

        impl crate::Decode for Set {
            fn decode_with_tag_and_constraints<D: crate::Decoder>(
                decoder: &mut D,
                tag: Tag,
                _: Constraints,
            ) -> Result<Self, D::Error> {
                use crate::de::Error;

                #[derive(crate::AsnType, crate::Decode)]
                #[rasn(crate_root = "crate")]
                #[rasn(choice)]
                pub enum Fields {
                    Age(u32),
                    Name(Utf8String),
                }

                decoder.decode_set::<Fields, _, _, _>(
                    tag,
                    <_>::default(),
                    |decoder, indice, tag| {
                        match (indice, tag) {
                            (0, u32::TAG) => <_>::decode(decoder).map(Fields::Age),
                            (1, Utf8String::TAG) => <_>::decode(decoder).map(Fields::Name),
                            (_, _) => Err(D::Error::custom("unknown field")),
                        }
                    },
                    |fields| {
                        let mut age = None;
                        let mut name = None;

                        for field in fields {
                            match field {
                                Fields::Age(value) => age = value.into(),
                                Fields::Name(value) => name = value.into(),
                            }
                        }

                        Ok(Self {
                            age: age.ok_or_else(|| D::Error::missing_field("age"))?,
                            name: name.ok_or_else(|| D::Error::missing_field("name"))?,
                        })
                    }
                )
            }
        }

        impl crate::Encode for Set {
            fn encode_with_tag_and_constraints<EN: crate::Encoder>(
                &self,
                encoder: &mut EN,
                tag: crate::Tag,
                constraints: Constraints,
            ) -> Result<(), EN::Error> {
                encoder.encode_set::<Self, _>(tag, constraints, |encoder| {
                    self.age.encode(encoder)?;
                    self.name.encode(encoder)?;
                    Ok(())
                })?;

                Ok(())
            }
        }
    }
}
