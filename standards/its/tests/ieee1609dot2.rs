use rasn_its::ieee1609dot2::*;

// TlmCertificateTrustListMessage ::= EtsiTs103097Data-Signed{EtsiTs102941Data (WITH COMPONENTS{..., content (WITH COMPONENTS{certificateTrustListTlm PRESENT})})}
// FROM https://cpoc.jrc.ec.europa.eu/ECTL.html
#[test]
fn test_ectl_trust_list() {
    let ectl_data: &[u8] = include_bytes!("data/CE4CF6C19BFED720.oer");
    let mut buffer = Vec::<u8>::with_capacity(ectl_data.len());
    let decoded = rasn::coer::decode::<Ieee1609Dot2Data>(ectl_data).unwrap();
    // inner unencrypted content is EtsiTs102941Data that we can't parse right now
    rasn::coer::encode_buf(&decoded, &mut buffer).unwrap();
    assert_eq!(ectl_data, &buffer[..]);
    buffer.clear();
}
// From https://cpoc.jrc.ec.europa.eu/TLMCertificates.html
#[test]
fn test_tlm_certificate() {
    let tlm_data: &[u8] = include_bytes!("data/E7A4B2B045E7ACF9.oer");
    let mut buffer = Vec::<u8>::with_capacity(tlm_data.len());
    let decoded = rasn::coer::decode::<Certificate>(tlm_data).unwrap();
    rasn::coer::encode_buf(&decoded, &mut buffer).unwrap();
    assert_eq!(tlm_data, &buffer[..]);
    buffer.clear();
}
