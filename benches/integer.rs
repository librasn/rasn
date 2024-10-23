// Based on https://github.com/dudycz/asn1_codecs_bench under Apache License Version 2.0 License
//
// Modifications:
// Adds other rasn codecs to the benchmark
// Adds optional and extended fields for `Color`

use criterion::{criterion_group, criterion_main, Criterion};
use rasn::{ber, oer, uper};

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]
pub mod world3d {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    // When adding an identical value constraints also on top of similar base type,
    // the constraints override is also benchmarked
    pub struct Color {
        #[rasn(value("0..=255"))]
        pub r: u8,
        #[rasn(value("0..=255"))]
        pub g: u8,
        #[rasn(value("0..=255"))]
        pub b: u8,
        #[rasn(value("0..=65335"))]
        pub a: u16,
        #[rasn(value("0..=4_294_967_295"))]
        pub w: Option<u32>,
        #[rasn(value("0..=4_294_967_295"))]
        pub l: Option<u32>,
        #[rasn(extension_addition)]
        #[rasn(value("0..=18_446_744_073_709_551_615"))]
        pub extension: Option<u64>,
        #[rasn(extension_addition)]
        #[rasn(value("0..=18_446_744_073_709_551_615"))]
        pub second_extension: Option<u64>,
    }
    impl Color {
        #[allow(clippy::too_many_arguments)]
        pub fn new(
            r: u8,
            g: u8,
            b: u8,
            a: u16,
            w: Option<u32>,
            l: Option<u32>,
            extension: Option<u64>,
            second_extension: Option<u64>,
        ) -> Self {
            // pub fn new(r: u8, g: u8, b: u8, a: u16) -> Self {
            Self {
                r,
                g,
                b,
                a,
                w,
                l,
                extension,
                second_extension,
            }
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
    let color = Color::new(
        42,
        128,
        i8::MAX as u8,
        i16::MAX as u16,
        Some(i32::MAX as u32),
        Some(i32::MAX as u32),
        Some(i64::MAX as u64),
        Some(i64::MAX as u64),
    );
    let elements = (0..10).map(|_| color.clone()).collect::<Vec<_>>();
    let column = Column { elements };
    let rows = (0..10).map(|_| column.clone()).collect::<Vec<_>>();
    let plane = Plane { rows };
    let depth = (0..10).map(|_| plane.clone()).collect::<Vec<_>>();

    World { depth }
}

fn uper_rasn_enc(c: &mut Criterion) {
    c.bench_function("RASN/encode UPER - sample.asn", |b| {
        b.iter(|| {
            let w = build_sample_rasn();
            let _ = uper::encode(&w).unwrap();
        })
    });
}
fn oer_rasn_enc(c: &mut Criterion) {
    c.bench_function("RASN/encode OER - sample.asn", |b| {
        b.iter(|| {
            let w = build_sample_rasn();
            let _ = oer::encode(&w).unwrap();
        })
    });
}
fn ber_rasn_enc(c: &mut Criterion) {
    c.bench_function("RASN/encode BER - sample.asn", |b| {
        b.iter(|| {
            let w = build_sample_rasn();
            let _ = ber::encode(&w).unwrap();
        })
    });
}

fn uper_rasn_dec(c: &mut Criterion) {
    let w = build_sample_rasn();
    let encoded = uper::encode(&w).unwrap();

    c.bench_function("RASN/decode UPER - sample.asn", |b| {
        b.iter(|| {
            let _ = uper::decode::<world3d::World>(&encoded).unwrap();
        })
    });
}
fn oer_rasn_dec(c: &mut Criterion) {
    let w = build_sample_rasn();
    let encoded = oer::encode(&w).unwrap();

    c.bench_function("RASN/decode OER - sample.asn", |b| {
        b.iter(|| {
            let _ = oer::decode::<world3d::World>(&encoded).unwrap();
        })
    });
}
// fn ber_rasn_dec(c: &mut Criterion) {
//     let w = build_sample_rasn();
//     let encoded = ber::encode(&w).unwrap();

//     c.bench_function("RASN/decode BER - sample.asn", |b| {
//         b.iter(|| {
//             let _ = ber::decode::<world3d::World>(&encoded).unwrap();
//         })
//     });
// }

criterion_group!(
    benches,
    uper_rasn_enc,
    uper_rasn_dec,
    oer_rasn_enc,
    oer_rasn_dec,
    ber_rasn_enc,
    // ber_rasn_dec
);
criterion_main!(benches);
