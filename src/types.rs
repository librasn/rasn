//! # Types
//! The `types` modules is a collection of Rust types and data structures that
//! are defined to represent various ASN.1 data types, and renamed to use
//! ASN.1's terminology.

mod any;
mod instance;
mod open;
mod prefix;
mod tag;
pub(crate) mod oid;

use alloc::boxed::Box;

pub use ::{
    alloc::string::String as Utf8String, bytes::Bytes as OctetString,
    num_bigint::BigInt as Integer, rasn_derive::AsnType,
};

pub use self::{
    any::Any,
    instance::InstanceOf,
    oid::{ConstOid, ObjectIdentifier, Oid},
    open::Open,
    prefix::{Explicit, Implicit},
    tag::{Class, Tag, TagTree},
};

///  Alias for `bitvec::BitVec` mapped to ASN.1'a `BIT STRING`.
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
///  `Ia5String` string alias that matches BER's encoding rules.
pub type Ia5String = Implicit<tag::IA5_STRING, Utf8String>;
///  `PrintableString` string alias that matches BER's encoding rules.
pub type PrintableString = Implicit<tag::PRINTABLE_STRING, Utf8String>;
///  `VisibleString` string alias that matches BER's encoding rules.
pub type VisibleString = Implicit<tag::VISIBLE_STRING, Utf8String>;
///  `String` alias that matches `BmpString` BER's encoding rules.
pub type BmpString = Implicit<tag::BMP_STRING, Utf8String>;
///  `String` alias that matches `TeletexString` BER's encoding rules.
pub type TeletexString = Implicit<tag::TELETEX_STRING, Utf8String>;
///  Alias to `alloc::collections::BTreeSet<T>`.
pub type SetOf<T> = alloc::collections::BTreeSet<T>;
///  `UniversalString` string alias that matches BER's encoding rules.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;
///  Alias for `chrono::DateTime<Utc>`.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  Alias for `chrono::DateTime<FixedOffset>`.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;

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
