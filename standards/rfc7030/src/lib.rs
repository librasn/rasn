/*
From RFC7030 (https://datatracker.ietf.org/doc/html/rfc7030):
CsrAttrs ::= SEQUENCE SIZE (0..MAX) OF AttrOrOID
AttrOrOID ::= CHOICE { oid OBJECT IDENTIFIER, attribute Attribute }
Attribute ::= SEQUENCE {
        type  OBJECT IDENTIFIER,
        values SET SIZE(0..MAX) OF ANY }
*/

#[cfg(test)]
type CsrAttrs = Vec<AttrOrOid>;

#[derive(rasn::AsnType, rasn::Decode, rasn::Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum AttrOrOid {
    OID(rasn::types::ObjectIdentifier),
    ATTRIBUTE(Attribute),
}

#[derive(rasn::AsnType, rasn::Decode, rasn::Encode, Debug, PartialEq, Clone)]
struct Attribute {
    r#type: rasn::types::ObjectIdentifier,
    values: rasn::types::SetOf<rasn::types::Open>,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn csr_attributes_encode() {
        let data = vec![
            // ecdsaWithSHA256 (ANSI X9.62 ECDSA algorithm with SHA256)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 10045, 4, 3, 2,
            ])),
            // commonName (X.520 DN component)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                2, 5, 4, 3,
            ])),
            // emailAddress (PKCS #9. Deprecated, use an altName extension instead)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 113549, 1, 9, 1,
            ])),
            // challengePassword (PKCS #9)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 113549, 1, 9, 7,
            ])),
            // ocsp (PKIX)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 3, 6, 1, 5, 5, 7, 48, 1,
            ])),
            // requestClientInfo (Microsoft attribute)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 3, 6, 1, 4, 1, 311, 21, 20,
            ])),
            // 1.2.840.113549.1.1.5 (sha1WithRsaEncryption)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 113549, 1, 1, 5,
            ])),
            AttrOrOid::ATTRIBUTE(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(vec![
                    1, 3, 6, 1, 5, 5, 7, 48, 1,
                ]),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Open::PrintableString(
                        rasn::types::PrintableString::new("And me second".to_string()),
                    ));
                    b.insert(rasn::types::Open::Bool(false));
                    b.insert(rasn::types::Open::Null);
                    b.insert(rasn::types::Open::VisibleString(
                        rasn::types::VisibleString::new("Me first!".to_string()),
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
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 113549, 1, 9, 7,
            ])),
            // ecPublicKey (ANSI X9.62 public key type)
            AttrOrOid::ATTRIBUTE(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(vec![1, 2, 840, 10045, 2, 1]),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Open::ObjectIdentifier(
                        // secp384r1 (SECG (Certicom) named elliptic curve)
                        rasn::types::ObjectIdentifier::new_unchecked(vec![1, 3, 132, 0, 34]),
                    ));
                    b
                })(),
            }),
            AttrOrOid::ATTRIBUTE(Attribute {
                // extensionRequest (PKCS #9 via CRMF)
                r#type: rasn::types::ObjectIdentifier::new_unchecked(vec![
                    1, 2, 840, 113549, 1, 9, 14,
                ]),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Open::ObjectIdentifier(
                        rasn::types::ObjectIdentifier::new_unchecked(vec![1, 3, 6, 1, 1, 1, 1, 22]),
                    ));
                    b
                })(),
            }),
            // ecdsaWithSHA384 (ANSI X9.62 ECDSA algorithm with SHA384)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 10045, 4, 3, 3,
            ])),
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
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 3, 6, 1, 1, 1, 1, 22,
            ])),
            // ecPublicKey (ANSI X9.62 public key type)
            AttrOrOid::ATTRIBUTE(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(vec![2, 999, 1]),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Open::PrintableString(
                        rasn::types::PrintableString::new("Parse SET as 2.999.1 data".to_string()),
                    ));
                    b
                })(),
            }),
            // challengePassword (PKCS #9)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 2, 840, 113549, 1, 9, 7,
            ])),
            AttrOrOid::ATTRIBUTE(Attribute {
                r#type: rasn::types::ObjectIdentifier::new_unchecked(vec![2, 999, 2]),
                values: (|| {
                    let mut b = BTreeSet::new();
                    b.insert(rasn::types::Open::ObjectIdentifier(
                        rasn::types::ObjectIdentifier::new_unchecked(vec![2, 999, 3]),
                    ));
                    b.insert(rasn::types::Open::ObjectIdentifier(
                        rasn::types::ObjectIdentifier::new_unchecked(vec![2, 999, 4]),
                    ));
                    b.insert(rasn::types::Open::PrintableString(
                        rasn::types::PrintableString::new("Parse SET as 2.999.2 data".to_string()),
                    ));
                    b
                })(),
            }),
            // brainpoolP384r1 (ECC Brainpool Standard Curves and Curve Generation)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                1, 3, 36, 3, 3, 2, 8, 1, 1, 11,
            ])),
            // sha-384 (NIST Algorithm)
            AttrOrOid::OID(rasn::types::ObjectIdentifier::new_unchecked(vec![
                2, 16, 840, 1, 101, 3, 4, 2, 2,
            ])),
        ];

        let data_bin = rasn::der::encode(&data).unwrap();
        println!("Encoded data (raw): {:x?}", &data_bin);
        let data_b64_bin = base64::encode(&data_bin);
        println!("Encoded data (b64): {}", &data_b64_bin);

        let txt = "MHwGBysGAQEBARYwIgYDiDcBMRsTGVBhcnNlIFNFVCBhcyAyLjk5OS4xIGRhdGEGCSqGSIb3DQEJBzAsBgOINwIxJQYDiDcDBgOINwQTGVBhcnNlIFNFVCBhcyAyLjk5OS4yIGRhdGEGCSskAwMCCAEBCwYJYIZIAWUDBAIC";
        let bin = base64::decode(&txt).unwrap();
        println!("Encoded data (raw): {:x?}", &bin);
        assert_eq!(data_bin, bin);
        let decoded_data = rasn::der::decode::<CsrAttrs>(&bin);
        println!("Result from decode {:?}", &decoded_data);
        assert!(decoded_data.is_ok());
        let decoded_data = decoded_data.unwrap();
        assert_eq!(decoded_data, data);
    }
}
