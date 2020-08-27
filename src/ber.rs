mod error;
mod identifier;
mod de;

pub use self::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T> {
    T::decode(&mut de::Parser::new(input))
}

// pub fn encode<T, W>(writer: &mut W, value: &T) -> Result<T> {
//     todo!()
// }

