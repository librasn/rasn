use rasn_pkix::Certificate;

fn main() {
    const DER_BYTES: &[u8] = include_bytes!("../standards/pkix/tests/data/letsencrypt-x3.crt");
    let original = rasn::der::decode::<Certificate>(DER_BYTES).unwrap();
    let ber = rasn::ber::encode(&original).unwrap();
    let cer = rasn::cer::encode(&original).unwrap();
    let uper = rasn::uper::encode(&original).unwrap();
    let aper = rasn::aper::encode(&original).unwrap();

    let der_len = DER_BYTES.len();
    println!("Original DER size: {der_len:.2}");
    println!(
        "BER size: {} ({:.2}%)",
        ber.len(),
        percentage_difference(der_len, ber.len())
    );
    println!(
        "CER size: {} ({:.2}%)",
        cer.len(),
        percentage_difference(der_len, cer.len())
    );
    println!(
        "UPER size: {} ({:.2}%)",
        uper.len(),
        percentage_difference(der_len, uper.len())
    );
    println!(
        "APER size: {} ({:.2}%)",
        aper.len(),
        percentage_difference(der_len, aper.len())
    );
}

pub fn percentage_difference(num1: usize, num2: usize) -> f64 {
    let difference = num1 as f64 - num2 as f64;
    (difference / num1 as f64) * 100.0
}
