//! Generic ASN.1 decoding framework.

use alloc::{boxed::Box, vec::Vec};
use num_bigint::BigInt;

use crate::error::DecodeError;
use crate::types::{self, AsnType, Constraints, Enumerated, SetOf, Tag};

pub use nom::Needed;
pub use rasn_derive::Decode;

/// A generic ASN.1 decoding iterator. JER and XER are not supported.
pub fn iter<D: Decode>(input: &[u8], codec: crate::codec::Codec) -> Iter<'_, D> {
    Iter::new(input, codec)
}

/// Represents the input buffer in one of two states:
/// - `Borrowed(&[u8])` for the original input slice.
/// - `Owned { data: Vec<u8>, pos: usize }` after extra data has been appended.
///   Here `pos` tracks how many bytes have been consumed.
enum IterBuffer<'a> {
    Borrowed(&'a [u8]),
    Owned { data: Vec<u8>, pos: usize },
}

impl IterBuffer<'_> {
    /// Returns the current unread portion of the data.
    fn as_slice(&self) -> &[u8] {
        match self {
            IterBuffer::Borrowed(slice) => slice,
            IterBuffer::Owned { data, pos } => &data[*pos..],
        }
    }
    /// Updates the buffer to account for the fact that `consumed` bytes were processed.
    fn update_after_consumption(&mut self, consumed: usize) {
        match self {
            IterBuffer::Borrowed(slice) => *slice = &slice[consumed..],
            IterBuffer::Owned { data, pos } => {
                *pos += consumed;
                // Drop the consumed bytes from the buffer once half of the data is already consumed.
                if *pos > data.len() / 2 {
                    data.drain(0..*pos);
                    *pos = 0;
                }
            }
        }
    }

    /// Converts a Borrowed variant to an Owned one.
    #[allow(clippy::wrong_self_convention)]
    fn to_owned(&mut self) {
        if let IterBuffer::Borrowed(slice) = self {
            let vec = slice.to_vec();
            *self = IterBuffer::Owned { data: vec, pos: 0 };
        }
    }

    /// Appends new bytes.
    ///
    /// Internal buffer is in the Owned state from this point forward.
    fn extend(&mut self, bytes: &[u8]) {
        self.to_owned();
        if let IterBuffer::Owned { data, .. } = self {
            data.extend_from_slice(bytes);
        }
    }
}

/// A generic ASN.1 decoding iterator.
pub struct Iter<'input, D: Decode> {
    buf: IterBuffer<'input>,
    codec: crate::codec::Codec,
    _kind: core::marker::PhantomData<D>,
}

impl<'input, D: Decode> Iter<'input, D> {
    /// Create a new iterator from a borrowed input slice.
    pub fn new(input: &'input [u8], codec: crate::codec::Codec) -> Self {
        Self {
            buf: IterBuffer::Borrowed(input),
            codec,
            _kind: core::marker::PhantomData,
        }
    }

    /// Append new bytes to the input stream.
    /// After this call the iterator will switch to using an owned buffer.
    pub fn append_bytes(&mut self, bytes: &'input [u8]) {
        self.buf.extend(bytes);
    }
}

impl<D: Decode> Iterator for Iter<'_, D> {
    type Item = Result<D, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.buf.as_slice();
        match self.codec.decode_from_binary_with_remainder(input) {
            Ok((value, remainder)) => {
                // Determine how many bytes were consumed.
                let consumed = input.len() - remainder.len();
                self.buf.update_after_consumption(consumed);
                Some(Ok(value))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

/// A **data type** that can decoded from any ASN.1 format.
pub trait Decode: Sized + AsnType {
    /// Decode this value from a given ASN.1 decoder.
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call [`Decode::decode_with_tag_and_constraints`] with
    /// your types associated [`AsnType::TAG`]. You should only ever need to
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
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        Self::decode_with_tag_and_constraints(decoder, tag, Self::CONSTRAINTS)
    }

    /// Decode this value from a given ASN.1 decoder with a set of constraints
    /// on what values of that type are allowed.
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call [`Decode::decode_with_tag_and_constraints`] with
    /// your types associated [`AsnType::TAG`] and [`AsnType::CONSTRAINTS`].
    fn decode_with_constraints<D: Decoder>(
        decoder: &mut D,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        Self::decode_with_tag_and_constraints(decoder, Self::TAG, constraints)
    }

    /// Decode this value implicitly tagged with `tag` from a given ASN.1
    /// decoder with a set of constraints on what values of that type are allowed.
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error>;
}

/// A **data format** decode any ASN.1 data type.
///
/// Const `RCL` is the count of root components in the root component list of a sequence or set.
/// Const `ECL` is the count of extension additions in the extension addition component type list in a sequence or set.
pub trait Decoder<const RCL: usize = 0, const ECL: usize = 0>: Sized {
    /// The associated success type returned on success.
    type Ok;
    /// The associated error type returned on failure.
    type Error: Error + Into<crate::error::DecodeError> + From<crate::error::DecodeError>;
    /// Helper type for decoding nested instances of `Decoder` with different fields.
    type AnyDecoder<const R: usize, const E: usize>: Decoder<RCL, ECL, Ok = Self::Ok, Error = Self::Error>
        + Decoder;

    /// Returns codec variant of `Codec` that current decoder is decoding.
    #[must_use]
    fn codec(&self) -> crate::Codec;

    /// Decode an unknown ASN.1 value identified by `tag` from the available input.
    fn decode_any(&mut self) -> Result<types::Any, Self::Error>;
    /// Decode a `BIT STRING` identified by `tag` from the available input.
    fn decode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::BitString, Self::Error>;
    /// Decode a `BOOL` identified by `tag` from the available input.
    fn decode_bool(&mut self, tag: Tag) -> Result<bool, Self::Error>;
    /// Decode an enumerated enum's discriminant identified by `tag` from the available input.
    fn decode_enumerated<E: Enumerated>(&mut self, tag: Tag) -> Result<E, Self::Error>;
    /// Decode a `INTEGER` identified by `tag` from the available input.
    fn decode_integer<I: types::IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<I, Self::Error>;

    /// Decode a `REAL` identified by `tag` from the available input.
    fn decode_real<R: types::RealType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<R, Self::Error>;

    /// Decode `NULL` identified by `tag` from the available input.
    fn decode_null(&mut self, tag: Tag) -> Result<(), Self::Error>;
    /// Decode a `OBJECT IDENTIFIER` identified by `tag` from the available input.
    fn decode_object_identifier(
        &mut self,
        tag: Tag,
    ) -> Result<types::ObjectIdentifier, Self::Error>;
    /// Decode a `SEQUENCE` identified by `tag` from the available input. Returning
    /// a new `Decoder` containing the sequence's contents to be decoded.
    ///
    /// Const `RC` is the count of root components in a sequence.
    /// Const `EC` is the count of extension addition components in a sequence.
    /// Generic `D` is the sequence type.
    /// Generic `DF` is the closure that will initialize the sequence with default values, typically when no values are present.
    /// Generic `F` is the closure that will decode the sequence by decoding the fields in the order as defined in the type.
    /// NOTE: If you implement this manually, make sure to decode fields in the same order and pass the correct count of fields.
    fn decode_sequence<const RC: usize, const EC: usize, D, DF, F>(
        &mut self,
        tag: Tag,
        default_initializer_fn: Option<DF>,
        decode_fn: F,
    ) -> Result<D, Self::Error>
    where
        D: crate::types::Constructed<RC, EC>,
        DF: FnOnce() -> D,
        F: FnOnce(&mut Self::AnyDecoder<RC, EC>) -> Result<D, Self::Error>;
    /// Decode a `SEQUENCE OF D` where `D: Decode` identified by `tag` from the available input.
    fn decode_sequence_of<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Vec<D>, Self::Error>;
    /// Decode a `SET OF D` where `D: Decode` identified by `tag` from the available input.
    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::SetOf<D>, Self::Error>;
    /// Decode a `OCTET STRING` identified by `tag` from the available input.
    fn decode_octet_string<'buf, T>(
        &'buf mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<T, Self::Error>
    where
        T: From<&'buf [u8]> + From<Vec<u8>>;

    /// Decode a `UTF8 STRING` identified by `tag` from the available input.
    fn decode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::Utf8String, Self::Error>;

    /// Decode a `VisibleString` identified by `tag` from the available input.
    fn decode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::VisibleString, Self::Error>;

    /// Decode a `GeneralString` identified by `tag` from the available input.
    fn decode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GeneralString, Self::Error>;

    /// Decode a `GraphicString` identified by `tag` from the available input.
    fn decode_graphic_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::GraphicString, Self::Error>;

    /// Decode a `Ia5String` identified by `tag` from the available input.
    fn decode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::Ia5String, Self::Error>;

    /// Decode a `PrintableString` identified by `tag` from the available input.
    fn decode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::PrintableString, Self::Error>;

    /// Decode a `NumericString` identified by `tag` from the available input.
    fn decode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::NumericString, Self::Error>;

    /// Decode a `TeletexString` identified by `tag` from the available input.
    fn decode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::TeletexString, Self::Error>;

    /// Decode a `BmpString` identified by `tag` from the available input.
    fn decode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<types::BmpString, Self::Error>;

    /// Decode an ASN.1 value that has been explicitly prefixed with `tag` from the available input.
    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error>;
    /// Decode an optional ASN.1 type that has been explicitly prefixed with `tag` from the available input.
    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        tag: Tag,
    ) -> Result<Option<D>, Self::Error>;
    /// Decode a `UtcTime` identified by `tag` from the available input.
    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime, Self::Error>;
    /// Decode a `GeneralizedTime` identified by `tag` from the available input.
    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime, Self::Error>;
    /// Decode a 'DATE' identified by 'tag' from the available input
    fn decode_date(&mut self, tag: Tag) -> Result<types::Date, Self::Error>;

    /// Decode a `SET` identified by `tag` from the available input. Decoding
    /// `SET`s works a little different than other methods, as you need to
    /// provide two types `SET` and `FIELDS`, `SET` represents the complete type,
    /// and `FIELDS` must represent a `CHOICE` with a variant for each field
    /// from `SET`. As with `SET`s the field order is not guarenteed, so you'll
    /// have map from `Vec<FIELDS>` to `SET` in `decode_operation`.
    ///
    /// Const `RC` is the count of root components in a sequence.
    /// Const `EC` is the count of extension addition components in a sequence.
    /// Generic `FIELDS` is the choice type, used by `F` to map the decoded field values correctly.
    /// Generic `SET` is the set type.
    /// Generic `D` is the closure that will decode the set by decoding the fields in the order as defined in the type.
    /// Generic `F` is the closure that will map the `FIELDS` to the set.
    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        tag: Tag,
        decode_fn: D,
        field_fn: F,
    ) -> Result<SET, Self::Error>
    where
        SET: Decode + crate::types::Constructed<RC, EC>,
        FIELDS: Decode,
        D: Fn(&mut Self::AnyDecoder<RC, EC>, usize, Tag) -> Result<FIELDS, Self::Error>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, Self::Error>;

    /// Decode an the optional value in a `SEQUENCE` or `SET`.
    fn decode_choice<D>(&mut self, constraints: Constraints) -> Result<D, Self::Error>
    where
        D: crate::types::DecodeChoice;

    /// Decode an the optional value in a `SEQUENCE` or `SET`.
    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, Self::Error>;

    /// Decode an the optional value in a `SEQUENCE` or `SET` with `tag`.
    /// Passing the correct tag is required even when used with codecs where
    /// the tag is not present.
    fn decode_optional_with_tag<D: Decode>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error>;

    /// Decode an the optional value in a `SEQUENCE` or `SET` with `constraints`.
    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>;

    /// Decode an the optional value in a `SEQUENCE` or `SET` with `tag`
    /// and `constraints`.
    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>;

    /// Decode a `DEFAULT` value in a `SEQUENCE` or `SET`.
    fn decode_default<D: Decode, F: FnOnce() -> D>(
        &mut self,
        default_fn: F,
    ) -> Result<D, Self::Error> {
        self.decode_default_with_tag(D::TAG, default_fn)
    }

    /// Decode a `DEFAULT` value in a `SEQUENCE` or `SET` with `tag` and `default_fn`.
    fn decode_default_with_tag<D: Decode, F: FnOnce() -> D>(
        &mut self,
        tag: Tag,
        default_fn: F,
    ) -> Result<D, Self::Error> {
        Ok(self
            .decode_optional_with_tag::<D>(tag)?
            .unwrap_or_else(default_fn))
    }

    /// Decode a `DEFAULT` value with constraints in a `SEQUENCE` or `SET` with a given `default_fn`.
    fn decode_default_with_constraints<D: Decode, F: FnOnce() -> D>(
        &mut self,
        default_fn: F,
        constraints: Constraints,
    ) -> Result<D, Self::Error> {
        Ok(self
            .decode_optional_with_constraints::<D>(constraints)?
            .unwrap_or_else(default_fn))
    }

    /// Decode a `DEFAULT` value in a `SEQUENCE` or `SET` with `tag`, `constraints` and `default_fn`.
    fn decode_default_with_tag_and_constraints<D: Decode, F: FnOnce() -> D>(
        &mut self,
        tag: Tag,
        default_fn: F,
        constraints: Constraints,
    ) -> Result<D, Self::Error> {
        Ok(self
            .decode_optional_with_tag_and_constraints::<D>(tag, constraints)?
            .unwrap_or_else(default_fn))
    }

    /// Decode an extension addition value in a `SEQUENCE` or `SET`.
    fn decode_extension_addition<D>(&mut self) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_constraints(Constraints::default())
    }
    /// Decode an extension addition with explicit tag in a `SEQUENCE` or `SET`.
    fn decode_extension_addition_with_explicit_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode;

    /// Decode an extension addition value with tag in a `SEQUENCE` or `SET`.
    fn decode_extension_addition_with_tag<D>(&mut self, tag: Tag) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_tag_and_constraints(tag, Constraints::default())
    }
    /// Decode an extension addition with constraints in a `SEQUENCE` or `SET`
    fn decode_extension_addition_with_constraints<D>(
        &mut self,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode,
    {
        self.decode_extension_addition_with_tag_and_constraints(D::TAG, constraints)
    }
    /// Decode a extension addition value with tag and constraints in a `SEQUENCE` or `SET`.
    fn decode_extension_addition_with_tag_and_constraints<D>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Option<D>, Self::Error>
    where
        D: Decode;

    /// Decode a `DEFAULT` value in a `SEQUENCE`'s or `SET`'s extension
    fn decode_extension_addition_with_default<D: Decode, F: FnOnce() -> D>(
        &mut self,
        default_fn: F,
    ) -> Result<D, Self::Error> {
        self.decode_extension_addition_with_default_and_constraints(
            default_fn,
            Constraints::default(),
        )
    }
    /// Decode a `DEFAULT` value with tag in a `SEQUENCE`'s or `SET`'s extension
    fn decode_extension_addition_with_default_and_tag<D: Decode, F: FnOnce() -> D>(
        &mut self,
        tag: Tag,
        default_fn: F,
    ) -> Result<D, Self::Error> {
        self.decode_extension_addition_with_default_and_tag_and_constraints::<D, F>(
            tag,
            default_fn,
            Constraints::default(),
        )
    }

    /// Decode a `DEFAULT` value with constraints in a `SEQUENCE`'s or `SET`'s extension
    fn decode_extension_addition_with_default_and_constraints<D: Decode, F: FnOnce() -> D>(
        &mut self,
        default_fn: F,
        constraints: Constraints,
    ) -> Result<D, Self::Error> {
        Ok(self
            .decode_extension_addition_with_constraints::<D>(constraints)?
            .unwrap_or_else(default_fn))
    }
    /// Decode a `DEFAULT` value with tag and constraints in a `SEQUENCE`'s or `SET`'s extension
    fn decode_extension_addition_with_default_and_tag_and_constraints<
        D: Decode,
        F: FnOnce() -> D,
    >(
        &mut self,
        tag: Tag,
        default_fn: F,
        constraints: Constraints,
    ) -> Result<D, Self::Error> {
        Ok(self
            .decode_extension_addition_with_tag_and_constraints::<D>(tag, constraints)?
            .unwrap_or_else(default_fn))
    }

    /// Decode a extension addition group in a `SEQUENCE` or `SET`.
    ///
    /// Const `RC` is the count of root components in a sequence.
    /// Const `EC` is the count of extension addition components in a sequence.
    /// Generic `D` is the type of the extension addition group.
    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: Decode + crate::types::Constructed<RC, EC>,
    >(
        &mut self,
    ) -> Result<Option<D>, Self::Error>;
}

/// A generic error that can occur while decoding ASN.1.
/// Caller needs always to pass a `crate::Codec` variant to `Error` when implementing the decoder
pub trait Error: core::fmt::Display {
    /// Creates a new general error using `msg` when decoding ASN.1.
    #[must_use]
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self;
    /// Creates a new error about needing more data to finish parsing.
    #[must_use]
    fn incomplete(needed: Needed, codec: crate::Codec) -> Self;
    /// Creates a new error about exceeding the maximum allowed data for a type.
    #[must_use]
    fn exceeds_max_length(length: num_bigint::BigUint, codec: crate::Codec) -> Self;
    /// Creates a new error about a missing field.
    #[must_use]
    fn missing_field(name: &'static str, codec: crate::Codec) -> Self;
    /// Creates a new error about being unable to match any variant in a choice.
    #[must_use]
    fn no_valid_choice(name: &'static str, codec: crate::Codec) -> Self;
    /// Creates a new error about being unable to decode a field in a compound
    /// type, such as a set or sequence.
    #[must_use]
    fn field_error(name: &'static str, error: DecodeError, codec: crate::Codec) -> Self;
    /// Creates a new error about finding a duplicate field.
    #[must_use]
    fn duplicate_field(name: &'static str, codec: crate::Codec) -> Self;
    /// Create a new error about unknown field.
    #[must_use]
    fn unknown_field(index: usize, tag: Tag, codec: crate::Codec) -> Self;
}

impl Decode for () {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_null(tag)
    }
}

impl<D: Decode> Decode for Option<D> {
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DE::Error> {
        decoder.decode_optional()
    }

    fn decode_with_tag<DE: Decoder>(decoder: &mut DE, tag: Tag) -> Result<Self, DE::Error> {
        decoder.decode_optional_with_tag(tag)
    }

    fn decode_with_constraints<DE: Decoder>(
        decoder: &mut DE,
        constraints: Constraints,
    ) -> Result<Self, DE::Error> {
        decoder.decode_optional_with_constraints(constraints)
    }

    fn decode_with_tag_and_constraints<DE: Decoder>(
        decoder: &mut DE,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, DE::Error> {
        decoder.decode_optional_with_tag_and_constraints(tag, constraints)
    }
}

impl Decode for bool {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_bool(tag)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            fn decode_with_tag_and_constraints<D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints) -> Result<Self, D::Error> {
                decoder.decode_integer::<$int>(tag, constraints)
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
    // TODO cannot support u128 as it is constrained type by default and current constraints uses i128 for bounds
    // u128,
    usize,
    BigInt
}

impl<const START: i128, const END: i128> Decode for types::ConstrainedInteger<START, END> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder
            .decode_integer::<types::Integer>(tag, constraints)
            .map(Self)
    }
}

impl Decode for types::Integer {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_integer::<types::Integer>(tag, constraints)
    }
}

#[cfg(feature = "f32")]
impl Decode for f32 {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_real::<f32>(tag, Constraints::default())
    }
}

#[cfg(feature = "f64")]
impl Decode for f64 {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_real::<f64>(tag, Constraints::default())
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        T::decode(decoder).map(Box::new)
    }

    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        T::decode_with_tag(decoder, tag).map(Box::new)
    }

    fn decode_with_constraints<DE: Decoder>(
        decoder: &mut DE,
        constraints: Constraints,
    ) -> Result<Self, DE::Error> {
        T::decode_with_constraints(decoder, constraints).map(Box::new)
    }

    fn decode_with_tag_and_constraints<DE: Decoder>(
        decoder: &mut DE,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, DE::Error> {
        T::decode_with_tag_and_constraints(decoder, tag, constraints).map(Box::new)
    }
}

impl Decode for types::OctetString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder
            .decode_octet_string::<Vec<u8>>(tag, constraints)
            .map(Into::into)
    }
}

impl Decode for types::ObjectIdentifier {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_object_identifier(tag)
    }
}

impl Decode for types::Utf8String {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_utf8_string(tag, constraints)
    }
}

impl Decode for types::UtcTime {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_utc_time(tag)
    }
}

impl Decode for types::GeneralizedTime {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_generalized_time(tag)
    }
}

impl Decode for types::Any {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        _: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_any()
    }
}

impl<T: Decode> Decode for alloc::vec::Vec<T> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_sequence_of(tag, constraints)
    }
}

impl<T: Decode + Eq + core::hash::Hash> Decode for SetOf<T> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_set_of(tag, constraints)
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        let sequence = decoder.decode_sequence_of(tag, constraints)?;
        sequence.try_into().map_err(|seq: Vec<_>| {
            D::Error::from(DecodeError::incorrect_item_number_in_sequence(
                N,
                seq.len(),
                decoder.codec(),
            ))
        })
    }
}

impl<T: AsnType, V: Decode> Decode for types::Implicit<T, V> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        Ok(Self::new(V::decode_with_tag_and_constraints(
            decoder,
            tag,
            constraints,
        )?))
    }
}

impl<T: AsnType, V: Decode> Decode for types::Explicit<T, V> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        Ok(Self::new(decoder.decode_explicit_prefix(tag)?))
    }
}
