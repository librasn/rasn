/// ASN.1 definitions for ETSI TS 103 097 extension module
use alloc::string::ToString;
pub mod extension_module;
use crate::ieee1609dot2::{
    Certificate, CertificateId, HashedData, Ieee1609Dot2Content, Ieee1609Dot2Data, RecipientInfo,
    SignerIdentifier,
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

impl core::ops::Deref for EtsiTs103097Data {
    type Target = Ieee1609Dot2Data;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TryFrom<Ieee1609Dot2Data> for EtsiTs103097Data {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(data: Ieee1609Dot2Data) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097Data(data);
        etsi_data.validate_components()
    }
}

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
                                    alloc::format!("PskRecipInfo, SymmRecipInfo or RekRecipInfo is not allowed, occured in index {index}"),
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

/// ETSI TS 103 097 data - unsecured
#[derive(Debug, Clone, Encode, Decode, AsnType, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataUnsecured<T: rasn::Decode>(
    EtsiTs103097Data,
    core::marker::PhantomData<T>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataUnsecured<T> {
    type Target = EtsiTs103097Data;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: rasn::Decode> TryFrom<EtsiTs103097Data> for EtsiTs103097DataUnsecured<T> {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(data: EtsiTs103097Data) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataUnsecured(data, core::marker::PhantomData);
        etsi_data.validate_components()
    }
}

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
                        Err(err) => Err(
                            rasn::error::InnerSubtypeConstraintError::InvalidInnerContaining {
                                expected: "Ieee1609Dot2Content::UnsecuredData::T",
                                err,
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
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSigned<T: rasn::Decode>(EtsiTs103097Data, core::marker::PhantomData<T>);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataSigned<T> {
    type Target = EtsiTs103097Data;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: rasn::Decode> TryFrom<EtsiTs103097Data> for EtsiTs103097DataSigned<T> {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(data: EtsiTs103097Data) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataSigned(data, core::marker::PhantomData);
        etsi_data.validate_components()
    }
}

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
                                            Err(err) => Err(
                                                rasn::error::InnerSubtypeConstraintError::InvalidInnerContaining  {
                                                    expected: "Ieee1609Dot2Content::UnsecuredData::T",
                                                    err

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

/// ETSI TS 103 097 data - signed external payload
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSignedExternalPayload<T: rasn::Decode>(
    EtsiTs103097Data,
    core::marker::PhantomData<T>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataSignedExternalPayload<T> {
    type Target = EtsiTs103097Data;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: rasn::Decode> TryFrom<EtsiTs103097Data> for EtsiTs103097DataSignedExternalPayload<T> {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(data: EtsiTs103097Data) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataSignedExternalPayload(data, core::marker::PhantomData);
        etsi_data.validate_components()
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataSignedExternalPayload<T> {
    fn validate_and_decode_containing(
        self,
        _: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.0.content {
            Ieee1609Dot2Content::SignedData(signed_data) => {
                let tbs_payload = &signed_data.tbs_data.payload;
                match &tbs_payload.ext_data_hash {
                    Some(ext_hash) => {
                        if !matches!(ext_hash, HashedData::Sha256HashedData(..)) {
                            return Err(
                                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                                    component_path: "EtsiTs103097DataSignedExternalPayload.content.signedData.tbsData.payload.extDataHash",
                                    component_type: "HashedData",
                                    details: "Only SHA-256 hashed data is allowed".to_string(),
                                }
                            );
                        }
                        Ok(self)
                    },
                    None => Err(
                        rasn::error::InnerSubtypeConstraintError::MissingRequiredComponent {
                            component_path: "EtsiTs103097DataSignedExternalPayload.content.signedData.tbsData.payload",
                            components: &["extDataHash"],
                        }
                    ),
                }
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    component_path: "EtsiTs103097DataSignedExternalPayload.content",
                    component_type: "Ieee1609Dot2Content",
                    details: "Only SignedData is allowed".to_string(),
                },
            ),
        }
    }
}

/// ETSI TS 103 097 data - encrypted
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataEncrypted<T: rasn::Decode>(
    EtsiTs103097Data,
    core::marker::PhantomData<T>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataEncrypted<T> {
    type Target = EtsiTs103097Data;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: rasn::Decode> TryFrom<EtsiTs103097Data> for EtsiTs103097DataEncrypted<T> {
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(data: EtsiTs103097Data) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataEncrypted(data, core::marker::PhantomData);
        etsi_data.validate_components()
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataEncrypted<T> {
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        match &self.0.content {
            Ieee1609Dot2Content::EncryptedData(encrypted_data) => {
                match &encrypted_data.ciphertext {
                    crate::ieee1609dot2::SymmetricCiphertext::Aes128ccm(ciphertext)
                    | crate::ieee1609dot2::SymmetricCiphertext::Sm4Ccm(ciphertext) => {
                        if let Some(codec) = decode_containing_with {
                            if codec
                                .decode_from_binary::<T>(&ciphertext.ccm_ciphertext)
                                .is_ok()
                            {
                                return Ok(self);
                            } else {
                                return Err(
                                    rasn::error::InnerSubtypeConstraintError::InvalidComponentValue {
                                        component_path:
                                            "EtsiTs103097DataEncrypted.content.encryptedData.ciphertext",
                                        component_name: "ccmCiphertext",
                                        details: "Failed to decode ccmCiphertext as the specified type"
                                            .to_string(),
                                    },
                                );
                            }
                        }
                        Ok(self)
                    }
                    // _ => Err(
                    //     rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    //         component_path:
                    //             "EtsiTs103097DataEncrypted.content.encryptedData.ciphertext",
                    //         component_type: "SymmetricCiphertext",
                    //         details: "Only aes128ccm and sm4ccm is allowed".to_string(),
                    //     },
                    // ),
                }
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    component_path: "EtsiTs103097DataEncrypted.content",
                    component_type: "Ieee1609Dot2Content",
                    details: "Only EncryptedData is allowed".to_string(),
                },
            ),
        }
    }
}

/// ETSI TS 103 097 data - signed and encrypted
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSignedAndEncrypted<T: rasn::Decode>(
    EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataSignedAndEncrypted<T> {
    type Target = EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: rasn::Decode> TryFrom<EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>>
    for EtsiTs103097DataSignedAndEncrypted<T>
{
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(
        data: EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>,
    ) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataSignedAndEncrypted(data);
        etsi_data.validate_components()
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataSignedAndEncrypted<T> {
    fn validate_and_decode_containing(
        self,
        codec: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        Ok(Self(self.0.validate_and_decode_containing(codec)?))
    }
}

/// ETSI TS 103 097 data - encrypted unicast
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataEncryptedUnicast<T: rasn::Decode>(
    EtsiTs103097DataEncrypted<EtsiTs103097DataUnsecured<T>>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataEncryptedUnicast<T> {
    type Target = EtsiTs103097DataEncrypted<EtsiTs103097DataUnsecured<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: rasn::Decode> TryFrom<EtsiTs103097DataEncrypted<EtsiTs103097DataUnsecured<T>>>
    for EtsiTs103097DataEncryptedUnicast<T>
{
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(
        data: EtsiTs103097DataEncrypted<EtsiTs103097DataUnsecured<T>>,
    ) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataEncryptedUnicast(data);
        etsi_data.validate_components()
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataEncryptedUnicast<T> {
    fn validate_and_decode_containing(
        self,
        codec: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        let encrypted = self.0.validate_and_decode_containing(codec)?;
        //
        match &encrypted.0.content {
            Ieee1609Dot2Content::EncryptedData(encrypted_data) => {
                if encrypted_data.recipients.len() != 1 {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::InvalidComponentSize {
                            component_path:
                                "EtsiTs103097DataEncryptedUnicast.content.encryptedData",
                            component_name: "recipients",
                            details: "Exactly one recipient is required".to_string(),
                        },
                    );
                }
                Ok(Self(encrypted))
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    component_path: "EtsiTs103097DataEncryptedUnicast.content",
                    component_type: "Ieee1609Dot2Content",
                    details: "Only EncryptedData is allowed".to_string(),
                },
            ),
        }
    }
}

/// ETSI TS 103 097 data - signed and encrypted unicast
#[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq, Eq)]
#[rasn(delegate)]
pub struct EtsiTs103097DataSignedAndEncryptedUnicast<T: rasn::Decode>(
    EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>,
);

impl<T: rasn::Decode> core::ops::Deref for EtsiTs103097DataSignedAndEncryptedUnicast<T> {
    type Target = EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: rasn::Decode> TryFrom<EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>>
    for EtsiTs103097DataSignedAndEncryptedUnicast<T>
{
    type Error = rasn::error::InnerSubtypeConstraintError;
    fn try_from(
        data: EtsiTs103097DataEncrypted<EtsiTs103097DataSigned<T>>,
    ) -> Result<Self, Self::Error> {
        let etsi_data = EtsiTs103097DataSignedAndEncryptedUnicast(data);
        etsi_data.validate_components()
    }
}

impl<T: rasn::Decode> InnerSubtypeConstraint for EtsiTs103097DataSignedAndEncryptedUnicast<T> {
    fn validate_and_decode_containing(
        self,
        codec: Option<rasn::Codec>,
    ) -> Result<Self, rasn::error::InnerSubtypeConstraintError> {
        let encrypted = self.0.validate_and_decode_containing(codec)?;
        match &encrypted.0.content {
            Ieee1609Dot2Content::EncryptedData(encrypted_data) => {
                if encrypted_data.recipients.len() != 1 {
                    return Err(
                        rasn::error::InnerSubtypeConstraintError::InvalidComponentSize {
                            component_path:
                                "EtsiTs103097DataSignedAndEncryptedUnicast.content.encryptedData",
                            component_name: "recipients",
                            details: "Exactly one recipient is required".to_string(),
                        },
                    );
                }
                Ok(Self(encrypted))
            }
            _ => Err(
                rasn::error::InnerSubtypeConstraintError::InvalidComponentVariant {
                    component_path: "EtsiTs103097DataSignedAndEncryptedUnicast.content",
                    component_type: "Ieee1609Dot2Content",
                    details: "Only EncryptedData is allowed".to_string(),
                },
            ),
        }
    }
}
