//! A module that contains `SET`, `SEQUENCE`, `SET OF` and `SEQUENCE OF` types.

use core::hash::BuildHasher;
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
impl<T> From<alloc::vec::Vec<T>> for SetOf<T> {
    fn from(vec: alloc::vec::Vec<T>) -> Self {
        Self(vec)
    }
}
impl<T: Clone> From<&[T]> for SetOf<T> {
    fn from(vec: &[T]) -> Self {
        Self(vec.to_vec())
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
        let self_counts =
            self.0
                .iter()
                .fold(HashMap::with_capacity(self.len()), |mut acc, item| {
                    *acc.entry(item).or_insert(0) += 1;
                    acc
                });

        let other_counts =
            other
                .0
                .iter()
                .fold(HashMap::with_capacity(other.len()), |mut acc, item| {
                    *acc.entry(item).or_insert(0) += 1;
                    acc
                });

        self_counts == other_counts
    }
}
impl<T> Eq for SetOf<T> where T: Eq + core::hash::Hash {}

impl<T: core::hash::Hash + Eq> core::hash::Hash for SetOf<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // We can't use unstable_sort/sort to calculate the hash for the unordered list.
        // This is because we can't implement PartialOrd/Ord for T in `SetOf``, when the T uses itself as a subtype.

        // We can hash the item and its count instead, and then combine the hashes to get equal hashes of unordered list.

        // Count occurrences of each element
        let mut element_counts = HashMap::with_capacity(self.0.len());
        for item in &self.0 {
            *element_counts.entry(item).or_insert(0u64) += 1;
        }

        // Combine element hashes with counts and aggregate
        let mut combined_hash: u64 = 0;
        for (item, count) in &element_counts {
            let element_hasher = element_counts.hasher();
            let item_hash = element_hasher.hash_one(item);
            // Combine the element hash with its count
            let combined_element_hash = item_hash.wrapping_mul(*count);
            // Aggregate the combined element hashes using addition
            combined_hash = combined_hash.wrapping_add(combined_element_hash);
        }

        // Write the final combined hash to the provided hasher
        state.write_u64(combined_hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_of() {
        let int_set: SetOf<u8> = [1, 2, 3, 4, 5].into();
        let int_set_reversed: &SetOf<u8> = &[5, 4, 3, 2, 1].into();
        let int_set_diff: SetOf<u8> = [1, 2, 3, 4, 6].into();
        assert_eq!(&int_set, int_set_reversed);
        assert_ne!(int_set, int_set_diff);

        let mut set_a = SetOf::new();
        set_a.push(1);
        set_a.push(2);
        set_a.push(3);

        let mut set_b = SetOf::new();
        set_b.push(3);
        set_b.push(2);
        set_b.push(1);

        let set_c: SetOf<_> = alloc::vec![4, 5, 6].into();

        assert_eq!(set_a, set_b);
        assert_ne!(set_a, set_c);
        let hasher = hashbrown::hash_map::DefaultHashBuilder::default();
        let hashed_a = hasher.hash_one(set_a);
        let hashed_b = hasher.hash_one(set_b);
        let hashed_c = hasher.hash_one(set_c);
        assert_eq!(hashed_a, hashed_b);
        assert_ne!(hashed_a, hashed_c);
    }
}
