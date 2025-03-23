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
fn x509_decode(c: &mut Criterion) {
    let data: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let mut group = c.benchmark_group("X.509 - Decode");
    group.bench_function("rasn", |b| {
        b.iter(|| black_box(rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap()))
    });
    group.bench_function("x509-parser", |b| {
        b.iter(|| {
            black_box(
                <x509_parser::certificate::X509Certificate as x509_parser::prelude::FromDer<
                    x509_parser::error::X509Error,
                >>::from_der(data)
                .unwrap(),
            )
        })
    });
    group.bench_function("x509-cert", |b| {
        b.iter(|| {
            black_box(<x509_cert::Certificate as x509_cert::der::Decode>::from_der(data).unwrap())
        })
    });
    group.bench_function("x509-certificate", |b| {
        b.iter(|| black_box(x509_certificate::X509Certificate::from_der(data).unwrap()))
    });
    group.bench_function("pyca/cryptography-x509", |b| {
        b.iter(|| {
            black_box(
                ::asn1::parse_single::<cryptography_x509::certificate::Certificate>(data).unwrap(),
            )
        })
    });
    group.finish();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn x509_encode(c: &mut Criterion) {
    let data: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let mut group = c.benchmark_group("X.509 - Encode");
    group.bench_function("rasn", |b| {
        let cert = rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap();
        b.iter(|| black_box(rasn::der::encode(&cert)))
    });
    group.bench_function("x509-cert", |b| {
        use x509_cert::der::{Decode, Encode};
        let cert = x509_cert::Certificate::from_der(data).unwrap();
        b.iter(|| black_box(cert.to_der().unwrap()))
    });
    group.bench_function("x509-certificate", |b| {
        let cert = x509_certificate::X509Certificate::from_der(data).unwrap();
        b.iter(|| black_box(cert.encode_der().unwrap()))
    });
    group.bench_function("pyca/cryptography-x509", |b| {
        b.iter(|| {
            let cert =
                ::asn1::parse_single::<cryptography_x509::certificate::Certificate>(data).unwrap();
            black_box(::asn1::write_single(&cert).unwrap())
        })
    });
    group.finish();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn x509_rtt(c: &mut Criterion) {
    let data: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let mut group = c.benchmark_group("X.509 - Round Time Trip");
    group.bench_function("rasn", |b| {
        b.iter(|| {
            black_box(rasn::der::encode(
                &rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap(),
            ))
        })
    });
    group.bench_function("x509-cert", |b| {
        use x509_cert::der::{Decode, Encode};
        b.iter(|| {
            black_box(
                x509_cert::Certificate::from_der(data)
                    .unwrap()
                    .to_der()
                    .unwrap(),
            )
        })
    });
    group.bench_function("x509-certificate", |b| {
        b.iter(|| {
            black_box(
                x509_certificate::X509Certificate::from_der(data)
                    .unwrap()
                    .encode_der()
                    .unwrap(),
            )
        })
    });
    group.bench_function("pyca/cryptography-x509", |b| {
        b.iter(|| {
            black_box(
                ::asn1::write_single(
                    &::asn1::parse_single::<cryptography_x509::certificate::Certificate>(data)
                        .unwrap(),
                )
                .unwrap(),
            )
        })
    });
    group.finish();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
criterion_group!(codec, x509_decode, x509_encode, x509_rtt, rasn);

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
criterion_group!(codec, rasn);
criterion_main!(codec);
