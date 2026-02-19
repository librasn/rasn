use rasn::prelude::*;

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
struct Message {
    id: i32,
    body: Body,
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(choice)]
enum Body {
    #[rasn(tag(context, 3000))]
    Request(Request),

    #[rasn(tag(context, 3001))]
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
        vec![
            0x30, 0x0C, 0x80, 0x01, 0x01, 0xA1, 0x07, 0xBF, 0x97, 0x38, 0x03, 0x80, 0x01, 0x01
        ],
        rasn::der::encode(&Message {
            id: 1,
            body: Body::Request(Request { num: 1 }),
        })
        .unwrap()
    );
}
