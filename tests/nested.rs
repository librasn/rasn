use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
pub enum ProtocolMessages {
    #[rasn(tag(0))]
    Message(MessagePDU),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct MessagePDU {
    message_id: rasn::types::Integer,
    name_objects: NameObjects,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum NameObjects {
    #[rasn(tag(0))]
    TestObject(TestObject),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct TestObject {
    #[rasn(tag(0))]
    nested_enum: NestedEnum,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
#[rasn(choice)]
enum NestedEnum {
    #[rasn(tag(0))]
    NestObject(NestObject),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq, Clone)]
pub struct NestObject {
    #[rasn(tag(0))]
    nested_value: rasn::types::Integer,
}

#[test]
fn it_works() {
    let data = ProtocolMessages::Message(MessagePDU {
        message_id: 0.into(),
        name_objects: NameObjects::TestObject(TestObject {
            nested_enum: NestedEnum::NestObject(NestObject {
                nested_value: 0.into(),
            }),
        }),
    });

    let bin = rasn::ber::encode(&data).unwrap();
    println!("{:?}", bin);
    assert_eq!(data, rasn::ber::decode(&bin).unwrap());
}
