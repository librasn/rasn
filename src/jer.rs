pub mod de;
pub mod enc;

/// Attempts to decode `T` from `input` using JER.
/// # Errors
/// Returns error specific to JER decoder if decoding is not possible.
pub fn decode<'de, T: crate::Decode>(input: &'de str) -> Result<T, de::error::Error> {
    T::decode(&mut crate::jer::de::Decoder::new(
        input,
    ))
}

// /// Attempts to encode `value` to JER.
// /// # Errors
// /// Returns error specific to JER encoder if encoding is not possible.
// pub fn encode<T: serde::ser::Serialize>(
//     value: &T,
// ) -> Result<alloc::string::String, enc::error::Error> {
// }
