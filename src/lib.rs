//! # Rasn: A safe no_std ASN.1 Codec Framework
//!
#![no_std]
#[deny(missing_docs)]

extern crate alloc;

mod tag;

pub mod de;
pub mod enc;
pub mod types;

// Data Formats

pub mod ber;
pub mod cer;
pub mod der;

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
