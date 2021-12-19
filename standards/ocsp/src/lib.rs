#![no_std]

use rasn::prelude::*;

use rasn_pkix::{
    AlgorithmIdentifier, AuthorityInfoAccessSyntax, Certificate, CertificateSerialNumber,
    CrlReason, Extensions, GeneralName, Name,
};

pub type Version = Integer;
pub type KeyHash = OctetString;
pub type UnknownInfo = ();
pub type ArchiveCutoff = GeneralizedTime;
pub type AcceptableResponses = SequenceOf<ObjectIdentifier>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcspRequest {
    pub tbs_request: TbsRequest,
    #[rasn(tag(explicit(0)))]
    pub optional_signature: Option<Signature>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TbsRequest {
    #[rasn(tag(explicit(0)), default)]
    pub version: Version,
    #[rasn(tag(explicit(1)))]
    pub requestor_name: Option<GeneralName>,
    pub request_list: SequenceOf<Request>,
    #[rasn(tag(explicit(2)))]
    pub request_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature {
    pub signature_algorithm: AlgorithmIdentifier,
    pub signature: BitString,
    #[rasn(tag(explicit(0)))]
    pub certs: Option<SequenceOf<Certificate>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Request {
    pub req_cert: CertId,
    #[rasn(tag(explicit(0)))]
    pub single_request_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CertId {
    pub hash_algorithm: AlgorithmIdentifier,
    pub issuer_name_hash: OctetString,
    pub issuer_key_hash: OctetString,
    pub serial_number: CertificateSerialNumber,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcspResponse {
    pub status: OcspResponseStatus,
    #[rasn(tag(explicit(0)))]
    pub bytes: Option<ResponseBytes>,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum OcspResponseStatus {
    /// Response has valid confirmations.
    Successful = 0,
    /// Illegal confirmation request.
    MalformedRequest = 1,
    /// Internal error in issuer.
    InternalError = 2,
    /// Try again later.
    TryLater = 3,
    /// Must sign the request.
    SigRequired = 5,
    /// Request unauthorized.
    Unauthorized = 6,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResponseBytes {
    pub r#type: ObjectIdentifier,
    pub response: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicOcspResponse {
    pub tbs_response_data: ResponseData,
    pub signature_algorithm: AlgorithmIdentifier,
    pub signature: BitString,
    #[rasn(tag(explicit(0)))]
    pub certs: Option<SequenceOf<Certificate>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResponseData {
    #[rasn(tag(explicit(0), default))]
    pub version: Version,
    pub responder_id: ResponderId,
    pub produced_at: GeneralizedTime,
    pub responses: SequenceOf<SingleResponse>,
    #[rasn(tag(explicit(1)))]
    pub response_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum ResponderId {
    #[rasn(tag(explicit(1)))]
    ByName(Name),
    #[rasn(tag(explicit(2)))]
    ByKey(KeyHash),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SingleResponse {
    pub cert_id: CertId,
    pub cert_status: CertStatus,
    pub this_update: GeneralizedTime,
    #[rasn(tag(explicit(0)))]
    pub next_update: Option<GeneralizedTime>,
    #[rasn(tag(explicit(1)))]
    pub single_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum CertStatus {
    #[rasn(tag(0))]
    Good,
    #[rasn(tag(1))]
    Revoked(RevokedInfo),
    #[rasn(tag(2))]
    Unknown(UnknownInfo),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RevokedInfo {
    pub revocation_time: GeneralizedTime,
    #[rasn(tag(explicit(0)))]
    pub revocation_reason: Option<CrlReason>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceLocator {
    pub issuer: Name,
    pub locator: AuthorityInfoAccessSyntax,
}
