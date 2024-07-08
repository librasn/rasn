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
use nom::AsBytes;

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

macro_rules! impl_restricted_core_traits {
    // ($($target:ty, $width:ty),*) => {
        ($(($target:ty, $width:ty)),* $(,)?) => {
    $(
    impl TryFrom<&'_ [u8]> for $target {
        type Error = PermittedAlphabetError;
        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            Ok(Self(Self::try_from_slice(value)?))
            }
        }

    impl TryFrom<&'_ str> for $target {
        type Error = PermittedAlphabetError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(Self(Self::try_from_slice(value)?))
        }
    }

    impl TryFrom<Vec<u8>> for $target {
        type Error = PermittedAlphabetError;
        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self(Self::try_from_slice(value.as_slice())?))
        }
    }

    impl TryFrom<String> for $target {
        type Error = PermittedAlphabetError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Ok(Self(Self::try_from_slice(&value)?))
        }
    }

    impl TryFrom<bytes::Bytes> for $target {
        type Error = PermittedAlphabetError;

        fn try_from(value: bytes::Bytes) -> Result<Self, Self::Error> {
            Ok(Self(Self::try_from_slice(value.as_ref().as_bytes())?))
        }
    }

    impl TryFrom<BitString> for $target {
        type Error = PermittedAlphabetError;

        fn try_from(string: BitString) -> Result<Self, Self::Error> {
            Self::try_from_permitted_alphabet(string, None)
        }
    }

    // impl From<$target> for bytes::Bytes {
    //     fn from(value: $target) -> Self {
    //         value.0.into()
    //     }
    // }

    impl core::ops::Deref for $target {
        type Target = Vec<$width>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl core::ops::DerefMut for $target {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    )*
};
}
impl_restricted_core_traits!(
    (BmpString, u16),
    (GeneralString, u8),
    (Ia5String, u8),
    (NumericString, u8),
    (PrintableString, u8),
    (TeletexString, u32),
    (VisibleString, u8)
);
