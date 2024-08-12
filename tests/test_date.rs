use std::str::FromStr;

use rasn::types::*;

pub struct TestDate(pub Date);

#[test]
pub fn date_from_str() {
    let test_date = TestDate(Date::from_str("2024-04-04").unwrap());
    assert_eq!(test_date.0.to_string(), "2024-04-04");
}
