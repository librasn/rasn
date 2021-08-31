//! Generic ASN.1 decoding framework.

use alloc::{boxed::Box, vec::Vec};
use core::convert::TryInto;

use crate::types::{self, AsnType, Tag};

pub use nom::Needed;
pub use rasn_derive::Decode;

/// A **data type** that can decoded from any ASN.1 format.
pub trait Decode: Sized + AsnType {
    /// Decode this value from a given ASN.1 decoder.
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call `Decode::decode_with_tag` with
    /// your types associated `AsnType::TAG`. You should only ever need to
    /// implement this if you have a type that *cannot* be implicitly tagged,
    /// such as a `CHOICE` type, which case you want to implement the decoding
    /// in `decode`.
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        Self::decode_with_tag(decoder, Self::TAG)
    }

    /// Decode this value implicitly tagged with `tag` from a given ASN.1 decoder.
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error>;
}

/// A **data format** decode any ASN.1 data type.
pub trait Decoder: Sized {
    type Error: Error;

    /// Decode a unknown ASN.1 value identified by `tag` from the available input.
    fn decode_any(&mut self, tag: Tag) -> Result<Vec<u8>, Self::Error>;
    /// Decode a `BIT STRING` identified by `tag` from the available input.
    fn decode_bit_string(&mut self, tag: Tag) -> Result<types::BitString, Self::Error>;
    /// Decode a `BOOL` identified by `tag` from the available input.
    fn decode_bool(&mut self, tag: Tag) -> Result<bool, Self::Error>;
    /// Decode an enumerated enum's discriminant identified by `tag` from the available input.
    fn decode_enumerated(&mut self, tag: Tag) -> Result<types::Integer, Self::Error>;
    /// Decode a `INTEGER` identified by `tag` from the available input.
    fn decode_integer(&mut self, tag: Tag) -> Result<types::Integer, Self::Error>;
    /// Decode `NULL` identified by `tag` from the available input.
    fn decode_null(&mut self, tag: Tag) -> Result<(), Self::Error>;
    /// Decode a `OBJECT IDENTIFIER` identified by `tag` from the available input.
    fn decode_object_identifier(
        &mut self,
        tag: Tag,
    ) -> Result<types::ObjectIdentifier, Self::Error>;
    /// Decode a `SEQUENCE` identified by `tag` from the available input. Returning
    /// a new `Decoder` containing the sequence's contents to be decoded.
    fn decode_sequence<D, F>(&mut self, tag: Tag, decode_fn: F) -> Result<D, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<D, Self::Error>;
    /// Decode a `SEQUENCE OF D` where `D: Decode` identified by `tag` from the available input.
    fn decode_sequence_of<D: Decode>(&mut self, tag: Tag) -> Result<Vec<D>, Self::Error>;
    /// Decode a `OCTET STRING` identified by `tag` from the available input.
    fn decode_octet_string(&mut self, tag: Tag) -> Result<Vec<u8>, Self::Error>;
    /// Decode a `UTF8 STRING` identified by `tag` from the available input.
    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String, Self::Error>;
    /// Decode an ASN.1 value that has been explicitly prefixed with `tag` from the available input.
    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error>;
    /// Decode a `UtcTime` identified by `tag` from the available input.
    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime, Self::Error>;
    /// Decode a `GeneralizedTime` identified by `tag` from the available input.
    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime, Self::Error>;

    /// Decode a `OBJECT IDENTIFIER` identified by `tag` from the available input.
    /// This is a specialisation of [`Self::decode_object_identifier`] for
    /// formats where you can zero copy the input.
    fn decode_oid<'de>(&'de mut self, _tag: Tag) -> Result<&'de types::Oid, Self::Error> {
        Err(Self::Error::custom(
            "This format does not support losslessly decoding object identifiers.",
        ))
    }
}

/// A generic error that can occur while decoding ASN.1.
pub trait Error: core::fmt::Display {
    /// Creates a new general error using `msg` when decoding ASN.1.
    fn custom<D: core::fmt::Display>(msg: D) -> Self;
    /// Creates a new error about needing more data to finish parsing.
    fn incomplete(needed: Needed) -> Self;
    /// Creates a new error about exceeding the maximum allowed data for a type.
    fn exceeds_max_length(length: usize) -> Self;
}

impl Decode for () {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_null(tag)
    }
}

impl<D: Decode> Decode for Option<D> {
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DE::Error> {
        Ok(D::decode(decoder).ok())
    }

    fn decode_with_tag<DE: Decoder>(decoder: &mut DE, tag: Tag) -> Result<Self, DE::Error> {
        Ok(D::decode_with_tag(decoder, tag).ok())
    }
}

impl Decode for bool {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_bool(tag)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                core::convert::TryInto::try_into(decoder.decode_integer(tag)?)
                    .map_err(Error::custom)
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
    usize,
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        T::decode(decoder).map(Box::new)
    }

    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        T::decode_with_tag(decoder, tag).map(Box::new)
    }
}

impl Decode for types::Integer {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_integer(tag)
    }
}

impl Decode for types::OctetString {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_octet_string(tag).map(Self::from)
    }
}

impl Decode for types::ObjectIdentifier {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_object_identifier(tag)
    }
}

impl Decode for types::BitString {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_bit_string(tag)
    }
}

impl Decode for types::Utf8String {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_utf8_string(tag)
    }
}

impl Decode for types::UtcTime {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_utc_time(tag)
    }
}

impl Decode for types::GeneralizedTime {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_generalized_time(tag)
    }
}

impl<T: Decode> Decode for alloc::vec::Vec<T> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        decoder.decode_sequence_of(tag)
    }
}

impl<T: Decode + Default, const N: usize> Decode for [T; N] {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        let sequence = decoder.decode_sequence_of(tag)?;

        sequence.try_into().map_err(|seq: Vec<_>| {
            Error::custom(alloc::format!(
                "Incorrect number of items provided. Expected {}, Actual {}.",
                N,
                seq.len()
            ))
        })
    }
}

impl<T: AsnType, V: Decode> Decode for types::Implicit<T, V> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        Ok(Self::new(V::decode_with_tag(decoder, tag)?))
    }
}

impl<T: AsnType, V: Decode> Decode for types::Explicit<T, V> {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        Ok(Self::new(decoder.decode_explicit_prefix(tag)?))
    }
}
