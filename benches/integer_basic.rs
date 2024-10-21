// Based on https://github.com/dudycz/asn1_codecs_bench under Apache License Version 2.0 License
//
// Modifications:
// Adds other rasn codecs to the benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rasn::{ber, oer, uper};

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]
pub mod world3d {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Color {
        #[rasn(value("0..=255"))]
        pub r: u8,
        #[rasn(value("0..=255"))]
        pub g: u8,
        #[rasn(value("0..=255"))]
        pub b: u8,
        #[rasn(value("0..=65335"))]
        pub a: u16,
    }
    impl Color {
        pub fn new(r: u8, g: u8, b: u8, a: u16) -> Self {
            Self { r, g, b, a }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Column {
        #[rasn(size("10"))]
        pub elements: SequenceOf<Color>,
    }
    impl Column {
        pub fn new(elements: SequenceOf<Color>) -> Self {
            Self { elements }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Plane {
        #[rasn(size("10"))]
        pub rows: SequenceOf<Column>,
    }
    impl Plane {
        pub fn new(rows: SequenceOf<Column>) -> Self {
            Self { rows }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct World {
        #[rasn(size("10"))]
        pub depth: SequenceOf<Plane>,
    }
    impl World {
        pub fn new(depth: SequenceOf<Plane>) -> Self {
            Self { depth }
        }
    }
}

pub fn build_sample_rasn() -> world3d::World {
    use world3d::*;
    let color = Color::new(42, 128, 77, 12312);
    let elements = (0..10).map(|_| color.clone()).collect::<Vec<_>>();
    let column = Column { elements };
    let rows = (0..10).map(|_| column.clone()).collect::<Vec<_>>();
    let plane = Plane { rows };
    let depth = (0..10).map(|_| plane.clone()).collect::<Vec<_>>();

    World { depth }
}

macro_rules! rasn_enc_fn {
    ($fn_name:ident, $codec:ident) => {
        fn $fn_name(c: &mut Criterion) {
            c.bench_function(
                &format!(
                    "RASN/encode {} - basic integer sample.asn",
                    stringify!($codec).to_uppercase()
                ),
                |b| b.iter(|| black_box($codec::encode(&build_sample_rasn()).unwrap())),
            );
        }
    };
}

macro_rules! rasn_dec_fn {
    ($fn_name:ident, $codec:ident) => {
        fn $fn_name(c: &mut Criterion) {
            let w = build_sample_rasn();
            let encoded = $codec::encode(&w).unwrap();

            c.bench_function(
                &format!(
                    "RASN/decode {} - basic integer sample.asn",
                    stringify!($codec).to_uppercase()
                ),
                |b| b.iter(|| black_box($codec::decode::<world3d::World>(&encoded).unwrap())),
            );
        }
    };
}

rasn_enc_fn!(uper_rasn_enc, uper);
rasn_enc_fn!(oer_rasn_enc, oer);
rasn_enc_fn!(ber_rasn_enc, ber);

rasn_dec_fn!(uper_rasn_dec, uper);
rasn_dec_fn!(oer_rasn_dec, oer);
rasn_dec_fn!(ber_rasn_dec, ber);

criterion_group!(
    benches,
    uper_rasn_enc,
    uper_rasn_dec,
    oer_rasn_enc,
    oer_rasn_dec,
    ber_rasn_enc,
    ber_rasn_dec
);
criterion_main!(benches);
