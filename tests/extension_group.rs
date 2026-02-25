pub mod extension_addition_group {
    extern crate alloc;
    use rasn::prelude::*;
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct S1ExtGroupB2 {
        pub b2: Option<bool>,
    }
    impl S1ExtGroupB2 {
        pub fn new(b2: Option<bool>) -> Self {
            Self { b2 }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct S1ExtGroupB3 {
        pub b3: Option<bool>,
    }
    impl S1ExtGroupB3 {
        pub fn new(b3: Option<bool>) -> Self {
            Self { b3 }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1 {
        pub b1: bool,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_b2: Option<S1ExtGroupB2>,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_b3: Option<S1ExtGroupB3>,
    }
    impl S1 {
        pub fn new(
            b1: bool,
            ext_group_b2: Option<S1ExtGroupB2>,
            ext_group_b3: Option<S1ExtGroupB3>,
        ) -> Self {
            Self {
                b1,
                ext_group_b2,
                ext_group_b3,
            }
        }
    }
}

const SAMPLE_S1: extension_addition_group::S1 = extension_addition_group::S1 {
    b1: true,
    ext_group_b2: None,
    ext_group_b3: Some(extension_addition_group::S1ExtGroupB3 { b3: Some(true) }),
};

macro_rules! round_trip {
    ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
        let value: $typ = $value;
        let expected: &[u8] = $expected;
        let actual_encoding = rasn::$codec::encode(&value).unwrap();

        pretty_assertions::assert_eq!(&*actual_encoding, expected);

        let decoded_value = rasn::$codec::decode::<$typ>(&actual_encoding);
        match decoded_value {
            Ok(decoded) => {
                pretty_assertions::assert_eq!(value, decoded);
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }};
}

#[test]
fn extension_group_roundtrip_aper() {
    let encoded = &[0xc0, 0xa0, 0x01, 0xc0];
    round_trip!(aper, extension_addition_group::S1, SAMPLE_S1, encoded);
}

#[test]
fn extension_group_roundtrip_uper() {
    let encoded_correct = &[0xc0, 0xa0, 0x38, 0x00];
    round_trip!(
        uper,
        extension_addition_group::S1,
        SAMPLE_S1,
        encoded_correct
    );
}

#[test]
fn extension_group_roundtrip_ber() {
    let encoded = &[0x30, 0x06, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff];
    round_trip!(ber, extension_addition_group::S1, SAMPLE_S1, encoded);
}

#[test]
fn extension_group_roundtrip_cer() {
    let encoded = &[0x30, 0x80, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff, 0x00, 0x00];
    round_trip!(cer, extension_addition_group::S1, SAMPLE_S1, encoded);
}

#[test]
fn extension_group_roundtrip_coer() {
    let encoded = &[0x80, 0xff, 0x02, 0x06, 0x40, 0x02, 0x80, 0xff];
    round_trip!(coer, extension_addition_group::S1, SAMPLE_S1, encoded);
}

#[test]
fn extension_group_roundtrip_der() {
    let encoded = &[0x30, 0x06, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff];
    round_trip!(der, extension_addition_group::S1, SAMPLE_S1, encoded);
}

#[test]
fn extension_group_roundtrip_jer() {
    let expected = "{\"b1\":true,\"b3\":true}";
    let encoded = rasn::jer::encode(&SAMPLE_S1).unwrap();
    pretty_assertions::assert_eq!(expected, encoded);
    let decoded = rasn::jer::decode::<extension_addition_group::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_oer() {
    let encoded = &[0x80, 0xff, 0x02, 0x06, 0x40, 0x02, 0x80, 0xff];
    round_trip!(oer, extension_addition_group::S1, SAMPLE_S1, encoded);
}
