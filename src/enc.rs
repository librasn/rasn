//! Generic ASN.1 encoding framework.

use crate::types::{self, AsnType, Constraints, Enumerated, IntegerType, SetOf, Tag};

use num_bigint::BigInt;
pub use rasn_derive::Encode;

/// A **data type** that can be encoded to a ASN.1 data format.
pub trait Encode: AsnType {
    /// Encodes `self`'s data into the given [`crate::Encoder`].
    ///
    /// **Note for implementors** You typically do not need to implement this.
    /// The default implementation will call [`Encode::encode_with_tag_and_constraints`] with
    /// your types associated [`AsnType::TAG`] and [`AsnType::CONSTRAINTS`]. You
    /// should only ever need to implement this if you have a type that *cannot*
    /// be implicitly tagged, such as a `CHOICE` type, in which case you want to
    /// implement encoding in [`Self::encode`].
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, Self::TAG, Self::CONSTRAINTS)
    }

    /// Encode this value with `tag` into the given [`crate::Encoder`].
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, tag, Self::CONSTRAINTS)
    }

    /// Encode this value into the given [`crate::Encoder`] with the
    /// constraints the values this is allowed to encode into.
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn encode_with_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        self.encode_with_tag_and_constraints(encoder, Self::TAG, constraints)
    }

    /// Encode this value with `tag` into the given [`crate::Encoder`] with the
    /// constraints the values this is allowed to encode into.
    ///
    /// **Note** For `CHOICE` and other types that cannot be implicitly tagged
    /// this will **explicitly tag** the value, for all other types, it will
    /// **implicitly** tag the value.
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error>;
}

/// A **data format** encode any ASN.1 data type.
/// Const `RC` is the count of root components in sequence or set.
/// Const `EC` is the count of extension components in sequence or set.
pub trait Encoder<const RC: usize = 0, const EC: usize = 0> {
    /// The associated success type returned on success.
    type Ok;
    /// The associated error type returned on failure.
    type Error: Error + Into<crate::error::EncodeError> + From<crate::error::EncodeError>;
    /// Helper type for encoding recursive `Encoder` instances with different `RC` or  `EC` values.
    type AnyEncoder<const R: usize, const E: usize>: Encoder<RC, EC, Ok = Self::Ok, Error = Self::Error>
        + Encoder;

    /// Returns codec variant of `Codec` that current encoder is encoding.
    fn codec(&self) -> crate::Codec;

    /// Encode an unknown ASN.1 value.
    fn encode_any(&mut self, tag: Tag, value: &types::Any) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BOOL` value.
    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BIT STRING` value.
    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BitStr,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `ENUMERATED` value.
    fn encode_enumerated<E: Enumerated>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `OBJECT IDENTIFIER` value.
    fn encode_object_identifier(
        &mut self,
        tag: Tag,
        value: &[u32],
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `INTEGER` value.
    fn encode_integer<I: IntegerType>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &I,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `NULL` value.
    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error>;

    /// Encode a `OCTET STRING` value.
    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `GeneralString` value.
    fn encode_general_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::GeneralString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Utf8String` value.
    fn encode_utf8_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &str,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `VisibleString` value.
    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Ia5String` value.
    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `Ia5String` value.
    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `NumericString` value.
    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `TeletexString` value.
    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::TeletexString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `BmpString` value.
    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `GeneralizedTime` value.
    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `UtcTime` value.
    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a 'Date' value.
    fn encode_date(&mut self, tag: Tag, value: &types::Date) -> Result<Self::Ok, Self::Error>;

    /// Encode a explicitly tagged value.
    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `SEQUENCE` value.
    ///
    /// Const `R` is the count of root components in sequence or set.
    /// Const `E` is the count of extension components in sequence or set.
    /// Generic `C` is the sequence type.
    /// NOTE: If you implement this manually, make sure to encode fields in the same order and pass the correct count of fields.
    fn encode_sequence<const R: usize, const E: usize, C, F>(
        &mut self,
        tag: Tag,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<R, E>,
        F: FnOnce(&mut Self::AnyEncoder<R, E>) -> Result<(), Self::Error>;

    /// Encode a `SEQUENCE OF` value.
    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        value: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a `SET` value.
    ///
    /// Const `N` is the count of root components in sequence or set.
    /// Generic `C` is the set type.
    fn encode_set<const R: usize, const E: usize, C, F>(
        &mut self,
        tag: Tag,
        value: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed<R, E>,
        F: FnOnce(&mut Self::AnyEncoder<R, E>) -> Result<(), Self::Error>;

    /// Encode a `SET OF` value.
    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        tag: Tag,
        value: &types::SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode the value a field or skip if it matches the default..
    fn encode_or_default<E: Encode + Default + PartialEq>(
        &mut self,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        if value != &E::default() {
            self.encode_some(value)
        } else {
            self.encode_none::<E>()
        }
    }

    /// Encode the present value of an optional field.
    fn encode_with_tag_or_default<E: Encode + Default + PartialEq>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        if value != &E::default() {
            self.encode_some_with_tag(tag, value)
        } else {
            self.encode_none_with_tag(tag)
        }
    }

    /// Encode the present value of an optional field.
    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error>;

    /// Encode the present value of an optional field.
    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_some_with_tag_and_constraints(tag, E::CONSTRAINTS, value)
    }

    /// Encode the present value of an optional field.
    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode the absent value of an optional field.
    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error>;

    /// Encode the absent value with `tag` of an optional field.
    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error>;

    /// Encode the present value of an optional field.
    fn encode_default<E: Encode + PartialEq>(
        &mut self,
        value: &E,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some(value),
            None => self.encode_none::<E>(),
        }
    }

    /// Encode the present value of an optional field.
    fn encode_default_with_tag<E: Encode + PartialEq>(
        &mut self,
        tag: Tag,
        value: &E,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some_with_tag(tag, value),
            None => self.encode_none_with_tag(tag),
        }
    }

    /// Encode the present value of an optional field.
    fn encode_default_with_tag_and_constraints<E: Encode + PartialEq>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some_with_tag_and_constraints(tag, constraints, value),
            None => self.encode_none_with_tag(tag),
        }
    }

    /// Encode the present constrained value of an optional field.
    fn encode_default_with_constraints<E: Encode + PartialEq>(
        &mut self,
        constraints: Constraints,
        value: &E,
        default: impl FnOnce() -> E,
    ) -> Result<Self::Ok, Self::Error> {
        match (*value != (default)()).then_some(value) {
            Some(value) => self.encode_some_with_tag_and_constraints(E::TAG, constraints, value),
            None => self.encode_none_with_tag(E::TAG),
        }
    }

    /// Encode a `CHOICE` value.
    fn encode_choice<E: Encode + crate::types::Choice>(
        &mut self,
        constraints: Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a extension addition value.
    fn encode_extension_addition<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: E,
    ) -> Result<Self::Ok, Self::Error>;

    /// Encode a extension addition group value.
    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        value: Option<&E>,
    ) -> Result<Self::Ok, Self::Error>
    where
        E: Encode + crate::types::Constructed<RL, EL>;
}

/// A generic error that occurred while trying to encode ASN.1.
pub trait Error: core::fmt::Display {
    /// Creates a new general error using `msg` and current `codec` when encoding ASN.1.
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self;
}

impl Error for core::convert::Infallible {
    fn custom<D: core::fmt::Display>(msg: D, codec: crate::Codec) -> Self {
        core::panic!("Infallible error! {}, from: {}", msg, codec)
    }
}

impl<E: Encode> Encode for &'_ E {
    fn encode<EN: Encoder>(&self, encoder: &mut EN) -> Result<(), EN::Error> {
        E::encode(self, encoder)
    }

    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        E::encode_with_tag(self, encoder, tag)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        E::encode_with_constraints(self, encoder, constraints)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag_and_constraints(self, encoder, tag, constraints)
    }
}

impl Encode for () {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_null(tag).map(drop)
    }
}

impl<E: Encode> Encode for Option<E> {
    fn encode<EN: Encoder>(&self, encoder: &mut EN) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some::<E>(value),
            None => encoder.encode_none::<E>(),
        }
        .map(drop)
    }

    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some_with_tag(tag, value),
            None => encoder.encode_none_with_tag(tag),
        }
        .map(drop)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => {
                encoder.encode_some_with_tag_and_constraints(Self::TAG, constraints, value)
            }
            None => encoder.encode_none_with_tag(Self::TAG),
        }
        .map(drop)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        match self {
            Some(value) => encoder.encode_some_with_tag_and_constraints(tag, constraints, value),

            None => encoder.encode_none_with_tag(tag),
        }
        .map(drop)
    }
}

impl Encode for bool {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bool(tag, *self).map(drop)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+) => {
        $(
            impl Encode for $int {
                fn encode_with_tag_and_constraints<E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints) -> Result<(), E::Error> {
                    encoder.encode_integer(
                        tag,
                        constraints,
                        self
                    ).map(drop)
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
    u128,
    usize
}

impl Encode for BigInt {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_integer(tag, constraints, self).map(drop)
    }
}

impl<const START: i128, const END: i128> Encode for types::ConstrainedInteger<START, END> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_integer(tag, constraints, &**self).map(drop)
    }
}

impl Encode for types::Integer {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_integer(tag, constraints, self).map(drop)
    }
}

impl Encode for types::OctetString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_octet_string(tag, constraints, self)
            .map(drop)
    }
}

impl Encode for types::Utf8String {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_utf8_string(tag, constraints, self).map(drop)
    }
}

impl Encode for &'_ str {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_utf8_string(tag, constraints, self).map(drop)
    }
}

impl Encode for types::ObjectIdentifier {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_object_identifier(tag, self).map(drop)
    }
}

impl Encode for types::Oid {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_object_identifier(tag, self).map(drop)
    }
}

impl Encode for types::UtcTime {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_utc_time(tag, self).map(drop)
    }
}

impl Encode for types::GeneralizedTime {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_generalized_time(tag, self).map(drop)
    }
}

impl Encode for types::Any {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_any(tag, self).map(drop)
    }
}

impl<E: Encode> Encode for alloc::boxed::Box<E> {
    fn encode<EN: Encoder>(&self, encoder: &mut EN) -> Result<(), EN::Error> {
        E::encode(self, encoder)
    }

    fn encode_with_tag<EN: Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
        E::encode_with_tag(self, encoder, tag)
    }

    fn encode_with_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        E::encode_with_constraints(self, encoder, constraints)
    }

    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        E::encode_with_tag_and_constraints(self, encoder, tag, constraints)
    }
}

impl<E: Encode> Encode for alloc::vec::Vec<E> {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        encoder.encode_sequence_of(tag, self, constraints).map(drop)
    }
}

impl<E: Encode + Eq + core::hash::Hash> Encode for SetOf<E> {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        encoder.encode_set_of(tag, self, constraints).map(drop)
    }
}

impl<E: Encode, const N: usize> Encode for [E; N] {
    fn encode_with_tag_and_constraints<EN: Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), EN::Error> {
        encoder.encode_sequence_of(tag, self, constraints).map(drop)
    }
}

impl<T: AsnType, V: Encode> Encode for types::Implicit<T, V> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        V::encode_with_tag_and_constraints(&self.value, encoder, tag, constraints).map(drop)
    }
}

impl<T: AsnType, V: Encode> Encode for types::Explicit<T, V> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_explicit_prefix(tag, &self.value).map(drop)
    }
}
