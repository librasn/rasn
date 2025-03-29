#![cfg_attr(not(test), no_std)]
extern crate alloc;

/// ASN.1 definitions for ETSI TS 103 097
pub mod ts103097;

/// ASN.1 definitions for IEEE 1609.2
pub mod ieee1609dot2;

/// A macro to implement `From` and `Deref` for a delegate type pair.
/// This is not suitable for newtypes with inner constraints.
#[macro_export]
macro_rules! delegate {
    ($from_type:ty, $to_type:ty) => {
        impl From<$from_type> for $to_type {
            fn from(item: $from_type) -> Self {
                Self(item)
            }
        }
        impl From<$to_type> for $from_type {
            fn from(item: $to_type) -> Self {
                item.0
            }
        }
        impl core::ops::Deref for $to_type {
            type Target = $from_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl core::ops::DerefMut for $to_type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
