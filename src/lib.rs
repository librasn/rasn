#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;

pub mod de;
pub mod enc;
pub mod types;

// Data Formats

pub mod ber;
pub mod cer;
pub mod der;

#[doc(inline)]
pub use self::{
    de::{Decode, Decoder},
    enc::{Encode, Encoder},
    types::{AsnType, Tag, TagTree},
};

/// A prelude containing the codec traits and all types defined in the [`types`]
/// module.
pub mod prelude {
    pub use crate::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        types::*,
    };
}

#[doc(hidden)]
pub use static_assertions as sa;
