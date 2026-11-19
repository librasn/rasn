#[test]
fn issue_141() {
    use rasn::{AsnType, Decode, prelude::*};

    /// A general time type.
    #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[rasn(choice)]
    pub enum TestTime {
        Utc(UtcTime),
        General(GeneralizedTime),
    }

    let s = &[23, 13, 55, 48, 48, 49, 48, 49, 48, 48, 48, 48, 49, 48, 90];
    let t: Option<TestTime> = rasn::der::decode(s.as_slice()).unwrap();

    assert_eq!(t, Some(TestTime::Utc(utc(1970, 1, 1, 0, 0, 10))));
}

#[test]
fn complementary_issue_141() {
    use rasn::prelude::*;

    /// A general time type.
    #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[rasn(choice)]
    pub enum TestTime {
        Utc(UtcTime),
        General(GeneralizedTime),
    }

    /// A general time type.
    #[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[rasn(tag(application, 1))]
    pub struct TestSeq {
        pub next_update: Option<TestTime>,
    }

    let s = &[
        97, 15, 23, 13, 55, 48, 48, 49, 48, 49, 48, 48, 48, 48, 49, 48, 90,
    ];
    let t: TestSeq = rasn::der::decode(s.as_slice()).unwrap();

    assert_eq!(
        t,
        TestSeq {
            next_update: Some(TestTime::Utc(utc(1970, 1, 1, 0, 0, 10)))
        }
    );
}

fn utc(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> rasn::types::UtcTime {
    let date = rasn::types::Date::new(year, month, day).unwrap();
    rasn::types::UtcTime::new(date, hour, minute, second).unwrap()
}
