#![no_std]
#![feature(const_generics)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod de;
pub mod enc;
pub mod tag;
pub mod types;

// Data Formats

pub mod ber;

#[doc(inline)]
pub use de::{Decode, Decoder};
#[doc(inline)]
pub use enc::{Encode, Encoder};
#[doc(inline)]
pub use tag::Tag;
#[doc(inline)]
pub use types::AsnType;

#[doc(hidden)]
pub use static_assertions as sa;
