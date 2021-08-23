//! Version 3 Message Format (RFC 3412)
//!
//! SNMPv3 defines a message format that encapsulates the PDU format of v2 (by
//! reference to RFC 3416) with extensions for the v3 security model.

use rasn::types::{Integer, OctetString};
use rasn::{AsnType, Decode, Encode};

pub use crate::v2::{
    GetBulkRequest, GetNextRequest, GetRequest, InformRequest, Pdus, Response, SetRequest, Trap,
    VarBind, VarBindValue,
};

#[derive(AsnType, Debug, Clone)]
pub struct Message {
    pub version: Integer,
    pub global_data: HeaderData,
    pub security_parameters: USMSecurityParameters,
    pub scoped_data: ScopedPduData,
}

impl Decode for Message {
    fn decode_with_tag<D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::Tag,
    ) -> Result<Self, D::Error> {
        NestedMessage::decode_with_tag(decoder, tag).map(Self::from)
    }
}

impl Encode for Message {
    fn encode_with_tag<E: rasn::Encoder>(
        &self,
        encoder: &mut E,
        tag: rasn::Tag,
    ) -> Result<(), E::Error> {
        NestedMessage::from(self.clone()).encode_with_tag(encoder, tag)
    }
}

impl From<NestedMessage> for Message {
    fn from(m: NestedMessage) -> Self {
        Self {
            version: m.version,
            global_data: m.global_data,
            security_parameters: m.security_parameters.0,
            scoped_data: m.scoped_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(tag(universal, 4))]
struct Nested<T>(T);

#[derive(AsnType, Debug, Clone, Decode, Encode)]
struct NestedMessage {
    version: Integer,
    global_data: HeaderData,
    security_parameters: Nested<USMSecurityParameters>,
    scoped_data: ScopedPduData,
}

impl From<Message> for NestedMessage {
    fn from(m: Message) -> Self {
        Self {
            version: m.version,
            global_data: m.global_data,
            security_parameters: Nested(m.security_parameters),
            scoped_data: m.scoped_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct HeaderData {
    pub message_id: Integer,
    pub max_size: Integer,
    pub flags: OctetString,
    pub security_model: Integer,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
#[rasn(choice)]
pub enum ScopedPduData {
    CleartextPdu(ScopedPdu),
    EncryptedPdu(OctetString),
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct ScopedPdu {
    pub engine_id: OctetString,
    pub name: OctetString,
    pub data: Pdus,
}

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct USMSecurityParameters {
    pub authoritative_engine_id: OctetString,
    pub authoritative_engine_boots: Integer,
    pub authoritative_engine_time: Integer,
    pub user_name: OctetString,
    pub authentication_parameters: OctetString,
    pub privacy_parameters: OctetString,
}
