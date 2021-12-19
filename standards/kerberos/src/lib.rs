//! # Kerberos Version 5
//! This is an implementation of the data types from [RFC 4120] also known as
//! "Kerberos V5". Kerberos is an authentication framework for verifying
//! identities of "principals" (e.g. a user or network server) on an open
//! unprotected network.
//!
//! This is accomplished without relying on assertions by the host operating
//! system, without basing trust on host addresses, without requiring physical
//! security of all the hosts on the network, and under the assumption that
//! packets traveling along the network can be read, modified, and inserted at
//! will. Kerberos performs authentication under these conditions as a trusted
//! third-party authentication service by using conventional (shared secret
//! key) cryptography.
//!
//! Like other `rasn` core crates this crate does not provide the ability to
//! authenticate on its own, but provides shared data types to create your own
//! Kerberos clients and servers.
//!
//! [RFC 4120]: https://datatracker.ietf.org/doc/html/rfc4120
#![no_std]

#[cfg(feature = "otp")]
pub mod otp;

use rasn::prelude::*;

pub type HostAddresses = SequenceOf<HostAddress>;
pub type AdMandatoryForKdc = AuthorizationData;
pub type AdIfRelevant = AuthorizationData;
pub type EtypeInfo2 = SequenceOf<EtypeInfo2Entry>;
pub type EtypeInfo = SequenceOf<EtypeInfoEntry>;
pub type PaEncTimestamp = EncryptedData;
pub type LastReq = SequenceOf<LastReqItem>;
pub type MethodData = SequenceOf<PaData>;
pub type KerberosString = GeneralString;
pub type ETypeList = SequenceOf<i32>;
pub type Realm = KerberosString;

pub const OID: ConstOid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_KERBEROS_V5;

//     ::= INTEGER (0..999999) -- microseconds
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct Microseconds(pub Integer);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrincipalName {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub string: SequenceOf<KerberosString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct KerberosTime(pub GeneralizedTime);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostAddress {
    #[rasn(tag(0))]
    pub addr_type: i32,
    #[rasn(tag(1))]
    pub address: OctetString,
}

pub type AuthorizationData = SequenceOf<AuthorizationDataItem>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorizationDataItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub data: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaData {
    #[rasn(tag(1))]
    pub r#type: i32,
    #[rasn(tag(2))]
    pub value: OctetString,
}

// KerberosFlags   ::= BIT STRING (SIZE (32..MAX))
pub type KerberosFlags = BitString;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptedData {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub kvno: Option<u32>,
    #[rasn(tag(2))]
    pub cipher: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptionKey {
    #[rasn(tag(0))]
    pub keytype: i32,
    #[rasn(tag(1))]
    pub keyvalue: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Checksum {
    #[rasn(tag(0))]
    pub cksumtype: i32,
    #[rasn(tag(1))]
    pub checksum: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 1))]
pub struct Ticket {
    #[rasn(tag(0))]
    pub tkt_vno: Integer,
    #[rasn(tag(1))]
    pub realm: Realm,
    #[rasn(tag(2))]
    pub sname: PrincipalName,
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 3))]
pub struct EncTicketPart {
    #[rasn(tag(0))]
    pub flags: TicketFlags,
    #[rasn(tag(1))]
    pub key: EncryptionKey,
    #[rasn(tag(2))]
    pub crealm: Realm,
    #[rasn(tag(3))]
    pub cname: PrincipalName,
    #[rasn(tag(4))]
    pub transited: TransitedEncoding,
    #[rasn(tag(5))]
    pub auth_time: KerberosTime,
    #[rasn(tag(6))]
    pub start_time: Option<KerberosTime>,
    #[rasn(tag(7))]
    pub end_time: KerberosTime,
    #[rasn(tag(8))]
    pub renew_till: Option<KerberosTime>,
    #[rasn(tag(9))]
    pub caddr: Option<HostAddresses>,
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct TicketFlags(pub KerberosFlags);

impl TicketFlags {
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    pub fn forwardable() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    pub fn forwarded() -> Self {
        Self(KerberosFlags::from_element(2))
    }

    pub fn proxiable() -> Self {
        Self(KerberosFlags::from_element(3))
    }

    pub fn proxy() -> Self {
        Self(KerberosFlags::from_element(4))
    }

    pub fn may_postdate() -> Self {
        Self(KerberosFlags::from_element(5))
    }

    pub fn postdated() -> Self {
        Self(KerberosFlags::from_element(6))
    }

    pub fn invalid() -> Self {
        Self(KerberosFlags::from_element(7))
    }

    pub fn renewable() -> Self {
        Self(KerberosFlags::from_element(8))
    }

    pub fn initial() -> Self {
        Self(KerberosFlags::from_element(9))
    }

    pub fn pre_authent() -> Self {
        Self(KerberosFlags::from_element(10))
    }

    pub fn hw_authent() -> Self {
        Self(KerberosFlags::from_element(11))
    }

    pub fn transited_policy_checked() -> Self {
        Self(KerberosFlags::from_element(12))
    }

    pub fn ok_as_delegate() -> Self {
        Self(KerberosFlags::from_element(13))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 10), delegate)]
pub struct AsReq(pub KdcReq);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 12), delegate)]
pub struct TgsReq(pub KdcReq);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcReq {
    #[rasn(tag(1))]
    pub pvno: Integer,
    #[rasn(tag(2))]
    pub msg_type: Integer,
    #[rasn(tag(3))]
    pub padata: Option<SequenceOf<PaData>>,
    #[rasn(tag(4))]
    pub req_body: KdcReqBody,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcReqBody {
    #[rasn(tag(0))]
    pub kdc_options: KdcOptions,
    #[rasn(tag(1))]
    pub cname: Option<PrincipalName>,
    #[rasn(tag(2))]
    pub realm: Realm,
    #[rasn(tag(3))]
    pub sname: Option<PrincipalName>,
    #[rasn(tag(4))]
    pub from: Option<KerberosTime>,
    #[rasn(tag(5))]
    pub till: KerberosTime,
    #[rasn(tag(6))]
    pub rtime: Option<KerberosTime>,
    #[rasn(tag(7))]
    pub nonce: u32,
    #[rasn(tag(8))]
    pub etype: SequenceOf<i32>,
    #[rasn(tag(9))]
    pub addresses: Option<HostAddresses>,
    #[rasn(tag(10))]
    pub enc_authorization_data: Option<EncryptedData>,
    #[rasn(tag(11))]
    pub additional_tickets: Option<SequenceOf<Ticket>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct KdcOptions(pub KerberosFlags);

impl KdcOptions {
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    pub fn forwardable() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    pub fn forwarded() -> Self {
        Self(KerberosFlags::from_element(2))
    }

    pub fn proxiable() -> Self {
        Self(KerberosFlags::from_element(3))
    }

    pub fn proxy() -> Self {
        Self(KerberosFlags::from_element(4))
    }

    pub fn allow_postdate() -> Self {
        Self(KerberosFlags::from_element(5))
    }

    pub fn postdated() -> Self {
        Self(KerberosFlags::from_element(6))
    }

    pub fn unused7() -> Self {
        Self(KerberosFlags::from_element(7))
    }

    pub fn renewable() -> Self {
        Self(KerberosFlags::from_element(8))
    }

    pub fn unused9() -> Self {
        Self(KerberosFlags::from_element(9))
    }

    pub fn unused10() -> Self {
        Self(KerberosFlags::from_element(10))
    }

    pub fn opt_hardware_auth() -> Self {
        Self(KerberosFlags::from_element(11))
    }

    pub fn unused12() -> Self {
        Self(KerberosFlags::from_element(12))
    }

    pub fn unused13() -> Self {
        Self(KerberosFlags::from_element(13))
    }

    pub fn unused15() -> Self {
        Self(KerberosFlags::from_element(15))
    }

    pub fn renewable_ok() -> Self {
        Self(KerberosFlags::from_element(27))
    }

    pub fn enc_tkt_in_skey() -> Self {
        Self(KerberosFlags::from_element(28))
    }

    pub fn renew() -> Self {
        Self(KerberosFlags::from_element(30))
    }

    pub fn validate() -> Self {
        Self(KerberosFlags::from_element(31))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 11), delegate)]
pub struct AsRep(pub KdcRep);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 13), delegate)]
pub struct TgsRep(pub KdcRep);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KdcRep {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub padata: Option<SequenceOf<PaData>>,
    #[rasn(tag(3))]
    pub crealm: Realm,
    #[rasn(tag(4))]
    pub cname: PrincipalName,
    #[rasn(tag(5))]
    pub ticket: Ticket,
    #[rasn(tag(6))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 25), delegate)]
pub struct EncAsRepPart(pub EncKdcRepPart);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 26), delegate)]
pub struct EncTgsRepPart(pub EncKdcRepPart);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncKdcRepPart {
    #[rasn(tag(0))]
    pub key: EncryptionKey,
    #[rasn(tag(1))]
    pub last_req: LastReq,
    #[rasn(tag(2))]
    pub nonce: u32,
    #[rasn(tag(3))]
    pub key_expiration: Option<KerberosTime>,
    #[rasn(tag(4))]
    pub flags: TicketFlags,
    #[rasn(tag(5))]
    pub auth_time: KerberosTime,
    #[rasn(tag(6))]
    pub start_time: Option<KerberosTime>,
    #[rasn(tag(7))]
    pub end_time: KerberosTime,
    #[rasn(tag(8))]
    pub renew_till: Option<KerberosTime>,
    #[rasn(tag(9))]
    pub srealm: Realm,
    #[rasn(tag(10))]
    pub sname: PrincipalName,
    #[rasn(tag(11))]
    pub caddr: Option<HostAddresses>,
    #[rasn(tag(12))]
    pub encrypted_pa_data: Option<SequenceOf<PaData>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LastReqItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub value: KerberosTime,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 14))]
pub struct ApReq {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub ap_options: ApOptions,
    #[rasn(tag(3))]
    pub ticket: Ticket,
    #[rasn(tag(4))]
    pub authenticator: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct ApOptions(pub KerberosFlags);

impl ApOptions {
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    pub fn use_session_key() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    pub fn mutual_required() -> Self {
        Self(KerberosFlags::from_element(2))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 2))]
pub struct Authenticator {
    #[rasn(tag(0))]
    pub authenticator_vno: Integer,
    #[rasn(tag(1))]
    pub crealm: Realm,
    #[rasn(tag(2))]
    pub cname: PrincipalName,
    #[rasn(tag(3))]
    pub cksum: Option<Checksum>,
    #[rasn(tag(4))]
    pub cusec: Microseconds,
    #[rasn(tag(5))]
    pub ctime: KerberosTime,
    #[rasn(tag(6))]
    pub subkey: Option<EncryptionKey>,
    #[rasn(tag(7))]
    pub seq_number: Option<u32>,
    #[rasn(tag(8))]
    pub authorization_data: Option<AuthorizationData>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 15))]
pub struct ApRep {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 27))]
pub struct EncApRepPart {
    #[rasn(tag(0))]
    pub ctime: KerberosTime,
    #[rasn(tag(1))]
    pub cusec: Microseconds,
    #[rasn(tag(2))]
    pub subkey: Option<EncryptionKey>,
    #[rasn(tag(3))]
    pub seq_number: Option<u32>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 20))]
pub struct KrbSafe {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub safe_body: KrbSafeBody,
    #[rasn(tag(3))]
    pub cksum: Checksum,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KrbSafeBody {
    #[rasn(tag(0))]
    pub user_data: OctetString,
    #[rasn(tag(1))]
    pub timestamp: Option<KerberosTime>,
    #[rasn(tag(2))]
    pub usec: Option<Microseconds>,
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
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 28))]
pub struct EncKrbPrivPart {
    #[rasn(tag(0))]
    pub user_data: OctetString,
    #[rasn(tag(1))]
    pub timestamp: Option<KerberosTime>,
    #[rasn(tag(2))]
    pub usec: Option<Microseconds>,
    #[rasn(tag(3))]
    pub seq_number: Option<u32>,
    #[rasn(tag(4))]
    pub s_address: HostAddress,
    #[rasn(tag(5))]
    pub r_address: Option<HostAddress>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 22))]
pub struct KrbCred {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub tickets: SequenceOf<Ticket>,
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 29))]
pub struct EncKrbCredPart {
    #[rasn(tag(0))]
    pub ticket_info: SequenceOf<KrbCredInfo>,
    #[rasn(tag(1))]
    pub nonce: Option<u32>,
    #[rasn(tag(2))]
    pub timestamp: Option<KerberosTime>,
    #[rasn(tag(3))]
    pub usec: Option<Microseconds>,
    #[rasn(tag(4))]
    pub s_address: Option<HostAddress>,
    #[rasn(tag(5))]
    pub r_address: Option<HostAddress>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KrbCredInfo {
    #[rasn(tag(0))]
    pub key: EncryptionKey,
    #[rasn(tag(1))]
    pub prealm: Option<Realm>,
    #[rasn(tag(2))]
    pub pname: Option<PrincipalName>,
    #[rasn(tag(3))]
    pub flags: Option<TicketFlags>,
    #[rasn(tag(4))]
    pub auth_time: Option<KerberosTime>,
    #[rasn(tag(5))]
    pub start_time: Option<KerberosTime>,
    #[rasn(tag(6))]
    pub end_time: Option<KerberosTime>,
    #[rasn(tag(7))]
    pub renew_till: Option<KerberosTime>,
    #[rasn(tag(8))]
    pub srealm: Option<Realm>,
    #[rasn(tag(9))]
    pub sname: Option<PrincipalName>,
    #[rasn(tag(10))]
    pub caddr: Option<HostAddresses>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(application, 30))]
pub struct KrbError {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub ctime: Option<KerberosTime>,
    #[rasn(tag(3))]
    pub cusec: Option<Microseconds>,
    #[rasn(tag(4))]
    pub stime: KerberosTime,
    #[rasn(tag(5))]
    pub susec: Microseconds,
    #[rasn(tag(6))]
    pub error_code: i32,
    #[rasn(tag(7))]
    pub crealm: Option<Realm>,
    #[rasn(tag(8))]
    pub cname: Option<PrincipalName>,
    #[rasn(tag(9))]
    pub realm: Realm,
    #[rasn(tag(10))]
    pub sname: PrincipalName,
    #[rasn(tag(11))]
    pub e_text: Option<KerberosString>,
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdKdcIssued {
    #[rasn(tag(0))]
    pub ad_checksum: Checksum,
    #[rasn(tag(1))]
    pub realm: Option<Realm>,
    #[rasn(tag(2))]
    pub sname: Option<PrincipalName>,
    #[rasn(tag(3))]
    pub elements: AuthorizationData,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdAndOr {
    #[rasn(tag(0))]
    pub condition_count: i32,
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
