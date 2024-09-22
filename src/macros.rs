//! Macros to automate common operations in `rasn` such as constructing
//! constraints.

pub use crate::{
    constraints, permitted_alphabet_constraint, size_constraint, value_constraint, Decode, Encode,
};

#[macro_use]
#[cfg(test)]
pub mod test;

/// Helper macro to create constant value constraints.
///
/// Usage:
// ```rust
/// use rasn::{types::Constraint, macros::*};
/// // Full range
/// const FULL_RANGE: Constraint = value_constraint!(0, 100);
/// const FULL_RANGE_EXTEND: Constraint = value_constraint!(0, 100, true);
/// const START_ONLY: Constraint = value_constraint!(start: 42);
/// const START_ONLY_EXTENDED: Constraint = value_constraint!(start: 42, true);
/// const END_ONLY: Constraint = value_constraint!(end: 42);
/// const SINGLE: Constraint = value_constraint!(42);
/// const EXT_SINGLE: Constraint = value_constraint!(42, true);
///```
#[macro_export]
macro_rules! value_constraint {
    ($($args:tt)*) => {
        $crate::bounded_constraint!(Value, $($args)*)
    };
}

/// Helper macro to create constant size constraints.
///
/// Usage:
// ```rust
/// use rasn::{types::Constraint, macros::*};
/// // Full range
/// const RANGE: Constraint = size_constraint!(0, 100);
/// const RANGE_EXTEND: Constraint = size_constraint!(0, 100, true);
/// const START_ONLY: Constraint = size_constraint!(start: 42);
/// const START_ONLY_EXTENDED: Constraint = size_constraint!(start: 42, true);
/// const END_ONLY: Constraint = size_constraint!(end: 42);
/// const FIXED: Constraint = size_constraint!(42);
/// const EXT_FIXED: Constraint = size_constraint!(42, true);
///```
#[macro_export]
macro_rules! size_constraint {
    ($($args:tt)*) => {
        $crate::bounded_constraint!(Size, $($args)*)
    };
}

/// Helper macro to create an array of constant constraints.
///
/// Usage:
/// ```rust
/// use rasn::{types::Constraints, macros::*};
/// const CONSTRAINTS: Constraints = constraints!(value_constraint!(0, 100), size_constraint!(0, 100));
/// ```
/// See other macros about the parameter usage.
#[macro_export]
macro_rules! constraints {
    ($($constraint:expr),+ $(,)?) => {
        $crate::types::constraints::Constraints::new(&[$($constraint),+])
    };
}
/// Helper macro to create a permitted alphabet constraint.
///
/// Usage:
/// ```rust
/// use rasn::prelude::*;
/// use rasn::macros::*;
/// const CONSTRAINT: Constraints = constraints!(permitted_alphabet_constraint!(&[
///     b'0' as u32,
///     b'1' as u32,
///     b'2' as u32,
///     b'3' as u32,
///     b'4' as u32,
///     b'5' as u32
/// ]));
/// ```
#[macro_export]
macro_rules! permitted_alphabet_constraint {
    ( $alphabet:expr) => {
        $crate::types::constraints::Constraint::PermittedAlphabet(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::PermittedAlphabet::new($alphabet),
            ),
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bounded_constraint {
    // Start and end
    ($constraint_type:ident, $start:expr, $end:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::const_new($start, $end),
                ),
            ),
        )
    };

    // Only start provided
    ($constraint_type:ident, start: $start:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::start_from($start),
                ),
            ),
        )
    };

    // Only end provided
    ($constraint_type:ident, end: $end:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::up_to($end),
                ),
            ),
        )
    };

    // Single value
    ($constraint_type:ident, $single:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::Single($single),
                ),
            ),
        )
    };

    // Single value with extensibility
    ($constraint_type:ident, $single:expr, $extensible:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::Single($single),
                ),
            )
            .set_extensible($extensible),
        )
    };

    // Range with extensibility
    ($constraint_type:ident, $start:expr, $end:expr, $extensible:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::const_new($start, $end),
                ),
            )
            .set_extensible($extensible),
        )
    };

    // Only start with extensibility
    ($constraint_type:ident, start: $start:expr, $extensible:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::start_from($start),
                ),
            )
            .set_extensible($extensible),
        )
    };

    // Only end with extensibility
    ($constraint_type:ident, end: $end:expr, $extensible:expr) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::up_to($end),
                ),
            )
            .set_extensible($extensible),
        )
    };
}
