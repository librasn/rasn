extern crate alloc;
use super::base_types::{
    CrlSeries, HashedId10, HashedId8, IValue, LaId, LinkageSeed, SequenceOfLinkageSeed, Time32,
    Uint16, Uint32, Uint8,
};
use crate::delegate;
use bon::Builder;
use rasn::prelude::*;

/// A Certificate Revocation List (CRL) structure containing revocation information.
///
/// # Fields
///
/// * `version` - The version number of the CRL. For this version of the standard it is 1.
///
/// * `crl_series` - Represents the CRL series to which this CRL belongs. Used to determine
///   whether the revocation information in a CRL is relevant to a particular certificate
///   as specified in 5.1.3.2.
///
/// * `crl_craca` - Contains the low-order eight octets of the hash of the Certificate
///   Revocation Authorization CA (CRACA) certificate that authorized this CRL's issuance.
///   Used to determine revocation information relevance as specified in 5.1.3.2.
///   In a valid signed CRL (see 7.4), this must be consistent with the `associatedCraca`
///   field in the Service Specific Permissions (7.4.3.3). The HashedId8 is calculated using
///   the whole-certificate hash algorithm (6.4.3) applied to the COER-encoded certificate.
///
/// * `issue_date` - Specifies when the CRL was issued.
///
/// * `next_crl` - Contains the expected issuance time for the next CRL with the same
///   `crl_series` and `craca_id`. Must be strictly after `issue_date` for the CRL to be valid.
///   Used to set the expected update time for revocation information associated with the
///   (`crl_craca`, `crl_series`) pair as specified in 5.1.3.6.
///
/// * `priority_info` - Contains information to help devices with limited storage space
///   determine which revocation information to retain or discard.
///
/// * `type_specific` - Contains the CRL body.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct CrlContents {
    #[rasn(value("1"))]
    pub version: Uint8,
    #[rasn(identifier = "crlSeries")]
    pub crl_series: CrlSeries,
    #[rasn(identifier = "crlCraca")]
    pub crl_craca: HashedId8,
    #[rasn(identifier = "issueDate")]
    pub issue_date: Time32,
    #[rasn(identifier = "nextCrl")]
    pub next_crl: Time32,
    #[rasn(identifier = "priorityInfo")]
    pub priority_info: CrlPriorityInfo,
    #[rasn(identifier = "typeSpecific")]
    pub type_specific: TypeSpecificCrlContents,
}

/// Information to help devices with limited storage space determine which revocation
/// information to retain or discard.
///
/// # Fields
///
/// * `priority` - Indicates the relative priority of this revocation information compared
///   to other CRLs issued for certificates with the same `craca_id` and `crl_series`.
///   Higher values indicate higher importance.
///
/// # Note
///
/// This mechanism is for future use; details are not specified in this version of the standard.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CrlPriorityInfo {
    pub priority: Option<Uint8>,
}

/// Type-specific CRL contents.
///
/// # Variants
///
/// * `FullHashCrl` - A full hash-based CRL listing hashes of all certificates that:
///   - Contain the indicated `craca_id` and `crl_series` values
///   - Are revoked by hash
///   - Have been revoked
///   - Have not expired
///
/// * `DeltaHashCrl` - A delta hash-based CRL listing hashes of all certificates that:
///   - Contain the indicated `craca_id` and `crl_series` values
///   - Are revoked by hash
///   - Have been revoked since the previous CRL with the same `craca_id` and `crl_series`
///
/// * `FullLinkedCrl` and `FullLinkedCrlWithAlg` - Full linkage ID-based CRLs listing
///   individual and/or group linkage data for all certificates that:
///   - Contain the indicated `craca_id` and `crl_series` values
///   - Are revoked by linkage value
///   - Have been revoked
///   - Have not expired
///
/// * `DeltaLinkedCrl` and `DeltaLinkedCrlWithAlg` - Delta linkage ID-based CRLs listing
///   individual and/or group linkage data for all certificates that:
///   - Contain the specified `craca_id` and `crl_series` values
///   - Are revoked by linkage data
///   - Have been revoked since the previous CRL with the same identifiers
///
/// The difference between `*LinkedCrl` and `*LinkedCrlWithAlg` variants is in how the
/// cryptographic algorithms for seed evolution and linkage value generation (5.1.3.4)
/// are communicated to the receiver.
///
/// # Notes
///
/// - Once a certificate is revoked, it remains revoked for its lifetime. CRL signers
///   should include revoked certificates on all CRLs between revocation and expiry.
///
/// - Seed evolution and linkage value generation function identification:
///   - For `*WithAlg` variants: functions are specified explicitly in the structure
///   - For regular variants: functions are determined by the `crl_craca` hash algorithm:
///     - SHA-256/384: `seedEvoFn1-sha256` and `lvGenFn1-aes128`
///     - SM3: `seedEvoFn1-sm3` and `lvGenFn1-sm4`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum TypeSpecificCrlContents {
    FullHashCrl(ToBeSignedHashIdCrl),
    DeltaHashCrl(ToBeSignedHashIdCrl),
    FullLinkedCrl(ToBeSignedLinkageValueCrl),
    DeltaLinkedCrl(ToBeSignedLinkageValueCrl),
    #[rasn(extension_addition)]
    FullLinkedCrlWithAlg(ToBeSignedLinkageValueCrlWithAlgIdentifier),
    #[rasn(extension_addition)]
    DeltaLinkedCrlWithAlg(ToBeSignedLinkageValueCrlWithAlgIdentifier),
}

/// Information about a revoked certificate.
///
/// # Fields
///
/// * `crl_serial` - Counter that increments by 1 every time a new full or delta CRL
///   is issued for the indicated `crl_craca` and `crl_series` values.
///
/// * `entries` - Contains the individual revocation information items.
///
/// # Note
///
/// To indicate a hash-based CRL contains no individual revocation information items,
/// the recommended approach is to use an empty SEQUENCE OF in the
/// `SequenceOfHashBasedRevocationInfo`.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedHashIdCrl {
    #[rasn(identifier = "crlSerial")]
    pub crl_serial: Uint32,
    pub entries: SequenceOfHashBasedRevocationInfo,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfHashBasedRevocationInfo(pub SequenceOf<HashBasedRevocationInfo>);

delegate!(
    SequenceOf<HashBasedRevocationInfo>,
    SequenceOfHashBasedRevocationInfo
);

/// Information about a hash-based revoked certificate.
///
/// # Fields
///
/// * `id` - The HashedId10 identifying the revoked certificate. Calculated using the
///   whole-certificate hash algorithm (as described in 6.4.3) applied to the COER-encoded
///   certificate, canonicalized as defined in the Certificate definition.
///
/// * `expiry` - Value computed from the validity period's start and duration values
///   in the certificate.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct HashBasedRevocationInfo {
    pub id: HashedId10,
    pub expiry: Time32,
}

/// CRL structure containing linkage-based revocation information.
///
/// # Fields
///
/// * `i_rev` - The value used in the algorithm specified in 5.1.3.4. Applies to all
///   linkage-based revocation information included within both individual and groups.
///
/// * `index_within_i` - Counter that starts at 0 for the first CRL issued for a specific
///   combination of `crl_craca`, `crl_series`, and `i_rev`. Increments by 1 for each new
///   full or delta CRL issued with the same identifiers but without changing `i_rev`.
///
/// * `individual` - Contains individual linkage data. When no individual linkage data
///   exists, use an empty SEQUENCE OF in the `SequenceOfJMaxGroup`.
///
/// * `groups` - Contains group linkage data. When no group linkage data exists, use
///   an empty SEQUENCE OF in the `SequenceOfGroupCrlEntry`.
///
/// * `groups_single_seed` - Contains group linkage data generated with a single seed.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedLinkageValueCrl {
    #[rasn(identifier = "iRev")]
    pub i_rev: IValue,
    #[rasn(identifier = "indexWithinI")]
    pub index_within_i: Uint8,
    pub individual: Option<SequenceOfJMaxGroup>,
    pub groups: Option<SequenceOfGroupCrlEntry>,
    #[rasn(extension_addition, identifier = "groupsSingleSeed")]
    pub groups_single_seed: Option<SequenceOfGroupSingleSeedCrlEntry>,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfJMaxGroup(pub SequenceOf<JMaxGroup>);

delegate!(SequenceOf<JMaxGroup>, SequenceOfJMaxGroup);

/// Group structure for linkage-based revocation information with a jMax parameter.
///
/// # Fields
///
/// * `jmax` - The value used in the algorithm specified in 5.1.3.4. Applies to all
///   linkage-based revocation information included within contents.
///
/// * `contents` - Contains individual linkage data.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct JMaxGroup {
    pub jmax: Uint8,
    pub contents: SequenceOfLAGroup,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfLAGroup(pub SequenceOf<LAGroup>);

delegate!(SequenceOf<LAGroup>, SequenceOfLAGroup);

/// Group structure containing linkage authority identifiers and associated data.
///
/// # Fields
///
/// * `la1_id` - The LinkageAuthorityIdentifier1 value used in the algorithm specified
///   in 5.1.3.4. Applies to all linkage-based revocation information in contents.
///
/// * `la2_id` - The LinkageAuthorityIdentifier2 value used in the algorithm specified
///   in 5.1.3.4. Applies to all linkage-based revocation information in contents.
///
/// * `contents` - Contains individual linkage data.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LAGroup {
    #[rasn(identifier = "la1Id")]
    pub la1_id: LaId,
    #[rasn(identifier = "la2Id")]
    pub la2_id: LaId,
    pub contents: SequenceOfIMaxGroup,
}

/// This type is used for clarity of definitions."]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfIMaxGroup(pub SequenceOf<IMaxGroup>);

delegate!(SequenceOf<IMaxGroup>, SequenceOfIMaxGroup);

/// Group structure containing revocation information with an iMax parameter.
///
/// # Fields
///
/// * `i_max` - Indicates when revocation information can be safely deleted. For entries
///   in contents, revocation information is no longer needed when `i_cert > i_max` as
///   the holder will have no more valid certificates. This value is not used directly
///   in linkage value calculations.
///
/// * `contents` - Contains individual linkage data for certificates revoked using two
///   seeds, following the algorithm in 5.1.3.4. The `seedEvolutionFunctionIdentifier`
///   and `linkageValueGenerationFunctionIdentifier` are obtained as specified in 7.3.3.
///
/// * `single_seed` - Contains individual linkage data for certificates revoked using a
///   single seed, following the algorithm in 5.1.3.4. The `seedEvolutionFunctionIdentifier`
///   and `linkageValueGenerationFunctionIdentifier` are obtained as specified in 7.3.3.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IMaxGroup {
    #[rasn(identifier = "iMax")]
    pub i_max: Uint16,
    pub contents: SequenceOfIndividualRevocation,
    #[rasn(extension_addition, identifier = "singleSeed")]
    pub single_seed: Option<SequenceOfLinkageSeed>,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfIndividualRevocation(pub SequenceOf<IndividualRevocation>);

delegate!(
    SequenceOf<IndividualRevocation>,
    SequenceOfIndividualRevocation
);

/// Structure containing linkage seed pairs for individual revocations.
///
/// # Fields
///
/// * `linkage_seed1` - The LinkageSeed1 value used in the algorithm specified in 5.1.3.4.
///
/// * `linkage_seed2` - The LinkageSeed2 value used in the algorithm specified in 5.1.3.4.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IndividualRevocation {
    #[rasn(identifier = "linkageSeed1")]
    pub linkage_seed1: LinkageSeed,
    #[rasn(identifier = "linkageSeed2")]
    pub linkage_seed2: LinkageSeed,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfGroupCrlEntry(pub SequenceOf<GroupCrlEntry>);

delegate!(SequenceOf<GroupCrlEntry>, SequenceOfGroupCrlEntry);

/// Parameters for linkage-based certificate revocation.
///
/// # Fields
///
/// * `i_max` - Indicates the threshold beyond which revocation information no longer
///   needs to be calculated for these certificates (when `i_cert > i_max`). At this point,
///   the holders are known to have no more valid certificates for that (`crl_craca`,
///   `crl_series`) pair.
///
/// * `la1_id` - The LinkageAuthorityIdentifier1 value used in the algorithm specified
///   in 5.1.3.4. Applies to all linkage-based revocation information included within
///   contents.
///
/// * `linkage_seed1` - The LinkageSeed1 value used in the algorithm specified in 5.1.3.4.
///
/// * `la2_id` - The LinkageAuthorityIdentifier2 value used in the algorithm specified
///   in 5.1.3.4. Applies to all linkage-based revocation information included within
///   contents.
///
/// * `linkage_seed2` - The LinkageSeed2 value used in the algorithm specified in 5.1.3.4.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GroupCrlEntry {
    #[rasn(identifier = "iMax")]
    pub i_max: Uint16,
    #[rasn(identifier = "la1Id")]
    pub la1_id: LaId,
    #[rasn(identifier = "linkageSeed1")]
    pub linkage_seed1: LinkageSeed,
    #[rasn(identifier = "la2Id")]
    pub la2_id: LaId,
    #[rasn(identifier = "linkageSeed2")]
    pub linkage_seed2: LinkageSeed,
}

/// CRL structure containing linkage-based revocation information with explicit algorithm identifiers.
///
/// # Fields
///
/// * `i_rev` - The value used in the algorithm specified in 5.1.3.4. Applies to all
///   linkage-based revocation information included within both individual and groups.
///
/// * `index_within_i` - Counter that starts at 0 for the first CRL issued for a specific
///   combination of `crl_craca`, `crl_series`, and `i_rev`. Increments by 1 for each new
///   full or delta CRL issued with the same identifiers but without changing `i_rev`.
///
/// * `seed_evolution` - Identifier for the seed evolution function, used as specified
///   in 5.1.3.4.
///
/// * `lv_generation` - Identifier for the linkage value generation function, used as
///   specified in 5.1.3.4.
///
/// * `individual` - Contains individual linkage data.
///
/// * `groups` - Contains group linkage data for linkage value generation with two seeds.
///
/// * `groups_single_seed` - Contains group linkage data for linkage value generation
///   with one seed.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedLinkageValueCrlWithAlgIdentifier {
    #[rasn(identifier = "iRev")]
    pub i_rev: IValue,
    #[rasn(identifier = "indexWithinI")]
    pub index_within_i: Uint8,
    #[rasn(identifier = "seedEvolution")]
    pub seed_evolution: SeedEvolutionFunctionIdentifier,
    #[rasn(identifier = "lvGeneration")]
    pub lv_generation: LvGenerationFunctionIdentifier,
    pub individual: Option<SequenceOfJMaxGroup>,
    pub groups: Option<SequenceOfGroupCrlEntry>,
    #[rasn(identifier = "groupsSingleSeed")]
    pub groups_single_seed: Option<SequenceOfGroupSingleSeedCrlEntry>,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfGroupSingleSeedCrlEntry(pub SequenceOf<GroupSingleSeedCrlEntry>);

delegate!(
    SequenceOf<GroupSingleSeedCrlEntry>,
    SequenceOfGroupSingleSeedCrlEntry
);

/// Contains the linkage seed for group revocation with a single seed.
/// The seed is used as specified in the algorithms in 5.1.3.4.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct GroupSingleSeedCrlEntry {
    #[rasn(identifier = "iMax")]
    pub i_max: Uint16,
    #[rasn(identifier = "laId")]
    pub la_id: LaId,
    #[rasn(identifier = "linkageSeed")]
    pub linkage_seed: LinkageSeed,
}

/// This structure contains an identifier for the algorithms specified in 5.1.3.4.
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ExpansionAlgorithmIdentifier {
    #[rasn(identifier = "sha256ForI-aesForJ")]
    Sha256forIAesForJ = 0,
    #[rasn(identifier = "sm3ForI-sm4ForJ")]
    Sm3forISm4forJ = 1,
}

/// This is the identifier for the seed evolution function. See 5.1.3 for details of use.
pub type SeedEvolutionFunctionIdentifier = ();

/// This is the identifier for the linkage value generation function. See 5.1.3 for details of use.
pub type LvGenerationFunctionIdentifier = ();
