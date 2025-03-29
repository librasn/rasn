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
                    component_path: "EtsiTs103097Certificate.toBeSignedCertificate.certificateId",
                    component_name: "CertificateId",
                    details: "Only CertificateId::Name or CertificateId::None is permitted"
                        .to_string(),
                },
            );
        }
        if tbs.cert_request_permissions.is_some() {
            return Err(
                rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                    component_path: "EtsiTs103097Certificate.toBeSignedCertificate",
                    component_name: "certRequestPermissions",
                },
            );
        }
        if tbs.can_request_rollover.is_some() {
            return Err(
                rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                    component_path: "EtsiTs103097Certificate.toBeSignedCertificate",
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
                            component_path:
                                "EtsiTs103097Data.content.signedData.tbsData.headerInfo",
                            components: &["generationTime"],
                        },
                    );
                }
                if header_info.p2pcd_learning_request.is_some() {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                            component_path:
                                "EtsiTs103097Data.content.signedData.tbsData.headerInfo",
                            component_name: "p2pcdLearningRequest",
                        },
                    );
                }
                if header_info.missing_crl_identifier.is_some() {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::UnexpectedComponentPresent {
                            component_path:
                                "EtsiTs103097Data.content.signedData.tbsData.headerInfo",
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
                                    component_path: "EtsiTs103097Data.content.signedData.signer",
                                    component_name: "certificate",
                                    details: "Exactly one certificate is required".to_string(),
                                },
                            );
                        }
                    }
                    _ => {
                        return Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                component_path: "EtsiTs103097Data.content.signedData.signer",
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
                                component_path: "EtsiTs103097Data.content.encryptedData.recipients",
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
                        component_path: "EtsiTs103097Data.content",
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
#[derive(Debug, Clone, Encode, Decode, AsnType, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataUnsecured<T: rasn::Decode>(
    EtsiTs103097Data,
    core::marker::PhantomData<T>,
);

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataUnsecured<T> {
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.0.content {
            Ieee1609Dot2Content::UnsecuredData(unsecured) => {
                if let Some(codec) = decode_containing_with {
                    let decoded = codec.decode_from_binary::<T>(unsecured);
                    match decoded {
                        Ok(_) => Ok(self),
                        Err(_) => Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                component_path: "EtsiTs103097Data.content",
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
                    component_path: "EtsiTs103097DataUnsecured.content",
                    component_name: "unsecuredData",
                    details: "Only UnsecuredData is allowed".to_string(),
                },
            ),
        }
    }
}

/// ETSI TS 103 097 data - signed
#[derive(Debug, Clone, AsnType, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSigned<T: rasn::Decode>(EtsiTs103097Data, core::marker::PhantomData<T>);

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataSigned<T> {
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.0.content {
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
                                                    component_path: "EtsiTs103097Data.content.signedData.tbsData.payload.data.content",
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
                                        component_path: "EtsiTs103097Data.content.signedData.tbsData.payload.data.content",
                                        component_type: "Ieee1609Dot2Content",
                                        details: "Only UnsecuredData is allowed".to_string(),
                                    }
                                ),
                            }
                        }
                        _ => Err(
                            rasn::error::InnerSubtypeConstraintError::MissingRequiredComponent {
                                component_path: "EtsiTs103097DataSigned.content.signedData.tbsData.payload",
                                components: &["Data payload field must be present."],
                            }
                        ),
                    }
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    component_path: "EtsiTs103097DataSigned.content",
                    component_type: "Ieee1609Dot2Content",
                    details: "Only SignedData is allowed".to_string(),
                },
            ),
        }
    }
}

/// ETSI TS 103 097 data - signed
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSignedExternalPayload<T: rasn::Decode>(
    EtsiTs103097Data,
    core::marker::PhantomData<T>,
);
// impl<T: rasn::Decode + rasn::Decode> rasn::Decode for EtsiTs103097DataSignedExternalPayload<T> {
//     fn decode_with_tag_and_constraints<D: rasn::Decoder>(
//         decoder: &mut D,
//         tag: rasn::types::Tag,
//         constraints: rasn::types::Constraints,
//     ) -> core::result::Result<Self, D::Error> {
//         let delegate_constraint: rasn::types::Constraints =
//             <EtsiTs103097Data as rasn::AsnType>::CONSTRAINTS
//                 .intersect(constraints)
//                 .intersect(const { rasn::types::Constraints::default() });
//         match tag {
//             rasn::types::Tag::EOC => Ok(Self(
//                 <EtsiTs103097Data>::decode(decoder)?,
//                 core::marker::PhantomData,
//             )),
//             _ => <EtsiTs103097Data as rasn::Decode>::decode_with_tag_and_constraints(
//                 decoder,
//                 tag,
//                 delegate_constraint,
//             )
//             .map(|data| Self(data, core::marker::PhantomData)),
//         }
//     }
// }
