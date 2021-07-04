#![no_std]

extern crate alloc;

use core::convert::TryInto;

use rasn::{
    types::{ConstOid, Implicit, Integer, ObjectIdentifier, OctetString, Oid},
    AsnType, Decode, Encode, Tag,
};

use self::consts::*;

#[doc(hidden)]
pub use rasn;

pub type ObjectName = ObjectIdentifier;
pub type IpAddress = Implicit<A0, [u8; 4]>;
pub type Counter = Implicit<A1, u32>;
pub type Gauge = Implicit<A2, u32>;
pub type TimeTicks = Implicit<A3, u32>;

pub const INTERNET: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET;
pub const DIRECTORY: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY;
pub const MGMT: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT;
pub const EXPERIMENTAL: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_EXPERIMENTAL;
pub const PRIVATE: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE;
pub const ENTERPRISES: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES;

/// A wrapper around arbitrary ASN.1 syntax, encoded in Basic Enncoding Rules,
/// this type is not typically useful on its own, and should decoded into its
/// inner type when possible.
#[derive(Debug, Clone, AsnType)]
#[rasn(tag(application, 4))]
pub struct Opaque(alloc::vec::Vec<u8>);


/// Helper trait for wrapping any valid ASN.1 type in an `Opaque` struct which
/// first encodes the data into Basic Encoding Rules.
pub trait IntoOpaque {
    fn into_opaque(&self) -> Result<Opaque, rasn::ber::enc::Error>;
}

impl<T: Encode> IntoOpaque for T {
    fn into_opaque(&self) -> Result<Opaque, rasn::ber::enc::Error> {
        rasn::ber::encode(self).map(Opaque)
    }
}

macro_rules! try_from_impls {
    ($(impl $name:ty => $pattern:pat => $return:expr);+ $(;)?) => {
        $(
            impl core::convert::TryFrom<$crate::ObjectSyntax> for $name {
                type Error = $crate::InvalidVariant;
                fn try_from(value: $crate::ObjectSyntax) -> Result<Self, Self::Error> {
                    Ok(match value {
                        $pattern => $return,
                        _ => return Err(InvalidVariant),
                    })
                }
            }
        )+
    }
}

try_from_impls! {
    impl OctetString => ObjectSyntax::Simple(SimpleSyntax::String(value)) => value;
    impl Integer => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value;
    impl u8 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u16 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u32 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u64 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u128 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i8 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i16 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i32 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i64 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i128 => ObjectSyntax::Simple(SimpleSyntax::Number(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl ObjectIdentifier => ObjectSyntax::Simple(SimpleSyntax::Object(value)) => value;
    impl () => ObjectSyntax::Simple(SimpleSyntax::Empty) => ();
    impl NetworkAddress => ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(value)) => value;
    impl Counter => ObjectSyntax::ApplicationWide(ApplicationSyntax::Counter(value)) => value;
    impl Gauge => ObjectSyntax::ApplicationWide(ApplicationSyntax::Gauge(value)) => value;
    impl TimeTicks => ObjectSyntax::ApplicationWide(ApplicationSyntax::Ticks(value)) => value;
    impl Opaque => ObjectSyntax::ApplicationWide(ApplicationSyntax::Arbitrary(value)) => value;
    impl IpAddress => ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(NetworkAddress::Internet(value))) => value;
}


impl AsRef<[u8]> for Opaque {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Decode for Opaque {
    fn decode_with_tag<D: rasn::Decoder>(
        decoder: &mut D,
        tag: Tag,
    ) -> Result<Self, D::Error> {
        decoder.decode_octet_string(tag).map(Self)
    }
}

impl Encode for Opaque {
    fn encode_with_tag<EN: rasn::Encoder>(
        &self,
        encoder: &mut EN,
        tag: Tag,
    ) -> Result<(), EN::Error> {
        encoder.encode_octet_string(tag, &self.0).map(drop)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! common_impls {
    ($name:ident, $network_type:ty, $access_variant:ident, $status_variant:ident, $const_oid:expr) => {
        impl $crate::rasn::AsnType for $name {
            const TAG: Tag = <$network_type as $crate::rasn::AsnType>::TAG;
        }

        impl $crate::ObjectType for $name {
            type Syntax = $network_type;
            const ACCESS: $crate::Access = $crate::Access::$access_variant;
            const STATUS: $crate::Status = $crate::Status::$status_variant;
            const VALUE: $crate::rasn::types::ConstOid = $crate::rasn::types::ConstOid(&$const_oid);
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! delegate_impls {
    ($name:ident, $rust_type:ty) => {
        impl From<$rust_type> for $name {
            fn from(value: $rust_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $rust_type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl core::ops::Deref for $name {
            type Target = $rust_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }


    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! opaque_impls {
    ($name:ident) => {
        impl core::convert::TryFrom<$crate::Opaque> for $name {
            type Error = $crate::rasn::ber::de::Error;

            fn try_from(value: $crate::Opaque) -> Result<Self, Self::Error> {
                $crate::rasn::ber::decode(value.as_ref())
            }
        }

        impl core::convert::TryFrom<$name> for $crate::Opaque {
            type Error = $crate::rasn::ber::enc::Error;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                use $crate::IntoOpaque;
                value.into_opaque()
            }
        }

        impl $crate::rasn::Decode for $name {
            fn decode_with_tag<D: $crate::rasn::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                $crate::Opaque::decode_with_tag(decoder, tag)
                    .and_then(|opaque| $crate::rasn::ber::decode(opaque.as_ref()).map_err($crate::rasn::de::Error::custom))
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag<EN: $crate::rasn::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
                self.clone().into_object_syntax().map_err($crate::rasn::enc::Error::custom)?.encode_with_tag(encoder, tag)
            }
        }

    }
}

#[macro_export]
macro_rules! object_type {
    (
        $(#[$($tree:tt)+])*
        $vi: vis struct $name: ident { $($fields:tt)* }
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        pub struct $name { $($fields)+ }

        $crate::common_impls!($name, $crate::Opaque, $access_variant, $status_variant, $const_oid);
        $crate::opaque_impls!($name);

        $crate::object_type! {
            $($rest)*
        }
    };

    (
        $(#[$($tree:tt)+])*
        $vi: vis struct $name: ident ($fv: vis $typ: ty);
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        pub struct $name($fv $typ);

        $crate::common_impls!($name, $typ, $access_variant, $status_variant,  $const_oid);
        $crate::delegate_impls!($name, $typ);

        impl $crate::rasn::Decode for $name {
            fn decode_with_tag<D: $crate::rasn::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                use core::convert::TryFrom;

                let syntax = $crate::ObjectSyntax::decode_with_tag(decoder, tag)?;

                <$typ>::try_from(syntax).map_err($crate::rasn::de::Error::custom).map(Self)
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag<EN: $crate::rasn::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
                self.clone().into_object_syntax().map_err($crate::rasn::enc::Error::custom)?.encode_with_tag(encoder, tag)
            }
        }

        $crate::object_type! {
            $($rest)*
        }
    };
    (
        $(#[$($tree:tt)+])*
        $vi: vis opaque struct $name: ident ($fv: vis $typ: ty);
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        pub struct $name($fv $typ);

        $crate::common_impls!($name, $crate::Opaque, $access_variant, $status_variant, $const_oid);
        $crate::delegate_impls!($name, $typ);
        $crate::opaque_impls!($name);

        $crate::object_type! {
            $($rest)*
        }
    };

    (
        $(#[$($tree:tt)+])*
        impl $name:ident {
            $($block:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        impl $name {
            $($block)*
        }

        $crate::object_type! {
            $($rest)*
        }
    };

    () => {};
}

/// A managed Management Information Base (MIB) object.
pub trait ObjectType where Self: TryInto<Self::Syntax>, <Self as TryInto<Self::Syntax>>::Error: rasn::enc::Error + core::fmt::Display {
    /// The abstract syntax for the object type. This must resolve to an
    /// instance of the ASN.1 type [`ObjectSyntax`].
    type Syntax: Into<ObjectSyntax>;
    /// Determines the access level of the object.
    const ACCESS: Access;
    /// The current status of the object.
    const STATUS: Status;
    /// The object identifier for the object.
    const VALUE: rasn::types::ConstOid;

    fn into_object_syntax(self) -> Result<ObjectSyntax, <Self as TryInto<Self::Syntax>>::Error>  {
        Ok(self.try_into().map_err(rasn::enc::Error::custom)?.into())
    }
}

/// The message was valid SMI but was not the variant we expected.
pub struct InvalidVariant;

impl core::fmt::Display for InvalidVariant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Message was valid SMI but was not the variant we expected")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    NotAccessible,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Status {
    Mandatory,
    Optional,
    Obsolete,
}

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ObjectSyntax {
    Simple(SimpleSyntax),
    ApplicationWide(ApplicationSyntax),
}

macro_rules! from_impls {
    ($(impl $({$($generics:tt)+})? $from:ty => $typ:ty: ($value: ident) -> $variant:expr);+ $(;)?)  => {
        $(
            impl $(<$($generics)+>)? From<$from> for $typ {
                fn from($value: $from) -> Self {
                    $variant
                }
            }
        )+
    }
}

// ObjectSyntax from impls
from_impls! {
    impl SimpleSyntax => ObjectSyntax: (value) -> ObjectSyntax::Simple(value);
    impl ApplicationSyntax => ObjectSyntax: (value) -> ObjectSyntax::ApplicationWide(value);
    impl Integer => ObjectSyntax: (value) -> SimpleSyntax::Number(value).into();
    impl u8 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl u16 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl u32 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl u64 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl u128 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl i8 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl i16 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl i32 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl i64 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl i128 => ObjectSyntax: (value) -> SimpleSyntax::Number(value.into()).into();
    impl OctetString => ObjectSyntax:  (value) -> SimpleSyntax::String(value).into();
    impl ObjectIdentifier => ObjectSyntax:  (value) -> SimpleSyntax::Object(value).into();
    impl NetworkAddress => ObjectSyntax:  (value) -> ApplicationSyntax::Address(value).into();
    impl Counter => ObjectSyntax:  (value) -> ApplicationSyntax::Counter(value).into();
    impl Gauge => ObjectSyntax:  (value) -> ApplicationSyntax::Gauge(value).into();
    impl TimeTicks => ObjectSyntax: (value) -> ApplicationSyntax::Ticks(value).into();
    impl Opaque => ObjectSyntax: (value) -> ApplicationSyntax::Arbitrary(value).into();
    impl IpAddress => ObjectSyntax: (value) -> ApplicationSyntax::Address(NetworkAddress::Internet(value)).into();
}

// Simple and Application syntax from impls
from_impls! {
    impl Integer => SimpleSyntax: (value) -> SimpleSyntax::Number(value);
    impl u8 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl u16 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl u32 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl u64 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl u128 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl i8 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl i16 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl i32 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl i64 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl i128 => SimpleSyntax: (value) -> SimpleSyntax::Number(value.into());
    impl OctetString => SimpleSyntax:  (value) -> SimpleSyntax::String(value);
    impl ObjectIdentifier => SimpleSyntax:  (value) -> SimpleSyntax::Object(value);
    impl NetworkAddress => ApplicationSyntax:  (value) -> ApplicationSyntax::Address(value);
    impl Counter => ApplicationSyntax:  (value) -> ApplicationSyntax::Counter(value);
    impl Gauge => ApplicationSyntax:  (value) -> ApplicationSyntax::Gauge(value);
    impl TimeTicks => ApplicationSyntax: (value) -> ApplicationSyntax::Ticks(value);
    impl Opaque => ApplicationSyntax: (value) -> ApplicationSyntax::Arbitrary(value);
    impl IpAddress => ApplicationSyntax: (value) -> ApplicationSyntax::Address(NetworkAddress::Internet(value));
}

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum SimpleSyntax {
    Number(Integer),
    String(OctetString),
    Object(ObjectIdentifier),
    Empty,
}

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ApplicationSyntax {
    Address(NetworkAddress),
    Counter(Counter),
    Gauge(Gauge),
    Ticks(TimeTicks),
    Arbitrary(Opaque),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum NetworkAddress {
    Internet(IpAddress),
}

mod consts {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType)]
    #[rasn(tag(application, 0))]
    pub struct A0;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType)]
    #[rasn(tag(application, 1))]
    pub struct A1;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType)]
    #[rasn(tag(application, 2))]
    pub struct A2;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType)]
    #[rasn(tag(application, 3))]
    pub struct A3;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, AsnType)]
    #[rasn(tag(application, 4))]
    pub struct A4;
}
