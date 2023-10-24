pub mod de;
pub mod enc;

use crate::types::Integer;
use serde::{Deserialize, Serialize};

/// Attempts to decode `T` from `input` using JER.
/// # Errors
/// Returns error specific to JER decoder if decoding is not possible.
pub fn decode<'de, T: serde::de::Deserialize<'de>>(input: &'de str) -> Result<T, de::error::Error> {
    Ok(serde_json::from_str(input)?)
}

/// Attempts to encode `value` to JER.
/// # Errors
/// Returns error specific to JER encoder if encoding is not possible.
pub fn encode<T: serde::ser::Serialize>(
    value: &T,
) -> Result<alloc::string::String, enc::error::Error> {
    Ok(serde_json::to_string(value)?)
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Integer")]
pub struct JerInteger(#[serde(getter = "get_integer_value")] i128);

#[allow(dead_code)]
fn get_integer_value(value: &Integer) -> i128 {
    i128::from_str_radix(&value.to_str_radix(10), 10).unwrap_or_else(|e| core::panic!("{e:#?}"))
}

impl Into<Integer> for JerInteger {
    fn into(self) -> Integer {
        self.0.into()
    }
}