/// ITU-T X.696 (02/2021) version of OER decoding
// In OER, without knowledge of the type of the value encoded, it is not possible to determine
// the structure of the encoding. In particular, the end of the encoding cannot be determined from
// the encoding itself without knowledge of the type being encoded ITU-T X.696 (6.2).
mod config;

pub struct Decoder<'input> {
    input: &'input [u8],
    // config: DecoderOptions,
    initial_len: usize,
}
