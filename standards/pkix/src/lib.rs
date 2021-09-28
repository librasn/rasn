//! # Public Key Infrastructure Certificate and Certificate Revocation List (CRL) Profile
//!
//! `rasn-pkix` is an implementation of the data types defined in IETF
//! [RFC 5280] also known PKIX. This does not provide an implementation of a
//! PKIX certificate generator or validator, `rasn-pkix` provides a
//! implementation of the underlying data types used decode and
//! encode certificates from DER.
//!
//! [RFC 3279]: https://datatracker.ietf.org/doc/html/rfc3279
//! [RFC 4055]: https://datatracker.ietf.org/doc/html/rfc4055
//! [RFC 4491]: https://datatracker.ietf.org/doc/html/rfc4491
//! [RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280

#![no_std]

pub mod est;

use rasn::{types::*, Decode, Encode};

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
pub type TeletexDomainDefinedAttributes = SequenceOf<TeletexDomainDefinedAttribute>;
pub type AttributeType = ObjectIdentifier;
pub type AttributeValue = Any;
pub type RdnSequence = SequenceOf<RelativeDistinguishedName>;
pub type RelativeDistinguishedName = SetOf<Attribute>;
pub type X520Name = DirectoryString;
pub type X520CommonName = DirectoryString;
pub type X520LocalityName = DirectoryString;
pub type X520StateOrProvinceName = DirectoryString;
pub type X520OrganisationName = DirectoryString;
pub type X520OrganisationalUnitName = DirectoryString;
pub type X520Title = DirectoryString;
pub type X520DnQualifier = PrintableString;
pub type X520CountryName = PrintableString;
pub type X520SerialNumber = PrintableString;
pub type X520Pseudonym = DirectoryString;
pub type DomainComponent = Ia5String;
pub type EmailAddress = Ia5String;
pub type CertificateSerialNumber = Integer;
pub type UniqueIdentifier = BitString;
pub type Extensions = SequenceOf<Extension>;
pub type NetworkAddress = X121Address;
pub type X121Address = NumericString;
pub type TerminalIdentifier = PrintableString;
pub type OrganisationName = PrintableString;
pub type NumericUserIdentifier = NumericString;
pub type OrganisationalUnitNames = SequenceOf<OrganisationalUnitName>;
pub type OrganisationalUnitName = PrintableString;
pub type ExtensionAttributes = SetOf<ExtensionAttribute>;
pub type CommonName = PrintableString;
pub type TeletexCommonName = TeletexString;
pub type TeletexOrganisationalUnitNames = SequenceOf<TeletexOrganisationalUnitName>;
pub type TeletexOrganisationalUnitName = TeletexString;
pub type PdsName = PrintableString;
pub type TerminalType = u8;
pub type BuiltInDomainDefinedAttributes = SequenceOf<BuiltInDomainDefinedAttribute>;
pub type KeyIdentifier = OctetString;
pub type SubjectKeyIdentifier = KeyIdentifier;
pub type PolicyQualifierId = ObjectIdentifier;

/// An X.509 certificate
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

/// The validity period of the certificate.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Validity {
    pub not_before: Time,
    pub not_after: Time,
}

/// A general time type.
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum Time {
    Utc(UtcTime),
    General(GeneralizedTime),
}

/// The subject's public key, and the algorithm used to encode it.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(AsnType, Default, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorityKeyIdentifier {
    #[rasn(tag(0))]
    pub key_identifier: Option<KeyIdentifier>,
    #[rasn(tag(1))]
    pub authority_cert_issuer: Option<GeneralNames>,
    #[rasn(tag(2))]
    pub authority_cert_serial_number: Option<CertificateSerialNumber>,
}

/// Extension to an X.509 certificate.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Extension {
    pub extn_id: ObjectIdentifier,
    #[rasn(default)]
    pub critical: bool,
    pub extn_value: OctetString,
}

/// A signed list of revoked certificates.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct CertificateList {
    pub tbs_cert_list: TbsCertList,
    pub signature_algorithim: AlgorithmIdentifier,
    pub signature: BitString,
}

/// The list of revoked certificates along with associated metadata.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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
    #[rasn(tag(0))]
    pub crl_extensions: Option<Extensions>,
}

/// Identifies a revoked certificate.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevokedCerificate {
    /// The ID of the certificate being revoked.
    pub user_certificate: CertificateSerialNumber,
    /// When the certificate was revoked.
    pub revocation_date: Time,
    /// Extensions to the revoked entry.
    pub crl_entry_extensions: Option<Extensions>,
}

/// Identifies what algorithm was used, along with any parameters used as input.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlgorithmIdentifier {
    /// The identifier for the algorithm.
    pub algorithm: ObjectIdentifier,
    /// Parameters for the algorithm, if any.
    pub parameters: Option<Any>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrAddress {
    pub built_in_standard_attributes: BuiltInStandardAttributes,
    pub built_in_domain_defined_attributes: Option<BuiltInDomainDefinedAttributes>,
    pub extension_attributes: Option<ExtensionAttributes>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuiltInDomainDefinedAttribute {
    pub r#type: PrintableString,
    pub value: PrintableString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(tag(application, 1))]
#[rasn(choice)]
pub enum CountryName {
    X121DccCode(NumericString),
    Iso3166Alpha2Code(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(tag(application, 2))]
#[rasn(choice)]
pub enum AdministrationDomainName {
    Numeric(NumericString),
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum PrivateDomainName {
    Numeric(NumericString),
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionAttribute {
    #[rasn(tag(0))]
    pub extension_attribute_type: Integer,
    #[rasn(tag(1))]
    pub extension_attribute_value: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(set)]
pub struct TeletexPersonalName {
    #[rasn(tag(0))]
    pub surname: TeletexString,
    #[rasn(tag(1))]
    pub given_name: Option<TeletexString>,
    #[rasn(tag(2))]
    pub initials: Option<TeletexString>,
    #[rasn(tag(3))]
    pub generation_qualifier: Option<TeletexString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum PhysicalDeliveryCountryName {
    X121DccCode(NumericString),
    Iso3166Alpha2Code(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum PostalCode {
    Numeric(NumericString),
    Printable(PrintableString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(set)]
pub struct UnformattedPostalAddress {
    pub printable_address: Option<SequenceOf<PrintableString>>,
    pub teletex_string: Option<TeletexString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(set)]
pub struct PdsParameter {
    pub printable_string: Option<PrintableString>,
    pub teletex_string: Option<TeletexString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum ExtendedNetworkAddress {
    E1634Address(E1634Address),
    #[rasn(tag(0))]
    PsapAddress(PresentationAddress),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct E1634Address {
    #[rasn(tag(0))]
    pub number: NumericString,
    #[rasn(tag(1))]
    pub sub_address: Option<NumericString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PresentationAddress {
    #[rasn(tag(explicit(0)))]
    pub p_selector: Option<OctetString>,
    #[rasn(tag(explicit(1)))]
    pub s_selector: Option<OctetString>,
    #[rasn(tag(explicit(2)))]
    pub t_selector: Option<OctetString>,
    #[rasn(tag(explicit(3)))]
    pub n_addresses: SetOf<OctetString>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexDomainDefinedAttribute {
    pub r#type: TeletexString,
    pub value: TeletexString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum Name {
    RdnSequence(RdnSequence),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialOrd, Ord, PartialEq, Eq)]
pub struct Attribute {
    pub r#type: AttributeType,
    pub value: AttributeValue,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyInformation {
    pub policy_identifier: CertPolicyId,
    pub policy_qualifiers: Option<SequenceOf<PolicyQualifierInfo>>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyQualifierInfo {
    pub id: PolicyQualifierId,
    pub qualifier: Any,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserNotice {
    pub notice_ref: Option<NoticeReference>,
    pub explicit_text: Option<DisplayText>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoticeReference {
    pub organisation: DisplayText,
    pub notice_numbers: SequenceOf<Integer>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum DisplayText {
    Ia5String(Ia5String),
    VisibleString(VisibleString),
    BmpString(BmpString),
    Utf8String(Utf8String),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyMapping {
    pub issuer_domain_policy: CertPolicyId,
    pub subject_domain_policy: CertPolicyId,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum GeneralName {
    #[rasn(tag(0))]
    OtherName(InstanceOf<OctetString>),
    #[rasn(tag(1))]
    Rfc822Name(Ia5String),
    #[rasn(tag(2))]
    DnsName(Ia5String),
    #[rasn(tag(3))]
    X400Address(OrAddress),
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdiPartyName {
    #[rasn(tag(0))]
    pub name_assigner: Option<DirectoryString>,
    #[rasn(tag(1))]
    pub party_name: DirectoryString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum DirectoryString {
    Teletex(TeletexString),
    Printable(PrintableString),
    Universal(UniversalString),
    Utf8(Utf8String),
    Bmp(BmpString),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct BasicConstraints {
    #[rasn(default)]
    pub ca: bool,
    pub path_len_constraint: Option<Integer>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameConstraints {
    #[rasn(tag(0))]
    pub permitted_subtrees: Option<GeneralSubtrees>,
    #[rasn(tag(1))]
    pub excluded_subtrees: Option<GeneralSubtrees>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralSubtree {
    pub base: GeneralName,
    #[rasn(tag(0), default)]
    pub minimum: BaseDistance,
    #[rasn(tag(1))]
    pub maximum: Option<BaseDistance>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyConstraints {
    #[rasn(tag(0))]
    pub require_explicit_policy: Option<SkipCerts>,
    #[rasn(tag(1))]
    pub inhibit_policy_mapping: Option<SkipCerts>,
}

#[derive(AsnType, Clone, Debug, Default, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct DistributionPoint {
    #[rasn(tag(0))]
    pub distribution_point: Option<DistributionPointName>,
    #[rasn(tag(1))]
    pub reasons: Option<ReasonFlags>,
    #[rasn(tag(2))]
    pub crl_issuer: Option<GeneralNames>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum DistributionPointName {
    #[rasn(tag(0))]
    FullName(GeneralNames),
    #[rasn(tag(1))]
    NameRelativeToCrlIssuer(RelativeDistinguishedName),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccessDescription {
    pub access_method: ObjectIdentifier,
    pub access_location: GeneralName,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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

#[cfg(test)]
mod tests {
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
        assert_eq!(expected_de, rasn::der::decode(&expected_enc).unwrap());
    }

    extern crate alloc;
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
                        rasn::der::encode(&Ia5String::from(alloc::string::String::from(
                            "http://cps.root-x1.letsencrypt.org"
                        )))
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
            rasn::der::decode::<CertificatePolicies>(&expected_enc).unwrap()
        );
    }
}
