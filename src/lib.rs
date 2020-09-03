#![no_std]
#![feature(const_generics)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod de;
pub mod enc;
pub mod error;
pub mod tag;
pub mod types;

// Data Formats

pub mod ber;

pub use de::{Decode, Decoder};
pub use enc::{Encode, Encoder};
pub use tag::Tag;
pub use types::AsnType;

#[doc(hidden)]
pub use static_assertions as sa;
