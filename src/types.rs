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
    num_bigint::BigInt as Integer, rasn_derive::AsnType,
};

pub use self::{
    any::Any,
    constraints::Constraints,
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

pub trait IntegerType: Clone {}

impl IntegerType for Integer {}

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
    i8: INTEGER,
    i16: INTEGER,
    i32: INTEGER,
    i64: INTEGER,
    i128: INTEGER,
    isize: INTEGER,
    u8: INTEGER,
    u16: INTEGER,
    u32: INTEGER,
    u64: INTEGER,
    u128: INTEGER,
    usize: INTEGER,
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
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl AsnType for Any {
    const TAG: Tag = Tag::EOC;
    const TAG_TREE: TagTree = TagTree::Choice(&[]);
}
