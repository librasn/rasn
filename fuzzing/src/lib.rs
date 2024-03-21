// Attempts to decode random fuzz data and if we're successful, we check
// that the encoder can produce encoding that the is *semantically*
// equal to the original decoded value. So we decode that value back
// into Rust because the encoder is guarenteed to produce the same
// encoding as the accepted input since `data` could contain trailing
// bytes not used by the decoder.
macro_rules! fuzz_type {
    ($data:expr, $($typ:ty),+ $(,)?) => {
        $(
            if let Ok(value) = rasn::der::decode::<$typ>($data) {
                assert_eq!(value, rasn::der::decode(&rasn::der::encode(&value).unwrap()).unwrap());
            }
        )+
    }
}

pub fn fuzz(data: &[u8]) {
    fuzz_type!(data, rasn_pkix::Certificate);
}
