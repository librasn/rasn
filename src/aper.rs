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
}
