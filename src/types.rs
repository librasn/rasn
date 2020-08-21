mod oid;

pub use alloc::string::String as Utf8String;
pub use bytes::Bytes as OctetString;
pub use num_bigint::BigInt as Integer;
pub use oid::ObjectIdentifier;
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
