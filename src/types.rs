mod oid;

pub use num_bigint::BigInt as Integer;
pub use bytes::Bytes as OctetString;
pub use oid::ObjectIdentifier;
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
