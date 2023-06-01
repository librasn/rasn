//! # Enrollment over Secure Transport
//!
//! This module implements [RFC 7030]'s data types in certificate enrollment
//! for clients using Certificate Management over CMS (CMC) messages over a
//! secure transport. This profile, called Enrollment over Secure Transport
//! (EST), describes a simple, yet functional, certificate management protocol
//! targeting Public Key Infrastructure (PKI) clients that need to acquire
//! client certificates and associated Certification Authority (CA) certificates.

use rasn::prelude::*;

/// The identifier of the secret key to be used by the server to encrypt the
/// private key.
pub type AsymmetricDecryptKeyIdentifier = rasn::types::OctetString;
/// The main body of a CSR attribute request.
pub type CsrAttrs = SequenceOf<AttrOrOid>;

/// The OID identifying a `DecryptKeyIdentifier` attribute.
pub const ASYMMETRIC_DECRYPT_KEY_OID: ConstOid =
    Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_AA_ASYMMETRIC_DECRYPT_KEY;

/// Either an OID pointing a specific signature scheme or an attribute for
/// a particular crypto system.
#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum AttrOrOid {
    Oid(ObjectIdentifier),
    Attribute(Attribute),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct Attribute {
    pub r#type: ObjectIdentifier,
    pub values: SetOf<Any>,
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::{borrow::Cow, collections::BTreeSet, string::ToString, vec};

    use super::*;

    #[test]
    fn csr_attributes_encode() {
        let data = vec![
            // ecdsaWithSHA256 (ANSI X9.62 ECDSA algorithm with SHA256)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 10045, 4, 3, 2],
            ))),
            // commonName (X.520 DN component)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![2, 5, 4, 3],
            ))),
            // emailAddress (PKCS #9. Deprecated, use an altName extension instead)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 113549, 1, 9, 1],
            ))),
            // challengePassword (PKCS #9)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 113549, 1, 9, 7],
            ))),
            // ocsp (PKIX)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 3, 6, 1, 5, 5, 7, 48, 1],
            ))),
            // requestClientInfo (Microsoft attribute)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 3, 6, 1, 4, 1, 311, 21, 20],
            ))),
            // 1.2.840.113549.1.1.5 (sha1WithRsaEncryption)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 113549, 1, 1, 5],
            ))),
            AttrOrOid::Attribute(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(Cow::from(vec![
                    1, 3, 6, 1, 5, 5, 7, 48, 1,
                ])),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::PrintableString::try_from(
                            "And me second".to_string(),
                        ).unwrap())
                        .unwrap(),
                    ));
                    b.insert(rasn::types::Any::new(rasn::der::encode(&false).unwrap()));
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::Open::Null).unwrap(),
                    ));
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::VisibleString::try_from("Me first!").unwrap()).unwrap(),
                    ));
                    b
                })(),
            }),
        ];

        let bin = rasn::der::encode(&data).unwrap();
        assert_eq!(
            bin,
            [
                48, 114, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 6, 3, 85, 4, 3, 6, 9, 42, 134, 72,
                134, 247, 13, 1, 9, 1, 6, 9, 42, 134, 72, 134, 247, 13, 1, 9, 7, 6, 8, 43, 6, 1, 5,
                5, 7, 48, 1, 6, 9, 43, 6, 1, 4, 1, 130, 55, 21, 20, 6, 9, 42, 134, 72, 134, 247,
                13, 1, 1, 5, 48, 43, 6, 8, 43, 6, 1, 5, 5, 7, 48, 1, 49, 31, 1, 1, 0, 5, 0, 19, 13,
                65, 110, 100, 32, 109, 101, 32, 115, 101, 99, 111, 110, 100, 26, 9, 77, 101, 32,
                102, 105, 114, 115, 116, 33
            ]
        );
    }

    #[test]
    fn csr_attributes_decode_1() {
        let data = vec![
            // challengePassword (PKCS #9)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 113549, 1, 9, 7],
            ))),
            // ecPublicKey (ANSI X9.62 public key type)
            AttrOrOid::Attribute(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(Cow::from(vec![
                    1, 2, 840, 10045, 2, 1,
                ])),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Any::new(
                        // secp384r1 (SECG (Certicom) named elliptic curve)
                        rasn::der::encode(&rasn::types::ObjectIdentifier::new_unchecked(
                            Cow::from(vec![1, 3, 132, 0, 34]),
                        ))
                        .unwrap(),
                    ));
                    b
                })(),
            }),
            AttrOrOid::Attribute(Attribute {
                // extensionRequest (PKCS #9 via CRMF)
                r#type: rasn::types::ObjectIdentifier::new_unchecked(Cow::from(vec![
                    1, 2, 840, 113549, 1, 9, 14,
                ])),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::ObjectIdentifier::new_unchecked(
                            Cow::from(vec![1, 3, 6, 1, 1, 1, 1, 22]),
                        ))
                        .unwrap(),
                    ));
                    b
                })(),
            }),
            // ecdsaWithSHA384 (ANSI X9.62 ECDSA algorithm with SHA384)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 10045, 4, 3, 3],
            ))),
        ];

        let data_bin = rasn::der::encode(&data).unwrap();
        let txt = "MEEGCSqGSIb3DQEJBzASBgcqhkjOPQIBMQcGBSuBBAAiMBYGCSqGSIb3DQEJDjEJBgcrBgEBAQEWBggqhkjOPQQDAw==";
        let bin = base64::decode(&txt).unwrap();
        assert_eq!(data_bin, bin);
        let decoded_data = rasn::der::decode::<CsrAttrs>(&bin);
        assert!(decoded_data.is_ok());
        let decoded_data = decoded_data.unwrap();
        assert_eq!(decoded_data, data);
    }

    #[test]
    fn csr_attributes_decode_2() {
        let data = vec![
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 3, 6, 1, 1, 1, 1, 22],
            ))),
            // ecPublicKey (ANSI X9.62 public key type)
            AttrOrOid::Attribute(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(Cow::from(vec![2, 999, 1])),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::PrintableString::try_from(
                            "Parse SET as 2.999.1 data".to_string(),
                        ).unwrap())
                        .unwrap(),
                    ));
                    b
                })(),
            }),
            // challengePassword (PKCS #9)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 2, 840, 113549, 1, 9, 7],
            ))),
            AttrOrOid::Attribute(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(Cow::from(vec![2, 999, 2])),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::ObjectIdentifier::new_unchecked(
                            Cow::from(vec![2, 999, 3]),
                        ))
                        .unwrap(),
                    ));
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::ObjectIdentifier::new_unchecked(
                            Cow::from(vec![2, 999, 4]),
                        ))
                        .unwrap(),
                    ));
                    b.insert(rasn::types::Any::new(
                        rasn::der::encode(&rasn::types::PrintableString::try_from(
                            "Parse SET as 2.999.2 data".to_string(),
                        ).unwrap())
                        .unwrap(),
                    ));
                    b
                })(),
            }),
            // brainpoolP384r1 (ECC Brainpool Standard Curves and Curve Generation)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![1, 3, 36, 3, 3, 2, 8, 1, 1, 11],
            ))),
            // sha-384 (NIST Algorithm)
            AttrOrOid::Oid(rasn::types::ObjectIdentifier::new_unchecked(Cow::from(
                vec![2, 16, 840, 1, 101, 3, 4, 2, 2],
            ))),
        ];

        let data_bin = rasn::der::encode(&data).unwrap();
        let txt = "MHwGBysGAQEBARYwIgYDiDcBMRsTGVBhcnNlIFNFVCBhcyAyLjk5OS4xIGRhdGEGCSqGSIb3DQEJBzAsBgOINwIxJQYDiDcDBgOINwQTGVBhcnNlIFNFVCBhcyAyLjk5OS4yIGRhdGEGCSskAwMCCAEBCwYJYIZIAWUDBAIC";
        let bin = base64::decode(&txt).unwrap();
        assert_eq!(data_bin, bin);
        let decoded_data = rasn::der::decode::<CsrAttrs>(&bin);
        assert!(decoded_data.is_ok());
        let decoded_data = decoded_data.unwrap();
        assert_eq!(decoded_data, data);
    }
}
