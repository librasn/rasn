//! Constraints of values on a given type.

use super::IntegerType;
use num_bigint::BigInt;

// TODO type can have multiple constraints of same kind, up to infinite
// const SUPPORTED_CONSTRAINTS_COUNT: usize = 5;

// ASN.1 has typically union and intersection constraints
// add union and intersection methods?
// Typically constraints must match all the constraints applied to them, so unless specified otherwise
// we should use intersection for these constraints.

/// A set of constraints for a given type on what kinds of values are allowed.
/// Used in certain codecs to optimise encoding and decoding values.
///
/// The effective constraint is typically the intersection or union among other constraints.
/// As a result, we can store one constraint of each kind, updated with the latest constraint.
///
/// TODO architecture needs a re-design - multple different-style value constraints are allowed
/// for example, value constraint can have multiple single values, or a range of values which do not overlap.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Constraints {
    value: Option<Extensible<Value>>,
    size: Option<Extensible<Size>>,
    permitted_alphabet: Option<Extensible<PermittedAlphabet>>,
    extensible: bool,
}

impl<'constraint> Constraints {
    /// Empty constraints.
    pub const NONE: Self = Self {
        value: None,
        size: None,
        permitted_alphabet: None,
        extensible: false,
    };

    /// Creates a new set of constraints from a given slice.
    pub const fn new(constraints: &'constraint [Constraint]) -> Self {
        let mut value: Option<Extensible<Value>> = None;
        let mut size: Option<Extensible<Size>> = None;
        let mut permitted_alphabet: Option<Extensible<PermittedAlphabet>> = None;
        let mut extensible = false;
        let mut i = 0;
        while i < constraints.len() {
            match constraints[i] {
                Constraint::Value(v) => {
                    value = match value {
                        Some(value) => Some(value.intersect(&v)),
                        None => Some(v),
                    };
                }
                Constraint::Size(s) => {
                    size = if let Some(size) = size {
                        Some(size.intersect(&s))
                    } else {
                        Some(s)
                    };
                }
                Constraint::PermittedAlphabet(p) => {
                    permitted_alphabet = if let Some(perm) = permitted_alphabet {
                        Some(perm.intersect(&p))
                    } else {
                        Some(p)
                    };
                }
                Constraint::Extensible => {
                    extensible = true;
                }
                Constraint::Empty => {}
            }
            i += 1;
        }
        Self {
            value,
            size,
            permitted_alphabet,
            extensible,
        }
    }

    /// A const variant of the default function.
    pub const fn default() -> Self {
        Self::NONE
    }

    /// Merges a set of constraints with another set, if they don't exist.
    /// This function should be only used on compile-time.
    pub const fn intersect(&self, rhs: Constraints) -> Self {
        let value = match (self.value, rhs.value) {
            (Some(value), Some(rhs_value)) => Some(value.intersect(&rhs_value)),
            (Some(value), None) => Some(value),
            (None, Some(rhs_value)) => Some(rhs_value),
            (None, None) => None,
        };
        let size = match (self.size, rhs.size) {
            (Some(size), Some(rhs_size)) => Some(size.intersect(&rhs_size)),
            (Some(size), None) => Some(size),
            (None, Some(rhs_size)) => Some(rhs_size),
            (None, None) => None,
        };
        let permitted_alphabet = match (self.permitted_alphabet, rhs.permitted_alphabet) {
            (Some(perm), Some(rhs_perm)) => Some(perm.intersect(&rhs_perm)),
            (Some(perm), None) => Some(perm),
            (None, Some(rhs_perm)) => Some(rhs_perm),
            (None, None) => None,
        };
        let extensible = self.extensible || rhs.extensible;

        Self {
            value,
            size,
            permitted_alphabet,
            extensible,
        }
    }

    /// Returns the size constraint from the set, if available.
    pub const fn size(&self) -> Option<&Extensible<Size>> {
        self.size.as_ref()
    }

    /// Returns the permitted alphabet constraint from the set, if available.
    pub const fn permitted_alphabet(&self) -> Option<&Extensible<PermittedAlphabet>> {
        self.permitted_alphabet.as_ref()
    }

    /// Returns whether any of the constraints are extensible.
    pub const fn extensible(&self) -> bool {
        if self.extensible {
            return true;
        }
        if let Some(value) = &self.value {
            if value.extensible.is_some() {
                return true;
            }
        }
        if let Some(size) = &self.size {
            if size.extensible.is_some() {
                return true;
            }
        }
        if let Some(permitted_alphabet) = &self.permitted_alphabet {
            if permitted_alphabet.extensible.is_some() {
                return true;
            }
        }
        false
    }

    /// Returns the value constraint from the set, if available.
    pub const fn value(&self) -> Option<&Extensible<Value>> {
        self.value.as_ref()
    }
}

/// The set of possible constraints a given value can have.
///
/// Do not change the amount of variants in this enum without changing the `SUPPORTED_CONSTRAINTS_COUNT` constant.
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
    /// Empty constraint.
    Empty,
}

/// The discriminant of [Constraint] values.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum ConstraintDiscriminant {
    Value,
    Size,
    PermittedAlphabet,
    Extensible,
    Empty,
}
impl ConstraintDiscriminant {
    /// Constant equality check.
    pub const fn eq(&self, other: &ConstraintDiscriminant) -> bool {
        *self as isize == *other as isize
    }
}

impl Constraint {
    /// Returns the discriminant of the value.
    pub const fn kind(&self) -> ConstraintDiscriminant {
        match self {
            Self::Value(_) => ConstraintDiscriminant::Value,
            Self::Size(_) => ConstraintDiscriminant::Size,
            Self::PermittedAlphabet(_) => ConstraintDiscriminant::PermittedAlphabet,
            Self::Extensible => ConstraintDiscriminant::Extensible,
            Self::Empty => ConstraintDiscriminant::Empty,
        }
    }
    /// Returns the default constraint, which is an empty constraint.
    pub const fn default() -> Self {
        Self::Empty
    }
    /// Returns the discriminant as an `isize` integer.
    pub const fn variant_as_isize(&self) -> isize {
        match self {
            Self::Value(_) => 0,
            Self::Size(_) => 1,
            Self::PermittedAlphabet(_) => 2,
            Self::Extensible => 3,
            Self::Empty => 4,
        }
    }
    // pub const fn intersect(&self, other: &Self) -> Self {
    //     // TODO implement intersection of extensible constraints
    //     match (self, other) {
    //         (Self::Value(a), Self::Value(b)) => {
    //             let is_extensible = a.extensible.is_some() || b.extensible.is_some();
    //             let a = a.constraint.intersect(&b.constraint);
    //             Self::Value(Extensible::new(a).set_extensible(is_extensible))
    //         }
    //         (Self::Size(a), Self::Size(b)) => {
    //             let is_extensible = a.extensible.is_some() || b.extensible.is_some();
    //             let a = a.constraint.intersect(&b.constraint);
    //             Self::Size(Extensible::new(a).set_extensible(is_extensible))
    //         }
    //         (Self::PermittedAlphabet(a), Self::PermittedAlphabet(b)) => {
    //             let is_extensible = a.extensible.is_some() || b.extensible.is_some();
    //             let a = a.constraint.intersect(&b.constraint);
    //             Self::PermittedAlphabet(Extensible::new(a).set_extensible(is_extensible))
    //         }
    //         (Self::Extensible, Self::Extensible) => Self::Extensible,
    //         (Self::Empty, _) | (_, Self::Empty) => Self::Empty,
    //         _ => Self::Extensible,
    //     }
    // }

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
            Self::Empty => false,
        }
    }
}

/// A wrapper around [Constraint] covering whether the constraint is "extensible".
///
/// Extensible means that it can have values outside of its constraints, and what possible
/// constraints thosevalues in the extended set can have, if any.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Extensible<T: 'static> {
    /// The underlying constraint type.
    pub constraint: T,
    /// Whether the constraint is extensible, and if it is, a list of extensible
    /// constraints.
    /// Extensibility means that the allowed constraint values can change, not type of the constraint itself.
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

impl Extensible<Value> {
    /// Intersects two extensible value constraints.
    pub const fn intersect(&self, other: &Self) -> Self {
        let extensible = match (self.extensible, other.extensible) {
            (Some(a), Some(_)) => Some(a),
            (Some(a), None) => None,
            (None, Some(b)) => None,
            (None, None) => None,
        };
        let constraint = self.constraint.intersect(&other.constraint);
        match extensible {
            Some(ext_ref) => Self::new_extensible(constraint, ext_ref),
            None => Self::new(constraint),
        }
    }
}
impl Extensible<Size> {
    /// Intersects two extensible size constraints.
    pub const fn intersect(&self, other: &Self) -> Self {
        let extensible = match (self.extensible, other.extensible) {
            (Some(a), Some(_)) => Some(a),
            (Some(_), None) => None,
            (None, Some(_)) => None,
            (None, None) => None,
        };
        let constraint = self.constraint.intersect(&other.constraint);
        match extensible {
            Some(ext_ref) => Self::new_extensible(constraint, ext_ref),
            None => Self::new(constraint),
        }
    }
}
impl Extensible<PermittedAlphabet> {
    /// Intersects two extensible permitted alphabet constraints.
    pub const fn intersect(&self, other: &Self) -> Self {
        // Extensiblity status might not be correct
        let extensible = match (self.extensible, other.extensible) {
            (Some(a), Some(_b)) => Some(a),
            (Some(_), None) => None,
            (None, Some(_)) => None,
            (None, None) => None,
        };
        let constraint = self.constraint.intersect(&other.constraint);
        match extensible {
            Some(ext_ref) => Self::new_extensible(constraint, ext_ref),
            None => Self::new(constraint),
        }
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
pub struct Value {
    /// Bound of the value
    pub(crate) value: Bounded<i128>,
    /// Sign of the bound, used for numeric values
    pub(crate) signed: bool,
    /// Range of the bound in bytes, used for numeric values
    pub(crate) range: Option<u8>,
}

impl Value {
    /// Creates a new value constraint from a given bound.
    pub const fn new(value: Bounded<i128>) -> Self {
        let (signed, range) = value.range_in_bytes();
        Self {
            value,
            signed,
            range,
        }
    }
    /// Gets the sign of the value constraint.
    pub const fn get_sign(&self) -> bool {
        self.signed
    }
    /// Gets the range of the value constraint.
    pub const fn get_range(&self) -> Option<u8> {
        self.range
    }
    /// Intersect between two `Value` constraints
    pub const fn intersect(&self, other: &Self) -> Self {
        let value = self.value.intersect(other.value);
        let (signed, range) = value.range_in_bytes();
        Self {
            value,
            signed,
            range,
        }
    }
}

impl core::ops::Deref for Value {
    type Target = Bounded<i128>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

macro_rules! from_primitives {
    ($($int:ty),+ $(,)?) => {
        $(
            impl From<Bounded<$int>> for Value {
                fn from(bounded: Bounded<$int>) -> Self {
                    Self::new(match bounded {
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
        Ok(Self::new(match bounded {
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
    /// Creates a varying range constraint.
    #[must_use]
    pub const fn new(range: Bounded<usize>) -> Self {
        Self(range)
    }

    /// Creates a fixed size constraint.
    #[must_use]
    pub const fn fixed(length: usize) -> Self {
        Self(Bounded::Single(length))
    }

    /// Returns whether the size is fixed.
    #[must_use]
    pub const fn is_fixed(&self) -> bool {
        matches!(self.0, Bounded::Single(_))
    }
    /// Returns whether the size has a varying range.
    #[must_use]
    pub const fn is_range(&self) -> bool {
        matches!(self.0, Bounded::Range { .. })
    }
    #[must_use]
    pub const fn intersect(&self, other: &Self) -> Self {
        Self(self.0.intersect(other.0))
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// Intersect between two `PermittedAlphabet` constraints.
    ///
    /// TODO not currently possible to intersect
    /// because new instance requires a static lifetime,
    /// so we just override for now.
    pub const fn intersect(&self, other: &Self) -> Self {
        Self(other.0)
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
    /// Calculates the the amount of bytes that are required to represent integer `value`.
    /// Particularly useful for OER codec
    const fn octet_size_by_range(value: i128) -> Option<u8> {
        let abs_value = value.unsigned_abs();
        Some(if abs_value <= u8::MAX as u128 {
            1
        } else if abs_value <= u16::MAX as u128 {
            2
        } else if abs_value <= u32::MAX as u128 {
            4
        } else if abs_value <= u64::MAX as u128 {
            8
        } else {
            return None;
        })
    }
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

impl Bounded<i128> {
    /// Intersect the values of two bounded ranges.
    pub const fn intersect(&self, other: Self) -> Self {
        match (self, other) {
            (Self::None, _) | (_, Self::None) => Self::None,
            (Self::Single(a), Self::Single(b)) if *a == b => Self::Single(*a),
            (
                Self::Single(a),
                Self::Range {
                    start: Some(b),
                    end: Some(c),
                },
            ) if *a >= b && *a <= c => Self::Single(*a),
            (
                Self::Range {
                    start: Some(b),
                    end: Some(c),
                },
                Self::Single(a),
            ) if a >= *b && a <= *c => Self::Single(a),
            (
                Self::Range {
                    start: Some(a),
                    end: Some(b),
                },
                Self::Range {
                    start: Some(c),
                    end: Some(d),
                },
            ) if *a <= d && *b >= c => Self::Range {
                // usize is unsigned, so we can't have negative values
                // usize always < i128::MAX
                start: Some(max_signed(*a, c)),
                end: Some(min(*b, d)),
            },
            _ => Self::None,
        }
    }
}

impl Bounded<usize> {
    /// Intersect the values of two bounded ranges.
    pub const fn intersect(&self, other: Self) -> Self {
        match (self, other) {
            (Self::None, _) | (_, Self::None) => Self::None,
            (Self::Single(a), Self::Single(b)) if *a == b => Self::Single(*a),
            (
                Self::Single(a),
                Self::Range {
                    start: Some(b),
                    end: Some(c),
                },
            ) if *a >= b && *a <= c => Self::Single(*a),
            (
                Self::Range {
                    start: Some(b),
                    end: Some(c),
                },
                Self::Single(a),
            ) if a >= *b && a <= *c => Self::Single(a),
            (
                Self::Range {
                    start: Some(a),
                    end: Some(b),
                },
                Self::Range {
                    start: Some(c),
                    end: Some(d),
                },
            ) if *a <= d && *b >= c => Self::Range {
                // usize is unsigned, so we can't have negative values
                // usize always < i128::MAX
                start: Some(max_unsigned(*a as u128, c as u128) as usize),
                end: Some(min(*b as i128, d as i128) as usize),
            },
            _ => Self::None,
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
const fn max_signed(a: i128, b: i128) -> i128 {
    [a, b][(a < b) as usize]
}

const fn max_unsigned(a: u128, b: u128) -> u128 {
    [a, b][(a < b) as usize]
}
const fn min(a: i128, b: i128) -> i128 {
    [a, b][(a > b) as usize]
}

impl Bounded<i128> {
    /// Returns the sign and the range in bytes of the constraint.
    pub const fn range_in_bytes(&self) -> (bool, Option<u8>) {
        match self {
            Self::Single(value) => (*value < 0, Self::octet_size_by_range(*value)),
            Self::Range {
                start: Some(start),
                end: Some(end),
            } => {
                let is_signed = *start < 0;
                let end_abs = end.unsigned_abs();
                let start_abs = start.unsigned_abs();
                let bound_max = max_unsigned(start_abs, end_abs);
                let octets = Self::octet_size_by_range(bound_max as i128);
                (is_signed, octets)
            }
            Self::Range {
                start: Some(start),
                end: None,
            } => (*start < 0, None),
            Self::Range { start: None, .. } | Self::None => (true, None),
        }
    }
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
    use crate as rasn;
    use crate::prelude::*;

    #[test]
    fn range() {
        let constraints = Bounded::new(0, 255);
        assert_eq!(256, constraints.range().unwrap());
    }
    #[test]
    fn test_merge_constraints() {
        #[derive(AsnType, Decode, Debug, PartialEq)]
        #[rasn(delegate, from("a..=z", "A..=Z", "-", "."), size("1..=64"))]
        pub struct NameString(pub VisibleString);

        #[derive(AsnType, Decode, Debug, PartialEq)]
        #[rasn(tag(application, 1))]
        pub struct InitialString {
            #[rasn(size(1))]
            pub initial: NameString,
        }
        const INNER: Constraint = rasn::types::Constraint::Size(
            rasn::types::constraints::Extensible::new(rasn::types::constraints::Size::new(
                rasn::types::constraints::Bounded::single_value(1i128 as usize),
            ))
            .set_extensible(false),
        );
        const LESS_INNER: Constraints = rasn::types::Constraints::new(&[INNER]);
        const MERGED: Constraints =
            <NameString as rasn::AsnType>::CONSTRAINTS.intersect(Constraints::new(&[
                rasn::types::Constraint::Size(
                    rasn::types::constraints::Extensible::new(rasn::types::constraints::Size::new(
                        rasn::types::constraints::Bounded::single_value(1i128 as usize),
                    ))
                    .set_extensible(false),
                ),
            ]));
        const FIELD_CONSTRAINT_0: rasn::types::Constraints =
            // rasn::types::Constraints::from_fixed_size(
            <NameString as rasn::AsnType>::CONSTRAINTS.intersect(Constraints::new(&[
                    rasn::types::Constraint::Size(
                        rasn::types::constraints::Extensible::new(
                            rasn::types::constraints::Size::new(
                                rasn::types::constraints::Bounded::single_value(1i128 as usize),
                            ),
                        )
                        .set_extensible(false),
                    ),
                ]));
        // );
        dbg!(FIELD_CONSTRAINT_0);
    }
    // #[test]
    // fn test_nested_extensible() {
    //     #[derive(AsnType, Encode, Decode, Debug, PartialEq)]
    //     #[rasn(tag(application, 2), delegate, value("0..=9999", extensible))]
    //     pub struct ExtensibleEmployeeNumber(pub Integer);

    //     #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    //     #[rasn(
    //         tag(application, 3),
    //         delegate,
    //         from("0..=9"),
    //         size(8, extensible, "9..=20")
    //     )]
    //     pub struct ExtensibleDate(pub VisibleString);

    //     #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    //     #[rasn(delegate, from("a..=z", "A..=Z", "-", "."), size("1..=64", extensible))]
    //     pub struct ExtensibleNameString(pub VisibleString);

    //     #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    //     #[rasn(tag(application, 1))]
    //     #[non_exhaustive]
    //     pub struct ExtensibleName {
    //         pub given_name: ExtensibleNameString,
    //         #[rasn(size(1))]
    //         pub initial: ExtensibleNameString,
    //         pub family_name: ExtensibleNameString,
    //     }
    //     #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    //     #[rasn(set)]
    //     #[non_exhaustive]
    //     pub struct ExtensibleChildInformation {
    //         name: ExtensibleName,
    //         #[rasn(tag(explicit(0)))]
    //         date_of_birth: ExtensibleDate,
    //     }

    //     #[derive(AsnType, Encode, Decode, Debug, PartialEq)]
    //     #[rasn(set, tag(application, 0))]
    //     #[non_exhaustive]
    //     pub struct ExtensiblePersonnelRecord {
    //         pub name: ExtensibleName,
    //         #[rasn(tag(explicit(0)))]
    //         pub title: VisibleString,
    //         pub number: ExtensibleEmployeeNumber,
    //         #[rasn(tag(explicit(1)))]
    //         pub date_of_hire: ExtensibleDate,
    //         #[rasn(tag(explicit(2)))]
    //         pub name_of_spouse: ExtensibleName,
    //         #[rasn(tag(3), default, size(2, extensible))]
    //         pub children: Option<Vec<ExtensibleChildInformation>>,
    //     }
    // #[derive(AsnType, Decode, Encode, Debug, PartialEq)]
    // #[rasn(set)]
    // #[non_exhaustive]
    // pub struct ExtensibleTest {
    //     name: ExtensibleName,
    //     #[rasn(tag(1), size(1, extensible))]
    //     record: ExtensiblePersonnelRecord,
    // }
}
