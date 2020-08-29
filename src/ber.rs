mod identifier;
mod de;
mod enc;

pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, de::Error> {
    T::decode(&mut de::Parser::new(input))
}

// pub fn encode<T, W>(writer: &mut W, value: &T) -> Result<T> {
//     todo!()
// }

