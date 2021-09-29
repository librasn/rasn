use rasn::der::{decode, encode};

use rasn_cms::*;

// from "openssl cms" output
const SIGNED_DATA: &[u8] = include_bytes!("data/signed.cms");
const ENCRYPTED_DATA: &[u8] = include_bytes!("data/encrypted.cms");

#[test]
fn test_cms_signed() {
    let info = decode::<ContentInfo>(SIGNED_DATA).unwrap();
    assert_eq!(OID_CONTENT_SIGNED_DATA, info.content_type);
    let data = decode::<SignedData>(info.content.as_bytes()).unwrap();
    println!("{:#?}", data);

    assert_eq!(OID_CONTENT_DATA, data.encap_content_info.content_type);

    let encoded_data = encode(&data).unwrap();
    let decoded_data = decode::<SignedData>(&encoded_data).unwrap();
    assert_eq!(decoded_data, data);
}

#[test]
fn test_cms_encrypted() {
    let info = decode::<ContentInfo>(ENCRYPTED_DATA).unwrap();
    assert_eq!(OID_CONTENT_ENVELOPED_DATA, info.content_type);
    let data = decode::<EnvelopedData>(info.content.as_bytes()).unwrap();
    println!("{:#?}", data);

    assert_eq!(OID_CONTENT_DATA, data.encrypted_content_info.content_type);

    let encoded_data = encode(&data).unwrap();
    let decoded_data = decode::<EnvelopedData>(&encoded_data).unwrap();
    assert_eq!(decoded_data, data);
}
