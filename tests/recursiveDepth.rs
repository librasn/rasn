// These tests verify that rasn correctly enforces maximum recursion depth limits during
// decoding to prevent stack exhaustion attacks.
//
// The test payloads were generated using the script available at:
// https://gist.github.com/Boslx/5f7be237dd996b2a95d8b9528bc1dce7

use crate::recursive_module::RecursiveChoice;
use rasn::error::DecodeErrorKind;
use rasn::{aper, ber, cer, coer, der, jer, oer, uper};

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod recursive_module {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum RecursiveChoice {
        id(Integer),
        sequence(RecursiveSequence),
        set(RecursiveSet),
        sequenceOf(RecursiveSequenceOf),
        setOf(RecursiveSetOf),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RecursiveSequence {
        pub id: Integer,
        pub next: Option<Box<RecursiveChoice>>,
    }
    impl RecursiveSequence {
        pub fn new(id: Integer, next: Option<Box<RecursiveChoice>>) -> Self {
            Self { id, next }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RecursiveSequenceOf(pub SequenceOf<RecursiveChoice>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(set, automatic_tags)]
    pub struct RecursiveSet {
        pub id: Integer,
        pub next: Option<Box<RecursiveChoice>>,
    }
    impl RecursiveSet {
        pub fn new(id: Integer, next: Option<Box<RecursiveChoice>>) -> Self {
            Self { id, next }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RecursiveSetOf(pub SetOf<RecursiveChoice>);
}

fn nested_exceeds_max_parse_depth_level(mut kind: &DecodeErrorKind) -> Option<usize> {
    let mut level = 0;
    loop {
        match kind {
            DecodeErrorKind::ExceedsMaxParseDepth => return Some(level),
            DecodeErrorKind::FieldError { nested, .. } => {
                level += 1;
                kind = &nested.kind
            }
            _ => return None,
        }
    }
}

#[test]
fn recursive_depth_150_aper() {
    let bytes = &*include_bytes!("data/recursive_depth_150.aper");
    let err = aper::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_ber() {
    let bytes = &*include_bytes!("data/recursive_depth_150.ber");
    let err = ber::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_cer() {
    let bytes = &*include_bytes!("data/recursive_depth_150.cer");
    let err = cer::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_coer() {
    let bytes = &*include_bytes!("data/recursive_depth_150.coer");
    let err = coer::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_der() {
    let bytes = &*include_bytes!("data/recursive_depth_150.der");
    let err = der::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_jer() {
    // serde_json has a limit of 128
    // https://github.com/serde-rs/json/blob/4f6dbfac79647d032b0997b5ab73022340c6dab7/src/de.rs#L63
    let text = include_str!("data/recursive_depth_150.jer");
    let err = jer::decode::<RecursiveChoice>(text).unwrap_err();
    assert!(
        matches!(&*err.kind, DecodeErrorKind::Parser { msg } if msg.contains("recursion limit exceeded"))
    );
}

#[test]
fn recursive_depth_150_oer() {
    let bytes = &*include_bytes!("data/recursive_depth_150.oer");
    let err = oer::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}

#[test]
fn recursive_depth_150_uper() {
    let bytes = &*include_bytes!("data/recursive_depth_150.uper");
    let err = uper::decode::<RecursiveChoice>(bytes).unwrap_err();
    let level = nested_exceeds_max_parse_depth_level(&err.kind)
        .expect("expected ExceedsMaxParseDepth somewhere in nested FieldError");
    assert_eq!(level, 16);
}
