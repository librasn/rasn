pub mod de;
pub mod enc;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;

fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! round_trip {
        ($codec:ident, $typ:ty, $value:expr, $expected:expr) => {{
            let value = $value;
            let actual_encoding = crate::$codec::encode(&value).unwrap();

            assert_eq!($expected, &*actual_encoding);

            let decoded_value: $typ = crate::$codec::decode(&actual_encoding).unwrap();

            assert_eq!(value, decoded_value);
        }}
    }

    #[test]
    fn sequence_of() {
        round_trip!(uper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
        round_trip!(aper, Vec<u8>, vec![1; 5], &[0b00000101, 1, 1, 1, 1, 1]);
    }
}
