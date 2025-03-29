extern crate alloc;
use crate::ieee1609dot2::base_types::{HashedId8, Time32};
use bon::Builder;
use rasn::prelude::*;

pub const EXTENSION_MODULE_VERSION: u8 = 1;
pub const ETSI_TS103097_EXTENSION_MODULE_OID: &Oid =
    Oid::const_new(&[0, 4, 0, 5, 5, 103_097, 2, 1, 1]);

pub const ETSI_TS102941_CRL_REQUEST_ID: ExtId = ExtId(1);
pub const ETSI_TS102941_DELTA_CTL_REQUEST_ID: ExtId = ExtId(2);

/// This type is used as an identifier for instances of ExtContent within an EXT-TYPE.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ExtId(pub u8);

impl From<u8> for ExtId {
    fn from(item: u8) -> Self {
        ExtId(item)
    }
}
impl core::ops::Deref for ExtId {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Adapted and moved from IEEE 1609.2-2022 Base Types as shared trait among these standards.
/// T `EXT-TYPE` class defines objects in a form suitable for import into the definition of HeaderInfo.
pub trait ExtType: AsnType + Encode + Decode {
    type ExtContent: AsnType + Encode + Decode;
    const EXT_ID: ExtId;
}

///  This parameterized type represents a (id, content) pair drawn from the set ExtensionTypes, which is constrained to contain objects defined by the class EXT-TYPE.
#[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(automatic_tags)]
pub struct Extension<T: ExtType> {
    #[rasn(identifier = "id")]
    pub id: ExtId,
    #[rasn(identifier = "content")]
    pub content: T::ExtContent,
}

#[derive(AsnType, Copy, Clone, Debug, Encode, Decode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum EtsiTs103097HeaderInfoExtensions {
    CrlRequest,
    DeltaCtlRequest,
}
impl EtsiTs103097HeaderInfoExtensions {
    pub fn ext_id(&self) -> ExtId {
        match self {
            Self::CrlRequest => ETSI_TS102941_CRL_REQUEST_ID,
            Self::DeltaCtlRequest => ETSI_TS102941_DELTA_CTL_REQUEST_ID,
        }
    }
}

#[derive(AsnType, Debug, Decode, Clone, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub enum EtsiExtContent {
    CrlRequest(EtsiTs102941CrlRequest),
    DeltaCtlRequest(EtsiTs102941DeltaCtlRequest),
}

// Custom encode - we just pass the internal type to be encoded
impl rasn::Encode for EtsiExtContent {
    fn encode<'encoder, E: rasn::Encoder<'encoder>>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), E::Error> {
        match self {
            EtsiExtContent::CrlRequest(value) => value.encode_with_tag(encoder, Tag::SEQUENCE),
            EtsiExtContent::DeltaCtlRequest(value) => value.encode_with_tag(encoder, Tag::SEQUENCE),
        }
    }
    fn encode_with_tag_and_constraints<'encoder, EN: rasn::Encoder<'encoder>>(
        &self,
        encoder: &mut EN,
        _: rasn::types::Tag,
        _: rasn::types::Constraints,
        _: rasn::types::Identifier,
    ) -> core::result::Result<(), EN::Error> {
        Self::encode(self, encoder)
    }
}

impl ExtType for EtsiTs103097HeaderInfoExtensions {
    type ExtContent = EtsiExtContent;
    const EXT_ID: ExtId = ExtId(0); // Default shoult not be used
}

pub type EtsiTs102941DeltaCtlRequest = EtsiTs102941CtlRequest;

#[derive(AsnType, Debug, Encode, Clone, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct EtsiOriginatingHeaderInfoExtension(Extension<EtsiTs103097HeaderInfoExtensions>);

/// We have to skip the choice type of EtsiExtContent when decoding
impl rasn::Decode for EtsiOriginatingHeaderInfoExtension {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::types::Tag,
        _: rasn::types::Constraints,
    ) -> core::result::Result<Self, D::Error> {
        Ok(Self(decoder
            .decode_sequence::<2usize, 0usize, Extension<EtsiTs103097HeaderInfoExtensions>, _, _>(
                tag,
                None::<fn() -> Extension<EtsiTs103097HeaderInfoExtensions>>,
                |decoder| {
                    let id = <_>::decode_with_tag(
                        decoder,
                        rasn::types::Tag::new(rasn::types::Class::Context, 0usize as u32),
                    )
                    .map_err(|error| {
                        rasn::de::Error::field_error("Extension.id", error.into(), decoder.codec())
                    })?;
                    match id {
                        ETSI_TS102941_CRL_REQUEST_ID => {
                            let content = <EtsiTs102941CrlRequest>::decode_with_tag(
                                decoder,
                                rasn::types::Tag::new(rasn::types::Class::Context, 1usize as u32),
                            )
                            .map_err(|error| {
                                rasn::de::Error::field_error(
                                    "Extension.content - EtsiTs102941CrlRequest",
                                    error.into(),
                                    decoder.codec(),
                                )
                            })?;
                            Ok(Extension {
                                id,
                                content: EtsiExtContent::CrlRequest(content),
                            })
                        }
                        ETSI_TS102941_DELTA_CTL_REQUEST_ID => {
                            let content = <EtsiTs102941DeltaCtlRequest>::decode_with_tag(
                                decoder,
                                rasn::types::Tag::new(rasn::types::Class::Context, 1usize as u32),
                            )
                            .map_err(|error| {
                                rasn::de::Error::field_error(
                                    "Extension.content - EtsiTs102941DeltaCtlRequest",
                                    error.into(),
                                    decoder.codec(),
                                )
                            })?;
                             Ok(Extension {
                                id,
                                content: EtsiExtContent::DeltaCtlRequest(content),
                            })
                        }
                        _ => {
                             Err(rasn::de::Error::custom(
                                alloc::format!("Unknown extension id: {:?}", id),
                                decoder.codec(),
                            ))
                        }
                    }
                },
                )?))
    }
}

// With the current version of the standard, we allow only two ways to create extensions
impl EtsiOriginatingHeaderInfoExtension {
    pub fn new_crl_request(request: EtsiTs102941CrlRequest) -> Self {
        Extension {
            id: EtsiTs103097HeaderInfoExtensions::CrlRequest.ext_id(),
            content: EtsiExtContent::CrlRequest(request),
        }
        .into()
    }
    pub fn new_delta_ctl_request(request: EtsiTs102941DeltaCtlRequest) -> Self {
        Extension {
            id: EtsiTs103097HeaderInfoExtensions::DeltaCtlRequest.ext_id(),
            content: EtsiExtContent::DeltaCtlRequest(request),
        }
        .into()
    }
}

impl From<Extension<EtsiTs103097HeaderInfoExtensions>> for EtsiOriginatingHeaderInfoExtension {
    fn from(item: Extension<EtsiTs103097HeaderInfoExtensions>) -> Self {
        EtsiOriginatingHeaderInfoExtension(item)
    }
}

#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EtsiTs102941CrlRequest {
    #[rasn(identifier = "issuerId")]
    pub issuer_id: HashedId8,
    #[rasn(identifier = "lastKnownUpdate")]
    pub last_known_update: Option<Time32>,
}

#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EtsiTs102941CtlRequest {
    #[rasn(identifier = "issuerId")]
    pub issuer_id: HashedId8,
    #[rasn(identifier = "lastKnownCtlSequence")]
    pub last_known_ctl_sequence: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crl_request() {
        let crl_request = EtsiTs102941CrlRequest::builder()
            .issuer_id(HashedId8([2; 8].into()))
            .last_known_update(10.into())
            .build();
        let extension = EtsiOriginatingHeaderInfoExtension::new_crl_request(crl_request);
        let encoded = rasn::coer::encode(&extension).unwrap();
        let expected = &[
            0x01,
            0b1000_0000,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x00,
            0x00,
            0x00,
            0x0A,
        ];
        pretty_assertions::assert_eq!(encoded, expected);
        let decoded =
            rasn::coer::decode::<super::EtsiOriginatingHeaderInfoExtension>(&encoded).unwrap();
        pretty_assertions::assert_eq!(extension, decoded);
    }
    #[test]
    fn test_ctl_request() {
        let ctl_request = EtsiTs102941CtlRequest::builder()
            .issuer_id(HashedId8([2; 8].into()))
            .last_known_ctl_sequence(10)
            .build();
        let extension = EtsiOriginatingHeaderInfoExtension::new_delta_ctl_request(ctl_request);
        let encoded = rasn::coer::encode(&extension).unwrap();
        let expected = &[
            0x02,
            0b1000_0000,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x02,
            0x0A,
        ];
        pretty_assertions::assert_eq!(encoded, expected);
        let decoded =
            rasn::coer::decode::<super::EtsiOriginatingHeaderInfoExtension>(&encoded).unwrap();
        pretty_assertions::assert_eq!(extension, decoded);
    }
}
