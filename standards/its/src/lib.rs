#![cfg_attr(not(test), no_std)]
extern crate alloc;

/// ASN.1 definitions for [ETSI TS 102 894-2].
///
/// The standard defines the ITS Common Data Dictionary (CDD), a set of
/// reusable data objects used across ITS application- and facilities-layer
/// messages. The exact interpretation of these elements is defined in the
/// corresponding message standards.
///
/// [ETSI TS 102 894-2]: https://www.etsi.org/deliver/etsi_ts/102800_102899/10289402/02.01.01_60/ts_10289402v020101p.pdf
#[allow(rustdoc::broken_intra_doc_links)]
#[allow(rustdoc::invalid_rust_codeblocks)]
#[allow(rustdoc::bare_urls)]
#[allow(clippy::doc_overindented_list_items)]
pub mod ts102894_2;

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
