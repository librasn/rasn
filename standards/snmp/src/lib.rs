//! # Simple Network Management Protocol
//! `rasn-snmp` implementation of the protocol data types from IETF RFCs 1157,
//! 1901, and 3416. This does not provide an implementation of an agent or proxy,
//! but provides the data types needed to build your own agent or proxy
//! implementation.
//!
//! This library in combination with it's sibling crates [`rasn`], [`rasn-smi`],
//! and [`rasn-mib`] allow you to decode, and encode SNMP protocol messages
//! using entirely safe Rust. All of these libraries are also `#[no_std]` so
//! they support any platform that supports [`alloc`].
//!
//! [`rasn`]: https://docs.rs/rasn
//! [`rasn-smi`]: https://docs.rs/rasn-smi
//! [`rasn-mib`]: https://docs.rs/rasn-mib
//!
//! ```rust,no_run
//! use rasn_snmp::{v2c::Message, v2::Pdus};
//!
//! let data: &[u8] = &[];
//!
//! // Decode SNMPv2c message containing a SNMPv2 PDU.
//! let message: Message<Pdus> = rasn::ber::decode(data).unwrap();
//!
//!// Handle the request.
//! match message.data {
//!     Pdus::GetRequest(request) => {},
//!     Pdus::GetNextRequest(request) => {},
//!     Pdus::Response(request) => {},
//!     // ...
//! # _ => {}
//! }
//! ```

#![no_std]

extern crate alloc;

pub mod v1;
pub mod v2;
pub mod v2c;
