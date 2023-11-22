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
