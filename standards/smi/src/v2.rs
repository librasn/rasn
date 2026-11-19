//! Version 2 (RFC 2578)

use alloc::format;

use rasn::macros::*;
use rasn::prelude::*;

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

pub const ORG: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION;
pub const DOD: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD;
pub const INTERNET: &Oid = crate::v1::INTERNET;
pub const DIRECTORY: &Oid = crate::v1::DIRECTORY;
pub const MGMT: &Oid = crate::v1::MGMT;
pub const EXPERIMENTAL: &Oid = crate::v1::EXPERIMENTAL;
pub const PRIVATE: &Oid = crate::v1::PRIVATE;
pub const ENTERPRISES: &Oid = crate::v1::ENTERPRISES;
pub const SECURITY: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY;
pub const SNMP_V2: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2;
pub const SNMP_DOMAINS: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_DOMAINS;
pub const SNMP_PROXIES: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_PROXIES;
pub const SNMP_MODULES: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_MODULES;

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

/// `ExtUTCTime` (RFC 2578 §2), a time to the minute in UTC.
#[derive(AsnType, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(universal, 4))]
pub struct ExtUtcTime(pub UtcTime);

impl Encode for ExtUtcTime {
    fn encode_with_tag_and_constraints<'encoder, EN: Encoder<'encoder>>(
        &self,
        encoder: &mut EN,
        tag: Tag,
        _: &Constraints,
        identifier: Identifier,
    ) -> Result<(), EN::Error> {
        const CONSTRAINT_1: constraints::Constraints = constraints!(value_constraint!(13));
        let date = self.0.date();
        let year = date.year();
        let year = if (0..=9999).contains(&year) {
            format!("{year:04}")
        } else {
            format!("{year:+05}")
        };
        let text = format!(
            "{year}{:02}{:02}{:02}{:02}Z",
            date.month(),
            date.day(),
            self.0.hour(),
            self.0.minute()
        );
        encoder
            .encode_octet_string(tag, &CONSTRAINT_1, text.as_bytes(), identifier)
            .map(drop)
    }
}

impl Decode for ExtUtcTime {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: &Constraints,
    ) -> Result<Self, D::Error> {
        let bytes = OctetString::decode_with_tag(decoder, tag)?;
        parse_ext_utc_time(&bytes).map(Self).ok_or_else(|| {
            rasn::de::Error::custom("Invalid `ExtUtcTime` encoding", decoder.codec())
        })
    }
}

/// Parses `YYMMDDHHMMZ`, where `YY` is a year from 1900 to 1999, or
/// `YYYYMMDDHHMMZ`.
fn parse_ext_utc_time(bytes: &[u8]) -> Option<UtcTime> {
    fn number(digits: &[u8]) -> Option<u16> {
        digits.iter().try_fold(0, |value, &digit| {
            digit
                .is_ascii_digit()
                .then(|| value * 10 + u16::from(digit - b'0'))
        })
    }
    let (year, rest) = match bytes.len() {
        11 => (1900 + number(&bytes[..2])?, &bytes[2..]),
        13 => (number(&bytes[..4])?, &bytes[4..]),
        _ => return None,
    };
    let [month, day, hour, minute] = [0, 2, 4, 6].map(|at| number(&rest[at..at + 2]));
    if rest[8] != b'Z' {
        return None;
    }
    let date = Date::new(year.into(), month? as u8, day? as u8)?;
    UtcTime::new(date, hour? as u8, minute? as u8, 0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_utc_time() {
        let time = ExtUtcTime(UtcTime::new(Date::new(2024, 4, 4).unwrap(), 13, 5, 0).unwrap());
        let encoded = rasn::ber::encode(&time).unwrap();
        assert_eq!(&encoded[2..], b"202404041305Z");
        assert_eq!(rasn::ber::decode::<ExtUtcTime>(&encoded).unwrap(), time);

        let short = rasn::ber::encode(&OctetString::from_static(b"9912312359Z")).unwrap();
        let expected = UtcTime::new(Date::new(1999, 12, 31).unwrap(), 23, 59, 0).unwrap();
        assert_eq!(rasn::ber::decode::<ExtUtcTime>(&short).unwrap().0, expected);

        for invalid in [&b"202404041305"[..], b"20240404130Z", b"202402301305Z"] {
            let encoded = rasn::ber::encode(&OctetString::from_slice(invalid)).unwrap();
            assert!(rasn::ber::decode::<ExtUtcTime>(&encoded).is_err());
        }
    }
}
