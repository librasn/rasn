//! # Aligned Packed Encoding Rules
//!
//! Codec functions for APER, rasn provides a "basic" decoder, and canonical encoder.
//! This means that users are able decode any valid APER value, and that rasn's
//! encoding will always produce the same output for the same value.
use crate::types::Constraints;

pub use super::per::*;

/// Attempts to decode `T` from `input` using APER-BASIC.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    crate::per::decode(de::DecoderOptions::aligned(), input)
}
/// Attempts to decode `T` from `input` using APER-BASIC. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid APER-BASIC encoding specific to the expected type.
pub fn decode_with_remainder<T: crate::Decode>(
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    crate::per::decode_with_remainder(de::DecoderOptions::aligned(), input)
}

/// Attempts to encode `value` to APER-CANONICAL.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    crate::per::encode(enc::EncoderOptions::aligned(), value)
}

/// Encodes `value` to APER-CANONICAL into an existing `buffer`, reusing its allocation.
/// The buffer is cleared before encoding.
/// # Errors
/// Returns error specific to APER encoder if encoding is not possible.
pub fn encode_buf<T: crate::Encode>(
    value: &T,
    buffer: &mut alloc::vec::Vec<u8>,
) -> Result<(), crate::error::EncodeError> {
    crate::per::encode_buf(enc::EncoderOptions::aligned(), value, buffer)
}

/// Attempts to decode `T` from `input` using APER-BASIC.
pub fn decode_with_constraints<T: crate::Decode>(
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    crate::per::decode_with_constraints(de::DecoderOptions::aligned(), constraints, input)
}

/// Attempts to encode `value` to APER-CANONICAL.
pub fn encode_with_constraints<T: crate::Encode>(
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    crate::per::encode_with_constraints(enc::EncoderOptions::aligned(), constraints, value)
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        types::{constraints::*, *},
    };

    #[test]
    fn bitstring() {
        use bitvec::prelude::*;
        // B ::= BIT STRING (SIZE (9))
        // C ::= BIT STRING (SIZE (5..7))

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct D {
            a: bool,
            b: BitString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct E {
            a: bool,
            #[rasn(size(1))]
            b: BitString,
            #[rasn(size(16))]
            c: BitString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct G {
            a: BitString,
            b: bool,
        }

        // H ::= SEQUENCE SIZE (0..2) OF BIT STRING (SIZE(1..255))
        // I ::= SEQUENCE SIZE (0..2) OF BIT STRING (SIZE(1..256))
        // J ::= SEQUENCE SIZE (0..2) OF BIT STRING (SIZE(2..256))
        // K ::= SEQUENCE SIZE (0..2) OF BIT STRING (SIZE(2..257))
        // L ::= BIT STRING (SIZE (1..160, ...))

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct M {
            a: bool,
            #[rasn(size("1..=160", extensible))]
            b: BitString,
        }

        // N ::= BIT STRING (SIZE(0..65535))
        // O ::= BIT STRING (SIZE(0..65536))

        round_trip!(
            aper,
            BitString,
            bitvec::bitvec![u8, Msb0; 0, 1, 0, 0],
            &[0x04, 0x40]
        );
        // round_trip!(
        //     aper,
        //     BitString,
        //     BitString::from_vec({
        //         let mut bytes = vec![0x55; 300];
        //         bytes[299] = 0x54;
        //         bytes
        //     }),
        //     &*{
        //         let mut bytes = vec![0x89, 0x5f];
        //         bytes.extend([0x55; 299]);
        //         bytes.push(0x54);
        //         bytes
        //     }
        // );
        round_trip!(
            aper,
            BitString,
            BitString::from_vec([0x55; 2048].into()),
            &*{
                let mut bytes = vec![0xc1];
                bytes.extend([0x55; 2048]);
                bytes.push(0x00);
                bytes
            }
        );
        // round_trip!(aper, B, (b'\x12\x80', 9), b'\x12\x80');
        // round_trip!(aper, C, (b'\x34', 6), b'\x40\x34');
        // round_trip!(aper, D, {'a': True, 'b': (b'\x40', 4)}, b'\x80\x04\x40');
        // round_trip!(aper, E, {'a': True, 'b': (b'\x80', 1), 'c': (b'\x7f\x01', 16)}, b'\xdf\xc0\x40');
        // round_trip!(aper, F, (b'\x80', 1), b'\x01\x80');
        // round_trip!(aper, F, (b'\xe0', 3), b'\x03\xe0');
        // round_trip!(aper, F, (b'\x01', 8), b'\x08\x01');
        // round_trip!(aper, G, {'a': (b'\x80', 2), 'b': True}, b'\x02\xa0');
        // round_trip!(aper, G, {'a': (b'', 0), 'b': True}, b'\x00\x80');
        // round_trip!(aper, H, [(b'\x40', 2)], b'\x40\x40\x40');
        // round_trip!(aper, I, [(b'\x40', 2)], b'\x40\x01\x40');
        // round_trip!(aper, J, [(b'\x40', 2)], b'\x40\x00\x40');
        // round_trip!(aper, K, [(b'\x40', 2)], b'\x40\x00\x40');
        // round_trip!(aper, L, (b'\x80', 1), b'\x00\x00\x80');
        // round_trip!(aper, M, {'a': True, 'b': (b'\xe0', 3)}, b'\x80\x80\xe0');
        // round_trip!(aper, N, (b'', 0), b'\x00\x00');
        // round_trip!(aper, O, (b'', 0), b'\x00');
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn integer() {
        type B = ConstrainedInteger<5, 99>;

        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct C {
            a: bool,
            b: Integer,
            c: bool,
            #[rasn(value("-10..=400"))]
            d: Integer,
        }

        type D = ConstrainedInteger<0, 254>;
        type E = ConstrainedInteger<0, 255>;
        type F = ConstrainedInteger<0, 256>;
        type G = ConstrainedInteger<0, 65535>;
        type H = ConstrainedInteger<0, 65536>;
        type I = ConstrainedInteger<0, 10_000_000_000>;

        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct J {
            a: bool,
            #[rasn(value("0..=254"))]
            b: Integer,
            #[rasn(value("0..=255"))]
            c: Integer,
            d: bool,
            #[rasn(value("0..=256"))]
            e: Integer,
        }

        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct L {
            #[rasn(value("7..=7"))]
            a: Integer,
        }

        type N = ConstrainedInteger<0, 65535>;
        type O = ConstrainedInteger<0, 65536>;
        type P = ConstrainedInteger<0, 2_147_483_647>;
        type Q = ConstrainedInteger<0, 4_294_967_295>;
        type R = ConstrainedInteger<0, 4_294_967_296>;

        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct S {
            a: bool,
            #[rasn(value("-10000..=704000000000000001"))]
            b: Integer,
            c: bool,
        }

        round_trip!(aper, Integer, 32768.into(), &[0x03, 0x00, 0x80, 0x00]);
        round_trip!(aper, Integer, 32767.into(), &[0x02, 0x7f, 0xff]);
        round_trip!(aper, Integer, 256.into(), &[0x02, 0x01, 0x00]);
        round_trip!(aper, Integer, 255.into(), &[0x02, 0x00, 0xff]);
        round_trip!(aper, Integer, 128.into(), &[0x02, 0x00, 0x80]);
        round_trip!(aper, Integer, 127.into(), &[0x01, 0x7f]);
        round_trip!(aper, Integer, 1.into(), &[0x01, 0x01]);
        round_trip!(aper, Integer, 0.into(), &[0x01, 0x00]);
        round_trip!(aper, Integer, (-1).into(), &[0x01, 0xff]);
        round_trip!(aper, Integer, (-128).into(), &[0x01, 0x80]);
        round_trip!(aper, Integer, (-129).into(), &[0x02, 0xff, 0x7f]);
        round_trip!(aper, Integer, (-256).into(), &[0x02, 0xff, 0x00]);
        round_trip!(aper, Integer, (-32768).into(), &[0x02, 0x80, 0x00]);
        round_trip!(aper, Integer, (-32769).into(), &[0x03, 0xff, 0x7f, 0xff]);
        round_trip!(aper, B, B::new(5), &[0x00]);
        round_trip!(aper, B, B::new(6), &[0x02]);
        round_trip!(aper, B, B::new(99), &[0xbc]);
        round_trip!(
            aper,
            C,
            C {
                a: true,
                b: Integer::from(43_554_344_223_i64),
                c: false,
                d: Integer::from(-9)
            },
            &[0x80, 0x05, 0x0a, 0x24, 0x0a, 0x8d, 0x1f, 0x00, 0x00, 0x01]
        );
        round_trip!(aper, D, D::new(253), &[0xfd]);
        round_trip!(aper, E, E::new(253), &[0xfd]);
        round_trip!(aper, F, F::new(253), &[0x00, 0xfd]);
        round_trip!(aper, G, G::new(253), &[0x00, 0xfd]);
        round_trip!(aper, H, H::new(253), &[0x00, 0xfd]);
        round_trip!(aper, H, H::new(256), &[0x40, 0x01, 0x00]);
        round_trip!(aper, H, H::new(65536), &[0x80, 0x01, 0x00, 0x00]);
        round_trip!(aper, I, I::new(0), &[0x00, 0x00]);
        round_trip!(aper, I, I::new(1), &[0x00, 0x01]);
        round_trip!(
            aper,
            I,
            I::new(10_000_000_000_i64),
            &[0x80, 0x02, 0x54, 0x0b, 0xe4, 0x00]
        );
        round_trip!(
            aper,
            J,
            J {
                a: false,
                b: Integer::from(253),
                c: Integer::from(253),
                d: false,
                e: Integer::from(253)
            },
            &[0x7e, 0x80, 0xfd, 0x00, 0x00, 0xfd]
        );
        round_trip!(
            aper,
            L,
            L {
                a: Integer::from(7)
            },
            &[]
        );
        // round_trip!(aper, M, 103.into(), &[0x80, 0x01, 0x67]);
        round_trip!(aper, N, N::new(1), &[0x00, 0x01]);
        round_trip!(aper, N, N::new(255), &[0x00, 0xff]);
        round_trip!(aper, N, N::new(256), &[0x01, 0x00]);
        round_trip!(aper, N, N::new(65535), &[0xff, 0xff]);
        round_trip!(aper, O, O::new(1), &[0x00, 0x01]);
        round_trip!(aper, O, O::new(255), &[0x00, 0xff]);
        round_trip!(aper, O, O::new(256), &[0x40, 0x01, 0x00]);
        round_trip!(aper, O, O::new(65535), &[0x40, 0xff, 0xff]);
        round_trip!(aper, O, O::new(65536), &[0x80, 0x01, 0x00, 0x00]);
        round_trip!(aper, P, P::new(1), &[0x00, 0x01]);
        round_trip!(aper, P, P::new(255), &[0x00, 0xff]);
        round_trip!(aper, P, P::new(256), &[0x40, 0x01, 0x00]);
        round_trip!(aper, P, P::new(65535), &[0x40, 0xff, 0xff]);
        round_trip!(aper, P, P::new(65536), &[0x80, 0x01, 0x00, 0x00]);
        round_trip!(aper, P, P::new(16_777_215), &[0x80, 0xff, 0xff, 0xff]);
        round_trip!(aper, P, P::new(16_777_216), &[0xc0, 0x01, 0x00, 0x00, 0x00]);
        round_trip!(
            aper,
            P,
            P::new(100_000_000),
            &[0xc0, 0x05, 0xf5, 0xe1, 0x00]
        );
        round_trip!(
            aper,
            Q,
            Q::new(4_294_967_295_u64),
            &[0xc0, 0xff, 0xff, 0xff, 0xff]
        );
        round_trip!(
            aper,
            R,
            R::new(4_294_967_296_u64),
            &[0x80, 0x01, 0x00, 0x00, 0x00, 0x00]
        );
        round_trip!(
            aper,
            S,
            S {
                a: true,
                b: 0.into(),
                c: true
            },
            &[0x90, 0x27, 0x10, 0x80]
        );
    }

    #[test]
    fn visible_string() {
        // B ::= VisibleString (SIZE (5))
        // C ::= VisibleString (SIZE (19..1000))
        // D ::= SEQUENCE {
        //   a BOOLEAN,
        //   b VisibleString (SIZE (1))
        // }
        // H ::= SEQUENCE {
        //   a BOOLEAN,
        //   b VisibleString (SIZE (0..2))
        // }
        // I ::= VisibleString (FROM (\a\..\z\)) (SIZE (1..255))

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct D {
            a: bool,
            #[rasn(size(1))]
            b: VisibleString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct E {
            a: bool,
            #[rasn(size(2))]
            b: VisibleString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct F {
            a: bool,
            #[rasn(size(3))]
            b: VisibleString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct G {
            a: bool,
            #[rasn(size("0..=1"))]
            b: VisibleString,
        }

        #[allow(dead_code)]
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct H {
            a: bool,
            #[rasn(size("0..=2"))]
            b: VisibleString,
        }
        // J ::= VisibleString (FROM (\a\))
        // K ::= VisibleString (FROM (\a\..\a\))

        // round_trip_with_constraints!(
        //     aper,
        //     VisibleString,
        //     Constraints::new(&[Constraint::Size(Size::new(Bounded::new(19, 133)).into())]),
        //     VisibleString::try_from("HejHoppHappHippAbcde").unwrap(),
        //     &[
        //         0x02, 0x48, 0x65, 0x6a, 0x48, 0x6f, 0x70, 0x70, 0x48, 0x61, 0x70, 0x70, 0x48, 0x69,
        //         0x70, 0x70, 0x41, 0x62, 0x63, 0x64, 0x65
        //     ]
        // );
        // round_trip_with_constraints!(
        //     aper,
        //     VisibleString,
        //     Constraints::new(&[Constraint::Size(Size::new(Bounded::Single(5)).into())]),
        //     VisibleString::try_from("Hejaa").unwrap(),
        //     &[0x48, 0x65, 0x6a, 0x61, 0x61]
        // );
        // round_trip_with_constraints!(
        //     aper,
        //     VisibleString,
        //     Constraints::new(&[Constraint::Size(Size::new(Bounded::new(19, 1000)).into())]),
        //     VisibleString::try_from(str::repeat("HejHoppHappHippAbcde", 17)).unwrap(),
        //     &*{
        //         let mut bytes = vec![0x01, 0x41];
        //         for _ in 0..17 {
        //             bytes.extend([
        //                 0x48, 0x65, 0x6a, 0x48, 0x6f, 0x70, 0x70, 0x48, 0x61,
        //                 0x70, 0x70, 0x48, 0x69, 0x70, 0x70, 0x41, 0x62, 0x63,
        //                 0x64, 0x65
        //             ]);
        //         }
        //         bytes
        //     }
        // );
        // round_trip!(aper, D, D { a: true, b: "1".try_into().unwrap() }, &[0x98, 0x80]);
        // round_trip!(aper, E, E { a: true, b: "12".try_into().unwrap() }, &[0x98, 0x99, 0x00]);
        // round_trip!(aper, F, F { a: true, b: "123".try_into().unwrap() }, &[0x80, 0x31, 0x32, 0x33]);
        // round_trip!(aper, G, G { a: true, b: "1".try_into().unwrap() }, &[0xcc, 0x40]);
        // round_trip!(aper, H, H { a: true, b: "1".try_into().unwrap() }, &[0xa0, 0x31]);
        const PERMITTED_CONSTRAINT: Constraints = constraints!(
            permitted_alphabet_constraint!(&[
                b'a' as u32,
                b'b' as u32,
                b'c' as u32,
                b'd' as u32,
                b'e' as u32,
                b'f' as u32,
                b'g' as u32,
                b'h' as u32,
                b'i' as u32,
                b'j' as u32,
                b'k' as u32,
                b'l' as u32,
                b'm' as u32,
                b'n' as u32,
                b'o' as u32,
                b'p' as u32,
                b'q' as u32,
                b'r' as u32,
                b's' as u32,
                b't' as u32,
                b'u' as u32,
                b'v' as u32,
                b'w' as u32,
                b'x' as u32,
                b'y' as u32,
                b'z' as u32,
            ]),
            size_constraint!(1, 255)
        );
        round_trip_with_constraints!(
            aper,
            VisibleString,
            PERMITTED_CONSTRAINT,
            VisibleString::try_from("hej").unwrap(),
            &[0x02, 0x68, 0x65, 0x6a]
        );
        const PERMITTED_CONSTRAINT_2: Constraints =
            constraints!(permitted_alphabet_constraint!(&[b'a' as u32]));
        round_trip_with_constraints!(
            aper,
            VisibleString,
            PERMITTED_CONSTRAINT_2,
            VisibleString::try_from("a").unwrap(),
            &[0x01]
        );
    }

    #[test]
    fn issue_192() {
        // https://github.com/XAMPPRocky/rasn/issues/192
        use crate as rasn;

        use rasn::AsnType;

        #[derive(rasn::AsnType, rasn::Encode, rasn::Decode, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Updates {
            pub updates: Vec<u8>,
        }

        #[derive(rasn::AsnType, rasn::Encode, rasn::Decode, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[rasn(choice)]
        #[non_exhaustive]
        pub enum Message {
            Updates(Updates),
        }

        let msg = Message::Updates(Updates { updates: vec![1] });

        round_trip!(aper, Message, msg, &[0, 1, 1]);
    }

    #[test]
    fn issue_201() {
        use crate as rasn;
        use crate::prelude::*;

        const T124_IDENTIFIER_KEY: &Oid = Oid::const_new(&[0, 0, 20, 124, 0, 1]);
        #[derive(Debug, AsnType, Encode, rasn::Decode)]
        #[rasn(choice, automatic_tags)]
        enum Key {
            #[rasn(tag(explicit(5)))]
            Object(ObjectIdentifier),
            H221NonStandard(OctetString),
        }

        #[derive(Debug, AsnType, rasn::Encode, rasn::Decode)]
        #[rasn(automatic_tags)]
        struct ConnectData {
            t124_identifier_key: Key,
            connect_pdu: OctetString,
        }

        let connect_pdu: OctetString = vec![0u8, 1u8, 2u8, 3u8].into();
        let connect_data = ConnectData {
            t124_identifier_key: Key::Object(T124_IDENTIFIER_KEY.into()),
            connect_pdu,
        };

        let encoded = rasn::aper::encode(&connect_data).expect("failed to encode");
        assert_eq!(
            encoded,
            vec![
                0x00, 0x05, 0x00, 0x14, 0x7C, 0x00, 0x01, 0x04, 0x00, 0x01, 0x02, 0x03
            ]
        );
        let _: ConnectData = rasn::aper::decode(&encoded).expect("failed to decode");
    }

    #[test]
    fn fixed_size_bit_strings_longer_than_16_bits_are_octet_aligned() {
        // ITU-T X.691 (02/2021) §16.10: a fixed size above 16 bits is
        // octet-aligned, also within an extensible root; §16.9: up to 16 bits
        // it is not.
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Fixed24 {
            a: bool,
            #[rasn(size(24))]
            b: BitString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct FixedArray24 {
            a: bool,
            b: FixedBitString<24>,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Extensible32 {
            a: bool,
            #[rasn(size(32, extensible))]
            b: BitString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Fixed16 {
            a: bool,
            #[rasn(size(16))]
            b: BitString,
        }

        round_trip!(
            aper,
            Fixed24,
            Fixed24 {
                a: true,
                b: BitString::from_slice(&[0xab, 0xcd, 0xef]),
            },
            &[0x80, 0xab, 0xcd, 0xef]
        );
        round_trip!(
            aper,
            FixedArray24,
            FixedArray24 {
                a: true,
                b: {
                    let mut bits = FixedBitString::<24>::default();
                    bits[..24].copy_from_bitslice(
                        BitString::from_slice(&[0xab, 0xcd, 0xef]).as_bitslice(),
                    );
                    bits
                },
            },
            &[0x80, 0xab, 0xcd, 0xef]
        );
        round_trip!(
            aper,
            Extensible32,
            Extensible32 {
                a: true,
                b: BitString::from_slice(&[1, 2, 3, 4]),
            },
            &[0x80, 0x01, 0x02, 0x03, 0x04]
        );
        round_trip!(
            aper,
            Fixed16,
            Fixed16 {
                a: true,
                b: BitString::from_slice(&[0xab, 0xcd]),
            },
            &[0xd5, 0xe6, 0x80]
        );
    }

    #[test]
    fn length_determinants_follow_the_constrained_whole_number_cases() {
        // ITU-T X.691 (02/2021) §11.5.7: a length with a range of 256 is one
        // octet-aligned octet, a larger range below 64K two octet-aligned
        // octets; an upper bound of 64K or more makes the length unconstrained
        // (§11.9.3.3, §11.9.3.5).
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Octets256 {
            a: bool,
            #[rasn(size("1..=256"))]
            b: OctetString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Octets1000 {
            a: bool,
            #[rasn(size("1..=1000"))]
            b: OctetString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Octets65536 {
            a: bool,
            #[rasn(size("1..=65536"))]
            b: OctetString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Ia5256 {
            a: bool,
            #[rasn(size("1..=256"))]
            b: Ia5String,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Printable1000 {
            a: bool,
            #[rasn(size("1..=1000"))]
            b: PrintableString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Bits256 {
            a: bool,
            #[rasn(size("1..=256"))]
            b: BitString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Bits1000 {
            a: bool,
            #[rasn(size("1..=1000"))]
            b: BitString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Booleans256 {
            a: bool,
            #[rasn(size("1..=256"))]
            b: SequenceOf<bool>,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Booleans1000 {
            a: bool,
            #[rasn(size("1..=1000"))]
            b: SequenceOf<bool>,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Booleans65536 {
            a: bool,
            #[rasn(size("1..=65536"))]
            b: SequenceOf<bool>,
        }

        let octets = || OctetString::from_static(&[0x11, 0x22, 0x33]);
        let bits = || {
            let mut bits = BitString::from_slice(&[0xa0]);
            bits.truncate(3);
            bits
        };
        let booleans = || vec![true, false, true];
        round_trip!(
            aper,
            Octets256,
            Octets256 {
                a: true,
                b: octets()
            },
            &[0x80, 0x02, 0x11, 0x22, 0x33]
        );
        round_trip!(
            aper,
            Octets1000,
            Octets1000 {
                a: true,
                b: octets()
            },
            &[0x80, 0x00, 0x02, 0x11, 0x22, 0x33]
        );
        round_trip!(
            aper,
            Octets65536,
            Octets65536 {
                a: true,
                b: octets()
            },
            &[0x80, 0x03, 0x11, 0x22, 0x33]
        );
        round_trip!(
            aper,
            Ia5256,
            Ia5256 {
                a: true,
                b: Ia5String::try_from("abc").unwrap()
            },
            &[0x80, 0x02, 0x61, 0x62, 0x63]
        );
        round_trip!(
            aper,
            Printable1000,
            Printable1000 {
                a: true,
                b: PrintableString::try_from("abc").unwrap()
            },
            &[0x80, 0x00, 0x02, 0x61, 0x62, 0x63]
        );
        round_trip!(
            aper,
            Bits256,
            Bits256 { a: true, b: bits() },
            &[0x80, 0x02, 0xa0]
        );
        round_trip!(
            aper,
            Bits1000,
            Bits1000 { a: true, b: bits() },
            &[0x80, 0x00, 0x02, 0xa0]
        );
        round_trip!(
            aper,
            Booleans256,
            Booleans256 {
                a: true,
                b: booleans()
            },
            &[0x80, 0x02, 0xa0]
        );
        round_trip!(
            aper,
            Booleans1000,
            Booleans1000 {
                a: true,
                b: booleans()
            },
            &[0x80, 0x00, 0x02, 0xa0]
        );
        round_trip!(
            aper,
            Booleans65536,
            Booleans65536 {
                a: true,
                b: booleans()
            },
            &[0x80, 0x03, 0xa0]
        );
    }

    #[test]
    fn sequence_of_components_are_not_aligned_by_the_length() {
        // ITU-T X.691 (02/2021) §20.6: the components follow the length
        // determinant, with no padding; a count outside an extensible root
        // takes an unconstrained length (§20.4).
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", delegate, size("1..=8"))]
        struct Booleans8(SequenceOf<bool>);
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Wrapped8 {
            a: bool,
            b: Booleans8,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", delegate, size("1..=16", extensible))]
        struct Booleans16(SequenceOf<bool>);

        round_trip!(
            aper,
            Wrapped8,
            Wrapped8 {
                a: true,
                b: Booleans8(vec![true, false, true])
            },
            &[0xaa]
        );
        let mut seventeen = vec![true; 17];
        seventeen[1] = false;
        seventeen[3] = false;
        seventeen[5] = false;
        round_trip!(
            aper,
            Booleans16,
            Booleans16(seventeen),
            &[0x80, 0x11, 0xab, 0xff, 0x80]
        );
    }

    #[test]
    fn known_multiplier_strings_align_by_upper_bound_times_character_width() {
        // ITU-T X.691 (02/2021) §30.5.6: a fixed size is octet-aligned when
        // the upper bound times the character width is more than 16 bits;
        // §30.5.7: other sizes when it is 16 bits or more.
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate", delegate, size("1..=2"))]
        struct Ia5UpTo2(Ia5String);
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Ia5One {
            a: bool,
            #[rasn(size(1))]
            b: Ia5String,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct NumericUpTo2 {
            a: bool,
            #[rasn(size("1..=2"))]
            b: NumericString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Numeric3 {
            a: bool,
            #[rasn(size(3))]
            b: NumericString,
        }
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Numeric5 {
            a: bool,
            #[rasn(size(5))]
            b: NumericString,
        }

        // 2 characters of 8 bits: 16 bits, aligned.
        round_trip!(
            aper,
            Ia5UpTo2,
            Ia5UpTo2(Ia5String::try_from("w").unwrap()),
            &[0x00, 0x77]
        );
        // 1 character of 8 bits, fixed: not aligned.
        round_trip!(
            aper,
            Ia5One,
            Ia5One {
                a: true,
                b: Ia5String::try_from("J").unwrap()
            },
            &[0xa5, 0x00]
        );
        // 2 characters of 4 bits: 8 bits, not aligned.
        round_trip!(
            aper,
            NumericUpTo2,
            NumericUpTo2 {
                a: true,
                b: NumericString::try_from("1").unwrap()
            },
            &[0x88]
        );
        // 3 characters of 4 bits, fixed: 12 bits, not aligned.
        round_trip!(
            aper,
            Numeric3,
            Numeric3 {
                a: true,
                b: NumericString::try_from("123").unwrap()
            },
            &[0x91, 0xa0]
        );
        // 5 characters of 4 bits, fixed: 20 bits, aligned.
        round_trip!(
            aper,
            Numeric5,
            Numeric5 {
                a: true,
                b: NumericString::try_from("12345").unwrap()
            },
            &[0x80, 0x23, 0x45, 0x60]
        );
    }

    #[test]
    fn fixed_size_octet_strings_align_after_the_extension_bit() {
        // ITU-T X.691 (02/2021) §17.3, §17.7: the extension bit comes first,
        // then the octet-aligned octets of the fixed size.
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Extensible3 {
            a: bool,
            #[rasn(size(3, extensible))]
            b: OctetString,
        }

        round_trip!(
            aper,
            Extensible3,
            Extensible3 {
                a: true,
                b: OctetString::from_static(&[1, 2, 3])
            },
            &[0x80, 0x01, 0x02, 0x03]
        );
    }

    #[test]
    fn an_extensible_permitted_alphabet_is_not_per_visible() {
        // ITU-T X.691 (02/2021) §10.3.11: the characters use the whole IA5
        // alphabet, 8 bits each, and nothing makes the type extensible
        // (§10.3.18); 2 characters of 8 bits, fixed, are not aligned.
        #[derive(Debug, AsnType, Decode, Encode, PartialEq)]
        #[rasn(crate_root = "crate")]
        struct Letters {
            a: bool,
            #[rasn(from("a..=d", extensible), size(2))]
            b: Ia5String,
        }

        round_trip!(
            aper,
            Letters,
            Letters {
                a: true,
                b: Ia5String::try_from("ab").unwrap()
            },
            &[0xb0, 0xb1, 0x00]
        );
    }
}
