//! A module that contains `SET`, `SEQUENCE`, `SET OF` and `SEQUENCE OF` types.

/// A `SET` or `SEQUENCE` value.
/// `RL` is the number of fields in the "root component list".
/// `EL` is the number of fields in the list of extensions.
pub trait Constructed<const RL: usize = 0, const EL: usize = 0> {
    /// Fields contained in the "root component list".
    const FIELDS: super::fields::Fields<RL>;
    /// Whether the type is extensible.
    const IS_EXTENSIBLE: bool = false;
    /// Fields contained in the list of extensions.
    const EXTENDED_FIELDS: Option<super::fields::Fields<EL>> = None;
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
/// Works internally like  `Vec<T>`, where the order just does not matter.
#[derive(Debug, Clone)]
pub struct SetOf<T> {
    elements: alloc::vec::Vec<T>,
}

impl<T> SetOf<T>
where
    T: Eq,
{
    /// Construct a new empty set of value.
    #[must_use]
    pub fn new() -> Self {
        SetOf {
            elements: alloc::vec::Vec::new(),
        }
    }

    /// Create a new `SetOf` from a `Vec<T>`.
    #[must_use]
    pub fn from_vec(elements: alloc::vec::Vec<T>) -> Self {
        Self { elements }
    }

    /// Create a new `SetOf` with capacity for `n` elements.
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            elements: alloc::vec::Vec::with_capacity(n),
        }
    }

    /// Insert an element into the set.
    pub fn insert(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Get the number of elements in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the given value doesn't contain any elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Remove an element from the set.
    pub fn remove(&mut self, item: &T) -> bool {
        if let Some(idx) = self.elements.iter().position(|i| i == item) {
            // Order should not matter in SET OF
            self.elements.swap_remove(idx);
            true
        } else {
            false
        }
    }

    /// Check if the set contains an element.
    pub fn contains(&self, item: &T) -> bool {
        for i in &self.elements {
            if i == item {
                return true;
            }
        }
        false
    }

    /// Convert the set to a `Vec<&T>`. `&T` refers to the original element in the set.
    #[must_use]
    pub fn to_vec(&self) -> alloc::vec::Vec<&T> {
        self.elements.iter().collect()
    }
}

impl<T> PartialEq for SetOf<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.len() {
            return false;
        }
        for item in &self.elements {
            if !other.contains(item) {
                return false;
            }
        }
        for item in &other.elements {
            if !self.elements.contains(item) {
                return false;
            }
        }

        true
    }
}

impl<T> PartialEq<alloc::vec::Vec<T>> for SetOf<T>
where
    T: Eq,
{
    fn eq(&self, other: &alloc::vec::Vec<T>) -> bool {
        if self.elements.len() != other.len() {
            return false;
        }
        for item in &self.elements {
            if !other.contains(item) {
                return false;
            }
        }
        for item in other {
            if !self.elements.contains(item) {
                return false;
            }
        }

        true
    }
}
impl<T> Eq for SetOf<T> where T: Eq {}

impl<T> core::hash::Hash for SetOf<T>
where
    T: Eq + core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for item in &self.elements {
            item.hash(state);
        }
    }
}

impl<T: Eq> From<alloc::vec::Vec<T>> for SetOf<T> {
    fn from(vec: alloc::vec::Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}
impl<T: Clone + Eq> From<&[T]> for SetOf<T> {
    fn from(vec: &[T]) -> Self {
        Self::from_vec(vec.to_vec())
    }
}

impl<T: Clone + Eq, const N: usize> From<[T; N]> for SetOf<T> {
    fn from(array: [T; N]) -> Self {
        Self::from_vec(array.to_vec())
    }
}

impl<T: Eq> Default for SetOf<T> {
    fn default() -> Self {
        Self::new()
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
        // Duplicate test
        let set_d = SetOf::from_vec(alloc::vec![1, 1, 2, 2, 3, 3]);
        assert_ne!(set_a, set_d);
    }
}
