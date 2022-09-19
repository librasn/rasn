//! # ASN.1 Data Types
//! The `types` modules is a collection of Rust types and data structures that
//! are defined to represent various ASN.1 data types, and renamed to use
//! ASN.1's terminology.

mod any;
pub mod constraints;
mod instance;
pub(crate) mod oid;
mod open;
mod prefix;
mod strings;
mod tag;

use alloc::boxed::Box;

pub use ::{
    alloc::string::String as Utf8String, bytes::Bytes as OctetString,
    rasn_derive::AsnType, num_bigint::BigInt as Integer,
};

pub use self::{
    any::Any,
    constraints::{Constraints, Constraint},
    instance::InstanceOf,
    oid::{ConstOid, ObjectIdentifier, Oid},
    open::Open,
    prefix::{Explicit, Implicit},
    strings::VisibleString,
    tag::{Class, Tag, TagTree},
};

///  The `BIT STRING` type.
pub type BitString = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
///  The `BIT STRING` type.
pub type BitStr = bitvec::slice::BitSlice<u8, bitvec::order::Msb0>;
///  The `Ia5String` type.
pub type Ia5String = Implicit<tag::IA5_STRING, Utf8String>;
///  The `GeneralString` type.
pub type GeneralString = Implicit<tag::GENERAL_STRING, Utf8String>;
///  The `PrintableString` type.
pub type PrintableString = Implicit<tag::PRINTABLE_STRING, Utf8String>;
///  The `BmpString` type.
pub type BmpString = Implicit<tag::BMP_STRING, Utf8String>;
///  The `TeletexString` type.
pub type TeletexString = Implicit<tag::TELETEX_STRING, OctetString>;
///  The `NumericString` type.
pub type NumericString = Implicit<tag::NUMERIC_STRING, Utf8String>;
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

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub tag: Tag,
    pub presence: FieldPresence,
}

impl Field {
    pub const fn new_required(tag: Tag) -> Self {
        Self {
            tag,
            presence: FieldPresence::Required,
        }
    }

    pub const fn new_optional(tag: Tag) -> Self {
        Self {
            tag,
            presence: FieldPresence::Optional,
        }
    }

    pub const fn new_default(tag: Tag) -> Self {
        Self {
            tag,
            presence: FieldPresence::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FieldPresence {
    Required,
    Optional,
    #[default]
    Default,
}

impl FieldPresence {
    pub fn is_optional_or_default(&self) -> bool {
        matches!(self, Self::Optional | Self::Default)
    }
}

pub trait Constructed {
    const FIELDS: &'static [Field];
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
    num_bigint::BigInt: INTEGER,
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
                    constraints::Constraint::Value(constraints::Value::new(constraints::Range::const_new(<$int>::MIN as i128, <$int>::MAX as i128))),
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
    const CONSTRAINTS: Constraints<'static> = Constraints::new(&[
        Constraint::Size(constraints::Size::new(constraints::Range::single_value(N)))
    ]);
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl AsnType for Any {
    const TAG: Tag = Tag::EOC;
    const TAG_TREE: TagTree = TagTree::Choice(&[]);
}
