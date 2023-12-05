// Attempts to decode random fuzz data and if we're successful, we check
// that the encoder can produce encoding that the is *semantically*
// equal to the original decoded value. So we decode that value back
// into Rust because the encoder is guaranteed to produce the same
// encoding as the accepted input since `data` could contain trailing
// bytes not used by the decoder.
use rasn::prelude::*;
// On top of unconstrained `Integer`, we also test some constrained types.
type IntegerA = ConstrainedInteger<0, { u64::MAX as i128 }>;
type IntegerB = ConstrainedInteger<{ i64::MIN as i128 }, { i64::MAX as i128 }>;
type IntegerC = ConstrainedInteger<{ i64::MIN as i128 }, 0>;

macro_rules! fuzz_type {
    ($codec:ident, $data:expr, $($typ:ty),+ $(,)?) => {
        $(
            if let Ok(value) = rasn::$codec::decode::<$typ>($data) {
                assert_eq!(value, rasn::$codec::decode::<$typ>(&rasn::$codec::encode(&value).unwrap()).unwrap());
            }
        )+
    }
}

pub fn fuzz_oer(data: &[u8]) {
    fuzz_type!(oer, data, IntegerA);
    fuzz_type!(oer, data, IntegerB);
    fuzz_type!(oer, data, IntegerC);
}
pub fn fuzz_pkix(data: &[u8]) {
    fuzz_type!(der, data, rasn_pkix::Certificate);
}

pub fn fuzz(_data: &[u8]) {
    todo!()
}
