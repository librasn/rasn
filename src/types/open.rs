use super::{
    AsnType, BitString, GeneralizedTime, InstanceOf, Integer, ObjectIdentifier, OctetString,
    UniversalString, UtcTime, VisibleString,
};
use crate::{Decode, Encode};

/// An "open" type representing any valid ASN.1 type.
#[derive(AsnType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
#[rasn(crate_root = "crate")]
#[rasn(choice)]
pub enum Open {
    /// A bit string value.
    BitString(BitString),
    // BmpString(BmpString),
    /// A bool value.
    Bool(bool),
    /// A generalized time value.
    GeneralizedTime(GeneralizedTime),
    // Ia5String(Ia5String),
    /// A integer value.
    Integer(Integer),
    /// A null value.
    Null,
    /// A object identifier value.
    ObjectIdentifier(ObjectIdentifier),
    /// A octet string value.
    OctetString(OctetString),
    // PrintableString(PrintableString),
    /// A universal string value.
    UniversalString(UniversalString),
    /// A utc time value.
    UtcTime(UtcTime),
    /// A visible string value.
    VisibleString(VisibleString),
    /// An "instance of" value.
    InstanceOf(alloc::boxed::Box<InstanceOf<Open>>),
}
