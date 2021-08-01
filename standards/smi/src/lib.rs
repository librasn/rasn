//! # Structure of Management Information
//! `rasn-smi` is an implementation of the ASN.1 data types from the IETF RFCs
//! 1155 and 2578 on the Structure of Management Information the [`rasn`]
//! codec framework. These definitions are both transport layer agnostic,
//! and encoding rule agnostic.
//! ```rust,no_run
//! // Replace with your data.
//! let data: &[u8] = &[];
//! // Decode object from BER.
//! let object: rasn_smi::v2::ObjectSyntax = rasn::ber::decode(&data).unwrap();
//! // Encode it back into DER
//! let data = rasn::der::encode(&object).unwrap();
//! ```

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
