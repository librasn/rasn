//! Implementation of [RFC 3279](https://www.rfc-editor.org/rfc/rfc3279).
use rasn::prelude::*;

pub const ID_SHA1: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_OIW_SECSIG_ALGORITHM_SHA1;
pub const DH_PUBLIC_NUMBER: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X942_NUMBER_TYPE_PUBLIC;
pub const ELLIPTIC_CURVE: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC;
pub const C_TWO_CURVE: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO;
pub const C2ONB191V4: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2ONB191V4;
pub const C2ONB191V5: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2ONB191V5;
pub const C2ONB239V4: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2ONB239V4;
pub const C2ONB239V5: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2ONB239V5;
pub const C2PNB163V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB163V1;
pub const C2PNB163V2: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB163V2;
pub const C2PNB163V3: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB163V3;
pub const C2PNB176W1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB176W1;
pub const C2PNB208W1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB208W1;
pub const C2PNB272W1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB272W1;
pub const C2PNB304W1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB304W1;
pub const C2PNB368W1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2PNB368W1;
pub const C2TNB191V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB191V1;
pub const C2TNB191V2: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB191V2;
pub const C2TNB191V3: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB191V3;
pub const C2TNB239V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB239V1;
pub const C2TNB239V2: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB239V2;
pub const C2TNB239V3: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB239V3;
pub const C2TNB359V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB359V1;
pub const C2TNB431R1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_CHARACTERISTIC_TWO_C2TNB431R1;
pub const PRIME: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME;
pub const PRIME_192V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_192V1;
pub const PRIME_192V2: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_192V2;
pub const PRIME_192V3: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_192V3;
pub const PRIME_239V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_239V1;
pub const PRIME_239V2: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_239V2;
pub const PRIME_239V3: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_239V3;
pub const PRIME_256V1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_PRIME_256V1;
pub const ID_EC_SIG_TYPE: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_SIG_TYPE;
pub const ECDSA_WITH_SHA1: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_EC_SIG_TYPE_SHA1;
pub const ID_FIELD_TYPE: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_FIELD_TYPE;
pub const CHARACTERISTIC_TWO_FIELD: &Oid =
    Oid::ISO_MEMBER_BODY_US_ANSI_X962_FIELD_TYPE_CHARACTERISTIC_TWO_FIELD;
pub const ID_CHARACTERISTIC_TWO_BASIS: &Oid =
    Oid::ISO_MEMBER_BODY_US_ANSI_X962_FIELD_TYPE_CHARACTERISTIC_TWO_FIELD_BASIS;
pub const GN_BASIS: &Oid =
    Oid::ISO_MEMBER_BODY_US_ANSI_X962_FIELD_TYPE_CHARACTERISTIC_TWO_FIELD_BASIS_TP;
pub const PRIME_FIELD: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_FIELD_TYPE_PRIME;
pub const ID_PUBLIC_KEY_TYPE: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_KEY_TYPE;
pub const ID_EC_PUBLIC_KEY: &Oid = Oid::ISO_MEMBER_BODY_US_ANSI_X962_KEY_TYPE_EC_PUBLIC_KEY;
pub const MD2: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_DIGEST_MD2;
pub const MD5: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_DIGEST_MD5;
pub const PKCS1: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1;
pub const MD2_WITH_RSA_ENCRYPTION: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_MD2_RSA;
pub const MD5_WITH_RSA_ENCRYPTION: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_MD5_RSA;
pub const RSA_ENCRYPTION: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_RSA;
pub const SHA1_WITH_RSA_ENCRYPTION: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS1_SHA1_RSA;
pub const ID_DSA: &Oid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA;
pub const ID_DSA_WITH_SHA1: &Oid = Oid::ISO_MEMBER_BODY_US_X957_X9CM_DSA_SHA1;
pub const ID_KEY_EXCHANGE_ALGORITHM: &Oid =
    Oid::JOINT_ISO_ITU_T_COUNTRY_US_ORGANIZATION_GOV_CSOR_INFOSEC_ALGORITHMS_KEY_EXCHANGE;

pub type DsaPublicKey = Integer;
pub type DhPublicKey = Integer;
pub type KeaParamsId = OctetString;
pub type PrimeP = Integer;
pub type Trinomial = Integer;
pub type FieldElement = OctetString;
pub type EcPoint = OctetString;
pub type EcpVer = Integer;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DssParams {
    pub p: Integer,
    pub q: Integer,
    pub g: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DssSigValue {
    pub r: Integer,
    pub s: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RsaPublicKey {
    pub modulus: Integer,
    pub public_exponent: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DomainParameters {
    /// Odd prime
    pub p: Integer,
    /// Generator
    pub g: Integer,
    /// Factor of `p-1`
    pub q: Integer,
    /// subgroup factor, j >= 2
    pub j: Option<Integer>,
    pub validation_params: Option<ValidationParams>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ValidationParams {
    seed: BitString,
    pgen_counter: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FieldId {
    field_type: ObjectIdentifier,
    parameters: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct EcdsaSigValue {
    r: Integer,
    s: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct CharacteristicTwo {
    m: Integer,
    basis: ObjectIdentifier,
    parameters: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Pentanomial {
    k1: Integer,
    k2: Integer,
    k3: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct EcpkParameters {
    ec_parameters: EcParameters,
    named_curve: ObjectIdentifier,
    implicitly_ca: (),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct EcParameters {
    ec_parameters: EcpVer,
    named_curve: FieldId,
    curve: Curve,
    base: EcPoint,
    order: Integer,
    cofactor: Option<Integer>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Curve {
    a: FieldElement,
    b: FieldElement,
    seed: Option<BitString>,
}
