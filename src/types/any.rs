/// Represents a complete encoded ASN.1 value of any type. Usually identified
/// with an [`ObjectIdentifier`][crate::types::ObjectIdentifier].
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Any {
    pub(crate) contents: alloc::vec::Vec<u8>,
}

impl Any {
    pub fn new(contents: alloc::vec::Vec<u8>) -> Self {
        Self { contents }
    }
}
