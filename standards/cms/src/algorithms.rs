//! # Algorithms used with CMS.
//! Algorithms OIDs and parameter data types.

use rasn::prelude::*;

use rasn_pkix::AlgorithmIdentifier;

pub const SHA1: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_OIW_SECSIG_ALGORITHM_SHA1;
pub const MD5: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_DIGEST_ALGORITHM_MD5;
pub const DSA: &Oid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA;
pub const DSA_WITH_SHA1: &Oid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA_SHA1;
pub const RSA: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_RSA;
pub const MD5_WITH_RSA: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_MD5_RSA;
pub const SHA1_WITH_RSA: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_SHA1_RSA;
pub const PUBLIC_NUMBER: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X942_NUMBER_TYPE_PUBLIC;

pub const ESDH: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_ESDH;
pub const SSDH: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_SSDH;
pub const CMS3DESWRAP: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_CMS3DESWRAP;
pub const CMS3RC2WRAP: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_CMS3RC2WRAP;

pub const DES_EDE3_CBC: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_ENCRYPTION_ALGORITHM_DES_EDE3_CBC;
pub const RC2_CBC: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_ENCRYPTION_ALGORITHM_RC2_CBC;

pub const HMAC_SHA1: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_HMAC_SHA1;
pub const PBKDF2: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS5_PBKDF2;
pub const PBMAC1: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS5_PBMAC1;

pub const AES: &Oid = Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES;
pub const AES128_CBC: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES128_CBC;
pub const AES128_WRAP: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES128_WRAP;
pub const AES192_CBC: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES192_CBC;
pub const AES192_WRAP: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES192_WRAP;
pub const AES256_CBC: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES256_CBC;
pub const AES256_WRAP: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_NIST_ALGORITHMS_AES256_WRAP;

pub type DssPubKey = Integer;
pub type AesIv = OctetString;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RsaPublicKey {
    pub modulus: Integer,
    pub public_exponent: Integer,
}

pub type DhPublicKey = Integer;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DssSigValue {
    pub r: Integer,
    pub s: Integer,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DssParameters {
    pub p: Integer,
    pub q: Integer,
    pub g: Integer,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DhDomainParameters {
    pub prime: Integer,
    pub generator: Integer,
    pub factor: Integer,
    pub subgroup_factor: Option<Integer>,
    pub validation_parameters: Option<ValidationParameters>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ValidationParameters {
    pub seed: BitString,
    pub pgen_counter: Integer,
}

pub type KeyWrapAlgorithm = AlgorithmIdentifier;
pub type Rc2wrapParameter = Rc2ParameterVersion;
pub type Rc2ParameterVersion = Integer;
pub type CbcParameter = Iv;
pub type Iv = OctetString;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rc2CbcParameter {
    pub rc2_parameter_version: Integer,
    pub iv: OctetString,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pbkdf2Parameters {
    pub salt: Pbkdf2Salt,
    pub iteration_count: Integer,
    pub key_length: Option<Integer>,
    #[rasn(default = "default_pbkdf2_algorithm")]
    pub prf: AlgorithmIdentifier,
}

pub fn default_pbkdf2_algorithm() -> AlgorithmIdentifier {
    AlgorithmIdentifier {
        algorithm: HMAC_SHA1.into(),
        parameters: None,
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum Pbkdf2Salt {
    Specified(OctetString),
    OtherSource(AlgorithmIdentifier),
}

/// Password-Based Message Authentication Code 1 (PBMAC1) parameters defined in
/// [RFC 8018 A.5](https://www.rfc-editor.org/rfc/rfc8018#appendix-A.5)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pbmac1Parameter {
    pub key_derivation_func: AlgorithmIdentifier,
    pub message_auth_scheme: AlgorithmIdentifier,
}
