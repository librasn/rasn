use rasn::types::*;

pub use crate::algorithms::RsaPublicKey;

pub type EncodingParameters = OctetString;
pub type Version = Integer;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RsaPrivateKey {
    pub version: Integer,
    pub modulus: Integer,
    pub public_exponent: Integer,
    pub private_exponent: Integer,
    pub prime1: Integer,
    pub prime2: Integer,
    pub exponent1: Integer,
    pub exponent2: Integer,
    pub coefficient: Integer,
    pub other_prime_infos: Option<OtherPrimeInfos>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
struct OtherPrimeInfos(pub SequenceOf<OtherPrimeInfo>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct OtherPrimeInfo {
    prime: Integer,
    exponent: Integer,
    coefficient: Integer,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RsaesOaepParams {
    hash_algorithm: HashAlgorithm,
}
