#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod my_module {
    extern crate alloc;
    use core::borrow::Borrow;
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

const SAMPLE_S1: my_module::S1 = my_module::S1 {
    b1: true,
    ext_group_b2: None,
    ext_group_b3: Some(my_module::S1ExtGroupB3 { b3: Some(true) }),
};

#[test]
fn extension_group_decode_aper() {
    let encoded = &[
        0xc0, 0xa0, 0x00, 0x01, 0xc0
    ];

    let decoded = rasn::aper::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_aper() {
    let encoded = rasn::aper::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::aper::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_ber() {
    let encoded = &[
        0x30, 0x06, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff
    ];
    let decoded = rasn::ber::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_ber() {
    let encoded = rasn::ber::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::ber::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_cer() {
    let encoded = &[
        0x30, 0x80, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff, 0x00, 0x00
    ];
    let decoded = rasn::cer::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_cer() {
    let encoded = rasn::cer::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::cer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_coer() {
    let encoded = &[
        0x80, 0xff, 0x02, 0x06, 0x40, 0x02, 0x80, 0xff
    ];
    let decoded = rasn::coer::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_coer() {
    let encoded = rasn::coer::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::coer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_der() {
    let encoded = &[
        0x30, 0x06, 0x80, 0x01, 0xff, 0x82, 0x01, 0xff
    ];
    let decoded = rasn::der::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_der() {
    let encoded = rasn::der::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::der::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_jer() {
    let encoded = "{\"b1\":true,\"b3\":true}";
    let decoded = rasn::jer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_jer() {
    let encoded = rasn::jer::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::jer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_oer() {
    let encoded = &[
        0x80, 0xff, 0x02, 0x06, 0x40, 0x02, 0x80, 0xff
    ];
    let decoded = rasn::oer::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_oer() {
    let encoded = rasn::oer::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::oer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_decode_uper() {
    let encoded = &[
        0xc0, 0xa0, 0x38, 0x00
    ];
    let decoded = rasn::uper::decode::<my_module::S1>(encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_uper() {
    let encoded = rasn::uper::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::uper::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}

#[test]
fn extension_group_roundtrip_xer() {
    let encoded = rasn::xer::encode(&SAMPLE_S1).unwrap();
    let decoded = rasn::xer::decode::<my_module::S1>(&encoded).unwrap();
    pretty_assertions::assert_eq!(SAMPLE_S1, decoded);
}
