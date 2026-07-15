//! Regression tests for <https://github.com/librasn/rasn/issues/552>.
//!
//! `der::decode` must reject input that carries trailing bytes after the
//! top-level value: X.690 §8.1.1.1 requires a DER message to be exactly one
//! complete TLV with no trailing data. Previously `der::decode` returned the
//! decoded value and silently dropped any remaining bytes.

use rasn::error::DecodeErrorKind;
use rasn::prelude::*;

#[test]
fn der_decode_rejects_trailing_bytes() {
    // NULL (`05 00`) followed by four garbage bytes, the exact example from the
    // issue (ASN.1 NULL decodes to the unit type). Before the fix this decoded to
    // `()` and dropped `DE AD BE EF`.
    let input = &[0x05, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    let err = rasn::der::decode::<()>(input).expect_err("trailing data must be rejected");
    assert!(
        matches!(
            &*err.kind,
            DecodeErrorKind::UnexpectedExtraData { length: 4 }
        ),
        "issue #552: expected UnexpectedExtraData {{ length: 4 }}, got {:?}",
        err.kind
    );
}

#[test]
fn der_decode_rejects_trailing_bytes_on_integer() {
    // Same property for a primitive with content: INTEGER 1 (`02 01 01`) plus a
    // single trailing byte.
    let input = &[0x02, 0x01, 0x01, 0xFF];
    let err = rasn::der::decode::<Integer>(input).expect_err("trailing data must be rejected");
    assert!(
        matches!(
            &*err.kind,
            DecodeErrorKind::UnexpectedExtraData { length: 1 }
        ),
        "issue #552: expected UnexpectedExtraData {{ length: 1 }}, got {:?}",
        err.kind
    );
}

#[test]
fn der_decode_accepts_exact_input() {
    // The exact encoding with no trailing bytes still decodes successfully.
    rasn::der::decode::<()>(&[0x05, 0x00]).expect("exact NULL must still decode");
    assert_eq!(
        rasn::der::decode::<Integer>(&[0x02, 0x01, 0x2A]).expect("exact INTEGER must still decode"),
        Integer::from(42)
    );
}

#[test]
fn der_decode_round_trip_holds_for_exact_input() {
    // `encode(decode(bytes))` reproduces the exact input when there is no
    // trailing data. The round-trip divergence reported in the issue only arose
    // because trailing bytes were silently absorbed on decode.
    let input = &[0x02, 0x01, 0x2A]; // INTEGER 42
    let value = rasn::der::decode::<Integer>(input).expect("decode");
    let reencoded = rasn::der::encode(&value).expect("encode");
    assert_eq!(reencoded, input);
}
