//! Constraints of values on a given type.

use super::IntegerType;
use num_bigint::BigInt;

/// A marker trait with validation methods for types that have ASN.1 inner subtype constraints.
pub trait InnerSubtypeConstraint: Sized {
    /// Validates the inner subtype constraints and returns the type on success.
    /// Does not attempt to decode internal ASN.1 `CONTAINING` constraints as a marked type. See `validate_and_decode_containing` for CONTAINING validation.
    fn validate_components(self) -> Result<Self, crate::error::InnerSubtypeConstraintError> {
        self.validate_and_decode_containing(None)
    }

    /// Validates the inner subtype constraints and attempts to decode internal ASN.1 `CONTAINING` constraints as a marked type.
    /// Usually this means that some field has `OPAQUE` data, and we need to decode it further as a specific type, as defined in the inner subtype constraint.
    /// Manual implementation of this function is required for all types that have inner subtype constraints.
    /// # Arguments
    /// * `decode_containing_with` - the codec to validate and decode the containing data with - ASN.1 type definiton has `CONTAINING.
    fn validate_and_decode_containing(
        self,
        decode_containing_with: Option<crate::Codec>,
    ) -> Result<Self, crate::error::InnerSubtypeConstraintError>;
}

/// A set of constraints for a given type on what kinds of values are allowed.
/// Used in certain codecs to optimise encoding and decoding values.
///
/// The effective constraint is typically the intersection or union among other constraints.
/// As a result, we can store one constraint of each kind, updated with the latest constraint.
///
/// TODO architecture needs a re-design - multiple constraints with same type are allowed forming on non-contiguous set of values.
/// We can't currently present this.
/// For example, value constraint can have multiple single values, or a range of values which do not overlap, and are effective at the same time.
/// This is challenging to implement in compile-time, and we may need to use runtime checks.
/// E.g effective constraint can have up to infinite single value constraints, and overall constraint value is not continuous.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Constraints {
    value: Option<Extensible<Value>>,
    size: Option<Extensible<Size>>,
    permitted_alphabet: Option<Extensible<PermittedAlphabet>>,
    extensible: bool,
}

impl Constraints {
    /// Empty constraints.
    pub const NONE: Self = Self {
        value: None,
        size: None,
        permitted_alphabet: None,
        extensible: false,
    };

    /// Creates a new set of constraints from a given slice.
    pub const fn new(constraints: &[Constraint]) -> Self {
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

    /// Creates an intersection of two constraint sets.
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

    /// Returns the effective size constraint, if available.
    pub const fn size(&self) -> Option<&Extensible<Size>> {
        self.size.as_ref()
    }

    /// Returns the effective permitted alphabet constraint, if available.
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
        }
    }
    /// Returns the discriminant as an `isize` integer.
    pub const fn variant_as_isize(&self) -> isize {
        match self {
            Self::Value(_) => 0,
            Self::Size(_) => 1,
            Self::PermittedAlphabet(_) => 2,
            Self::Extensible => 3,
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

macro_rules! impl_extensible {
    ($($type:ty),+) => {
        $(
            impl Extensible<$type> {
                /// Intersects two extensible constraints.
                pub const fn intersect(&self, other: &Self) -> Self {
                    // All lost in our use case? https://stackoverflow.com/questions/33524834/whats-the-result-set-operation-of-extensible-constraints-in-asn-1
                    // ASN.1 treats serially applied constraints differently from nested extensible constraints?
                    // We currently support only serially applied constraints.
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
            impl From<$type> for Extensible<$type> {
                fn from(value: $type) -> Self {
                    Self {
                        constraint: value,
                        extensible: None,
                    }
                }
            }
        )+
    };
}
impl_extensible!(Value, Size, PermittedAlphabet);

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
    /// Intersect between two `Size` constraints
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
        match value.unsigned_abs() {
            x if x <= u8::MAX as u128 => Some(1),
            x if x <= u16::MAX as u128 => Some(2),
            x if x <= u32::MAX as u128 => Some(4),
            x if x <= u64::MAX as u128 => Some(8),
            _ => None,
        }
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

macro_rules! impl_bounded_range {
    ($($type:ty),+) => {
        $(
            impl Bounded<$type> {
                /// Returns the number representing the difference between the lower and upper bound.
                pub const fn range(&self) -> Option<$type> {
                    match self {
                        Self::Single(_) => Some(1),
                        Self::Range {
                            start: Some(start),
                            end: Some(end),
                        } => Some(end.wrapping_sub(*start).saturating_add(1)),
                        _ => None,
                    }
                }
                /// Returns the effective value which is either the number, or the positive
                /// offset of that number from the start of the value range. `Either::Left`
                /// represents the positive offset, and `Either::Right` represents
                /// the number.
                pub const fn effective_value(&self, value: $type) -> either::Either<$type, $type> {
                    match self {
                        Self::Range {
                            start: Some(start), ..
                        } => {
                            debug_assert!(value >= *start);
                            either::Left(value - *start)
                        }
                        _ => either::Right(value),
                    }
                }
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
                            start: Some(max(*a as i128, c as i128) as $type),
                            end: Some(min(*b as i128, d as i128) as $type),
                        },
                        _ => Self::None,
                    }
                }
            }
        )+
    };
}
impl_bounded_range!(i128, usize);

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
const fn max(a: i128, b: i128) -> i128 {
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
    #[inline(always)]
    pub fn in_bound<I: IntegerType>(&self, element: &I) -> bool {
        match &self {
            Self::Range { start, end } => {
                start.as_ref().is_none_or(|&start| {
                    if let Some(e) = element.to_i128() {
                        e >= start
                    } else if let Some(e) = element.to_bigint() {
                        e >= BigInt::from(start)
                    } else {
                        false
                    }
                }) && end.as_ref().is_none_or(|&end| {
                    if let Some(e) = element.to_i128() {
                        e <= end
                    } else if let Some(e) = element.to_bigint() {
                        e <= BigInt::from(end)
                    } else {
                        false
                    }
                })
            }
            Self::Single(value) => {
                if let Some(e) = element.to_i128() {
                    e == *value
                } else {
                    false
                }
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
                start.as_ref().is_none_or(|start| element >= start)
                    && end.as_ref().is_none_or(|end| element <= end)
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
        let constraints = Bounded::new(0, 255usize);
        assert_eq!(256, constraints.range().unwrap());
    }
}
