//! A module that contains `SET`, `SEQUENCE`, `SET OF` and `SEQUENCE OF` types.

use core::hash::{BuildHasher, Hash};
use hashbrown::HashMap;

/// A `SET` or `SEQUENCE` value.
pub trait Constructed<const N: usize> {
    /// Fields contained in the "root component list".
    const FIELDS: super::fields::Fields<N>;
    /// Fields contained in the list of extensions.
    const EXTENDED_FIELDS: Option<super::fields::Fields<N>> = None;
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
///
/// Works internally like  `HashMap<T, usize>`, where the count of each element is tracked and used appropriately to represent an unordered list.
#[derive(Debug, Clone)]
pub struct SetOf<T> {
    elements: HashMap<T, usize>,
}

impl<T> SetOf<T>
where
    T: Eq + Hash,
{
    /// Construct a new empty set of value.
    pub fn new() -> Self {
        SetOf {
            elements: HashMap::new(),
        }
    }

    /// Create a new `SetOf` from a `Vec<T>`.
    pub fn from_vec(vec: alloc::vec::Vec<T>) -> Self {
        let mut elements = HashMap::with_capacity(vec.len());
        for item in vec {
            *elements.entry(item).or_insert(0) += 1;
        }
        Self { elements }
    }

    /// Create a new `SetOf` with capacity for `n` elements.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            elements: HashMap::with_capacity(n),
        }
    }

    /// Insert an element into the set.
    pub fn insert(&mut self, item: T) {
        *self.elements.entry(item).or_insert(0) += 1;
    }

    /// Get the number of elements in the set.
    pub fn len(&self) -> usize {
        let mut len = 0;
        for (_, count) in &self.elements {
            for _ in 0..*count {
                len += 1;
            }
        }
        len
    }

    /// Returns whether the given value doesn't contain any elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Remove an element from the set.
    pub fn remove(&mut self, item: &T) -> bool {
        if let Some(count) = self.elements.get_mut(item) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.elements.remove(item);
            }
            true
        } else {
            false
        }
    }

    /// Check if the set contains an element.
    pub fn contains(&self, item: &T) -> bool {
        self.elements.contains_key(item)
    }

    /// Convert the set to a `Vec<&T>`. `&T` refers to the original element in the set.
    pub fn to_vec(&self) -> alloc::vec::Vec<&T> {
        let mut vec = alloc::vec::Vec::with_capacity(self.elements.values().sum());
        for (item, count) in &self.elements {
            for _ in 0..*count {
                vec.push(item);
            }
        }
        vec
    }
}

impl<T> PartialEq for SetOf<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<T> PartialEq<HashMap<T, usize>> for SetOf<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &HashMap<T, usize>) -> bool {
        &self.elements == other
    }
}
impl<T> Eq for SetOf<T> where T: Eq + Hash {}

impl<T> Hash for SetOf<T>
where
    T: Eq + Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Combine element hashes with counts and aggregate
        let mut combined_hash: u64 = 0;
        for (item, count) in &self.elements {
            let item_hash = self.elements.hasher().hash_one(item);
            // Combine the element hash with its count
            let combined_element_hash = item_hash.wrapping_mul(*count as u64);
            // Aggregate the combined element hashes using addition
            combined_hash = combined_hash.wrapping_add(combined_element_hash);
        }

        // Write the final combined hash to the provided hasher
        state.write_u64(combined_hash);
    }
}

impl<T: Eq + Hash> From<alloc::vec::Vec<T>> for SetOf<T> {
    fn from(vec: alloc::vec::Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}
impl<T: Clone + Eq + Hash> From<&[T]> for SetOf<T> {
    fn from(vec: &[T]) -> Self {
        Self::from_vec(vec.to_vec())
    }
}

impl<T: Clone + Eq + Hash, const N: usize> From<[T; N]> for SetOf<T> {
    fn from(array: [T; N]) -> Self {
        Self::from_vec(array.to_vec())
    }
}

impl<T: Eq + Hash> Default for SetOf<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_of() {
        use core::hash::BuildHasher;

        let int_set: SetOf<u8> = [1, 2, 3, 4, 5].into();
        let int_set_reversed: &SetOf<u8> = &[5, 4, 3, 2, 1].into();
        let int_set_diff: SetOf<u8> = [1, 2, 3, 4, 6].into();
        assert_eq!(&int_set, int_set_reversed);
        assert_ne!(int_set, int_set_diff);

        let mut set_a = SetOf::new();
        set_a.insert(1);
        set_a.insert(2);
        set_a.insert(3);

        let mut set_b = SetOf::new();
        set_b.insert(3);
        set_b.insert(2);
        set_b.insert(1);

        let set_c: SetOf<_> = alloc::vec![4, 5, 6].into();

        assert_eq!(set_a, set_b);
        assert_ne!(set_a, set_c);
        let hasher = hashbrown::hash_map::DefaultHashBuilder::default();
        let hashed_a = hasher.hash_one(&set_a);
        let hashed_c = hasher.hash_one(set_c);
        assert_ne!(hashed_a, hashed_c);
        // Duplicate test
        let set_d = SetOf::from_vec(alloc::vec![1, 1, 2, 2, 3, 3]);
        assert_ne!(set_a, set_d);
    }
}
