//! # ASN.1 Data Types
//! The `types` modules is a collection of Rust types and data structures that
//! are defined to represent various ASN.1 data types, and renamed to use
//! ASN.1's terminology.

mod any;
mod identifier;
mod instance;
mod open;
mod prefix;
mod tag;

pub mod constraints;
pub mod fields;
pub mod variants;

pub(crate) mod constructed;
pub(crate) mod date;
pub(crate) mod integer;
pub(crate) mod oid;

pub(crate) mod real;

pub(crate) mod strings;

use crate::macros::{constraints, size_constraint, value_constraint};
use alloc::boxed::Box;

pub use {
    self::{
        any::Any,
        constraints::{Constraint, Constraints, Extensible},
        constructed::{Constructed, SequenceOf, SetOf},
        identifier::Identifier,
        instance::InstanceOf,
        integer::{ConstrainedInteger, Integer, IntegerType},
        oid::{ObjectIdentifier, Oid},
        open::Open,
        prefix::{Explicit, Implicit},
        strings::{
            BitStr, BitString, BmpString, FixedBitString, FixedOctetString, GeneralString,
            GraphicString, Ia5String, NumericString, OctetString, PrintableString, TeletexString,
            Utf8String, VisibleString,
        },
        tag::{Class, Tag, TagTree},
    },
    rasn_derive::AsnType,
};

pub use self::real::RealType;

///  The `UniversalString` type.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;
///  The `UTCTime` type.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  The `GeneralizedTime` type.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;
/// The `Date` type.
pub type Date = chrono::NaiveDate;

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

    /// The set of constraints for values of the given type.
    const CONSTRAINTS: Constraints = Constraints::NONE;

    /// Identifier of an ASN.1 type as specified in the original specification
    /// if not identical with the identifier of `Self`
    const IDENTIFIER: Identifier = Identifier::EMPTY;

    /// Whether the type is present with value. `OPTIONAL` fields are common in `SEQUENCE` or `SET`.
    ///
    /// Custom implementation is only used for `OPTIONAL` type.
    fn is_present(&self) -> bool {
        true
    }
}

/// A `CHOICE` value.
pub trait Choice: Sized {
    /// Variants contained in the "root component list".
    const VARIANTS: &'static [TagTree];
    /// Constraint for the choice type, based on the number of root components. Used for PER encoding.
    const VARIANCE_CONSTRAINT: Constraints;
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
pub trait Enumerated: Sized + 'static + PartialEq + Copy + core::fmt::Debug + AsnType {
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
        Self::EXTENDED_VARIANTS.is_some_and(|array| array.iter().any(|variant| variant == self))
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

macro_rules! asn_type {
    ($($name:ty: $value:ident),+) => {
        $(
            impl AsnType for $name {
                const TAG: Tag = Tag::$value;
                const IDENTIFIER: Identifier = Identifier::$value;
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
                const IDENTIFIER: Identifier = Identifier::INTEGER;
                const CONSTRAINTS: Constraints = constraints!(value_constraint!((<$int>::MIN as i128), (<$int>::MAX as i128)));
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
    u128, // TODO upper constraint truncated
    usize,
}
impl AsnType for num_bigint::BigInt {
    const TAG: Tag = Tag::INTEGER;
    const IDENTIFIER: Identifier = Identifier::INTEGER;
}

impl AsnType for str {
    const TAG: Tag = Tag::UTF8_STRING;
    const IDENTIFIER: Identifier = Identifier::UTF8_STRING;
}

impl<T: AsnType> AsnType for &'_ T {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
    const IDENTIFIER: Identifier = T::IDENTIFIER;

    fn is_present(&self) -> bool {
        (*self).is_present()
    }
}

impl<T: AsnType> AsnType for Box<T> {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
    const IDENTIFIER: Identifier = T::IDENTIFIER;
}

impl<T: AsnType> AsnType for alloc::vec::Vec<T> {
    const TAG: Tag = Tag::SEQUENCE;
    const IDENTIFIER: Identifier = Identifier::SEQUENCE_OF;
}

impl<T: AsnType> AsnType for Option<T> {
    const TAG: Tag = T::TAG;
    const TAG_TREE: TagTree = T::TAG_TREE;
    const IDENTIFIER: Identifier = T::IDENTIFIER;

    fn is_present(&self) -> bool {
        self.is_some()
    }
}

impl<T> AsnType for SetOf<T> {
    const TAG: Tag = Tag::SET;
    const IDENTIFIER: Identifier = Identifier::SET_OF;
}

impl<T: AsnType, const N: usize> AsnType for [T; N] {
    const TAG: Tag = Tag::SEQUENCE;
    const CONSTRAINTS: Constraints = constraints!(size_constraint!(N));
    const IDENTIFIER: Identifier = Identifier::SEQUENCE_OF;
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
    const IDENTIFIER: Identifier = Identifier::SEQUENCE_OF;
}

impl AsnType for Any {
    const TAG: Tag = Tag::EOC;
    const TAG_TREE: TagTree = TagTree::Choice(&[]);
}

#[cfg(feature = "f32")]
impl AsnType for f32 {
    const TAG: Tag = Tag::REAL;
    const IDENTIFIER: Identifier = Identifier::REAL;
}

#[cfg(feature = "f64")]
impl AsnType for f64 {
    const TAG: Tag = Tag::REAL;
    const IDENTIFIER: Identifier = Identifier::REAL;
}

impl<T> AsnType for core::marker::PhantomData<T> {
    const TAG: Tag = Tag::NULL;
    const TAG_TREE: TagTree = TagTree::Leaf(Tag::NULL);
}
