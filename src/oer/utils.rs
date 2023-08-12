use crate::types::constraints::{Bounded, Extensible, Value};

/// ITU-T X.696 (02/2021) 10.0
///
/// Number of octets by value ranges for unsigned integers
/// 1, 2, 4 or 8 octets
const UNSIGNED_RANGES: [(i128, i128, u8); 4] = [
    (0i128, u8::MAX as i128, 1),
    (u8::MAX as i128, u16::MAX as i128, 2),
    (u16::MAX as i128, u32::MAX as i128, 4),
    (u32::MAX as i128, u64::MAX as i128, 8),
];
/// Number of octets by value ranges for signed integers
/// 1, 2, 4 or 8 octets
const SIGNED_RANGES: [(i128, i128, u8); 4] = [
    (i8::MIN as i128, i8::MAX as i128, 1),
    (i16::MIN as i128, i16::MAX as i128, 2),
    (i32::MIN as i128, i32::MAX as i128, 4),
    (i64::MIN as i128, i64::MAX as i128, 8),
];
// Constraints limit Bound to i128 in Value type (see Value struct)
// Only Value constraint is OER visible (range, single value)
// TODO - maybe use BigInt instead of i128 some day?
pub fn determine_integer_size_and_sign<T, U, E>(
    value_constraint: &Extensible<Value>,
    data: U,
    // transform_fn takes data, integers' signed status, and octet number required to contain integer
    // based on the constraints
    mut transform_fn: impl FnMut(U, bool, Option<u8>) -> Result<T, E>,
) -> Result<T, E> {
    // Ignore constraints if extension marker is present
    if value_constraint.extensible.is_some() {
        return transform_fn(data, true, None);
    }
    match value_constraint.constraint.0 {
        Bounded::Range {
            start: Some(start),
            end: Some(end),
        } => {
            if start >= 0 {
                for (min, max, octets) in UNSIGNED_RANGES {
                    if min <= start && end <= max {
                        return transform_fn(data, false, Some(octets));
                    }
                }
                // Upper bound is out of range, use length determinant
                return transform_fn(data, false, None);
            }
            for (min, max, octets) in SIGNED_RANGES {
                if min <= start && end <= max {
                    return transform_fn(data, true, Some(octets));
                }
            }
            // Negative lower bound, and out of range, use length determinant and signed integers
            transform_fn(data, true, None)
        }
        Bounded::Range {
            start: Some(start),
            end: None,
        } => transform_fn(data, start < 0, None),
        Bounded::Range {
            start: None,
            end: Some(_) | None,
        }
        | Bounded::None => transform_fn(data, true, None),
        // TODO - check if this is correct way instead of not encoding at all, or true/false
        Bounded::Single(value) => transform_fn(data, value < 0, None),
    }
}
