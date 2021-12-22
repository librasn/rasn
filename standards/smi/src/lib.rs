#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

#[macro_use]
mod macros;
mod object_type;

pub mod v1;
pub mod v2;

#[doc(hidden)]
pub use rasn;

pub use object_type::{Access, ObjectType, Status};
