//! Version 1 (RFC 1157)

use rasn::{
    types::{Integer, ObjectIdentifier, OctetString},
    AsnType, Decode, Encode,
};
use smi::v1::{NetworkAddress, ObjectName, ObjectSyntax, TimeTicks};

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct Message<T> {
    pub version: Integer,
    pub community: OctetString,
    pub data: T,
}

impl<T> Message<T> {
    pub const VERSION_1: u64 = 0;
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(choice)]
pub enum Pdus {
    GetRequest(GetRequest),
    GetNextRequest(GetNextRequest),
    GetResponse(GetResponse),
    SetRequest(SetRequest),
    Trap(Trap),
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(0))]
#[rasn(delegate)]
pub struct GetRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(1))]
#[rasn(delegate)]
pub struct GetNextRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(2))]
#[rasn(delegate)]
pub struct GetResponse(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(3))]
#[rasn(delegate)]
pub struct SetRequest(pub Pdu);

pub type VarBindList = alloc::vec::Vec<VarBind>;

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct Pdu {
    pub request_id: Integer,
    pub error_status: Integer,
    pub error_index: Integer,
    pub variable_bindings: VarBindList,
}

impl Pdu {
    pub const ERROR_STATUS_NO_ERROR: u64 = 0;
    pub const ERROR_STATUS_TOO_BIG: u64 = 1;
    pub const ERROR_STATUS_NO_SUCH_NAME: u64 = 2;
    pub const ERROR_STATUS_BAD_VALUE: u64 = 3;
    pub const ERROR_STATUS_READ_ONLY: u64 = 4;
    pub const ERROR_STATUS_GEN_ERR: u64 = 5;
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(context, 4))]
pub struct Trap {
    pub enterprise: ObjectIdentifier,
    pub agent_addr: NetworkAddress,
    pub generic_trap: Integer,
    pub specific_trap: Integer,
    pub time_stamp: TimeTicks,
    pub variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct VarBind {
    pub name: ObjectName,
    pub value: ObjectSyntax,
}
