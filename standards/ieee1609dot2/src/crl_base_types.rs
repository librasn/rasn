extern crate alloc;
use crate::ieee1609_dot2_base_types::{
    CrlSeries, Duration, GeographicRegion, HashedId10, HashedId8, IValue, LaId, LinkageSeed,
    Opaque, Psid, SequenceOfLinkageSeed, Signature, Time32, Uint16, Uint3, Uint32, Uint8,
    ValidityPeriod,
};
use rasn::prelude::*;

#[doc = "*"]
#[doc = " * @brief The fields in this structure have the following meaning:"]
#[doc = " *"]
#[doc = " * @param version: is the version number of the CRL. For this version of this"]
#[doc = " * standard it is 1."]
#[doc = " *"]
#[doc = " * @param crlSeries: represents the CRL series to which this CRL belongs. This"]
#[doc = " * is used to determine whether the revocation information in a CRL is relevant"]
#[doc = " * to a particular certificate as specified in 5.1.3.2."]
#[doc = " *"]
#[doc = " * @param crlCraca: contains the low-order eight octets of the hash of the"]
#[doc = " * certificate of the Certificate Revocation Authorization CA (CRACA) that"]
#[doc = " * ultimately authorized the issuance of this CRL. This is used to determine"]
#[doc = " * whether the revocation information in a CRL is relevant to a particular"]
#[doc = " * certificate as specified in 5.1.3.2. In a valid signed CRL as specified in"]
#[doc = " * 7.4 the crlCraca is consistent with the associatedCraca field in the"]
#[doc = " * Service Specific Permissions as defined in 7.4.3.3. The HashedId8 is"]
#[doc = " * calculated with the whole-certificate hash algorithm, determined as"]
#[doc = " * described in 6.4.3, applied to the COER-encoded certificate, canonicalized "]
#[doc = " * as defined in the definition of Certificate."]
#[doc = " *"]
#[doc = " * @param issueDate: specifies the time when the CRL was issued."]
#[doc = " *"]
#[doc = " * @param nextCrl: contains the time when the next CRL with the same crlSeries"]
#[doc = " * and cracaId is expected to be issued. The CRL is invalid unless nextCrl is"]
#[doc = " * strictly after issueDate. This field is used to set the expected update time"]
#[doc = " * for revocation information associated with the (crlCraca, crlSeries) pair as"]
#[doc = " * specified in 5.1.3.6."]
#[doc = " *"]
#[doc = " * @param priorityInfo: contains information that assists devices with limited"]
#[doc = " * storage space in determining which revocation information to retain and"]
#[doc = " * which to discard."]
#[doc = " *"]
#[doc = " * @param\ttypeSpecific: contains the CRL body."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
impl CrlContents {
    pub fn new(
        version: Uint8,
        crl_series: CrlSeries,
        crl_craca: HashedId8,
        issue_date: Time32,
        next_crl: Time32,
        priority_info: CrlPriorityInfo,
        type_specific: TypeSpecificCrlContents,
    ) -> Self {
        Self {
            version,
            crl_series,
            crl_craca,
            issue_date,
            next_crl,
            priority_info,
            type_specific,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief This data structure contains information that assists devices with"]
#[doc = " * limited storage space in determining which revocation information to retain"]
#[doc = " * and which to discard."]
#[doc = " *"]
#[doc = " * @param priority: indicates the priority of the revocation information"]
#[doc = " * relative to other CRLs issued for certificates with the same cracaId and"]
#[doc = " * crlSeries values. A higher value for this field indicates higher importance"]
#[doc = " * of this revocation information."]
#[doc = " *"]
#[doc = " * @note This mechanism is for future use; details are not specified in this"]
#[doc = " * version of the standard."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CrlPriorityInfo {
    pub priority: Option<Uint8>,
}
impl CrlPriorityInfo {
    pub fn new(priority: Option<Uint8>) -> Self {
        Self { priority }
    }
}
#[doc = "*"]
#[doc = " * @brief This structure contains an identifier for the algorithms specified "]
#[doc = " * in 5.1.3.4."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ExpansionAlgorithmIdentifier {
    #[rasn(identifier = "sha256ForI-aesForJ")]
    sha256ForI_aesForJ = 0,
    #[rasn(identifier = "sm3ForI-sm4ForJ")]
    sm3ForI_sm4ForJ = 1,
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param iMax: indicates that for these certificates, revocation information "]
#[doc = " * need no longer be calculated once iCert > iMax as the holders are known "]
#[doc = " * to have no more valid certs for that (crlCraca, crlSeries) at that point."]
#[doc = " *"]
#[doc = " * @param la1Id: is the value LinkageAuthorityIdentifier1 used in the "]
#[doc = " * algorithm given in 5.1.3.4. This value applies to all linkage-based "]
#[doc = " * revocation information included within contents."]
#[doc = " *"]
#[doc = " * @param linkageSeed1: is the value LinkageSeed1 used in the algorithm given "]
#[doc = " * in 5.1.3.4."]
#[doc = " *"]
#[doc = " * @param la2Id: is the value LinkageAuthorityIdentifier2 used in the "]
#[doc = " * algorithm given in 5.1.3.4. This value applies to all linkage-based "]
#[doc = " * revocation information included within contents."]
#[doc = " *"]
#[doc = " * @param linkageSeed2: is the value LinkageSeed2 used in the algorithm given "]
#[doc = " * in 5.1.3.4."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
impl GroupCrlEntry {
    pub fn new(
        i_max: Uint16,
        la1_id: LaId,
        linkage_seed1: LinkageSeed,
        la2_id: LaId,
        linkage_seed2: LinkageSeed,
    ) -> Self {
        Self {
            i_max,
            la1_id,
            linkage_seed1,
            la2_id,
            linkage_seed2,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief This structure contains the linkage seed for group revocation with "]
#[doc = " * a single seed. The seed is used as specified in the algorithms in 5.1.3.4."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct GroupSingleSeedCrlEntry {
    #[rasn(identifier = "iMax")]
    pub i_max: Uint16,
    #[rasn(identifier = "laId")]
    pub la_id: LaId,
    #[rasn(identifier = "linkageSeed")]
    pub linkage_seed: LinkageSeed,
}
impl GroupSingleSeedCrlEntry {
    pub fn new(i_max: Uint16, la_id: LaId, linkage_seed: LinkageSeed) -> Self {
        Self {
            i_max,
            la_id,
            linkage_seed,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param\tid: is the HashedId10 identifying the revoked certificate. The "]
#[doc = " * HashedId10 is calculated with the whole-certificate hash algorithm, "]
#[doc = " * determined as described in 6.4.3, applied to the COER-encoded certificate,"]
#[doc = " * canonicalized as defined in the definition of Certificate."]
#[doc = " *"]
#[doc = " * @param expiry: is the value computed from the validity period's start and"]
#[doc = " * duration values in that certificate."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct HashBasedRevocationInfo {
    pub id: HashedId10,
    pub expiry: Time32,
}
impl HashBasedRevocationInfo {
    pub fn new(id: HashedId10, expiry: Time32) -> Self {
        Self { id, expiry }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param iMax indicates that for the entries in contents, revocation "]
#[doc = " * information need no longer be calculated once iCert > iMax as the holder "]
#[doc = " * is known to have no more valid certs at that point. iMax is not directly "]
#[doc = " * used in the calculation of the linkage values, it is used to determine "]
#[doc = " * when revocation information can safely be deleted."]
#[doc = " *"]
#[doc = " * @param contents contains individual linkage data for certificates that are "]
#[doc = " * revoked using two seeds, per the algorithm given in per the mechanisms "]
#[doc = " * given in 5.1.3.4 and with seedEvolutionFunctionIdentifier and "]
#[doc = " * linkageValueGenerationFunctionIdentifier obtained as specified in 7.3.3."]
#[doc = " *"]
#[doc = " * @param singleSeed contains individual linkage data for certificates that "]
#[doc = " * are revoked using a single seed, per the algorithm given in per the "]
#[doc = " * mechanisms given in 5.1.3.4 and with seedEvolutionFunctionIdentifier and "]
#[doc = " * linkageValueGenerationFunctionIdentifier obtained as specified in 7.3.3."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IMaxGroup {
    #[rasn(identifier = "iMax")]
    pub i_max: Uint16,
    pub contents: SequenceOfIndividualRevocation,
    #[rasn(extension_addition, identifier = "singleSeed")]
    pub single_seed: Option<SequenceOfLinkageSeed>,
}
impl IMaxGroup {
    pub fn new(
        i_max: Uint16,
        contents: SequenceOfIndividualRevocation,
        single_seed: Option<SequenceOfLinkageSeed>,
    ) -> Self {
        Self {
            i_max,
            contents,
            single_seed,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param linkageSeed1 is the value LinkageSeed1 used in the algorithm given "]
#[doc = " * in 5.1.3.4."]
#[doc = " *"]
#[doc = " * @param linkageSeed2 is the value LinkageSeed2 used in the algorithm given "]
#[doc = " * in 5.1.3.4."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IndividualRevocation {
    #[rasn(identifier = "linkageSeed1")]
    pub linkage_seed1: LinkageSeed,
    #[rasn(identifier = "linkageSeed2")]
    pub linkage_seed2: LinkageSeed,
}
impl IndividualRevocation {
    pub fn new(linkage_seed1: LinkageSeed, linkage_seed2: LinkageSeed) -> Self {
        Self {
            linkage_seed1,
            linkage_seed2,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param\tjMax: is the value jMax used in the algorithm given in 5.1.3.4. This"]
#[doc = " * value applies to all linkage-based revocation information included within"]
#[doc = " * contents."]
#[doc = " *"]
#[doc = " * @param contents: contains individual linkage data."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct JMaxGroup {
    pub jmax: Uint8,
    pub contents: SequenceOfLAGroup,
}
impl JMaxGroup {
    pub fn new(jmax: Uint8, contents: SequenceOfLAGroup) -> Self {
        Self { jmax, contents }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param la1Id: is the value LinkageAuthorityIdentifier1 used in the"]
#[doc = " * algorithm given in 5.1.3.4. This value applies to all linkage-based"]
#[doc = " * revocation information included within contents."]
#[doc = " *"]
#[doc = " * @param la2Id: is the value LinkageAuthorityIdentifier2 used in the"]
#[doc = " * algorithm given in 5.1.3.4. This value applies to all linkage-based"]
#[doc = " * revocation information included within contents."]
#[doc = " *"]
#[doc = " * @param contents: contains individual linkage data."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LAGroup {
    #[rasn(identifier = "la1Id")]
    pub la1_id: LaId,
    #[rasn(identifier = "la2Id")]
    pub la2_id: LaId,
    pub contents: SequenceOfIMaxGroup,
}
impl LAGroup {
    pub fn new(la1_id: LaId, la2_id: LaId, contents: SequenceOfIMaxGroup) -> Self {
        Self {
            la1_id,
            la2_id,
            contents,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief This is the identifier for the linkage value generation function. "]
#[doc = " * See 5.1.3 for details of use."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct LvGenerationFunctionIdentifier(());
#[doc = "*"]
#[doc = " * @brief This is the identifier for the seed evolution function. See 5.1.3 "]
#[doc = " * for details of use."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SeedEvolutionFunctionIdentifier(());
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfGroupCrlEntry(pub SequenceOf<GroupCrlEntry>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfGroupSingleSeedCrlEntry(pub SequenceOf<GroupSingleSeedCrlEntry>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfHashBasedRevocationInfo(pub SequenceOf<HashBasedRevocationInfo>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfIMaxGroup(pub SequenceOf<IMaxGroup>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfIndividualRevocation(pub SequenceOf<IndividualRevocation>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfJMaxGroup(pub SequenceOf<JMaxGroup>);
#[doc = "*"]
#[doc = " * @brief This type is used for clarity of definitions."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct SequenceOfLAGroup(pub SequenceOf<LAGroup>);
#[doc = "*"]
#[doc = " * @brief This data structure represents information about a revoked"]
#[doc = " * certificate."]
#[doc = " *"]
#[doc = " * @param crlSerial: is a counter that increments by 1 every time a new full"]
#[doc = " * or delta CRL is issued for the indicated crlCraca and crlSeries values."]
#[doc = " *"]
#[doc = " * @param entries: contains the individual revocation information items."]
#[doc = " *"]
#[doc = " * @note To indicate that a hash-based CRL contains no individual revocation "]
#[doc = " * information items, the recommended approach is for the SEQUENCE OF in the "]
#[doc = " * SequenceOfHashBasedRevocationInfo in this field to indicate zero entries."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ToBeSignedHashIdCrl {
    #[rasn(identifier = "crlSerial")]
    pub crl_serial: Uint32,
    pub entries: SequenceOfHashBasedRevocationInfo,
}
impl ToBeSignedHashIdCrl {
    pub fn new(crl_serial: Uint32, entries: SequenceOfHashBasedRevocationInfo) -> Self {
        Self {
            crl_serial,
            entries,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " *"]
#[doc = " * @param\tiRev: is the value iRev used in the algorithm given in 5.1.3.4. This"]
#[doc = " * value applies to all linkage-based revocation information included within"]
#[doc = " * either indvidual or groups."]
#[doc = " *"]
#[doc = " * @param\tindexWithinI: is a counter that is set to 0 for the first CRL issued"]
#[doc = " * for the indicated combination of crlCraca, crlSeries, and iRev, and"]
#[doc = " * increments by 1 every time a new full or delta CRL is issued for the"]
#[doc = " * indicated crlCraca and crlSeries values without changing iRev."]
#[doc = " *"]
#[doc = " * @param individual: contains individual linkage data."]
#[doc = " *"]
#[doc = " * @note To indicate that a linkage ID-based CRL contains no individual"]
#[doc = " * linkage data, the recommended approach is for the SEQUENCE OF in the"]
#[doc = " * SequenceOfJMaxGroup in this field to indicate zero entries."]
#[doc = " *"]
#[doc = " * @param groups: contains group linkage data."]
#[doc = " *"]
#[doc = " * @note To indicate that a linkage ID-based CRL contains no group linkage"]
#[doc = " * data, the recommended approach is for the SEQUENCE OF in the"]
#[doc = " * SequenceOfGroupCrlEntry in this field to indicate zero entries."]
#[doc = " *"]
#[doc = " * @param groupsSingleSeed: contains group linkage data generated with a single "]
#[doc = " * seed."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
impl ToBeSignedLinkageValueCrl {
    pub fn new(
        i_rev: IValue,
        index_within_i: Uint8,
        individual: Option<SequenceOfJMaxGroup>,
        groups: Option<SequenceOfGroupCrlEntry>,
        groups_single_seed: Option<SequenceOfGroupSingleSeedCrlEntry>,
    ) -> Self {
        Self {
            i_rev,
            index_within_i,
            individual,
            groups,
            groups_single_seed,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief In this structure:"]
#[doc = " * "]
#[doc = " * @param iRev is the value iRev used in the algorithm given in 5.1.3.4. This "]
#[doc = " * value applies to all linkage-based revocation information included within "]
#[doc = " * either indvidual or groups."]
#[doc = " * "]
#[doc = " * @param indexWithinI is a counter that is set to 0 for the first CRL issued "]
#[doc = " * for the indicated combination of crlCraca, crlSeries, and iRev, and increments by 1 every time a new full or delta CRL is issued for the indicated crlCraca and crlSeries values without changing iRev."]
#[doc = " * "]
#[doc = " * @param seedEvolution contains an identifier for the seed evolution "]
#[doc = " * function, used as specified in  5.1.3.4."]
#[doc = " * "]
#[doc = " * @param lvGeneration contains an identifier for the linkage value "]
#[doc = " * generation function, used as specified in  5.1.3.4."]
#[doc = " * "]
#[doc = " * @param individual contains individual linkage data."]
#[doc = " * "]
#[doc = " * @param groups contains group linkage data for linkage value generation "]
#[doc = " * with two seeds."]
#[doc = " * "]
#[doc = " * @param groupsSingleSeed contains group linkage data for linkage value "]
#[doc = " * generation with one seed."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
impl ToBeSignedLinkageValueCrlWithAlgIdentifier {
    pub fn new(
        i_rev: IValue,
        index_within_i: Uint8,
        seed_evolution: SeedEvolutionFunctionIdentifier,
        lv_generation: LvGenerationFunctionIdentifier,
        individual: Option<SequenceOfJMaxGroup>,
        groups: Option<SequenceOfGroupCrlEntry>,
        groups_single_seed: Option<SequenceOfGroupSingleSeedCrlEntry>,
    ) -> Self {
        Self {
            i_rev,
            index_within_i,
            seed_evolution,
            lv_generation,
            individual,
            groups,
            groups_single_seed,
        }
    }
}
#[doc = "*"]
#[doc = " * @brief This structure contains type-specific CRL contents."]
#[doc = " *"]
#[doc = " * @param fullHashCrl: contains a full hash-based CRL, i.e., a listing of the"]
#[doc = " * hashes of all certificates that:"]
#[doc = " *  - contain the indicated cracaId and crlSeries values, and"]
#[doc = " *  - are revoked by hash, and"]
#[doc = " *  - have been revoked, and"]
#[doc = " *  - have not expired."]
#[doc = " *"]
#[doc = " * @param deltaHashCrl: contains a delta hash-based CRL, i.e., a listing of"]
#[doc = " * the hashes of all certificates that:"]
#[doc = " *  - contain the indicated cracaId and crlSeries values, and"]
#[doc = " *  - are revoked by hash, and"]
#[doc = " *  - have been revoked since the previous CRL that contained the indicated"]
#[doc = " * cracaId and crlSeries values."]
#[doc = " *"]
#[doc = " * @param fullLinkedCrl and fullLinkedCrlWithAlg: contain a full linkage"]
#[doc = " * ID-based CRL, i.e., a listing of the individual and/or group linkage data"]
#[doc = " * for all certificates that:"]
#[doc = " *  - contain the indicated cracaId and crlSeries values, and"]
#[doc = " *  - are revoked by linkage value, and"]
#[doc = " *  - have been revoked, and"]
#[doc = " *  - have not expired."]
#[doc = " * The difference between fullLinkedCrl and fullLinkedCrlWithAlg is in how"]
#[doc = " * the cryptographic algorithms to be used in the seed evolution function and"]
#[doc = " * linkage value generation function of 5.1.3.4 are communicated to the"]
#[doc = " * receiver of the CRL. See below in this subclause for details."]
#[doc = " *"]
#[doc = " * @param deltaLinkedCrl and deltaLinkedCrlWithAlg: contain a delta linkage"]
#[doc = " * ID-based CRL, i.e., a listing of the individual and/or group linkage data"]
#[doc = " * for all certificates that:"]
#[doc = " *  - contain the specified cracaId and crlSeries values, and"]
#[doc = " *  -\tare revoked by linkage data, and"]
#[doc = " *  -\thave been revoked since the previous CRL that contained the indicated"]
#[doc = " * cracaId and crlSeries values."]
#[doc = " * The difference between deltaLinkedCrl and deltaLinkedCrlWithAlg is in how"]
#[doc = " * the cryptographic algorithms to be used in the seed evolution function"]
#[doc = " * and linkage value generation function of 5.1.3.4 are communicated to the"]
#[doc = " * receiver of the CRL. See below in this subclause for details."]
#[doc = " *"]
#[doc = " * @note It is the intent of this standard that once a certificate is revoked,"]
#[doc = " * it remains revoked for the rest of its lifetime. CRL signers are expected "]
#[doc = " * to include a revoked certificate on all CRLs issued between the "]
#[doc = " * certificate's revocation and its expiry."]
#[doc = " *"]
#[doc = " * @note Seed evolution function and linkage value generation function"]
#[doc = " * identification. In order to derive linkage values per the mechanisms given"]
#[doc = " * in 5.1.3.4, a receiver needs to know the seed evolution function and the"]
#[doc = " * linkage value generation function."]
#[doc = " *"]
#[doc = " * If the contents of this structure is a"]
#[doc = " * ToBeSignedLinkageValueCrlWithAlgIdentifier, then the seed evolution function"]
#[doc = " * and linkage value generation function are given explicitly as specified in"]
#[doc = " * the specification of ToBeSignedLinkageValueCrlWithAlgIdentifier."]
#[doc = " *"]
#[doc = " * If the contents of this structure is a ToBeSignedLinkageValueCrl, then the"]
#[doc = " * seed evolution function and linkage value generation function are obtained"]
#[doc = " * based on the crlCraca field in the CrlContents:"]
#[doc = " *  - If crlCraca was obtained with SHA-256 or SHA-384, then"]
#[doc = " * seedEvolutionFunctionIdentifier is seedEvoFn1-sha256 and"]
#[doc = " * linkageValueGenerationFunctionIdentifier is lvGenFn1-aes128."]
#[doc = " *  - If crlCraca was obtained with SM3, then seedEvolutionFunctionIdentifier"]
#[doc = " * is seedEvoFn1-sm3 and linkageValueGenerationFunctionIdentifier is"]
#[doc = " * lvGenFn1-sm4."]
#[doc = " "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum TypeSpecificCrlContents {
    fullHashCrl(ToBeSignedHashIdCrl),
    deltaHashCrl(ToBeSignedHashIdCrl),
    fullLinkedCrl(ToBeSignedLinkageValueCrl),
    deltaLinkedCrl(ToBeSignedLinkageValueCrl),
    #[rasn(extension_addition)]
    fullLinkedCrlWithAlg(ToBeSignedLinkageValueCrlWithAlgIdentifier),
    #[rasn(extension_addition)]
    deltaLinkedCrlWithAlg(ToBeSignedLinkageValueCrlWithAlgIdentifier),
}
