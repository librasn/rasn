use rasn::{types::*, Decode, Encode};

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum ProtocolMessages{
    #[rasn(tag(0))]
    message(MessagePDU),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct MessagePDU{
    messageID: rasn::types::Integer,
    nameObjects: NameObjects,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum NameObjects {
    #[rasn(tag(6))]
    getVariableAccessAttributes(GetVariableAccessAttributesRequest),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum GetVariableAccessAttributesRequest {
    #[rasn(tag(0))]
    nameId(rasn::types::Integer),
}

#[test]
fn it_works() {
    let data = ProtocolMessages::message(MessagePDU {
        messageID: 303731.into(),
        nameObjects: NameObjects::getVariableAccessAttributes(GetVariableAccessAttributesRequest::nameId(0.into())),
    });

    let bin = rasn::ber::encode(&data).unwrap();
    assert_eq!(data, rasn::ber::decode(&bin).unwrap());
}
