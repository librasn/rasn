/// ASN.1 definitions for ETSI TS 103 097 extension module
extern crate alloc;
use alloc::string::ToString;
pub mod extension_module;
use crate::{
    delegate,
    ieee1609dot2::{
        Certificate, CertificateId, HashedData, Ieee1609Dot2Content, Ieee1609Dot2Data,
        RecipientInfo, SignerIdentifier,
    },
};

// use rasn::error::InnerSubtypeConstraintError;
use rasn::prelude::*;

pub const ETSI_TS103097_MODULE_OID: &Oid = Oid::const_new(&[0, 4, 0, 5, 5, 103_097, 1, 3, 1]);

/// ETSI TS 103 097 certificate
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097Certificate(Certificate);

impl core::ops::Deref for EtsiTs103097Certificate {
    type Target = Certificate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Certificate> for EtsiTs103097Certificate {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(cert: Certificate) -> Result<Self, Self::Error> {
        let etsi_cert = EtsiTs103097Certificate(cert);
        etsi_cert.validate_components()
    }
}

impl InnerSubtypeConstraint for EtsiTs103097Certificate {
    fn validate_and_decode_containing(
        self,
        _: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        let tbs = &self.0.to_be_signed;
        let id = &tbs.id;

        if !matches!(id, CertificateId::Name(_) | CertificateId::None(_)) {
            return Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                    type_name: "EtsiTs103097Certificate::toBeSignedCertificate::CertificateId",
                    component_name: "CertificateId",
                    details: "Only CertificateId::Name or CertificateId::None is permitted"
                        .to_string(),
                },
            );
        }
        if tbs.cert_request_permissions.is_some() {
            return Err(
                rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                    type_name: "EtsiTs103097Certificate::toBeSignedCertificate",
                    component_name: "certRequestPermissions",
                },
            );
        }
        if tbs.can_request_rollover.is_some() {
            return Err(
                rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                    type_name: "EtsiTs103097Certificate::toBeSignedCertificate",
                    component_name: "canRequestRollover",
                },
            );
        }
        Ok(self)
    }
}

/// ETSI TS 103 097 data
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097Data(Ieee1609Dot2Data);

impl InnerSubtypeConstraint for EtsiTs103097Data {
    fn validate_and_decode_containing(
        self,
        _: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.0.content {
            Ieee1609Dot2Content::SignedData(signed_data) => {
                let tbs_data = &signed_data.tbs_data;
                let header_info = &tbs_data.header_info;
                if header_info.generation_time.is_none() {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::MissingRequiredComponent {
                            type_name: "EtsiTs103097Data::Content::SignedData::TbsData::HeaderInfo",
                            components: &["generationTime"],
                        },
                    );
                }
                if header_info.p2pcd_learning_request.is_some() {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                            type_name: "EtsiTs103097Data::Content::SignedData::TbsData::HeaderInfo",
                            component_name: "p2pcdLearningRequest",
                        },
                    );
                }
                if header_info.missing_crl_identifier.is_some() {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                            type_name: "EtsiTs103097Data::Content::SignedData::TbsData::HeaderInfo",
                            component_name: "missingCrlIdentifier",
                        },
                    );
                }
                let signer = &signed_data.signer;
                match signer {
                    SignerIdentifier::Certificate(cert) => {
                        if cert.len() != 1 {
                            return Err(
                                rasn::error::InnerSubtypeConstraintError::InvalidComponentSize {
                                    type_name: "EtsiTs103097Data::Content::SignedData::Signer",
                                    component_name: "certificate",
                                    details: "Exactly one certificate is required".to_string(),
                                },
                            );
                        }
                    }
                    _ => {
                        return Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                type_name: "EtsiTs103097Data::Content::SignedData::Signer",
                                component_name: "signer",
                                details: "Only Certificate variant is allowed".to_string(),
                            },
                        )
                    }
                }
            }
            Ieee1609Dot2Content::EncryptedData(encrypted_data) => {
                let recipients = &encrypted_data.recipients;
                for (index, recipient) in recipients.iter().enumerate() {
                    if matches!(
                        recipient,
                        RecipientInfo::PskRecipInfo(_)
                            | RecipientInfo::SymmRecipInfo(_)
                            | RecipientInfo::RekRecipInfo(_)
                    ) {
                        return Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                type_name: "EtsiTs103097Data::Content::EncryptedData::Recipients",
                                component_name: "RecipientInfo",
                                details:
                                    format!("PskRecipInfo, SymmRecipInfo or RekRecipInfo is not allowed, occured in index {}", index),
                            },
                        );
                    }
                }
            }
            _ => {
                return Err(
                    rasn::error::InnerSubtypeConstraintError::MissingRequiredComponent {
                        type_name: "EtsiTs103097Data::Content",
                        components: &["signedData", "encryptedData"],
                    },
                );
            }
        };

        Ok(self)
    }
}

delegate!(Ieee1609Dot2Data, EtsiTs103097Data);

/// ETSI TS 103 097 data - unsecured
#[derive(Debug, Clone, AsnType, PartialEq, Eq)]
pub struct EtsiTs103097DataUnsecured<T: rasn::Decode> {
    data: EtsiTs103097Data,
    __phantom: core::marker::PhantomData<T>,
}

impl Encode for EtsiTs103097DataUnsecured<HashedData> {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        self.data
            .encode_with_tag_and_constraints(encoder, tag, constraints, identifier)
    }
}
impl Decode for EtsiTs103097DataUnsecured<HashedData> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        let data = EtsiTs103097Data::decode_with_tag_and_constraints(decoder, tag, constraints)?;
        Ok(EtsiTs103097DataUnsecured {
            data,
            __phantom: core::marker::PhantomData,
        })
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataUnsecured<T> {
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.data.content {
            Ieee1609Dot2Content::UnsecuredData(unsecured) => {
                if let Some(codec) = decode_containing_with {
                    let decoded = codec.decode_from_binary::<T>(unsecured);
                    match decoded {
                        Ok(_) => Ok(self),
                        Err(_) => Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                type_name: "EtsiTs103097DataUnsecured::Content",
                                component_name: "unsecuredData",
                                details:
                                    "Invalid value for unsecuredData that is constrained by generic T"
                                        .to_string(),
                            },
                        ),
                    }
                } else {
                    Ok(self)
                }
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                    type_name: "EtsiTs103097DataUnsecured::Content",
                    component_name: "unsecuredData",
                    details: "Only UnsecuredData is allowed".to_string(),
                },
            ),
        }
    }
}

// EtsiTs103097Data-Signed {ToBeSignedDataContent} ::= EtsiTs103097Data (WITH COMPONENTS {...,
//     content (WITH COMPONENTS {
//       signedData (WITH COMPONENTS {...,
//         tbsData (WITH COMPONENTS {
//           payload (WITH COMPONENTS {
//             data (WITH COMPONENTS {...,
//               content (WITH COMPONENTS {
//                 unsecuredData (CONTAINING ToBeSignedDataContent)
//               })
//             }) PRESENT
//           })
//         })
//       })
//     })
//   })

/// ETSI TS 103 097 data - signed
#[derive(Debug, Clone, AsnType, PartialEq, Eq)]
pub struct EtsiTs103097DataSigned<T: rasn::Decode> {
    data: EtsiTs103097Data,
    __phantom: core::marker::PhantomData<T>,
}

impl<T: rasn::Decode> Encode for EtsiTs103097DataSigned<T> {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        self.data
            .encode_with_tag_and_constraints(encoder, tag, constraints, identifier)
    }
}

impl<T: rasn::Decode> Decode for EtsiTs103097DataSigned<T> {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        let data = EtsiTs103097Data::decode_with_tag_and_constraints(decoder, tag, constraints)?;
        Ok(EtsiTs103097DataSigned {
            data,
            __phantom: core::marker::PhantomData,
        })
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataSigned<T> {
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.data.content {
            Ieee1609Dot2Content::SignedData(signed_data) => {
                let tbs_payload = &signed_data.tbs_data.payload.data;
                match tbs_payload {
                        Some(data) => {
                            match &data.content {
                                Ieee1609Dot2Content::UnsecuredData(unsecured) => {
                                    if let Some(codec) = decode_containing_with {
                                        let decoded = codec.decode_from_binary::<T>(unsecured);
                                        match decoded {
                                            Ok(_) => Ok(self),
                                            Err(_) => Err(
                                                rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                                    type_name: "EtsiTs103097DataSigned::Content::SignedData::TbsData::Payload::Data::Content",
                                                    component_name: "unsecuredData",
                                                    details: "Invalid value for unsecuredData that is constrained by generic T".to_string(),
                                                }
                                            ),
                                        }
                                    } else {
                                        Ok(self)
                                    }
                                }
                                _ => Err(
                                    rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                                        type_name: "EtsiTs103097DataSigned::Content::SignedData::TbsData::Payload::Data::Content",
                                        component_name: "Ieee1609Dot2Content",
                                        details: "Only UnsecuredData is allowed".to_string(),
                                    }
                                ),
                            }
                        }
                        _ => Err(
                            rasn::error::InnerSubtypeConstraintError::MissingRequiredComponent {
                                type_name: "EtsiTs103097DataSigned::Content::SignedData::TbsData::Payload",
                                components: &["Data payload field must be present."],
                            }
                        ),
                    }
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    type_name: "EtsiTs103097DataSigned::Content",
                    component_name: "Ieee1609Dot2Content",
                    details: "Only SignedData is allowed".to_string(),
                },
            ),
        }
    }
}
