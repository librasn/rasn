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
pub type SubjectDirectoryAttributes = Vec<Attribute>;
pub type GeneralNames = SequenceOf<GeneralName>;
pub type SubjectAltName = GeneralNames;
pub type PolicyMappings = SequenceOf<PolicyMapping>;
pub type CpsUri = Ia5String;
pub type PolicyQualifierInfo = InstanceOf<Any>;
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Certificate {
    pub tbs_certificate: TbsCertificate,
    pub signature_algorithm: AlgorithmIdentifier,
    pub signature: BitString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct TbsCertificate {
    #[rasn(tag(explicit(0)), default)]
    pub version: Version,
    pub serial_number: CertificateSerialNumber,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub validity: Validity,
    pub subject: Name,
    pub subject_public_key_info: SubjectPublicKeyInfo,
    #[rasn(tag(1))]
    pub issuer_unique_id: Option<UniqueIdentifier>,
    #[rasn(tag(2))]
    pub subject_unique_id: Option<UniqueIdentifier>,
    #[rasn(tag(3))]
    pub extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Version(u64);

impl Version {
    pub const V1: Self = Self(0);
    pub const V2: Self = Self(1);
    pub const V3: Self = Self(2);
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Validity {
    pub not_before: Time,
    pub not_after: Time,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[rasn(choice)]
pub enum Time {
    Utc(UtcTime),
    General(GeneralizedTime),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubjectPublicKeyInfo {
    pub algorithm: AlgorithmIdentifier,
    pub subject_public_key: BitString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Extension {
    pub extn_id: ObjectIdentifier,
    #[rasn(default)]
    pub critical: bool,
    pub extn_value: OctetString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct CertificateList {
    pub tbs_cert_list: TbsCertList,
    pub signature_algorithim: AlgorithmIdentifier,
    pub signature: BitString,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct TbsCertList {
    pub version: Version,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub this_update: Time,
    pub next_update: Option<Time>,
    pub revoked_certificates: Vec<RevokedCerificate>,
    #[rasn(tag(0))]
    pub crl_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevokedCerificate {
    pub user_certificate: CertificateSerialNumber,
    pub revocation_date: Time,
    pub crl_entry_extensions: Option<Extensions>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlgorithmIdentifier {
    pub algorithm: ObjectIdentifier,
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
    PsapAddress(PresentationAddress)
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
    r#type: AttributeType,
    pub value: SetOf<AttributeValue>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyInformation {
    pub policy_identifier: CertPolicyId,
    pub policy_qualifiers: Vec<PolicyQualifierInfo>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserNotice {
    pub notice_ref: Option<NoticeReference>,
    pub explicit_text: Option<DisplayText>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoticeReference {
    pub organisation: DisplayText,
    pub notice_numbers: Vec<Integer>,
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

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
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
        rasn::der::decode::<Time>(&[0x17, 0x0D, 0x31, 0x38, 0x30, 0x32, 0x30, 0x39, 0x31, 0x32, 0x33, 0x32, 0x30, 0x37, 0x5A]).unwrap();
    }
}
