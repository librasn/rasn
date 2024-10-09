//! Constraints of values on a given type.

use super::IntegerType;
use alloc::borrow::Cow;
use num_bigint::BigInt;

/// A set of constraints for a given type on what kinds of values are allowed.
/// Used in certain codecs to optimise encoding and decoding values.
#[derive(Debug, Clone)]
pub struct Constraints<'constraint>(pub Cow<'constraint, [Constraint]>);

impl<'r> Constraints<'r> {
    /// No constraints on a given type.
    pub const NONE: Self = Self(Cow::Borrowed(&[]));

    /// Creates a set of constraints from a set of values.
    pub const fn new(constraints: &'r [Constraint]) -> Self {
        Self(Cow::Borrowed(constraints))
    }

    /// A const variant of the default function.
    pub const fn default() -> Self {
        Self::NONE
    }

    /// Overrides a set of constraints with another set.
    #[inline(always)]
    pub fn override_constraints(mut self, mut rhs: Constraints) -> Constraints {
        let mut i = 0;
        while i < self.0.len() {
            if !rhs.0.iter().any(|child| child.kind() == self.0[i].kind()) {
                // No matching constraint in rhs, so move it
                let parent = self.0.to_mut().swap_remove(i);
                rhs.0.to_mut().push(parent);
            } else {
                i += 1;
            }
        }
        rhs
    }

    /// Returns the size constraint from the set, if available.
    pub fn size(&self) -> Option<&Extensible<Size>> {
        self.0.iter().find_map(|constraint| constraint.to_size())
    }

    /// Returns the permitted alphabet constraint from the set, if available.
    pub fn permitted_alphabet(&self) -> Option<&Extensible<PermittedAlphabet>> {
        self.0
            .iter()
            .find_map(|constraint| constraint.as_permitted_alphabet())
    }

    /// Returns whether any of the constraints are extensible.
    pub fn extensible(&self) -> bool {
        self.0.iter().any(|constraint| constraint.is_extensible())
    }

    /// Returns the value constraint from the set, if available.
    pub fn value(&self) -> Option<&Extensible<Value>> {
        self.0.iter().find_map(|constraint| constraint.to_value())
    }
}

impl<'r> From<&'r [Constraint]> for Constraints<'r> {
    fn from(constraints: &'r [Constraint]) -> Self {
        Self::new(constraints)
    }
}

impl<'r, const N: usize> From<&'r [Constraint; N]> for Constraints<'r> {
    fn from(constraints: &'r [Constraint; N]) -> Self {
        Self::new(constraints)
    }
}

/// The set of possible constraints a given value can have.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Constraint {
    /// A set of possible values which the type can be.
    Value(Extensible<Value>),
    /// The amount of possible values the type can have.
    Size(Extensible<Size>),
    /// The set of possible characters the type can have.
    PermittedAlphabet(Extensible<PermittedAlphabet>),
    /// The value itself is extensible, only valid for constructed types,
    /// choices, or enumerated values.
    Extensible,
}

/// The discriminant of [Constraint] values.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum ConstraintDiscriminant {
    Value,
    Size,
    PermittedAlphabet,
    Extensible,
}

impl Constraint {
    /// Returns the discriminant of the value.
    pub const fn kind(&self) -> ConstraintDiscriminant {
        match self {
            Self::Value(_) => ConstraintDiscriminant::Value,
            Self::Size(_) => ConstraintDiscriminant::Size,
            Self::PermittedAlphabet(_) => ConstraintDiscriminant::PermittedAlphabet,
            Self::Extensible => ConstraintDiscriminant::Extensible,
        }
    }

    /// Returns the value constraint, if set.
    pub const fn as_value(&self) -> Option<&Extensible<Value>> {
        match self {
            Self::Value(integer) => Some(integer),
            _ => None,
        }
    }

    /// Returns the permitted alphabet constraint, if set.
    pub const fn as_permitted_alphabet(&self) -> Option<&Extensible<PermittedAlphabet>> {
        match self {
            Self::PermittedAlphabet(alphabet) => Some(alphabet),
            _ => None,
        }
    }

    /// Returns the size constraint, if set.
    pub const fn to_size(&self) -> Option<&Extensible<Size>> {
        match self {
            Self::Size(size) => Some(size),
            _ => None,
        }
    }

    /// Returns the value constraint, if set.
    pub const fn to_value(&self) -> Option<&Extensible<Value>> {
        match self {
            Self::Value(integer) => Some(integer),
            _ => None,
        }
    }

    /// Returns whether the type is extensible.
    pub const fn is_extensible(&self) -> bool {
        match self {
            Self::Value(value) => value.extensible.is_some(),
            Self::Size(size) => size.extensible.is_some(),
            Self::PermittedAlphabet(alphabet) => alphabet.extensible.is_some(),
            Self::Extensible => true,
        }
    }
}

/// A wrapper around [Constraint] covering whether the constraint is "extensible".
///
/// Extensible means that it can have values outside of its constraints, and what possible
/// constraints thosevalues in the extended set can have, if any.
#[derive(Debug, Copy, Default, Clone, PartialEq)]
pub struct Extensible<T: 'static> {
    /// The underlying constraint type.
    pub constraint: T,
    /// Whether the constraint is extensible, and if it is, a list of extensible
    /// constraints.
    pub extensible: Option<&'static [T]>,
}

impl<T> Extensible<T> {
    /// Creates a new wrapper around a given constraint, by default this means
    /// that the underlying constraint is not extensible.
    pub const fn new(constraint: T) -> Self {
        Self {
            constraint,
            extensible: None,
        }
    }

    /// Creates a new extensible constraint with a given set of constraints
    /// on the extended values.
    pub const fn new_extensible(constraint: T, constraints: &'static [T]) -> Self {
        Self {
            constraint,
            extensible: Some(constraints),
        }
    }

    /// Sets the constraint to be extensible with no constraints on extended
    /// values.
    pub const fn set_extensible(self, extensible: bool) -> Self {
        let extensible = if extensible {
            let empty: &[T] = &[];
            Some(empty)
        } else {
            None
        };

        self.extensible_with_constraints(extensible)
    }

    /// Sets the constraint to either not be extended or extensible with a set
    /// of constraints.
    pub const fn extensible_with_constraints(mut self, constraints: Option<&'static [T]>) -> Self {
        self.extensible = constraints;
        self
    }
}

impl From<Value> for Extensible<Value> {
    fn from(value: Value) -> Self {
        Self {
            constraint: value,
            extensible: None,
        }
    }
}

impl From<Size> for Extensible<Size> {
    fn from(size: Size) -> Self {
        Self {
            constraint: size,
            extensible: None,
        }
    }
}

impl From<PermittedAlphabet> for Extensible<PermittedAlphabet> {
    fn from(alphabet: PermittedAlphabet) -> Self {
        Self {
            constraint: alphabet,
            extensible: None,
        }
    }
}

/// A single or range of numeric values a type can be.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(pub(crate) Bounded<i128>);

impl Value {
    /// Creates a new value constraint from a given bound.
    pub const fn new(value: Bounded<i128>) -> Self {
        Self(value)
    }
}

impl core::ops::Deref for Value {
    type Target = Bounded<i128>;

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
            impl From<Bounded<$int>> for Value {
                fn from(bounded: Bounded<$int>) -> Self {
                    Self(match bounded {
                        Bounded::Range { start, end } => Bounded::Range {
                            start: start.map(From::from),
                            end: end.map(From::from),
                        },
                        Bounded::Single(value) => Bounded::Single(value.into()),
                        Bounded::None => Bounded::None,
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

impl TryFrom<Bounded<usize>> for Value {
    type Error = <i128 as TryFrom<usize>>::Error;

    fn try_from(bounded: Bounded<usize>) -> Result<Self, Self::Error> {
        Ok(Self(match bounded {
            Bounded::Range { start, end } => Bounded::Range {
                start: start.map(TryFrom::try_from).transpose()?,
                end: end.map(TryFrom::try_from).transpose()?,
            },
            Bounded::Single(value) => Bounded::Single(value.try_into()?),
            Bounded::None => Bounded::None,
        }))
    }
}

/// A single or range of length values a type can have.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size(pub(crate) Bounded<usize>);

impl Size {
    #[must_use]
    /// Creates a varying range constraint.
    pub const fn new(range: Bounded<usize>) -> Self {
        Self(range)
    }

    #[must_use]
    /// Creates a fixed size constraint.
    pub const fn fixed(length: usize) -> Self {
        Self(Bounded::Single(length))
    }

    #[must_use]
    /// Returns whether the size is fixed.
    pub const fn is_fixed(&self) -> bool {
        matches!(self.0, Bounded::Single(_))
    }
    /// Returns whether the size has a varying range.
    #[must_use]
    pub const fn is_range(&self) -> bool {
        matches!(self.0, Bounded::Range { .. })
    }
}

impl core::ops::Deref for Size {
    type Target = Bounded<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Size {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A range of alphabet characters a type can have.
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermittedAlphabet(&'static [u32]);

impl PermittedAlphabet {
    /// Creates a new constraint from a given range.
    pub const fn new(range: &'static [u32]) -> Self {
        Self(range)
    }

    /// Returns the range of allowed possible values.
    pub const fn as_inner(&self) -> &'static [u32] {
        self.0
    }
}

impl core::ops::Deref for PermittedAlphabet {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// A set of potential bounded values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bounded<T> {
    /// No bounds on a given type.
    #[default]
    None,
    /// A single value is permitted for a given type.
    Single(T),
    /// the range of values permitted for a given type.
    Range {
        /// The lower bound of the range, if any.
        start: Option<T>,
        /// The upper bound of the range, if any.
        end: Option<T>,
    },
}

impl<T> Bounded<T> {
    /// Creates a bounded range that starts from value and has no end.
    pub const fn start_from(value: T) -> Self {
        Self::Range {
            start: Some(value),
            end: None,
        }
    }

    /// Creates a bounded range that ends at value and has no defined start.
    pub const fn up_to(value: T) -> Self {
        Self::Range {
            start: None,
            end: Some(value),
        }
    }

    /// Creates new bound from a single value.
    pub const fn single_value(value: T) -> Self {
        Self::Single(value)
    }

    /// Returns the lower bound of a given range, if any.
    pub const fn as_start(&self) -> Option<&T> {
        match &self {
            Self::Range { start, .. } => start.as_ref(),
            Self::Single(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the upper bound of a given range, if any.
    pub const fn as_end(&self) -> Option<&T> {
        match &self {
            Self::Range { end, .. } => end.as_ref(),
            Self::Single(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the bounds of a given range, if any.
    pub const fn start_and_end(&self) -> (Option<&T>, Option<&T>) {
        match &self {
            Self::Range { start, end } => (start.as_ref(), end.as_ref()),
            Self::Single(value) => (Some(value), Some(value)),
            _ => (None, None),
        }
    }
}

impl<T: Copy + IntegerType> Bounded<T> {
    /// Assuming T is an integer, returns the minimum possible bound, or zero
    /// if not present.
    pub const fn minimum(&self) -> T {
        match self.as_start() {
            Some(value) => *value,
            None => T::ZERO,
        }
    }
}

impl<T: num_traits::WrappingSub<Output = T> + num_traits::SaturatingAdd<Output = T> + From<u8>>
    Bounded<T>
{
    /// Returns the number representing the difference between the lower and upper bound.
    pub fn range(&self) -> Option<T> {
        match self {
            Self::Single(_) => Some(T::from(1u8)),
            Self::Range {
                start: Some(start),
                end: Some(end),
            } => Some(end.wrapping_sub(start).saturating_add(&1.into())),
            _ => None,
        }
    }
}

impl<T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + core::cmp::PartialOrd> Bounded<T>
where
    for<'a> &'a T: core::ops::Sub<Output = T>,
{
    /// Returns the effective value which is either the number, or the positive
    /// offset of that number from the start of the value range. `Either::Left`
    /// represents the positive offset, and `Either::Right` represents
    /// the number.
    pub fn effective_value(&self, value: T) -> either::Either<T, T> {
        match self {
            Self::Range {
                start: Some(start), ..
            } => {
                debug_assert!(&value >= start);
                either::Left(&value - start)
            }
            _ => either::Right(value),
        }
    }
}

impl<T> Bounded<T>
where
    T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + PartialOrd<T>,
{
    /// The same as [`Self::effective_value`] except using [`crate::types::Integer<I>`].
    pub fn effective_integer_value<I>(self, value: I) -> either::Either<I, I>
    where
        I: IntegerType + core::ops::Sub<Output = I>,
        I: From<T> + PartialOrd,
    {
        if let Bounded::Range {
            start: Some(start), ..
        } = self
        {
            let start = I::from(start);
            debug_assert!(value >= start);
            either::Left(value - start)
        } else {
            either::Right(value)
        }
    }
}

impl From<Value> for Constraint {
    fn from(size: Value) -> Self {
        Self::Value(size.into())
    }
}

impl From<Extensible<Value>> for Constraint {
    fn from(size: Extensible<Value>) -> Self {
        Self::Value(size)
    }
}

impl From<Size> for Constraint {
    fn from(size: Size) -> Self {
        Self::Size(size.into())
    }
}

impl From<Extensible<Size>> for Constraint {
    fn from(size: Extensible<Size>) -> Self {
        Self::Size(size)
    }
}

impl From<PermittedAlphabet> for Constraint {
    fn from(size: PermittedAlphabet) -> Self {
        Self::PermittedAlphabet(size.into())
    }
}

impl From<Extensible<PermittedAlphabet>> for Constraint {
    fn from(size: Extensible<PermittedAlphabet>) -> Self {
        Self::PermittedAlphabet(size)
    }
}

impl Bounded<i128> {
    /// Returns `true` if the given element is within the bounds of the constraint.
    /// Constraint type is `i128` here, so we can make checks based on that.
    pub fn in_bound<I: IntegerType>(&self, element: &I) -> bool {
        match &self {
            Self::Single(value) => {
                if let Some(e) = element.to_i128() {
                    e == *value
                } else {
                    false
                }
            }
            Self::Range { start, end } => {
                start.as_ref().map_or(true, |&start| {
                    if let Some(e) = element.to_i128() {
                        e >= start
                    } else if let Some(e) = element.to_bigint() {
                        e >= BigInt::from(start)
                    } else {
                        false
                    }
                }) && end.as_ref().map_or(true, |&end| {
                    if let Some(e) = element.to_i128() {
                        e <= end
                    } else if let Some(e) = element.to_bigint() {
                        e <= BigInt::from(end)
                    } else {
                        false
                    }
                })
            }
            Self::None => true,
        }
    }
}

impl<T: PartialEq + PartialOrd> Bounded<T> {
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
        Self::Range {
            start: Some(start),
            end: Some(end),
        }
    }

    /// Returns whether a given element is contained within a bound.
    pub fn contains(&self, element: &T) -> bool {
        match &self {
            Self::Single(value) => value == element,
            Self::Range { start, end } => {
                start.as_ref().map_or(true, |start| element >= start)
                    && end.as_ref().map_or(true, |end| element <= end)
            }
            Self::None => true,
        }
    }

    /// Returns whether a given element is contained within a bound, returning
    /// an error if not.
    pub fn contains_or<E>(&self, element: &T, error: E) -> Result<(), E> {
        self.contains_or_else(element, || error)
    }

    /// Returns whether a given element is contained within a bound, returning
    /// an error if not.
    pub fn contains_or_else<E>(&self, element: &T, error: impl FnOnce() -> E) -> Result<(), E> {
        match self.contains(element) {
            true => Ok(()),
            false => Err((error)()),
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Bounded<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Range { start, end } => match (start.as_ref(), end.as_ref()) {
                (Some(start), Some(end)) => write!(f, "{start}..{end}"),
                (Some(start), None) => write!(f, "{start}.."),
                (None, Some(end)) => write!(f, "..{end}"),
                (None, None) => write!(f, ".."),
            },
            Self::Single(value) => value.fmt(f),
            Self::None => write!(f, ".."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let constraints = Bounded::new(0, 255);
        assert_eq!(256, constraints.range().unwrap());
    }
}
