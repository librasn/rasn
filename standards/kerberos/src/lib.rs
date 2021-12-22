#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "otp")]
pub mod otp;

use rasn::prelude::*;

pub type HostAddresses = SequenceOf<HostAddress>;
pub type AdMandatoryForKdc = AuthorizationData;

/// Element are intended for interpretation only by application servers that
/// understand the particular `type` of the embedded element.  Application
/// servers that do not understand the type of an element embedded within the
/// [AdIfRelevant] element MAY ignore the uninterpretable element. This element
/// promotes interoperability across implementations that may have local
/// extensions for authorization.
pub type AdIfRelevant = AuthorizationData;
pub type EtypeInfo2 = SequenceOf<EtypeInfo2Entry>;
pub type EtypeInfo = SequenceOf<EtypeInfoEntry>;
pub type PaEncTimestamp = EncryptedData;
pub type LastReq = SequenceOf<LastReqValue>;
pub type MethodData = SequenceOf<PaData>;
pub type KerberosString = GeneralString;
pub type ETypeList = SequenceOf<i32>;

/// The name of the authentication server.
pub type Realm = KerberosString;

//     ::= INTEGER (0..999999) -- microseconds
pub type Microseconds = Integer;

pub const OID: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_KERBEROS_V5;

/// The name of the party to verify. Taken together, a [PrincipalName] and a
/// [Realm] form a principal identifier.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrincipalName {
    /// The type of name that follows. This should be should be treated as a
    /// hint. Ignoring the type, no two names can be the same (i.e., at least
    /// one of the components, or the realm, must be different).
    #[rasn(tag(0))]
    pub r#type: i32,
    /// A sequence of components that form a name.
    #[rasn(tag(1))]
    pub string: SequenceOf<KerberosString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct KerberosTime(pub GeneralizedTime);

/// The address of a given host.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostAddress {
    /// The type of `address`. Some predefined values include:
    ///
    /// - [HostAddress::IPV4]
    /// - [HostAddress::DIRECTIONAL]
    /// - [HostAddress::CHAOS_NET]
    /// - [HostAddress::XNS]
    /// - [HostAddress::ISO]
    /// - [HostAddress::DECNET_PHASE_IV]
    /// - [HostAddress::APPLE_TALK_DDP]
    /// - [HostAddress::NET_BIOS]
    /// - [HostAddress::IPV6]
    #[rasn(tag(0))]
    pub addr_type: i32,
    /// A single address of type defined by `addr_type`.
    #[rasn(tag(1))]
    pub address: OctetString,
}

impl HostAddress {
    pub const IPV4: i32 = 2;
    pub const DIRECTIONAL: i32 = 3;
    pub const CHAOS_NET: i32 = 5;
    pub const XNS: i32 = 6;
    pub const ISO: i32 = 7;
    pub const DECNET_PHASE_IV: i32 = 12;
    pub const APPLE_TALK_DDP: i32 = 16;
    pub const NET_BIOS: i32 = 20;
    pub const IPV6: i32 = 24;
}

/// Authorization data.
pub type AuthorizationData = SequenceOf<AuthorizationDataValue>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorizationDataValue {
    /// Specifies the format for the `data` field.  All negative values are
    /// reserved for local use. Non-negative values are reserved for registered
    /// use. Some pre-defined values include:
    ///
    /// - [AuthorizationDataValue::IF_RELEVANT]
    /// - [AuthorizationDataValue::KDC_ISSUED]
    /// - [AuthorizationDataValue::AND_OR]
    /// - [AuthorizationDataValue::MANDATORY_FOR_KDC]
    #[rasn(tag(0))]
    pub r#type: i32,
    /// Authorization data to be interpreted according to the value of the
    /// corresponding `type` field.
    #[rasn(tag(1))]
    pub data: OctetString,
}

impl AuthorizationDataValue {
    /// DER encoding of [AdIfRelevant].
    pub const IF_RELEVANT: i32 = 1;
    /// DER encoding of [AdKdcIssued].
    pub const KDC_ISSUED: i32 = 4;
    /// DER encoding of [AdAndOr].
    pub const AND_OR: i32 = 5;
    /// DER encoding of [AdMandatoryForKdc].
    pub const MANDATORY_FOR_KDC: i32 = 8;
}

/// Pre-Authenication data.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaData {
    #[rasn(tag(1))]
    pub r#type: i32,
    #[rasn(tag(2))]
    pub value: OctetString,
}

// KerberosFlags   ::= BIT STRING (SIZE (32..MAX))
pub type KerberosFlags = BitString;

/// Container for arbitrary encrypted data
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptedData {
    /// Identifies which encryption algorithm was used to encipher the cipher.
    #[rasn(tag(0))]
    pub etype: i32,
    /// Contains the version number of the key under which data is encrypted.
    /// It is only present in messages encrypted under long lasting keys, such
    /// as principals' secret keys.
    #[rasn(tag(1))]
    pub kvno: Option<u32>,
    /// Contains the enciphered text.
    #[rasn(tag(2))]
    pub cipher: OctetString,
}

/// The means by which cryptographic keys used for encryption are transferred.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptionKey {
    /// This field specifies the encryption type of the encryption key that
    /// follows in the `value` field.
    #[rasn(tag(0))]
    pub r#type: i32,
    /// The key itself.
    #[rasn(tag(1))]
    pub value: OctetString,
}

/// Checksum of cleartext data.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Checksum {
    /// The algorithm used to generate the accompanying checksum.
    #[rasn(tag(0))]
    pub r#type: i32,
    /// The checksum itself.
    #[rasn(tag(1))]
    pub checksum: OctetString,
}

/// Record that helps a client authenticate to a service.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 1))]
pub struct Ticket {
    /// The version number for the ticket format.
    #[rasn(tag(0))]
    pub tkt_vno: Integer,
    /// The realm that issued a ticket.  It also serves to identify the realm
    /// part of the server's principal identifier. Since a Kerberos server can
    /// only issue tickets for servers within its realm, the two will always
    /// be identical.
    #[rasn(tag(1))]
    pub realm: Realm,
    /// All components of the name part of the server's identity, including
    /// those parts that identify a specific instance of a service.
    #[rasn(tag(2))]
    pub sname: PrincipalName,
    /// The encrypted encoding of [EncTicketPart]. It is encrypted in the key
    /// shared by Kerberos and the end server (the server's secret key), using a
    /// key usage value of `2`.
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

/// The encrypted part of a [Ticket].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 3))]
pub struct EncTicketPart {
    /// Options that were used or requested when the ticket was issued.
    #[rasn(tag(0))]
    pub flags: TicketFlags,
    /// Used to pass the session key from Kerberos to the application server and
    /// the client.
    #[rasn(tag(1))]
    pub key: EncryptionKey,
    /// The name of the realm in which the client is registered and in which
    /// initial authentication took place.
    #[rasn(tag(2))]
    pub crealm: Realm,
    /// The name part of the client's principal identifier.
    #[rasn(tag(3))]
    pub cname: PrincipalName,
    /// Lists the names of the Kerberos realms that took part in authenticating
    /// the user to whom this ticket was issued.  It does not specify the order
    /// in which the realms were transited.
    #[rasn(tag(4))]
    pub transited: TransitedEncoding,
    /// The time of initial authentication for the named principal. It
    /// is the time of issue for the original ticket on which this ticket is
    /// based.  It is included in the ticket to provide additional information
    /// to the end service, and to provide the necessary information for
    /// implementation of a "hot list" service at the KDC. An end service that
    /// is particularly paranoid could refuse to accept tickets for which the
    /// initial authentication occurred "too far" in the past.
    #[rasn(tag(5))]
    pub auth_time: KerberosTime,
    /// The time after which the ticket is valid. Together with `end_time`, this
    /// field specifies the life of the ticket. If the `start_time` field is
    /// absent from the ticket, then the `auth_time` field should be used in its
    /// place to determine the life of the ticket.
    #[rasn(tag(6))]
    pub start_time: Option<KerberosTime>,
    /// The time after which the ticket will not be honored (its expiration
    /// time). Note that individual services may place their own limits on the
    /// life of a ticket and MAY reject tickets which have not yet expired. As
    /// such, this is really an upper bound on the expiration time for
    /// the ticket.
    #[rasn(tag(7))]
    pub end_time: KerberosTime,
    /// The maximum `end_time` that may be included in a renewal. It can be
    /// thought of as the absolute expiration time for the ticket, including
    /// all renewals.
    ///
    /// Only present in tickets that have the [TicketFlags::renewable] flag set
    /// in the flags field.
    #[rasn(tag(8))]
    pub renew_till: Option<KerberosTime>,
    /// Addresses from which the ticket can be used.
    ///
    /// If there are no addresses, the ticket can be used from any location. The
    /// decision by the KDC to issue or by the end server to accept addressless
    /// tickets is a policy decision and is left to the Kerberos and end-service
    /// administrators; they may refuse to issue or accept such tickets. Because
    /// of the wide deployment of network address translation, it is recommended
    /// that policy allow the issue and acceptance of such tickets.
    #[rasn(tag(9))]
    pub caddr: Option<HostAddresses>,
    /// Restrictions on any authority obtained on the basis of authentication
    /// using the ticket. It is possible for any principal in possession of
    /// credentials to add entries to the authorization data field since these
    /// entries further restrict what can be done with the ticket. Such
    /// additions can be made by specifying the additional entries when a new
    /// ticket is obtained during the TGS exchange, or they MAY be added during
    /// chained delegation using the authorization data field of
    /// the authenticator.
    #[rasn(tag(10))]
    pub authorization_data: Option<AuthorizationData>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitedEncoding {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub contents: OctetString,
}

/// Various options that were used or requested when the ticket was issued.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct TicketFlags(pub KerberosFlags);

impl TicketFlags {
    /// Reserved for future expansion of this field.
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    /// Tells the ticket-granting server that it is OK to issue a new TGT with a
    /// different network address based on the presented ticket.
    pub fn forwardable() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    /// Indicates that the ticket has either been forwarded or was issued based
    /// on authentication involving a forwarded TGT.
    pub fn forwarded() -> Self {
        Self(KerberosFlags::from_element(2))
    }

    /// Identical to the [TicketFlags::forwardable] flag, except that it tells
    /// the ticket-granting server that only non-TGTs may be issued with
    /// different network addresses.
    pub fn proxiable() -> Self {
        Self(KerberosFlags::from_element(3))
    }

    /// Indicates that a ticket is a proxy.
    pub fn proxy() -> Self {
        Self(KerberosFlags::from_element(4))
    }

    /// Tells the ticket-granting server that a post-dated ticket may be issued
    /// based on this TGT.
    pub fn may_postdate() -> Self {
        Self(KerberosFlags::from_element(0x8))
    }

    /// Indicates that this ticket has been postdated. The end-service can check
    /// the `auth_time` field to see when the original authentication occurred.
    pub fn postdated() -> Self {
        Self(KerberosFlags::from_element(0x10))
    }

    /// Indicates that a ticket is invalid, and it must be validated by the KDC
    /// before use. Application servers must reject tickets which have this
    /// flag set.
    pub fn invalid() -> Self {
        Self(KerberosFlags::from_element(0x20))
    }

    /// Indicates that a ticket can be used to obtain a replacement ticket that
    /// expires at a later date.
    pub fn renewable() -> Self {
        Self(KerberosFlags::from_element(0x40))
    }

    /// Indicates that this ticket was issued using the AS protocol, and not
    /// issued based on a TGT.
    pub fn initial() -> Self {
        Self(KerberosFlags::from_element(0x80))
    }

    /// Indicates that during initial authentication, the client was
    /// authenticated by the KDC before a ticket was issued.  The strength of
    /// the pre-authentication method is not indicated, but is acceptable to
    /// the KDC.
    pub fn pre_authent() -> Self {
        Self(KerberosFlags::from_slice(&[0x1, 00]).unwrap())
    }

    /// Indicates that the protocol employed for initial authentication required
    /// the use of hardware expected to be possessed solely by the named client.
    /// The hardware authentication method is selected by the KDC and the
    /// strength of the method is not indicated.
    pub fn hw_authent() -> Self {
        Self(KerberosFlags::from_slice(&[0x2, 00]).unwrap())
    }

    /// Indicates that the KDC for the realm has checked the transited field
    /// against a realm-defined policy for trusted certifiers. If this flag is
    /// reset (0), then the application server must check the transited field
    /// itself, and if unable to do so, it must reject the authentication. If
    /// the flag is set (1), then the application server MAY skip its own
    /// validation of the transited field, relying on the validation performed
    /// by the KDC.  At its option the application server MAY still apply its
    /// own validation based on a separate policy for acceptance.
    pub fn transited_policy_checked() -> Self {
        Self(KerberosFlags::from_slice(&[0x4, 00]).unwrap())
    }

    /// Indicates that the server (not the client) specified in the ticket has
    /// been determined by policy of the realm to be a suitable recipient of
    /// delegation. A client can use the presence of this flag to help it decide
    /// whether to delegate credentials (either grant a proxy or a forwarded
    /// TGT) to this server. The client is free to ignore the value of this
    /// flag. When setting this flag, an administrator should consider the
    /// security and placement of the server on which the service will run, as
    /// well as whether the service requires the use of delegated credentials.
    pub fn ok_as_delegate() -> Self {
        Self(KerberosFlags::from_slice(&[0x80, 0]).unwrap())
    }
}

/// Initial ticket request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 10), delegate)]
pub struct AsReq(pub KdcReq);

/// Additional ticket request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 12), delegate)]
pub struct TgsReq(pub KdcReq);

/// The ticket request struct.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcReq {
    /// The protocol version number.
    #[rasn(tag(1))]
    pub pvno: Integer,
    /// The type of a protocol message. It will almost always be the same as the
    /// application identifier associated with a message. It is included to make
    /// the identifier more readily accessible to the application.
    #[rasn(tag(2))]
    pub msg_type: Integer,
    /// Pre-authentication data
    #[rasn(tag(3))]
    pub padata: Option<SequenceOf<PaData>>,
    /// The remaining fields in ticket request. If a checksum is generated for
    /// the request, it is done using this field.
    #[rasn(tag(4))]
    pub req_body: KdcReqBody,
}

/// The remaining fields in ticket request. If a checksum is generated for
/// the request, it is done using this field.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcReqBody {
    /// Flags that the client wants set on the tickets as well as other
    /// information that is to modify the behavior of the KDC.
    #[rasn(tag(0))]
    pub kdc_options: KdcOptions,
    /// The name part of the client's principal identifier.
    #[rasn(tag(1))]
    pub cname: Option<PrincipalName>,
    /// This field specifies the realm part of the server's principal
    /// identifier. In the AS exchange, this is also the realm part of the
    /// client's principal identifier.
    #[rasn(tag(2))]
    pub realm: Realm,
    /// All components of the name part of the server's identity, including
    /// those parts that identify a specific instance of a service.
    ///
    /// May only be absent when the [KdcOptions::enc_tkt_in_skey] option is
    /// specified. If the sname is absent, the name of the server is taken from
    /// the name of the client in the ticket passed as `additional_tickets`.
    #[rasn(tag(3))]
    pub sname: Option<PrincipalName>,
    /// Included in the [AsReq] and [TgsReq] ticket requests when the requested
    /// ticket is to be postdated. It specifies the desired starttime for the
    /// requested ticket. If this field is omitted, then the KDC should use the
    /// current time instead.
    #[rasn(tag(4))]
    pub from: Option<KerberosTime>,
    /// the expiration date requested by the client in a ticket request. It is
    /// not optional, but if the requested endtime is "19700101000000Z", the
    /// requested ticket is to have the maximum endtime permitted according to
    /// KDC policy.
    #[rasn(tag(5))]
    pub till: KerberosTime,
    /// If present, the requested renew-till time sent from a client to the KDC
    /// in a ticket request.
    #[rasn(tag(6))]
    pub rtime: Option<KerberosTime>,
    /// A random number generated by the client. If the same number is included
    /// in the encrypted response from the KDC, it provides evidence that the
    /// response is fresh and has not been replayed by an attacker. Nonces must
    /// never be reused.
    #[rasn(tag(7))]
    pub nonce: u32,
    /// The desired encryption algorithm(s) to be used in the response.
    #[rasn(tag(8))]
    pub etype: SequenceOf<i32>,
    /// The addresses from which the requested ticket is to be valid. Normally
    /// it includes the addresses for the client's host.  If a proxy is
    /// requested, this field will contain other addresses.  The contents of
    /// this field are usually copied by the KDC into the `caddr` field of the
    /// resulting ticket.
    ///
    /// included in the initial request for tickets, and it is optionally
    /// included in requests for additional tickets from the
    /// ticket-granting server.
    #[rasn(tag(9))]
    pub addresses: Option<HostAddresses>,
    /// If present (and it can only be present in the [TgsReq] form), is an
    /// encoding of the desired `authorization_data` encrypted under the
    /// `subkey` key if present in the [Authenticator], or alternatively from
    /// the session key in the TGT (both the Authenticator and TGT come from the
    /// `padata` field in the [TgsReq]). The key usage value used when
    /// encrypting is `5` if a `subkey` key is used, or `4` if the session key
    /// is used.
    #[rasn(tag(10))]
    pub enc_authorization_data: Option<EncryptedData>,
    /// Additional tickets included in the request.
    #[rasn(tag(11))]
    pub additional_tickets: Option<SequenceOf<Ticket>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct KdcOptions(pub KerberosFlags);

impl KdcOptions {
    /// Reserved for future expansion of this field.
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    /// Indicates that the ticket to be issued is to have its forwardable flag
    /// set. It may only be set on the initial request, or in a subsequent
    /// request if the TGT on which it is based is also forwardable.
    pub fn forwardable() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    /// Indicates that this is a request for forwarding. The address(es) of the
    /// host from which the resulting ticket is to be valid are included in the
    /// addresses field of the request.
    pub fn forwarded() -> Self {
        Self(KerberosFlags::from_element(2))
    }

    /// Indicates that the ticket to be issued is to have its proxiable flag
    /// set. It may only be set on the initial request, or a subsequent request
    /// if the TGT on which it is based is also proxiable.
    pub fn proxiable() -> Self {
        Self(KerberosFlags::from_element(4))
    }

    /// Indicates that this is a request for a proxy. This option will only be
    /// honored if the TGT in the request has its PROXIABLE bit set. The
    /// address(es) of the host from which the resulting ticket is to be valid
    /// are included in the addresses field of the request.
    pub fn proxy() -> Self {
        Self(KerberosFlags::from_element(0x8))
    }

    /// Indicates that the ticket to be issued is to have its MAY-POSTDATE flag
    /// set. It may only be set on the initial request, or in a subsequent
    /// request if the TGT on which it is based also has its MAY-POSTDATE flag set.
    pub fn allow_postdate() -> Self {
        Self(KerberosFlags::from_element(0x10))
    }

    /// Indicates that this is a request for a postdated ticket. This option
    /// will only be honored if the TGT on which it is based has its
    /// [TicketFlags::may_postdate] flag set.  The resulting ticket will also
    /// have its INVALID flag set, and that flag may be reset by a subsequent
    /// request to the KDC after the starttime in the ticket has been reached.
    pub fn postdated() -> Self {
        Self(KerberosFlags::from_element(0x20))
    }

    /// Indicates that the ticket to be issued is to have its RENEWABLE flag set.
    /// It may only be set on the initial request, or when the TGT on which the
    /// request is based is also renewable. If this option is requested, then
    /// the `rtime` field in the request contains the desired absolute
    /// expiration time for the ticket.
    pub fn renewable() -> Self {
        Self(KerberosFlags::from_element(0x80))
    }

    pub fn opt_hardware_auth() -> Self {
        Self(KerberosFlags::from_slice(&[0x4, 00]).unwrap())
    }

    /// Indicates checking of the transited field is disabled.  Tickets issued
    /// without the performance of this check will be noted by the reset (0)
    /// value of the [TicketFlags::transited_policy_checked] flag, indicating to
    /// the application server that the transited field must be checked locally.
    /// KDCs are encouraged but not required to honor the
    /// [Self::disable_transited_check] option.
    pub fn disable_transited_check() -> Self {
        Self(KerberosFlags::from_slice(&[0x4, 00, 00, 00]).unwrap())
    }

    /// Indicates that a renewable ticket will be acceptable if a ticket with
    /// the requested life cannot otherwise be provided, in which case a
    /// renewable ticket may be issued with a `renew_till` equal to the
    /// requested endtime.  The value of the renew-till field may still be
    /// limited by local limits, or limits selected by the individual principal
    /// or server.
    pub fn renewable_ok() -> Self {
        Self(KerberosFlags::from_slice(&[0x8, 00, 00, 00]).unwrap())
    }

    /// Indicates that the ticket for the end server is to be encrypted in the
    /// session key from the additional TGT provided.
    pub fn enc_tkt_in_skey() -> Self {
        Self(KerberosFlags::from_slice(&[0x10, 00, 00, 00]).unwrap())
    }

    /// Indicates that the present request is for a renewal. The ticket provided
    /// is encrypted in the secret key for the server on which it is valid. This
    /// option will only be honored if the ticket to be renewed has its
    /// [TicketFlags::renewable] flag set and if the time in its `renew_till`
    /// field has not passed. The ticket to be renewed is passed in the `padata`
    /// field as part of the authentication header.
    pub fn renew() -> Self {
        Self(KerberosFlags::from_slice(&[0x40, 00, 00, 00]).unwrap())
    }

    /// Indicates that the request is to validate a postdated ticket. It will
    /// only be honored if the ticket presented is postdated, presently has its
    /// [TicketFlags::invalid] flag set, and would otherwise be usable at this
    /// time. A ticket cannot be validated before its `start_time`. The ticket
    /// presented for validation is encrypted in the key of the server for which
    /// it is valid and is passed in the `padata` field as part of the
    /// authentication header.
    pub fn validate() -> Self {
        Self(KerberosFlags::from_slice(&[0x80, 00, 00, 00]).unwrap())
    }
}

/// The initial KDC response.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 11), delegate)]
pub struct AsRep(pub KdcRep);

/// Subsequent KDC response.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 13), delegate)]
pub struct TgsRep(pub KdcRep);

/// The main KDC body.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcRep {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// Pre-authentication data.
    #[rasn(tag(2))]
    pub padata: Option<SequenceOf<PaData>>,
    /// The name of the realm in which the client is registered and in which
    /// initial authentication took place.
    #[rasn(tag(3))]
    pub crealm: Realm,
    /// The name part of the client's principal identifier.
    #[rasn(tag(4))]
    pub cname: PrincipalName,
    /// The newly-issued ticket.
    #[rasn(tag(5))]
    pub ticket: Ticket,
    /// the encrypted part of the message follows each appearance of this field.
    /// The key usage value for encrypting this field is 3 in an [AsRep]
    /// message, using the client's long-term key or another key selected via
    /// pre-authentication mechanisms. In a TgsRep message, the key usage value
    /// is 8 if the TGS session key is used, or 9 if a TGS authenticator subkey
    /// is used.
    #[rasn(tag(6))]
    pub enc_part: EncryptedData,
}

/// The encrypted initial request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 25), delegate)]
pub struct EncAsRepPart(pub EncKdcRepPart);

/// The encrypted subsequent request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 26), delegate)]
pub struct EncTgsRepPart(pub EncKdcRepPart);

/// The encrypted part of the [KdcRep] body.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncKdcRepPart {
    /// Used to pass the session key from Kerberos to the application server and
    /// the client.
    #[rasn(tag(0))]
    pub key: EncryptionKey,
    /// The time(s) of the last request by a principal.
    ///
    /// Depending on what information is available, this might be the last time
    /// that a request for a TGT was made, or the last time that a request based
    /// on a TGT was successful. It also might cover all servers for a realm, or
    /// just the particular server. Some implementations MAY display this
    /// information to the user to aid in discovering unauthorized use of one's
    /// identity. It is similar in spirit to the last login time displayed when
    /// logging in to timesharing systems.
    #[rasn(tag(1))]
    pub last_req: LastReq,
    /// A random number generated by the client.  If the same number is included
    /// in the encrypted response from the KDC, it provides evidence that the
    /// response is fresh and has not been replayed by an attacker. Nonces must
    /// never be reused.
    #[rasn(tag(2))]
    pub nonce: u32,
    #[rasn(tag(3))]
    /// The time that the client's secret key is due to expire.
    pub key_expiration: Option<KerberosTime>,
    /// Options that were used or requested when the ticket was issued.
    #[rasn(tag(4))]
    pub flags: TicketFlags,
    /// The time of initial authentication for the named principal. It
    /// is the time of issue for the original ticket on which this ticket is
    /// based.
    ///
    /// It is included in the ticket to provide additional information to the
    /// end service, and to provide the necessary information for implementation
    /// of a "hot list" service at the KDC. An end service that is particularly
    /// paranoid could refuse to accept tickets for which the initial
    /// authentication occurred "too far" in the past.
    #[rasn(tag(5))]
    pub auth_time: KerberosTime,
    /// The time after which the ticket is valid. Together with `end_time`, this
    /// field specifies the life of the ticket. If the `start_time` field is
    /// absent from the ticket, then the `auth_time` field should be used in its
    /// place to determine the life of the ticket.
    #[rasn(tag(6))]
    pub start_time: Option<KerberosTime>,
    /// The time after which the ticket will not be honored (its expiration
    /// time). Note that individual services may place their own limits on the
    /// life of a ticket and MAY reject tickets which have not yet expired. As
    /// such, this is really an upper bound on the expiration time for
    /// the ticket.
    #[rasn(tag(7))]
    pub end_time: KerberosTime,
    /// The maximum `end_time` that may be included in a renewal. It can be
    /// thought of as the absolute expiration time for the ticket, including
    /// all renewals.
    ///
    /// Only present in tickets that have the [TicketFlags::renewable] flag set
    /// in the flags field.
    #[rasn(tag(8))]
    pub renew_till: Option<KerberosTime>,
    /// The realm part of the server's principal identifier.
    #[rasn(tag(9))]
    pub srealm: Realm,
    /// All components of the name part of the server's identity, including
    /// those parts that identify a specific instance of a service.
    #[rasn(tag(10))]
    pub sname: PrincipalName,
    /// Addresses from which the ticket can be used.
    ///
    /// If there are no addresses, the ticket can be used from any location. The
    /// decision by the KDC to issue or by the end server to accept addressless
    /// tickets is a policy decision and is left to the Kerberos and end-service
    /// administrators; they may refuse to issue or accept such tickets. Because
    /// of the wide deployment of network address translation, it is recommended
    /// that policy allow the issue and acceptance of such tickets.
    #[rasn(tag(11))]
    pub caddr: Option<HostAddresses>,
    #[rasn(tag(12))]
    pub encrypted_pa_data: Option<SequenceOf<PaData>>,
}

/// The time of the last request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LastReqValue {
    /// How the `value` field is to be interpreted.
    ///
    /// Negative values indicate that the information pertains only to the
    /// responding server.  Non-negative values pertain to all servers for
    /// the realm.
    ///
    /// - `0` — No information is conveyed by `value`.
    /// - `1` — `value` is the time of last initial request for a TGT.
    /// - `2` — `value ` is the time of last initial request.
    /// - `3` — `value` is the time of issue for the newest TGT used.
    /// - `4` — `value` is the time of the last renewal.
    /// - `5` — `value` is the time of last request (of any type).
    /// - `6` — `value` is the time when the password will expire.
    /// - `7`, then `value` is the time when the account will expire.
    #[rasn(tag(0))]
    pub r#type: i32,
    /// The time of the last request, defined by `type`.
    #[rasn(tag(1))]
    pub value: KerberosTime,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 14))]
pub struct ApReq {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// Options for the request.
    #[rasn(tag(2))]
    pub ap_options: ApOptions,
    /// Ticket authenticating the client to the server.
    #[rasn(tag(3))]
    pub ticket: Ticket,
    /// The encrypted [Authenticator], which includes the client's choice of
    /// a subkey.
    #[rasn(tag(4))]
    pub authenticator: EncryptedData,
}

/// Options for [ApReq].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct ApOptions(pub KerberosFlags);

impl ApOptions {
    /// Reserved for future expansion.
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    /// Indicates that the ticket the client is presenting to a server is
    /// encrypted in the session key from the server's TGT.  When this option is
    /// not specified, the ticket is encrypted in the server's secret key.
    pub fn use_session_key() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    /// Indicates that the client requires mutual authentication, and that it
    /// must respond with a [ApRep] message.
    pub fn mutual_required() -> Self {
        Self(KerberosFlags::from_element(2))
    }
}

/// The authenticator included in the [ApReq]; it certifies to a server that the
/// sender has recent knowledge of the encryption key in the accompanying
/// ticket, to help the server detect replays. It also assists in the selection
/// of a "true session key" to use with the particular session.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 2))]
pub struct Authenticator {
    /// The version number for the format of the authenticator.
    #[rasn(tag(0))]
    pub authenticator_vno: Integer,
    /// The name of the realm in which the client is registered and in which
    /// initial authentication took place.
    #[rasn(tag(1))]
    pub crealm: Realm,
    /// The name part of the client's principal identifier.
    #[rasn(tag(2))]
    pub cname: PrincipalName,
    /// Contains a checksum of the application data that accompanies the
    /// [ApReq], computed using a key usage value of `10` in normal application
    /// exchanges, or `6` when used in the [TgsReq] `padata`.
    #[rasn(tag(3))]
    pub cksum: Option<Checksum>,
    /// The microsecond part of the client's timestamp.  Its value (before
    /// encryption) ranges from 0 to 999999. It often appears along with
    /// `ctime`. The two fields are used together to specify a reasonably
    /// accurate timestamp.
    #[rasn(tag(4))]
    pub cusec: Microseconds,
    /// Contains the current time on the client's host.
    #[rasn(tag(5))]
    pub ctime: KerberosTime,
    /// Contains the client's choice for an encryption key to be used to protect
    /// this specific application session. Unless an application specifies
    /// otherwise, if this field is left out, the session key from the ticket
    /// will be used.
    #[rasn(tag(6))]
    pub subkey: Option<EncryptionKey>,
    /// The initial sequence number to be used by the [KrbPriv] or [KrbSafe]
    /// messages when sequence numbers are used to detect replays.
    #[rasn(tag(7))]
    pub seq_number: Option<u32>,
    /// Restrictions on any authority obtained on the basis of authentication
    /// using the ticket. It is possible for any principal in possession of
    /// credentials to add entries to the authorization data field since these
    /// entries further restrict what can be done with the ticket. Such
    /// additions can be made by specifying the additional entries when a new
    /// ticket is obtained during the TGS exchange, or they MAY be added during
    /// chained delegation using the authorization data field of
    /// the authenticator.
    #[rasn(tag(8))]
    pub authorization_data: Option<AuthorizationData>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 15))]
pub struct ApRep {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// The encrypted version of [EncApRepPart].
    #[rasn(tag(2))]
    pub enc_part: EncryptedData,
}

/// The body of [ApRep].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 27))]
pub struct EncApRepPart {
    /// Contains the current time on the client's host.
    #[rasn(tag(0))]
    pub ctime: KerberosTime,
    /// The microsecond part of the client's timestamp.  Its value (before
    /// encryption) ranges from 0 to 999999. It often appears along with
    /// `ctime`. The two fields are used together to specify a reasonably
    /// accurate timestamp.
    #[rasn(tag(1))]
    pub cusec: Microseconds,
    #[rasn(tag(2))]
    pub subkey: Option<EncryptionKey>,
    /// The sequence number of the message.
    #[rasn(tag(3))]
    pub seq_number: Option<u32>,
}

/// Message containing user data along with a collision-proof checksum keyed
/// with the last encryption key negotiated via subkeys, or with the session key
/// if no negotiation has occurred.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 20))]
pub struct KrbSafe {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// The body of the message.
    #[rasn(tag(2))]
    pub body: KrbSafeBody,
    /// the checksum of the application data, computed with a key usage value
    /// of `15`.
    #[rasn(tag(3))]
    pub cksum: Checksum,
}

/// The body of [KrbSafe].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KrbSafeBody {
    /// Application-specific data that is being passed from the sender to the recipient.
    #[rasn(tag(0))]
    pub user_data: OctetString,
    /// The current time as known by the sender of the message.
    ///
    /// By checking the timestamp, the recipient of the message is able to make
    /// sure that it was recently generated, and is not a replay.
    #[rasn(tag(1))]
    pub timestamp: Option<KerberosTime>,
    /// The microsecond part of the timestamp.
    #[rasn(tag(2))]
    pub usec: Option<Microseconds>,
    /// The sequence number of the message.
    #[rasn(tag(3))]
    pub seq_number: Option<u32>,
    #[rasn(tag(4))]
    pub s_address: HostAddress,
    #[rasn(tag(5))]
    pub r_address: Option<HostAddress>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 21))]
pub struct KrbPriv {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// The encrypted body of [KrbPriv].
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

/// The body of [KrbPriv].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 28))]
pub struct EncKrbPrivPart {
    /// Application-specific data that is being passed from the sender to the recipient.
    #[rasn(tag(0))]
    pub user_data: OctetString,
    /// The current time as known by the sender of the message.
    ///
    /// By checking the timestamp, the recipient of the message is able to make
    /// sure that it was recently generated, and is not a replay.
    #[rasn(tag(1))]
    pub timestamp: Option<KerberosTime>,
    /// The microsecond part of the timestamp.
    #[rasn(tag(2))]
    pub usec: Option<Microseconds>,
    /// The sequence number of the message.
    #[rasn(tag(3))]
    pub seq_number: Option<u32>,
    /// The address in use by the sender of the message.
    #[rasn(tag(4))]
    pub sender_address: HostAddress,
    /// The address in use by the recipient of the message.
    ///
    /// It may be omitted for some uses (such as broadcast protocols), but the
    /// recipient may arbitrarily reject such messages. This field, along with
    /// `sender_address`, can be used to help detect messages that have been
    /// incorrectly or maliciously delivered to the wrong recipient.
    #[rasn(tag(5))]
    pub recipient_address: Option<HostAddress>,
}

/// Message that can be used to send Kerberos credentials from one principal
/// to another.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 22))]
pub struct KrbCred {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// Tickets obtained from the KDC specifically for use by the
    /// intended recipient.
    #[rasn(tag(2))]
    pub tickets: SequenceOf<Ticket>,
    /// The encrypted body of [EncKrbCredPart].
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

/// The body of [KrbCred].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 29))]
pub struct EncKrbCredPart {
    /// Sequence of tickets to be sent and information needed to use the
    /// tickets, including the session key from each.
    #[rasn(tag(0))]
    pub ticket_info: SequenceOf<KrbCredInfo>,
    /// A random number generated by the client.  If the same number is included
    /// in the encrypted response from the KDC, it provides evidence that the
    /// response is fresh and has not been replayed by an attacker. Nonces must
    /// never be reused.
    #[rasn(tag(1))]
    pub nonce: Option<u32>,
    /// The current time as known by the sender of the message.
    ///
    /// By checking the timestamp, the recipient of the message is able to make
    /// sure that it was recently generated, and is not a replay.
    #[rasn(tag(2))]
    pub timestamp: Option<KerberosTime>,
    /// The microsecond part of the timestamp.
    #[rasn(tag(3))]
    pub usec: Option<Microseconds>,
    /// The address in use by the sender of the message.
    #[rasn(tag(4))]
    pub sender_address: Option<HostAddress>,
    /// The address in use by the recipient of the message.
    #[rasn(tag(5))]
    pub recipient_address: Option<HostAddress>,
}

/// The tickets and information needed to use them in [KrbCred].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KrbCredInfo {
    /// The session key from the sender to the intended recipient.
    #[rasn(tag(0))]
    pub key: EncryptionKey,
    /// The realm of the delegated principal identity.
    #[rasn(tag(1))]
    pub prealm: Option<Realm>,
    /// The name of the delegated principal identity.
    #[rasn(tag(2))]
    pub pname: Option<PrincipalName>,
    /// Options that were used or requested when the ticket was issued.
    #[rasn(tag(3))]
    pub flags: Option<TicketFlags>,
    /// The time of initial authentication for the named principal. It
    /// is the time of issue for the original ticket on which this ticket is
    /// based.
    #[rasn(tag(4))]
    pub auth_time: Option<KerberosTime>,
    /// The time after which the ticket is valid. Together with `end_time`, this
    /// field specifies the life of the ticket. If the `start_time` field is
    /// absent from the ticket, then the `auth_time` field should be used in its
    /// place to determine the life of the ticket.
    #[rasn(tag(5))]
    pub start_time: Option<KerberosTime>,
    /// The time after which the ticket will not be honored (its expiration
    /// time). Note that individual services may place their own limits on the
    /// life of a ticket and MAY reject tickets which have not yet expired. As
    /// such, this is really an upper bound on the expiration time for
    /// the ticket.
    #[rasn(tag(6))]
    pub end_time: Option<KerberosTime>,
    /// The maximum `end_time` that may be included in a renewal. It can be
    /// thought of as the absolute expiration time for the ticket, including
    /// all renewals.
    #[rasn(tag(7))]
    pub renew_till: Option<KerberosTime>,
    /// The realm part of the server's principal identifier.
    #[rasn(tag(8))]
    pub srealm: Option<Realm>,
    /// All components of the name part of the server's identity, including
    /// those parts that identify a specific instance of a service.
    #[rasn(tag(9))]
    pub sname: Option<PrincipalName>,
    /// Addresses from which the ticket can be used.
    #[rasn(tag(10))]
    pub caddr: Option<HostAddresses>,
}

/// An error from Kerberos.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 30))]
pub struct KrbError {
    /// The protocol version number.
    #[rasn(tag(0))]
    pub pvno: Integer,
    /// The type of a protocol message.  It will almost always be the same as
    /// the application identifier associated with a message. It is included to
    /// make the identifier more readily accessible to the application.
    #[rasn(tag(1))]
    pub msg_type: Integer,
    /// Contains the current time on the client's host.
    #[rasn(tag(2))]
    pub ctime: Option<KerberosTime>,
    /// The microsecond part of the client's timestamp.  Its value (before
    /// encryption) ranges from 0 to 999999. It often appears along with
    /// `ctime`. The two fields are used together to specify a reasonably
    /// accurate timestamp.
    #[rasn(tag(3))]
    pub cusec: Option<Microseconds>,
    /// The current time on the server.
    #[rasn(tag(4))]
    pub stime: KerberosTime,
    /// The microsecond part of the server's timestamp.
    #[rasn(tag(5))]
    pub susec: Microseconds,
    /// The error code returned by Kerberos or the server when a request fails.
    #[rasn(tag(6))]
    pub error_code: i32,
    /// The name of the realm in which the client is registered and in which
    /// initial authentication took place.
    #[rasn(tag(7))]
    pub crealm: Option<Realm>,
    /// The name part of the client's principal identifier.
    #[rasn(tag(8))]
    pub cname: Option<PrincipalName>,
    /// The realm of the issuing principal.
    #[rasn(tag(9))]
    pub realm: Realm,
    /// The name of the issuing principal.
    #[rasn(tag(10))]
    pub sname: PrincipalName,
    /// Additional text to help explain the error code associated with the
    /// failed request.
    #[rasn(tag(11))]
    pub e_text: Option<KerberosString>,
    /// Additional data about the error for use by the application to help it
    /// recover from or handle the error.
    #[rasn(tag(12))]
    pub e_data: Option<OctetString>,
}

pub type TypedData = SequenceOf<TypedDataItem>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedDataItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub value: Option<OctetString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaEncTsEnc {
    #[rasn(tag(0))]
    pub patimestamp: KerberosTime,
    #[rasn(tag(1))]
    pub pausec: Option<Microseconds>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtypeInfoEntry {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub salt: Option<OctetString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtypeInfo2Entry {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub salt: Option<KerberosString>,
    #[rasn(tag(2))]
    pub s2kparams: Option<OctetString>,
}

/// Provides a means for Kerberos principal credentials to embed within
/// themselves privilege attributes and other mechanisms for positive
/// authorization, amplifying the privileges of the principal beyond what can be
/// done using credentials without such an a-data element.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdKdcIssued {
    /// A cryptographic checksum computed over the DER encoding of the
    /// [AuthorizationData] in the `elements` field, keyed with the session key.
    /// Its checksumtype is the mandatory checksum type for the encryption type
    /// of the session key, and its key usage value is `19`.
    #[rasn(tag(0))]
    pub ad_checksum: Checksum,
    /// The realm of the issuing principal if different from that of the
    /// KDC itself.
    #[rasn(tag(1))]
    pub realm: Option<Realm>,
    /// The name of the issuing principal if different from that of the
    /// KDC itself.
    #[rasn(tag(2))]
    pub sname: Option<PrincipalName>,
    /// A sequence of authorization data elements issued by the KDC.
    #[rasn(tag(3))]
    pub elements: AuthorizationData,
}

/// Used to implement an "or" operation by setting the condition-count field to
/// `1`, and it may specify an "and" operation by setting the condition count to
/// the number of embedded elements.  Application servers that do not implement
/// this element MUST reject tickets that contain authorization data elements of
/// this type.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdAndOr {
    /// The number elements required to be satisfied.
    #[rasn(tag(0))]
    pub condition_count: i32,
    /// The authorization elements.
    #[rasn(tag(1))]
    pub elements: AuthorizationData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdCammac {
    #[rasn(tag(0))]
    pub elements: AuthorizationData,
    #[rasn(tag(1))]
    pub kdc_verifier: Option<VerifierMac>,
    #[rasn(tag(2))]
    pub svc_verifier: Option<VerifierMac>,
    #[rasn(tag(3))]
    pub other_verifiers: SequenceOf<Verifier>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[rasn(choice)]
pub enum Verifier {
    Mac(VerifierMac),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerifierMac {
    #[rasn(tag(0))]
    pub identifier: Option<PrincipalName>,
    #[rasn(tag(1))]
    pub kvno: Option<u32>,
    #[rasn(tag(2))]
    pub enctype: Option<i32>,
    #[rasn(tag(3))]
    pub mac: Checksum,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct AdLoginAlias {
    #[rasn(tag(0))]
    pub login_aliases: SequenceOf<PrincipalName>,
}

impl AdLoginAlias {
    pub fn new(login_aliases: SequenceOf<PrincipalName>) -> Self {
        Self { login_aliases }
    }
}
