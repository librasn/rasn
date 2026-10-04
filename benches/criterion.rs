mod common;

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use common::x509::ParsedCertificate;
use common::*;

fn rasn(c: &mut Criterion) {
    let decoded = black_box(bench_default());

    macro_rules! bench_encoding_rules {
        ($($rules : ident),+) => {{
            $(
                let data: Vec<u8> = black_box(rasn::$rules::encode(&decoded).unwrap());
                let mut group = c.benchmark_group(stringify!($rules));
                group.throughput(Throughput::Bytes(data.len() as u64));
                group.bench_function("encode", |b| b.iter_with_large_drop(|| black_box(rasn::$rules::encode(&decoded).unwrap())));
                group.bench_function("decode", |b| b.iter_with_large_drop(|| black_box(rasn::$rules::decode::<Bench>(&data).unwrap())));
                group.finish();
            )+
        }}
    }

    bench_encoding_rules!(ber, der, cer, uper, oer);
}

/// The certificate every X.509 group measures. The groups' "full" entries
/// also decode or encode its extension bodies and name attribute values,
/// which among the compared crates only `x509-parser` does, and does by
/// default.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const X509: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn x509_decode(c: &mut Criterion) {
    use x509_parser::nom::Parser;

    let data = X509;
    let mut group = c.benchmark_group("X.509 - Decode");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("rasn", |b| {
        b.iter(|| black_box(rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap()))
    });
    group.bench_function("rasn (full)", |b| {
        b.iter(|| black_box(ParsedCertificate::decode(data).unwrap()))
    });
    group.bench_function("x509-parser", |b| {
        b.iter(|| {
            black_box(
                x509_parser::certificate::X509CertificateParser::new()
                    .with_deep_parse_extensions(false)
                    .parse(data)
                    .unwrap(),
            )
        })
    });
    group.bench_function("x509-parser (full)", |b| {
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
    let data = X509;
    let mut group = c.benchmark_group("X.509 - Encode");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("rasn", |b| {
        let cert = rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap();
        b.iter(|| black_box(rasn::der::encode(&cert).unwrap()))
    });
    group.bench_function("rasn (full)", |b| {
        let cert = ParsedCertificate::decode(data).unwrap();
        b.iter(|| black_box(cert.encode().unwrap()))
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
        let cert =
            ::asn1::parse_single::<cryptography_x509::certificate::Certificate>(data).unwrap();
        b.iter(|| black_box(::asn1::write_single(&cert).unwrap()))
    });
    group.finish();
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn x509_rtt(c: &mut Criterion) {
    let data = X509;
    let mut group = c.benchmark_group("X.509 - Round Time Trip");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("rasn", |b| {
        b.iter(|| {
            black_box(
                rasn::der::encode(&rasn::der::decode::<rasn_pkix::Certificate>(data).unwrap())
                    .unwrap(),
            )
        })
    });
    group.bench_function("rasn (full)", |b| {
        b.iter(|| black_box(ParsedCertificate::decode(data).unwrap().encode().unwrap()))
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
