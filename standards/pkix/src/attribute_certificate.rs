use crate::{
    AlgorithmIdentifier, Attribute, CertificateSerialNumber, Extensions, GeneralName, GeneralNames,
    UniqueIdentifier,
};
use rasn::prelude::*;

pub type Targets = SequenceOf<Target>;
pub type AttrSpec = SequenceOf<ObjectIdentifier>;
pub type ProxyInfo = SequenceOf<Targets>;

pub const AUDIT_IDENTITY: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_PE_AUDIT_IDENTIFY;
pub const AA_CONTROLS: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_PE_AA_CONTROLS;
pub const AC_PROXYING: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_PE_AC_PROXYING;
pub const TARGET_INFORMATION: &Oid =
    Oid::JOINT_ISO_ITU_T_DS_CERTIFICATE_EXTENSION_TARGET_INFORMATION;
pub const AUTHENTICATION_INFO: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ACA_AUTHENTICATION_INFO;
pub const ACCESS_IDENTITY: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ACA_ACCESS_IDENTITY;
pub const CHARGING_IDENTITY: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ACA_CHARGING_IDENTITY;
pub const GROUP: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ACA_GROUP;
pub const ENC_ATTRIBUTES: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ACA_ENC_ATTRIBUTES;
pub const ROLE: &Oid = Oid::JOINT_ISO_ITU_T_DS_ATTRIBUTE_TYPE_ROLE;
pub const CLEARANCE: &Oid = Oid::JOINT_ISO_ITU_T_DS_ATTRIBUTE_TYPE_CLEARANCE;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AttributeCertificate {
    pub info: AttributeCertificateInfo,
    pub signature_algorithm: AlgorithmIdentifier,
    pub signature_value: BitString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AttributeCertificateInfo {
    pub version: AttributeCertificateVersion,
    pub holder: Holder,
    pub issuer: Issuer,
    pub signature: AlgorithmIdentifier,
    pub serial_number: CertificateSerialNumber,
    pub attr_cert_validity_period: AttributeCertificateValidityPeriod,
    pub attributes: SequenceOf<Attribute>,
    pub issuer_unique_id: Option<UniqueIdentifier>,
    pub extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct AttributeCertificateVersion(pub Integer);

impl AttributeCertificateVersion {
    pub const V2: u8 = 1;
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Holder {
    /// The issuer and serial number of the holder's public key certificate.
    #[rasn(tag(0))]
    pub base_certificate_id: Option<IssuerSerial>,
    /// The name of the claimant or role.
    #[rasn(tag(1))]
    pub entity_name: Option<GeneralNames>,
    /// Used to directly authenticate the holder, for example, an executable.
    #[rasn(tag(2))]
    pub object_digest_info: Option<ObjectDigestInfo>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectDigestInfo {
    pub digested_object_type: DisgestedObjectType,
    pub other_object_type_id: Option<ObjectIdentifier>,
    pub digest_algorithm: AlgorithmIdentifier,
    pub object_digest: BitString,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum DisgestedObjectType {
    PublicKey = 0,
    PublicKeyCert = 1,
    OtherObjectTypes = 2,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Issuer {
    V1(GeneralNames),
    #[rasn(tag(0))]
    V2(V2Form),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct V2Form {
    pub issuer_name: Option<GeneralNames>,
    #[rasn(tag(0))]
    pub base_certificate_id: Option<IssuerSerial>,
    #[rasn(tag(1))]
    pub object_digest_info: Option<ObjectDigestInfo>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct IssuerSerial {
    pub issuer: GeneralNames,
    pub serial: CertificateSerialNumber,
    pub issuer_uid: Option<UniqueIdentifier>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AttributeCertificateValidityPeriod {
    pub not_before: GeneralizedTime,
    pub not_after: GeneralizedTime,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Target {
    #[rasn(tag(0))]
    Name(GeneralName),
    #[rasn(tag(1))]
    Group(GeneralName),
    #[rasn(tag(2))]
    Cert(TargetCert),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TargetCert {
    pub target_certificate: IssuerSerial,
    pub target_name: Option<GeneralName>,
    pub cert_digest_info: Option<ObjectDigestInfo>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct IetfAttrSyntax {
    #[rasn(tag(0))]
    pub policy_authority: Option<GeneralNames>,
    pub values: SequenceOf<IetfAttrSyntaxValue>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum IetfAttrSyntaxValue {
    Octets(OctetString),
    Oid(ObjectIdentifier),
    String(Utf8String),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct SvceAuthInfo {
    pub service: GeneralName,
    pub ident: GeneralName,
    pub auth_info: Option<OctetString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RoleSyntax {
    #[rasn(tag(0))]
    role_authority: Option<GeneralNames>,
    #[rasn(tag(1))]
    role_name: GeneralName,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Clearance {
    policy_id: ObjectIdentifier,
    #[rasn(default = "ClassList::unclassified")]
    class_list: ClassList,
    security_categories: Option<SetOf<SecurityCategory>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct ClassList(pub BitString);

impl ClassList {
    pub fn unmarked() -> Self {
        Self(BitString::from_element(0))
    }

    pub fn unclassified() -> Self {
        Self(BitString::from_element(1))
    }

    pub fn restricted() -> Self {
        Self(BitString::from_element(2))
    }

    pub fn confidential() -> Self {
        Self(BitString::from_element(3))
    }

    pub fn secret() -> Self {
        Self(BitString::from_element(4))
    }

    pub fn top_secret() -> Self {
        Self(BitString::from_element(5))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityCategory {
    #[rasn(tag(0))]
    r#type: ObjectIdentifier,
    #[rasn(tag(explicit(1)))]
    value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AaControls {
    path_len_constraint: Option<Integer>,
    #[rasn(tag(0))]
    permitted_attrs: Option<AttrSpec>,
    #[rasn(tag(1))]
    excluded_attrs: Option<AttrSpec>,
    #[rasn(default = "true_bool")]
    permit_unspecified: bool,
}

fn true_bool() -> bool {
    true
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AttributeCertificateClearAttributes {
    pub issuer: GeneralName,
    pub serial: Integer,
    pub attrs: SequenceOf<Attribute>,
}
