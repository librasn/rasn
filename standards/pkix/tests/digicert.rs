#[test]
fn it_works() {
    let contents = pem::parse(include_bytes!("DigiCertAssuredIDTLSCA.crt.pem")).unwrap();

    rasn::der::decode::<rasn_pkix::Certificate>(&contents.contents).unwrap();
}

