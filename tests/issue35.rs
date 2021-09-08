use rasn::AsnType;
use rasn::Decode;
use rasn::Encode;
use std::error::Error;

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
struct Message {
    id: i32,
    body: BODY,
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(choice)]
enum BODY {
    #[rasn(tag(context, 100))]
    Request(Request),

    #[rasn(tag(context, 200))]
    Response(Response),
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
struct Request {
    num: i32,
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
struct Response {
    ret: i32,
}

#[test]
fn it_works() {
    assert_eq!(
        vec![0x30, 0x0B, 0x80, 0x01, 0x01, 0xA1, 0x06, 0xBF, 0x64, 0x03, 0x80, 0x01, 0x01],
        rasn::der::encode(&Message {
            id: 1,
            body: BODY::Request(Request { num: 1 }),
        })
        .unwrap()
    );
}
