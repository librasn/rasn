pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: D, slice: &[u8]) -> Result<Self,  D::Error>;
}

pub trait Decoder {
    type Error: crate::error::Error;

    fn decode_bool(&self, slice: &[u8]) -> Result<bool, Self::Error>;
    fn decode_integer(&self, slice: &[u8]) -> Result<num_bigint::BigInt, Self::Error>;
    fn decode_octet_string(&self, slice: &[u8]) -> Result<bytes::Bytes, Self::Error>;
}

impl Decode for bool {
    fn decode<D: Decoder>(decoder: D, slice: &[u8]) -> Result<Self, D::Error> {
        decoder.decode_bool(slice)
    }
}

macro_rules! impl_integers {
    ($($int:ty),+ $(,)?) => {
        $(
        impl Decode for $int {
            fn decode<D: Decoder>(decoder: D, slice: &[u8]) -> Result<Self, D::Error> {
                use core::convert::TryInto;
                decoder.decode_integer(slice)?.try_into().map_err(crate::error::Error::custom)
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
    fn decode<D: Decoder>(decoder: D, slice: &[u8]) -> Result<Self, D::Error> {
        decoder.decode_integer(slice)
    }
}
