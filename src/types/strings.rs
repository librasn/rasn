mod bit;
mod bmp;
mod constrained;
mod general;
mod ia5;
mod numeric;
mod octet;
mod printable;
mod teletex;
mod visible;

use crate::error::strings::PermittedAlphabetError;
use crate::prelude::*;

pub use {
    alloc::string::String as Utf8String,
    bit::{BitStr, BitString, FixedBitString},
    bmp::BmpString,
    general::GeneralString,
    ia5::Ia5String,
    numeric::NumericString,
    octet::{FixedOctetString, OctetString},
    printable::PrintableString,
    teletex::TeletexString,
    visible::VisibleString,
};

pub(crate) use constrained::{
    should_be_indexed, DynConstrainedCharacterString, StaticPermittedAlphabet,
};

const fn bytes_to_chars<const N: usize>(input: [u8; N]) -> [u32; N] {
    let mut chars: [u32; N] = [0; N];

    let mut index = 0;
    while index < N {
        chars[index] = input[index] as u32;
        index += 1;
    }

    chars
}
// impl TryFrom<String> for BmpString {
//     type Error = PermittedAlphabetError;
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         Ok(Self(Self::try_from_slice(&value)?))
//     }
// }

// impl TryFrom<Vec<u8>> for BmpString {
//     type Error = PermittedAlphabetError;
//     fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
//         let vec = Self::try_from_slice(value.as_slice())?;
//         Ok(Self(vec))
//     }
// }

// impl TryFrom<&'_ str> for BmpString {
//     type Error = PermittedAlphabetError;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         Ok(Self(Self::try_from_slice(value)?))
//     }
// }

// impl TryFrom<&'_ [u8]> for BmpString {
//     type Error = PermittedAlphabetError;

//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         Ok(Self(Self::try_from_slice(value)?))
//     }
// }

macro_rules! impl_try_from {
        ($($target:ty),*) => {
        $(
        impl TryFrom<&'_ [u8]> for $target {
            type Error = PermittedAlphabetError;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            Ok(Self::new(Self::try_from_slice(value)?))
            }
        }
        )*
    };
}
impl_try_from!(BmpString);
