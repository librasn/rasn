use crate::{Decode, Decoder, Result};

impl Decode for bool {
    fn decode<T: Decoder>(decoder: T, slice: &[u8]) -> Result<Self> {
        decoder.decode_bool(slice)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            fn decode<T: Decoder>(decoder: T, slice: &[u8]) -> Result<Self> {
                use core::convert::TryInto;
                use snafu::OptionExt;
                decoder.decode_integer(slice)?.try_into().ok().with_context(|| {
                    crate::error::IntegerOverflow { max_width: <$int>::MAX.count_ones() }
                })
            }
        }
        )+
    }
}

impl_integers! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
}

impl Decode for num_bigint::BigInt {
    fn decode<T: Decoder>(decoder: T, slice: &[u8]) -> Result<Self> {
        decoder.decode_integer(slice)
    }
}
