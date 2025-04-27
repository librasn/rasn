#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod algorithms;
pub mod attribute_certificate;
pub mod est;

use rasn::prelude::*;

pub type InvalidityDate = GeneralizedTime;
pub type CertificateIssuer = GeneralNames;
pub type CrlNumber = Integer;
pub type BaseCrlNumber = CrlNumber;
pub type SubjectInfoAccessSyntax = SequenceOf<AccessDescription>;
pub type AuthorityInfoAccessSyntax = SequenceOf<AccessDescription>;
pub type FreshestCrl = CrlDistributionPoints;
pub type InhibitAnyPolicy = CrlDistributionPoints;
pub type KeyPurposeId = ObjectIdentifier;
pub type ExtKeyUsageSyntax = SequenceOf<KeyPurposeId>;
pub type ReasonFlags = BitString;
pub type SkipCerts = Integer;
pub type BaseDistance = Integer;
pub type CrlDistributionPoints = SequenceOf<DistributionPoint>;
pub type GeneralSubtrees = SequenceOf<GeneralSubtree>;
pub type SubjectDirectoryAttributes = SequenceOf<Attribute>;
pub type GeneralNames = SequenceOf<GeneralName>;
pub type SubjectAltName = GeneralNames;
pub type PolicyMappings = SequenceOf<PolicyMapping>;
pub type CpsUri = Ia5String;
pub type CertPolicyId = ObjectIdentifier;
pub type CertificatePolicies = SequenceOf<PolicyInformation>;
pub type KeyUsage = BitString;
pub type AttributeType = ObjectIdentifier;
pub type AttributeValue = Any;
pub type RdnSequence = SequenceOf<RelativeDistinguishedName>;
pub type X520DnQualifier = PrintableString;
pub type DomainComponent = Ia5String;
pub type EmailAddress = Ia5String;
pub type CertificateSerialNumber = Integer;
pub type UniqueIdentifier = BitString;
pub type NetworkAddress = X121Address;
pub type X121Address = NumericString;
pub type TerminalIdentifier = PrintableString;
pub type OrganisationName = PrintableString;
pub type NumericUserIdentifier = NumericString;
pub type TerminalType = u8;
pub type KeyIdentifier = OctetString;
pub type SubjectKeyIdentifier = KeyIdentifier;
pub type PolicyQualifierId = ObjectIdentifier;
pub type TrustAnchorTitle = Utf8String;
pub type TrustAnchorList = SequenceOf<TrustAnchorChoice>;
pub type CertPolicyFlags = BitString;

macro_rules! derefable {
    ($ty:ident, $inner:ty) => {
        impl From<$inner> for $ty {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl core::ops::Deref for $ty {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// An X.509 certificate
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Certificate {
    /// Certificate information.
    pub tbs_certificate: TbsCertificate,
    /// contains the identifier for the cryptographic algorithm used by the CA
    /// to sign this certificate.
    pub signature_algorithm: AlgorithmIdentifier,
    /// Contains a digital signature computed upon the ASN.1 DER encoded
    /// `tbs_certificate`.  The ASN.1 DER encoded tbsCertificate is used as the
    /// input to the signature function. The details of this process are
    /// specified for each of the algorithms listed in [RFC 3279], [RFC 4055],
    /// and [RFC 4491].
    ///
    pub signature_value: BitString,
}

/// Information associated with the subject of the certificate and the CA that
/// issued it.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TbsCertificate {
    /// The version of the encoded certificate.
    #[rasn(tag(explicit(0)), default)]
    pub version: Version,
    /// The serial number MUST be a positive integer assigned by the CA to each
    /// certificate.  It MUST be unique for each certificate issued by a given
    /// CA (i.e., the issuer name and serial number identify a unique
    /// certificate).  CAs MUST force the serialNumber to be a
    /// non-negative integer.
    ///
    /// Given the uniqueness requirements above, serial numbers can be expected
    /// to contain long integers.  Certificate users MUST be able to handle
    /// serialNumber values up to 20 octets.  Conforming CAs MUST NOT use
    /// serialNumber values longer than 20 octets.
    ///
    /// Note: Non-conforming CAs may issue certificates with serial numbers that
    /// are negative or zero.  Certificate users SHOULD be prepared to
    /// gracefully handle such certificates.
    pub serial_number: CertificateSerialNumber,
    /// The algorithm identifier for the algorithm used by the CA to sign
    /// the certificate.
    ///
    /// This field MUST contain the same algorithm identifier as the
    /// [`Certificate.signature_algorithm`].  The contents of the optional
    /// parameters field will vary according to the algorithm identified.
    /// [RFC 3279], [RFC 4055], and [RFC 4491] list supported signature algorithms,
    /// but other signature algorithms MAY also be supported.
    pub signature: AlgorithmIdentifier,
    /// The entity that has signed and issued the certificate. The issuer field
    /// MUST contain a non-empty distinguished name (DN).
    pub issuer: Name,
    /// The time interval during which the CA warrants that it will maintain
    /// information about the status of the certificate.
    pub validity: Validity,
    /// The entity associated with the public key stored in the subject public
    /// key field.
    pub subject: Name,
    /// The public key and identifies the algorithm with which the key is used
    /// (e.g., RSA, DSA, or Diffie-Hellman).
    pub subject_public_key_info: SubjectPublicKeyInfo,
    #[rasn(tag(1))]
    pub issuer_unique_id: Option<UniqueIdentifier>,
    #[rasn(tag(2))]
    pub subject_unique_id: Option<UniqueIdentifier>,
    /// Extensions to the certificate.
    #[rasn(tag(explicit(3)))]
    pub extensions: Option<Extensions>,
}

/// The version of a encoded certificate.
///
/// When extensions are used, as expected in this profile, version MUST be 3
/// (value is 2).  If no extensions are present, but a UniqueIdentifier is
/// present, the version SHOULD be 2 (value is 1); however, the version MAY
/// be 3.  If only basic fields are present, the version SHOULD be 1 (the
/// value is omitted from the certificate as the default value); however,
/// the version MAY be 2 or 3.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct Version(u64);

impl Version {
    pub const V1: Self = Self(0);
    pub const V2: Self = Self(1);
    pub const V3: Self = Self(2);

    /// Returns the raw value of the version. Note that the version is
    /// zero-indexed (v1 is 0, v2 is 1, etc).
    pub fn raw_value(self) -> u64 {
        self.0
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}", self.0.saturating_add(1))
    }
}

/// Trust anchors are widely used to verify digital signatures and
/// validate certification paths [RFC 5280][X.509].  
///
/// They are required when validating certification paths. Though widely used, there is no
/// standard format for representing trust anchor information.  The RFC-5914
/// document describes the TrustAnchorInfo structure.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TrustAnchorInfo {
    /// version identifies the version of TrustAnchorInfo.  Defaults to 1.
    #[rasn(tag(explicit(1)), default)]
    pub version: TrustAnchorInfoVersion,
    /// pubKey identifies the public key and algorithm associated with the
    /// trust anchor using the SubjectPublicKeyInfo structure [RFC 5280].  The
    /// SubjectPublicKeyInfo structure contains the algorithm identifier
    /// followed by the public key itself.
    pub pub_key: SubjectPublicKeyInfo,
    /// keyId contains the public key identifier of the trust anchor public key.
    pub key_id: KeyIdentifier,
    /// taTitle is OPTIONAL.  When it is present, it provides a human-readable name
    /// for the trust anchor.
    pub ta_title: Option<TrustAnchorTitle>,
    /// certPath is OPTIONAL.  When it is present, it provides the controls
    /// needed to initialize an X.509 certification path validation algorithm
    /// implementation (see Section 6 of [RFC 5280]).  When absent, the trust
    /// anchor cannot be used to validate the signature on an X.509
    /// certificate.
    pub cert_path: Option<CertPathControls>,
    #[rasn(tag(explicit(1)))]
    /// exts is OPTIONAL.  When it is present, it can be used to associate
    /// additional information with the trust anchor using the standard
    /// Extensions structure.  Extensions that are anticipated to be widely
    /// used have been included in the CertPathControls structure to avoid
    /// overhead associated with use of the Extensions structure.  To avoid
    /// duplication with the CertPathControls field, the following types of
    /// extensions MUST NOT appear in the exts field and are ignored if they
    /// do appear: id-ce-certificatePolicies, id-ce-policyConstraints, id-ce-
    /// inhibitAnyPolicy, or id-ce-nameConstraints.
    pub exts: Option<Extensions>,
    #[rasn(tag(2))]
    /// The taTitleLangTag field identifies the language used to express the
    /// taTitle.  When taTitleLangTag is absent, English ("en" language tag)
    /// is used.
    pub ta_title_lang_tag: Option<Utf8String>,
}

/// CertPathControls provides the controls needed to initialize an X.509
// certification path validation algorithm implementation
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct CertPathControls {
    /// taName provides the X.500 distinguished name associated with the
    /// trust anchor, and this distinguished name is used to construct and
    /// validate an X.509 certification path.  The name MUST NOT be an empty
    /// sequence.
    pub ta_name: Name,
    #[rasn(tag(0))]
    /// certificate provides an OPTIONAL X.509 certificate, which can be used
    /// in some environments to represent the trust anchor in certification
    /// path development and validation
    pub certificate: Option<Certificate>,
    #[rasn(tag(1))]
    /// policySet contains a sequence of
    /// certificate policy identifiers to be provided as inputs to the
    /// certification path validation algorithm.
    pub policy_set: Option<CertificatePolicies>,
    #[rasn(tag(2))]
    /// policyFlags is OPTIONAL.  When present, three Boolean values for
    /// input to the certification path validation algorithm are provided in
    /// a BIT STRING.  When absent, the input to the certification path
    /// validation algorithm is { FALSE, FALSE, FALSE }, which represents the
    /// most liberal setting for these flags.
    pub policy_flags: Option<CertPolicyFlags>,
    #[rasn(tag(3))]
    /// nameConstrhas the same syntax and semantics as the
    /// Name Constraints certificate extension [RFC 5280], which includes a
    /// list of permitted names and a list of excluded names.
    pub name_constr: Option<NameConstraints>,
    #[rasn(tag(4))]
    /// The pathLenConstraint field gives the maximum number of non-self-
    /// issued intermediate certificates that may follow this certificate in
    /// a valid certification path.  (Note: The last certificate in the
    /// certification path is not an intermediate certificate and is not
    /// included in this limit.  Usually, the last certificate is an end
    /// entity certificate, but it can be a CA certificate.
    pub path_len_constraint: Option<Integer>,
}

/// TrustAnchorChoice provides three options for representing a trust anchor.
///
/// The certificate option allows for the use of a certificate with no additional
/// associated constraints.
///
/// The tbsCert option allows for associating constraints by removing a signature
/// on a certificate and changing the extensions field.
///
/// The taInfo option allows for use of the TrustAnchorInfo structure defined
/// in RFC-5914.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum TrustAnchorChoice {
    Certificate(Certificate),
    #[rasn(tag(explicit(1)))]
    TbsCertificate(TbsCertificate),
    #[rasn(tag(explicit(2)))]
    TrustAnchorInfo(alloc::boxed::Box<TrustAnchorInfo>),
}

/// TrustAnchorInfoVersion identifies the version of TrustAnchorInfo. Future updates
/// to RFC 5914 may include changes to the TrustAnchorInfo structure,
/// in which case the version number should be incremented.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct TrustAnchorInfoVersion(u64);

impl TrustAnchorInfoVersion {
    pub const V1: Self = Self(1);

    /// Returns the raw value of the version. Note that the version is
    /// one-indexed.
    pub fn raw_value(self) -> u64 {
        self.0
    }
}

impl Default for TrustAnchorInfoVersion {
    fn default() -> Self {
        Self::V1
    }
}

impl core::fmt::Display for TrustAnchorInfoVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}", self.0)
    }
}

/// The validity period of the certificate.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Validity {
    pub not_before: Time,
    pub not_after: Time,
}

/// A general time type.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum Time {
    Utc(UtcTime),
    General(GeneralizedTime),
}

/// The subject's public key, and the algorithm used to encode it.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectPublicKeyInfo {
    pub algorithm: AlgorithmIdentifier,
    pub subject_public_key: BitString,
}

/// Identifying the public key corresponding to the private key used to sign a
/// certificate.
///
/// This extension is used where an issuer has multiple signing keys (either due
/// to multiple concurrent key pairs or due to changeover).  The identification
/// MAY be based on either the key identifier (the subject key identifier in the
/// issuer's certificate) or the issuer name and serial number.
#[derive(AsnType, Default, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AuthorityKeyIdentifier {
    #[rasn(tag(0))]
    pub key_identifier: Option<KeyIdentifier>,
    #[rasn(tag(1))]
    pub authority_cert_issuer: Option<GeneralNames>,
    #[rasn(tag(2))]
    pub authority_cert_serial_number: Option<CertificateSerialNumber>,
}

/// Extension to an X.509 certificate.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Extension {
    pub extn_id: ObjectIdentifier,
    #[rasn(default)]
    pub critical: bool,
    pub extn_value: OctetString,
}

/// A signed list of revoked certificates.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct CertificateList {
    pub tbs_cert_list: TbsCertList,
    pub signature_algorithm: AlgorithmIdentifier,
    pub signature: BitString,
}

/// The list of revoked certificates along with associated metadata.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TbsCertList {
    /// The version of the list.
    pub version: Version,
    /// The algorithm used in the signature.
    pub signature: AlgorithmIdentifier,
    /// The authority that issued the certificate list.
    pub issuer: Name,
    /// The issue date of this list.
    pub this_update: Time,
    /// When the next update will be available. The update may be available
    /// sooner than `next_update`, but it will not be issued later.
    pub next_update: Option<Time>,
    /// The list of revoked certificates.
    pub revoked_certificates: SequenceOf<RevokedCerificate>,
    /// Extensions to the list.
    #[rasn(tag(explicit(0)))]
    pub crl_extensions: Option<Extensions>,
}

/// Identifies a revoked certificate.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RevokedCerificate {
    /// The ID of the certificate being revoked.
    pub user_certificate: CertificateSerialNumber,
    /// When the certificate was revoked.
    pub revocation_date: Time,
    /// Extensions to the revoked entry.
    pub crl_entry_extensions: Option<Extensions>,
}

/// Identifies what algorithm was used, along with any parameters used as input.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlgorithmIdentifier {
    /// The identifier for the algorithm.
    pub algorithm: ObjectIdentifier,
    /// Parameters for the algorithm, if any.
    pub parameters: Option<Any>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct OrAddress {
    pub built_in_standard_attributes: BuiltInStandardAttributes,
    pub built_in_domain_defined_attributes: Option<BuiltInDomainDefinedAttributes>,
    pub extension_attributes: Option<ExtensionAttributes>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltInStandardAttributes {
    pub country_name: Option<CountryName>,
    pub administraion_domain_name: Option<AdministrationDomainName>,
    #[rasn(tag(0))]
    pub network_address: Option<NetworkAddress>,
    #[rasn(tag(1))]
    pub terminal_identifier: Option<TerminalIdentifier>,
    #[rasn(tag(2))]
    pub private_domain_name: Option<PrivateDomainName>,
    #[rasn(tag(3))]
    pub organisation_name: Option<OrganisationName>,
    #[rasn(tag(4))]
    pub numeric_user_identifier: Option<NumericUserIdentifier>,
    #[rasn(tag(5))]
    pub personal_name: Option<PersonalName>,
    #[rasn(tag(6))]
    pub organisational_unit_name: Option<OrganisationalUnitNames>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=4"))]
pub struct BuiltInDomainDefinedAttributes(SequenceOf<BuiltInDomainDefinedAttribute>);
derefable!(
    BuiltInDomainDefinedAttributes,
    SequenceOf<BuiltInDomainDefinedAttribute>
);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltInDomainDefinedAttribute {
    #[rasn(size("1..=8"))]
    pub r#type: PrintableString,
    #[rasn(size("1..=128"))]
    pub value: PrintableString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(explicit(application, 1)))]
#[rasn(choice)]
pub enum CountryName {
    #[rasn(size(3))]
    X121DccCode(NumericString),
    #[rasn(size(2))]
    Iso3166Alpha2Code(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum PrivateDomainName {
    #[rasn(size("1..=16"))]
    Numeric(NumericString),
    #[rasn(size("1..=16"))]
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(tag(explicit(application, 2)), choice)]
pub enum AdministrationDomainName {
    #[rasn(size("0..=16"))]
    Numeric(NumericString),
    #[rasn(size("0..=16"))]
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(set)]
pub struct PersonalName {
    #[rasn(tag(0))]
    pub surname: PrintableString,
    #[rasn(tag(1))]
    pub given_name: Option<PrintableString>,
    #[rasn(tag(2))]
    pub initials: Option<PrintableString>,
    #[rasn(tag(3))]
    pub generation_qualifier: Option<PrintableString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=4"))]
pub struct OrganisationalUnitNames(SequenceOf<OrganisationalUnitName>);
derefable!(OrganisationalUnitNames, SequenceOf<OrganisationalUnitName>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=32"))]
pub struct OrganisationalUnitName(PrintableString);
derefable!(OrganisationalUnitName, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1.."))]
pub struct Extensions(SequenceOf<Extension>);
derefable!(Extensions, SequenceOf<Extension>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1.."))]
pub struct RelativeDistinguishedName(SetOf<AttributeTypeAndValue>);
derefable!(RelativeDistinguishedName, SetOf<AttributeTypeAndValue>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=256"))]
pub struct ExtensionAttributes(SetOf<ExtensionAttribute>);
derefable!(ExtensionAttributes, SetOf<ExtensionAttribute>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionAttribute {
    #[rasn(tag(0), value("0..=256"))]
    pub extension_attribute_type: u16,
    #[rasn(tag(1))]
    pub extension_attribute_value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(set)]
pub struct TeletexPersonalName {
    #[rasn(tag(0), size("1..=40"))]
    pub surname: TeletexString,
    #[rasn(tag(1), size("1..=16"))]
    pub given_name: Option<TeletexString>,
    #[rasn(tag(2), size("1..=5"))]
    pub initials: Option<TeletexString>,
    #[rasn(tag(3), size("1..=3"))]
    pub generation_qualifier: Option<TeletexString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum PhysicalDeliveryCountryName {
    #[rasn(size(3))]
    X121DccCode(NumericString),
    #[rasn(size(2))]
    Iso3166Alpha2Code(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum PostalCode {
    #[rasn(size("1..=16"))]
    Numeric(NumericString),
    #[rasn(size("1..=16"))]
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=64"))]
pub struct CommonName(PrintableString);
derefable!(CommonName, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=64"))]
pub struct TeletexCommonName(TeletexString);
derefable!(TeletexCommonName, TeletexString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=64"))]
pub struct TeletexOrganizationName(TeletexString);
derefable!(TeletexOrganizationName, TeletexString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=4"))]
pub struct TeletexOrganisationalUnitNames(SequenceOf<TeletexOrganisationalUnitName>);
derefable!(
    TeletexOrganisationalUnitNames,
    SequenceOf<TeletexOrganisationalUnitName>
);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=32"))]
pub struct TeletexOrganisationalUnitName(TeletexString);
derefable!(TeletexOrganisationalUnitName, TeletexString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=16"))]
pub struct PdsName(PrintableString);
derefable!(PdsName, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=30"))]
pub struct PrintableAddress(PrintableString);
derefable!(PrintableAddress, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=180"))]
pub struct TeletexAddress(TeletexString);
derefable!(TeletexAddress, TeletexString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(set)]
pub struct UnformattedPostalAddress {
    #[rasn(size("1..=6"))]
    pub printable_address: Option<SequenceOf<PrintableAddress>>,
    pub teletex_string: Option<TeletexAddress>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(set)]
pub struct PdsParameter {
    #[rasn(size("1..=30"))]
    pub printable_string: Option<PrintableString>,
    #[rasn(size("1..=30"))]
    pub teletex_string: Option<TeletexString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum ExtendedNetworkAddress {
    E1634Address(E1634Address),
    #[rasn(tag(0))]
    PsapAddress(PresentationAddress),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct E1634Address {
    #[rasn(tag(0), size("1..=15"))]
    pub number: NumericString,
    #[rasn(tag(1), size("1..=40"))]
    pub sub_address: Option<NumericString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PresentationAddress {
    #[rasn(tag(explicit(0)))]
    pub p_selector: Option<OctetString>,
    #[rasn(tag(explicit(1)))]
    pub s_selector: Option<OctetString>,
    #[rasn(tag(explicit(2)))]
    pub t_selector: Option<OctetString>,
    #[rasn(tag(explicit(3)), size("1.."))]
    pub n_addresses: SetOf<OctetString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=4"))]
pub struct TeletexDomainDefinedAttributes(SequenceOf<TeletexDomainDefinedAttribute>);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TeletexDomainDefinedAttribute {
    #[rasn(size("1..=8"))]
    pub r#type: TeletexString,
    #[rasn(size("1..=128"))]
    pub value: TeletexString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Name {
    RdnSequence(RdnSequence),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub r#type: AttributeType,
    pub values: SetOf<AttributeValue>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AttributeTypeAndValue {
    pub r#type: AttributeType,
    pub value: AttributeValue,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyInformation {
    pub policy_identifier: CertPolicyId,
    pub policy_qualifiers: Option<SequenceOf<PolicyQualifierInfo>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyQualifierInfo {
    pub id: PolicyQualifierId,
    pub qualifier: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserNotice {
    pub notice_ref: Option<NoticeReference>,
    pub explicit_text: Option<DisplayText>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoticeReference {
    pub organisation: DisplayText,
    pub notice_numbers: SequenceOf<Integer>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum DisplayText {
    Ia5String(Ia5String),
    VisibleString(VisibleString),
    BmpString(BmpString),
    Utf8String(Utf8String),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyMapping {
    pub issuer_domain_policy: CertPolicyId,
    pub subject_domain_policy: CertPolicyId,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum GeneralName {
    #[rasn(tag(0))]
    OtherName(InstanceOf<OctetString>),
    #[rasn(tag(1))]
    Rfc822Name(Ia5String),
    #[rasn(tag(2))]
    DnsName(Ia5String),
    // Boxed because it's 368 bytes, and the next largest enum variant is 64.
    #[rasn(tag(3))]
    X400Address(alloc::boxed::Box<OrAddress>),
    #[rasn(tag(4))]
    DirectoryName(Name),
    #[rasn(tag(5))]
    EdiPartyName(EdiPartyName),
    #[rasn(tag(6))]
    Uri(Ia5String),
    #[rasn(tag(7))]
    IpAddress(OctetString),
    #[rasn(tag(8))]
    RegisteredId(ObjectIdentifier),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdiPartyName {
    #[rasn(tag(0))]
    pub name_assigner: Option<DirectoryString>,
    #[rasn(tag(1))]
    pub party_name: DirectoryString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size(2))]
pub struct X520CountryName(PrintableString);
derefable!(X520CountryName, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate, size("1..=64"))]
pub struct X520SerialNumber(PrintableString);
derefable!(X520SerialNumber, PrintableString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520StateOrProvinceName {
    #[rasn(size("1..=128"))]
    Teletex(TeletexString),
    #[rasn(size("1..=128"))]
    Printable(PrintableString),
    #[rasn(size("1..=128"))]
    Universal(UniversalString),
    #[rasn(size("1..=128"))]
    Utf8(Utf8String),
    #[rasn(size("1..=128"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520OrganisationName {
    #[rasn(size("1..=64"))]
    Teletex(TeletexString),
    #[rasn(size("1..=64"))]
    Printable(PrintableString),
    #[rasn(size("1..=64"))]
    Universal(UniversalString),
    #[rasn(size("1..=64"))]
    Utf8(Utf8String),
    #[rasn(size("1..=64"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520OrganisationalUnitName {
    #[rasn(size("1..=64"))]
    Teletex(TeletexString),
    #[rasn(size("1..=64"))]
    Printable(PrintableString),
    #[rasn(size("1..=64"))]
    Universal(UniversalString),
    #[rasn(size("1..=64"))]
    Utf8(Utf8String),
    #[rasn(size("1..=64"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520Title {
    #[rasn(size("1..=64"))]
    Teletex(TeletexString),
    #[rasn(size("1..=64"))]
    Printable(PrintableString),
    #[rasn(size("1..=64"))]
    Universal(UniversalString),
    #[rasn(size("1..=64"))]
    Utf8(Utf8String),
    #[rasn(size("1..=64"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520Pseudonym {
    #[rasn(size("1..=128"))]
    Teletex(TeletexString),
    #[rasn(size("1..=128"))]
    Printable(PrintableString),
    #[rasn(size("1..=128"))]
    Universal(UniversalString),
    #[rasn(size("1..=128"))]
    Utf8(Utf8String),
    #[rasn(size("1..=128"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520LocalityName {
    #[rasn(size("1..=128"))]
    Teletex(TeletexString),
    #[rasn(size("1..=128"))]
    Printable(PrintableString),
    #[rasn(size("1..=128"))]
    Universal(UniversalString),
    #[rasn(size("1..=128"))]
    Utf8(Utf8String),
    #[rasn(size("1..=128"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520Name {
    #[rasn(size("1..=32768"))]
    Teletex(TeletexString),
    #[rasn(size("1..=32768"))]
    Printable(PrintableString),
    #[rasn(size("1..=32768"))]
    Universal(UniversalString),
    #[rasn(size("1..=32768"))]
    Utf8(Utf8String),
    #[rasn(size("1..=32768"))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum X520CommonName {
    #[rasn(size("1..=64"))]
    Teletex(TeletexString),
    #[rasn(size("1..=64"))]
    Printable(PrintableString),
    #[rasn(size("1..=64"))]
    Universal(UniversalString),
    #[rasn(size("1..=64"))]
    Utf8(Utf8String),
    #[rasn(size("1..=64"))]
    Bmp(BmpString),
}

macro_rules! directory_string_compat {
    ($($from:ident),+ $(,)?) => {
        $(
            impl PartialEq<$from> for DirectoryString {
                fn eq(&self, rhs: &$from) -> bool {
                    match (rhs, self) {
                        ($from::Teletex(lhs), Self::Teletex(rhs)) => lhs == rhs,
                        ($from::Printable(lhs), Self::Printable(rhs)) => lhs == rhs,
                        ($from::Universal(lhs), Self::Universal(rhs)) => lhs == rhs,
                        ($from::Utf8(lhs), Self::Utf8(rhs)) => lhs == rhs,
                        ($from::Bmp(lhs), Self::Bmp(rhs)) => lhs == rhs,
                        _ => false,
                    }
                }
            }

            impl From<$from> for DirectoryString {
                fn from(value: $from) -> Self {
                    match value {
                        $from::Teletex(value) => Self::Teletex(value),
                        $from::Printable(value) => Self::Printable(value),
                        $from::Universal(value) => Self::Universal(value),
                        $from::Utf8(value) => Self::Utf8(value),
                        $from::Bmp(value) => Self::Bmp(value),
                    }
                }
            }
        )+
    }
}

directory_string_compat! {
    X520Name,
    X520CommonName,
    X520LocalityName,
    X520Pseudonym,
    X520Title,
    X520OrganisationalUnitName,
    X520OrganisationName,
    X520StateOrProvinceName,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(choice)]
pub enum DirectoryString {
    #[rasn(size("1.."))]
    Teletex(TeletexString),
    #[rasn(size("1.."))]
    Printable(PrintableString),
    #[rasn(size("1.."))]
    Universal(UniversalString),
    #[rasn(size("1.."))]
    Utf8(Utf8String),
    #[rasn(size("1.."))]
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicConstraints {
    #[rasn(default)]
    pub ca: bool,
    pub path_len_constraint: Option<Integer>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct NameConstraints {
    #[rasn(tag(0))]
    pub permitted_subtrees: Option<GeneralSubtrees>,
    #[rasn(tag(1))]
    pub excluded_subtrees: Option<GeneralSubtrees>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct GeneralSubtree {
    pub base: GeneralName,
    #[rasn(tag(0), default)]
    pub minimum: BaseDistance,
    #[rasn(tag(1))]
    pub maximum: Option<BaseDistance>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyConstraints {
    #[rasn(tag(0))]
    pub require_explicit_policy: Option<SkipCerts>,
    #[rasn(tag(1))]
    pub inhibit_policy_mapping: Option<SkipCerts>,
}

#[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DistributionPoint {
    #[rasn(tag(0))]
    pub distribution_point: Option<DistributionPointName>,
    #[rasn(tag(1))]
    pub reasons: Option<ReasonFlags>,
    #[rasn(tag(2))]
    pub crl_issuer: Option<GeneralNames>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum DistributionPointName {
    #[rasn(tag(0))]
    FullName(GeneralNames),
    #[rasn(tag(1))]
    NameRelativeToCrlIssuer(RelativeDistinguishedName),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AccessDescription {
    pub access_method: ObjectIdentifier,
    pub access_location: GeneralName,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum CrlReason {
    Unspecified = 0,
    KeyCompromise = 1,
    CaCompromise = 2,
    AffiliationChanged = 3,
    Superseded = 4,
    CessationOfOperation = 5,
    CertificateHold = 6,
    RemoveFromCRL = 8,
    PrivilegeWithdrawn = 9,
    AaCompromise = 10,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct IssuingDistributionPoint {
    #[rasn(tag(0))]
    pub distribution_point: Option<DistributionPointName>,
    #[rasn(tag(1), default)]
    pub only_contains_user_certs: bool,
    #[rasn(tag(2), default)]
    pub only_contains_ca_certs: bool,
    #[rasn(tag(3))]
    pub only_some_reasons: Option<ReasonFlags>,
    #[rasn(tag(4), default)]
    pub indirect_crl: bool,
    #[rasn(tag(5), default)]
    pub only_contains_attribute_certs: bool,
}

/** PrivateKeyUsagePeriod

This certificate extensions was deprecated in
[RFC3280 4.2.1.4](https://www.rfc-editor.org/rfc/rfc3280#section-4.2.1.4) but
this was undone by
[RFC5280 1](https://www.rfc-editor.org/rfc/rfc5280#section-1).

[RFC 5280 A.2](https://www.rfc-editor.org/rfc/rfc5280#appendix-A.2):

```text
   -- private key usage period extension OID and syntax

   id-ce-privateKeyUsagePeriod OBJECT IDENTIFIER ::=  { id-ce 16 }

   PrivateKeyUsagePeriod ::= SEQUENCE {
        notBefore       [0]     GeneralizedTime OPTIONAL,
        notAfter        [1]     GeneralizedTime OPTIONAL }
        -- either notBefore or notAfter MUST be present
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrivateKeyUsagePeriod {
    #[rasn(tag(0))]
    pub not_before: Option<GeneralizedTime>,
    #[rasn(tag(1))]
    pub not_after: Option<GeneralizedTime>,
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;

    #[test]
    fn time() {
        rasn::der::decode::<Time>(&[
            0x17, 0x0D, 0x31, 0x38, 0x30, 0x32, 0x30, 0x39, 0x31, 0x32, 0x33, 0x32, 0x30, 0x37,
            0x5A,
        ])
        .unwrap();
    }

    #[test]
    fn algorithm_identifier() {
        let expected_de = AlgorithmIdentifier {
            algorithm: ObjectIdentifier::new_unchecked((&[1, 2, 840, 113549, 1, 1, 1][..]).into()),
            parameters: Some(Any::new(rasn::der::encode(&()).unwrap())),
        };

        let expected_enc = &[
            0x30, 0x0D, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01, 0x05,
            0x00,
        ][..];

        assert_eq!(expected_enc, rasn::der::encode(&expected_de).unwrap());
        assert_eq!(expected_de, rasn::der::decode(expected_enc).unwrap());
    }

    #[test]
    fn certificate_policies() {
        let expected_de: CertificatePolicies = alloc::vec![
            PolicyInformation {
                policy_identifier: ObjectIdentifier::new_unchecked(
                    (&[2, 23, 140, 1, 2, 1][..]).into()
                ),
                policy_qualifiers: None,
            },
            PolicyInformation {
                policy_identifier: ObjectIdentifier::new_unchecked(
                    (&[1, 3, 6, 1, 4, 1, 44947, 1, 1, 1][..]).into()
                ),
                policy_qualifiers: Some(alloc::vec![PolicyQualifierInfo {
                    id: ObjectIdentifier::new_unchecked((&[1, 3, 6, 1, 5, 5, 7, 2, 1][..]).into()),
                    qualifier: Any::new(
                        rasn::der::encode(
                            &Ia5String::try_from(alloc::string::String::from(
                                "http://cps.root-x1.letsencrypt.org"
                            ))
                            .unwrap()
                        )
                        .unwrap()
                    ),
                }]),
            }
        ];

        let expected_enc = &[
            0x30, 0x4B, 0x30, 0x08, 0x06, 0x06, 0x67, 0x81, 0x0C, 0x01, 0x02, 0x01, 0x30, 0x3F,
            0x06, 0x0B, 0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0xDF, 0x13, 0x01, 0x01, 0x01, 0x30,
            0x30, 0x30, 0x2E, 0x06, 0x08, 0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x02, 0x01, 0x16,
            0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x63, 0x70, 0x73, 0x2E, 0x72, 0x6F,
            0x6F, 0x74, 0x2D, 0x78, 0x31, 0x2E, 0x6C, 0x65, 0x74, 0x73, 0x65, 0x6E, 0x63, 0x72,
            0x79, 0x70, 0x74, 0x2E, 0x6F, 0x72, 0x67,
        ][..];

        assert_eq!(expected_enc, rasn::der::encode(&expected_de).unwrap());
        assert_eq!(
            expected_de,
            rasn::der::decode::<CertificatePolicies>(expected_enc).unwrap()
        );
    }

    #[test]
    fn trust_anchor_info_version() {
        let alg_id = AlgorithmIdentifier {
            algorithm: ObjectIdentifier::new_unchecked((&[1, 2][..]).into()),
            parameters: None,
        };

        let spki = SubjectPublicKeyInfo {
            algorithm: alg_id,
            subject_public_key: Default::default(),
        };

        let tai = TrustAnchorInfo {
            version: Default::default(),
            pub_key: spki,
            key_id: Default::default(),
            ta_title: None,
            cert_path: None,
            exts: None,
            ta_title_lang_tag: None,
        };

        let expected_enc = &[
            0x30, 0x0c, 0x30, 0x08, 0x30, 0x03, 0x06, 0x01, 0x2A, 0x03, 0x01, 0x00, 0x04, 0x00,
        ][..];
        let actual_enc = rasn::der::encode(&tai).unwrap();

        assert_eq!(expected_enc, actual_enc);
        assert_eq!(TrustAnchorInfoVersion(1), tai.version);
    }
}
