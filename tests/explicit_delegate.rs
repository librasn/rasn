use rasn::prelude::*;

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(tag(explicit(application, 1)), delegate)]
pub struct ExplicitDelegate(pub Sequence);

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
pub struct Sequence { b: bool }

const _: () = assert!(Tag::const_eq(ExplicitDelegate::TAG, &Tag::new(Class::Application, 1)));

#[test]
fn der() {
    let value = ExplicitDelegate(Sequence { b: true });
    let enc = rasn::der::encode(&value).unwrap();

    assert_eq!(enc, &[0x61, 0x5, 0x30, 0x3, 0x01, 0x1, 0xFF]);
    assert_eq!(value, rasn::der::decode(&enc).unwrap());
}
