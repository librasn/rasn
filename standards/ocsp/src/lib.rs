//! # Online Certificate Status Protocol
//! This crate provides an implementation of the data types for [RFC 6960], also
//! known as Online Certificate Status Protocol (OCSP).
//!
//! Like other `rasn` core crates, this doesn't provide a OCSP client or server
//! but provides the core data types used to be able to create your own clients
//! and servers.
#![no_std]

use rasn::prelude::*;

use rasn_pkix::{
    AlgorithmIdentifier, AuthorityInfoAccessSyntax, Certificate, CertificateSerialNumber,
    CrlReason, Extensions, GeneralName, Name,
};

pub type Version = Integer;
pub type Nonce = OctetString;
pub type KeyHash = OctetString;
pub type UnknownInfo = ();
pub type ArchiveCutoff = GeneralizedTime;
pub type AcceptableResponses = SequenceOf<ObjectIdentifier>;

/// The (optionally signed) OCSP request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcspRequest {
    /// The body of the request.
    pub tbs_request: TbsRequest,
    /// The signature, if present.
    #[rasn(tag(explicit(0)))]
    pub optional_signature: Option<Signature>,
}

/// The body of the [OcspRequest].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TbsRequest {
    /// The version of the protocol.
    #[rasn(tag(explicit(0)), default)]
    pub version: Version,
    /// The name of the OCSP requestor.
    #[rasn(tag(explicit(1)))]
    pub requestor_name: Option<GeneralName>,
    /// One or more single certificate status requests.
    pub request_list: SequenceOf<Request>,
    /// Extensions applicable to the requests.
    #[rasn(tag(explicit(2)))]
    pub request_extensions: Option<Extensions>,
}

/// The signature for an [OcspRequest].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature {
    /// The algorithm used to sign the request.
    pub signature_algorithm: AlgorithmIdentifier,
    /// the actual signature contents
    pub signature: BitString,
    /// Certificates the server needs to verify the signed response (normally up
    /// to but not including the client's root certificate).
    #[rasn(tag(explicit(0)))]
    pub certs: Option<SequenceOf<Certificate>>,
}

/// A single request.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Request {
    /// The identifier of a target certificate.
    pub req_cert: CertId,
    /// Extensions applicable to this single certificate status request.
    #[rasn(tag(explicit(0)))]
    pub single_request_extensions: Option<Extensions>,
}

/// The identifier of the certificate.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CertId {
    /// The hash algorithm used to generate the `issuer_name_hash` and
    /// `issuer_key_hash` values.
    pub hash_algorithm: AlgorithmIdentifier,
    /// The hash of the issuer's distinguished name.
    ///
    /// The hash shall be calculated over the DER encoding of the issuer's name
    /// field in the certificate being checked.
    pub issuer_name_hash: OctetString,
    /// The hash of the issuer's public key.
    ///
    /// The hash shall be calculated over the value (excluding tag and length)
    /// of the subject public key field in the issuer's certificate.
    pub issuer_key_hash: OctetString,
    /// The serial number of the certificate for which status is
    /// being requested.
    pub serial_number: CertificateSerialNumber,
}

/// A confirmation response.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcspResponse {
    /// The processing status of the prior request.
    pub status: OcspResponseStatus,
    /// The body of the response.
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

/// The body of a [OcspResponse].
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResponseBytes {
    /// The OID identifying the type of response.
    pub r#type: ObjectIdentifier,
    /// The DER encoded response.
    pub response: OctetString,
}

/// A Basic OCSP response.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicOcspResponse {
    /// The response body.
    pub tbs_response_data: ResponseData,
    /// The algorithm used to generate the signature.
    pub signature_algorithm: AlgorithmIdentifier,
    /// The actual signature of the response.
    ///
    /// The value for shall be computed on the hash of the DER encoding
    /// of [ResponseData].
    pub signature: BitString,
    /// certificates in that help the OCSP client verify the
    /// responder's signature.
    #[rasn(tag(explicit(0)))]
    pub certs: Option<SequenceOf<Certificate>>,
}

/// The body of [BasicOcspResponse].
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
    /// The identifier of the certificate.
    pub cert_id: CertId,
    /// The revocation status of the certificate.
    pub cert_status: CertStatus,
    /// The start of the validity interval of the response.
    pub this_update: GeneralizedTime,
    /// The end of the validity interval of the response.
    #[rasn(tag(explicit(0)))]
    pub next_update: Option<GeneralizedTime>,
    /// The optional extensions.
    #[rasn(tag(explicit(1)))]
    pub single_extensions: Option<Extensions>,
}

/// The revocation status of the certificate.
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrlId {
    #[rasn(tag(explicit(0)))]
    pub url: Option<Ia5String>,
    #[rasn(tag(explicit(1)))]
    pub num: Option<Integer>,
    #[rasn(tag(explicit(2)))]
    pub time: Option<GeneralizedTime>,
}
