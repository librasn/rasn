use alloc::vec::Vec;

/// Represents a complete encoded ASN.1 value of any type. Usually identified
/// with an [`ObjectIdentifier`][crate::types::ObjectIdentifier].
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Any {
    pub(crate) contents: Vec<u8>,
}

impl Any {
    /// Creates a new wrapper around the opaque value.
    #[must_use]
    pub fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }

    /// Provides the raw representation of the value as bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.contents
    }

    /// Converts `Self` into the raw representation of the value.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.contents
    }
}

impl AsRef<[u8]> for Any {
    fn as_ref(&self) -> &[u8] {
        self.contents.as_ref()
    }
}

impl From<Vec<u8>> for Any {
    fn from(value: Vec<u8>) -> Self {
        Any::new(value)
    }
}
