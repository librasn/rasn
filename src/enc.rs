use crate::{tag::Tag, types};

pub use rasn_derive::Encode;

/// A **data type** that can be encoded to a ASN.1 data format.
pub trait Encode: types::AsnType {
    /// Encodes `self`'s data into the given `Encoder`.
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call `Encode::encode_with_tag` with
    /// your types associated `AsnType::TAG`. You should only ever need to
    /// implement this if you have a type that *cannot* be implicitly tagged,
    /// such as a `CHOICE` type.
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        self.encode_with_tag(encoder, Self::TAG)
    }

    /// Encode this value implicitly tagged with `tag` into the given `Decoder`.
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error>;
}

/// A **data format** encode any ASN.1 data type.
pub trait Encoder {
    type Ok;
    type Error: Error;

    /// Encode an unknown ASN.1 value.
    fn encode_any(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error>;
    /// Encode a `BIT STRING` value.
    fn encode_bit_string(
        &mut self,
        tag: Tag,
        value: &types::BitString,
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `BOOL` value.
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error>;
    /// Encode a `ENUMERATED` value.
    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error>;
    /// Encode a explicitly tag value.
    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `GeneralizedTime` value.
    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `INTEGER` value.
    fn encode_integer(&mut self, tag: Tag, value: &types::Integer)
        -> Result<Self::Ok, Self::Error>;
    /// Encode a `NULL` value.
    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error>;
    /// Encode a `OBJECT IDENTIFIER` value.
    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `OCTET STRING` value.
    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error>;
    /// Encode a `SEQUENCE` value.
    fn encode_sequence<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Self::Error>;
    /// Encode a `SEQUENCE OF` value.
    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `UtcTime` value.
    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error>;
    /// Encode a `Utf8String` value.
    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error>;
}

/// A generic error that occurred while trying to encode ASN.1.
pub trait Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self;
}

impl Encode for () {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_null(tag).map(drop)
    }
}

impl<E: Encode> Encode for Option<E> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        if let Some(inner) = &self {
            Ok(Encode::encode_with_tag(inner, encoder, tag)?)
        } else {
            Ok(())
        }
    }
}

impl Encode for bool {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_bool(tag, *self).map(drop)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+) => {
        $(
            impl Encode for $int {
                fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
                    encoder.encode_integer(tag, &(*self).into()).map(drop)
                }
            }
        )+
    }
}

impl_integers! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize
}

impl Encode for types::Integer {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_integer(tag, self).map(drop)
    }
}

impl Encode for types::OctetString {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_octet_string(tag, self).map(drop)
    }
}

impl Encode for types::Utf8String {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_utf8_string(tag, self).map(drop)
    }
}

impl Encode for &'_ str {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_utf8_string(tag, self).map(drop)
    }
}

impl Encode for types::BitString {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_bit_string(tag, self).map(drop)
    }
}

impl Encode for types::ObjectIdentifier {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_object_identifier(tag, self).map(drop)
    }
}

impl Encode for types::UtcTime {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_utc_time(tag, self).map(drop)
    }
}

impl Encode for types::GeneralizedTime {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        encoder.encode_generalized_time(tag, self).map(drop)
    }
}

impl<E: Encode> Encode for types::SequenceOf<E> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        encoder.encode_sequence_of(tag, self).map(drop)
    }
}

impl<T: crate::types::AsnType, V: Encode> Encode for types::Implicit<T, V> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        V::encode_with_tag(&self.value, encoder, tag).map(drop)
    }
}

impl<T: crate::types::AsnType, V: Encode> Encode for types::Explicit<T, V> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        encoder.encode_explicit_prefix(tag, &self.value).map(drop)
    }
}

impl Encode for alloc::collections::BTreeMap<Tag, types::Open> {
    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        encoder
            .encode_sequence(tag, |encoder| {
                for (tag, value) in self {
                    <_>::encode_with_tag(value, encoder, *tag)?;
                }

                Ok(())
            })
            .map(drop)
    }
}
