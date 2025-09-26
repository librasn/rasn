use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    ItemList ::= SEQUENCE (SIZE(0..65535)) OF INTEGER
END
*/

#[doc = " Anonymous SEQUENCE OF member "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "INTEGER")]
pub struct AnonymousItemList(pub Integer);
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=65535"))]
pub struct ItemList(pub SequenceOf<AnonymousItemList>);

#[test]
fn test_minimal_64k_sequence_round_trip() {
    // Round-trip a small sequence to ensure decoder accepts 16-bit constrained length for span 65536
    let original = ItemList(SequenceOf::from(vec![AnonymousItemList(Integer::from(7))]));
    let encoded = rasn::uper::encode(&original).expect("encode");
    // First two bytes should be 0x0001 length determinant (16-bit field)
    assert_eq!(&encoded[0..2], &[0x00, 0x01]);
    let decoded: ItemList = rasn::uper::decode(&encoded).expect("decode");
    assert_eq!(decoded, original);
}
