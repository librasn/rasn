pub mod de;
pub mod enc;

/// Attempts to decode `T` from `input` using JER.
/// # Errors
/// Returns error specific to JER decoder if decoding is not possible.
pub fn decode<'de, T: crate::Decode>(input: &'de str) -> Result<T, de::error::Error> {
    T::decode(&mut crate::jer::de::Decoder::new(
        input,
    )?)
}

// /// Attempts to encode `value` to JER.
// /// # Errors
// /// Returns error specific to JER encoder if encoding is not possible.
// pub fn encode<T: serde::ser::Serialize>(
//     value: &T,
// ) -> Result<alloc::string::String, enc::error::Error> {
// }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[derive(AsnType, Decode, Encode, Debug)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    struct TestTypeA {
        #[rasn(value("0..3", extensible))]
        juice: Integer,
        wine: Inner,
        #[rasn(extension_addition)]
        grappa: BitString
    }

    #[derive(AsnType, Decode, Encode, Debug)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct Inner {
        #[rasn(value("0..3"))]
        wine: u8,
    }

    #[test]
    fn decodes_sequence() {
        let result: TestTypeA = crate::jer::decode(r#"{"juice":86174683764,"wine":{"wine":4},"grappa":"10101"}"#).unwrap();
        println!("{result:?}");
    }
}