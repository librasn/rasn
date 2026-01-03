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
// This test verifies that recursive definitions in Ieee1609Dot2Data, which could be
// exploited to exhaust the parser's stack, are correctly handled by the depth limit.
#[test]
fn test_ectl_trust_list_deeply_nested_data() {
    let ectl_data: &[u8] = include_bytes!("data/CE4CF6C19BFED720_with_recursion_depth150.oer");
    let result = rasn::coer::decode::<Ieee1609Dot2Data>(ectl_data);
    let err = result.unwrap_err();
    assert!(err.matches_root_cause(|kind| matches!(
        kind,
        rasn::error::DecodeErrorKind::ExceedsMaxParseDepth
    )));
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
