use rasn::{types::*, Decode, Encode};

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum ProtocolMessages{
    #[rasn(tag(0))]
    Message(MessagePdu),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct MessagePdu{
    message_id: rasn::types::Integer,
    name_objects: NameObjects,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum NameObjects {
    #[rasn(tag(6))]
    GetVariableAccessAttributes(GetVariableAccessAttributesRequest),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum GetVariableAccessAttributesRequest {
    #[rasn(tag(0))]
    NameId(rasn::types::Integer),
}

#[test]
fn it_works() {
    let data = ProtocolMessages::Message(MessagePdu {
        message_id: 303731.into(),
        name_objects: NameObjects::GetVariableAccessAttributes(GetVariableAccessAttributesRequest::NameId(0.into())),
    });

    let bin = rasn::ber::encode(&data).unwrap();
    assert_eq!(data, rasn::ber::decode(&bin).unwrap());
}
