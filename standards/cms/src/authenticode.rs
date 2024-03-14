//! Windows Authenticode Portable Executable Signature Format
//!
//! Authenticode is a digital signature format that is used to determine the origin and integrity of
//! software binaries. Authenticode is based on Public-Key Cryptography Standards (PKCS) #7 signed
//! data and X.509 certificates to bind an Authenticode-signed binary to the identity of a software
//! publisher.
//!
//! Reference: [Windows Authenticode Portable Executable Signature Format] (http://msdn.microsoft.com/en-US/windows/hardware/gg463183)
//! **NOTE**: the document differs from the actual implementation. This crate contains the structures used in actual signing.
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Neg};

use rasn::types::{Any, BitString, BmpString, Ia5String, ObjectIdentifier, OctetString, Oid};
use rasn::{AsnType, Decode, Encode};
use rasn_pkix::AlgorithmIdentifier;

pub const SPC_INDIRECT_DATA_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_INDIRECT_DATA_OBJID;
pub const SPC_PE_IMAGE_DATA_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_PE_IMAGE_DATA_OBJID;
pub const SPC_SP_OPUS_INFO_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_SP_OPUS_INFO_OBJID;
pub const SPC_STATEMENT_TYPE_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_STATEMENT_TYPE_OBJID;
pub const SPC_CAB_DATA_OBJID: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_CAB_DATA_OBJID;
pub const SPC_SIPINFO_OBJID: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_SIPINFO_OBJID;
pub const SPC_PE_IMAGE_PAGE_HASHES_V1: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_PE_IMAGE_PAGE_HASHES_V1;
pub const SPC_PE_IMAGE_PAGE_HASHES_V2: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_PE_IMAGE_PAGE_HASHES_V2;
pub const SPC_NESTED_SIGNATURE_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_NESTED_SIGNATURE_OBJID;
pub const SPC_TIME_STAMP_REQUEST_OBJID: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_TIME_STAMP_REQUEST_OBJID;
pub const SPC_RFC3161_OBJID: &Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_MICROSOFT_SPC_RFC3161_OBJID;

pub const SPC_CLASS_UUID: OctetString = OctetString::from_static(&[
    0xa6, 0xb5, 0x86, 0xd5, 0xb4, 0xa1, 0x24, 0x66, 0xae, 0x05, 0xa2, 0x17, 0xda, 0x8e, 0x60, 0xd6,
]);

pub type SpcUuid = OctetString;

/// An Authenticode signature's ContentInfo structure contains several structures that in turn contain
/// the file's hash value, page hash values (if present), the file description, and various optional or legacy
/// ASN.1 fields. The root structure is SpcIndirectDataContent.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpcIndirectDataContent {
    pub data: SpcAttributeTypeAndOptionalValue,
    pub message_digest: DigestInfo,
}

/// The SpcAttributeTypeAndOptionalValue structure has two fields, which are set for an
/// Authenticode-signed PE file.
/// The attribute_type is set to SPC_PE_IMAGE_DATAOBJ OID (1.3.6.1.4.1.311.2.1.15)
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpcAttributeTypeAndOptionalValue {
    pub attribute_type: ObjectIdentifier,
    pub value: Option<Any>,
}

/// The DigestInfo structure defines the digest algorithm and data
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigestInfo {
    pub digest_algorithm: AlgorithmIdentifier,
    pub digest: OctetString,
}

/// The SpcPeImageData structure specifies which portions of the Windows PE file are hashed.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpcPeImageData {
    pub flags: SpcPeImageFlags,
    #[rasn(tag(explicit(0)))]
    pub file: Option<SpcLink>,
}

/// Flags specify which portions of the Windows PE file are hashed.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct SpcPeImageFlags(pub BitString);

impl SpcPeImageFlags {
    pub fn include_resources() -> Self {
        Self(BitString::from_element(0))
    }

    pub fn include_debug_info() -> Self {
        Self(BitString::from_element(1))
    }

    pub fn include_import_address_table() -> Self {
        Self(BitString::from_element(2))
    }
}

impl BitOr for SpcPeImageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for SpcPeImageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for SpcPeImageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for SpcPeImageFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Neg for SpcPeImageFlags {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(!self.0)
    }
}

/// SPCLink originally contained information that describes the software publisher
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum SpcLink {
    #[rasn(tag(0))]
    Url(Ia5String),
    #[rasn(tag(1))]
    Moniker(SpcSerializedObject),
    #[rasn(tag(explicit(2)))]
    File(SpcString),
}

/// SpcString is either Unicode or ASCII string
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum SpcString {
    #[rasn(tag(0))]
    Unicode(BmpString),
    #[rasn(tag(1))]
    Ascii(Ia5String),
}

/// SpcSerializedObject contains a binary structure with page hashes
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpcSerializedObject {
    pub class_id: SpcUuid,
    pub serialized_data: OctetString,
}

/// This structure is present in SignerInfo authenticated attributes.
/// It is identified by SPC_SP_OPUS_INFO_OBJID (1.3.6.1.4.1.311.2.1.12)
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpcSpOpusInfo {
    #[rasn(tag(explicit(0)))]
    pub program_name: Option<SpcString>,
    #[rasn(tag(explicit(1)))]
    pub more_info: Option<SpcLink>,
}
