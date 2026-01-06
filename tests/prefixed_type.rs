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

#[test]
fn test_issue_519_automatic_tagging_for_tagged_choice_round_trip() {
    // https://github.com/librasn/rasn/issues/519

    /*
        With following ASN.1 definitions, BER encoding of TC1 and TC2 should be identical,
        and alternatives in a choice (`a` and `b`) are automatically tagged.

        TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
            --untagged choice
            C1 ::= CHOICE { a INTEGER, b BOOLEAN }

            --tagged choice. This is explicit tagging. (see ITU-T X.680 section 31.2.7 clause c)
            TC1  ::= [4] C1
            --another form of tagged choice
            TC2  ::= [4] CHOICE { a INTEGER, b BOOLEAN }
        END
    */

    use rasn::prelude::*;

    #[doc = "untagged choice"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum C1 {
        A(Integer),
        B(bool),
    }
    #[doc = "tagged choice. This is explicit tagging. (see ITU-T X.680 section 31.2.7 clause c)"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, tag(explicit(context, 4)))]
    pub struct TC1(pub C1);
    #[doc = "another form of tagged choice"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, tag(explicit(context, 4)), automatic_tags)]
    pub enum TC2 {
        A(Integer),
        B(bool),
    }

    let choice = C1::A(0x55.into());
    let tc1 = TC1(choice.clone());
    let tc2 = TC2::A(0x55.into());

    let choice_enc = rasn::ber::encode(&choice).unwrap();
    let tc1_enc = rasn::ber::encode(&tc1).unwrap();
    let tc2_enc = rasn::ber::encode(&tc2).unwrap();

    assert_eq!(choice_enc, vec![0x80, 0x01, 0x55]);
    assert_eq!(tc1_enc, vec![0xa4, 0x03, 0x80, 0x01, 0x55]);
    assert_eq!(tc2_enc, tc1_enc); // should be identical to tc1_enc

    let tc1_de = rasn::ber::decode::<TC1>(&tc1_enc).unwrap();
    let tc2_de = rasn::ber::decode::<TC2>(&tc2_enc).unwrap();
    assert_eq!(tc1, tc1_de);
    assert_eq!(tc2, tc2_de);
}
