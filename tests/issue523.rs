#[cfg(feature = "codec_per")]
use rasn::prelude::*;

#[cfg(feature = "codec_per")]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct Seq64 {
    pub root: Integer,
    #[rasn(extension_addition)]
    pub e00: Option<Integer>,
    #[rasn(extension_addition)]
    pub e01: Option<Integer>,
    #[rasn(extension_addition)]
    pub e02: Option<Integer>,
    #[rasn(extension_addition)]
    pub e03: Option<Integer>,
    #[rasn(extension_addition)]
    pub e04: Option<Integer>,
    #[rasn(extension_addition)]
    pub e05: Option<Integer>,
    #[rasn(extension_addition)]
    pub e06: Option<Integer>,
    #[rasn(extension_addition)]
    pub e07: Option<Integer>,
    #[rasn(extension_addition)]
    pub e08: Option<Integer>,
    #[rasn(extension_addition)]
    pub e09: Option<Integer>,
    #[rasn(extension_addition)]
    pub e10: Option<Integer>,
    #[rasn(extension_addition)]
    pub e11: Option<Integer>,
    #[rasn(extension_addition)]
    pub e12: Option<Integer>,
    #[rasn(extension_addition)]
    pub e13: Option<Integer>,
    #[rasn(extension_addition)]
    pub e14: Option<Integer>,
    #[rasn(extension_addition)]
    pub e15: Option<Integer>,
    #[rasn(extension_addition)]
    pub e16: Option<Integer>,
    #[rasn(extension_addition)]
    pub e17: Option<Integer>,
    #[rasn(extension_addition)]
    pub e18: Option<Integer>,
    #[rasn(extension_addition)]
    pub e19: Option<Integer>,
    #[rasn(extension_addition)]
    pub e20: Option<Integer>,
    #[rasn(extension_addition)]
    pub e21: Option<Integer>,
    #[rasn(extension_addition)]
    pub e22: Option<Integer>,
    #[rasn(extension_addition)]
    pub e23: Option<Integer>,
    #[rasn(extension_addition)]
    pub e24: Option<Integer>,
    #[rasn(extension_addition)]
    pub e25: Option<Integer>,
    #[rasn(extension_addition)]
    pub e26: Option<Integer>,
    #[rasn(extension_addition)]
    pub e27: Option<Integer>,
    #[rasn(extension_addition)]
    pub e28: Option<Integer>,
    #[rasn(extension_addition)]
    pub e29: Option<Integer>,
    #[rasn(extension_addition)]
    pub e30: Option<Integer>,
    #[rasn(extension_addition)]
    pub e31: Option<Integer>,
    #[rasn(extension_addition)]
    pub e32: Option<Integer>,
    #[rasn(extension_addition)]
    pub e33: Option<Integer>,
    #[rasn(extension_addition)]
    pub e34: Option<Integer>,
    #[rasn(extension_addition)]
    pub e35: Option<Integer>,
    #[rasn(extension_addition)]
    pub e36: Option<Integer>,
    #[rasn(extension_addition)]
    pub e37: Option<Integer>,
    #[rasn(extension_addition)]
    pub e38: Option<Integer>,
    #[rasn(extension_addition)]
    pub e39: Option<Integer>,
    #[rasn(extension_addition)]
    pub e40: Option<Integer>,
    #[rasn(extension_addition)]
    pub e41: Option<Integer>,
    #[rasn(extension_addition)]
    pub e42: Option<Integer>,
    #[rasn(extension_addition)]
    pub e43: Option<Integer>,
    #[rasn(extension_addition)]
    pub e44: Option<Integer>,
    #[rasn(extension_addition)]
    pub e45: Option<Integer>,
    #[rasn(extension_addition)]
    pub e46: Option<Integer>,
    #[rasn(extension_addition)]
    pub e47: Option<Integer>,
    #[rasn(extension_addition)]
    pub e48: Option<Integer>,
    #[rasn(extension_addition)]
    pub e49: Option<Integer>,
    #[rasn(extension_addition)]
    pub e50: Option<Integer>,
    #[rasn(extension_addition)]
    pub e51: Option<Integer>,
    #[rasn(extension_addition)]
    pub e52: Option<Integer>,
    #[rasn(extension_addition)]
    pub e53: Option<Integer>,
    #[rasn(extension_addition)]
    pub e54: Option<Integer>,
    #[rasn(extension_addition)]
    pub e55: Option<Integer>,
    #[rasn(extension_addition)]
    pub e56: Option<Integer>,
    #[rasn(extension_addition)]
    pub e57: Option<Integer>,
    #[rasn(extension_addition)]
    pub e58: Option<Integer>,
    #[rasn(extension_addition)]
    pub e59: Option<Integer>,
    #[rasn(extension_addition)]
    pub e60: Option<Integer>,
    #[rasn(extension_addition)]
    pub e61: Option<Integer>,
    #[rasn(extension_addition)]
    pub e62: Option<Integer>,
    #[rasn(extension_addition)]
    pub e63: Option<Integer>,
}

#[cfg(feature = "codec_per")]
#[test]
fn uper_issue523_normally_small_length_64_extensions() {
    let original = Seq64 {
        root: Integer::from(42),
        e00: None,
        e01: None,
        e02: None,
        e03: None,
        e04: None,
        e05: None,
        e06: None,
        e07: None,
        e08: None,
        e09: None,
        e10: None,
        e11: None,
        e12: None,
        e13: None,
        e14: None,
        e15: None,
        e16: None,
        e17: None,
        e18: None,
        e19: None,
        e20: None,
        e21: None,
        e22: None,
        e23: None,
        e24: None,
        e25: None,
        e26: None,
        e27: None,
        e28: None,
        e29: None,
        e30: None,
        e31: None,
        e32: None,
        e33: None,
        e34: None,
        e35: None,
        e36: None,
        e37: None,
        e38: None,
        e39: None,
        e40: None,
        e41: None,
        e42: None,
        e43: None,
        e44: None,
        e45: None,
        e46: None,
        e47: None,
        e48: None,
        e49: None,
        e50: None,
        e51: None,
        e52: None,
        e53: None,
        e54: None,
        e55: None,
        e56: None,
        e57: None,
        e58: None,
        e59: None,
        e60: None,
        e61: None,
        e62: None,
        e63: None,
    };
    let encoded = rasn::uper::encode(&original).expect("encode failed");
    let decoded: Seq64 = rasn::uper::decode(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
}
