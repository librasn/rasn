//! # ASN.1 Data Types
//! The `types` modules is a collection of Rust types and data structures that
//! are defined to represent various ASN.1 data types, and renamed to use
//! ASN.1's terminology.

mod any;
mod instance;
mod open;
mod prefix;
mod tag;

pub mod constraints;
pub mod fields;
pub mod variants;

pub(crate) mod oid;
pub(crate) mod strings;

use alloc::boxed::Box;
use num_bigint::BigUint;

pub use {
    self::{
        any::Any,
        constraints::{Constraint, Constraints, Extensible},
        instance::InstanceOf,
        oid::{ObjectIdentifier, Oid},
        open::Open,
        prefix::{Explicit, Implicit},
        strings::{
            BitStr, BitString, BmpString, FixedBitString, FixedOctetString, GeneralString,
            Ia5String, NumericString, OctetString, PrintableString, TeletexString, Utf8String,
            VisibleString,
        },
        tag::{Class, Tag, TagTree},
    },
    num_bigint::BigInt as Integer,
    rasn_derive::AsnType,
};

///  The `SET OF` type.
pub type SetOf<T> = alloc::collections::BTreeSet<T>;
///  The `UniversalString` type.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;
///  The `UTCTime` type.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  The `GeneralizedTime` type.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;
/// The `Date` type.
pub type Date =  Implicit<tag::DATE, UtcTime>;

///  The `SEQUENCE OF` type.
/// ## Usage
/// ASN1 declaration such as ...
/// ```asn
/// Test-type-a ::= SEQUENCE OF BOOLEAN
/// Test-type-b ::= SEQUENCE OF INTEGER(1,...)
/// ```
/// ... can be represented using `rasn` as ...
/// ```rust
/// use rasn::prelude::*;
///
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate)]
/// struct TestTypeA(pub SequenceOf<bool>);
///
/// // Constrained inner primitive types need to be wrapped in a helper newtype
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate, value("1", extensible))]
/// struct InnerTestTypeB(pub Integer);
///  
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate)]
/// struct TestTypeB(pub SequenceOf<InnerTestTypeB>);
/// ```
pub type SequenceOf<T> = alloc::vec::Vec<T>;

/// A trait representing any type that can represented in ASN.1.
pub trait AsnType {
    /// The associated tag for the type.
    ///
    /// **Note** When implementing CHOICE types, this should be set to
    /// [`Tag::EOC`] and instead set the [`Self::TAG_TREE`] constant to contain
    /// all variants.
    const TAG: Tag;
    /// The root of this type's tree of tag's if it a CHOICE type, otherwise its
    /// `Leaf` that points [`Self::TAG`].
    const TAG_TREE: TagTree = TagTree::Leaf(Self::TAG);

    const CONSTRAINTS: Constraints<'static> = Constraints::NONE;

    /// Identifier of an ASN.1 type as specified in the original specification
    /// if not identical with the identifier of `Self`
    const IDENTIFIER: Option<&'static str> = None;
}

/// A `SET` or `SEQUENCE` value.
pub trait Constructed {
    /// Fields contained in the "root component list".
    const FIELDS: self::fields::Fields;
    /// Fields contained in the list of extensions.
    const EXTENDED_FIELDS: Option<self::fields::Fields> = None;
}

/// A `CHOICE` value.
pub trait Choice: Sized {
    /// Variants contained in the "root component list".
    const VARIANTS: &'static [TagTree];
    /// Variants contained in the list of extensions.
    const EXTENDED_VARIANTS: Option<&'static [TagTree]> = None;
    /// Variant identifiers for text-based encoding rules
    const IDENTIFIERS: &'static [&'static str];
}

/// A `CHOICE` value.
pub trait DecodeChoice: Choice + crate::Decode {
    /// Decode the choice value based on the provided `tag`.
    fn from_tag<D: crate::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error>;
}

/// A `ENUMERATED` value.
pub trait Enumerated: Sized + 'static + PartialEq + Copy + core::fmt::Debug {
    /// Variants contained in the "root component list".
    const VARIANTS: &'static [Self];
    /// Variants contained in the list of extensions.
    const EXTENDED_VARIANTS: Option<&'static [Self]>;

    /// Variants contained in the "root component list" mapped to their respective discriminant.
    const DISCRIMINANTS: &'static [(Self, isize)];
    /// Variants contained in the list of extensions mapped to their respective discriminant, if
    /// present.
    const EXTENDED_DISCRIMINANTS: Option<&'static [(Self, isize)]>;

    /// Identifiers of enum variants
    const IDENTIFIERS: &'static [&'static str];

    /// Returns the number of "root" variants for a given type.
    fn variance() -> usize {
        Self::VARIANTS.len()
    }

    /// Returns the number of "extended" variants for a given type.
    fn extended_variance() -> usize {
        Self::EXTENDED_VARIANTS.map_or(0, |array| array.len())
    }

    /// Returns the number of "root" and "extended" variants for a given type.
    fn complete_variance() -> usize {
        Self::variance() + Self::extended_variance()
    }

    /// Whether `self` is a variant contained in `Self::EXTENDED_VARIANTS`.
    fn is_extended_variant(&self) -> bool {
        Self::EXTENDED_VARIANTS.map_or(false, |array| array.iter().any(|variant| variant == self))
    }

    /// Returns the enumeration for the variant, if it's an extended variant
    /// then it will return it's extended enumeration index.
    fn enumeration_index(&self) -> usize {
        if self.is_extended_variant() {
            Self::EXTENDED_VARIANTS
                .unwrap()
                .iter()
                .position(|lhs| lhs == self)
                .unwrap()
        } else {
            Self::VARIANTS
                .iter()
                .position(|lhs| lhs == self)
                .expect("Variant not defined in Enumerated::VARIANTS")
        }
    }

    /// Returns the discriminant value of `self`.
    fn discriminant(&self) -> isize {
        Self::DISCRIMINANTS
            .iter()
            .chain(
                Self::EXTENDED_DISCRIMINANTS
                    .iter()
                    .flat_map(|array| array.iter()),
            )
            .find_map(|(lhs, value)| (lhs == self).then_some(*value))
            .expect("variant not defined in `Enumerated`")
    }

    /// Returns a variant, if the provided discriminant matches any variant.
    fn from_discriminant(value: isize) -> Option<Self> {
        Self::DISCRIMINANTS
            .iter()
            .chain(
                Self::EXTENDED_DISCRIMINANTS
                    .iter()
                    .flat_map(|array| array.iter()),
            )
            .find_map(|(variant, discriminant)| (value == *discriminant).then_some(*variant))
    }

    /// Returns a variant, if the index matches any "root" variant.
    fn from_enumeration_index(index: usize) -> Option<Self> {
        Self::VARIANTS.get(index).copied()
    }

    /// Returns a variant, if the index matches any "extended" variant.
    fn from_extended_enumeration_index(index: usize) -> Option<Self> {
        Self::EXTENDED_VARIANTS.and_then(|array| array.get(index).copied())
    }

    /// Returns the variant identifier
    fn identifier(&self) -> &'static str {
        let index = if self.is_extended_variant() {
            Self::EXTENDED_VARIANTS
                .unwrap()
                .iter()
                .position(|lhs| lhs == self)
                .unwrap()
                + Self::VARIANTS.len()
        } else {
            Self::VARIANTS
                .iter()
                .position(|lhs| lhs == self)
                .expect("Variant not defined in Enumerated::VARIANTS")
        };
        Self::IDENTIFIERS[index]
    }

    /// Returns a variant, if the provided identifier matches any variant.
    fn from_identifier(identifier: &str) -> Option<Self> {
        Self::IDENTIFIERS
            .iter()
            .enumerate()
            .find(|id| id.1.eq(&identifier))
            .and_then(|(i, _)| {
                if i < Self::VARIANTS.len() {
                    Self::VARIANTS.get(i).copied()
                } else {
                    Self::EXTENDED_VARIANTS
                        .and_then(|array| array.get(i - Self::VARIANTS.len()).copied())
                }
            })
    }
}

/// A integer which has encoded constraint range between `START` and `END`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstrainedInteger<const START: i128, const END: i128>(pub(crate) Integer);

impl<const START: i128, const END: i128> AsnType for ConstrainedInteger<START, END> {
    const TAG: Tag = Tag::INTEGER;
    const CONSTRAINTS: Constraints<'static> =
        Constraints::new(&[constraints::Constraint::Value(Extensible::new(
            constraints::Value::new(constraints::Bounded::const_new(START, END)),
        ))]);
}

impl<const START: i128, const END: i128> core::ops::Deref for ConstrainedInteger<START, END> {
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<Integer>, const START: i128, const END: i128> From<T>
    for ConstrainedInteger<START, END>
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

macro_rules! asn_type {
    ($($name:ty: $value:ident),+) => {
        $(
            impl AsnType for $name {
                const TAG: Tag = Tag::$value;
            }
        )+
    }
}

asn_type! {
    bool: BOOL,
    Integer: INTEGER,
    OctetString: OCTET_STRING,
    ObjectIdentifier: OBJECT_IDENTIFIER,
    Oid: OBJECT_IDENTIFIER,
    Utf8String: UTF8_STRING,
    UtcTime: UTC_TIME,
    GeneralizedTime: GENERALIZED_TIME,
    (): NULL,
    &'_ str: UTF8_STRING

}

macro_rules! asn_integer_type {
    ($($int:ty),+ $(,)?) => {
        $(
            impl AsnType for $int {
                const TAG: Tag = Tag::INTEGER;
                const CONSTRAINTS: Constraints<'static> = Constraints::new(&[
                    constraints::Constraint::Value(Extensible::new(constraints::Value::new(constraints::Bounded::const_new(<$int>::MIN as i128, <$int>::MAX as i128)))),
                ]);
            }
        )+
    }
}

asn_integer_type! {
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

pub trait IntegerType:
    Sized
    + Clone
    + core::fmt::Debug
    + TryFrom<i64>
    + TryFrom<i128>
    + TryInto<i128>
    + Into<Integer>
    + num_traits::Num
    + num_traits::CheckedAdd
{
    const WIDTH: u32;

    fn try_from_bytes(input: &[u8], codec: crate::Codec)
        -> Result<Self, crate::error::DecodeError>;

    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    // `num_traits::WrappingAdd` is not implemented for `BigInt`
    #[doc(hidden)]
    fn wrapping_add(self, other: Self) -> Self;
}

macro_rules! integer_type_decode {
    ((signed $t1:ty, $t2:ty), $($ts:tt)*) => {
        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;

            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_signed_bytes(input, codec)
            }

            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = (<$t1>::BITS / 8) as usize;
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }

                // in the case of superfluous leading bytes (especially zeroes),
                // we may still want to try to decode the integer even though
                // the length is >BYTE_SIZE ...
                let leading_byte = if input[0] & 0x80 == 0x80 { 0xFF } else { 0x00 };
                let input_iter = input.iter().copied().skip_while(|n| *n == leading_byte);
                let data_length = input_iter.clone().count();

                // ... but if its still too large after skipping leading bytes,
                // there's no way to decode this without overflowing
                if data_length > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }

                let mut bytes = [leading_byte; BYTE_SIZE];
                let start = bytes.len() - data_length;

                for (b, d) in bytes[start..].iter_mut().zip(input_iter) {
                    *b = d;
                }

                Ok(Self::from_be_bytes(bytes))
            }

            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Ok(<$t2>::try_from_bytes(input, codec)? as $t1)
            }

            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }

        integer_type_decode!($($ts)*);
    };
    ((unsigned $t1:ty, $t2:ty), $($ts:tt)*) => {
        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;

            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_unsigned_bytes(input, codec)
            }

            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Ok(<$t2>::try_from_bytes(input, codec)? as $t1)
            }

            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = (<$t1>::BITS / 8) as usize;
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }

                let input_iter = input.iter().copied().skip_while(|n| *n == 0x00);
                let data_length = input_iter.clone().count();

                if data_length > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }

                let mut bytes = [0x00; BYTE_SIZE];
                let start = bytes.len() - data_length;

                for (b, d) in bytes[start..].iter_mut().zip(input_iter) {
                    *b = d;
                }


                Ok(Self::from_be_bytes(bytes))
            }

            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }

        integer_type_decode!($($ts)*);
    };
    (,) => {};
    () => {};
}

integer_type_decode!(
    (unsigned u8, i8),
    (signed i8, u8),
    (unsigned u16, i16),
    (signed i16, u16),
    (unsigned u32, i32),
    (signed i32, u32),
    (unsigned u64, i64),
    (signed i64, u64),
    (unsigned u128, i128),
    (signed i128, u128),
    (unsigned usize, isize),
    (signed isize, usize),
);

impl IntegerType for Integer {
    const WIDTH: u32 = u32::MAX;

    fn try_from_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }

        Ok(Integer::from_signed_bytes_be(input))
    }

    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        Self::try_from_bytes(input, codec)
    }

    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }

        Ok(BigUint::from_bytes_be(input).into())
    }

    fn wrapping_add(self, other: Self) -> Self {
        self + other
    }
}

impl AsnType for str {
    const TAG: Tag = Tag::UTF8_STRING;
}

impl<T: AsnType> AsnType for &'_ T {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
}

impl<T: AsnType> AsnType for Box<T> {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
}

impl<T: AsnType> AsnType for alloc::vec::Vec<T> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T: AsnType> AsnType for Option<T> {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
}

impl<T> AsnType for alloc::collections::BTreeSet<T> {
    const TAG: Tag = Tag::SET;
}

impl<T: AsnType, const N: usize> AsnType for [T; N] {
    const TAG: Tag = Tag::SEQUENCE;
    const CONSTRAINTS: Constraints<'static> =
        Constraints::new(&[Constraint::Size(Extensible::new(constraints::Size::new(
            constraints::Bounded::single_value(N),
        )))]);
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl AsnType for Any {
    const TAG: Tag = Tag::EOC;
    const TAG_TREE: TagTree = TagTree::Choice(&[]);
}
