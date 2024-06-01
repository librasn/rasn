use num_bigint::BigInt;
use rasn::prelude::*;

macro_rules! test_large_encoding {
    ($codec:ident, $integer:expr, $number:expr) => {
        let result = rasn::$codec::encode(&$integer).unwrap();
        assert!(result.len() > 128);
        let decoded = rasn::$codec::decode::<Integer>(&result).unwrap();
        match decoded {
            Integer::Big(value) => assert!(value == $number),
            _ => panic!("Expected BigInt integer"),
        }
    };
}

macro_rules! test_primitive_encoding {
    ($codec:ident, $integer:expr, $number:expr) => {
        let result = rasn::$codec::encode(&$integer).unwrap();
        assert!(result.len() > 4);
        let decoded = rasn::$codec::decode::<Integer>(&result).unwrap();
        match decoded {
            Integer::Primitive(value) => assert!(value == $number),
            _ => panic!("Expected primitive integer"),
        }
    };
}

#[test]
fn test_large_int() {
    // Signed integer with byte length of 128
    // Needs long form to represent in most cases
    let number: BigInt = BigInt::from(256u32).pow(127u32) - 1u32;
    let integer: Integer = number.clone().into();
    test_large_encoding!(ber, integer, number);
    test_large_encoding!(cer, integer, number);
    test_large_encoding!(der, integer, number);
    test_large_encoding!(oer, integer, number);
    test_large_encoding!(coer, integer, number);
    test_large_encoding!(uper, integer, number);
    test_large_encoding!(aper, integer, number);
}

#[test]
fn test_primititive_int() {
    let number: PrimitiveInteger = i32::MAX.into();
    let integer: Integer = number.into();
    test_primitive_encoding!(ber, integer, number);
    test_primitive_encoding!(cer, integer, number);
    test_primitive_encoding!(der, integer, number);
    test_primitive_encoding!(oer, integer, number);
    test_primitive_encoding!(coer, integer, number);
    test_primitive_encoding!(uper, integer, number);
    test_primitive_encoding!(aper, integer, number);
}
