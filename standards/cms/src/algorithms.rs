use rasn::prelude::*;

use rasn_pkix::AlgorithmIdentifier;

pub const SHA1: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_OIW_SECSIG_ALGORITHM_SHA1;
pub const MD5: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_DIGEST_ALGORITHM_MD5;
pub const DSA: ConstOid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA;
pub const DSA_WITH_SHA1: ConstOid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA_SHA1;
pub const RSA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_RSA;
pub const MD5_WITH_RSA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_MD5_RSA;
pub const SHA1_WITH_RSA: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_SHA1_RSA;
pub const PUBLIC_NUMBER: ConstOid = Oid::ISO_MEMBER_BODY_US_ANSI_X942_NUMBER_TYPE_PUBLIC;

pub const ESDH: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_ESDH;
pub const SSDH: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_SSDH;
pub const CMS3DESWRAP: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_CMS3DESWRAP;
pub const CMS3RC2WRAP: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_ALGORITHM_CMS3RC2WRAP;

pub const DES_EDE3_CBC: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_ENCRYPTION_ALGORITHM_DES_EDE3_CBC;
pub const RC2_CBC: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_ENCRYPTION_ALGORITHM_RC2_CBC;

pub const HMAC_SHA1: ConstOid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_HMAC_SHA1;
pub const PBKDF2: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS5_PBKDF2;

pub type DssPubKey = Integer;

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
     salt: Pbkdf2Salt,
     iteration_count: Integer,
     key_length: Option<Integer>,
     #[rasn(default="default_pbkdf2_algorithm")]
     prf: AlgorithmIdentifier,
}

pub fn default_pbkdf2_algorithm() -> AlgorithmIdentifier {
    AlgorithmIdentifier {
        algorithm: HMAC_SHA1.into(),
        parameters:None,
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum Pbkdf2Salt {
    Specified(OctetString),
    OtherSource(AlgorithmIdentifier),
}
