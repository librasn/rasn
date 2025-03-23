mod common;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use common::*;

fn rasn(c: &mut Criterion) {
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

    bench_encoding_rules!(ber, der, cer, uper, oer);
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn x509(c: &mut Criterion) {
    let data: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let mut group = c.benchmark_group("X.509");
    group.bench_function("rasn", |b| {
        b.iter(|| black_box(rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap()))
    });
    group.bench_function("x509-parser", |b| {
        b.iter(|| {
            black_box(
                <x509_parser::certificate::X509Certificate as x509_parser::prelude::FromDer<
                    x509_parser::error::X509Error,
                >>::from_der(data),
            )
        })
    });
    group.bench_function("x509-cert", |b| {
        b.iter(|| black_box(<x509_cert::Certificate as x509_cert::der::Decode>::from_der(data)))
    });
    group.bench_function("x509-certificate", |b| {
        b.iter(|| black_box(x509_certificate::X509Certificate::from_der(data)))
    });
    group.bench_function("pyca/cryptography-x509", |b| {
        b.iter(|| {
            black_box(::asn1::parse_single::<
                cryptography_x509::certificate::Certificate,
            >(data))
        })
    });
    group.finish();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
criterion_group!(codec, x509, rasn);

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
criterion_group!(codec, rasn);
criterion_main!(codec);
