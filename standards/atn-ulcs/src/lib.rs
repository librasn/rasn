#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1", extensible), identifier = "Fully-encoded-data")]
pub struct FullyEncodedData(pub SequenceOf<PdvList>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum PDVListPresentationDataValues {
    #[rasn(tag(context, 0), identifier = "single-ASN1-type")]
    SingleAsn1Type(Any),
    #[rasn(tag(context, 1), identifier = "octet-aligned")]
    OctetAligned(OctetString),
    #[rasn(tag(context, 2))]
    Arbitrary(BitString),
}

/// Contains one or more presentation-data-value-list (PDV-list) values
/// ATN commentary.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(identifier = "PDV-list")]
pub struct PdvList {
    #[rasn(identifier = "transfer-syntax-name")]
    pub transfer_syntax_name: Option<TransferSyntaxName>,
    #[rasn(identifier = "presentation-context-identifier")]
    pub presentation_context_identifier: PresentationContextIdentifier,
    #[rasn(identifier = "presentation-data-values")]
    pub presentation_data_values: PDVListPresentationDataValues,
}

impl PdvList {
    pub fn new(
        transfer_syntax_name: Option<TransferSyntaxName>,
        presentation_context_identifier: PresentationContextIdentifier,
        presentation_data_values: PDVListPresentationDataValues,
    ) -> Self {
        Self {
            transfer_syntax_name,
            presentation_context_identifier,
            presentation_data_values,
        }
    }
}

/// ATN: not used for ATN Upper Layers
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(
    delegate,
    value("1..=127", extensible),
    identifier = "Presentation-context-identifier"
)]
pub struct PresentationContextIdentifier(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "Transfer-syntax-name")]
pub struct TransferSyntaxName(pub ObjectIdentifier);
