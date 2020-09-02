mod oid;

use crate::tag::{self, Tag};

pub use alloc::string::String as Utf8String;
pub use bytes::Bytes as OctetString;
pub use num_bigint::BigInt as Integer;
pub use oid::ObjectIdentifier;

/// A reference to a `BIT STRING`.
pub type BitSlice = bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;
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
pub type SequenceOf<T> = alloc::vec::Vec<T>;
///  `UniversalString` string alias that matches BER's encoding rules.
pub type UniversalString = Implicit<tag::UNIVERSAL_STRING, Utf8String>;

pub use rasn_derive::AsnType;

/// A trait representing any type that can represented in ASN.1.
pub trait AsnType {
    /// The associated tag for the type.
    const TAG: Tag;
}

#[derive(AsnType, crate::Decode, crate::Encode)]
#[rasn(crate_root = "crate")]
#[rasn(choice)]
pub enum Open {
    BitString(BitString),
    IA5String(IA5String),
    PrintableString(PrintableString),
    VisibleString(VisibleString),
    BmpString(BmpString),
    NumericString(NumericString),
    SequenceOf(SequenceOf<Open>),
    UniversalString(UniversalString),
    Bool(bool),
    Integer(Integer),
    Null,
    OctetString(OctetString),
}

macro_rules! tag_kind {
    ($($name:ident),+) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name<T, V>{
                _tag: core::marker::PhantomData<T>,
                pub(crate) value: V,
            }

            impl<T, V> $name<T, V>{
                pub fn new(value: V) -> Self {
                    Self {
                        value,
                        _tag: core::marker::PhantomData,
                    }
                }
            }

            impl<T, V> From<V> for $name<T, V> {
                fn from(value: V) -> Self {
                    Self::new(value)
                }
            }

            impl<T, V> core::ops::Deref for $name<T, V> {
                type Target = V;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl<T, V> core::ops::DerefMut for $name<T, V> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }
        )+
    }
}

tag_kind!(Implicit, Explicit);

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
    Utf8String: UTF8_STRING,
    (): NULL,
    &'_ str: UTF8_STRING,
    &'_ BitSlice: BIT_STRING

}

impl<T> AsnType for SequenceOf<T> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T: AsnType, V> AsnType for Implicit<T, V> {
    const TAG: Tag = T::TAG;
}

impl<T: AsnType, V> AsnType for Explicit<T, V> {
    const TAG: Tag = T::TAG;
}
