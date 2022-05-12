#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod algorithms;
pub mod authenticode;
pub mod firmware_wrapper;
pub mod pkcs7_compat;

use alloc::boxed::Box;

use rasn::prelude::*;
pub use rasn_pkix::{
    AlgorithmIdentifier, Attribute, Certificate, CertificateList, CertificateSerialNumber, Name,
    SubjectKeyIdentifier,
};

/// OID of top-level CMS ContentInfo
pub const CONTENT_INFO: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_CONTENTINFO;

/// OID of CMS ContentType
pub const CONTENT_TYPE: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_CONTENT_TYPE;

/// OID of MessageDigest
pub const MESSAGE_DIGEST: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_MESSAGE_DIGEST;

/// OID of SigningTime
pub const SIGNING_TIME: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SIGNING_TIME;

/// OID of CounterSignature
pub const COUNTER_SIGNATURE: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_COUNTER_SIGNATURE;

// content types
/// OID of Data content type
pub const CONTENT_DATA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS7_DATA;

/// OID of SignedData content type
pub const CONTENT_SIGNED_DATA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS7_SIGNED_DATA;

/// OID of EnvelopedData content type
pub const CONTENT_ENVELOPED_DATA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS7_ENVELOPED_DATA;

/// OID of DigestedData content type
pub const CONTENT_DIGESTED_DATA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS7_DIGESTED_DATA;

/// OID of EncryptedData content type
pub const CONTENT_ENCRYPTED_DATA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS7_ENCRYPTED_DATA;

/// OID of AuthenticatedData content type
pub const CONTENT_AUTHENTICATED_DATA: ConstOid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_AUTHENTICATED_DATA;

pub type CmsVersion = Integer;
pub type ContentType = ObjectIdentifier;
pub type DigestAlgorithmIdentifier = AlgorithmIdentifier;
pub type DigestAlgorithmIdentifiers = SetOf<DigestAlgorithmIdentifier>;
pub type SignatureAlgorithmIdentifier = AlgorithmIdentifier;
pub type ContentEncryptionAlgorithmIdentifier = AlgorithmIdentifier;
pub type KeyEncryptionAlgorithmIdentifier = AlgorithmIdentifier;
pub type KeyDerivationAlgorithmIdentifier = AlgorithmIdentifier;
pub type MessageAuthenticationCodeAlgorithm = AlgorithmIdentifier;
pub type CertificateSet = SetOf<CertificateChoices>;
pub type RevocationInfoChoices = SetOf<RevocationInfoChoice>;
pub type SignerInfos = SetOf<SignerInfo>;
pub type SignedAttributes = SetOf<Attribute>;
pub type UnsignedAttributes = SetOf<Attribute>;
pub type SignatureValue = OctetString;
pub type RecipientInfos = SetOf<RecipientInfo>;
pub type UnprotectedAttributes = SetOf<Attribute>;
pub type EncryptedContent = OctetString;
pub type EncryptedKey = OctetString;
pub type RecipientEncryptedKeys = SequenceOf<RecipientEncryptedKey>;
pub type UserKeyingMaterial = OctetString;
pub type Digest = OctetString;
pub type AuthAttributes = SetOf<Attribute>;
pub type UnauthAttributes = SetOf<Attribute>;
pub type MessageAuthenticationCode = OctetString;
pub type Signature = BitString;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthEnvelopedData {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub originator_info: Option<OriginatorInfo>,
    pub recipient_infos: RecipientInfos,
    pub auth_encrypted_content_info: EncryptedContentInfo,
    #[rasn(tag(1))]
    pub auth_attrs: Option<AuthAttributes>,
    pub mac: MessageAuthenticationCode,
    #[rasn(tag(2))]
    pub unauth_attrs: Option<UnauthAttributes>,
}

/// ContentInfo encapsulates a single identified content type, and the
/// identified type may provide further encapsulation.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentInfo {
    pub content_type: ContentType,
    #[rasn(tag(explicit(0)))]
    pub content: Any,
}

/// SignedData represents a signed-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignedData {
    pub version: CmsVersion,
    pub digest_algorithms: DigestAlgorithmIdentifiers,
    pub encap_content_info: EncapsulatedContentInfo,
    #[rasn(tag(0))]
    pub certificates: Option<CertificateSet>,
    #[rasn(tag(1))]
    pub crls: Option<RevocationInfoChoices>,
    pub signer_infos: SignerInfos,
}

/// EnvelopedData represents an enveloped-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvelopedData {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub originator_info: Option<OriginatorInfo>,
    pub recipient_infos: RecipientInfos,
    pub encrypted_content_info: EncryptedContentInfo,
    #[rasn(tag(1))]
    pub unprotected_attrs: Option<UnprotectedAttributes>,
}

/// DigestedData represents a digested-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigestedData {
    pub version: CmsVersion,
    pub digest_algorithm: DigestAlgorithmIdentifier,
    pub encap_content_info: EncapsulatedContentInfo,
    pub digest: Digest,
}

/// EncryptedData represents an encrypted-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptedData {
    pub version: CmsVersion,
    pub encrypted_content_info: EncryptedContentInfo,
    #[rasn(tag(1))]
    pub unprotected_attrs: Option<UnprotectedAttributes>,
}

/// AuthenticatedData represents an authenticated-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthenticatedData {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub originator_info: Option<OriginatorInfo>,
    pub recipient_infos: RecipientInfos,
    pub mac_algorithm: MessageAuthenticationCodeAlgorithm,
    #[rasn(tag(1))]
    pub digest_algorithm: Option<DigestAlgorithmIdentifier>,
    pub encap_content_info: EncapsulatedContentInfo,
    #[rasn(tag(2))]
    pub auth_attrs: Option<AuthAttributes>,
    pub mac: MessageAuthenticationCode,
    #[rasn(tag(3))]
    pub unauth_attrs: Option<UnauthAttributes>,
}

/// The CertificateChoices type gives either a PKCS #6 extended
/// certificate, an X.509 certificate, a version 1 X.509
///  attribute certificate (ACv1) [X.509-97], a version 2 X.509 attribute
/// certificate (ACv2) [X.509-00], or any other certificate format.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum CertificateChoices {
    Certificate(Box<Certificate>),
    #[rasn(tag(0))]
    ExtendedCertificate(Box<ExtendedCertificate>),
    #[rasn(tag(2))]
    V2AttributeCertificate(Box<rasn_pkix::attribute_certificate::AttributeCertificate>),
    #[rasn(tag(3))]
    Other(OtherCertificateFormat),
}

/// OtherCertificateFormat represents a custom certificate format
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OtherCertificateFormat {
    pub other_cert_format: ObjectIdentifier,
    pub other_cert: Any,
}

/// The RevocationInfoChoice type gives a revocation status
/// information alternatives. It is intended that the set contain
/// information sufficient to determine whether the certificates and
/// attribute certificates with which the set is associated are revoked.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum RevocationInfoChoice {
    Crl(CertificateList),
    #[rasn(tag(1))]
    Other(OtherRevocationInfoFormat),
}

/// The OtherRevocationInfoFormat alternative is provided to support any
/// other revocation information format without further modifications to
/// the CMS.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OtherRevocationInfoFormat {
    pub other_rev_info_format: ObjectIdentifier,
    pub other_rev_info: Any,
}

/// The content is represented in the type EncapsulatedContentInfo
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncapsulatedContentInfo {
    pub content_type: ContentType,
    #[rasn(tag(explicit(0)))]
    pub content: Option<OctetString>,
}

/// Per-signer information is represented in the type SignerInfo
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignerInfo {
    pub version: CmsVersion,
    pub sid: SignerIdentifier,
    pub digest_algorithm: DigestAlgorithmIdentifier,
    #[rasn(tag(0))]
    pub signed_attrs: Option<SignedAttributes>,
    pub signature_algorithm: SignatureAlgorithmIdentifier,
    pub signature: SignatureValue,
    #[rasn(tag(1))]
    pub unsigned_attrs: Option<UnsignedAttributes>,
}

/// SignerIdentifier data type represents the choice of signer identifications
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum SignerIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

/// The IssuerAndSerialNumber type identifies a certificate, and thereby
/// an entity and a public key, by the distinguished name of the
/// certificate issuer and an issuer-specific certificate serial number.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IssuerAndSerialNumber {
    pub issuer: Name,
    pub serial_number: CertificateSerialNumber,
}

/// OriginatorInfo optionally provides information about the
/// originator. It is present only if required by the key management algorithm.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OriginatorInfo {
    #[rasn(tag(0))]
    pub certs: Option<CertificateSet>,
    #[rasn(tag(1))]
    pub crls: Option<RevocationInfoChoices>,
}

/// EncryptedContentInfo is the encrypted content information
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptedContentInfo {
    pub content_type: ContentType,
    pub content_encryption_algorithm: ContentEncryptionAlgorithmIdentifier,
    #[rasn(tag(0))]
    pub encrypted_content: Option<EncryptedContent>,
}

/// RecipientInfo is a per-recipient information.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum RecipientInfo {
    KeyTransRecipientInfo(KeyTransRecipientInfo),
    #[rasn(tag(1))]
    KeyAgreeRecipientInfo(KeyAgreeRecipientInfo),
    #[rasn(tag(2))]
    KekRecipientInfo(KekRecipientInfo),
    #[rasn(tag(3))]
    PasswordRecipientInfo(PasswordRecipientInfo),
    #[rasn(tag(4))]
    OtherRecipientInfo(OtherRecipientInfo),
}

/// Per-recipient information using key transport is represented in the
/// type KeyTransRecipientInfo.  Each instance of KeyTransRecipientInfo
/// transfers the content-encryption key to one recipient.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyTransRecipientInfo {
    pub version: CmsVersion,
    pub rid: RecipientIdentifier,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_key: EncryptedKey,
}

/// RecipientIdentifier specifies the recipient's certificate or key that was used by
/// the sender to protect the content-encryption key.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum RecipientIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

/// Recipient information using key agreement is represented in the type
/// KeyAgreeRecipientInfo.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyAgreeRecipientInfo {
    pub version: CmsVersion,
    #[rasn(tag(explicit(0)))]
    pub originator: OriginatorIdentifierOrKey,
    #[rasn(tag(explicit(1)))]
    pub user_keying_material: Option<UserKeyingMaterial>,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub recipient_encrypted_keys: RecipientEncryptedKeys,
}

/// RecipientEncryptedKey includes a recipient identifier and
/// encrypted key for one or more recipients.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecipientEncryptedKey {
    pub key_agree_recipient_identifier: KeyAgreeRecipientIdentifier,
    pub encrypted_key: EncryptedKey,
}

/// KeyAgreeRecipientIdentifier is a CHOICE with two alternatives
/// specifying the recipient's certificate, and thereby the
/// recipient's public key, that was used by the sender to generate a
/// pairwise key-encryption key.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum KeyAgreeRecipientIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    RecipientKeyIdentifier(RecipientKeyIdentifier),
}

/// RecipientKeyIdentifier identifies the recipient's key.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecipientKeyIdentifier {
    pub subject_key_identifier: SubjectKeyIdentifier,
    pub date: Option<GeneralizedTime>,
    pub other: Option<OtherKeyAttribute>,
}

/// Additional information used by the recipient to determine
/// the key-encryption key used by the sender.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OtherKeyAttribute {
    pub key_attr_id: ObjectIdentifier,
    pub key_attr: Option<Any>,
}

/// OriginatorIdentifierOrKey is a CHOICE with three alternatives specifying the
/// sender's key agreement public key.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum OriginatorIdentifierOrKey {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
    #[rasn(tag(1))]
    OriginatorPublicKey(OriginatorPublicKey),
}

/// The OriginatorPublicKey alternative
/// includes the algorithm identifier and sender's key agreement
/// public key.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OriginatorPublicKey {
    pub algorithm: AlgorithmIdentifier,
    pub public_key: BitString,
}

/// Recipient information using previously distributed symmetric keys is
/// represented in the type KEKRecipientInfo.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KekRecipientInfo {
    pub version: CmsVersion,
    pub kek_id: KekIdentifier,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_key: EncryptedKey,
}

/// KekIdentifier specifies a symmetric key-encryption key that was previously
/// distributed to the sender and one or more recipients.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KekIdentifier {
    pub key_identifier: OctetString,
    pub date: Option<GeneralizedTime>,
    pub other: Option<OtherKeyAttribute>,
}

/// Recipient information using a password or shared secret value is
/// represented in the type PasswordRecipientInfo.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PasswordRecipientInfo {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub key_derivation_algorithm: Option<KeyDerivationAlgorithmIdentifier>,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_eey: EncryptedKey,
}

/// Recipient information for additional key management techniques are
/// represented in the type OtherRecipientInfo.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OtherRecipientInfo {
    pub ori_type: ObjectIdentifier,
    pub ori_value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum ExtendedCertificateOrCertificate {
    Certificate(Certificate),
    #[rasn(tag(0))]
    ExtendedCertificate(ExtendedCertificate),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedCertificate {
    pub extended_certificate_info: ExtendedCertificateInfo,
    pub signature_algorithm: SignatureAlgorithmIdentifier,
    pub signature: Signature,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedCertificateInfo {
    pub version: CmsVersion,
    pub certificate: Certificate,
    pub attributes: UnauthAttributes,
}
