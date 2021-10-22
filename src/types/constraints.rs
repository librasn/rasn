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

    pub fn value(&self) -> Range<Integer> {
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
    Value(Range<Integer>),
    Size(Range<usize>),
    Extensible(bool),
}

impl Constraint {
    pub fn to_extensible(&self) -> Option<bool> {
        match self {
            Self::Extensible(value) => Some(*value),
            _ => None,
        }
    }

    pub fn to_size(&self) -> Option<Range<usize>> {
        match self {
            Self::Size(size) => Some(*size),
            _ => None,
        }
    }

    pub fn to_value(&self) -> Option<Range<Integer>> {
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Range<T> {
    min: Option<T>,
    max: Option<T>,
}

impl<T> Range<T> {
    pub fn start_from(value: T) -> Self {
        Self {
            min: Some(value),
            max: None,
        }
    }

    pub fn up_to(value: T) -> Self {
        Self {
            min: None,
            max: Some(value),
        }
    }

    pub fn as_min(&self) -> Option<&T> {
        self.min.as_ref()
    }
}

impl<T: Clone> Range<T> {
    pub fn single_value(value: T) -> Self {
        Self {
            min: Some(value.clone()),
            max: Some(value),
        }
    }
}

impl<T: Default + Clone> Range<T> {
    pub fn min(&self) -> T {
        self.min.clone().unwrap_or_default()
    }

    pub fn max(&self) -> Option<T> {
        self.max.clone()
    }
}

impl<T: core::ops::Sub<Output = T> + Clone> Range<T> {
    pub fn range(&self) -> Option<T> {
        match (self.min.clone(), self.max.clone()) {
            (Some(min), Some(max)) => Some(max - min),
            _ => None,
        }
    }
}

impl<T: PartialEq + PartialOrd> Range<T> {
    pub fn contains(&self, element: &T) -> bool {
        self.min.as_ref().map_or(true, |min| element >= min)
            && self.max.as_ref().map_or(true, |max| element < max)
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
        if let Some(min) = self.min.clone() {
            debug_assert!(value >= min);
            either::Left(value - min)
        } else {
            either::Right(value)
        }
    }
}

impl From<Range<usize>> for Range<Integer> {
    fn from(size: Range<usize>) -> Self {
        Self {
            min: size.min.map(From::from),
            max: size.max.map(From::from),
        }
    }
}

impl From<Range<Integer>> for Constraint {
    fn from(size: Range<Integer>) -> Self {
        Self::Value(size)
    }
}

impl From<Range<usize>> for Constraint {
    fn from(size: Range<usize>) -> Self {
        Self::Size(size)
    }
}

impl<T: PartialEq + PartialOrd> Range<T> {
    pub fn new(min: T, max: T) -> Self {
        debug_assert!(min < max);

        Self {
            min: Some(min),
            max: Some(max),
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Range<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match (self.min.as_ref(), self.max.as_ref()) {
            (Some(min), Some(max)) => write!(f, "{min}..{max}"),
            (Some(min), None) => write!(f, "{min}.."),
            (None, Some(max)) => write!(f, "..{max}"),
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
