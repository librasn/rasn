//! The BER family of codecs on messages as they occur in the wild.
//!
//! Each input is a real encoding, chosen for a shape the others lack:
//!
//! - `letsencrypt-x3.crt`: the Let's Encrypt Authority X3 intermediate, an
//!   RSA-2048 certificate whose key and signature need long-form lengths.
//! - `letsencrypt-org.crt`: the certificate `letsencrypt.org` served in
//!   September 2026, an ECDSA P-256 leaf with ten subject alternative names,
//!   a CRL distribution point and a signed certificate timestamp list.
//! - `isrg-root-x1.crt`: ISRG Root X1, an RSA-4096 root with a 512-octet
//!   signature and few extensions.
//! - `letsencrypt-ye2-96.crl`: shard 96 of the Let's Encrypt YE2 CRL as
//!   published on 2026-10-04, a SEQUENCE OF 1559 revoked certificates, some
//!   with a reason code.
//! - `digicert-ocsp-response.der`: the response of `ocsp.digicert.com` for
//!   the `digicert.com` certificate on 2026-10-04: GeneralizedTime, explicit
//!   tags and an implicitly tagged CHOICE.
//! - `signed.cms` and `pesig.p7` (from the `rasn-cms` tests): a CMS
//!   SignedData written by `openssl cms` and an Authenticode signature, each
//!   with a certificate chain and a SET OF signed attributes that DER and CER
//!   must order.
//! - `cms-signed-data-stream.ber`: a SignedData written by
//!   `openssl cms -sign -stream` with a throwaway key, in the BER form
//!   streaming signers emit: indefinite lengths and a constructed OCTET
//!   STRING.
//! - `kerberos-as-rep.der` and `snmpv3-get-request.der` (from the
//!   `rasn-kerberos` and `rasn-snmp` tests): messages of 197 and 134 octets
//!   with APPLICATION tags, where per-message costs dominate; Kerberos tags
//!   every field explicitly.
//!
//! Every input is decoded under BER and, when it is valid DER, under DER;
//! the decoded value is encoded under DER and CER. Certificates are also
//! decoded and encoded in full, extension bodies and name attributes
//! included.

#[allow(dead_code)]
mod common;

use common::x509::ParsedCertificate;
use criterion::measurement::WallTime;
use criterion::{
    BenchmarkGroup, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use rasn::{Decode, Encode};

/// A group for the input `data`, reporting throughput in its octets.
fn group<'c>(c: &'c mut Criterion, name: &str, data: &[u8]) -> BenchmarkGroup<'c, WallTime> {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Bytes(data.len() as u64));
    group
}

/// Benchmarks the codecs on `data`, which encodes a `T`.
fn codec<T: Decode + Encode>(group: &mut BenchmarkGroup<WallTime>, data: &[u8]) {
    let value = rasn::ber::decode::<T>(data).unwrap();
    group.bench_function("ber/decode", |b| {
        b.iter(|| black_box(rasn::ber::decode::<T>(data).unwrap()))
    });
    if rasn::der::decode::<T>(data).is_ok() {
        group.bench_function("der/decode", |b| {
            b.iter(|| black_box(rasn::der::decode::<T>(data).unwrap()))
        });
    }
    group.bench_function("der/encode", |b| {
        b.iter(|| black_box(rasn::der::encode(&value).unwrap()))
    });
    group.bench_function("cer/encode", |b| {
        b.iter(|| black_box(rasn::cer::encode(&value).unwrap()))
    });
}

/// Benchmarks the codecs on `data`, which encodes a `T`.
fn message<T: Decode + Encode>(c: &mut Criterion, name: &str, data: &[u8]) {
    let mut group = group(c, name, data);
    codec::<T>(&mut group, data);
    group.finish();
}

/// Benchmarks the codecs on the DER certificate `data`, as a whole and in
/// full.
fn certificate(c: &mut Criterion, name: &str, data: &[u8]) {
    let mut group = group(c, name, data);
    codec::<rasn_pkix::Certificate>(&mut group, data);
    let parsed = ParsedCertificate::decode(data).unwrap();
    group.bench_function("der/decode (full)", |b| {
        b.iter(|| black_box(ParsedCertificate::decode(data).unwrap()))
    });
    group.bench_function("der/encode (full)", |b| {
        b.iter(|| black_box(parsed.encode().unwrap()))
    });
    group.finish();
}

/// The SignedData carried by the CMS ContentInfo `content_info`.
fn signed_data(content_info: &[u8]) -> Vec<u8> {
    rasn::ber::decode::<rasn_cms::ContentInfo>(content_info)
        .unwrap()
        .content
        .into_bytes()
}

fn certificates(c: &mut Criterion) {
    certificate(
        c,
        "Certificate - Let's Encrypt X3",
        include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt"),
    );
    certificate(
        c,
        "Certificate - letsencrypt.org",
        include_bytes!("data/letsencrypt-org.crt"),
    );
    certificate(
        c,
        "Certificate - ISRG Root X1",
        include_bytes!("data/isrg-root-x1.crt"),
    );
}

fn crl(c: &mut Criterion) {
    message::<rasn_pkix::CertificateList>(
        c,
        "CRL - Let's Encrypt YE2 shard 96",
        include_bytes!("data/letsencrypt-ye2-96.crl"),
    );
}

fn ocsp(c: &mut Criterion) {
    let response = rasn::der::decode::<rasn_ocsp::OcspResponse>(include_bytes!(
        "data/digicert-ocsp-response.der"
    ))
    .unwrap();
    message::<rasn_ocsp::BasicOcspResponse>(
        c,
        "OCSP - DigiCert BasicOCSPResponse",
        &response.bytes.unwrap().response,
    );
}

fn cms(c: &mut Criterion) {
    message::<rasn_cms::SignedData>(
        c,
        "CMS - openssl SignedData",
        &signed_data(include_bytes!("../standards/cms/tests/data/signed.cms")),
    );
    message::<rasn_cms::SignedData>(
        c,
        "CMS - streamed SignedData",
        &signed_data(include_bytes!("data/cms-signed-data-stream.ber")),
    );
    message::<rasn_cms::pkcs7_compat::SignedData>(
        c,
        "CMS - Authenticode SignedData",
        &signed_data(include_bytes!("../standards/cms/tests/data/pesig.p7")),
    );
}

fn kerberos(c: &mut Criterion) {
    message::<rasn_kerberos::AsRep>(
        c,
        "Kerberos - AS-REP",
        include_bytes!("data/kerberos-as-rep.der"),
    );
}

fn snmp(c: &mut Criterion) {
    message::<rasn_snmp::v3::Message>(
        c,
        "SNMPv3 - GetRequest",
        include_bytes!("data/snmpv3-get-request.der"),
    );
}

criterion_group!(real_world, certificates, crl, ocsp, cms, kerberos, snmp);
criterion_main!(real_world);
