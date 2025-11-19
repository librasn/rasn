use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    A ::= SEQUENCE {
	i			OCTET STRING (SIZE(3)),
	t			OCTET STRING (SIZE(2)),
	e			INTEGER OPTIONAL,
	...
}
END
*/

#[doc = "SEQUENCE A"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct A {
    pub i: FixedOctetString<3>,
    pub t: FixedOctetString<2>,
    pub e: Option<Integer>,
}

#[test]
fn test_sequence_a() {
    // Encoded byte array
    let encoded_value = [0x00u8, 0x19, 0xf0, 0x45, 0x00, 0x01];
    // Decoded ASN.1 structure
    let original = A {
        i: FixedOctetString::from([0x19, 0xf0, 0x45]),
        t: FixedOctetString::from([0x00, 0x01]),
        e: None,
    };
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("Encoded: {:x?}", encoded);
    assert_eq!(&encoded_value, &encoded[..]);
    let decoded: A = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(decoded, original);
}
