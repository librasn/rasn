//! # Encoding JER.
pub mod error;

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "BigInt")]
pub(crate) struct IntegerDef(#[serde(getter = "get_integer_value")] i128);

#[allow(dead_code)]
pub(crate) fn get_integer_value(value: &BigInt) -> i128 {
    i128::from_str_radix(&value.to_str_radix(10), 10).unwrap_or_else(|e| core::panic!("{e:#?}"))
}

impl Into<BigInt> for IntegerDef {
    fn into(self) -> BigInt {
        self.0.into()
    }
}
