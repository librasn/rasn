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

use alloc::boxed::Box;

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
#[derive(AsnType, Debug, Decode, Encode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    /// The contents of this field is defined by the Security Model.
    ///
    /// [RFC 3412 § 6.6](https://datatracker.ietf.org/doc/html/rfc3412#section-6.6)
    pub security_parameters: OctetString,
    /// Message data.
    pub scoped_data: ScopedPduData,
}

impl Message {
    /// Decodes the `security_parameters` field from a `Codec` into `T`.
    ///
    /// # Errors
    /// - If `T::ID` does not match `global_data.security_model`.
    /// - If the value fails to be encoded as the `codec`.
    pub fn decode_security_parameters<T: SecurityParameters>(
        &self,
        codec: rasn::codec::Codec,
    ) -> Result<T, alloc::boxed::Box<dyn core::fmt::Display>> {
        if self.global_data.security_model != T::ID.into() {
            return Err(Box::new(alloc::format!(
                "`{}` doesn't match security model `{}`",
                self.global_data.security_model,
                T::ID
            )) as Box<_>);
        }

        codec.decode::<T>(&self.security_parameters)
            .map_err(|error| Box::new(error) as Box<_>)
    }

    /// Encodes the given security parameters into the `security_parameters`
    /// field given a certain `Codec`.
    ///
    /// # Errors
    /// - If the value fails to be encoded as the `codec`.
    pub fn encode_security_parameters<T: SecurityParameters>(
        &mut self,
        codec: rasn::codec::Codec,
        value: &T,
    ) -> Result<(), alloc::boxed::Box<dyn core::fmt::Display>> {
        self.global_data.security_model = T::ID.into();

        self.security_parameters = codec.encode::<T>(value)
            .map_err(|error| Box::new(error) as Box<_>)?
            .into();
        Ok(())
    }
}

/// A trait representing a type that is a valid "security parameter" used for
/// an SNMP v3 `Message`. You can use this in combination with the
/// [`Message::decode_security_parameters`] and
/// [`Message::encode_security_parameters`] to safely retrieve the encoded
/// parameters from a given message.
pub trait SecurityParameters: Decode + Encode {
    const ID: u32;
}

impl SecurityParameters for USMSecurityParameters {
    const ID: u32 = 3;
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
