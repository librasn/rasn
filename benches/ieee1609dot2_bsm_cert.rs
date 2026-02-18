use criterion::{Criterion, criterion_group, criterion_main};
use rasn::prelude::*;
use rasn_its::ieee1609dot2::base_types::*;
use rasn_its::ieee1609dot2::*;

pub fn build_sample() -> Ieee1609Dot2Data {
    Ieee1609Dot2Data::builder()
        .protocol_version(3)
        .content(Ieee1609Dot2Content::SignedData(Box::new(
            SignedData::builder()
                .hash_id(HashAlgorithm::Sha256)
                .tbs_data(
                    ToBeSignedData::builder()
                        .payload(
                            SignedDataPayload::builder()
                                .data(
                                    Ieee1609Dot2Data::builder()
                                        .protocol_version(3)
                                        .content(Ieee1609Dot2Content::UnsecuredData(Opaque(
                                            "This is a BSM\r\n".as_bytes().into(),
                                        )))
                                        .build(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .header_info(
                            HeaderInfo::builder()
                                .psid(Integer::from(32).into())
                                .generation_time(1_230_066_625_199_609_624.into())
                                .build(),
                        )
                        .build(),
                )
                .signer(SignerIdentifier::Certificate(
                    vec![Certificate::from(
                        ImplicitCertificate::new(
                            CertificateBase::builder()
                                .version(3)
                                .r#type(CertificateType::Implicit)
                                .issuer(IssuerIdentifier::Sha256AndDigest(HashedId8(
                                    "!\"#$%&'(".as_bytes().try_into().unwrap(),
                                )))
                                .to_be_signed(
                                    ToBeSignedCertificate::builder()
                                        .id(CertificateId::LinkageData(
                                            LinkageData::builder()
                                                .i_cert(IValue::from(100))
                                                .linkage_value(LinkageValue(
                                                    FixedOctetString::try_from(
                                                        b"123456789".as_slice(),
                                                    )
                                                    .unwrap(),
                                                ))
                                                .group_linkage_value(
                                                    GroupLinkageValue::builder()
                                                        .j_value(
                                                            b"ABCD".as_slice().try_into().unwrap(),
                                                        )
                                                        .value(
                                                            b"QRSTUVWXY"
                                                                .as_slice()
                                                                .try_into()
                                                                .unwrap(),
                                                        )
                                                        .build(),
                                                )
                                                .build(),
                                        ))
                                        .craca_id(HashedId3(b"abc".as_slice().try_into().unwrap()))
                                        .crl_series(CrlSeries::from(70))
                                        .validity_period(
                                            ValidityPeriod::builder()
                                                .start(81_828_384.into())
                                                .duration(Duration::Hours(169))
                                                .build(),
                                        )
                                        .region(GeographicRegion::IdentifiedRegion(
                                            vec![
                                                IdentifiedRegion::CountryOnly(UnCountryId::from(
                                                    124,
                                                )),
                                                IdentifiedRegion::CountryOnly(UnCountryId::from(
                                                    484,
                                                )),
                                                IdentifiedRegion::CountryOnly(UnCountryId::from(
                                                    840,
                                                )),
                                            ]
                                            .into(),
                                        ))
                                        .app_permissions(
                                            vec![
                                                PsidSsp {
                                                    psid: Integer::from(32).into(),
                                                    ssp: None,
                                                },
                                                PsidSsp {
                                                    psid: Integer::from(38).into(),
                                                    ssp: None,
                                                },
                                            ]
                                            .into(),
                                        )
                                        .verify_key_indicator(
                                            VerificationKeyIndicator::ReconstructionValue(
                                                EccP256CurvePoint::CompressedY0(
                                                    FixedOctetString::from([
                                                        0x91u8, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
                                                        0x98, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96,
                                                        0x97, 0x98, 0x91, 0x92, 0x93, 0x94, 0x95,
                                                        0x96, 0x97, 0x98, 0x91, 0x92, 0x93, 0x94,
                                                        0x95, 0x96, 0x97, 0x98,
                                                    ]),
                                                ),
                                            ),
                                        )
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .unwrap(),
                    )]
                    .into(),
                ))
                .signature(Signature::EcdsaNistP256(
                    EcdsaP256Signature::builder()
                        .r_sig(EccP256CurvePoint::CompressedY0(
                            b"12345678123456781234567812345678"
                                .as_slice()
                                .try_into()
                                .unwrap(),
                        ))
                        .s_sig(
                            b"ABCDEFGHABCDEFGHABCDEFGHABCDEFGH"
                                .as_slice()
                                .try_into()
                                .unwrap(),
                        )
                        .build(),
                ))
                .build(),
        )))
        .build()
}

fn oer_enc_dec(c: &mut Criterion) {
    let cert = build_sample();
    let mut buffer = Vec::<u8>::with_capacity(core::mem::size_of::<Ieee1609Dot2Data>());

    c.bench_function(
        "RASN/ encode/decode OER ieee1609dot2 - bsm with certificate",
        |b| {
            b.iter(|| {
                rasn::coer::encode_buf(&cert, &mut buffer).unwrap();
                rasn::coer::decode::<Ieee1609Dot2Data>(&buffer).unwrap();
                buffer.clear();
            })
        },
    );
}
// Ieee1609Dot2Data
// FROM https://cpoc.jrc.ec.europa.eu/ECTL.html
fn ectl_list_enc_dec(c: &mut Criterion) {
    let tlm_data: &[u8] = include_bytes!("../standards/its/tests/data/CE4CF6C19BFED720.oer");
    let mut buffer = Vec::<u8>::with_capacity(tlm_data.len());

    c.bench_function(
        "RASN/ decode/encode OER ieee1609dot2 - ECTL - European Certificate Trust List",
        |b| {
            b.iter(|| {
                let decoded = rasn::coer::decode::<Ieee1609Dot2Data>(tlm_data).unwrap();
                rasn::coer::encode_buf(&decoded, &mut buffer).unwrap();
                buffer.clear();
            })
        },
    );
}

criterion_group!(benches, oer_enc_dec, ectl_list_enc_dec);
criterion_main!(benches);
