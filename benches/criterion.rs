//! Port of the `asn1tools` benchmark in Rust.

mod common;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use common::*;

fn asn1tools(c: &mut Criterion) {
    let decoded = black_box(bench_default());

    macro_rules! bench_encoding_rules {
        ($($rules : ident),+) => {{
            $(
                let data: Vec<u8> = black_box(rasn::$rules::encode(&decoded).unwrap());
                let mut group = c.benchmark_group(stringify!($rules));
                group.bench_function("encode", |b| b.iter_with_large_drop(|| black_box(rasn::$rules::encode(&decoded).unwrap())));
                group.bench_function("decode", |b| b.iter_with_large_drop(|| black_box(rasn::$rules::decode::<Bench>(&data).unwrap())));
                group.finish();
            )+
        }}
    }

    bench_encoding_rules!(ber);
    bench_encoding_rules!(der);
    bench_encoding_rules!(cer);
}

fn x509(c: &mut Criterion) {
    use x509_parser::prelude::*;

    let data: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let mut group = c.benchmark_group("Certificate");
    group.bench_function("rasn", |b| {
        b.iter(|| black_box(rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap()))
    });
    group.bench_function("x509_parser", |b| {
        b.iter(|| black_box(X509Certificate::from_der(data)))
    });
    group.finish();
}

criterion_group!(codec, x509, asn1tools);
criterion_main!(codec);
