use crate::types::Integer;

#[derive(Debug, Default, Copy, Clone)]
pub struct Constraints<'constraint>(pub &'constraint [Constraint]);

impl Constraints<'_> {
    pub const NONE: Self = Self(&[]);

    pub fn size(&self) -> Range<usize> {
        self.0
            .iter()
            .find_map(|constraint| constraint.to_size())
            .unwrap_or_default()
    }

    pub fn extensible(&self) -> Option<bool> {
        self.0
            .iter()
            .find_map(|constraint| constraint.to_extensible())
    }

    pub fn value(&self) -> Value {
        self.0
            .iter()
            .find_map(|constraint| constraint.to_value())
            .unwrap_or_default()
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
    Size(Range<usize>),
    Extensible(bool),
}

impl Constraint {
    pub const fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Value(integer) => Some(integer),
            _ => None,
        }
    }

    pub const fn to_extensible(&self) -> Option<bool> {
        match self {
            Self::Extensible(value) => Some(*value),
            _ => None,
        }
    }

    pub const fn to_size(&self) -> Option<Range<usize>> {
        match self {
            Self::Size(size) => Some(*size),
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
        matches!(self, Self::Extensible(_))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(Range<Integer>);

impl Value {
    pub const fn new(value: Range<Integer>) -> Self {
        Self(value)
    }
}

impl core::ops::Deref for Value {
    type Target = Range<Integer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Range<usize>> for Value {
    fn from(range: Range<usize>) -> Self {
        Self(Range {
            start: range.start.map(From::from),
            end: range.end.map(From::from),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size(Range<usize>);

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

impl<T: Clone> Range<T> {
    pub fn single_value(value: T) -> Self {
        Self {
            start: Some(value.clone()),
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

impl<T: core::ops::Sub<Output = T> + Clone> Range<T> {
    pub fn range(&self) -> Option<T> {
        match (self.start.clone(), self.end.clone()) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }
}

impl<T: PartialEq + PartialOrd> Range<T> {
    pub fn contains(&self, element: &T) -> bool {
        self.start.as_ref().map_or(true, |start| element >= start)
            && self.end.as_ref().map_or(true, |end| element < end)
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

impl From<Range<Integer>> for Value {
    fn from(size: Range<Integer>) -> Self {
        Self(Range {
            start: size.start.map(From::from),
            end: size.end.map(From::from),
        })
    }
}

impl From<Value> for Constraint {
    fn from(size: Value) -> Self {
        Self::Value(size)
    }
}

impl From<Range<usize>> for Constraint {
    fn from(size: Range<usize>) -> Self {
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

    fn range() {
        let constraints = Range::new(0, 255);
        assert_eq!(256, constraints.range().unwrap());
    }
}
