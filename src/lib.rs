#![doc = include_str!("../README.md")]
#![no_std]
#[deny(missing_docs)]
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

pub mod prelude {
    pub use crate::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        types::*,
    };
}

#[doc(hidden)]
pub use static_assertions as sa;
