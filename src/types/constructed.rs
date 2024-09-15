//! A module that contains `SET`, `SEQUENCE`, `SET OF` and `SEQUENCE OF` types.

use hashbrown::HashMap;

/// A `SET` or `SEQUENCE` value.
pub trait Constructed {
    /// Fields contained in the "root component list".
    const FIELDS: super::fields::Fields;
    /// Fields contained in the list of extensions.
    const EXTENDED_FIELDS: Option<super::fields::Fields> = None;
}

///  The `SEQUENCE OF` type.
/// ## Usage
/// ASN1 declaration such as ...
/// ```asn
/// Test-type-a ::= SEQUENCE OF BOOLEAN
/// Test-type-b ::= SEQUENCE OF INTEGER(1,...)
/// ```
/// ... can be represented using `rasn` as ...
/// ```rust
/// use rasn::prelude::*;
///
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate)]
/// struct TestTypeA(pub SequenceOf<bool>);
///
/// // Constrained inner primitive types need to be wrapped in a helper newtype
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate, value("1", extensible))]
/// struct InnerTestTypeB(pub Integer);
///
/// #[derive(AsnType, Decode, Encode)]
/// #[rasn(delegate)]
/// struct TestTypeB(pub SequenceOf<InnerTestTypeB>);
/// ```
pub type SequenceOf<T> = alloc::vec::Vec<T>;

/// The `SET OF` type - an unordered list of zero, one or more values of the component type.
/// Works internally identical to a `Vec<T>`, but with a different name and order of the elements does not matter.
#[derive(Debug, Clone)]
pub struct SetOf<T>(alloc::vec::Vec<T>);

impl<T> SetOf<T> {
    /// Create a new empty `SetOf`.
    pub fn new() -> Self {
        Self(alloc::vec::Vec::new())
    }
    /// Create a new `SetOf` with capacity for `n` elements.
    pub fn with_capacity(n: usize) -> Self {
        Self(alloc::vec::Vec::with_capacity(n))
    }
    /// Create a new `SetOf` from a `Vec<T>`.
    pub fn from_vec(vec: alloc::vec::Vec<T>) -> Self {
        Self(vec)
    }
    /// Convert the `SetOf` into a `Vec<T>`.
    pub fn to_vec(self) -> alloc::vec::Vec<T> {
        self.0
    }
    pub fn into_inner(self) -> alloc::vec::Vec<T> {
        self.0
    }
}
impl<T> core::ops::Deref for SetOf<T> {
    type Target = alloc::vec::Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for SetOf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<alloc::vec::Vec<u8>> for SetOf<u8> {
    fn from(vec: alloc::vec::Vec<u8>) -> Self {
        Self(vec)
    }
}
impl<T: Clone, const N: usize> From<[T; N]> for SetOf<T> {
    fn from(array: [T; N]) -> Self {
        Self(array.to_vec())
    }
}

impl<T> Default for SetOf<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> PartialEq for SetOf<T>
where
    T: PartialEq + Eq + core::hash::Hash,
{
    /// Compare two `SetOf` values for equality.
    /// The order of elements in the `SetOf` does not matter.
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        // Frequency count of elements in both sets
        let self_counts = self.0.iter().fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        });

        let other_counts = other.0.iter().fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        });

        self_counts == other_counts
    }
}
impl<T> Eq for SetOf<T> where T: Eq + core::hash::Hash {}

impl<T: core::hash::Hash + Clone + Ord> core::hash::Hash for SetOf<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let mut sorted = self.0.clone();
        sorted.sort_unstable();
        sorted.hash(state);
    }
}
