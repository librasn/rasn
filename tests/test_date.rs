use rasn::types::*;

pub struct TestDate(pub Date);

#[test]
pub fn date_to_string() {
    let test_date = TestDate(Date::new(2024, 4, 4).unwrap());
    assert_eq!(test_date.0.to_string(), "2024-04-04");
}
