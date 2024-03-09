use rasn::prelude::*;

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 2))]
pub struct UnbindRequest;

#[test]
fn issue225() {
    let unbind_request = UnbindRequest;
    let encoded = rasn::ber::encode(&unbind_request).unwrap();
    assert_eq!(encoded, &[66, 0]);
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UntaggedUnit;

#[test]
fn untagged_unit_struct() {
    let encoded_unit_struct = rasn::ber::encode(&UntaggedUnit).unwrap();
    let encoded_unit = rasn::ber::encode(&()).unwrap();
    assert_eq!(encoded_unit, encoded_unit_struct);
}
