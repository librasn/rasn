//! # Firmware Package Wrappers
//! Implementation of [RFC 4108] also known as "Using Cryptographic Message
//! Syntax (CMS) to Protect Firmware Packages". This module is used to protect
//! firmware packages with CMS, as well as use for receipts and error reports
//! for firmware package loading. The protected firmware package can be
//! associated with any particular hardware module.
//!
//! The firmware package contains object code for one or more programmable
//! components that make up the hardware module. The firmware package, which is
//! treated as an opaque binary object, is digitally signed. Optional encryption
//! and compression are also supported. When all three are used, the firmware
//! package is compressed, then encrypted, and then signed.
//!
//! As with all `rasn` core crate implementations, this module does not provide
//! the actual functionality for signing, encrypting, or compressing data;
//! instead provides a shared set of data types that can be used with other
//! crates to sign, encrypt, and compress your own firmware packages.
//!
//! [rfc 4108]: https://datatracker.ietf.org/doc/html/rfc4108
use rasn::prelude::*;

use super::EnvelopedData;

pub const FIRMWARE_PACKAGE: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_FIRMWARE_PACKAGE;
pub const FIRMWARE_PACKAGE_ID: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_FIRMWARE_PACKAGE_ID;
pub const DECRYPT_KEY_ID: &'static Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_DECRYPT_KEY_ID;
pub const CRYPTO_ALGORITHMS: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_CRYPTO_ALGORITHMS;
pub const COMPRESS_ALGORITHMS: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_COMPRESS_ALGORITHMS;
pub const COMMUNITY_IDENTIFIERS: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_COMMUNITY_IDENTIFIERS;
pub const FIRMWARE_PACKAGE_INFO: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_FIRMWARE_PACKAGE_INFO;
pub const WRAPPED_FIRMWARE_KEY: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_WRAPPED_FIRMWARE_KEY;
pub const FIRMWARE_LOAD_RECEIPT: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_FIRMWARE_LOAD_RECEIPT;
pub const FIRMWARE_LOAD_ERROR: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_FIRMWARE_LOAD_ERROR;
pub const HARDWARE_MODULE_NAME: &'static Oid =
    Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY_MECHANISMS_PKIX_ON_HARDWARE_MODULE_NAME;
pub const TARGET_HARDWARE_IDS: &'static Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_TARGET_HARDWARE_IDS;

pub type FirmwarePackageData = OctetString;
pub type TargetHardwareIdentifiers = SequenceOf<ObjectIdentifier>;
pub type DecryptKeyIdentifier = OctetString;
pub type ImplementedCryptoAlgorithms = SequenceOf<ObjectIdentifier>;
pub type ImplementedCompressAlgorithms = SequenceOf<ObjectIdentifier>;
pub type CommunityIdentifiers = SequenceOf<CommunityIdentifier>;
pub type WrappedFirmwareKey = EnvelopedData;
pub type FirmwareReceiptVersion = Integer;
pub type FirmwareErrorVersion = Integer;
pub type VendorLoadErrorCode = Integer;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FirmwarePackageIdentifier {
    pub name: PreferredOrLegacyPackageIdentifier,
    pub stale: Option<PreferredOrLegacyStalePackageIdentifier>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum PreferredOrLegacyPackageIdentifier {
    Preferred(PreferredPackageIdentifier),
    Legacy(OctetString),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PreferredPackageIdentifier {
    pub firmware_package_id: ObjectIdentifier,
    pub version_number: Integer,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum PreferredOrLegacyStalePackageIdentifier {
    PreferredStaleVersionNumber(Integer),
    LegacyStaleVersion(OctetString),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum CommunityIdentifier {
    CommunityOid(ObjectIdentifier),
    HardwareModuleList(HardwareModules),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HardwareModules {
    pub hardware_type: ObjectIdentifier,
    pub hardware_serial_entries: SequenceOf<HardwareSerialEntry>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
pub enum HardwareSerialEntry {
    All,
    Single(OctetString),
    Block { low: OctetString, high: OctetString },
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FirmwarePackageInfo {
    pub firmware_package_type: Option<Integer>,
    pub dependencies: Option<SequenceOf<PreferredOrLegacyPackageIdentifier>>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FirmwarePackageLoadReceipt {
    #[rasn(default = "default_firmware_receipt_version")]
    pub version: FirmwareReceiptVersion,
    pub hardware_type: ObjectIdentifier,
    pub hardware_serial_number: OctetString,
    pub firmware_package_name: PreferredOrLegacyPackageIdentifier,
    pub trust_anchor_key_id: Option<OctetString>,
    #[rasn(tag(1))]
    pub decrypt_key_id: Option<OctetString>,
}

fn default_firmware_receipt_version() -> FirmwareReceiptVersion {
    1u8.into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FirmwarePackageLoadError {
    #[rasn(default = "default_firmware_error_version")]
    pub version: FirmwareErrorVersion,
    pub hardware_type: ObjectIdentifier,
    pub hardware_serial_number: OctetString,
    pub error_code: FirmwarePackageLoadErrorCode,
    pub vendor_error_code: Option<VendorLoadErrorCode>,
    pub firmware_package_name: Option<PreferredOrLegacyPackageIdentifier>,
    #[rasn(tag(1))]
    pub config: Option<SequenceOf<CurrentFirmwareConfig>>,
}

fn default_firmware_error_version() -> FirmwareErrorVersion {
    1u8.into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CurrentFirmwareConfig {
    pub firmware_package_type: Option<Integer>,
    pub firmware_package_name: PreferredOrLegacyPackageIdentifier,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
pub enum FirmwarePackageLoadErrorCode {
    DecodeFailure = 1,
    BadContentInfo = 2,
    BadSignedData = 3,
    BadEncapContent = 4,
    BadCertificate = 5,
    BadSignerInfo = 6,
    BadSignedAttrs = 7,
    BadUnsignedAttrs = 8,
    MissingContent = 9,
    NoTrustAnchor = 10,
    NotAuthorized = 11,
    BadDigestAlgorithm = 12,
    BadSignatureAlgorithm = 13,
    UnsupportedKeySize = 14,
    SignatureFailure = 15,
    ContentTypeMismatch = 16,
    BadEncryptedData = 17,
    UnprotectedAttrsPresent = 18,
    BadEncryptContent = 19,
    BadEncryptAlgorithm = 20,
    MissingCiphertext = 21,
    NoDecryptKey = 22,
    DecryptFailure = 23,
    BadCompressAlgorithm = 24,
    MissingCompressedContent = 25,
    DecompressFailure = 26,
    WrongHardware = 27,
    StalePackage = 28,
    NotInCommunity = 29,
    UnsupportedPackageType = 30,
    MissingDependency = 31,
    WrongDependencyVersion = 32,
    InsufficientMemory = 33,
    BadFirmware = 34,
    UnsupportedParameters = 35,
    BreaksDependency = 36,
    OtherError = 99,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HardwareModuleName {
    pub hardware_type: ObjectIdentifier,
    pub hardware_serial_number: OctetString,
}
