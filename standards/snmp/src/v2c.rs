//! Version 2, Community Version (RFC 1901)
use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Message<T> {
    pub version: Integer,
    pub community: OctetString,
    pub data: T,
}

impl<T> Message<T> {
    pub const VERSION: u64 = 1;
}
