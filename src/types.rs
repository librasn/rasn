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

pub use {
    self::{
        any::Any,
        constraints::{Constraint, Constraints, Extensible},
        instance::InstanceOf,
        oid::{ConstOid, ObjectIdentifier, Oid},
        open::Open,
        prefix::{Explicit, Implicit},
        strings::{
            BmpString, Ia5String, NumericString, PrintableString, TeletexString, Utf8String,
            VisibleString, GeneralString,
        },
        tag::{Class, Tag, TagTree},
    },
    bytes::Bytes as OctetString,
    num_bigint::BigInt as Integer,
    rasn_derive::AsnType,
};

///  The `BIT STRING` type.
pub type BitString = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
///  The `BIT STRING` type.
pub type BitStr = bitvec::slice::BitSlice<u8, bitvec::order::Msb0>;
///  The `SET OF` type.
pub type SetOf<T> = alloc::collections::BTreeSet<T>;
///  The `UniversalString` type.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;
///  The `UTCTime` type.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  The `GeneralizedTime` type.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;
///  The `SEQUENCE OF` type.
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
}

/// A `SET` or `SEQUENCE` value.
pub trait Constructed {
    const FIELDS: self::fields::Fields;
    const EXTENDED_FIELDS: Option<self::fields::Fields> = None;
}

/// A `CHOICE` value.
pub trait Choice {
    const VARIANTS: &'static [TagTree];
    const EXTENDED_VARIANTS: &'static [TagTree] = &[];
}

/// A `ENUMERATED` value.
pub trait Enumerated: Sized + 'static + PartialEq + Copy {
    const VARIANTS: &'static [Self];
    const EXTENDED_VARIANTS: Option<&'static [Self]>;

    const DISCRIMINANTS: &'static [(Self, isize)];
    const EXTENDED_DISCRIMINANTS: Option<&'static [(Self, isize)]>;

    fn variance() -> usize {
        Self::VARIANTS.len()
    }

    fn extended_variance() -> usize {
        Self::EXTENDED_VARIANTS.map_or(0, |array| array.len())
    }

    fn complete_variance() -> usize {
        Self::variance() + Self::extended_variance()
    }

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

    fn from_enumeration_index(index: usize) -> Option<Self> {
        Self::VARIANTS.get(index).copied()
    }

    fn from_extended_enumeration_index(index: usize) -> Option<Self> {
        Self::EXTENDED_VARIANTS.and_then(|array| array.get(index).copied())
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
    ConstOid: OBJECT_IDENTIFIER,
    BitString: BIT_STRING,
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
