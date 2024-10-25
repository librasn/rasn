//! PKCS7 compatibility module.
//!
//! The Cryptographic Message Syntax and PKCS7 are mostly the same. The only
//! difference is that the `EncapsulatedContentInfo` content field has the `Any`
//! type instead of the `OctetString`. See section 5.2.1. Compatibility with PKCS #7
//! of RFC5652 for further information.
use crate::{
    Any, AuthAttributes, CertificateSet, CmsVersion, ContentType, Digest,
    DigestAlgorithmIdentifier, DigestAlgorithmIdentifiers, MessageAuthenticationCode,
    MessageAuthenticationCodeAlgorithm, OriginatorInfo, RecipientInfos, RevocationInfoChoices,
    SignerInfos, UnauthAttributes,
};
use rasn::prelude::*;

/// The content is represented in the type EncapsulatedContentInfo
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncapsulatedContentInfo {
    pub content_type: ContentType,
    #[rasn(tag(explicit(0)))]
    pub content: Option<Any>,
}

/// SignedData represents a signed-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
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

/// DigestedData represents a digested-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigestedData {
    pub version: CmsVersion,
    pub digest_algorithm: DigestAlgorithmIdentifier,
    pub encap_content_info: EncapsulatedContentInfo,
    pub digest: Digest,
}

/// AuthenticatedData represents an authenticated-data content type
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
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
