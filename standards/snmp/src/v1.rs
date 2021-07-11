use rasn::{
    types::{Integer, ObjectIdentifier, OctetString},
    AsnType, Decode, Encode,
};
use smi::v1::{NetworkAddress, ObjectName, ObjectSyntax, TimeTicks};

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct Message<T> {
    version: Integer,
    community: OctetString,
    data: T,
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
    request_id: Integer,
    error_status: Integer,
    error_index: Integer,
    variable_bindings: VarBindList,
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
    enterprise: ObjectIdentifier,
    agent_addr: NetworkAddress,
    generic_trap: Integer,
    specific_trap: Integer,
    time_stamp: TimeTicks,
    variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct VarBind {
    name: ObjectName,
    value: ObjectSyntax,
}
