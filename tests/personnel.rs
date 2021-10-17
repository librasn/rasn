use pretty_assertions::assert_eq;
use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set, tag(application, 0))]
pub struct PersonnelRecord {
    pub name: Name,
    #[rasn(tag(explicit(0)))]
    pub title: VisibleString,
    pub number: EmployeeNumber,
    #[rasn(tag(explicit(1)))]
    pub date_of_hire: Date,
    #[rasn(tag(explicit(2)))]
    pub name_of_spouse: Name,
    #[rasn(tag(3), default)]
    pub children: Vec<ChildInformation>,
}

impl Default for PersonnelRecord {
    fn default() -> Self {
        Self {
            name: Name {
                given_name: String::from("John").into(),
                initial: String::from("P").into(),
                family_name: String::from("Smith").into(),
            },
            title: String::from("Director").into(),
            number: EmployeeNumber(51.into()),
            date_of_hire: Date(String::from("19710917").into()),
            name_of_spouse: Name {
                given_name: String::from("Mary").into(),
                initial: String::from("T").into(),
                family_name: String::from("Smith").into(),
            },
            children: vec![
                ChildInformation {
                    name: Name {
                        given_name: String::from("Ralph").into(),
                        initial: String::from("T").into(),
                        family_name: String::from("Smith").into(),
                    },
                    date_of_birth: Date(String::from("19571111").into()),
                },
                ChildInformation {
                    name: Name {
                        given_name: String::from("Susan").into(),
                        initial: String::from("B").into(),
                        family_name: String::from("Jones").into(),
                    },
                    date_of_birth: Date(String::from("19590717").into()),
                },
            ],
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set)]
pub struct ChildInformation {
    pub name: Name,
    #[rasn(tag(explicit(0)))]
    pub date_of_birth: Date,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
pub struct Name {
    pub given_name: VisibleString,
    pub initial: VisibleString,
    pub family_name: VisibleString,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 2), delegate)]
pub struct EmployeeNumber(pub Integer);

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 3), delegate)]
pub struct Date(pub VisibleString);

#[test]
fn ber() {
    #[rustfmt::skip]
    const EXPECTED: &[u8] = &[
        0x60, 0x81, 0x85,

            0x61, 0x10,
                0x1a, 0x4, 0x4a, 0x6f, 0x68, 0x6e,
                0x1a, 0x1, 0x50,
                0x1a, 0x5, 0x53, 0x6d, 0x69, 0x74, 0x68,

            0x42, 0x01, 0x33,

            0xA0, 0x0A,
                0x1a, 0x8, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72,

            0xA1, 0x0A,
                0x43, 0x8, 0x31, 0x39, 0x37, 0x31, 0x30, 0x39, 0x31, 0x37,

            0xA2, 0x12,
                0x61, 0x10,
                    0x1a, 0x4, 0x4d, 0x61, 0x72, 0x79,
                    0x1a, 0x1, 0x54,
                    0x1a, 0x5, 0x53, 0x6d, 0x69, 0x74, 0x68,

            0xA3, 0x42,
                0x31, 0x1F,
                    0x61, 0x11,
                        0x1a, 0x5, 0x52, 0x61, 0x6c, 0x70, 0x68,
                        0x1a, 0x1, 0x54,
                        0x1a, 0x5, 0x53, 0x6d, 0x69, 0x74, 0x68,
                    0xA0, 0x0A,
                        0x43, 0x8, 0x31, 0x39, 0x35, 0x37, 0x31, 0x31, 0x31, 0x31,
                0x31, 0x1F,
                    0x61, 0x11,
                        0x1a, 0x5, 0x53, 0x75, 0x73, 0x61, 0x6e,
                        0x1a, 0x1, 0x42,
                        0x1a, 0x5, 0x4a, 0x6f, 0x6e, 0x65, 0x73,
                    0xA0, 0x0A,
                        0x43, 0x8, 0x31, 0x39, 0x35, 0x39, 0x30, 0x37, 0x31, 0x37,
    ];

    assert_eq!(
        EXPECTED,
        rasn::der::encode(&PersonnelRecord::default()).unwrap()
    );
}
