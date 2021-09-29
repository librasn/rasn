//! # Cryptographic Message Syntax
//!
//! `rasn-cms` is an implementation of the data types defined in IETF
//! [RFC 5652] also known as CMS or PKCS#7. This does not provide an implementation of a
//! CMS generator or validator, `rasn-cms` provides an
//! implementation of the underlying data types used to decode and
//! encode the CMS structures from/to DER or BER.
//!
//! [RFC 5652]: https://datatracker.ietf.org/doc/html/rfc5652

use rasn::prelude::*;
use rasn_pkix::{
    AlgorithmIdentifier, Attribute, Certificate, CertificateList, CertificateSerialNumber, Name,
    SubjectKeyIdentifier,
};

pub const OID_CONTENT_INFO: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 16, 1, 6]);
pub const OID_CONTENT_TYPE: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 3]);
pub const OID_MESSAGE_DIGEST: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 4]);
pub const OID_SIGNING_TIME: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 5]);
pub const OID_COUNTER_SIGNATURE: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 6]);

// content types
pub const OID_CONTENT_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 7, 1]);
pub const OID_CONTENT_SIGNED_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 7, 2]);
pub const OID_CONTENT_ENVELOPED_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 7, 3]);
pub const OID_CONTENT_DIGESTED_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 7, 5]);
pub const OID_CONTENT_ENCRYPTED_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 7, 6]);
pub const OID_CONTENT_AUTHENTICATED_DATA: ConstOid = ConstOid(&[1, 2, 840, 113549, 1, 9, 16, 1, 2]);

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

// ContentInfo ::= SEQUENCE {
//   contentType ContentType,
//   content [0] EXPLICIT ANY DEFINED BY contentType }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContentInfo {
    pub content_type: ContentType,
    #[rasn(tag(explicit(0)))]
    pub content: Any,
}

// SignedData ::= SEQUENCE {
//   version CMSVersion,
//   digestAlgorithms DigestAlgorithmIdentifiers,
//   encapContentInfo EncapsulatedContentInfo,
//   certificates [0] IMPLICIT CertificateSet OPTIONAL,
//   crls [1] IMPLICIT RevocationInfoChoices OPTIONAL,
//   signerInfos SignerInfos }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

// EnvelopedData ::= SEQUENCE {
//   version CMSVersion,
//   originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,
//   recipientInfos RecipientInfos,
//   encryptedContentInfo EncryptedContentInfo,
//   unprotectedAttrs [1] IMPLICIT UnprotectedAttributes OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnvelopedData {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub originator_info: Option<OriginatorInfo>,
    pub recipient_infos: RecipientInfos,
    pub encrypted_content_info: EncryptedContentInfo,
    #[rasn(tag(1))]
    pub unprotected_attrs: Option<UnprotectedAttributes>,
}

// DigestedData ::= SEQUENCE {
//   version CMSVersion,
//   digestAlgorithm DigestAlgorithmIdentifier,
//   encapContentInfo EncapsulatedContentInfo,
//   digest Digest }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct DigestedData {
    pub version: CmsVersion,
    pub digest_algorithm: DigestAlgorithmIdentifier,
    pub encap_content_info: EncapsulatedContentInfo,
    pub digest: Digest,
}

// EncryptedData ::= SEQUENCE {
//   version CMSVersion,
//   encryptedContentInfo EncryptedContentInfo,
//   unprotectedAttrs [1] IMPLICIT UnprotectedAttributes OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncryptedData {
    pub version: CmsVersion,
    pub encrypted_content_info: EncryptedContentInfo,
    #[rasn(tag(1))]
    pub unprotected_attrs: Option<UnprotectedAttributes>,
}

// AuthenticatedData ::= SEQUENCE {
//   version CMSVersion,
//   originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,
//   recipientInfos RecipientInfos,
//   macAlgorithm MessageAuthenticationCodeAlgorithm,
//   digestAlgorithm [1] DigestAlgorithmIdentifier OPTIONAL,
//   encapContentInfo EncapsulatedContentInfo,
//   authAttrs [2] IMPLICIT AuthAttributes OPTIONAL,
//   mac MessageAuthenticationCode,
//   unauthAttrs [3] IMPLICIT UnauthAttributes OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

// CertificateChoices ::= CHOICE {
//   certificate Certificate,
//   extendedCertificate [0] IMPLICIT ExtendedCertificate, -- Obsolete
//   v1AttrCert [1] IMPLICIT AttributeCertificateV1,       -- Obsolete
//   v2AttrCert [2] IMPLICIT AttributeCertificateV2,
//   other [3] IMPLICIT OtherCertificateFormat }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum CertificateChoices {
    Certificate(Certificate),
    #[rasn(tag(3))]
    Other(OtherCertificateFormat),
}

// OtherCertificateFormat ::= SEQUENCE {
//   otherCertFormat OBJECT IDENTIFIER,
//   otherCert ANY DEFINED BY otherCertFormat }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OtherCertificateFormat {
    other_cert_format: ObjectIdentifier,
    other_cert: Any,
}

// ExtendedCertificateInfo ::= SEQUENCE {
//   version CMSVersion,
//   certificate Certificate,
//   attributes UnauthAttributes }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedCertificateInfo {
    version: CmsVersion,
    certificate: Certificate,
    attributes: UnauthAttributes,
}

// RevocationInfoChoice ::= CHOICE {
//   crl CertificateList,
//   other [1] IMPLICIT OtherRevocationInfoFormat }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum RevocationInfoChoice {
    Crl(CertificateList),
    #[rasn(tag(1))]
    Other(OtherRevocationInfoFormat),
}

// OtherRevocationInfoFormat ::= SEQUENCE {
//   otherRevInfoFormat OBJECT IDENTIFIER,
//   otherRevInfo ANY DEFINED BY otherRevInfoFormat }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OtherRevocationInfoFormat {
    other_rev_info_format: ObjectIdentifier,
    other_rev_info: Any,
}

// EncapsulatedContentInfo ::= SEQUENCE {
//   eContentType ContentType,
//   eContent [0] EXPLICIT OCTET STRING OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncapsulatedContentInfo {
    pub content_type: ContentType,
    #[rasn(tag(explicit(0)))]
    pub content: Option<OctetString>,
}

// SignerInfo ::= SEQUENCE {
//   version CMSVersion,
//   sid SignerIdentifier,
//   digestAlgorithm DigestAlgorithmIdentifier,
//   signedAttrs [0] IMPLICIT SignedAttributes OPTIONAL,
//   signatureAlgorithm SignatureAlgorithmIdentifier,
//   signature SignatureValue,
//   unsignedAttrs [1] IMPLICIT UnsignedAttributes OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

// SignerIdentifier ::= CHOICE {
//   issuerAndSerialNumber IssuerAndSerialNumber,
//   subjectKeyIdentifier [0] SubjectKeyIdentifier }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum SignerIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

// IssuerAndSerialNumber ::= SEQUENCE {
//   issuer Name,
//   serialNumber CertificateSerialNumber }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct IssuerAndSerialNumber {
    pub issuer: Name,
    pub serial_number: CertificateSerialNumber,
}

// OriginatorInfo ::= SEQUENCE {
//   certs [0] IMPLICIT CertificateSet OPTIONAL,
//   crls [1] IMPLICIT RevocationInfoChoices OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OriginatorInfo {
    #[rasn(tag(0))]
    pub certs: Option<CertificateSet>,
    #[rasn(tag(1))]
    pub crls: Option<RevocationInfoChoices>,
}

// EncryptedContentInfo ::= SEQUENCE {
//   contentType ContentType,
//   contentEncryptionAlgorithm ContentEncryptionAlgorithmIdentifier,
//   encryptedContent [0] IMPLICIT EncryptedContent OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncryptedContentInfo {
    pub content_type: ContentType,
    pub content_encryption_algorithm: ContentEncryptionAlgorithmIdentifier,
    #[rasn(tag(0))]
    pub encrypted_content: Option<EncryptedContent>,
}

// RecipientInfo ::= CHOICE {
//   ktri KeyTransRecipientInfo,
//   kari [1] KeyAgreeRecipientInfo,
//   kekri [2] KEKRecipientInfo,
//   pwri [3] PasswordRecipientinfo,
//   ori [4] OtherRecipientInfo }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

// KeyTransRecipientInfo ::= SEQUENCE {
//   version CMSVersion,  -- always set to 0 or 2
//   rid RecipientIdentifier,
//   keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,
//   encryptedKey EncryptedKey }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyTransRecipientInfo {
    pub version: CmsVersion,
    pub rid: RecipientIdentifier,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_key: EncryptedKey,
}

// RecipientIdentifier ::= CHOICE {
//   issuerAndSerialNumber IssuerAndSerialNumber,
//   subjectKeyIdentifier [0] SubjectKeyIdentifier }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum RecipientIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

// KeyAgreeRecipientInfo ::= SEQUENCE {
//   version CMSVersion,  -- always set to 3
//   originator [0] EXPLICIT OriginatorIdentifierOrKey,
//   ukm [1] EXPLICIT UserKeyingMaterial OPTIONAL,
//   keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,
//   recipientEncryptedKeys RecipientEncryptedKeys }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyAgreeRecipientInfo {
    pub version: CmsVersion,
    #[rasn(tag(explicit(0)))]
    pub originator: OriginatorIdentifierOrKey,
    #[rasn(tag(explicit(1)))]
    pub user_keying_material: Option<UserKeyingMaterial>,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub recipient_encrypted_keys: RecipientEncryptedKeys,
}

// RecipientEncryptedKey ::= SEQUENCE {
//   rid KeyAgreeRecipientIdentifier,
//   encryptedKey EncryptedKey }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipientEncryptedKey {
    pub key_agree_recipient_identifier: KeyAgreeRecipientIdentifier,
    pub encrypted_key: EncryptedKey,
}

// KeyAgreeRecipientIdentifier ::= CHOICE {
//   issuerAndSerialNumber IssuerAndSerialNumber,
//   rKeyId [0] IMPLICIT RecipientKeyIdentifier }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum KeyAgreeRecipientIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    RecipientKeyIdentifier(RecipientKeyIdentifier),
}

// RecipientKeyIdentifier ::= SEQUENCE {
//   subjectKeyIdentifier SubjectKeyIdentifier,
//   date GeneralizedTime OPTIONAL,
//   other OtherKeyAttribute OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipientKeyIdentifier {
    pub subject_key_identifier: SubjectKeyIdentifier,
    pub date: Option<GeneralizedTime>,
    pub other: Option<OtherKeyAttribute>,
}

// OtherKeyAttribute ::= SEQUENCE {
//   keyAttrId OBJECT IDENTIFIER,
//   keyAttr ANY DEFINED BY keyAttrId OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OtherKeyAttribute {
    pub key_attr_id: ObjectIdentifier,
    pub key_attr: Option<Any>,
}

// OriginatorIdentifierOrKey ::= CHOICE {
//   issuerAndSerialNumber IssuerAndSerialNumber,
//   subjectKeyIdentifier [0] SubjectKeyIdentifier,
//   originatorKey [1] OriginatorPublicKey }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum OriginatorIdentifierOrKey {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(0))]
    SubjectKeyIdentifier(SubjectKeyIdentifier),
    #[rasn(tag(1))]
    OriginatorPublicKey(OriginatorPublicKey),
}

// OriginatorPublicKey ::= SEQUENCE {
//   algorithm AlgorithmIdentifier,
//   publicKey BIT STRING }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OriginatorPublicKey {
    pub algorithm: AlgorithmIdentifier,
    pub public_key: BitString,
}

// KEKRecipientInfo ::= SEQUENCE {
//   version CMSVersion,  -- always set to 4
//   kekid KEKIdentifier,
//   keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,
//   encryptedKey EncryptedKey }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct KekRecipientInfo {
    pub version: CmsVersion,
    pub kek_id: KekIdentifier,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_key: EncryptedKey,
}

// KEKIdentifier ::= SEQUENCE {
//   keyIdentifier OCTET STRING,
//   date GeneralizedTime OPTIONAL,
//   other OtherKeyAttribute OPTIONAL }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct KekIdentifier {
    pub key_identifier: OctetString,
    pub date: Option<GeneralizedTime>,
    pub other: Option<OtherKeyAttribute>,
}

// PasswordRecipientInfo ::= SEQUENCE {
//   version CMSVersion,   -- always set to 0
//   keyDerivationAlgorithm [0] KeyDerivationAlgorithmIdentifier OPTIONAL,
//   keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,
//   encryptedKey EncryptedKey }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordRecipientInfo {
    pub version: CmsVersion,
    #[rasn(tag(0))]
    pub key_derivation_algorithm: Option<KeyDerivationAlgorithmIdentifier>,
    pub key_encryption_algorithm: KeyEncryptionAlgorithmIdentifier,
    pub encrypted_eey: EncryptedKey,
}

// OtherRecipientInfo ::= SEQUENCE {
//   oriType OBJECT IDENTIFIER,
//   oriValue ANY DEFINED BY oriType }
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OtherRecipientInfo {
    pub ori_type: ObjectIdentifier,
    pub ori_value: Any,
}
