use crate::types::constraints::{Bounded, Extensible, Value};

/// ITU-T X.696 (02/2021) 10.0
///
/// Number of octets by value ranges for unsigned  and signed integers
/// 1, 2, 4 or 8 octets
pub fn octet_size_by_range(value: i128) -> Option<u8> {
    let range = if value >= 0 {
        // For unsigned values
        Some(((128 - value.leading_zeros() + 7) / 8) as u8)
    } else {
        // For signed values
        Some(((128 - (value + 1).leading_zeros() + 7) / 8) as u8)
    };
    match range {
        Some(1) => Some(1),
        Some(2) => Some(2),
        Some(3..5) => Some(4),
        Some(5..=8) => Some(8),
        _ => None,
    }
}

// // Constraints limit Bound to i128 in Value type (see Value struct)
// // Only Value constraint is OER visible (range, single value)
// // TODO - maybe use BigInt instead of i128 some day?
// pub fn determine_integer_size_and_sign<T, U, E>(
//     value_constraint: &Extensible<Value>,
//     data: U,
//     // transform_fn takes data, integers' signed status, and octet number required to contain integer
//     // based on the constraints
//     mut transform_fn: impl FnMut(U, bool, Option<u8>) -> Result<T, E>,
// ) -> Result<T, E> {
//     // Ignore constraints if extension marker is present
//     if value_constraint.extensible.is_some() {
//         return transform_fn(data, true, None);
//     }
//     match value_constraint.constraint.0 {
//         Bounded::Range {
//             start: Some(start),
//             end: Some(end),
//         } => {
//             if start >= 0 {
//                 for (min, max, octets) in UNSIGNED_RANGES {
//                     if min <= start && end <= max {
//                         return transform_fn(data, false, Some(octets));
//                     }
//                 }
//                 // Upper bound is out of range, use length determinant
//                 return transform_fn(data, false, None);
//             }
//             for (min, max, octets) in SIGNED_RANGES {
//                 if min <= start && end <= max {
//                     return transform_fn(data, true, Some(octets));
//                 }
//             }
//             // Negative lower bound, and out of range, use length determinant and signed integers
//             transform_fn(data, true, None)
//         }
//         Bounded::Range {
//             start: Some(start),
//             end: None,
//         } => transform_fn(data, start < 0, None),
//         Bounded::Range {
//             start: None,
//             end: Some(_) | None,
//         }
//         | Bounded::None => transform_fn(data, true, None),
//         // TODO - check if this is correct way instead of not encoding at all, or true/false
//         Bounded::Single(value) => transform_fn(data, value < 0, octet_size_by_range(value)),
//     }
// }

pub fn determine_integer_size_and_sign<T, U, E>(
    value_constraint: &Extensible<Value>,
    data: U,
    mut transform_fn: impl FnMut(U, bool, Option<u8>) -> Result<T, E>,
) -> Result<T, E> {
    if value_constraint.extensible.is_some() {
        return transform_fn(data, true, None);
    }

    match &value_constraint.constraint.0 {
        Bounded::Range { start, end } => match (start, end) {
            (Some(start), Some(end)) => {
                let is_signed = *start < 0;
                let octets = if is_signed {
                    get_signed_octets(*start, *end)
                } else {
                    get_unsigned_octets(*start, *end)
                };
                transform_fn(data, is_signed, octets)
            }
            (Some(start), None) => transform_fn(data, *start < 0, None),
            _ => transform_fn(data, true, None),
        },
        Bounded::Single(value) => transform_fn(data, *value < 0, octet_size_by_range(*value)),
        Bounded::None => transform_fn(data, true, None),
    }
}

fn get_unsigned_octets(start: i128, end: i128) -> Option<u8> {
    let max_value = end.max(start);
    octet_size_by_range(max_value)
}

fn get_signed_octets(start: i128, end: i128) -> Option<u8> {
    let min_value = start.min(end);
    let max_value = start.max(end);
    octet_size_by_range(min_value.abs().max(max_value.abs()))
}
