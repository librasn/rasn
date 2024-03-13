//! # Enhanced Security Services

use rasn::prelude::*;
use rasn_cms::{ContentType, IssuerAndSerialNumber, SubjectKeyIdentifier};
use rasn_pkix::{AlgorithmIdentifier, CertificateSerialNumber, GeneralNames, PolicyInformation};

pub type ContentIdentifier = OctetString;
pub type MessageSignatureDigest = OctetString;
pub type SecurityPolicyIdentifier = ObjectIdentifier;
pub type Hash = OctetString;

pub const CONTENT_REFERENCE: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_CONTENT_REFERENCE;
pub const RECEIPT_REQUEST: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_RECEIPT_REQUEST;
pub const CONTENT_IDENTIFIER: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_CONTENT_IDENTIFIER;
pub const RECEIPT: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_RECEIPT;
pub const CONTENT_HINT: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_CONTENT_HINT;
pub const MESSAGE_SIGNATURE_DIGEST: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_MESSAGE_SIGNATURE_DIGEST;
pub const SECURITY_LABEL: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_SECURITY_LABEL;
pub const EQUIVALVENT_LABELS: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_EQUIVALVENT_LABELS;
pub const ML_EXPAND_HISTORY: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_ML_EXPAND_HISTORY;
pub const SIGNING_CERTIFICATE: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_SIGNING_CERTIFICATE;
pub const SIGNING_CERTIFICATE_V2: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_SIGNING_CERTIFICATE_V2;
pub const SHA256: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_HASH_SHA256;

pub const RECEIPTS_TO: u8 = 16;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptRequest {
    pub signed_content_identifier: ContentIdentifier,
    pub receipts_from: ReceiptsFrom,
    pub receipts_to: SequenceOf<GeneralNames>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum ReceiptsFrom {
    #[rasn(tag(0))]
    AllOrFirstTier(AllOrFirstTier),
    #[rasn(tag(1))]
    ReceiptList(SequenceOf<GeneralNames>),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct AllOrFirstTier(pub Integer);

impl AllOrFirstTier {
    pub const ALL_RECEIPTS: u8 = 0;
    pub const FIRST_TIER_RECEIPIENTS: u8 = 1;
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Receipt {
    pub version: EssVersion,
    pub content_type: ContentType,
    pub signed_content_identifier: ContentIdentifier,
    pub originator_signature_value: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct EssVersion(pub Integer);

impl EssVersion {
    pub const V1: u8 = 1;
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentHints {
    pub content_description: Option<Utf8String>,
    pub content_type: ContentType,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentReference {
    pub content_type: ContentType,
    pub signed_content_identifier: ContentIdentifier,
    pub originator_signature_value: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(set)]
pub struct EssSecurityLabel {
    pub security_policy_identifier: SecurityPolicyIdentifier,
    pub security_classification: Option<SecurityClassification>,
    pub privacy_mark: Option<EssPrivacyMark>,
    pub security_categories: Option<SecurityCategories>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct SecurityClassification(pub u8);

impl SecurityClassification {
    pub const UNMARKED: Self = Self(0);
    pub const UNCLASSIFIED: Self = Self(1);
    pub const RESTRICTED: Self = Self(2);
    pub const CONFIDENTAL: Self = Self(3);
    pub const SECRET: Self = Self(4);
    pub const TOP_SECRET: Self = Self(5);
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum EssPrivacyMark {
    PString(PrintableString),
    Utf8String(Utf8String),
}

pub type SecurityCategories = SetOf<SecurityCategory>;
pub type EquivalentLabels = SequenceOf<EssSecurityLabel>;
pub type MLExpansionHistory = SequenceOf<MlData>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityCategory {
    #[rasn(tag(0))]
    pub r#type: ObjectIdentifier,
    #[rasn(tag(1))]
    pub value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MlData {
    pub mail_list_identifier: EntityIdentifier,
    pub expansion_time: GeneralizedTime,
    pub ml_receipt_policy: Option<MlReceiptPolicy>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum EntityIdentifier {
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum MlReceiptPolicy {
    #[rasn(tag(0))]
    None,
    #[rasn(tag(1))]
    InsteadOf(SequenceOf<GeneralNames>),
    #[rasn(tag(2))]
    InAdditionTo(SequenceOf<GeneralNames>),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SigningCertificate {
    pub certs: SequenceOf<EssCertId>,
    pub policies: Option<SequenceOf<PolicyInformation>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SigningCertificateV2 {
    pub certs: SequenceOf<EssCertIdv2>,
    pub policies: Option<SequenceOf<PolicyInformation>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EssCertIdv2 {
    #[rasn(default = "default_sha256")]
    pub hash_algorithm: AlgorithmIdentifier,
    pub cert_hash: Hash,
    pub issuer_serial: Option<IssuerSerial>,
}

fn default_sha256() -> AlgorithmIdentifier {
    AlgorithmIdentifier {
        algorithm: SHA256.into(),
        parameters: None,
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EssCertId {
    pub cert_hash: Hash,
    pub issuer_serial: Option<IssuerSerial>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IssuerSerial {
    pub issuer: GeneralNames,
    pub serial_number: CertificateSerialNumber,
}
