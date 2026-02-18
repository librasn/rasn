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

    assert_eq!(
        t,
        Some(TestTime::Utc("1970-01-01T00:00:10Z".parse().unwrap()))
    );
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
            next_update: Some(TestTime::Utc("1970-01-01T00:00:10Z".parse().unwrap()))
        }
    );
}
