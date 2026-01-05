#[test]
fn test_ber_encoding_of_tagged_type_round_trip() {
    //  -- BER encoding example from ITU-T X.690 clause 8.14:
    //
    //  With ASN.1 type definitions (in an explicit tagging environment) of:
    //  TestModule DEFINITIONS EXPLICIT TAGS::= BEGIN
    //      Type1 ::= VisibleString
    //      Type2 ::= [APPLICATION 3] IMPLICIT Type1
    //      Type3 ::= [2] Type2
    //      Type4 ::= [APPLICATION 7] IMPLICIT Type3
    //      Type5 ::= [2] IMPLICIT Type2
    //  END
    //  a value of "Jones" is encoded as follows ...

    use rasn::prelude::*;

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Type1(pub VisibleString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(application, 3))]
    pub struct Type2(pub Type1);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(explicit(context, 2)))]
    pub struct Type3(pub Type2);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(application, 7))]
    pub struct Type4(pub Type3);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(context, 2))]
    pub struct Type5(pub Type2);

    let type1 = Type1(VisibleString::from_iso646_bytes(b"Jones").unwrap());
    let type2 = Type2(type1.clone());
    let type3 = Type3(type2.clone());
    let type4 = Type4(type3.clone());
    let type5 = Type5(type2.clone());

    let type1_ber = rasn::ber::encode(&type1).unwrap();
    let type2_ber = rasn::ber::encode(&type2).unwrap();
    let type3_ber = rasn::ber::encode(&type3).unwrap();
    let type4_ber = rasn::ber::encode(&type4).unwrap();
    let type5_ber = rasn::ber::encode(&type5).unwrap();

    assert_eq!(type1_ber, vec![0x1A, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73_u8]);
    assert_eq!(type2_ber, vec![0x43, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73_u8]);
    assert_eq!(
        type3_ber,
        vec![0xa2, 0x07, 0x43, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73_u8]
    );
    assert_eq!(
        type4_ber,
        vec![0x67, 0x07, 0x43, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73_u8]
    );
    assert_eq!(type5_ber, vec![0x82, 0x05, 0x4A, 0x6F, 0x6E, 0x65, 0x73_u8]);

    assert_eq!(type1, rasn::ber::decode::<Type1>(&type1_ber).unwrap());
    assert_eq!(type2, rasn::ber::decode::<Type2>(&type2_ber).unwrap());
    assert_eq!(type3, rasn::ber::decode::<Type3>(&type3_ber).unwrap());
    assert_eq!(type4, rasn::ber::decode::<Type4>(&type4_ber).unwrap());
    assert_eq!(type5, rasn::ber::decode::<Type5>(&type5_ber).unwrap());
}
