#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "AlgorithmID-ShortForm")]
pub struct AlgorithmIdShortForm(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]

pub struct CompressedContent(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum CompressedContentInfoContentType {
    #[rasn(tag(context, 0), identifier = "contentType-ShortForm")]
    ContentTypeShortForm(ContentTypeShortForm),
    #[rasn(tag(context, 1), identifier = "contentType-OID")]
    ContentTypeOid(ObjectIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct CompressedContentInfo {
    #[rasn(identifier = "contentType")]
    pub content_type: CompressedContentInfoContentType,
    #[rasn(tag(explicit(context, 0)), identifier = "compressedContent")]
    pub compressed_content: CompressedContent,
}

impl CompressedContentInfo {
    pub fn new(
        content_type: CompressedContentInfoContentType,
        compressed_content: CompressedContent,
    ) -> Self {
        Self {
            content_type,
            compressed_content,
        }
    }
}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct CompressedData {
    #[rasn(identifier = "compressionAlgorithm")]
    pub compression_algorithm: CompressionAlgorithmIdentifier,
    #[rasn(identifier = "compressedContentInfo")]
    pub compressed_content_info: CompressedContentInfo,
}
impl CompressedData {
    pub fn new(
        compression_algorithm: CompressionAlgorithmIdentifier,
        compressed_content_info: CompressedContentInfo,
    ) -> Self {
        Self {
            compression_algorithm,
            compressed_content_info,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum CompressionAlgorithmIdentifier {
    #[rasn(tag(context, 0), identifier = "algorithmID-ShortForm")]
    AlgorithmIdShortForm(AlgorithmIdShortForm),
    #[rasn(tag(context, 1), identifier = "algorithmID-OID")]
    AlgorithmIdOid(ObjectIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "ContentType-ShortForm")]
pub struct ContentTypeShortForm(pub Integer);
