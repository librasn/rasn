use rasn::{types::*, *};

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum ProtocolMessages {
    #[rasn(tag(0))]
    Message(MessagePDU),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct MessagePDU {
    #[rasn(tag(0))]
    pub message_id: rasn::types::Integer,

    #[rasn(tag(1))]
    pub message_num: Option<rasn::types::Integer>,

    #[rasn(tag(2))]
    pub message_status: MessageStatus,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct MessageStatus {
    #[rasn(tag(0))]
    #[rasn(choice)]
    pub message_class: MessageClass,

    #[rasn(tag(1))]
    pub status_id: Option<rasn::types::Integer>,

    #[rasn(tag(2))]
    #[rasn(choice)]
    pub status_enum: Option<MessageEnum>,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum MessageClass {
    #[rasn(tag(0))]
    Test1(rasn::types::Integer),

    #[rasn(tag(1))]
    Test2(rasn::types::Integer),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum MessageEnum {
    #[rasn(tag(0))]
    Hello(rasn::types::Integer),

    #[rasn(tag(1))]
    World(rasn::types::Integer),

    #[rasn(tag(2))]
    Test(rasn::types::Integer),
}

#[test]
fn it_works() {
    let data = ProtocolMessages::Message(MessagePDU {
        message_id: 0.into(),
        message_num: None, 
        message_status: MessageStatus{
            message_class: MessageClass::Test1(0.into()),
            status_id: None,
            status_enum: None,
        }
    });

    let bin = rasn::ber::encode(&data).unwrap();
    assert_eq!(data, rasn::ber::decode(&bin).unwrap());
}
