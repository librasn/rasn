use rasn::{
    types::{Class, ConstOid, Implicit, Integer, ObjectIdentifier, OctetString, Oid},
    AsnType, Decode, Encode, Tag,
};

pub type ObjectName = ObjectIdentifier;
pub type IpAddress = Implicit<{ Tag::new(Class::Application, 0) }, [u8; 4]>;
pub type Counter = Implicit<{ Tag::new(Class::Application, 1) }, u32>;
pub type Gauge = Implicit<{ Tag::new(Class::Application, 2) }, u32>;
pub type TimeTicks = Implicit<{ Tag::new(Class::Application, 3) }, u32>;
pub type Opaque = Implicit<{ Tag::new(Class::Application, 4) }, OctetString>;

pub const INTERNET: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET;
pub const DIRECTORY: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY;
pub const MGMT: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT;
pub const EXPERIMENTAL: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_EXPERIMENTAL;
pub const PRIVATE: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE;
pub const ENTERPRISES: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES;

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ObjectSyntax {
    Simple(SimpleSyntax),
    ApplicationWide(ApplicationSyntax),
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum SimpleSyntax {
    Number(Integer),
    String(OctetString),
    Object(ObjectIdentifier),
    Empty,
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ApplicationSyntax {
    Address(NetworkAddress),
    Counter(Counter),
    Gauge(Gauge),
    Ticks(TimeTicks),
    Arbitrary(Opaque),
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum NetworkAddress {
    Internet(IpAddress),
}
