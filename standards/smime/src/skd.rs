//! # Symmetric Key Distribution
use rasn::prelude::*;
use rasn_cms::{algorithms::AES128_WRAP, CertificateSet, KekIdentifier, RecipientInfos};
use rasn_pkix::{
    attribute_certificate::AttributeCertificate, AlgorithmIdentifier, Certificate, GeneralName,
};

pub type GlkCompromise = GeneralName;
pub type SkdAlgRequest = ();

pub const ADD_MEMBER: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_ADD_MEMBER;
pub const ADD_OWNER: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_ADD_OWNER;
pub const DELETE_MEMBER: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_DELETE_MEMBER;
pub const GLARR: ConstOid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_CMC_GLARR;
pub const GLKEY: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_GLKEY;
pub const GL_DELETE: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_GL_DELETE;
pub const MANAGE_CERT: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_MANAGE_CERT;
pub const PROVIDE_CERT: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_PROVIDE_CERT;
pub const QUERY_REQUEST: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_QUERY_REQUEST;
pub const QUERY_RESPONSE: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_QUERY_RESPONSE;
pub const REKEY: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_REKEY;
pub const REMOVE_OWNER: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_REMOVE_OWNER;
pub const SKD_FAIL_INFO: ConstOid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_CET_SKD_FAIL_INFO;
pub const USE_KEK: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_USE_KEK;
pub const GLKEY_COMPROMISE: ConstOid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_GLKEY_COMPROMISE;
pub const GLKEY_REFRESH: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_SKD_GLKEY_REFRESH;
pub const SKD_ALG_REQUEST: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_CMC_GLARR_SKD_ALG_REQUEST;
pub const SKD_ALG_RESPONSE: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_CMC_GLARR_SKD_ALG_RESPONSE;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlUseKek {
    pub info: GlInfo,
    pub owner_info: SequenceOf<GlOwnerInfo>,
    #[rasn(default = "GlAdministration::managed")]
    pub administration: GlAdministration,
    key_attributes: Option<GlKeyAttributes>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlInfo {
    pub name: GeneralName,
    pub address: GeneralName,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlOwnerInfo {
    pub owner_name: GeneralName,
    pub owner_address: GeneralName,
    pub certificates: Option<Certificates>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct GlAdministration(pub Integer);

impl GlAdministration {
    pub fn unmanaged() -> Self {
        Self(Integer::from(0))
    }

    pub fn managed() -> Self {
        Self(Integer::from(1))
    }

    pub fn closed() -> Self {
        Self(Integer::from(2))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlKeyAttributes {
    #[rasn(tag(0), default)]
    pub rekey_controlled_by_glo: bool,
    #[rasn(tag(1), default = "true_bool")]
    pub recipients_not_mutually_aware: bool,
    #[rasn(tag(2), default)]
    pub duration: Integer,
    #[rasn(tag(3), default = "integer_two")]
    pub generation_counter: Integer,
    #[rasn(tag(4), default = "aes128_wrap_algorithm")]
    pub requested_algorithm: AlgorithmIdentifier,
}

fn true_bool() -> bool {
    true
}

fn integer_two() -> Integer {
    Integer::from(2)
}

fn aes128_wrap_algorithm() -> AlgorithmIdentifier {
    AlgorithmIdentifier {
        algorithm: AES128_WRAP.into(),
        parameters: None,
    }
}

impl Default for GlKeyAttributes {
    fn default() -> Self {
        Self {
            rekey_controlled_by_glo: false,
            recipients_not_mutually_aware: true_bool(),
            duration: Integer::default(),
            generation_counter: integer_two(),
            requested_algorithm: aes128_wrap_algorithm(),
        }
    }
}

impl From<GlNewKeyAttributes> for GlKeyAttributes {
    fn from(new_attributes: GlNewKeyAttributes) -> Self {
        let defaults = Self::default();

        Self {
            rekey_controlled_by_glo: new_attributes
                .rekey_controlled_by_glo
                .unwrap_or(defaults.rekey_controlled_by_glo),
            recipients_not_mutually_aware: new_attributes
                .recipients_not_mutually_aware
                .unwrap_or(defaults.recipients_not_mutually_aware),
            duration: new_attributes.duration.unwrap_or(defaults.duration),
            generation_counter: new_attributes
                .generation_counter
                .unwrap_or(defaults.generation_counter),
            requested_algorithm: new_attributes
                .requested_algorithm
                .unwrap_or(defaults.requested_algorithm),
        }
    }
}

pub type DeleteGl = GeneralName;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlAddMember {
    pub name: GeneralName,
    member: GlMember,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlMember {
    pub member_name: GeneralName,
    pub member_address: Option<GeneralName>,
    pub certificates: Option<Certificates>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Certificates {
    #[rasn(tag(0))]
    pub pkc: Option<Certificate>,
    #[rasn(tag(1))]
    pub ac: Option<SequenceOf<AttributeCertificate>>,
    #[rasn(tag(2))]
    cert_path: Option<CertificateSet>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlDeleteMember {
    pub name: GeneralName,
    pub member_to_delete: GeneralName,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlRekey {
    pub name: GeneralName,
    pub administration: Option<GlAdministration>,
    pub new_key_attributes: Option<GlNewKeyAttributes>,
    pub rekey_all_gl_keys: Option<bool>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlNewKeyAttributes {
    #[rasn(tag(0))]
    pub rekey_controlled_by_glo: Option<bool>,
    #[rasn(tag(1))]
    pub recipients_not_mutually_aware: Option<bool>,
    #[rasn(tag(2))]
    pub duration: Option<Integer>,
    #[rasn(tag(3))]
    pub generation_counter: Option<Integer>,
    #[rasn(tag(4))]
    pub requested_algorithm: Option<AlgorithmIdentifier>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlOwnerAdministration {
    pub name: GeneralName,
    pub owner_info: GlOwnerInfo,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlkRefresh {
    pub name: GeneralName,
    pub dates: SequenceOf<Date>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub start: GeneralizedTime,
    pub end: Option<GeneralizedTime>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlaQueryRequest {
    r#type: ObjectIdentifier,
    pub value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlaQueryResponse {
    r#type: ObjectIdentifier,
    value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlManageCert {
    pub name: GeneralName,
    pub member: GlMember,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlKey {
    pub name: GeneralName,
    pub identifier: KekIdentifier,
    pub wrapped: RecipientInfos,
    pub algorithm: AlgorithmIdentifier,
    pub not_before: GeneralizedTime,
    pub not_after: GeneralizedTime,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct SkdFailInfo(pub Integer);

impl SkdFailInfo {
    pub fn unspecified() -> Self {
        Self(0.into())
    }

    pub fn closed_gl() -> Self {
        Self(1.into())
    }

    pub fn unsupported_duration() -> Self {
        Self(2.into())
    }

    pub fn no_gla_certificate() -> Self {
        Self(3.into())
    }

    pub fn invalid_cert() -> Self {
        Self(4.into())
    }

    pub fn unsupported_algorithm() -> Self {
        Self(5.into())
    }

    pub fn no_glo_name_match() -> Self {
        Self(6.into())
    }

    pub fn invalid_gl_name() -> Self {
        Self(7.into())
    }

    pub fn name_already_in_use() -> Self {
        Self(8.into())
    }

    pub fn no_spam() -> Self {
        Self(9.into())
    }

    pub fn already_a_member() -> Self {
        Self(11.into())
    }

    pub fn not_a_member() -> Self {
        Self(12.into())
    }

    pub fn already_an_owner() -> Self {
        Self(13.into())
    }
}
