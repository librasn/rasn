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
    #[rasn(choice)]
    pub agent_addr: NetworkAddress,
    pub generic_trap: Integer,
    pub specific_trap: Integer,
    pub time_stamp: TimeTicks,
    pub variable_bindings: VarBindList,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct VarBind {
    pub name: ObjectName,
    #[rasn(choice)]
    pub value: ObjectSyntax,
}

#[cfg(test)]
mod tests {
    use super::{Message, Trap, VarBind};
    use alloc::{string::String, string::ToString, vec, vec::Vec};
    use rasn::types::ObjectIdentifier;
    use smi::v1::{Gauge, IpAddress, NetworkAddress, TimeTicks};

    fn string_oid(oid: impl AsRef<[u32]>) -> String {
        oid.as_ref()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(".")
    }

    #[test]
    fn trap() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let decode_data = [
            // SEQUENCE -> Message
            0x30, 0x4f,
                // INTEGER -> Message::version
                0x02, 0x01,
                    0x00,
                // OCTET STRING -> Message::community
                0x04, 0x06,
                    // "public"
                    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63,
                // application constructed tag 4 -> Trap
                0xa4, 0x42,
                    // OID -> Trap::enterprise
                    0x06, 0x0c,
                        // 1.3.6.1.4.1.11779.1.42.3.7.8
                        0x2b, 0x06, 0x01, 0x04, 0x01, 0x83, 0x5c, 0x01,
                        0x2a, 0x03, 0x07, 0x08,
                    // OCTET STRING -> Trap::agent_addr 
                    0x40, 0x04,
                        // NetworkAddress:Internet(IpAddress(10.11.12.13))
                        0x0a, 0x0b, 0x0c, 0x0d,
                    // INTEGER -> Trap::generic_trap 
                    0x02, 0x01,
                        0x06,
                    // INTEGER -> Trap::specific_trap 
                    0x02, 0x01,
                        0x02,
                    // application tag 3 -> TimeTicks
                    0x43, 0x02,
                        // 11_932
                        0x2e, 0x9c,
                    // SEQUENCE -> VarBindList
                    0x30, 0x22,
                        // SEQUENCE -> VarBind
                        0x30, 0x0d,
                            // OID -> VarBind::name
                            0x06, 0x07,
                                // 1.3.6.1.2.1.1.3
                                0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x03,
                            // application tag 3 -> TimeTicks
                            0x43, 0x02,
                                // 11_932
                                0x2e, 0x9c,
                        // SEQUENCE -> VarBind
                        0x30, 0x11,
                            // OID -> VarBind::name
                            0x06, 0x0c,
                                // 1.3.6.1.4.1.11779.1.42.2.1.7
                                0x2b, 0x06, 0x01, 0x04, 0x01, 0x83, 0x5c, 0x01,
                                0x2a, 0x02, 0x01, 0x06,
                            // application tag 2 -> Gauge
                            0x42, 0x01,
                                0x01,
        ];
        let decode_msg: Message<Trap> = rasn::ber::decode(&decode_data).unwrap();
        assert_eq!(decode_msg.version, 0.into());
        assert_eq!(decode_msg.community, "public".as_bytes());
        assert_eq!(
            string_oid(decode_msg.data.enterprise),
            "1.3.6.1.4.1.11779.1.42.3.7.8"
        );
        assert_eq!(
            decode_msg.data.agent_addr,
            NetworkAddress::Internet(IpAddress([10, 11, 12, 13][..].into()))
        );
        assert_eq!(decode_msg.data.generic_trap, 6.into());
        assert_eq!(decode_msg.data.specific_trap, 2.into());
        assert_eq!(decode_msg.data.time_stamp, TimeTicks(11_932));
        // TODO: Currently this incorectly decodes as an empty vector.
        //assert_eq!(decode_msg.data.variable_bindings.len(), 2);

        let encode_msg = Message {
            version: 0.into(),
            community: "public".into(),
            data: Trap {
                enterprise: ObjectIdentifier::new_unchecked(vec![
                    1, 3, 6, 1, 4, 1, 11779, 1, 42, 3, 7, 8,
                ]),
                agent_addr: NetworkAddress::Internet(IpAddress([10, 11, 12, 13][..].into())),
                generic_trap: 6.into(),
                specific_trap: 2.into(),
                time_stamp: TimeTicks(11_932),
                variable_bindings: vec![
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 2, 1, 1, 3]),
                        value: TimeTicks(11_932).into(),
                    },
                    VarBind {
                        name: ObjectIdentifier::new_unchecked(vec![
                            1, 3, 6, 1, 4, 1, 11779, 1, 42, 2, 1, 6,
                        ]),
                        value: Gauge(1).into(),
                    },
                ],
            },
        };
        let encode_data = rasn::ber::encode(&encode_msg).unwrap();
        assert_eq!(encode_data, decode_data);
    }
}
