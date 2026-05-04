use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(0))]
pub struct S1ExtGroupB2 {
    pub b2: Option<bool>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(tag(1))]
pub struct S1ExtGroupB3 {
    pub b3: Option<bool>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[non_exhaustive]
pub struct S1 {
    pub b1: bool,
    #[rasn(extension_addition_group)]
    pub ext_group_b2: Option<S1ExtGroupB2>,
    #[rasn(extension_addition_group)]
    pub ext_group_b3: Option<S1ExtGroupB3>,
}

#[cfg(feature = "codec_per")]
#[test]
fn uper_issue505_empty_extension_group_encoded_as_absent() {
    let value_with_empty_some = S1 {
        b1: true,
        ext_group_b2: Some(S1ExtGroupB2 { b2: None }),
        ext_group_b3: Some(S1ExtGroupB3 { b3: Some(true) }),
    };
    let value_with_none = S1 {
        b1: true,
        ext_group_b2: None,
        ext_group_b3: Some(S1ExtGroupB3 { b3: Some(true) }),
    };

    let enc_empty_some =
        rasn::uper::encode(&value_with_empty_some).expect("encode Some({all None}) failed");
    let enc_none = rasn::uper::encode(&value_with_none).expect("encode None failed");

    assert_eq!(
        enc_empty_some, enc_none,
        "issue #505: Some({{all None}}) encoded as {:02X?} but None encoded as {:02X?} — \
         empty extension group must be treated as absent per X.691 §19.9",
        enc_empty_some, enc_none
    );

    let decoded: S1 = rasn::uper::decode(&enc_none).expect("decode failed");
    assert!(decoded.b1);
    assert_eq!(decoded.ext_group_b3, Some(S1ExtGroupB3 { b3: Some(true) }));
}

#[cfg(feature = "codec_per")]
#[test]
fn uper_issue505_some_all_none_equals_none() {
    let with_empty_some = S1 {
        b1: true,
        ext_group_b2: Some(S1ExtGroupB2 { b2: None }),
        ext_group_b3: None,
    };
    let with_none = S1 {
        b1: true,
        ext_group_b2: None,
        ext_group_b3: None,
    };

    let enc_some = rasn::uper::encode(&with_empty_some).expect("encode failed");
    let enc_none = rasn::uper::encode(&with_none).expect("encode failed");

    assert_eq!(
        enc_some, enc_none,
        "Some({{all None}}) must encode identically to None per X.691 §19.9, \
         got {:02X?} vs {:02X?}",
        enc_some, enc_none
    );
}
