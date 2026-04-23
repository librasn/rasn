//! RFC 5280 5.1.2.6:
//!
//! > When there are no revoked certificates, the revoked certificates list
//! > MUST be absent.

use chrono::{TimeZone as _, Utc};
use rasn::prelude::*;
use rasn_pkix::{
    AlgorithmIdentifier, CertificateSerialNumber, Extensions, Name, RevokedCertificate,
    TbsCertList, Time, Version, algorithms::ECDSA_WITH_SHA1,
};

/// Structurally identical to `TbsCertList` but with the `revoked_certificates`
/// field omitted entirely. Used as the ground-truth.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
struct TbsCertListNoRevoked {
    pub version: Version,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub this_update: Time,
    pub next_update: Option<Time>,
    #[rasn(tag(explicit(0)))]
    pub crl_extensions: Option<Extensions>,
}

/// Structurally identical to `TbsCertList` but without `#[rasn(default)]` on
/// `revoked_certificates`, so encoding with an empty `Vec` emits the
/// non-conformant extra `30 00`.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
struct TbsCertListEmitsEmpty {
    pub version: Version,
    pub signature: AlgorithmIdentifier,
    pub issuer: Name,
    pub this_update: Time,
    pub next_update: Option<Time>,
    pub revoked_certificates: SequenceOf<RevokedCertificate>,
    #[rasn(tag(explicit(0)))]
    pub crl_extensions: Option<Extensions>,
}

fn tbs_cert_list(revoked: Vec<RevokedCertificate>) -> TbsCertList {
    TbsCertList {
        version: Version::V2,
        signature: AlgorithmIdentifier {
            algorithm: ECDSA_WITH_SHA1.to_owned(),
            parameters: None,
        },
        issuer: Name::RdnSequence(vec![]),
        this_update: Time::Utc(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
        next_update: Some(Time::Utc(
            Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap(),
        )),
        revoked_certificates: revoked,
        crl_extensions: None,
    }
}

fn tbs_cert_list_no_revoked() -> TbsCertListNoRevoked {
    TbsCertListNoRevoked {
        version: Version::V2,
        signature: AlgorithmIdentifier {
            algorithm: ECDSA_WITH_SHA1.to_owned(),
            parameters: None,
        },
        issuer: Name::RdnSequence(vec![]),
        this_update: Time::Utc(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
        next_update: Some(Time::Utc(
            Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap(),
        )),
        crl_extensions: None,
    }
}

fn tbs_cert_list_emits_empty() -> TbsCertListEmitsEmpty {
    TbsCertListEmitsEmpty {
        version: Version::V2,
        signature: AlgorithmIdentifier {
            algorithm: ECDSA_WITH_SHA1.to_owned(),
            parameters: None,
        },
        issuer: Name::RdnSequence(vec![]),
        this_update: Time::Utc(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()),
        next_update: Some(Time::Utc(
            Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap(),
        )),
        revoked_certificates: vec![],
        crl_extensions: None,
    }
}

#[test]
fn empty_revoked_certificates_roundtrips() {
    let tbs = tbs_cert_list(vec![]);
    let bytes = rasn::der::encode(&tbs).unwrap();
    let decoded: TbsCertList = rasn::der::decode(&bytes).unwrap();
    assert_eq!(tbs, decoded);
}

#[test]
fn populated_revoked_certificates_roundtrips() {
    let revoked = vec![
        RevokedCertificate {
            user_certificate: CertificateSerialNumber::from(0x1234u32),
            revocation_date: Time::Utc(Utc.with_ymd_and_hms(2026, 1, 2, 3, 4, 5).unwrap()),
            crl_entry_extensions: None,
        },
        RevokedCertificate {
            user_certificate: CertificateSerialNumber::from(0x5678u32),
            revocation_date: Time::Utc(Utc.with_ymd_and_hms(2026, 2, 3, 4, 5, 6).unwrap()),
            crl_entry_extensions: None,
        },
    ];

    let tbs = tbs_cert_list(revoked);
    let bytes = rasn::der::encode(&tbs).unwrap();
    let decoded: TbsCertList = rasn::der::decode(&bytes).unwrap();

    assert_eq!(tbs, decoded);
}

#[test]
fn empty_revoked_certificates_is_absent() {
    let with_empty = rasn::der::encode(&tbs_cert_list(vec![])).unwrap();
    let without_field = rasn::der::encode(&tbs_cert_list_no_revoked()).unwrap();
    assert_eq!(
        with_empty, without_field,
        "RFC 5280 5.1.2.6: empty `revoked_certificates` must be absent, but the encoded form still contains it."
    );
}

#[test]
fn nonconformant_still_accepted() {
    let buggy_wire = rasn::der::encode(&tbs_cert_list_emits_empty()).unwrap();

    // Sanity check the peer really did emit the `30 00` slab
    let conformant_wire = rasn::der::encode(&tbs_cert_list_no_revoked()).unwrap();
    assert_eq!(buggy_wire.len(), conformant_wire.len() + 2);

    let decoded: TbsCertList = rasn::der::decode(&buggy_wire).unwrap();
    assert!(decoded.revoked_certificates.is_empty());
    assert_eq!(decoded, tbs_cert_list(vec![]));

    // On re-encode, sequence is omitted.
    let encoded = rasn::der::encode(&decoded).unwrap();
    assert_eq!(encoded, conformant_wire);
}
