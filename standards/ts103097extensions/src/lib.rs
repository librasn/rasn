extern crate alloc;
use bon::Builder;
use rasn::prelude::*;

// In order to avoid cyclic dependencies, we make following weak types
type HashedId8 = [u8; 8];
type Time32 = u32;

pub const EXTENSION_MODULE_VERSION: u8 = 1;
pub const ETSI_TS103097_EXTENSION_MODULE_OID: &Oid =
    Oid::const_new(&[0, 4, 0, 5, 5, 103_097, 2, 1, 1]);

pub const ETSI_TS102941_CRL_REQUEST_ID: EtsiTs103097HeaderInfoExtensionId =
    EtsiTs103097HeaderInfoExtensionId(ExtId(1));
pub const ETSI_TS102941_DELTA_CTL_REQUEST_ID: EtsiTs103097HeaderInfoExtensionId =
    EtsiTs103097HeaderInfoExtensionId(ExtId(2));

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum EtsiExtensionContent {
    CrlRequest(EtsiTs102941CrlRequest),
    DeltaCtlRequest(EtsiTs102941DeltaCtlRequest),
}

#[derive(AsnType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct EtsiOriginatingHeaderInfoExtension {
    #[rasn(identifier = "id")]
    pub id: u8,
    #[rasn(identifier = "content")]
    pub content: EtsiExtensionContent,
}
impl rasn::Encode for EtsiOriginatingHeaderInfoExtension {
    fn encode_with_tag_and_constraints<'encoder, EN: rasn::Encoder<'encoder>>(
        &self,
        encoder: &mut EN,
        tag: rasn::types::Tag,
        _: rasn::types::Constraints,
    ) -> core::result::Result<(), EN::Error> {
        encoder
            .encode_sequence::<2usize, 0usize, Self, _>(tag, |encoder| {
                self.id.encode_with_tag(
                    encoder,
                    rasn::types::Tag::new(rasn::types::Class::Context, 0usize as u32),
                )?;
                match self.content {
                    EtsiExtensionContent::CrlRequest(ref content) => {
                        content.encode_with_tag(
                            encoder,
                            rasn::types::Tag::new(rasn::types::Class::Context, 1usize as u32),
                        )?;
                    }
                    EtsiExtensionContent::DeltaCtlRequest(ref content) => {
                        content.encode_with_tag(
                            encoder,
                            rasn::types::Tag::new(rasn::types::Class::Context, 2usize as u32),
                        )?;
                    }
                }
                Ok(())
            })
            .map(drop)
    }
}
impl rasn::Decode for EtsiOriginatingHeaderInfoExtension {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::types::Tag,
        _: rasn::types::Constraints,
    ) -> core::result::Result<Self, D::Error> {
        decoder.decode_sequence::<2usize, 0usize, _, _, _>(tag, None::<fn() -> Self>, |decoder| {
            let id = <_>::decode_with_tag(
                decoder,
                rasn::types::Tag::new(rasn::types::Class::Context, 0usize as u32),
            )
            .map_err(|error| {
                rasn::de::Error::field_error(
                    "EtsiOriginatingHeaderInfoExtension.id",
                    error.into(),
                    decoder.codec(),
                )
            })?;
            match id {
                ETSI_TS102941_CRL_REQUEST_ID => {
                    let content = <_>::decode_with_tag(
                        decoder,
                        rasn::types::Tag::new(rasn::types::Class::Context, 1usize as u32),
                    )
                    .map_err(|error| {
                        rasn::de::Error::field_error(
                            "EtsiOriginatingHeaderInfoExtension.content",
                            error.into(),
                            decoder.codec(),
                        )
                    })?;
                    Ok(Self {
                        id: **id,
                        content: EtsiExtensionContent::CrlRequest(content),
                    })
                }
                ETSI_TS102941_DELTA_CTL_REQUEST_ID => {
                    let content = <_>::decode_with_tag(
                        decoder,
                        rasn::types::Tag::new(rasn::types::Class::Context, 2usize as u32),
                    )
                    .map_err(|error| {
                        rasn::de::Error::field_error(
                            "EtsiOriginatingHeaderInfoExtension.content",
                            error.into(),
                            decoder.codec(),
                        )
                    })?;
                    Ok(Self {
                        id: **id,
                        content: EtsiExtensionContent::DeltaCtlRequest(content),
                    })
                }
                _ => Err(rasn::de::Error::custom(
                    format!(
                        "Unexpected value for EtsiOriginatingHeaderInfoExtension.id: {}",
                        **id
                    ),
                    decoder.codec(),
                )),
            }
        })
    }
}

// With the current version of the standard, we allow only two ways to create extensions
impl EtsiOriginatingHeaderInfoExtension {
    // Helper method to create CrlRequest extension
    pub fn new_crl_request(content: EtsiTs102941CrlRequest) -> Self {
        Self {
            id: 1, // etsiTs102941CrlRequestId
            content: EtsiExtensionContent::CrlRequest(content),
        }
    }

    // Helper method to create CtlRequest extension
    pub fn new_delta_ctl_request(content: EtsiTs102941DeltaCtlRequest) -> Self {
        Self {
            id: 2, // etsiTs102941DeltaCtlRequestId
            content: EtsiExtensionContent::DeltaCtlRequest(content),
        }
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct EtsiTs102941DeltaCtlRequest(pub EtsiTs102941CtlRequest);

impl From<EtsiTs102941CtlRequest> for EtsiTs102941DeltaCtlRequest {
    fn from(item: EtsiTs102941CtlRequest) -> Self {
        EtsiTs102941DeltaCtlRequest(item)
    }
}

impl core::ops::Deref for EtsiTs102941DeltaCtlRequest {
    type Target = EtsiTs102941CtlRequest;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct EtsiTs103097HeaderInfoExtensionId(pub ExtId);

impl From<ExtId> for EtsiTs103097HeaderInfoExtensionId {
    fn from(item: ExtId) -> Self {
        EtsiTs103097HeaderInfoExtensionId(item)
    }
}
impl core::ops::Deref for EtsiTs103097HeaderInfoExtensionId {
    type Target = ExtId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
