#[macro_use] extern crate afl;

use rasn::{ber, der, cer, types};


fn main() {
    afl::fuzz!(|data: &[u8]| {
        // Attempts to decode random fuzz data and if we're successful, we check
        // that the encoder can produce encoding that the is *semantically*
        // equal to the original decoded value. So we decode that value back
        // into Rust because the encoder is guarenteed to produce the same
        // encoding as the accepted input since `data` could contain trailing
        // bytes not used by the decoder.
        if let Ok(value) = ber::decode::<types::Open>(data) {
            assert_eq!(value, ber::decode(&ber::encode(&value).unwrap()).unwrap());
        }

        if let Ok(value) = cer::decode::<types::Open>(data) {
            assert_eq!(value, cer::decode(&cer::encode(&value).unwrap()).unwrap());
        }

        if let Ok(value) = der::decode::<types::Open>(data) {
            assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
        }
        // if let Ok(value) = cer::decode::<types::Open>(data) {
        //     assert_eq!(value, cer::decode(&cer::encode(&value).unwrap()).unwrap());
        // }

        // if let Ok(value) = der::decode::<types::Open>(data) {
        //     assert_eq!(value, der::decode(&der::encode(&value).unwrap()).unwrap());
        // }
    });
}

