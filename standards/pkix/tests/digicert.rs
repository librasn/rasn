use chrono::TimeZone;
use pretty_assertions::assert_eq;
use rasn::types::*;
use rasn_pkix::*;

#[test]
fn it_works() {
    let contents = pem::parse(include_bytes!("data/DigiCertAssuredIDTLSCA.crt.pem")).unwrap();

    let cert: rasn_pkix::Certificate = rasn::der::decode(&contents.contents).unwrap();

    assert_eq!(contents.contents, rasn::der::encode(&cert).unwrap());
}

#[test]
fn extensions() {
    let basic_usage = rasn::der::encode_scope(|encoder| {
        use rasn::Encoder;
        #[derive(AsnType)]
        pub struct Sequence {
            b: bool,
            i: Integer,
        }

        encoder.encode_sequence::<Sequence, _>(Tag::SEQUENCE, |encoder| {
            encoder.encode_bool(Tag::BOOL, true)?;
            encoder.encode_integer(Tag::INTEGER, <_>::default(), &0u32.into())?;
            Ok(())
        })?;

        Ok(())
    })
    .unwrap();

    let extension = Extension {
        extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 19][..]).into()),
        critical: true,
        extn_value: basic_usage.into(),
    };

    let extensions: Extensions = vec![extension.clone()];

    let expected = &[
        0xA3, 0x16, 0x30, 0x14, 0x30, 0x12, 0x06, 0x03, 0x55, 0x1D, 0x13, 0x01, 0x01, 0xFF, 0x04,
        0x08, 0x30, 0x06, 0x01, 0x01, 0xFF, 0x02, 0x01, 0x00,
    ];

    let expected_extension = &[
        0x30, 0x12, 0x06, 0x03, 0x55, 0x1D, 0x13, 0x01, 0x01, 0xFF, 0x04, 0x08, 0x30, 0x06, 0x01,
        0x01, 0xFF, 0x02, 0x01, 0x00,
    ];

    struct C0;
    impl AsnType for C0 {
        const TAG: Tag = Tag::new(Class::Context, 3);
    }

    assert_eq!(
        &*expected_extension,
        &*rasn::der::encode(&extension).unwrap()
    );
    assert_eq!(
        &*expected,
        &*rasn::der::encode(&Explicit::<C0, _>::new(extensions)).unwrap()
    );
}

#[test]
fn lets_encrypt_x3() {
    let signature = AlgorithmIdentifier {
        algorithm: ObjectIdentifier::new_unchecked((&[1, 2, 840, 113549, 1, 1, 11][..]).into()),
        parameters: Some(Any::new(rasn::der::encode(&()).unwrap())),
    };

    let cert = Certificate {
        tbs_certificate: TbsCertificate {
            version: Version::V3,
            serial_number: 13298795840390663119752826058995181320u128.into(),
            signature: signature.clone(),
            issuer: Name::RdnSequence(vec![
                {
                    let mut set = rasn::types::SetOf::new();
                    set.insert(AttributeTypeAndValue {
                        r#type: ObjectIdentifier::new_unchecked((&[2, 5, 4, 10][..]).into()),
                        value: Any::new(
                            rasn::der::encode(&PrintableString::try_from(String::from(
                                "Digital Signature Trust Co.",
                            )).unwrap())
                            .unwrap(),
                        ),
                    });

                    set
                },
                {
                    let mut set = rasn::types::SetOf::new();
                    set.insert(AttributeTypeAndValue {
                        r#type: ObjectIdentifier::new_unchecked((&[2, 5, 4, 3][..]).into()),
                        value: Any::new(
                            rasn::der::encode(&PrintableString::try_from(String::from(
                                "DST Root CA X3",
                            )).unwrap())
                            .unwrap(),
                        ),
                    });
                    set
                },
            ]),
            validity: Validity {
                not_before: Time::Utc(chrono::Utc.ymd(2016, 03, 17).and_hms(16, 40, 46)),
                not_after: Time::Utc(chrono::Utc.ymd(2021, 03, 17).and_hms(16, 40, 46)),
            },
            subject: Name::RdnSequence(vec![
                {
                    let mut set = rasn::types::SetOf::new();
                    set.insert(AttributeTypeAndValue {
                        r#type: ObjectIdentifier::new_unchecked((&[2, 5, 4, 6][..]).into()),
                        value: Any::new(
                            rasn::der::encode(&PrintableString::try_from(String::from("US")).unwrap()).unwrap(),
                        ),
                    });
                    set
                },
                {
                    let mut set = rasn::types::SetOf::new();
                    set.insert(AttributeTypeAndValue {
                        r#type: ObjectIdentifier::new_unchecked((&[2, 5, 4, 10][..]).into()),
                        value: Any::new(
                            rasn::der::encode(&PrintableString::try_from(String::from(
                                "Let's Encrypt",
                            )).unwrap())
                            .unwrap(),
                        ),
                    });
                    set
                },
                {
                    let mut set = rasn::types::SetOf::new();
                    set.insert(AttributeTypeAndValue {
                        r#type: ObjectIdentifier::new_unchecked((&[2, 5, 4, 3][..]).into()),
                        value: Any::new(
                            rasn::der::encode(&PrintableString::try_from(String::from(
                                "Let's Encrypt Authority X3",
                            )).unwrap())
                            .unwrap(),
                        ),
                    });
                    set
                },
            ]),
            subject_public_key_info: SubjectPublicKeyInfo {
                algorithm: AlgorithmIdentifier {
                    algorithm: ObjectIdentifier::new_unchecked(
                        (&[1, 2, 840, 113549, 1, 1, 1][..]).into(),
                    ),
                    parameters: Some(Any::new(rasn::der::encode(&()).unwrap())),
                },
                subject_public_key: BitString::from_slice(&[
                    0x30, 0x82, 0x1, 0xA, 0x2, 0x82, 0x1, 0x1, 0x00, 0x9c, 0xd3, 0x0c, 0xf0, 0x5a,
                    0xe5, 0x2e, 0x47, 0xb7, 0x72, 0x5d, 0x37, 0x83, 0xb3, 0x68, 0x63, 0x30, 0xea,
                    0xd7, 0x35, 0x26, 0x19, 0x25, 0xe1, 0xbd, 0xbe, 0x35, 0xf1, 0x70, 0x92, 0x2f,
                    0xb7, 0xb8, 0x4b, 0x41, 0x05, 0xab, 0xa9, 0x9e, 0x35, 0x08, 0x58, 0xec, 0xb1,
                    0x2a, 0xc4, 0x68, 0x87, 0x0b, 0xa3, 0xe3, 0x75, 0xe4, 0xe6, 0xf3, 0xa7, 0x62,
                    0x71, 0xba, 0x79, 0x81, 0x60, 0x1f, 0xd7, 0x91, 0x9a, 0x9f, 0xf3, 0xd0, 0x78,
                    0x67, 0x71, 0xc8, 0x69, 0x0e, 0x95, 0x91, 0xcf, 0xfe, 0xe6, 0x99, 0xe9, 0x60,
                    0x3c, 0x48, 0xcc, 0x7e, 0xca, 0x4d, 0x77, 0x12, 0x24, 0x9d, 0x47, 0x1b, 0x5a,
                    0xeb, 0xb9, 0xec, 0x1e, 0x37, 0x00, 0x1c, 0x9c, 0xac, 0x7b, 0xa7, 0x05, 0xea,
                    0xce, 0x4a, 0xeb, 0xbd, 0x41, 0xe5, 0x36, 0x98, 0xb9, 0xcb, 0xfd, 0x6d, 0x3c,
                    0x96, 0x68, 0xdf, 0x23, 0x2a, 0x42, 0x90, 0x0c, 0x86, 0x74, 0x67, 0xc8, 0x7f,
                    0xa5, 0x9a, 0xb8, 0x52, 0x61, 0x14, 0x13, 0x3f, 0x65, 0xe9, 0x82, 0x87, 0xcb,
                    0xdb, 0xfa, 0x0e, 0x56, 0xf6, 0x86, 0x89, 0xf3, 0x85, 0x3f, 0x97, 0x86, 0xaf,
                    0xb0, 0xdc, 0x1a, 0xef, 0x6b, 0x0d, 0x95, 0x16, 0x7d, 0xc4, 0x2b, 0xa0, 0x65,
                    0xb2, 0x99, 0x04, 0x36, 0x75, 0x80, 0x6b, 0xac, 0x4a, 0xf3, 0x1b, 0x90, 0x49,
                    0x78, 0x2f, 0xa2, 0x96, 0x4f, 0x2a, 0x20, 0x25, 0x29, 0x04, 0xc6, 0x74, 0xc0,
                    0xd0, 0x31, 0xcd, 0x8f, 0x31, 0x38, 0x95, 0x16, 0xba, 0xa8, 0x33, 0xb8, 0x43,
                    0xf1, 0xb1, 0x1f, 0xc3, 0x30, 0x7f, 0xa2, 0x79, 0x31, 0x13, 0x3d, 0x2d, 0x36,
                    0xf8, 0xe3, 0xfc, 0xf2, 0x33, 0x6a, 0xb9, 0x39, 0x31, 0xc5, 0xaf, 0xc4, 0x8d,
                    0x0d, 0x1d, 0x64, 0x16, 0x33, 0xaa, 0xfa, 0x84, 0x29, 0xb6, 0xd4, 0x0b, 0xc0,
                    0xd8, 0x7d, 0xc3, 0x93, 0x02, 0x03, 0x01, 0x00, 0x01,
                ]),
            },
            issuer_unique_id: None,
            subject_unique_id: None,
            extensions: Some(vec![
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 19][..]).into()),
                    critical: true,
                    extn_value: rasn::der::encode(&BasicConstraints {
                        ca: true,
                        path_len_constraint: Some(0u8.into()),
                    })
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 15][..]).into()),
                    critical: true,
                    extn_value: rasn::der::encode(
                        &bitvec::bitvec![u8, bitvec::prelude::Msb0; 1, 0, 0, 0, 0, 1, 1],
                    )
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked(
                        (&[1, 3, 6, 1, 5, 5, 7, 1, 1][..]).into(),
                    ),
                    critical: false,
                    extn_value: rasn::der::encode(&vec![
                        AccessDescription {
                            access_method: ObjectIdentifier::new_unchecked(
                                (&[1, 3, 6, 1, 5, 5, 7, 48, 1][..]).into(),
                            ),
                            access_location: GeneralName::Uri(
                                String::from("http://isrg.trustid.ocsp.identrust.com").try_into().unwrap(),
                            ),
                        },
                        AccessDescription {
                            access_method: ObjectIdentifier::new_unchecked(
                                (&[1, 3, 6, 1, 5, 5, 7, 48, 2][..]).into(),
                            ),
                            access_location: GeneralName::Uri(
                                String::from("http://apps.identrust.com/roots/dstrootcax3.p7c")
                                    .try_into().unwrap(),
                            ),
                        },
                    ])
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 35][..]).into()),
                    critical: false,
                    extn_value: rasn::der::encode(&AuthorityKeyIdentifier {
                        key_identifier: Some(OctetString::from(
                            &[
                                0xC4, 0xA7, 0xB1, 0xA4, 0x7B, 0x2C, 0x71, 0xFA, 0xDB, 0xE1, 0x4B,
                                0x90, 0x75, 0xFF, 0xC4, 0x15, 0x60, 0x85, 0x89, 0x10,
                            ][..],
                        )),
                        ..<_>::default()
                    })
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 32][..]).into()),
                    critical: false,
                    extn_value: rasn::der::encode(&vec![
                        PolicyInformation {
                            policy_identifier: ObjectIdentifier::new_unchecked(
                                (&[2, 23, 140, 1, 2, 1][..]).into(),
                            ),
                            policy_qualifiers: None,
                        },
                        PolicyInformation {
                            policy_identifier: ObjectIdentifier::new_unchecked(
                                (&[1, 3, 6, 1, 4, 1, 44947, 1, 1, 1][..]).into(),
                            ),
                            policy_qualifiers: Some(vec![PolicyQualifierInfo {
                                id: ObjectIdentifier::new_unchecked(
                                    (&[1, 3, 6, 1, 5, 5, 7, 2, 1][..]).into(),
                                ),
                                qualifier: Any::new(
                                    rasn::der::encode(&Ia5String::try_from(String::from(
                                        "http://cps.root-x1.letsencrypt.org",
                                    )).unwrap())
                                    .unwrap(),
                                ),
                            }]),
                        },
                    ])
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 31][..]).into()),
                    critical: false,
                    extn_value: rasn::der::encode(&vec![DistributionPoint {
                        distribution_point: Some(DistributionPointName::FullName(vec![
                            GeneralName::Uri(
                                String::from("http://crl.identrust.com/DSTROOTCAX3CRL.crl").try_into().unwrap(),
                            ),
                        ])),
                        ..<_>::default()
                    }])
                    .unwrap()
                    .into(),
                },
                Extension {
                    extn_id: ObjectIdentifier::new_unchecked((&[2, 5, 29, 14][..]).into()),
                    critical: false,
                    extn_value: rasn::der::encode(&SubjectKeyIdentifier::from(
                        &[
                            0xA8, 0x4A, 0x6A, 0x63, 0x04, 0x7D, 0xDD, 0xBA, 0xE6, 0xD1, 0x39, 0xB7,
                            0xA6, 0x45, 0x65, 0xEF, 0xF3, 0xA8, 0xEC, 0xA1,
                        ][..],
                    ))
                    .unwrap()
                    .into(),
                },
            ]),
        },
        signature_algorithm: signature,
        signature_value: BitString::from_slice(
            &[
                0xdd, 0x33, 0xd7, 0x11, 0xf3, 0x63, 0x58, 0x38, 0xdd, 0x18, 0x15, 0xfb, 0x09, 0x55,
                0xbe, 0x76, 0x56, 0xb9, 0x70, 0x48, 0xa5, 0x69, 0x47, 0x27, 0x7b, 0xc2, 0x24, 0x08,
                0x92, 0xf1, 0x5a, 0x1f, 0x4a, 0x12, 0x29, 0x37, 0x24, 0x74, 0x51, 0x1c, 0x62, 0x68,
                0xb8, 0xcd, 0x95, 0x70, 0x67, 0xe5, 0xf7, 0xa4, 0xbc, 0x4e, 0x28, 0x51, 0xcd, 0x9b,
                0xe8, 0xae, 0x87, 0x9d, 0xea, 0xd8, 0xba, 0x5a, 0xa1, 0x01, 0x9a, 0xdc, 0xf0, 0xdd,
                0x6a, 0x1d, 0x6a, 0xd8, 0x3e, 0x57, 0x23, 0x9e, 0xa6, 0x1e, 0x04, 0x62, 0x9a, 0xff,
                0xd7, 0x05, 0xca, 0xb7, 0x1f, 0x3f, 0xc0, 0x0a, 0x48, 0xbc, 0x94, 0xb0, 0xb6, 0x65,
                0x62, 0xe0, 0xc1, 0x54, 0xe5, 0xa3, 0x2a, 0xad, 0x20, 0xc4, 0xe9, 0xe6, 0xbb, 0xdc,
                0xc8, 0xf6, 0xb5, 0xc3, 0x32, 0xa3, 0x98, 0xcc, 0x77, 0xa8, 0xe6, 0x79, 0x65, 0x07,
                0x2b, 0xcb, 0x28, 0xfe, 0x3a, 0x16, 0x52, 0x81, 0xce, 0x52, 0x0c, 0x2e, 0x5f, 0x83,
                0xe8, 0xd5, 0x06, 0x33, 0xfb, 0x77, 0x6c, 0xce, 0x40, 0xea, 0x32, 0x9e, 0x1f, 0x92,
                0x5c, 0x41, 0xc1, 0x74, 0x6c, 0x5b, 0x5d, 0x0a, 0x5f, 0x33, 0xcc, 0x4d, 0x9f, 0xac,
                0x38, 0xf0, 0x2f, 0x7b, 0x2c, 0x62, 0x9d, 0xd9, 0xa3, 0x91, 0x6f, 0x25, 0x1b, 0x2f,
                0x90, 0xb1, 0x19, 0x46, 0x3d, 0xf6, 0x7e, 0x1b, 0xa6, 0x7a, 0x87, 0xb9, 0xa3, 0x7a,
                0x6d, 0x18, 0xfa, 0x25, 0xa5, 0x91, 0x87, 0x15, 0xe0, 0xf2, 0x16, 0x2f, 0x58, 0xb0,
                0x06, 0x2f, 0x2c, 0x68, 0x26, 0xc6, 0x4b, 0x98, 0xcd, 0xda, 0x9f, 0x0c, 0xf9, 0x7f,
                0x90, 0xed, 0x43, 0x4a, 0x12, 0x44, 0x4e, 0x6f, 0x73, 0x7a, 0x28, 0xea, 0xa4, 0xaa,
                0x6e, 0x7b, 0x4c, 0x7d, 0x87, 0xdd, 0xe0, 0xc9, 0x02, 0x44, 0xa7, 0x87, 0xaf, 0xc3,
                0x34, 0x5b, 0xb4, 0x42,
            ][..],
        ),
    };

    let original_data: &[u8] = include_bytes!("data/letsencrypt-x3.crt");
    let original = rasn::der::decode::<Certificate>(&original_data).unwrap();

    assert_eq!(
        original.tbs_certificate.version,
        cert.tbs_certificate.version
    );
    assert_eq!(
        original.tbs_certificate.serial_number,
        cert.tbs_certificate.serial_number
    );
    assert_eq!(original.tbs_certificate.issuer, cert.tbs_certificate.issuer);
    assert_eq!(
        original.tbs_certificate.validity,
        cert.tbs_certificate.validity
    );
    assert_eq!(
        original.tbs_certificate.subject,
        cert.tbs_certificate.subject
    );
    assert_eq!(
        original.tbs_certificate.subject_public_key_info,
        cert.tbs_certificate.subject_public_key_info
    );
    assert_eq!(
        original.tbs_certificate.issuer_unique_id,
        cert.tbs_certificate.issuer_unique_id
    );
    assert_eq!(
        original.tbs_certificate.subject_unique_id,
        cert.tbs_certificate.subject_unique_id
    );
    macro_rules! assert_extensions {
        ($($extension:ty),+ $(,)?) => {
            let mut original_iter = original.tbs_certificate.extensions.as_ref().unwrap().into_iter();
            let mut cert_iter = cert.tbs_certificate.extensions.as_ref().unwrap().into_iter();

            $({
                let print_error = |error| {
                    panic!("{}: {}", stringify!($extension), error)
                };

                let original = original_iter.next().unwrap();
                let cert = cert_iter.next().unwrap();

                assert_eq!(rasn::der::decode::<$extension>(&original.extn_value).unwrap_or_else(print_error), rasn::der::decode::<$extension>(&cert.extn_value).unwrap_or_else(print_error));
                assert_eq!(original, cert);
            })+
        }
    }

    assert_extensions! {
        BasicConstraints,
        KeyUsage,
        AuthorityInfoAccessSyntax,
        AuthorityKeyIdentifier,
        CertificatePolicies,
        CrlDistributionPoints,
        SubjectKeyIdentifier,
    }

    assert_eq!(original.signature_algorithm, cert.signature_algorithm);
    assert_eq!(original.signature_value, cert.signature_value);

    assert_eq!(original_data, rasn::der::encode(&cert).unwrap(),);
}
