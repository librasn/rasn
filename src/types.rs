mod oid;

use crate::tag::{self, Implicit, Tag};

pub use alloc::string::String as Utf8String;
pub use bytes::Bytes as OctetString;
pub use num_bigint::BigInt as Integer;
pub use oid::ObjectIdentifier;

///  Alias for `bitvec::BitVec` mapped to ASN.1'a `BIT STRING`.
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
///  `IA5String` string alias that matches BER's encoding rules.
pub type IA5String = Implicit<tag::IA5_STRING, Utf8String>;
///  `PrintableString` string alias that matches BER's encoding rules.
pub type PrintableString = Implicit<tag::PRINTABLE_STRING, Utf8String>;
///  `VisibleString` string alias that matches BER's encoding rules.
pub type VisibleString = Implicit<tag::VISIBLE_STRING, Utf8String>;
///  `BmpString` string alias that matches BER's encoding rules.
pub type BmpString = Implicit<tag::BMP_STRING, Utf8String>;
///  `NumericString` string alias that matches BER's encoding rules.
pub type NumericString = Implicit<tag::NUMERIC_STRING, Utf8String>;
///  `UniversalString` string alias that matches BER's encoding rules.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;

/// A trait representing any type that can represented in ASN.1.
pub trait AsnType {
    /// The associated tag for the type.
    const TAG: Tag;
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
    BitString: BIT_STRING,
    Utf8String: UTF8_STRING
}

impl<T> AsnType for alloc::vec::Vec<T> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T: AsnType, V> AsnType for crate::tag::Implicit<T, V> {
    const TAG: Tag = T::TAG;
}

impl<T: AsnType, V> AsnType for crate::tag::Explicit<T, V> {
    const TAG: Tag = T::TAG;
}
