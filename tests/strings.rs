use bitvec::prelude::*;
use rasn::prelude::*;
use rasn::{aper, ber, jer, oer, uper};

#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct Hashes {
    #[rasn(size("3"))]
    pub hashed3: OctetString,
    #[rasn(size("8"))]
    pub hashed8: OctetString,
    #[rasn(size("16"))]
    pub hashed16: OctetString,
    #[rasn(size("32"))]
    pub hashed32: OctetString,
    #[rasn(size("64"))]
    pub hashed64: OctetString,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedHashes {
    pub hashed3: FixedOctetString<3>,
    pub hashed8: FixedOctetString<8>,
    pub hashed16: FixedOctetString<16>,
    pub hashed32: FixedOctetString<32>,
    pub hashed64: FixedOctetString<64>,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedFixBitString {
    pub hashed3: FixedBitString<3>,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedBitString {
    #[rasn(size("3"))]
    pub hashed3: BitString,
}

fn build_octet() -> Hashes {
    Hashes {
        hashed3: vec![0x01, 0x02, 0x03].into(),
        hashed8: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].into(),
        hashed16: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
        .into(),
        hashed32: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
        .into(),
        hashed64: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A,
            0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40,
        ]
        .into(),
    }
}
fn build_fixed_octet() -> ConstrainedHashes {
    ConstrainedHashes {
        hashed3: [0x01, 0x02, 0x03].into(),
        hashed8: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].into(),
        hashed16: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
        .into(),
        hashed32: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
        .into(),
        hashed64: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A,
            0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40,
        ]
        .into(),
    }
}

// Test whether constrained OctetString and FixedOctetString are equal
macro_rules! test_decode_eq {
    ($fn_name:ident, $codec:ident) => {
        #[test]
        fn $fn_name() {
            let items = build_octet();
            let fixed_items = build_fixed_octet();
            let encoded = $codec::encode(&items).unwrap();
            let encoded_fixed = $codec::encode(&fixed_items).unwrap();
            let decoded = $codec::decode::<Hashes>(&encoded).unwrap();
            let decoded_fixed = $codec::decode::<ConstrainedHashes>(&encoded_fixed).unwrap();
            let bits = BitString::from_bitslice(&BitVec::<u8, Msb0>::repeat(true, 3));
            let bs = ConstrainedBitString {
                hashed3: BitString::from(bits),
            };
            let bool_array = [true, true, true];
            let mut bit_array: BitArray<[u8; 3], Msb0> = BitArray::ZERO;
            for (i, &value) in bool_array.iter().enumerate() {
                bit_array.set(i, value);
            }
            let bs_fixed = ConstrainedFixBitString { hashed3: bit_array };
            let encoded_bs = $codec::encode(&bs).unwrap();
            let encoded_bs_fixed = $codec::encode(&bs_fixed).unwrap();
            let decoded_bs = $codec::decode::<ConstrainedBitString>(&encoded_bs).unwrap();
            let decoded_bs_fixed =
                $codec::decode::<ConstrainedFixBitString>(&encoded_bs_fixed).unwrap();
            assert_eq!(bs, decoded_bs);
            assert_eq!(encoded_bs, encoded_bs_fixed);
            assert_eq!(bs_fixed, decoded_bs_fixed);
            assert_eq!(items, decoded);
            assert_eq!(encoded, encoded_fixed);
            assert_eq!(fixed_items, decoded_fixed);
            assert_eq!(items.hashed3.as_ref(), decoded_fixed.hashed3.as_ref());
            assert_eq!(items.hashed8.as_ref(), decoded_fixed.hashed8.as_ref());
            assert_eq!(items.hashed16.as_ref(), decoded_fixed.hashed16.as_ref());
            assert_eq!(items.hashed32.as_ref(), decoded_fixed.hashed32.as_ref());
            assert_eq!(items.hashed64.as_ref(), decoded_fixed.hashed64.as_ref());
        }
    };
}
test_decode_eq!(test_uper_octet_eq, uper);
test_decode_eq!(test_oer_octet_eq, oer);
test_decode_eq!(test_ber_octet_eq, ber);
test_decode_eq!(test_jer_octet_eq, jer);

#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ABitString {
    #[rasn(size("0..=255"))]
    pub the_string: BitString,
}

/// Tests that valid strings are parsed and invalid strings are rejected.
#[test]
fn test_jer_bitstring_dec() {
    use bitvec::prelude::*;

    let good_cases: Vec<(&str, usize, BitVec<u8, bitvec::order::Msb0>)> = vec![
        ("", 0, bitvec::bits![u8, Msb0;].into()),
        ("00", 1, bitvec::bits![u8, Msb0; 0].into()),
        ("00", 3, bitvec::bits![u8, Msb0; 0,0,0].into()),
        ("0F", 3, bitvec::bits![u8, Msb0; 0,0,0].into()),
        ("F0", 3, bitvec::bits![u8, Msb0; 1,1,1].into()),
        ("00", 7, bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0].into()),
        ("00", 8, bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,0].into()),
        ("0F", 8, bitvec::bits![u8, Msb0; 0,0,0,0,1,1,1,1].into()),
        (
            "\\u0030\\u0030",
            8,
            bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,0].into(),
        ),
        (
            "\\u0046\\u0046",
            8,
            bitvec::bits![u8, Msb0; 1,1,1,1,1,1,1,1].into(),
        ),
    ];

    let bad_cases: Vec<(&str, usize)> = vec![
        (" ", 0),
        ("!", 0),
        ("0", 0),
        (" 0", 0),
        ("0 ", 0),
        ("0!", 0),
        ("  ", 0),
        ("00 ", 0),
        (" 00", 0),
        ("000", 0),
        ("Œ", 0),
        ("ŒŒ", 0),
        ("ŒŒŒ", 0),
        ("ABCDEFG", 0),
        (" ABCDEF", 0),
        ("\u{0000}", 0),
        ("\u{FFFF}", 0),
        ("\u{0123}", 0),
        ("\u{30}", 0),
        ("\\u0030", 0),
        ("\\u202E\\u0030\\u0030", 0),
        ("⣐⡄", 0),
        ("😎", 0),
        ("🙈🙉🙊", 0),
        ("", 1),
        ("", 8),
        ("00", 0),
        ("00", 10),
        ("00", 16),
        ("00", 16384),
        ("0000", 0),
        ("0000", 8),
        ("0000", 17),
    ];

    for (case, length, bits) in good_cases {
        let json = format!("{{\"the_string\":{{\"length\":{length},\"value\":\"{case}\"}}}}");
        let expected = ABitString { the_string: bits };
        let decoded = jer::decode::<ABitString>(&json);
        if let Err(e) = decoded {
            panic!("should have decoded case \"{case}\" fine: {e:?}");
        }
        assert_eq!(decoded.unwrap(), expected);
    }

    for (case, length) in bad_cases {
        let json = format!("{{\"the_string\":{{\"length\":{length},\"value\":\"{case}\"}}}}");
        let decoded = jer::decode::<ABitString>(&json);
        if let Ok(decoded) = decoded {
            panic!(
                "should have rejected case \"{case}\", but decoded: {decoded:?} (length {})",
                decoded.the_string.len()
            );
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct AnOctetString {
    #[rasn(size("0..=255"))]
    pub the_string: OctetString,
}

/// Tests that valid strings are parsed and invalid strings are rejected.
#[test]
fn test_jer_octetstring_dec() {
    let good_cases: Vec<&[u8]> = vec![
        &[],
        &[0x00; 1],
        &[0x00; 2],
        &[0x0F; 1],
        &[0xFF; 1],
        &[0xFF; 2],
        &[0x00; 10],
        &[0x00; 100],
        &[0x00; 200],
        &[0x01, 0x23],
        &[0xAB, 0xCD],
        &[0xAB, 0xCD, 0xEF],
        &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
        &[0x00; 255],
        &[0x0F; 255],
        &[0xFF; 255],
        &[0x00; 256],
        &[0x0F; 256],
        &[0xFF; 256],
        &[0x00; 16384],
        &[0x0F; 16384],
        &[0xFF; 16384],
    ];

    let special_cases: Vec<(&str, &[u8])> =
        vec![("\\u0030\\u0030", &[0x00]), ("\\u0046\\u0046", &[0xFF])];

    let bad_cases = vec![
        " ",
        "!",
        "0",
        " 0",
        "0 ",
        "0!",
        "  ",
        "000",
        "Œ",
        "ŒŒ",
        "ŒŒŒ",
        "ABCDEFG",
        " ABCDEF",
        "\u{0000}",
        "\u{FFFF}",
        "\u{0123}",
        "\u{30}",
        "\\u0030",
        "\\u202E\\u0030\\u0030",
        "⣐⡄",
        "😎",
        "🙈🙉🙊",
    ];

    for case in good_cases {
        let upper_hex = case
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<String>>()
            .join("");
        let json = format!("{{\"the_string\":\"{upper_hex}\"}}");
        let expected = AnOctetString {
            the_string: case.into(),
        };
        let decoded = jer::decode::<AnOctetString>(&json);
        if let Err(e) = decoded {
            panic!("should have decoded case \"{upper_hex}\" fine: {e:?}");
        }
        assert_eq!(decoded.unwrap(), expected);
    }

    for (case, expected) in special_cases {
        let json = format!("{{\"the_string\":\"{case}\"}}");
        let expected = AnOctetString {
            the_string: expected.into(),
        };
        let decoded = jer::decode::<AnOctetString>(&json);
        if let Err(e) = decoded {
            panic!("should have decoded case \"{case}\" fine: {e:?}");
        }
        assert_eq!(decoded.unwrap(), expected);
    }

    for case in bad_cases {
        let json = format!("{{\"the_string\":\"{case}\"}}");
        let decoded = jer::decode::<AnOctetString>(&json);
        if let Ok(decoded) = decoded {
            panic!(
                "should have rejected case \"{case}\", but decoded: {decoded:?} (length {})",
                decoded.the_string.len()
            );
        }
    }
}

// Tests that OctetStrings are encoded and decoded correctly (APER, UPER).
const BYTE_ARRAYS: &[&[u8]] = &[
    &[],
    &[0x00; 1],
    &[0x00; 2],
    &[0x0F; 1],
    &[0xFF; 1],
    &[0xFF; 2],
    &[0x00; 10],
    &[0x00; 100],
    &[0x00; 128],
    &[0x00; 200],
    &[0x01, 0x23],
    &[0xAB, 0xCD],
    &[0xAB, 0xCD, 0xEF],
    &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
    &[0x00; 255],
    &[0x0F; 255],
    &[0xFF; 255],
    &[0x00; 256],
    &[0x0F; 256],
    &[0xFF; 256],
    &[0x00; 16383],
    &[0x0F; 16383],
    &[0xFF; 16383],
];

#[test]
fn test_per_encode_octet_string() {
    for case in BYTE_ARRAYS {
        let length = case.len(); // number of bytes
        let mut buf_expected: Vec<u8> = Vec::new();
        if length < 128 {
            // X.691, 11.9.a)
            buf_expected.push(length as u8);
        } else if length < 16384 {
            // X.691, 11.9.b)
            let length = (length as u16).to_be_bytes();
            buf_expected.push(0b10000000 | length[0]);
            buf_expected.push(length[1]);
        } else {
            // X.691, 11.9.c)
            todo!("implement chunk generation");
        }
        buf_expected.extend_from_slice(case);

        let bytes = OctetString::from_static(case);
        assert_eq!(buf_expected, aper::encode::<OctetString>(&bytes).unwrap());
        assert_eq!(buf_expected, uper::encode::<OctetString>(&bytes).unwrap());

        assert_eq!(*case, &*aper::decode::<OctetString>(&buf_expected).unwrap());
        assert_eq!(*case, &*uper::decode::<OctetString>(&buf_expected).unwrap());
    }
}

// Tests that UTF8Strings are encoded and decoded correctly (APER, UPER).
const UTF8_STRINGS: &[&str] = &[
    "",
    "Hello World!",
    "Hello World! 🌍",
    "こんにちは世界！",
    "你好世界！",
    "안녕하세요!",
    "مرحبا بالعالم!",
    "હેલો વિશ્વ!",
    "привіт світ!",
    " ",
    "!",
    "0",
    " 0",
    "0 ",
    "0!",
    "  ",
    "000",
    "Œ",
    "ŒŒ",
    "ŒŒŒ",
    "ABCDEFG",
    " ABCDEF",
    "åäö",
    "\0",
    "\n",
    "\r\nASN.1",
    "\u{0000}",
    "\u{0001}",
    "\u{FFFF}",
    "\u{0123}",
    "\u{30}",
    "\\u0030",
    "\\u202E\\u0030\\u0030",
    "⣐⡄",
    "😎",
    "🙈🙉🙊",
    "👭👩🏻‍🤝‍👨🏾🧑🏿‍🤝‍🧑🏼",
];

#[test]
fn test_per_encode_utf8_string() {
    for case in UTF8_STRINGS {
        let case = case.to_string();
        // The X.691 spec, chapter 11.9, says determinant should be the number
        // of characters, but for UTF-8 this is a non-trivial operation and
        // dependant on the supported Unicode release. Actual implementations
        // seem to interpret the spec as "number of octets" instead - which is
        // reasonable (see `asn1tools` for example).
        let length = case.len(); // number of bytes
        let mut buf_expected: Vec<u8> = Vec::new();
        if length < 128 {
            // X.691, 11.9.a)
            buf_expected.push(length as u8);
        } else if length < 16384 {
            // X.691, 11.9.b)
            let length = length.to_le_bytes();
            buf_expected.push(0b10000000 | length[1]);
            buf_expected.push(length[0]);
        } else {
            // X.691, 11.9.c)
            todo!("implement chunk generation");
        }
        buf_expected.extend_from_slice(case.as_bytes());

        assert_eq!(buf_expected, aper::encode::<Utf8String>(&case).unwrap());
        assert_eq!(buf_expected, uper::encode::<Utf8String>(&case).unwrap());

        assert_eq!(case, aper::decode::<Utf8String>(&buf_expected).unwrap());
        assert_eq!(case, uper::decode::<Utf8String>(&buf_expected).unwrap());
    }
}
