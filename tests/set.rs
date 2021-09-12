use rasn::{Decode, Encode, types::*};

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set)]
struct Set {
    age: Integer,
    name: String,
}

#[test]
fn asn_type() {
    static_assertions::const_assert!(Set::TAG.const_eq(&Tag::SET));
}

#[test]
fn encode() {
    assert_eq!(
        &[0x31, 0x9, 0x2, 0x1, 0x1, 0xc, 0x4, 0x4a, 0x61, 0x6e, 0x65][..],
        rasn::ber::encode(&Set { age: 1.into(), name: "Jane".into() }).unwrap()
    );
}

#[test]
fn decode() {
    let expected = Set { age: 1.into(), name: "Jane".into() };
    // Age then Name
    assert_eq!(
        expected,
        rasn::ber::decode::<Set>(&[0x31, 0x9, 0x2, 0x1, 0x1, 0xc, 0x4, 0x4a, 0x61, 0x6e, 0x65][..]).unwrap(),
    );
    // Name then Age
    assert_eq!(
        expected,
        rasn::ber::decode::<Set>(&[0x31, 0x9, 0xc, 0x4, 0x4a, 0x61, 0x6e, 0x65, 0x2, 0x1, 0x1][..]).unwrap(),
    );
}
