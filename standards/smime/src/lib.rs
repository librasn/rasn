#![doc = include_str!("../README.md")]
#![no_std]

pub mod ess;
pub mod skd;

use rasn::prelude::*;

#[doc(inline)]
pub use rasn_cms::{IssuerAndSerialNumber, RecipientKeyIdentifier, SubjectKeyIdentifier};

/// S/MIME Capabilities provides a method of broadcasting the
/// symmetric capabilities understood.  Algorithms SHOULD be ordered
/// by preference and grouped by type.
pub const CAPABILITIES: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_CAPABILITIES;

/// Encryption Key Preference provides a method of broadcasting the preferred
/// encryption certificate.
pub const ENCRYPTION_KEY_PREFERENCE: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_ENCRYPTION_KEY_PREFERENCE;

/// Indicates the ability to receive messages with binary encoding inside the
/// CMS wrapper. The attribute's value field is `None`.
pub const PREFER_BINARY_INSIDE: &Oid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CAPABILITY_PREFER_BINARY_INSIDE;

/// RC2 Key Length (number of bits)
pub type CapabilitiesParametersForRc2Cbc = Integer;
pub type Capabilities = SequenceOf<Capability>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capability {
    pub capability_id: ObjectIdentifier,
    pub parameters: Option<Any>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum EncryptionKeyPreference {
    #[rasn(tag(0))]
    IssuerAndSerialNumber(IssuerAndSerialNumber),
    #[rasn(tag(1))]
    ReceipentKeyId(RecipientKeyIdentifier),
    #[rasn(tag(2))]
    SubjectAltKeyIdentifier(SubjectKeyIdentifier),
}
