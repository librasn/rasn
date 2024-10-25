//! Version 2 (RFC 3416)
use rasn::prelude::*;

pub use smi::v2::{IpAddress, ObjectName, ObjectSyntax, TimeTicks};

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum Pdus {
    GetRequest(GetRequest),
    GetNextRequest(GetNextRequest),
    Response(Response),
    SetRequest(SetRequest),
    GetBulkRequest(GetBulkRequest),
    InformRequest(InformRequest),
    Trap(Trap),
    Report(Report),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(0))]
#[rasn(delegate)]
pub struct GetRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(1))]
#[rasn(delegate)]
pub struct GetNextRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(2))]
#[rasn(delegate)]
pub struct Response(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(3))]
#[rasn(delegate)]
pub struct SetRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(5))]
#[rasn(delegate)]
pub struct GetBulkRequest(pub BulkPdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(6))]
#[rasn(delegate)]
pub struct InformRequest(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(7))]
#[rasn(delegate)]
pub struct Trap(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(8))]
#[rasn(delegate)]
pub struct Report(pub Pdu);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pdu {
    pub request_id: i32,
    pub error_status: u32,
    pub error_index: u32,
    pub variable_bindings: VarBindList,
}

impl Pdu {
    pub const ERROR_STATUS_NO_ERROR: u32 = 0;
    pub const ERROR_STATUS_TOO_BIG: u32 = 1;
    pub const ERROR_STATUS_NO_SUCH_NAME: u32 = 2;
    pub const ERROR_STATUS_BAD_VALUE: u32 = 3;
    pub const ERROR_STATUS_READ_ONLY: u32 = 4;
    pub const ERROR_STATUS_GEN_ERR: u32 = 5;
    pub const ERROR_STATUS_NO_ACCESS: u32 = 6;
    pub const ERROR_STATUS_WRONG_TYPE: u32 = 7;
    pub const ERROR_STATUS_WRONG_LENGTH: u32 = 8;
    pub const ERROR_STATUS_WRONG_ENCODING: u32 = 9;
    pub const ERROR_STATUS_WRONG_VALUE: u32 = 10;
    pub const ERROR_STATUS_NO_CREATION: u32 = 11;
    pub const ERROR_STATUS_INCONSISTENT_VALUE: u32 = 12;
    pub const ERROR_STATUS_RESOURCE_UNAVAILABLE: u32 = 13;
    pub const ERROR_STATUS_COMMIT_FAILED: u32 = 14;
    pub const ERROR_STATUS_UNDO_FAILED: u32 = 15;
    pub const ERROR_STATUS_AUTHORIZATION_ERROR: u32 = 16;
    pub const ERROR_STATUS_NOT_WRITABLE: u32 = 17;
    pub const ERROR_STATUS_INCONSISTENT_NAME: u32 = 18;
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BulkPdu {
    pub request_id: i32,
    pub non_repeaters: u32,
    pub max_repetitions: u32,
    pub variable_bindings: VarBindList,
}

pub type VarBindList = alloc::vec::Vec<VarBind>;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VarBind {
    pub name: ObjectName,
    pub value: VarBindValue,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    use pretty_assertions::assert_eq;

    use super::*;
    use alloc::vec;
    use rasn::types::ObjectIdentifier;
    use smi::v2::SimpleSyntax;

    #[test]
    fn encode_decode_response() {
        let request = crate::v2c::Message {
            version: 1.into(),
            community: "public".into(),
            data: Response(Pdu {
                request_id: 1414684022,
                error_status: Pdu::ERROR_STATUS_NO_ERROR,
                error_index: 0,
                variable_bindings: vec![
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(
                            vec![1, 3, 6, 1, 2, 1, 2, 2, 1, 1, 12].into(),
                        ),
                        value: VarBindValue::NoSuchInstance,
                    },
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(
                            vec![1, 3, 6, 1, 2, 1, 2, 2, 1, 1, 13].into(),
                        ),
                        value: VarBindValue::Value(ObjectSyntax::Simple(SimpleSyntax::Integer(
                            6.into(),
                        ))),
                    },
                ],
            }),
        };

        #[rustfmt::skip]
        let expected_data: &[u8] = &[
            //    SEQUENCE {
            //       INTEGER 0x01 (1 decimal)
            //       OCTETSTRING 7075626C6963
            //       [2] {
            //          INTEGER 0x54525D76 (1414684022 decimal)
            //          INTEGER 0x00 (0 decimal)
            //          INTEGER 0x00 (0 decimal)
            //          SEQUENCE {
            //             SEQUENCE {
            //                OBJECTIDENTIFIER 1.3.6.1.2.2.1.1.12
            //                [1]
            //             }
            //             SEQUENCE {
            //                OBJECTIDENTIFIER 1.3.6.1.2.2.1.1.13
            //                INTEGER 0x06 (6 decimal)
            //             }
            //          }
            //       }
            //    }

            // SEQUENCE -> Message
            0x30, 0x3C,

            // INTEGER -> Message::version
            0x02, 0x01,

            // OCTET STRING -> Message::community
            0x01, 0x04,

                // "public"
                0x06, 0x70, 0x75, 0x62, 0x6C, 0x69, 0x63,

            // application constructed tag 2 -> Response
            0xA2, 0x2F,

                // INTEGER -> request_id
                0x02, 0x04, 0x54, 0x52, 0x5D, 0x76,

                // INTEGER -> error_status
                0x02, 0x01, 0x00,

                // INTEGER -> error_index
                0x02, 0x01, 0x00,

                // SEQUENCE -> VarBindList
                0x30, 0x21,

                    // VarBind Name
                    0x30, 0x0E,
                        // OBJECTIDENTIFIER 1.3.6.1.2.2.1.1.13
                        0x06, 0x0A, 0x2B, 0x06, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01, 0x01, 0x0C,

                        // VarBind Value NoSuchInstance
                        0x81, 0x00,

                    // VarBind Name
                    0x30, 0x0F,
                        // OBJECTIDENTIFIER 1.3.6.1.2.2.1.1.13
                        0x06, 0x0A, 0x2B, 0x06, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01, 0x01, 0x0D,

                        // INTEGER 0x06 (6 decimal)
                        0x02, 0x01, 0x06,
        ];

        // Encode Response message into BER
        let encoded_data = rasn::ber::encode(&request).unwrap();
        assert_eq!(encoded_data, expected_data);

        // Decode object from BER
        let decoded_request = rasn::ber::decode(&encoded_data);

        assert!(decoded_request.is_ok());
        assert_eq!(request, decoded_request.unwrap());
    }

    #[test]
    fn get_bulk_request() {
        let request = GetBulkRequest(BulkPdu {
            request_id: 1414684022,
            non_repeaters: 1,
            max_repetitions: 2,
            variable_bindings: vec![
                VarBind {
                    name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 3].into()),
                    value: VarBindValue::Unspecified,
                },
                VarBind {
                    name: ObjectIdentifier::new_unchecked(
                        vec![1, 3, 6, 1, 2, 1, 4, 0x16, 1, 2].into(),
                    ),
                    value: VarBindValue::Unspecified,
                },
                VarBind {
                    name: ObjectIdentifier::new_unchecked(
                        vec![1, 3, 6, 1, 2, 1, 4, 0x16, 1, 4].into(),
                    ),
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
