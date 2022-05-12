use rasn::der::{decode, encode};

use rasn_cms::authenticode::{
    SpcIndirectDataContent, SpcLink, SpcPeImageData, SPC_CLASS_UUID, SPC_INDIRECT_DATA_OBJID,
};
use rasn_cms::*;

// from "openssl cms" output
const SIGNED_DATA: &[u8] = include_bytes!("data/signed.cms");
const ENCRYPTED_DATA: &[u8] = include_bytes!("data/encrypted.cms");
const PE_SIG_DATA: &[u8] = include_bytes!("data/pesig.p7");

#[test]
fn test_cms_signed() {
    let info = decode::<ContentInfo>(SIGNED_DATA).unwrap();
    assert_eq!(CONTENT_SIGNED_DATA, info.content_type);
    let data = decode::<SignedData>(info.content.as_bytes()).unwrap();
    println!("{:#?}", data);

    assert_eq!(CONTENT_DATA, data.encap_content_info.content_type);

    let encoded_data = encode(&data).unwrap();
    let decoded_data = decode::<SignedData>(&encoded_data).unwrap();
    assert_eq!(decoded_data, data);
}

#[test]
fn test_cms_encrypted() {
    let info = decode::<ContentInfo>(ENCRYPTED_DATA).unwrap();
    assert_eq!(CONTENT_ENVELOPED_DATA, info.content_type);
    let data = decode::<EnvelopedData>(info.content.as_bytes()).unwrap();
    println!("{:#?}", data);

    assert_eq!(CONTENT_DATA, data.encrypted_content_info.content_type);

    let encoded_data = encode(&data).unwrap();
    let decoded_data = decode::<EnvelopedData>(&encoded_data).unwrap();
    assert_eq!(decoded_data, data);
}

#[test]
fn test_authenticode() {
    let info = decode::<ContentInfo>(PE_SIG_DATA).unwrap();
    assert_eq!(CONTENT_SIGNED_DATA, info.content_type);

    let signed_data = decode::<pkcs7_compat::SignedData>(info.content.as_bytes()).unwrap();
    assert_eq!(
        SPC_INDIRECT_DATA_OBJID,
        signed_data.encap_content_info.content_type
    );

    let content = decode::<SpcIndirectDataContent>(
        signed_data.encap_content_info.content.unwrap().as_bytes(),
    )
    .unwrap();

    let image_data = decode::<SpcPeImageData>(content.data.value.unwrap().as_bytes()).unwrap();
    println!("{:#?}", image_data);

    match image_data.file {
        Some(SpcLink::Moniker(obj)) if obj.class_id == SPC_CLASS_UUID => {}
        _ => panic!("Unexpected SpcUuid value"),
    }
}
