use super::*;
use crate::{Decode, Encode};

/// An "open" type representing any valid ASN.1 type.
#[derive(AsnType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
#[rasn(crate_root = "crate")]
#[rasn(choice)]
pub enum Open {
    BitString(BitString),
    // BmpString(BmpString),
    Bool(bool),
    GeneralizedTime(GeneralizedTime),
    // Ia5String(Ia5String),
    Integer(Integer),
    Null,
    ObjectIdentifier(ObjectIdentifier),
    OctetString(OctetString),
    // PrintableString(PrintableString),
    UniversalString(UniversalString),
    UtcTime(UtcTime),
    VisibleString(VisibleString),
    InstanceOf(alloc::boxed::Box<InstanceOf<Open>>),
}
