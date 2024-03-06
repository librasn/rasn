use rasn::prelude::*;

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 2))]
#[rasn(delegate)]
pub struct UnbindRequest(pub ());

#[test]
fn issue225() {
    let unbind_request = UnbindRequest(());
    let encoded = rasn::ber::encode(&unbind_request).unwrap();
    assert_eq!(encoded, &[66, 0]);
}
