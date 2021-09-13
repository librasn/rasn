macro_rules! fuzz_type {
    ($data:expr; $($typ:ty),+ $(,)?) => {
        $(
            if let Ok(value) = rasn::ber::decode::<$typ>($data) {
                assert_eq!(value, rasn::ber::decode(&rasn::ber::encode(&value).unwrap()).unwrap());
            }

            if let Ok(value) = rasn::cer::decode::<$typ>($data) {
                assert_eq!(value, rasn::cer::decode(&rasn::cer::encode(&value).unwrap()).unwrap());
            }

            if let Ok(value) = rasn::der::decode::<$typ>($data) {
                assert_eq!(value, rasn::der::decode(&rasn::der::encode(&value).unwrap()).unwrap());
            }
        )+
    }
}

#[test]
fn havoc11() {
    fuzz_type! {
        &*include_bytes!("data/havoc11.bin");
        rasn_pkix::Certificate,
    }
}

#[test]
fn splice() {
    let data: &[u8] = include_bytes!("data/splice.bin");
    let value = dbg!(rasn::ber::decode::<rasn_pkix::AlgorithmIdentifier>(data).unwrap());
    let encoded = rasn::ber::encode(&value).unwrap();
    assert_eq!(value, rasn::ber::decode(&encoded).unwrap());

    let value = dbg!(rasn::cer::decode::<rasn_pkix::AlgorithmIdentifier>(data).unwrap());
    let encoded = rasn::cer::encode(&value).unwrap();
    assert_eq!(value, rasn::cer::decode(&encoded).unwrap());

    let value = dbg!(rasn::der::decode::<rasn_pkix::AlgorithmIdentifier>(data).unwrap());
    let encoded = rasn::der::encode(&value).unwrap();
    assert_eq!(value, rasn::der::decode(&encoded).unwrap());
}

