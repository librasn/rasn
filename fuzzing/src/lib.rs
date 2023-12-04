// Attempts to decode random fuzz data and if we're successful, we check
// that the encoder can produce encoding that the is *semantically*
// equal to the original decoded value. So we decode that value back
// into Rust because the encoder is guaranteed to produce the same
// encoding as the accepted input since `data` could contain trailing
// bytes not used by the decoder.
use rasn::prelude::*;

macro_rules! fuzz_type {
    ($codec:ident, $data:expr, $($typ:ty),+ $(,)?) => {
        $(
            if let Ok(value) = rasn::$codec::decode::<$typ>($data) {
                assert_eq!(value, rasn::$codec::decode(&rasn::$codec::encode(&value).unwrap()).unwrap());
            }
        )+
    }
}

pub fn fuzz_oer(data: &[u8]) {
    fuzz_type!(oer, data, Integer);
}
pub fn fuzz_pkix(data: &[u8]) {
    fuzz_type!(der, data, rasn_pkix::Certificate);
}

pub fn fuzz(_data: &[u8]) {
    todo!()
}
