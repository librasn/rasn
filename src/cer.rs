pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::ber::de::Error> {
    T::decode(&mut crate::ber::de::Decoder::new(
        input,
        crate::ber::de::DecoderOptions::cer(),
    ))
}

pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, crate::ber::enc::Error> {
    let mut enc = crate::ber::enc::Encoder::new(crate::ber::enc::EncoderOptions::cer());

    value.encode(&mut enc)?;

    Ok(enc.output)
}
