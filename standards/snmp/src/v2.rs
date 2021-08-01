//! Version 2 (RFC 3416)

use rasn::{types::Integer, AsnType, Decode, Encode};

pub use smi::v2::{IpAddress, ObjectName, ObjectSyntax, TimeTicks};

pub type GetRequest = crate::v1::GetRequest;
pub type GetNextRequest = crate::v1::GetNextRequest;
pub type Response = crate::v1::GetResponse;
pub type SetRequest = crate::v1::SetRequest;
pub type VarBindList = alloc::vec::Vec<VarBind>;

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(choice)]
pub enum Pdus {
    GetRequest(GetRequest),
    GetNextRequest(GetNextRequest),
    Response(Response),
    SetRequest(SetRequest),
    GetBulkRequest(GetBulkRequest),
    InformRequest(InformRequest),
    Trap(Trap),
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(5))]
#[rasn(delegate)]
pub struct GetBulkRequest(pub BulkPdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(6))]
#[rasn(delegate)]
pub struct InformRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(7))]
#[rasn(delegate)]
pub struct Trap(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct Pdu {
    pub request_id: i32,
    pub error_status: Integer,
    pub error_index: u32,
    pub variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct BulkPdu {
    pub request_id: i32,
    pub non_repeaters: u32,
    pub max_repetitions: u32,
    pub variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct VarBind {
    pub name: ObjectName,
    #[rasn(choice)]
    pub value: VarBindValue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(choice)]
pub enum VarBindValue {
    Value(ObjectSyntax),
    Unspecified,
    #[rasn(tag(0))]
    NoSuchObject,
    #[rasn(tag(1))]
    NoSuchInstance,
    #[rasn(tag(2))]
    EndOfMibView,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use rasn::types::ObjectIdentifier;

    #[test]
    fn get_bulk_request() {
        let request = GetBulkRequest(BulkPdu {
            request_id: 1414684022,
            non_repeaters: 1,
            max_repetitions: 2,
            variable_bindings: vec![
                VarBind {
                    name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 3]),
                    value: VarBindValue::Unspecified,
                },
                VarBind {
                    name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 4, 0x16, 1, 2]),
                    value: VarBindValue::Unspecified,
                },
                VarBind {
                    name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 4, 0x16, 1, 4]),
                    value: VarBindValue::Unspecified,
                },
            ],
        });

        let data = rasn::ber::encode(&request).unwrap();

        rasn::ber::decode::<GetBulkRequest>(&data).unwrap();
        assert_eq!(
            data,
            vec![
                // [5] IMPLICIT SEEQUENCE
                0xA5, 0x39, // INTEGER
                0x02, 0x04, 0x54, 0x52, 0x5d, 0x76, // INTEGER
                0x02, 0x01, 0x01, // INTEGER
                0x02, 0x01, 0x02, // SEQUENCE
                0x30, 0x2b, // SEQUENCE
                0x30, 0x0b, // OBJECT IDENTIFIER
                0x06, 0x07, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x03, // NULL
                0x05, 0x00, // SEQUENCE
                0x30, 0x0d, // OBJECT IDENTIFIER
                0x06, 0x09, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x04, 0x16, 0x01, 0x02, // NULL
                0x05, 0x00, // SEQUENCE
                0x30, 0x0d, // OBJECT IDENTIFIER
                0x06, 0x09, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x04, 0x16, 0x01, 0x04, // NULL
                0x05, 0x00,
            ]
        );
    }
}
