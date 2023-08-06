//! Version 3 Message Format (RFCs 3412, 3414)
//!
//! SNMPv3 defines a message format that encapsulates the PDU format of v2 with extensions for the
//! v3 security model.
//!
//! - [RFC 3412](https://datatracker.ietf.org/doc/html/rfc3412): Message Processing and Dispatching
//!   for the Simple Network Management Protocol (SNMP)
//! - [RFC 3414](https://datatracker.ietf.org/doc/html/rfc3414): User-based Security Model (USM)
//!   for version 3 of the Simple Network Management Protocol (SNMPv3)
//! - [RFC 3416](https://datatracker.ietf.org/doc/html/rfc3416): Version 2 of the Protocol
//!   Operations for the Simple Network Management Protocol (SNMP)

use rasn::{
    prelude::*,
    types::{Integer, OctetString},
};

pub use crate::v2::{
    GetBulkRequest, GetNextRequest, GetRequest, InformRequest, Pdus, Response, SetRequest, Trap,
    VarBind, VarBindValue,
};

/// The SNMPv3 message format, corresponding to the SNMP version 3 Message Processing Model.
///
/// [RFC 3412 § 6](https://datatracker.ietf.org/doc/html/rfc3412#section-6)
#[derive(AsnType, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Message {
    /// Identifies the layout of the SNMPv3 Message.
    ///
    /// This element is in the same position as in SNMPv1 and SNMPv2c, allowing recognition. The
    /// value `3` is used for SNMPv3.
    ///
    /// [RFC 3412 § 6.1](https://datatracker.ietf.org/doc/html/rfc3412#section-6.1)
    pub version: Integer,
    /// Administrative parameters.
    pub global_data: HeaderData,
    /// Security model specific parameters.
    ///
    /// The contents of this field is defined by the Security Model, however only the User-based
    /// Security Model ([RFC 3414](https://datatracker.ietf.org/doc/html/rfc3414)) is defined here.
    ///
    /// [RFC 3412 § 6.6](https://datatracker.ietf.org/doc/html/rfc3412#section-6.6)
    pub security_parameters: USMSecurityParameters,
    /// Message data.
    pub scoped_data: ScopedPduData,
}

impl Decode for Message {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        NestedMessage::decode_with_tag_and_constraints(decoder, tag, constraints).map(Self::from)
    }
}

impl Encode for Message {
    fn encode_with_tag_and_constraints<E: rasn::Encoder>(
        &self,
        encoder: &mut E,
        tag: rasn::Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        NestedMessage::from(self.clone()).encode_with_tag_and_constraints(encoder, tag, constraints)
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

/// A helper type to allow a BER-encoded message to be nested in an OCTET STRING field.
#[derive(AsnType, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(universal, 4))]
struct Nested<T>(T);

impl<T: Decode> Decode for Nested<T> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_explicit_prefix(tag).map(Self)
    }
}

impl<T: Encode> Encode for Nested<T> {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_explicit_prefix(tag, &self.0).map(drop)
    }
}

/// The actual encoding for `Message`, with `security_parameters` nested in an OCTET STRING field.
/// The `Encode` and `Decode` impls for `Message` are defined in terms of converting to and from
/// `NestedMessage` prior to encode/decode.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

/// Administrative data about a `Message`.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HeaderData {
    /// The message ID.
    ///
    /// Used between two SNMP entities to coordinate request messages and responses, and by the
    /// v3MP to coordinate the processing of the message by different subsystem models within the
    /// architecture.
    ///
    /// [RFC 3412 § 6.2](https://datatracker.ietf.org/doc/html/rfc3412#section-6.2)
    pub message_id: Integer,
    /// The maximum message size supported by the sender of the message.
    ///
    /// [RFC 3412 § 6.3](https://datatracker.ietf.org/doc/html/rfc3412#section-6.3)
    pub max_size: Integer,
    /// Bit fields which control processing of the message.
    ///
    /// ```text
    /// .... ...1   authFlag
    /// .... ..1.   privFlag
    /// .... .1..   reportableFlag
    ///             Please observe:
    /// .... ..00   is OK, means noAuthNoPriv
    /// .... ..01   is OK, means authNoPriv
    /// .... ..10   reserved, MUST NOT be used
    /// .... ..11   is OK, means authPriv
    /// ```
    ///
    /// [RFC 3412 § 6.4](https://datatracker.ietf.org/doc/html/rfc3412#section-6.4)
    pub flags: OctetString,
    /// Identifies which Security Model was used by the sender to generate the message
    ///
    /// [RFC 3412 § 6.5](https://datatracker.ietf.org/doc/html/rfc3412#section-6.5)
    pub security_model: Integer,
}

/// Represents either the plain text `ScopedPdu` if the `privFlag` in `Message::flags` is zero, or
/// it represents an encrypted PDU (encoded as an OCTET STRING) which MUST be decrypted by the
/// security model in use to produce a plaintext `ScopedPdu`.
///
/// [RFC 3412 § 6.7](https://datatracker.ietf.org/doc/html/rfc3412#section-6.7)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum ScopedPduData {
    CleartextPdu(ScopedPdu),
    EncryptedPdu(OctetString),
}

/// Contains information to identify an administratively unique context and a PDU.
///
/// The object identifiers in the PDU refer to managed objects which are (expected to be)
/// accessible within the specified context.
///
/// [RFC 3412 § 6.8](https://datatracker.ietf.org/doc/html/rfc3412#section-6.8)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ScopedPdu {
    /// Uniquely identifies, within an administrative domain, an SNMP entity that may realize an
    /// instance of a context with a particular contextName.
    ///
    /// [RFC 3412 § 6.8.1](https://datatracker.ietf.org/doc/html/rfc3412#section-6.8.1)
    pub engine_id: OctetString,
    /// Identifies, in conjunction with the contextEngineID field, the particular context
    /// associated with the management information contained in the PDU portion of the message.
    ///
    /// [RFC 3412 § 6.8.2](https://datatracker.ietf.org/doc/html/rfc3412#section-6.8.2)
    pub name: OctetString,
    /// Contains the PDU.
    ///
    /// Among other things, the PDU contains the PDU type that is used by the v3MP to determine the
    /// type of the incoming SNMP message. The v3MP specifies that the PDU MUST be one of those
    /// specified in [RFC 3416](https://datatracker.ietf.org/doc/html/rfc3416).
    ///
    /// [RFC 3412 § 6.8.3](https://datatracker.ietf.org/doc/html/rfc3412#section-6.8.3)
    pub data: Pdus,
}

/// The security parameters encoding for User-based Security Model.
///
/// [RFC 3414 § 2.4](https://datatracker.ietf.org/doc/html/rfc3414#section-2.4)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct USMSecurityParameters {
    /// The `snmpEngineID` of the authoritative SNMP engine involved in the exchange of the
    /// message.
    pub authoritative_engine_id: OctetString,
    /// The `snmpEngineBoots` value at the authoritative SNMP engine involved in the exchange of
    /// the message.
    pub authoritative_engine_boots: Integer,
    /// The `snmpEngineTime` value at the authoritative SNMP engine involved in the exchange of the
    /// message.
    pub authoritative_engine_time: Integer,
    /// The user (principal) on whose behalf the message is being exchanged.
    ///
    /// Note that a zero-length `user_name` will not match any user, but it can be used for
    /// `snmpEngineID` discovery.
    pub user_name: OctetString,
    /// Defined by the authentication protocol in use for the message, if any.
    pub authentication_parameters: OctetString,
    /// Defined by the privacy protocol in use for the message, if any.
    pub privacy_parameters: OctetString,
}
