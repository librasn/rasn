#![cfg_attr(not(test), no_std)]
extern crate alloc;
use bon::Builder;
use rasn::error::InnerSubtypeConstraintError;
use rasn::prelude::*;
pub mod base_types;
pub use base_types as ieee1609_dot2_base_types;
pub mod crl_base_types;
pub use crl_base_types as ieee1609_dot2_crl_base_types;

use ieee1609_dot2_base_types::*;
use rasn_etsi_ts103097_extensions::{
    EtsiOriginatingHeaderInfoExtension, ExtId, ExtType, Extension,
};

/// OID for IEEE 1609.2 module
pub const IEEE1609_DOT2_OID: &Oid = Oid::const_new(&[
    1,    // iso
    3,    // identified-organization
    111,  // ieee
    2,    // standards-association-numbered-series-standards
    1609, // wave-stds
    2,    // dot2
    1,    // base
    1,    // schema
    2,    // major-version-2
    6,    // minor-version-6
]);

/// A macro to implement `From` and `Deref` for a delegate type pair.
#[macro_export]
macro_rules! delegate {
    ($from_type:ty, $to_type:ty) => {
        impl From<$from_type> for $to_type {
            fn from(item: $from_type) -> Self {
                Self(item)
            }
        }
        impl From<$to_type> for $from_type {
            fn from(item: $to_type) -> Self {
                item.0
            }
        }
        impl core::ops::Deref for $to_type {
            type Target = $from_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl core::ops::DerefMut for $to_type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub const CERT_EXT_ID_OPERATING_ORGANIZATION: ExtId = ExtId(1);
pub const P2PCD8_BYTE_LEARNING_REQUEST_ID: ExtId = ExtId(1);

//***************************************************************************
//                               Secured Data
//***************************************************************************

/// Contains other data types in this clause.
///
/// # Fields
/// - `protocol_version`: Current version of the protocol (version 3, represented
///   by integer 3). There are no major or minor version numbers.
/// - `content`: Contains the content in the form of an `Ieee1609Dot2Content`
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2.
/// The canonicalization applies to the `Ieee1609Dot2Content`.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct Ieee1609Dot2Data {
    #[builder(default = 3)]
    #[rasn(value("3"), identifier = "protocolVersion")]
    pub protocol_version: Uint8,
    pub content: Ieee1609Dot2Content,
}

/// Content types for IEEE 1609.2 data structures.
///
/// # Variants
/// - `UnsecuredData`: OCTET STRING to be consumed outside the SDS
/// - `SignedData`: Content signed according to this standard
/// - `EncryptedData`: Content encrypted according to this standard
/// - `SignedCertificateRequest`: Certificate request signed by an IEEE 1609.2
///   certificate or self-signed
/// - `SignedX509CertificateRequest`: Certificate request signed by an ITU-T X.509
///   certificate
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2 when type is
/// `SignedData`. The canonicalization applies to the `SignedData` structure.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Ieee1609Dot2Content {
    UnsecuredData(Opaque),
    SignedData(alloc::boxed::Box<SignedData>),
    EncryptedData(EncryptedData),
    SignedCertificateRequest(Opaque),
    #[rasn(extension_addition)]
    SignedX509CertificateRequest(Opaque),
}
/// Structure containing signed data and signature information.
///
/// # Fields
/// - `hash_id`: Hash algorithm used to generate the message hash for signing
///   and verification
/// - `tbs_data`: Data that is hashed as input to the signature
/// - `signer`: Determines keying material and hash algorithm used to sign the data
/// - `signature`: Digital signature calculated as specified in 5.3.1
///
/// # Signature Calculation
/// ## For Self-Signed
/// When `signer` indicates "self":
/// - Data input: COER encoding of `tbs_data` field (canonicalized per 6.3.6)
/// - Verification type: self
/// - Signer identifier input: empty string
///
/// ## For Certificate or Digest
/// When `signer` indicates "certificate" or "digest":
/// - Data input: COER encoding of `tbs_data` field (canonicalized per 6.3.6)
/// - Verification type: certificate
/// - Signer identifier input: COER-encoding of the Certificate used for
///   verification (canonicalized per 6.4.3)
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2.
/// Applies to both `ToBeSignedData` and `Signature`.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SignedData {
    #[rasn(identifier = "hashId")]
    pub hash_id: HashAlgorithm,
    #[rasn(identifier = "tbsData")]
    pub tbs_data: ToBeSignedData,
    pub signer: SignerIdentifier,
    pub signature: Signature,
}
/// Contains the data to be hashed when generating or verifying a signature.
/// See section 6.3.4 for hash input specification.
///
/// # Fields
/// - `payload`: Data provided by the entity invoking the SDS
/// - `header_info`: Additional data inserted by the SDS
///
/// # Hash Operation Input
/// The data input to the hash operation (per 5.3.1.2.2 or 5.3.1.3) is determined as follows:
///
/// ## When payload is present
/// - Uses COER encoding of the `ToBeSignedData`
///
/// ## When payload is omitted
/// - Uses concatenation of:
///   1. COER encoding of the `ToBeSignedData`
///   2. Hash of the omitted payload (using same hash algorithm as main operation)
/// - No additional wrapping or length indication
/// - Method for establishing omitted payload contents between signer and verifier
///   is out of scope (see 5.2.4.3.4)
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2.
/// Applies to:
/// - `SignedDataPayload` (when type is "data")
/// - `HeaderInfo`
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct ToBeSignedData {
    pub payload: SignedDataPayload,
    #[rasn(identifier = "headerInfo")]
    pub header_info: HeaderInfo,
}

/// Contains the data payload of a `ToBeSignedData`. Must contain at least one
/// optional element, and may contain multiple. See section 5.2.4.3.4 for details.
///
/// # Implementation Support
/// The security profile in Annex C allows implementations to specify:
/// - Supported forms of `SignedDataPayload`
/// - Methods for signers and verifiers to obtain external data for hashing
///
/// SDEE specifications using external data must explicitly define:
/// - How to obtain the data
/// - How to format the data before hash processing
///
/// # Fields
/// - `data`: Data explicitly transported within the structure
/// - `ext_data_hash`: Hash of data not explicitly transported, which the creator
///   wishes to cryptographically bind to the signature
/// - `omitted`: Indicates external data to be included in signature hash calculation
///   (mechanism specified in 6.3.6)
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2.
/// The canonicalization applies to the `Ieee1609Dot2Data`.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignedDataPayload {
    pub data: Option<Ieee1609Dot2Data>,
    #[rasn(identifier = "extDataHash")]
    pub ext_data_hash: Option<HashedData>,
    #[rasn(extension_addition)]
    pub omitted: Option<()>,
}

#[bon::bon]
impl SignedDataPayload {
    #[builder]
    pub fn new(
        data: Option<Ieee1609Dot2Data>,
        ext_data_hash: Option<HashedData>,
        omitted: Option<()>,
    ) -> Result<Self, InnerSubtypeConstraintError> {
        Self {
            data,
            ext_data_hash,
            omitted,
        }
        .validated()
    }
}

impl InnerSubtypeConstraint for SignedDataPayload {
    fn validated(self) -> Result<Self, InnerSubtypeConstraintError> {
        if self.data.is_none() && self.ext_data_hash.is_none() && self.omitted.is_none() {
            return Err(InnerSubtypeConstraintError::MissingAtLeastOneComponent {
                type_name: "SignedDataPayload",
                components: &["data", "ext_data_hash", "omitted"],
            });
        }
        Ok(self)
    }
}

/// Contains the hash of data with a specified hash algorithm. See section 5.3.3
/// for permitted hash algorithms.
///
/// # Variants
/// - `Sha256HashedData`: Data hashed with SHA-256
/// - `Sha384HashedData`: Data hashed with SHA-384
/// - `Sm3HashedData`: Data hashed with SM3
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum HashedData {
    Sha256HashedData(HashedId32),
    #[rasn(extension_addition)]
    Sha384HashedData(HashedId48),
    #[rasn(extension_addition)]
    Sm3HashedData(HashedId32),
}

/// Contains information used to establish validity per criteria in section 5.2.
///
/// # Fields
/// - `psid`: Application area with which sender claims payload association
///
/// - `generation_time`: Time of structure generation (see 5.2.5.2.2 and 5.2.5.2.3)
///
/// - `expiry_time`: Time after which data is no longer relevant
///   - If both `generation_time` and `expiry_time` present, SPDU is invalid if
///     `generation_time` is not strictly earlier than `expiry_time`
///
/// - `generation_location`: Location where signature was generated
///
/// - `p2pcd_learning_request`: Used to request certificates with known identifiers
///   but unknown full content
///   - Used for separate-certificate-pdu P2PCD (Clause 8)
///   - Mutually exclusive with `inline_p2pcd_request`
///   - `HashedId3` calculated using whole-certificate hash algorithm (see 6.4.3)
///
/// - `missing_crl_identifier`: For requesting known-issued but unreceived CRLs
///   (reserved for future use)
///
/// - `encryption_key`: Key for encrypting response SPDUs
///   - SDEE specification defines which responses use this key
///   - See 6.3.35, 6.3.37, and 6.3.34 for usage details
///   - Symmetric key type should only be used with encrypted SignedData
///
/// - `inline_p2pcd_request`: For requesting unknown certificates via inline P2PCD
///   - Mutually exclusive with `p2pcd_learning_request`
///   - `HashedId3` calculated as per 6.4.3
///
/// - `requested_certificate`: Provides certificates for inline P2PCD (Clause 8)
///
/// - `pdu_functional_type`: Indicates SPDU consumption by non-application process
///   (ISO 21177 [B14a], see 6.3.23b)
///
/// - `contributed_extensions`: Additional extensions using `ContributedExtensionBlocks`
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2:
/// - Applies to `EncryptionKey`
/// - For public `EncryptionKey` with elliptic curve points (`EccP256CurvePoint` or
///   `EccP384CurvePoint`):
///   - Points must be in compressed form (`compressed-y-0` or `compressed-y-1`)
/// - Does not apply to fields after extension marker, including `contributed_extensions`
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct HeaderInfo {
    pub psid: Psid,
    #[rasn(identifier = "generationTime")]
    pub generation_time: Option<Time64>,
    #[rasn(identifier = "expiryTime")]
    pub expiry_time: Option<Time64>,
    #[rasn(identifier = "generationLocation")]
    pub generation_location: Option<ThreeDLocation>,
    #[rasn(identifier = "p2pcdLearningRequest")]
    pub p2pcd_learning_request: Option<HashedId3>,
    #[rasn(identifier = "missingCrlIdentifier")]
    pub missing_crl_identifier: Option<MissingCrlIdentifier>,
    #[rasn(identifier = "encryptionKey")]
    pub encryption_key: Option<EncryptionKey>,
    #[rasn(extension_addition, identifier = "inlineP2pcdRequest")]
    pub inline_p2pcd_request: Option<SequenceOfHashedId3>,
    #[rasn(extension_addition, identifier = "requestedCertificate")]
    pub requested_certificate: Option<Certificate>,
    #[rasn(extension_addition, identifier = "pduFunctionalType")]
    pub pdu_functional_type: Option<PduFunctionalType>,
    #[rasn(extension_addition, identifier = "contributedExtensions")]
    pub contributed_extensions: Option<ContributedExtensionBlocks>,
}

/// Structure for requesting a known-issued but unreceived CRL.
/// Provided for future use; not defined in current standard version.
///
/// # Fields
/// - `craca_id`: HashedId3 of the CRACA (defined in 5.1.3)
///   - Calculated using whole-certificate hash algorithm (per 6.4.3)
///   - Applied to COER-encoded certificate
///   - Certificate must be canonicalized per Certificate definition
///
/// - `crl_series`: Requested CRL Series value (see 5.1.3 for details)
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MissingCrlIdentifier {
    #[rasn(identifier = "cracaId")]
    pub craca_id: HashedId3,
    #[rasn(identifier = "crlSeries")]
    pub crl_series: CrlSeries,
}

/// Identifies the functional entity intended to consume an SPDU, specifically for
/// security support services rather than application processes.
/// Further details defined in ISO 21177 [B20].
///
/// # Values
/// - `1`: TLS Handshake
///   - Not for direct application PDU consumption
///   - Used for securing communications to application process
///   - Provides holder's permissions to TLS handshake
///   - References: IETF 5246 [B15], IETF 8446 [B16], ISO 21177 [B20]
///
/// - `2`: ISO 21177 Extended Auth
///   - Not for direct application PDU consumption
///   - Provides additional holder permissions information
///   - See ISO 21177 [B20] for details
///
/// - `3`: ISO 21177 Session Extension
///   - Not for direct application PDU consumption
///   - Enables session persistence beyond certificate lifetime
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PduFunctionalType(pub u8);

delegate!(u8, PduFunctionalType);

/// TLS Handshake process
pub const TLS_HANDSHAKE: PduFunctionalType = PduFunctionalType(1);
/// ISO 21177 Extended Authentication
pub const ISO21177_EXTENDED_AUTH: PduFunctionalType = PduFunctionalType(2);
/// ISO 21177 Session Extension
pub const ISO21177_SESSION_EXTENSION: PduFunctionalType = PduFunctionalType(3);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
pub struct ContributedExtensionBlocks(pub SequenceOf<ContributedExtensionBlockType>);

delegate!(
    SequenceOf<ContributedExtensionBlockType>,
    ContributedExtensionBlocks
);

pub trait HeaderInfoContributedExtension {
    type Extn: AsnType + Encode + Decode;
    const ID: HeaderInfoContributorId;
}

#[derive(AsnType, Copy, Clone, Debug, Encode, Decode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum Ieee1609HeaderInfoExtensions {}

impl ExtType for Ieee1609HeaderInfoExtensions {
    type ExtContent = HashedId8;
    const EXT_ID: ExtId = P2PCD8_BYTE_LEARNING_REQUEST_ID;
}

impl HeaderInfoContributedExtension for Ieee1609ContributedHeaderInfoExtension {
    type Extn = Extension<Ieee1609HeaderInfoExtensions>;
    const ID: HeaderInfoContributorId = IEEE1609_HEADER_INFO_CONTRIBUTOR_ID;
}
impl HeaderInfoContributedExtension for EtsiOriginatingHeaderInfoExtension {
    type Extn = EtsiOriginatingHeaderInfoExtension;
    const ID: HeaderInfoContributorId = ETSI_HEADER_INFO_CONTRIBUTOR_ID;
}

/// Uses the parameterized type `Extension` to define an
/// `Ieee1609ContributedHeaderInfoExtension`.
///
/// Contains an open Extension Content field identified by an extension identifier.
/// The extension identifier value is:
/// - Unique within ETSI-defined extensions
/// - Not required to be unique across all contributing organizations
#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
pub struct Ieee1609ContributedHeaderInfoExtension(pub Extension<Ieee1609HeaderInfoExtensions>);

/// Defines the format of an extension block provided by an identified contributor.
///
/// Uses the template from `IEEE1609DOT2-HEADERINFO-CONTRIBUTED-EXTENSION` class
/// constraint applied to objects in `Ieee1609Dot2HeaderInfoContributedExtensions`.
///
/// # Fields
/// - `contributor_id`: Uniquely identifies the contributor
/// - `extns`: List of extensions from that contributor
///   - Extensions typically follow format specified in section 6.5,
///     but this is not required
#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
pub struct ContributedExtensionBlock<T: HeaderInfoContributedExtension> {
    #[rasn(identifier = "contributorId")]
    pub contributor_id: HeaderInfoContributorId,
    #[rasn(size("1.."))]
    pub extns: SequenceOf<T::Extn>,
}

#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub enum ContributedExtensionBlockType {
    Ieee1609(ContributedExtensionBlock<Ieee1609ContributedHeaderInfoExtension>),
    Etsi(ContributedExtensionBlock<EtsiOriginatingHeaderInfoExtension>),
}

/// Integer identifying a HeaderInfo extension contributing organization.
///
/// # Defined Values
/// - `1` (IEEE1609): Extensions originating with IEEE 1609
/// - `2` (ETSI): Extensions originating with ETSI TC ITS
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct HeaderInfoContributorId(pub u8);

delegate!(u8, HeaderInfoContributorId);

// Defined contributor IDs
pub const ETSI_HEADER_INFO_CONTRIBUTOR_ID: HeaderInfoContributorId = HeaderInfoContributorId(2);
pub const IEEE1609_HEADER_INFO_CONTRIBUTOR_ID: HeaderInfoContributorId = HeaderInfoContributorId(1);

/// Allows recipients to determine which keying material to use for data
/// authentication and indicates the verification type for hash generation
/// (specified in 5.3.1).
///
/// # Variants
/// ## Digest
/// Contains HashedId8 of the relevant certificate:
/// - Calculated with whole-certificate hash algorithm (per 6.4.3)
/// - Verification type: certificate
/// - Certificate data passed to hash function is authorization certificate
///
/// ## Certificate
/// Contains one or more Certificate structures:
/// - First certificate is authorization certificate
/// - Subsequent certificates are issuers of preceding certificates
/// - Verification type: certificate
/// - Certificate data passed to hash function is authorization certificate
///
/// ## SelfSigned
/// - Contains no additional data
/// - Verification type: self-signed
///
/// # Critical Information Fields
/// This is a critical information field (5.2.6):
/// - Implementation must recognize CHOICE value when verifying signed SPDU
/// - For Certificate variant:
///   - Must support at least one certificate
///   - Must support specified number of certificates
/// - Invalid SPDU if requirements not met
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2:
/// - Applies to every Certificate in the certificate field
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SignerIdentifier {
    Digest(HashedId8),
    Certificate(SequenceOfCertificate),
    #[rasn(identifier = "self")]
    SelfSigned(()),
}

/// This data structure is used to perform a countersignature over an already-signed SPDU.
///
/// This is the profile of an Ieee1609Dot2Data containing a signedData.
/// The tbsData within content is composed of a payload containing the hash
/// (extDataHash) of the externally generated, pre-signed
/// SPDU over which the countersignature is performed.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Countersignature(Ieee1609Dot2Data);
#[bon::bon]
impl Countersignature {
    #[builder]
    fn new(data: Ieee1609Dot2Data) -> Result<Self, InnerSubtypeConstraintError> {
        Self(data).validated()
    }
}

impl InnerSubtypeConstraint for Countersignature {
    fn validated(self) -> Result<Self, InnerSubtypeConstraintError> {
        if let Ieee1609Dot2Content::SignedData(ref signed_data) = self.0.content {
            if let SignedData {
                tbs_data:
                    ToBeSignedData {
                        payload:
                            SignedDataPayload {
                                data: None,
                                ext_data_hash: Some(_),
                                ..
                            },
                        header_info:
                            HeaderInfo {
                                generation_time: Some(_),
                                expiry_time: None,
                                generation_location: None,
                                p2pcd_learning_request: None,
                                missing_crl_identifier: None,
                                encryption_key: None,
                                ..
                            },
                        ..
                    },
                ..
            } = **signed_data
            {
                Ok(self)
            } else {
                Err(InnerSubtypeConstraintError::InvalidCombination {
                    type_name: "Countersignature::Ieee1609Dot2Content::SignedData",
                    details:
                        "SignedData does not match innner subtype constraint for Countersignature",
                })
            }
        } else {
            Err(InnerSubtypeConstraintError::InvalidCombination {
                type_name: "Countersignature::Ieee1609Dot2Content",
                details: "SignedData variant is required for Countersignature",
            })
        }
    }
}

// ***************************************************************************
// **                            Encrypted Data                             **
// ***************************************************************************

/// Encodes data encrypted to one or more recipients using their public or symmetric
/// keys as specified in 5.3.4.
///
/// # Fields
/// - `recipients`: One or more `RecipientInfo` entries
///   - May contain multiple types of `RecipientInfo`
///   - All entries must indicate/contain the same data encryption key
///
/// - `ciphertext`: Encrypted data
///   - Contains encrypted `Ieee1609Dot2Data` structure (per 5.3.4.2)
///
/// # Critical Information Fields
/// The `recipients` field is critical (per 5.2.6):
/// - Implementation must support specified number of `RecipientInfo` entries
/// - Must support at least eight entries
/// - Failure to support required number results in decryption failure
///
/// # Raw Data Example
/// For plaintext that isn't from previous SDS operation, it can be encapsulated
/// in `Ieee1609Dot2Data` of type `unsecuredData` (per 4.2.2.2.2):
/// ```text
/// Raw data:    01 23 45 67 89 AB CD EF
/// COER encoded: 03 80 08 01 23 45 67 89 AB CD EF
/// Where:
/// - 03: protocolVersion
/// - 80: choice unsecuredData
/// - 08: length of raw data
/// ```
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct EncryptedData {
    pub recipients: SequenceOfRecipientInfo,
    pub ciphertext: SymmetricCiphertext,
}

/// Transfers data encryption key to individual recipients of `EncryptedData`.
/// See Annex C.7 for guidance on appropriate use of each approach.
///
/// # Variants
/// ## `PskRecipInfo`
/// Data encrypted directly using pre-shared symmetric key
/// - Used with static encryption key approach (5.3.4)
///
/// ## `SymmRecipInfo`
/// Data encrypted with data encryption key, which was encrypted using symmetric key
/// - Used with ephemeral encryption key approach (5.3.4)
///
/// ## `CertRecipInfo`
/// Data encrypted with data encryption key, which was encrypted using public key from certificate
/// - Used with ephemeral encryption key approach (5.3.4)
/// - For ECIES: Parameter P1 (5.3.5) is certificate hash
///   - Using whole-certificate hash algorithm (6.4.3)
///   - Applied to COER-encoded, canonicalized certificate
/// - For SM2: No P1 parameter equivalent
///
/// ## `SignedDataRecipInfo`
/// Data encrypted with data encryption key, which was encrypted using public key from SignedData
/// - For ECIES: Parameter P1 (5.3.5) is SHA-256 hash of canonicalized `Ieee1609Dot2Data`
/// - For SM2: No P1 parameter equivalent
///
/// ## `RekRecipInfo`
/// Data encrypted with data encryption key, which was encrypted using public key from other source
/// - SDEE specification defines public key source
/// - For ECIES: Parameter P1 (5.3.5) is hash of empty string
/// - For SM2: No P1 parameter equivalent
///
/// # Implementation Note
/// Encryption key input is raw bytes without headers, encapsulation, or length indication
/// (unlike data encryption, where data is encapsulated in `Ieee1609Dot2Data`)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum RecipientInfo {
    PskRecipInfo(PreSharedKeyRecipientInfo),
    SymmRecipInfo(SymmRecipientInfo),
    CertRecipInfo(PKRecipientInfo),
    SignedDataRecipInfo(PKRecipientInfo),
    RekRecipInfo(PKRecipientInfo),
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfRecipientInfo(pub SequenceOf<RecipientInfo>);

delegate!(SequenceOf<RecipientInfo>, SequenceOfRecipientInfo);

/// Indicates a symmetric key that may be used directly to decrypt a `SymmetricCiphertext`.
///
/// # Value Calculation
/// Contains the low-order 8 bytes of a hash calculated from:
/// - COER encoding of a `SymmetricEncryptionKey` structure containing the symmetric key
/// - Hash algorithm determined as specified in 5.3.9.3
///
/// # Note
/// The symmetric key may be established by any appropriate means agreed by the
/// two parties to the exchange.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct PreSharedKeyRecipientInfo(pub HashedId8);

delegate!(HashedId8, PreSharedKeyRecipientInfo);

/// Contains recipient identification and encrypted data encryption key information.
///
/// # Fields
/// ## `recipient_id`
/// Hash of the symmetric key encryption key for decrypting the data encryption key:
/// - Contains low-order 8 bytes of hash
/// - Calculated from COER encoding of `SymmetricEncryptionKey` structure
/// - Uses hash algorithm specified in 5.3.9.4
/// - Symmetric key may be established by any agreed means between parties
///
/// ## `enc_key`
/// Contains encrypted data encryption key in `SymmetricCiphertext`:
/// - Key is input to encryption process without headers, encapsulation,
///   or length indication
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct SymmRecipientInfo {
    #[rasn(identifier = "recipientId")]
    pub recipient_id: HashedId8,
    #[rasn(identifier = "encKey")]
    pub enc_key: SymmetricCiphertext,
}

/// Contains recipient information and encrypted data encryption key.
///
/// # Fields
/// ## `recipient_id`
/// Contains hash of the encryption public key container, calculated differently
/// based on the containing `RecipientInfo` structure:
///
/// - For `certRecipInfo`:
///   - Contains `HashedId8` of the certificate
///   - Calculated using whole-certificate hash algorithm (per 6.4.3)
///   - Applied to COER-encoded certificate, canonicalized per Certificate definition
///
/// - For `signedDataRecipInfo`:
///   - Contains `HashedId8` of the `Ieee1609Dot2Data` (type signedData)
///   - Data canonicalized per 6.3.4
///   - Hash algorithm determined per 5.3.9.5
///
/// - For `rekRecipInfo`:
///   - Contains `HashedId8` of COER-encoded `PublicEncryptionKey`
///   - Hash algorithm determined per 5.3.9.5
///
/// ## `enc_key`
/// Contains the encrypted data encryption key:
/// - Key is input to encryption process without headers, encapsulation,
///   or length indication
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PKRecipientInfo {
    #[rasn(identifier = "recipientId")]
    pub recipient_id: HashedId8,
    #[rasn(identifier = "encKey")]
    pub enc_key: EncryptedDataEncryptionKey,
}

/// Contains encrypted data encryption key, input to encryption process without
/// headers, encapsulation, or length indication.
///
/// # Critical Information Field
/// If present and applicable to receiving SDEE, this is critical (per 5.2.6):
/// - If implementation receives encrypted SPDU and:
///   - Determines one or more `RecipientInfo` fields are relevant
///   - All relevant `RecipientInfos` contain unrecognized CHOICE in
///     `EncryptedDataEncryptionKey`
/// - Then implementation must indicate SPDU is not decryptable
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum EncryptedDataEncryptionKey {
    EciesNistP256(EciesP256EncryptedKey),
    EciesBrainpoolP256r1(EciesP256EncryptedKey),
    #[rasn(extension_addition)]
    EcencSm2256(EcencP256EncryptedKey),
}

/// Encapsulates a ciphertext generated with an approved symmetric algorithm.
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the indicated CHOICE value in an
///   encrypted SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SymmetricCiphertext {
    Aes128ccm(One28BitCcmCiphertext),
    #[rasn(extension_addition)]
    Sm4Ccm(One28BitCcmCiphertext),
}

/// Encapsulates encrypted ciphertext for symmetric algorithms with 128-bit blocks
/// in CCM mode.
///
/// # Characteristics
/// - Ciphertext is 16 bytes longer than plaintext (includes MAC)
/// - Successful decryption yields either:
///   - COER-encoded `Ieee1609Dot2Data` structure (see 6.3.41)
///   - 16-byte symmetric key (see 6.3.44)
///
/// # Fields
/// - `nonce`: Contains nonce N as specified in 5.3.8
/// - `ccm_ciphertext`: Contains ciphertext C as specified in 5.3.8
///
/// # Name Clarification
/// The "One28" in the structure name refers to:
/// - The symmetric cipher block size (128 bits)
/// - Not the key length (though AES-128-CCM and SM4-CCM happen to use 128-bit keys)
///
/// Since the cipher operates in counter mode (stream cipher):
/// - Block size only affects MAC size
/// - Does not affect raw ciphertext size
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct One28BitCcmCiphertext {
    #[rasn(size("12"))]
    pub nonce: FixedOctetString<12>,
    #[rasn(identifier = "ccmCiphertext")]
    pub ccm_ciphertext: Opaque,
}

/// This type is defined only for backwards compatibility.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Aes128CcmCiphertext(pub One28BitCcmCiphertext);

delegate!(One28BitCcmCiphertext, Aes128CcmCiphertext);

// ***************************************************************************
// **              Certificates and other Security Management               **
// ***************************************************************************

/// A profile of the `CertificateBase` structure specifying valid combinations
/// of fields for transmitting implicit and explicit certificates.
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2:
/// - Applies to the `CertificateBase`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Certificate(CertificateBase);

impl From<ImplicitCertificate> for Certificate {
    fn from(data: ImplicitCertificate) -> Self {
        Self(data.0)
    }
}
impl From<ExplicitCertificate> for Certificate {
    fn from(data: ExplicitCertificate) -> Self {
        Self(data.0)
    }
}

impl core::ops::Deref for Certificate {
    type Target = CertificateBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // We should not allow mutation of the inner data if they require validation!
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct TestCertificate(pub Certificate);

delegate!(Certificate, TestCertificate);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfCertificate(pub SequenceOf<Certificate>);

delegate!(SequenceOf<Certificate>, SequenceOfCertificate);

/// Certificate structure containing version, type, issuer, and content information.
///
/// # Fields
/// - `version`: Certificate format version (set to 3 in this version)
/// - `c_type`: Indicates explicit or implicit certificate
/// - `issuer`: Identifies the certificate issuer
/// - `to_be_signed`: Certificate contents, used in hash generation/verification
/// - `signature`: Present in `ExplicitCertificate`, calculated over `to_be_signed` hash
///
/// # Signature Calculation
/// Hash calculated as specified in 5.3.1:
/// - Data input: COER encoding of `to_be_signed`
/// - Signer identifier input:
///   - For self-signed (issuer is self): empty string
///   - Otherwise: Canonicalized COER encoding of issuer's certificate
///
/// # Canonicalization
/// Subject to canonicalization for operations in 6.1.2:
/// - Applies to `ToBeSignedCertificate` and `Signature`
///
/// # Whole-Certificate Hash
/// Used for calculating `HashedId3`, `HashedId8`, or `HashedId10`:
/// - Algorithm determined as specified in 5.3.9.2
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CertificateBase {
    #[rasn(value("3"))]
    pub version: Uint8,
    #[rasn(identifier = "type")]
    pub c_type: CertificateType,
    pub issuer: IssuerIdentifier,
    #[rasn(identifier = "toBeSigned")]
    pub to_be_signed: ToBeSignedCertificate,
    pub signature: Option<Signature>,
}
impl CertificateBase {
    #[must_use]
    pub const fn is_implicit(&self) -> bool {
        matches!(
            &self,
            CertificateBase {
                c_type: CertificateType::Implicit,
                to_be_signed: ToBeSignedCertificate {
                    verify_key_indicator: VerificationKeyIndicator::ReconstructionValue(_),
                    ..
                },
                signature: None,
                ..
            }
        )
    }
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        matches!(
            self,
            CertificateBase {
                c_type: CertificateType::Explicit,
                to_be_signed: ToBeSignedCertificate {
                    verify_key_indicator: VerificationKeyIndicator::VerificationKey(_),
                    ..
                },
                signature: Some(_),
                ..
            }
        )
    }
}

/// Indicates whether a certificate is explicit or implicit.
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.5:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CertificateType {
    Explicit = 0,
    Implicit = 1,
}

/// This is a profile of the CertificateBase structure providing all
///  the fields necessary for an implicit certificate, and no others.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct ImplicitCertificate(CertificateBase);

impl ImplicitCertificate {
    pub fn new(data: CertificateBase) -> Result<Self, InnerSubtypeConstraintError> {
        Self(data).validated()
    }
}
impl InnerSubtypeConstraint for ImplicitCertificate {
    fn validated(self) -> Result<Self, InnerSubtypeConstraintError> {
        if self.0.is_implicit() {
            Ok(self)
        } else {
            Err(InnerSubtypeConstraintError::InvalidCombination {
                type_name: "ImplicitCertificate",
                details: "CertificateBase is not implicit certificate",
            })
        }
    }
}

impl core::ops::Deref for ImplicitCertificate {
    type Target = CertificateBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // We should not allow mutation of the inner data if they require validation!
}

/// This is a profile of the CertificateBase structure providing all the fields necessary for an explicit certificate, and no others.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct ExplicitCertificate(CertificateBase);

impl ExplicitCertificate {
    pub fn new(data: CertificateBase) -> Result<Self, InnerSubtypeConstraintError> {
        Self(data).validated()
    }
}

impl InnerSubtypeConstraint for ExplicitCertificate {
    fn validated(self) -> Result<Self, InnerSubtypeConstraintError> {
        if self.0.is_explicit() {
            Ok(self)
        } else {
            Err(InnerSubtypeConstraintError::InvalidCombination {
                type_name: "ExplicitCertificate",
                details: "CertificateBase is not explicit certificate",
            })
        }
    }
}
impl core::ops::Deref for ExplicitCertificate {
    type Target = CertificateBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // We should not allow mutation of the inner data if they require validation!
}

/// Allows certificate recipients to determine which keying material to use for
/// certificate authentication.
///
/// # Variants
/// ## `Sha256AndDigest`, `Sha384AndDigest`, `Sm3AndDigest`
/// - Contains `HashedId8` of issuing certificate
///   - Calculated with whole-certificate hash algorithm (per 6.4.3)
///   - Applied to COER-encoded, canonicalized certificate
/// - Hash algorithm for certificate verification:
///   - `Sha256AndDigest`: SHA-256
///   - `Sha384AndDigest`: SHA-384
///   - `Sm3AndDigest`: SM3
/// - Certificate verified with public key of indicated issuing certificate
///
/// ## `VSelf`
/// - Indicates hash algorithm for certificate verification
/// - Certificate verified with public key from `verifyKeyIndicator` in `ToBeSignedCertificate`
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.5:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IssuerIdentifier {
    Sha256AndDigest(HashedId8),
    #[rasn(identifier = "self")]
    VSelf(HashAlgorithm),
    #[rasn(extension_addition)]
    Sha384AndDigest(HashedId8),
    #[rasn(extension_addition)]
    Sm3AndDigest(HashedId8),
}

/// Certificate data to be signed, containing identification and permissions information.
///
/// This is summary. For the most accurate information, consult the source ASN.1 definitions.
///
/// # Hash Calculation
/// For both implicit and explicit certificates, hash is calculated as:
/// `Hash(Data input) || Hash(Signer identifier input)` where:
/// - Data input: COER encoding of toBeSigned (canonicalized)
/// - Signer identifier input depends on verification type:
///   - For self-signed (issuer is self): empty string
///   - For certificate: COER encoding of canonicalized issuer certificate
///
/// For implicit certificates, H(CertU) from SEC 4 is:
/// `H[H(canonicalized ToBeSignedCertificate) || H(issuer Certificate)]`
/// See 5.3.2 for differences from SEC 4 regarding hash-to-integer conversion.
///
/// # Fields
/// - `id`: Certificate holder identification
/// - `craca_id`: Certificate Revocation Authorization CA identifier
///   - HashedId3 calculated with whole-certificate hash algorithm (6.4.3)
/// - `crl_series`: CRL series for CRACA
/// - `validity_period`: Certificate validity period
/// - `region`: Validity region (if omitted):
///   - Self-signed: Valid worldwide
///   - Otherwise: Same as issuing certificate
/// - `assurance_level`: Certificate holder's assurance level
/// - `app_permissions`: Application data signing permissions
///   - Each Psid can appear at most once
/// - `cert_issue_permissions`: Certificate signing permissions
///   - At most one entry with "all" permissions
/// - `cert_request_permissions`: Certificate request permissions
///   - At most one entry with "all" permissions
/// - `can_request_rollover`: Permission to request same-permission certificate
///   - Reserved for future use
/// - `encryption_key`: Public encryption key
/// - `verify_key_indicator`: Public key recovery material
/// - `flags`: Additional properties
///   - `usesCubk`: Supports compact unified butterfly key response
///     - Only relevant for CA certificates
///     - Used only for certificate response consistency checks
/// - `app_extensions`: Additional application activity permissions
/// - `cert_issue_extensions`: Additional certificate issuing permissions
/// - `cert_request_extensions`: Additional certificate request permissions
///
/// # Canonicalization
/// Subject to canonicalization (6.1.2):
/// - Applies to `PublicEncryptionKey` and `VerificationKeyIndicator`
/// - Elliptic curve points must use compressed form (`compressed-y-0`/`compressed-y-1`)
///
/// # Critical Information Fields
/// ## App Permissions
/// - Critical field per 5.2.6
/// - Must support at least 8 entries
/// - Invalid SPDU if verification fails
///
/// ## Cert Issue Permissions
/// - Critical field per 5.2.6
/// - Must support at least 8 entries
/// - Invalid SPDU if CA certificate chain verification fails
///
/// ## Cert Request Permissions
/// - Critical field per 5.2.6
/// - Must support at least 8 entries
/// - Invalid SPDU if request verification fails
///
/// Implementation-specific behavior when non-relevant fields exceed supported size.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedCertificate {
    pub id: CertificateId,
    #[rasn(identifier = "cracaId")]
    pub craca_id: HashedId3,
    #[rasn(identifier = "crlSeries")]
    pub crl_series: CrlSeries,
    #[rasn(identifier = "validityPeriod")]
    pub validity_period: ValidityPeriod,
    pub region: Option<GeographicRegion>,
    #[rasn(identifier = "assuranceLevel")]
    pub assurance_level: Option<SubjectAssurance>,
    #[rasn(identifier = "appPermissions")]
    pub app_permissions: Option<SequenceOfPsidSsp>,
    #[rasn(identifier = "certIssuePermissions")]
    pub cert_issue_permissions: Option<SequenceOfPsidGroupPermissions>,
    #[rasn(identifier = "certRequestPermissions")]
    pub cert_request_permissions: Option<SequenceOfPsidGroupPermissions>,
    #[rasn(identifier = "canRequestRollover")]
    pub can_request_rollover: Option<()>,
    #[rasn(identifier = "encryptionKey")]
    pub encryption_key: Option<PublicEncryptionKey>,
    #[rasn(identifier = "verifyKeyIndicator")]
    pub verify_key_indicator: VerificationKeyIndicator,
    #[rasn(extension_addition, size("8"))]
    pub flags: Option<FixedBitString<8>>,
    #[rasn(extension_addition, identifier = "appExtensions")]
    pub app_extensions: Option<SequenceOfAppExtensions>,
    #[rasn(extension_addition, identifier = "certIssueExtensions")]
    pub cert_issue_extensions: Option<SequenceOfCertIssueExtensions>,
    #[rasn(extension_addition, identifier = "certRequestExtension")]
    pub cert_request_extension: Option<SequenceOfCertRequestExtensions>,
}
#[bon::bon]
impl ToBeSignedCertificate {
    #[builder]
    pub fn new(
        id: CertificateId,
        craca_id: HashedId3,
        crl_series: CrlSeries,
        validity_period: ValidityPeriod,
        region: Option<GeographicRegion>,
        assurance_level: Option<SubjectAssurance>,
        app_permissions: Option<SequenceOfPsidSsp>,
        cert_issue_permissions: Option<SequenceOfPsidGroupPermissions>,
        cert_request_permissions: Option<SequenceOfPsidGroupPermissions>,
        can_request_rollover: Option<()>,
        encryption_key: Option<PublicEncryptionKey>,
        verify_key_indicator: VerificationKeyIndicator,
        flags: Option<FixedBitString<8>>,
        app_extensions: Option<SequenceOfAppExtensions>,
        cert_issue_extensions: Option<SequenceOfCertIssueExtensions>,
        cert_request_extension: Option<SequenceOfCertRequestExtensions>,
    ) -> Result<Self, InnerSubtypeConstraintError> {
        let cert = Self {
            id,
            craca_id,
            crl_series,
            validity_period,
            region,
            assurance_level,
            app_permissions,
            cert_issue_permissions,
            cert_request_permissions,
            can_request_rollover,
            encryption_key,
            verify_key_indicator,
            flags,
            app_extensions,
            cert_issue_extensions,
            cert_request_extension,
        };
        cert.validated()
    }
}

impl InnerSubtypeConstraint for ToBeSignedCertificate {
    fn validated(self) -> Result<Self, InnerSubtypeConstraintError> {
        if self.app_permissions.is_none()
            && self.cert_issue_permissions.is_none()
            && self.cert_request_permissions.is_none()
        {
            return Err(InnerSubtypeConstraintError::MissingAtLeastOneComponent {
                type_name: "ToBeSignedCertificate",
                components: &[
                    "app_permissions",
                    "cert_issue_permissions",
                    "cert_request_permissions",
                ],
            });
        }
        Ok(self)
    }
}

/// Contains information to identify the certificate holder when necessary.
///
/// # Variants
/// - `LinkageData`: Identifies certificate for revocation in linked certificate CRLs
///   (See sections 5.1.3 and 7.3 for details)
///
/// - `Name`: Identifies non-anonymous certificate holders
///   - Contents are policy-dependent and expected to be human-readable
///
/// - `BinaryId`: Supports non-human-readable identifiers
///
/// - `None`: Indicates certificate does not include an identifier
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the indicated CHOICE shall
///   reject the signed SPDU as invalid
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum CertificateId {
    LinkageData(LinkageData),
    Name(Hostname),
    #[rasn(size("1..=64"))]
    BinaryId(OctetString),
    None(()),
}

/// Contains information for matching against linkage ID-based CRLs to determine
/// certificate revocation status.
///
/// See sections 5.1.3.4 and 7.3 for detailed usage information.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct LinkageData {
    #[rasn(identifier = "iCert")]
    pub i_cert: IValue,
    #[rasn(identifier = "linkage-value")]
    pub linkage_value: LinkageValue,
    #[rasn(identifier = "group-linkage-value")]
    pub group_linkage_value: Option<GroupLinkageValue>,
}

/// Indicates permitted permission types in end-entity certificates whose permission
/// chain passes through this `PsidGroupPermissions` field.
///
/// # Variants
/// - `App`: End-entity certificate may contain `appPermissions` field
/// - `Enroll`: End-entity certificate may contain `certRequestPermissions` field
/// # Note
/// BIT STRING {app (0), enrol (1) } (SIZE (8)) (ALL EXCEPT {})
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size(8))]
pub struct EndEntityType(pub FixedBitString<8usize>);

/// When initializing `EndEntityType`, use the following constants as the first byte for `FixedBitString<8>`.
impl EndEntityType {
    pub const APP: u8 = 0b1000_0000;
    pub const ENROLL: u8 = 0b0100_0000;
    pub const BOTH: u8 = 0b1100_0000;
}

delegate!(FixedBitString<8>, EndEntityType);

/// Specifies permissions for certificate issuance and requests for a set of PSIDs.
/// See examples in D.5.3 and D.5.4.
///
/// # Fields
/// - `subject_permissions`: PSIDs and SSP Ranges covered
///
/// - `min_chain_length` and `chain_length_range`: Permitted certificate chain length
///   - Chain length: Number of certificates below this one, including end-entity
///   - Permitted length: `min_chain_length` to `min_chain_length + chain_length_range`
///   - Special cases:
///     - `min_chain_length` of 0 not allowed in `certIssuePermissions`
///     - `chain_length_range` of -1 allows any length ≥ `min_chain_length`
///
/// - `ee_type`: Types of certificates/requests this permission authorizes
///   - `app`: Allows chain to end in authorization certificate
///   - `enroll`: Allows chain to end in enrollment certificate
///   - Different `PsidGroupPermissions` instances may have different `ee_type` values
///
/// # Validation Rules
/// - Chain ending in authorization certificate requires `app` in `ee_type`
/// - Chain ending in enrollment certificate requires `enroll` in `ee_type`
/// - Invalid chain if end certificate type doesn't match `ee_type`
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct PsidGroupPermissions {
    #[rasn(identifier = "subjectPermissions")]
    pub subject_permissions: SubjectPermissions,
    #[rasn(
        default = "psid_group_permissions_min_chain_length_default",
        identifier = "minChainLength"
    )]
    #[builder(default = psid_group_permissions_min_chain_length_default())]
    pub min_chain_length: Integer,
    #[rasn(
        default = "psid_group_permissions_chain_length_range_default",
        identifier = "chainLengthRange"
    )]
    #[builder(default = psid_group_permissions_chain_length_range_default())]
    pub chain_length_range: Integer,
    #[rasn(
        default = "psid_group_permissions_ee_type_default",
        identifier = "eeType"
    )]
    #[builder(default = psid_group_permissions_ee_type_default())]
    pub ee_type: EndEntityType,
}

fn psid_group_permissions_min_chain_length_default() -> Integer {
    Integer::from(1)
}
fn psid_group_permissions_chain_length_range_default() -> Integer {
    Integer::from(0)
}
fn psid_group_permissions_ee_type_default() -> EndEntityType {
    // First byte holds the bits, container is larger than needed
    EndEntityType(FixedBitString::new([
        EndEntityType::APP,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]))
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfPsidGroupPermissions(pub SequenceOf<PsidGroupPermissions>);

delegate!(
    SequenceOf<PsidGroupPermissions>,
    SequenceOfPsidGroupPermissions
);

/// Indicates PSIDs and associated SSPs for certificate issuance or request
/// permissions granted by a `PsidGroupPermissions` structure.
///
/// # Variants
/// - `Explicit`: Grants permissions for specific PSIDs and SSP Ranges
/// - `All`: Grants permissions for all PSIDs not covered by other
///   `PsidGroupPermissions` in the same `certIssuePermissions` or
///   `certRequestPermissions` field
///
/// # Critical Information Fields
/// This is a critical information field as defined in 5.2.6:
/// - Implementation must recognize the indicated CHOICE when verifying SPDU
/// - For `Explicit` variant:
///   - Must support at least 8 entries in `PsidSspRange`
///   - Implementation must support specified number of entries
/// - Invalid SPDU (per 4.2.2.3.2) if:
///   - CHOICE is not recognized
///   - Number of entries in `Explicit` variant exceeds implementation support
///
/// Invalid in this context means validity cannot be established.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SubjectPermissions {
    Explicit(SequenceOfPsidSspRange),
    All(()),
}

/// Contains either verification key or reconstruction value depending on certificate type.
///
/// # Variants
/// - `VerificationKey`: For explicit certificates
///   - Contains public key for verifying holder's signatures
///
/// - `ReconstructionValue`: For implicit certificates
///   - Contains value for recovering public key (per SEC 4 and 5.3.2)
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.5:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2:
/// - Applies to `PublicVerificationKey` and `EccP256CurvePoint`
/// - `EccP256CurvePoint` must be in compressed form:
///   - Choice must be either `compressed-y-0` or `compressed-y-1`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VerificationKeyIndicator {
    VerificationKey(PublicVerificationKey),
    ReconstructionValue(EccP256CurvePoint),
}

/// Represents an individual `AppExtension`.
///
/// Extensions are drawn from the ASN.1 Information Object Set `SetCertExtensions`.
/// Each `AppExtension` is associated with a `CertIssueExtension` and a
/// `CertRequestExtension`, all identified by the same `id` value.
///
/// # Fields
/// - `id`: Identifies the extension type
/// - `content`: Provides the content of the extension
#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct AppExtension<T: CertExtType> {
    pub id: ExtId,
    pub content: T::App,
}

/// Contains `CertRequestExtensions` that apply to the certificate holder.
///
/// # Consistency Requirements
/// As specified in 5.2.4.2.3, each `CertRequestExtension` type has specific
/// consistency conditions governing:
/// - Consistency with `AppExtensions` in certificates issued by the holder
/// - Consistency with `CertRequestExtensions` in CA certificates in the holder's chain
///
/// See individual `CertRequestExtension` definitions for their specific
/// consistency conditions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfCertIssueExtensions(pub SequenceOf<OperatingOrganizationIssueExtension>);

delegate!(
    SequenceOf<OperatingOrganizationIssueExtension>,
    SequenceOfCertIssueExtensions
);

#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum IssuePermissions<T: CertExtType> {
    #[rasn(identifier = "specific")]
    Specific(T::Issue),
    #[rasn(identifier = "all")]
    All(()),
}

/// Represents an individual `CertIssueExtension`.
///
/// Extensions are drawn from the ASN.1 Information Object Set `SetCertExtensions`.
/// Each `CertIssueExtension` is associated with an `AppExtension` and a
/// `CertRequestExtension`, all identified by the same `id` value.
///
/// # Fields
/// - `id`: Identifies the extension type
///
/// - `permissions`: Indicates the permissions
///   - `All`: Certificate is entitled to issue all values of the extension
///   - `Specific`: Specifies which extension values may be issued when `All` doesn't apply
#[derive(Builder, AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CertIssueExtension<T: CertExtType> {
    pub id: ExtId,
    pub permissions: IssuePermissions<T>,
}
impl<T: CertExtType> CertIssueExtension<T> {
    pub fn new_specific(permissions: T::Issue) -> Self {
        Self {
            id: T::ID,
            permissions: IssuePermissions::Specific(permissions),
        }
    }
    pub fn new_all() -> Self {
        Self {
            id: T::ID,
            permissions: IssuePermissions::All(()),
        }
    }
}

/// Contains `CertRequestExtensions` that apply to the certificate holder.
///
/// # Consistency Requirements
/// As specified in 5.2.4.2.3, each `CertRequestExtension` type has specific
/// consistency conditions governing:
/// - Consistency with `AppExtensions` in certificates issued by the holder
/// - Consistency with `CertRequestExtensions` in CA certificates in the holder's chain
///
/// See individual `CertRequestExtension` definitions for their specific
/// consistency conditions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfCertRequestExtensions(pub SequenceOf<OperatingOrganizationRequestExtension>);

delegate!(
    SequenceOf<OperatingOrganizationRequestExtension>,
    SequenceOfCertRequestExtensions
);

#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
pub enum RequestPermissions<T: CertExtType> {
    #[rasn(identifier = "content")]
    Content(T::Req),
    #[rasn(identifier = "all")]
    All(()),
}

/// Represents an individual `CertRequestExtension`.
///
/// Extensions are drawn from the ASN.1 Information Object Set `SetCertExtensions`.
/// Each `CertRequestExtension` is associated with an `AppExtension` and a
/// `CertRequestExtension`, all identified by the same `id` value.
///
/// # Fields
/// - `id`: Identifies the extension type
///
/// - `permissions`: Indicates the permissions
///   - `All`: Certificate is entitled to issue all values of the extension
///   - `Specific`: Specifies which extension values may be issued when `All` doesn't apply
#[derive(Builder, AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CertRequestExtension<T: CertExtType> {
    pub id: ExtId,
    pub permissions: RequestPermissions<T>,
}

/// AppExtension used to identify an operating organization. Both associated
/// `CertIssueExtension` and `CertRequestExtension` are of type `OperatingOrganizationId`.
///
/// # SPDU Consistency
/// SDEE specification for the SPDU must specify how to determine an OBJECT
/// IDENTIFIER, either by:
/// - Including the full OBJECT IDENTIFIER in the SPDU
/// - Including a RELATIVE-OID with instructions for obtaining full OBJECT IDENTIFIER
///
/// SPDU is consistent if its determined OBJECT IDENTIFIER matches this field's value.
///
/// # Extension Properties
/// - No consistency conditions with corresponding `CertIssueExtension`
/// - Can appear in certificates issued by any CA
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct OperatingOrganizationId(pub ObjectIdentifier);

delegate!(ObjectIdentifier, OperatingOrganizationId);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct OperatingOrganizationExtension;

impl CertExtType for OperatingOrganizationExtension {
    type App = OperatingOrganizationId;
    type Issue = (); // NULL in ASN.1
    type Req = (); // NULL in ASN.1
    const ID: ExtId = CERT_EXT_ID_OPERATING_ORGANIZATION;
}

pub type OperatingOrganizationAppExtension = AppExtension<OperatingOrganizationExtension>;
pub type OperatingOrganizationIssueExtension = CertIssueExtension<OperatingOrganizationExtension>;
pub type OperatingOrganizationRequestExtension =
    CertRequestExtension<OperatingOrganizationExtension>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetCertExtensionsType {
    OperatingOrganization(OperatingOrganizationExtension),
}

/// Contains `AppExtensions` that apply to the certificate holder.
///
/// # Consistency Requirements
/// As specified in 5.2.4.2.3, each `AppExtension` type has specific
/// consistency conditions governing:
/// - Consistency with SPDUs signed by the holder
/// - Consistency with `CertIssueExtensions` in CA certificates in the holder's chain
/// # Note
/// See individual `AppExtension` definitions for their specific
/// consistency conditions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
pub struct SequenceOfAppExtensions(pub SequenceOf<OperatingOrganizationAppExtension>);

delegate!(
    SequenceOf<OperatingOrganizationAppExtension>,
    SequenceOfAppExtensions
);

// pub mod ieee1609_dot2_crl {
//     extern crate alloc;
//     use super::ieee1609_dot2::Ieee1609Dot2Data;
//     use super::ieee1609_dot2_base_types::{Opaque, Psid};
//     use super::ieee1609_dot2_crl_base_types::CrlContents;
//     use core::borrow::Borrow;
//     use rasn::prelude::*;
//     #[doc = "*"]
//     #[doc = " * @brief This is the PSID for the CRL application."]
//     #[doc = " "]
//     #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
//     #[rasn(delegate, value("256"))]
//     pub struct CrlPsid(pub Psid);
//     #[doc = "*"]
//     #[doc = " * @brief This structure is the SPDU used to contain a signed CRL. A valid "]
//     #[doc = " * signed CRL meets the validity criteria of 7.4."]
//     #[doc = " "]
//     #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
//     #[rasn(delegate)]
//     pub struct SecuredCrl(pub Ieee1609Dot2Data);
// }

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! round_trip {
        ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
            let value: $typ = $value;
            let expected: &[u8] = $expected;
            let actual_encoding = rasn::$codec::encode(&value).unwrap();
            // dbg!(&actual_encoding);

            pretty_assertions::assert_eq!(&*actual_encoding, expected);

            let decoded_value = rasn::$codec::decode::<$typ>(&actual_encoding);
            match decoded_value {
                Ok(decoded) => {
                    pretty_assertions::assert_eq!(value, decoded);
                }
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }};
    }

    #[test]
    fn test_signed_data_payload() {
        let payload_data = SignedDataPayload::builder()
            .data(
                Ieee1609Dot2Data::builder()
                    .protocol_version(3)
                    .content(Ieee1609Dot2Content::UnsecuredData(
                        OctetString::from("This is a BSM\r\n".as_bytes()).into(),
                    ))
                    .build(),
            )
            .build()
            .unwrap();
        round_trip!(
            coer,
            SignedDataPayload,
            payload_data,
            // 0x40, 0x03, 0x80, 0x0F, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x42, 0x53, 0x4D, 0x0D, 0x0A
            &[64, 3, 128, 15, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 66, 83, 77, 13, 10]
        );
    }
    #[test]
    fn test_ieee_basic_safety_message() {
        let ieee1609dot2data: Ieee1609Dot2Data = Ieee1609Dot2Data::builder()
            .protocol_version(3)
            .content(Ieee1609Dot2Content::SignedData(Box::new(
                SignedData::builder()
                    .hash_id(HashAlgorithm::Sha256)
                    .tbs_data(
                        ToBeSignedData::builder()
                            .payload(
                                SignedDataPayload::builder()
                                    .data(
                                        Ieee1609Dot2Data::builder()
                                            .protocol_version(3)
                                            .content(Ieee1609Dot2Content::UnsecuredData(
                                                OctetString::from("This is a BSM\r\n".as_bytes())
                                                    .into(),
                                            ))
                                            .build(),
                                    )
                                    .build()
                                    .unwrap(),
                            )
                            .header_info(
                                HeaderInfo::builder()
                                    .psid(Integer::from(32).into())
                                    .generation_time(1_230_066_625_199_609_624.into())
                                    .build(),
                            )
                            .build(),
                    )
                    .signer(SignerIdentifier::Digest(HashedId8(
                        "!\"#$%&'(".as_bytes().try_into().unwrap(),
                    )))
                    .signature(Signature::EcdsaNistP256(
                        EcdsaP256Signature::builder()
                            .r_sig(EccP256CurvePoint::CompressedY0(
                                b"12345678123456781234567812345678"
                                    .to_vec()
                                    .try_into()
                                    .unwrap(),
                            ))
                            .s_sig(
                                b"ABCDEFGHABCDEFGHABCDEFGHABCDEFGH"
                                    .to_vec()
                                    .try_into()
                                    .unwrap(),
                            )
                            .build(),
                    ))
                    .build(),
            )))
            .build();

        round_trip!(
            coer,
            Ieee1609Dot2Data,
            ieee1609dot2data,
            &[
                0x03, 0x81, 0x00, 0x40, 0x03, 0x80, 0x0F, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
                0x20, 0x61, 0x20, 0x42, 0x53, 0x4D, 0x0D, 0x0A, 0x40, 0x01, 0x20, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17, 0x18, 0x80, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
                0x80, 0x82, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34,
                0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32,
                0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
                0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
                0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
            ]
        );
    }
    #[test]
    fn test_bsm_with_cert() {
        round_trip!(
            coer,
            Ieee1609Dot2Data,
            Ieee1609Dot2Data::builder()
                .protocol_version(3)
                .content(Ieee1609Dot2Content::SignedData(Box::new(
                    SignedData::builder()
                        .hash_id(HashAlgorithm::Sha256)
                        .tbs_data(
                            ToBeSignedData::builder()
                                .payload(
                                    SignedDataPayload::builder()
                                        .data(
                                            Ieee1609Dot2Data::builder()
                                                .protocol_version(3)
                                                .content(Ieee1609Dot2Content::UnsecuredData(
                                                    Opaque("This is a BSM\r\n".as_bytes().into())
                                                ))
                                                .build(),
                                        )
                                        .build()
                                        .unwrap(),
                                )
                                .header_info(
                                    HeaderInfo::builder()
                                        .psid(Integer::from(32).into())
                                        .generation_time(1_230_066_625_199_609_624.into())
                                        .build(),
                                )
                                .build()
                        )
                        .signer(SignerIdentifier::Certificate(
                            vec![Certificate::from(
                                ImplicitCertificate::new(
                                    CertificateBase::builder()
                                        .version(3)
                                        .c_type(CertificateType::Implicit)
                                        .issuer(IssuerIdentifier::Sha256AndDigest(HashedId8(
                                            "!\"#$%&'(".as_bytes().try_into().unwrap()
                                        )))
                                        .to_be_signed(
                                            ToBeSignedCertificate::builder()
                                                .id(CertificateId::LinkageData(
                                                    LinkageData::builder()
                                                        .i_cert(IValue::from(100))
                                                        .linkage_value(LinkageValue(
                                                            FixedOctetString::try_from(
                                                                b"123456789".as_slice()
                                                            )
                                                            .unwrap()
                                                        ))
                                                        .group_linkage_value(
                                                            GroupLinkageValue::builder()
                                                                .j_value(
                                                                    b"ABCD"
                                                                        .as_slice()
                                                                        .try_into()
                                                                        .unwrap()
                                                                )
                                                                .value(
                                                                    b"QRSTUVWXY"
                                                                        .as_slice()
                                                                        .try_into()
                                                                        .unwrap()
                                                                )
                                                                .build()
                                                        )
                                                        .build()
                                                ))
                                                .craca_id(HashedId3(
                                                    b"abc".as_slice().try_into().unwrap()
                                                ))
                                                .crl_series(CrlSeries::from(70))
                                                .validity_period(
                                                    ValidityPeriod::builder()
                                                        .start(81_828_384.into())
                                                        .duration(Duration::Hours(169))
                                                        .build()
                                                )
                                                .region(GeographicRegion::IdentifiedRegion(
                                                    vec![
                                                        IdentifiedRegion::CountryOnly(
                                                            UnCountryId::from(124)
                                                        ),
                                                        IdentifiedRegion::CountryOnly(
                                                            UnCountryId::from(484)
                                                        ),
                                                        IdentifiedRegion::CountryOnly(
                                                            UnCountryId::from(840)
                                                        )
                                                    ]
                                                    .into()
                                                ))
                                                .app_permissions(
                                                    vec![
                                                        PsidSsp {
                                                            psid: Integer::from(32).into(),
                                                            ssp: None
                                                        },
                                                        PsidSsp {
                                                            psid: Integer::from(38).into(),
                                                            ssp: None
                                                        }
                                                    ]
                                                    .into()
                                                )
                                                .verify_key_indicator(
                                                    VerificationKeyIndicator::ReconstructionValue(
                                                        EccP256CurvePoint::CompressedY0(
                                                            FixedOctetString::from([
                                                                0x91u8, 0x92, 0x93, 0x94, 0x95,
                                                                0x96, 0x97, 0x98, 0x91, 0x92, 0x93,
                                                                0x94, 0x95, 0x96, 0x97, 0x98, 0x91,
                                                                0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
                                                                0x98, 0x91, 0x92, 0x93, 0x94, 0x95,
                                                                0x96, 0x97, 0x98
                                                            ])
                                                        )
                                                    )
                                                )
                                                .build()
                                                .unwrap()
                                        )
                                        .build()
                                )
                                .unwrap()
                            )]
                            .into()
                        ))
                        .signature(Signature::EcdsaNistP256(
                            EcdsaP256Signature::builder()
                                .r_sig(EccP256CurvePoint::CompressedY0(
                                    b"12345678123456781234567812345678"
                                        .as_slice()
                                        .try_into()
                                        .unwrap()
                                ))
                                .s_sig(
                                    b"ABCDEFGHABCDEFGHABCDEFGHABCDEFGH"
                                        .as_slice()
                                        .try_into()
                                        .unwrap()
                                )
                                .build()
                        ))
                        .build()
                )))
                .build(),
            // Standard-provided verification sample
            // 03 81 00 40 03 80 0F 54 68 69 73 20 69 73 20 61
            // 20 42 53 4D 0D 0A 40 01 20 11 12 13 14 15 16 17
            // 18 81 01 01 00 03 01 80 21 22 23 24 25 26 27 28
            // 50 80 80 00 64 31 32 33 34 35 36 37 38 39 41 42
            // 43 44 51 52 53 54 55 56 57 58 59 61 62 63 00 46
            // 04 E0 9A 20 84 00 A9 83 01 03 80 00 7C 80 01 E4
            // 80 03 48 01 02 00 01 20 00 01 26 81 82 91 92 93
            // 94 95 96 97 98 91 92 93 94 95 96 97 98 91 92 93
            // 94 95 96 97 98 91 92 93 94 95 96 97 98 80 82 31
            // 32 33 34 35 36 37 38 31 32 33 34 35 36 37 38 31
            // 32 33 34 35 36 37 38 31 32 33 34 35 36 37 38 41
            // 42 43 44 45 46 47 48 41 42 43 44 45 46 47 48 41
            // 42 43 44 45 46 47 48 41 42 43 44 45 46 47 48
            //
            // asnc1 output, compiled from standard:
            // 0x03, 0x81, 0x00, 0x40, 0x03, 0x80, 0x0f, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
            // 0x20, 0x61, 0x20, 0x42, 0x53, 0x4d, 0x0d, 0x0a, 0x40, 0x01, 0x20, 0x11, 0x12, 0x13,
            // 0x14, 0x15, 0x16, 0x17, 0x18, 0x81, 0x01, 0x01, 0x00, 0x03, 0x01, 0x80, 0x21, 0x22,
            // 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x50, 0x80, 0x80, 0x00, 0x64, 0x31, 0x32, 0x33,
            // 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x51, 0x52, 0x53, 0x54,
            // 0x55, 0x56, 0x57, 0x58, 0x59, 0x61, 0x62, 0x63, 0x00, 0x46, 0x04, 0xe0, 0x9a, 0x20,
            // 0x84, 0x00, 0xa9, 0x83, 0x01, 0x03, 0x80, 0x00, 0x7c, 0x80, 0x01, 0xe4, 0x80, 0x03,
            // 0x48, 0x01, 0x02, 0x00, 0x01, 0x20, 0x00, 0x01, 0x26, 0x81, 0x82, 0x91, 0x92, 0x93,
            // 0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91,
            // 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
            // 0x98, 0x80, 0x82, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33,
            // 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31,
            // 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
            // 0x48, 0x41, 0x42, 0x43, 0x44,0x45, 0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45,
            // 0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48
            //
            // NOTE below output is modified to be encoded as the extension version is being used (ToBeSignedCertificate extensions not optional)
            // &[
            //     0x03, 0x81, 0x00, 0x40, 0x03, 0x80, 0x0F, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
            //     0x20, 0x61, 0x20, 0x42, 0x53, 0x4D, 0x0D, 0x0A, 0x40, 0x01, 0x20, 0x11, 0x12, 0x13,
            //     0x14, 0x15, 0x16, 0x17, 0x18, 0x81, 0x01, 0x01, 0x00, 0x03, 0x01, 0x80, 0x21, 0x22,
            //     0x23, 0x24, 0x25, 0x26, 0x27, 0x28, // 0x50, - no extension present version
            //     0xd0, // preamble ends
            //     0x80, 0x80, 0x00, 0x64, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41,
            //     0x42, 0x43, 0x44, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x61, 0x62,
            //     0x63, 0x00, 0x46, 0x04, 0xE0, 0x9A, 0x20, 0x84, 0x00, 0xA9, 0x83, 0x01, 0x03, 0x80,
            //     0x00, 0x7C, 0x80, 0x01, 0xE4, 0x80, 0x03, 0x48, 0x01, 0x02, 0x00, 0x01, 0x20, 0x00,
            //     0x01, 0x26, 0x81, 0x82, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92,
            //     0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98,
            //     0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
            //     0x98, // extension bitmap starts for TBS cert
            //     0x02, 0x04, 0x70, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x02, 0x01,
            //     0x00, // ends
            //     0x80, 0x82, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34,
            //     0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32,
            //     0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
            //     0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
            //     0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48
            // ]
            &[
                0x03, 0x81, 0x00, 0x40, 0x03, 0x80, 0x0F, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
                0x20, 0x61, 0x20, 0x42, 0x53, 0x4D, 0x0D, 0x0A, 0x40, 0x01, 0x20, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17, 0x18, 0x81, 0x01, 0x01, 0x00, 0x03, 0x01, 0x80, 0x21, 0x22,
                0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x50, 0x80, 0x80, 0x00, 0x64, 0x31, 0x32, 0x33,
                0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x51, 0x52, 0x53, 0x54,
                0x55, 0x56, 0x57, 0x58, 0x59, 0x61, 0x62, 0x63, 0x00, 0x46, 0x04, 0xE0, 0x9A, 0x20,
                0x84, 0x00, 0xA9, 0x83, 0x01, 0x03, 0x80, 0x00, 0x7C, 0x80, 0x01, 0xE4, 0x80, 0x03,
                0x48, 0x01, 0x02, 0x00, 0x01, 0x20, 0x00, 0x01, 0x26, 0x81, 0x82, 0x91, 0x92, 0x93,
                0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91,
                0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
                0x98, 0x80, 0x82, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33,
                0x34, 0x35, 0x36, 0x37, 0x38, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x31,
                0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
                0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45,
                0x46, 0x47, 0x48, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48
            ]
        );
    }
}
