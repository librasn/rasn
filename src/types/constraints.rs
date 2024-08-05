use alloc::borrow::Cow;

use super::IntegerType;
use num_bigint::BigInt;

#[derive(Debug, Default, Clone)]
pub struct Constraints<'constraint>(pub Cow<'constraint, [Constraint]>);

impl<'r> Constraints<'r> {
    pub const NONE: Self = Self(Cow::Borrowed(&[]));

    pub const fn new(constraints: &'r [Constraint]) -> Self {
        Self(Cow::Borrowed(constraints))
    }

    /// Overrides a set of constraints with another set.
    pub fn override_constraints(self, mut rhs: Constraints) -> Constraints {
        for parent in self.0.iter() {
            if !rhs.0.iter().any(|child| child.kind() == parent.kind()) {
                rhs.0.to_mut().push(parent.clone());
            }
        }

        rhs
    }

    pub fn size(&self) -> Option<&Extensible<Size>> {
        self.0.iter().find_map(|constraint| constraint.to_size())
    }

    pub fn permitted_alphabet(&self) -> Option<&Extensible<PermittedAlphabet>> {
        self.0
            .iter()
            .find_map(|constraint| constraint.as_permitted_alphabet())
    }

    pub fn extensible(&self) -> bool {
        self.0.iter().any(|constraint| constraint.is_extensible())
    }

    pub fn value(&self) -> Option<Extensible<Value>> {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Value(Extensible<Value>),
    Size(Extensible<Size>),
    PermittedAlphabet(Extensible<PermittedAlphabet>),
    /// The value itself is extensible, only valid for constructed types,
    /// choices, or enumerated values.
    Extensible,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstraintDiscriminant {
    Value,
    Size,
    PermittedAlphabet,
    Extensible,
}

impl Constraint {
    pub const fn kind(&self) -> ConstraintDiscriminant {
        match self {
            Self::Value(_) => ConstraintDiscriminant::Value,
            Self::Size(_) => ConstraintDiscriminant::Size,
            Self::PermittedAlphabet(_) => ConstraintDiscriminant::PermittedAlphabet,
            Self::Extensible => ConstraintDiscriminant::Extensible,
        }
    }

    pub const fn as_value(&self) -> Option<&Extensible<Value>> {
        match self {
            Self::Value(integer) => Some(integer),
            _ => None,
        }
    }

    pub fn as_permitted_alphabet(&self) -> Option<&Extensible<PermittedAlphabet>> {
        match self {
            Self::PermittedAlphabet(alphabet) => Some(alphabet),
            _ => None,
        }
    }

    pub fn to_size(&self) -> Option<&Extensible<Size>> {
        match self {
            Self::Size(size) => Some(size),
            _ => None,
        }
    }

    pub fn to_value(&self) -> Option<Extensible<Value>> {
        match self {
            Self::Value(integer) => Some(integer.clone()),
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Extensible<T: 'static> {
    pub constraint: T,
    /// Whether the constraint is extensible, and if it is, a list of extensible
    /// constraints.
    pub extensible: Option<&'static [T]>,
}

impl<T> Extensible<T> {
    pub const fn new(constraint: T) -> Self {
        Self {
            constraint,
            extensible: None,
        }
    }

    pub const fn new_extensible(constraint: T, constraints: &'static [T]) -> Self {
        Self {
            constraint,
            extensible: Some(constraints),
        }
    }

    pub const fn set_extensible(self, extensible: bool) -> Self {
        let extensible = if extensible {
            let empty: &[T] = &[];
            Some(empty)
        } else {
            None
        };

        self.extensible_with_constraints(extensible)
    }

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(pub(crate) Bounded<i128>);

impl Value {
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
    pub fn is_fixed(&self) -> bool {
        matches!(self.0, Bounded::Single(_))
    }
    /// Returns whether the size has a varying range.
    #[must_use]
    pub fn is_range(&self) -> bool {
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
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bounded<T> {
    #[default]
    None,
    Single(T),
    Range {
        start: Option<T>,
        end: Option<T>,
    },
}

impl<T> Bounded<T> {
    pub const fn start_from(value: T) -> Self {
        Self::Range {
            start: Some(value),
            end: None,
        }
    }

    pub const fn up_to(value: T) -> Self {
        Self::Range {
            start: None,
            end: Some(value),
        }
    }

    pub const fn as_start(&self) -> Option<&T> {
        match &self {
            Self::Range { start, .. } => start.as_ref(),
            Self::Single(value) => Some(value),
            _ => None,
        }
    }

    pub const fn as_end(&self) -> Option<&T> {
        match &self {
            Self::Range { end, .. } => end.as_ref(),
            Self::Single(value) => Some(value),
            _ => None,
        }
    }

    pub const fn start_and_end(&self) -> (Option<&T>, Option<&T>) {
        match &self {
            Self::Range { start, end } => (start.as_ref(), end.as_ref()),
            Self::Single(value) => (Some(value), Some(value)),
            _ => (None, None),
        }
    }

    pub const fn single_value(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T: Default + Clone> Bounded<T> {
    pub fn as_minimum(&self) -> Option<&T> {
        match self {
            Self::Single(value) => Some(value),
            Self::Range {
                start: Some(start), ..
            } => Some(start),
            _ => None,
        }
    }

    pub fn minimum(&self) -> T {
        self.as_minimum().cloned().unwrap_or_default()
    }
}

impl<T: num_traits::WrappingSub<Output = T> + num_traits::SaturatingAdd<Output = T> + From<u8>>
    Bounded<T>
{
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

impl<T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + Clone + PartialOrd> Bounded<T> {
    /// Returns the effective value which is either the number, or the positive
    /// offset of that number from the start of the value range. `Either::Left`
    /// represents the positive offset, and `Either::Right` represents
    /// the number.
    pub fn effective_value(&self, value: T) -> either::Either<T, T> {
        match &self {
            Self::Range {
                start: Some(start), ..
            } => {
                debug_assert!(&value >= start);
                either::Left(value - start.clone())
            }
            _ => either::Right(value),
        }
    }
}

impl<T> Bounded<T>
where
    T: core::ops::Sub<Output = T> + core::fmt::Debug + Default + Clone + PartialOrd<T>,
{
    /// The same as [`effective_value`] except using [`crate::types::Integer<I>`].
    pub fn effective_integer_value<I>(&self, value: I) -> either::Either<I, I>
    where
        I: IntegerType + core::ops::Sub<Output = I>,
        I: From<T> + PartialOrd,
    {
        if let Bounded::Range {
            start: Some(start), ..
        } = self
        {
            let start = I::from(start.clone());
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
    pub fn in_bound<I: IntegerType>(&self, element: &crate::types::Integer<I>) -> bool {
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
