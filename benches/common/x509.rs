//! A certificate decoded the way an application consumes it: the envelope,
//! then the body of every extension and the value of every distinguished
//! name attribute, which `rasn_pkix` leaves as opaque octets.

use rasn::error::{DecodeError, EncodeError};
use rasn::prelude::*;
use rasn_pkix::{
    AuthorityInfoAccessSyntax, AuthorityKeyIdentifier, BasicConstraints, Certificate,
    CertificatePolicies, CrlDistributionPoints, DirectoryString, ExtKeyUsageSyntax, Extension,
    KeyUsage, Name, SubjectAltName, SubjectKeyIdentifier,
};

/// A certificate with its extension bodies and name attributes decoded.
pub struct ParsedCertificate {
    pub certificate: Certificate,
    pub issuer: Vec<DirectoryString>,
    pub subject: Vec<DirectoryString>,
    pub extensions: Vec<ExtensionValue>,
}

impl ParsedCertificate {
    /// Decodes the DER certificate `der` and everything it carries.
    pub fn decode(der: &[u8]) -> Result<Self, DecodeError> {
        let certificate: Certificate = rasn::der::decode(der)?;
        let tbs = &certificate.tbs_certificate;
        let issuer = directory_strings(&tbs.issuer)?;
        let subject = directory_strings(&tbs.subject)?;
        let extensions = tbs
            .extensions
            .iter()
            .flat_map(|extensions| extensions.iter())
            .map(ExtensionValue::decode)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            certificate,
            issuer,
            subject,
            extensions,
        })
    }

    /// Encodes every part an application encodes when it builds the
    /// certificate: each attribute value and extension body, then the
    /// envelope that carries them.
    ///
    /// The envelope is encoded from the decoded certificate, which still
    /// holds those parts as octets, so they are not written into it again:
    /// the codec work is the same as for a certificate built from them,
    /// without the copies that building one would take.
    pub fn encode(&self) -> Result<Vec<Vec<u8>>, EncodeError> {
        let mut parts =
            Vec::with_capacity(self.issuer.len() + self.subject.len() + self.extensions.len() + 1);
        for value in self.issuer.iter().chain(&self.subject) {
            parts.push(rasn::der::encode(value)?);
        }
        for extension in &self.extensions {
            parts.push(extension.encode()?);
        }
        parts.push(rasn::der::encode(&self.certificate)?);
        Ok(parts)
    }
}

/// Decodes the value of every attribute in `name`.
fn directory_strings(name: &Name) -> Result<Vec<DirectoryString>, DecodeError> {
    let Name::RdnSequence(rdns) = name;
    rdns.iter()
        .flat_map(|rdn| rdn.iter())
        .map(|attribute| rasn::der::decode(attribute.value.as_bytes()))
        .collect()
}

/// The decoded body of a certificate extension.
pub enum ExtensionValue {
    AuthorityInfoAccess(AuthorityInfoAccessSyntax),
    AuthorityKeyIdentifier(AuthorityKeyIdentifier),
    BasicConstraints(BasicConstraints),
    CertificatePolicies(CertificatePolicies),
    CrlDistributionPoints(CrlDistributionPoints),
    ExtendedKeyUsage(ExtKeyUsageSyntax),
    KeyUsage(KeyUsage),
    /// The signed certificate timestamp list, an OCTET STRING whose contents
    /// are not ASN.1 (RFC 6962 §3.3).
    SignedCertificateTimestamps(OctetString),
    SubjectAltName(SubjectAltName),
    SubjectKeyIdentifier(SubjectKeyIdentifier),
}

const AUTHORITY_INFO_ACCESS: &[u32] = &[1, 3, 6, 1, 5, 5, 7, 1, 1];
const AUTHORITY_KEY_IDENTIFIER: &[u32] = &[2, 5, 29, 35];
const BASIC_CONSTRAINTS: &[u32] = &[2, 5, 29, 19];
const CERTIFICATE_POLICIES: &[u32] = &[2, 5, 29, 32];
const CRL_DISTRIBUTION_POINTS: &[u32] = &[2, 5, 29, 31];
const EXTENDED_KEY_USAGE: &[u32] = &[2, 5, 29, 37];
const KEY_USAGE: &[u32] = &[2, 5, 29, 15];
const SIGNED_CERTIFICATE_TIMESTAMPS: &[u32] = &[1, 3, 6, 1, 4, 1, 11129, 2, 4, 2];
const SUBJECT_ALT_NAME: &[u32] = &[2, 5, 29, 17];
const SUBJECT_KEY_IDENTIFIER: &[u32] = &[2, 5, 29, 14];

impl ExtensionValue {
    /// Decodes the body of `extension`, panicking on an extension this module
    /// does not decode, since the inputs are fixed.
    fn decode(extension: &Extension) -> Result<Self, DecodeError> {
        let body = &extension.extn_value;
        Ok(match &extension.extn_id[..] {
            AUTHORITY_INFO_ACCESS => Self::AuthorityInfoAccess(rasn::der::decode(body)?),
            AUTHORITY_KEY_IDENTIFIER => Self::AuthorityKeyIdentifier(rasn::der::decode(body)?),
            BASIC_CONSTRAINTS => Self::BasicConstraints(rasn::der::decode(body)?),
            CERTIFICATE_POLICIES => Self::CertificatePolicies(rasn::der::decode(body)?),
            CRL_DISTRIBUTION_POINTS => Self::CrlDistributionPoints(rasn::der::decode(body)?),
            EXTENDED_KEY_USAGE => Self::ExtendedKeyUsage(rasn::der::decode(body)?),
            KEY_USAGE => Self::KeyUsage(rasn::der::decode(body)?),
            SIGNED_CERTIFICATE_TIMESTAMPS => {
                Self::SignedCertificateTimestamps(rasn::der::decode(body)?)
            }
            SUBJECT_ALT_NAME => Self::SubjectAltName(rasn::der::decode(body)?),
            SUBJECT_KEY_IDENTIFIER => Self::SubjectKeyIdentifier(rasn::der::decode(body)?),
            _ => panic!("unknown extension {}", extension.extn_id),
        })
    }

    /// Encodes the body as DER.
    fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        match self {
            Self::AuthorityInfoAccess(value) => rasn::der::encode(value),
            Self::AuthorityKeyIdentifier(value) => rasn::der::encode(value),
            Self::BasicConstraints(value) => rasn::der::encode(value),
            Self::CertificatePolicies(value) => rasn::der::encode(value),
            Self::CrlDistributionPoints(value) => rasn::der::encode(value),
            Self::ExtendedKeyUsage(value) => rasn::der::encode(value),
            Self::KeyUsage(value) => rasn::der::encode(value),
            Self::SignedCertificateTimestamps(value) => rasn::der::encode(value),
            Self::SubjectAltName(value) => rasn::der::encode(value),
            Self::SubjectKeyIdentifier(value) => rasn::der::encode(value),
        }
    }
}
