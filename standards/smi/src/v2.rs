//! Version 2 (RFC 2578)

use alloc::string::ToString;
use core::convert::TryInto;

use chrono::TimeZone;

use rasn::{
    prelude::*,
    types::{ConstOid, Integer, ObjectIdentifier, OctetString, Oid, Utf8String},
};

use crate::v1::InvalidVariant;

#[doc(inline)]
pub use crate::v1::ToOpaque;

pub type ObjectName = crate::v1::ObjectName;
pub type NotificationName = ObjectIdentifier;
pub type IpAddress = crate::v1::IpAddress;
pub type Counter32 = crate::v1::Counter;
pub type Integer32 = u32;
pub type Gauge32 = Unsigned32;
pub type Unsigned32 = crate::v1::Gauge;
pub type TimeTicks = crate::v1::TimeTicks;
pub type Opaque = crate::v1::Opaque;

#[derive(AsnType, Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, tag(application, 6))]
pub struct Counter64(pub u64);

pub const ORG: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION;
pub const DOD: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD;
pub const INTERNET: ConstOid = crate::v1::INTERNET;
pub const DIRECTORY: ConstOid = crate::v1::DIRECTORY;
pub const MGMT: ConstOid = crate::v1::MGMT;
pub const EXPERIMENTAL: ConstOid = crate::v1::EXPERIMENTAL;
pub const PRIVATE: ConstOid = crate::v1::PRIVATE;
pub const ENTERPRISES: ConstOid = crate::v1::ENTERPRISES;
pub const SECURITY: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY;
pub const SNMP_V2: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2;
pub const SNMP_DOMAINS: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_DOMAINS;
pub const SNMP_PROXIES: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_PROXIES;
pub const SNMP_MODULES: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_MODULES;

const FULL_DATE_FORMAT: &str = "%Y%m%d%H%MZ";
const SHORT_DATE_FORMAT: &str = "%y%m%d%H%MZ";

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum ObjectSyntax {
    Simple(SimpleSyntax),
    ApplicationWide(ApplicationSyntax),
}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum SimpleSyntax {
    Integer(Integer),
    String(OctetString),
    ObjectId(ObjectIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum ApplicationSyntax {
    Address(IpAddress),
    Counter(Counter32),
    Ticks(TimeTicks),
    Arbitrary(Opaque),
    BigCounter(Counter64),
    Unsigned(Unsigned32),
}

#[derive(AsnType, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(universal, 4))]
pub struct ExtUtcTime(pub chrono::DateTime<chrono::Utc>);

impl Encode for ExtUtcTime {
    fn encode_with_tag_and_constraints<EN: Encoder>(&self, encoder: &mut EN, tag: Tag, _: Constraints) -> Result<(), EN::Error> {
        encoder
            .encode_octet_string(
                tag,
                <_>::from(&[constraints::Size::new(constraints::Range::single_value(13)).into()]),
                self.0.format(FULL_DATE_FORMAT).to_string().as_bytes(),
            )
            .map(drop)
    }
}

impl Decode for ExtUtcTime {
    fn decode_with_tag_and_constraints<D: Decoder>(decoder: &mut D, tag: Tag, _: Constraints) -> Result<Self, D::Error> {
        let bytes = OctetString::decode_with_tag(decoder, tag)?;
        let len = bytes.len();
        let error = || Err(rasn::de::Error::custom("Invalid `ExtUtcTime` encoding"));

        // YYMMDDHHMMZ || YYYYMMDDHHMMZ
        if len != 11 || len != 13 {
            return (error)();
        }

        let string = Utf8String::from_utf8_lossy(&bytes);

        if let Ok(time) = chrono::Utc.datetime_from_str(&string, FULL_DATE_FORMAT) {
            Ok(Self(time))
        } else if let Ok(time) = chrono::Utc.datetime_from_str(&string, SHORT_DATE_FORMAT) {
            Ok(Self(time))
        } else {
            (error)()
        }
    }
}

macro_rules! try_from_impls_v2 {
    ($(impl $name:ty => $pattern:pat => $return:expr);+ $(;)?) => {
        $(
            impl core::convert::TryFrom<$crate::v2::ObjectSyntax> for $name {
                type Error = $crate::v1::InvalidVariant;
                fn try_from(value: $crate::v2::ObjectSyntax) -> Result<Self, Self::Error> {
                    Ok(match value {
                        $pattern => $return,
                        _ => return Err($crate::v1::InvalidVariant),
                    })
                }
            }
        )+
    }
}

from_impls! {
    // -> ObjectSyntax
    impl SimpleSyntax => ObjectSyntax: (value) -> ObjectSyntax::Simple(value);
    impl ApplicationSyntax => ObjectSyntax: (value) -> ObjectSyntax::ApplicationWide(value);
    impl Integer => ObjectSyntax: (value) -> SimpleSyntax::Integer(value).into();
    impl u8 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl u16 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl u32 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl u64 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl u128 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl i8 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl i16 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl i32 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl i64 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl i128 => ObjectSyntax: (value) -> SimpleSyntax::Integer(value.into()).into();
    impl OctetString => ObjectSyntax:  (value) -> SimpleSyntax::String(value).into();
    impl ObjectIdentifier => ObjectSyntax:  (value) -> SimpleSyntax::ObjectId(value).into();
    impl Counter32 => ObjectSyntax:  (value) -> ApplicationSyntax::Counter(value).into();
    impl Counter64 => ObjectSyntax:  (value) -> ApplicationSyntax::BigCounter(value).into();
    impl Unsigned32 => ObjectSyntax:  (value) -> ApplicationSyntax::Unsigned(value).into();
    impl TimeTicks => ObjectSyntax: (value) -> ApplicationSyntax::Ticks(value).into();
    impl Opaque => ObjectSyntax: (value) -> ApplicationSyntax::Arbitrary(value).into();
    impl IpAddress => ObjectSyntax: (value) -> ApplicationSyntax::Address(value).into();
    impl crate::v1::NetworkAddress => ObjectSyntax: (crate::v1::NetworkAddress::Internet(value)) -> ApplicationSyntax::Address(value).into();
    // -> SimpleSyntax
    impl Integer => SimpleSyntax: (value) -> SimpleSyntax::Integer(value);
    impl u8 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl u16 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl u32 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl u64 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl u128 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl i8 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl i16 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl i32 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl i64 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl i128 => SimpleSyntax: (value) -> SimpleSyntax::Integer(value.into());
    impl OctetString => SimpleSyntax:  (value) -> SimpleSyntax::String(value);
    impl ObjectIdentifier => SimpleSyntax:  (value) -> SimpleSyntax::ObjectId(value);
    // -> ApplicationSyntax
    impl Counter32 => ApplicationSyntax:  (value) -> ApplicationSyntax::Counter(value);
    impl Counter64 => ApplicationSyntax:  (value) -> ApplicationSyntax::BigCounter(value);
    impl Unsigned32 => ApplicationSyntax:  (value) -> ApplicationSyntax::Unsigned(value);
    impl TimeTicks => ApplicationSyntax: (value) -> ApplicationSyntax::Ticks(value);
    impl Opaque => ApplicationSyntax: (value) -> ApplicationSyntax::Arbitrary(value);
    impl IpAddress => ApplicationSyntax: (value) -> ApplicationSyntax::Address(value);
    impl crate::v1::NetworkAddress => ApplicationSyntax: (crate::v1::NetworkAddress::Internet(value)) -> ApplicationSyntax::Address(value);
}

try_from_impls_v2! {
    impl OctetString => ObjectSyntax::Simple(SimpleSyntax::String(value)) => value;
    impl rasn::types::Integer => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value;
    impl u8 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u16 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u32 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u64 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl u128 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i8 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i16 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i32 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i64 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl i128 => ObjectSyntax::Simple(SimpleSyntax::Integer(value)) => value.try_into().map_err(|_| InvalidVariant)?;
    impl ObjectIdentifier => ObjectSyntax::Simple(SimpleSyntax::ObjectId(value)) => value;
    impl Counter32 => ObjectSyntax::ApplicationWide(ApplicationSyntax::Counter(value)) => value;
    impl Counter64 => ObjectSyntax::ApplicationWide(ApplicationSyntax::BigCounter(value)) => value;
    impl Unsigned32 => ObjectSyntax::ApplicationWide(ApplicationSyntax::Unsigned(value)) => value;
    impl TimeTicks => ObjectSyntax::ApplicationWide(ApplicationSyntax::Ticks(value)) => value;
    impl Opaque => ObjectSyntax::ApplicationWide(ApplicationSyntax::Arbitrary(value)) => value;
    impl IpAddress => ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(value)) => value;
    impl crate::v1::NetworkAddress => ObjectSyntax::ApplicationWide(ApplicationSyntax::Address(value)) => crate::v1::NetworkAddress::Internet(value);
}
