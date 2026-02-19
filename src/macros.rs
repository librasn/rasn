//! Macros to automate common operations in `rasn` such as constructing
//! constraints.

pub use crate::{
    Decode, Encode, constraints, permitted_alphabet_constraint, size_constraint, value_constraint,
};

#[macro_use]
#[cfg(test)]
pub mod test;

/// Helper macro to create constant value constraints.
///
/// Usage:
/// ```rust
/// use rasn::{types::Constraint, macros::*};
/// // Full range
/// const FULL_RANGE: Constraint = value_constraint!(0, 100);
/// const FULL_RANGE_EXTEND: Constraint = value_constraint!(0, 100, extensible);
/// const START_ONLY: Constraint = value_constraint!(start: 42);
/// const START_ONLY_EXTENDED: Constraint = value_constraint!(start: 42, extensible);
/// const END_ONLY: Constraint = value_constraint!(end: 42);
/// const SINGLE: Constraint = value_constraint!(42);
/// const EXT_SINGLE: Constraint = value_constraint!(42, extensible);
/// ```
#[macro_export]
macro_rules! value_constraint {
    ($($args:tt)*) => {
        $crate::bounded_constraint!(Value, $($args)*)
    };
}

/// Helper macro to create constant size constraints.
///
/// Usage:
/// ```rust
/// use rasn::{types::Constraint, macros::*};
/// // Full range
/// const RANGE: Constraint = size_constraint!(0, 100);
/// const RANGE_EXTEND: Constraint = size_constraint!(0, 100, extensible);
/// const START_ONLY: Constraint = size_constraint!(start: 42);
/// const START_ONLY_EXTENDED: Constraint = size_constraint!(start: 42, extensible);
/// const END_ONLY: Constraint = size_constraint!(end: 42);
/// const FIXED: Constraint = size_constraint!(42);
/// const EXT_FIXED: Constraint = size_constraint!(42, extensible);
/// ```
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
    // Single value with extensibility
    ($constraint_type:ident, $single:expr, extensible) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::Single($single),
                ),
            )
            .set_extensible(true),
        )
    };

    // Range with extensibility
    ($constraint_type:ident, $start:expr, $end:expr, extensible) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::const_new($start, $end),
                ),
            )
            .set_extensible(true),
        )
    };

    // Only start with extensibility
    ($constraint_type:ident, start: $start:expr, extensible) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::start_from($start),
                ),
            )
            .set_extensible(true),
        )
    };

    // Only end with extensibility
    ($constraint_type:ident, end: $end:expr, extensible) => {
        $crate::types::constraints::Constraint::$constraint_type(
            $crate::types::constraints::Extensible::new(
                $crate::types::constraints::$constraint_type::new(
                    $crate::types::constraints::Bounded::up_to($end),
                ),
            )
            .set_extensible(true),
        )
    };

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
}

/// Helper macro to create a `&'static Oid` from a string literal. The string
/// literal must conform to the standard OID format and must be a valid OID
/// (first arc must be <= 2), but also accepts OIDs with and without a leading
/// `.`.
///
/// Usage:
/// ```rust
/// const SYS_DESCR: &'static rasn::types::Oid = rasn::oid!(".1.3.6.1.2.1.1.1.0");
/// assert_eq!(SYS_DESCR, rasn::types::Oid::new(&[1, 3, 6, 1, 2, 1, 1, 1, 0]).unwrap());
/// ```
#[macro_export]
macro_rules! oid {
    ($s:literal) => {{
        const OID: &'static $crate::types::Oid = const {
            const BYTE_STRING: &'static [u8] = const {
                let s: &'static str = $s;
                let bytes = s.as_bytes();

                core::assert!(!bytes.is_empty(), "OID string literals cannot be empty");
                core::assert!(bytes.is_ascii(), "OID string literals must be ASCII");
                core::assert!(
                    bytes[bytes.len() - 1] != b'.',
                    "OID string literals cannot end with a period"
                );

                match bytes {
                    [b'.', rest @ ..] => rest,
                    bytes => bytes,
                }
            };

            const COMPONENT_LEN: usize = const {
                let mut component_count = 1;
                let mut index = 0;
                while index < BYTE_STRING.len() {
                    let byte = BYTE_STRING[index];

                    core::assert!(
                        core::matches!(byte, b'0'..=b'9' | b'.'),
                        "OID string literals can only contain ASCII digits and periods"
                    );

                    if byte == b'.' {
                        if index + 1 < BYTE_STRING.len() {
                            core::assert!(
                                BYTE_STRING[index + 1] != b'.',
                                "OID string literals cannot contain two or more consecutive periods"
                            );
                        }
                        component_count += 1;
                    }

                    index += 1;
                }

                component_count
            };

            const COMPONENTS: [u32; COMPONENT_LEN] = const {
                let mut bytes_index = 0;

                let mut components = [0u32; COMPONENT_LEN];
                let mut index = 0;
                while bytes_index < BYTE_STRING.len() {
                    let byte = BYTE_STRING[bytes_index];
                    match byte {
                        b'0'..=b'9' => {
                            components[index] = components[index] * 10 + ((byte - b'0') as u32)
                        }
                        b'.' => index += 1,
                        _ => core::unreachable!(),
                    }

                    bytes_index += 1;
                }

                core::assert!(components[0] <= 2, "the first OID arc must be <= 2");

                components
            };

            $crate::types::Oid::new(&COMPONENTS).unwrap()
        };

        OID
    }};
}
