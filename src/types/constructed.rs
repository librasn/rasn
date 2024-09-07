//! A module that contains `SET`, `SEQUENCE`, `SET OF` and `SEQUENCE OF` types.

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
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Push a value to the `SetOf`.
    pub fn add(&mut self, value: T) {
        self.0.push(value);
    }
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.0.iter_mut()
    }
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.sort();
    }
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        self.0.sort_by(compare);
    }
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> Default for SetOf<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> PartialEq for SetOf<T>
where
    T: PartialEq + Ord + Clone,
{
    /// Compare two `SetOf` values for equality.
    /// The order of elements in the `SetOf` does not matter.
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut first = self.0.clone();
        first.sort_unstable();
        let mut second = other.0.clone();
        second.sort_unstable();
        first.eq(&second)
    }
}
impl<T: Ord + Clone> Eq for SetOf<T> {}

impl<T: Ord + Clone> PartialOrd for SetOf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> Ord for SetOf<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // First, compare by length
        match self.0.len().cmp(&other.0.len()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        // If lengths are equal, compare sorted contents
        // let mut self_sorted = self.0.clone();
        // let mut other_sorted = other.0.clone();
        // self_sorted.sort();
        // other_sorted.sort();
        self.0.cmp(&other.0)

        // self_sorted.cmp(&other_sorted)
    }
}
//impl hash
impl<T: core::hash::Hash + Clone + Ord> core::hash::Hash for SetOf<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let mut sorted = self.0.clone();
        sorted.sort_unstable();
        sorted.hash(state);
    }
}
