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

pub const KERBEROS_OID: ConstOid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_KERBEROS_V5;

//     ::= INTEGER (0..999999) -- microseconds
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
pub struct Microseconds(pub Integer);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
pub struct KerberosString(pub GeneralString);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
pub struct Realm(pub KerberosString);

#[derive(AsnType, Decode, Encode)]
pub struct PrincipalName {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub string: SequenceOf<KerberosString>,
}

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
pub struct KerberosTime(pub GeneralizedTime);

#[derive(AsnType, Decode, Encode)]
pub struct HostAddress {
    #[rasn(tag(0))]
    pub addr_type: i32,
    #[rasn(tag(1))]
    pub address: OctetString,
}

pub type AuthorizationData = SequenceOf<AuthorizationDataItem>;

#[derive(AsnType, Decode, Encode)]
pub struct AuthorizationDataItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub data: OctetString,
}

#[derive(AsnType, Decode, Encode)]
pub struct PaData {
    #[rasn(tag(1))]
    pub r#type: i32,
    #[rasn(tag(2))]
    value: OctetString,
}

// KerberosFlags   ::= BIT STRING (SIZE (32..MAX))
pub type KerberosFlags = BitString;

#[derive(AsnType, Decode, Encode)]
pub struct EncryptedData {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub kvno: Option<u32>,
    #[rasn(tag(2))]
    pub cipher: OctetString,
}

#[derive(AsnType, Decode, Encode)]
pub struct EncryptionKey {
    #[rasn(tag(0))]
    pub keytype: i32,
    #[rasn(tag(1))]
    pub keyvalue: OctetString,
}

#[derive(AsnType, Decode, Encode)]
pub struct Checksum {
    #[rasn(tag(0))]
    pub cksumtype: i32,
    #[rasn(tag(1))]
    pub checksum: OctetString,
}

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
pub struct TransitedEncoding {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub contents: OctetString,
}

// reserved(0),
// forwardable(1),
// forwarded(2),
// proxiable(3),
// proxy(4),
// may-postdate(5),
// postdated(6),
// invalid(7),
// renewable(8),
// initial(9),
// pre-authent(10),
// hw-authent(11),
// transited-policy-checked(12),
// ok-as-delegate(13)
pub type TicketFlags = KerberosFlags;

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 10), delegate)]
pub struct AsReq(pub KdcReq);

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 12), delegate)]
pub struct TgsReq(pub KdcReq);

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

// reserved(0),
// forwardable(1),
// forwarded(2),
// proxiable(3),
// proxy(4),
// allow-postdate(5),
// postdated(6),
// unused7(7),
// renewable(8),
// unused9(9),
// unused10(10),
// opt-hardware-auth(11),
// unused12(12),
// unused13(13),
// 15 is reserved for canonicalize
// unused15(15),
// 26 was unused in 1510
// disable-transited-check(26),
//
// renewable-ok(27),
// enc-tkt-in-skey(28),
// renew(30),
// validate(31)
pub type KdcOptions = KerberosFlags;

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 11), delegate)]
pub struct AsRep(pub KdcRep);

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 13), delegate)]
pub struct TgsRep(pub KdcRep);

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 25), delegate)]
pub struct EncAsRepPart(pub EncKdcRepPart);

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 26), delegate)]
pub struct EncTgsRepPart(pub EncKdcRepPart);

#[derive(AsnType, Decode, Encode)]
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
}

#[derive(AsnType, Decode, Encode)]
pub struct LastReqItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub value: KerberosTime,
}

#[derive(AsnType, Decode, Encode)]
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

// reserved(0),
// use-session-key(1),
// mutual-required(2)
pub type ApOptions = KerberosFlags;

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 15))]
pub struct ApRep {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(2))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
#[rasn(tag(application, 21))]
pub struct KrbPriv {
    #[rasn(tag(0))]
    pub pvno: Integer,
    #[rasn(tag(1))]
    pub msg_type: Integer,
    #[rasn(tag(3))]
    pub enc_part: EncryptedData,
}

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
pub struct TypedDataItem {
    #[rasn(tag(0))]
    pub r#type: i32,
    #[rasn(tag(1))]
    pub value: Option<OctetString>,
}

#[derive(AsnType, Decode, Encode)]
pub struct PaEncTsEnc {
    #[rasn(tag(0))]
    pub patimestamp: KerberosTime,
    #[rasn(tag(1))]
    pub pausec: Option<Microseconds>,
}

#[derive(AsnType, Decode, Encode)]
pub struct EtypeInfoEntry {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub salt: Option<OctetString>,
}

#[derive(AsnType, Decode, Encode)]
pub struct EtypeInfo2Entry {
    #[rasn(tag(0))]
    pub etype: i32,
    #[rasn(tag(1))]
    pub salt: Option<KerberosString>,
    #[rasn(tag(2))]
    pub s2kparams: Option<OctetString>,
}

#[derive(AsnType, Decode, Encode)]
pub struct AdKdcIssued {
    #[rasn(tag(0))]
    pub ad_checksum: Checksum,
    #[rasn(tag(1))]
    pub i_realm: Option<Realm>,
    #[rasn(tag(2))]
    pub i_sname: Option<PrincipalName>,
    #[rasn(tag(3))]
    pub elements: AuthorizationData,
}

#[derive(AsnType, Decode, Encode)]
pub struct AdAndOr {
    #[rasn(tag(0))]
    pub condition_count: i32,
    #[rasn(tag(1))]
    pub elements: AuthorizationData,
}
