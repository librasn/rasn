use crate::per::{self, de, enc};

/// Attempts to decode `T` from `input` using UPER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, per::de::Error> {
    T::decode(&mut de::Decoder::new(input, per::Alignment::Unaligned))
}

/// Attempts to encode `value` to UPER.
pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, per::enc::Error> {
    let mut enc = enc::Encoder::new(per::Alignment::Unaligned);

    value.encode(&mut enc)?;

    Ok(enc.output.into())
}

