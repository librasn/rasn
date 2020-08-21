use alloc::vec::Vec;
use core::ops;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObjectIdentifier(Vec<u32>);

impl ObjectIdentifier {
    /// Creates a new object identifier from `vec`.
    ///
    /// # Panics
    /// If `vec` contains less than two components.
    pub fn new(vec: Vec<u32>) -> Self {
        assert!(
            vec.len() >= 2,
            "ObjectIdentifier requires at least two components."
        );

        Self(vec)
    }
}

impl AsRef<[u32]> for ObjectIdentifier {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}

impl ops::Deref for ObjectIdentifier {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
