#[derive(Debug, Default, Copy, Clone)]
pub struct Constraints<'constraint>(pub &'constraint [Constraint]);

impl<'r> Constraints<'r> {
    pub const NONE: Self = Self(&[]);

    pub const fn new(constraints: &'r [Constraint]) -> Self {
        Self(constraints)
    }

    pub fn size(&self) -> Range<usize> {
        self.0
            .iter()
            .find_map(|constraint| constraint.to_size())
            .unwrap_or_default()
    }

    pub fn permitted_alphabet(&self) -> Option<&'static [u32]> {
        self.0
            .iter()
            .find_map(|constraint| constraint.as_permitted_alphabet())
    }

    pub fn extensible(&self) -> bool {
        self.0.iter().any(|constraint| constraint.is_extensible())
    }

    pub fn value(&self) -> Value {
        self.0
            .iter()
            .find_map(|constraint| constraint.to_value())
            .unwrap_or_default()
    }
}

impl<'r> From<&'r [Constraint]> for Constraints<'r> {
    fn from(constraints: &'r [Constraint]) -> Self {
        Self(constraints)
    }
}

impl<'r, const N: usize> From<&'r [Constraint; N]> for Constraints<'r> {
    fn from(constraints: &'r [Constraint; N]) -> Self {
        Self(constraints)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Value(Value),
    Size(Size),
    PermittedAlphabet(PermittedAlphabet),
    Extensible,
}

impl Constraint {
    pub const fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Value(integer) => Some(integer),
            _ => None,
        }
    }

    pub fn as_permitted_alphabet(&self) -> Option<&'static [u32]> {
        match self {
            Self::PermittedAlphabet(alphabet) => Some(alphabet.as_inner()),
            _ => None,
        }
    }

    pub fn to_size(&self) -> Option<Range<usize>> {
        match self {
            Self::Size(size) => Some(**size),
            _ => None,
        }
    }

    pub fn to_value(&self) -> Option<Value> {
        match self {
            Self::Value(integer) => Some(integer.clone()),
            _ => None,
        }
    }

    /// Returns whether the type is extensible.
    pub const fn is_extensible(&self) -> bool {
        matches!(self, Self::Extensible)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(Range<i128>);

impl Value {
    pub const fn new(value: Range<i128>) -> Self {
        Self(value)
    }
}

impl core::ops::Deref for Value {
    type Target = Range<i128>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! from_primitives {
    ($($int:ty),+ $(,)?) => {
        $(
            impl From<Range<$int>> for Value {
                fn from(range: Range<$int>) -> Self {
                    Self(Range {
                        start: range.start.map(From::from),
                        end: range.end.map(From::from),
                    })
                }
            }
        )+
    }
}

from_primitives! {
    u8, u16, u32, u64,
    i8, i16, i32, i64, i128,
}

impl TryFrom<Range<usize>> for Value {
    type Error = <i128 as TryFrom<usize>>::Error;

    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        Ok(Self(Range {
            start: range.start.map(TryFrom::try_from).transpose()?,
            end: range.end.map(TryFrom::try_from).transpose()?,
        }))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size(Range<usize>);

impl Size {
    pub const fn new(range: Range<usize>) -> Self {
        Self(range)
    }
}

impl core::ops::Deref for Size {
    type Target = Range<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Size {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermittedAlphabet(&'static [u32]);

impl PermittedAlphabet {
    pub const fn new(range: &'static [u32]) -> Self {
        Self(range)
    }

    pub fn as_inner(&self) -> &'static [u32] {
        self.0
    }
}

impl core::ops::Deref for PermittedAlphabet {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range<T> {
    start: Option<T>,
    end: Option<T>,
}

impl<T> Range<T> {
    pub const fn start_from(value: T) -> Self {
        Self {
            start: Some(value),
            end: None,
        }
    }

    pub const fn up_to(value: T) -> Self {
        Self {
            start: None,
            end: Some(value),
        }
    }

    pub const fn as_start(&self) -> Option<&T> {
        self.start.as_ref()
    }
}

impl<T: Clone + Copy> Range<T> {
    pub const fn single_value(value: T) -> Self {
        Self {
            start: Some(value),
            end: Some(value),
        }
    }
}

impl<T: Default + Clone> Range<T> {
    pub fn start(&self) -> T {
        self.start.clone().unwrap_or_default()
    }

    pub fn end(&self) -> Option<T> {
        self.end.clone()
    }
}

impl<T: PartialEq + PartialOrd + num_traits::WrappingSub<Output = T> + std::ops::Add<Output = T> + From<u8> + Clone + core::fmt::Debug> Range<T> {
    pub fn range(&self) -> Option<T> {
        match (&self.start, &self.end) {
            (Some(start), Some(end)) => {
                Some(
                    end.wrapping_sub(start) + (start == &T::from(0))
                        .then(|| T::from(1))
                        .unwrap_or_else(|| T::from(0))
                )
            }
            _ => None,
        }
    }
}

impl<T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + Clone + PartialOrd> Range<T> {
    /// Returns the effective value which is either the number, or the positive
    /// offset of that number from the start of the value range. `Either::Left`
    /// represents the positive offset, and `Either::Right` represents
    /// the number.
    pub fn effective_value(&self, value: T) -> either::Either<T, T> {
        if let Some(start) = self.start.clone() {
            debug_assert!(value >= start);
            either::Left(value - start)
        } else {
            either::Right(value)
        }
    }
}

impl<T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + Clone + PartialOrd<T>> Range<T>
    where crate::types::Integer: From<T>
{
    /// The same as [`effective_value`] except using [`crate::types::Integer`].
    pub fn effective_bigint_value(&self, value: crate::types::Integer) -> either::Either<crate::types::Integer, crate::types::Integer> {
        if let Some(start) = self.start.clone().map(crate::types::Integer::from) {
            debug_assert!(value >= start);
            either::Left(value - start)
        } else {
            either::Right(value)
        }
    }
}

impl From<Value> for Constraint {
    fn from(size: Value) -> Self {
        Self::Value(size)
    }
}

impl From<Size> for Constraint {
    fn from(size: Size) -> Self {
        Self::Size(size)
    }
}

impl<T: PartialEq + PartialOrd> Range<T> {
    /// Creates a new range from `start` to `end`.
    ///
    /// # Panics
    /// When `start > end`.
    pub fn new(start: T, end: T) -> Self {
        debug_assert!(start <= end);
        Self::const_new(start, end)
    }

    /// Const compatible range constructor.
    ///
    /// # Safety
    /// Requires `start <= end` otherwise functions will return incorrect results..
    /// In general you should prefer [`Self::new`] which has debug assertions
    /// to ensure this.
    pub const fn const_new(start: T, end: T) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.start.as_ref().map_or(true, |start| element >= start)
            && self.end.as_ref().map_or(true, |end| element <= end)
    }

    pub fn contains_or<E>(&self, element: &T, error: E) -> Result<(), E> {
        self.contains_or_else(element, || error)
    }

    pub fn contains_or_else<E>(&self, element: &T, error: impl FnOnce() -> E) -> Result<(), E> {
        match self.contains(element) {
            true => Ok(()),
            false => Err((error)()),
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Range<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match (self.start.as_ref(), self.end.as_ref()) {
            (Some(start), Some(end)) => write!(f, "{start}..{end}"),
            (Some(start), None) => write!(f, "{start}.."),
            (None, Some(end)) => write!(f, "..{end}"),
            (None, None) => write!(f, ".."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let constraints = Range::new(0, 255);
        assert_eq!(256, constraints.range().unwrap());
    }
}
