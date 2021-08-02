//! Version 1 (RFC 1155)

use core::convert::TryInto;

use rasn::{
    types::{ConstOid, Integer, ObjectIdentifier, OctetString, Oid},
    AsnType, Decode, Encode, Tag,
};

pub const INTERNET: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET;
pub const DIRECTORY: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY;
pub const MGMT: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT;
pub const EXPERIMENTAL: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_EXPERIMENTAL;
pub const PRIVATE: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE;
pub const ENTERPRISES: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES;

pub type ObjectName = ObjectIdentifier;

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ObjectSyntax {
    Simple(SimpleSyntax),
    ApplicationWide(ApplicationSyntax),
}

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum SimpleSyntax {
    Number(Integer),
    String(OctetString),
    Object(ObjectIdentifier),
    Empty,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
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

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[rasn(delegate, tag(application, 0))]
pub struct IpAddress(pub OctetString);

#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[rasn(delegate, tag(application, 1))]
pub struct Counter(pub u32);

#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[rasn(delegate, tag(application, 2))]
pub struct Gauge(pub u32);

#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
#[rasn(delegate, tag(application, 3))]
pub struct TimeTicks(pub u32);

/// A wrapper around arbitrary ASN.1 syntax, encoded in Basic Encoding Rules,
/// this type is not typically useful on its own, and should decoded into its
/// inner type when possible.
#[derive(AsnType, Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
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

impl AsRef<[u8]> for Opaque {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Decode for Opaque {
    fn decode_with_tag<D: rasn::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
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

/// The message was valid SMI but was not the variant we expected.
pub struct InvalidVariant;

impl core::fmt::Display for InvalidVariant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Message was valid SMI but was not the variant we expected")
    }
}

from_impls! {
    // -> ObjectSyntax
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
    // -> SimpleSyntax
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
    // -> ApplicationSyntax
    impl NetworkAddress => ApplicationSyntax:  (value) -> ApplicationSyntax::Address(value);
    impl Counter => ApplicationSyntax:  (value) -> ApplicationSyntax::Counter(value);
    impl Gauge => ApplicationSyntax:  (value) -> ApplicationSyntax::Gauge(value);
    impl TimeTicks => ApplicationSyntax: (value) -> ApplicationSyntax::Ticks(value);
    impl Opaque => ApplicationSyntax: (value) -> ApplicationSyntax::Arbitrary(value);
    impl IpAddress => ApplicationSyntax: (value) -> ApplicationSyntax::Address(NetworkAddress::Internet(value));
}

macro_rules! try_from_impls_v1 {
    ($(impl $name:ty => $pattern:pat => $return:expr);+ $(;)?) => {
        $(
            impl core::convert::TryFrom<$crate::v1::ObjectSyntax> for $name {
                type Error = $crate::v1::InvalidVariant;
                fn try_from(value: $crate::v1::ObjectSyntax) -> Result<Self, Self::Error> {
                    Ok(match value {
                        $pattern => $return,
                        _ => return Err($crate::v1::InvalidVariant),
                    })
                }
            }
        )+
    }
}

try_from_impls_v1! {
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
