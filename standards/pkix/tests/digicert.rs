#[test]
fn it_works() {
    let contents = pem::parse(include_bytes!("data/DigiCertAssuredIDTLSCA.crt.pem")).unwrap();

    rasn::der::decode::<rasn_pkix::Certificate>(&contents.contents).unwrap();
}

