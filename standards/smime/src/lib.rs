//! # Secure/Multipurpose Internet Mail Extensions
//! An implementation of [RFC 8551] also known as Secure/Multipurpose Internet
//! Mail Extensions (S/MIME). S/MIME provides a consistent way to send and
//! receive secure MIME data.  Based on the popular Internet MIME standard,
//! S/MIME provides the following cryptographic security services for electronic
//! messaging applications: authentication, message integrity, and
//! non-repudiation of origin (using digital signatures), and data
//! confidentiality (using encryption).
//!
//! Like other `rasn` core crates. This crate does not provide the ability to
//! do authentication or encryption on its own, but instead provides shared data
//! types for creating your own S/MIME clients and servers.
//!
//! [rfc 8551]: https://datatracker.ietf.org/doc/html/rfc8551
#![no_std]

pub mod ess;
pub mod skd;

use rasn::prelude::*;

#[doc(inline)]
pub use rasn_cms::{IssuerAndSerialNumber, RecipientKeyIdentifier, SubjectKeyIdentifier};

/// S/MIME Capabilities provides a method of broadcasting the
/// symmetric capabilities understood.  Algorithms SHOULD be ordered
/// by preference and grouped by type.
pub const CAPABILITIES: ConstOid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_CAPABILITIES;

/// Encryption Key Preference provides a method of broadcasting the preferred
/// encryption certificate.
pub const ENCRYPTION_KEY_PREFERENCE: ConstOid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_ENCRYPTION_KEY_PREFERENCE;

/// Indicates the ability to receive messages with binary encoding inside the
/// CMS wrapper. The attribute's value field is `None`.
pub const PREFER_BINARY_INSIDE: ConstOid =
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
