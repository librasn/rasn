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
            name: Name::john(),
            title: String::from("Director").try_into().unwrap(),
            number: <_>::default(),
            date_of_hire: Date(String::from("19710917").try_into().unwrap()),
            name_of_spouse: Name::mary(),
            children: vec![ChildInformation::ralph(), ChildInformation::susan()],
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

impl ChildInformation {
    pub fn ralph() -> Self {
        Self {
            name: Name {
                given_name: String::from("Ralph").try_into().unwrap(),
                initial: String::from("T").try_into().unwrap(),
                family_name: String::from("Smith").try_into().unwrap(),
            },
            date_of_birth: Date(String::from("19571111").try_into().unwrap()),
        }
    }

    pub fn susan() -> Self {
        Self {
            name: Name {
                given_name: String::from("Susan").try_into().unwrap(),
                initial: String::from("B").try_into().unwrap(),
                family_name: String::from("Jones").try_into().unwrap(),
            },
            date_of_birth: Date(String::from("19590717").try_into().unwrap()),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
pub struct Name {
    pub given_name: VisibleString,
    pub initial: VisibleString,
    pub family_name: VisibleString,
}

impl Name {
    pub fn john() -> Self {
        Self {
            given_name: String::from("John").try_into().unwrap(),
            initial: String::from("P").try_into().unwrap(),
            family_name: String::from("Smith").try_into().unwrap(),
        }
    }

    pub fn mary() -> Self {
        Self {
            given_name: String::from("Mary").try_into().unwrap(),
            initial: String::from("T").try_into().unwrap(),
            family_name: String::from("Smith").try_into().unwrap(),
        }
    }

    pub fn susan() -> Self {
        Self {
            given_name: String::from("Susan").try_into().unwrap(),
            initial: String::from("B").try_into().unwrap(),
            family_name: String::from("Jones").try_into().unwrap(),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 2), delegate, value("0..9999", extensible))]
pub struct ExtensibleEmployeeNumber(pub Integer);

impl From<EmployeeNumber> for ExtensibleEmployeeNumber {
    fn from(number: EmployeeNumber) -> Self {
        Self(number.0)
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 2), delegate)]
pub struct EmployeeNumber(pub Integer);

impl Default for EmployeeNumber {
    fn default() -> Self {
        Self(51.into())
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 3), delegate)]
pub struct Date(pub VisibleString);

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set, tag(application, 0))]
pub struct PersonnelRecordWithConstraints {
    pub name: NameWithConstraints,
    #[rasn(tag(explicit(0)))]
    pub title: VisibleString,
    pub number: EmployeeNumber,
    #[rasn(tag(explicit(1)))]
    pub date_of_hire: DateWithConstraints,
    #[rasn(tag(explicit(2)))]
    pub name_of_spouse: NameWithConstraints,
    #[rasn(tag(3), default)]
    pub children: Vec<ChildInformationWithConstraints>,
}

impl Default for PersonnelRecordWithConstraints {
    fn default() -> Self {
        PersonnelRecord::default().into()
    }
}

impl From<PersonnelRecord> for PersonnelRecordWithConstraints {
    fn from(record: PersonnelRecord) -> Self {
        Self {
            name: record.name.into(),
            title: record.title,
            number: record.number,
            date_of_hire: record.date_of_hire.into(),
            name_of_spouse: record.name_of_spouse.into(),
            children: record.children.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set, tag(application, 0))]
pub struct ExtensiblePersonnelRecord {
    pub name: ExtensibleName,
    #[rasn(tag(explicit(0)))]
    pub title: VisibleString,
    pub number: ExtensibleEmployeeNumber,
    #[rasn(tag(explicit(1)))]
    pub date_of_hire: ExtensibleDate,
    #[rasn(tag(explicit(2)))]
    pub name_of_spouse: ExtensibleName,
    #[rasn(tag(3), default)]
    pub children: Vec<ExtensibleChildInformation>,
}

impl Default for ExtensiblePersonnelRecord {
    fn default() -> Self {
        Self {
            name: Name::john().into(),
            title: String::from("Director").try_into().unwrap(),
            number: ExtensibleEmployeeNumber(5.into()),
            date_of_hire: ExtensibleDate(VisibleString::try_from("19710917").unwrap()),
            name_of_spouse: Name::mary().into(),
            children: vec![
                ChildInformation::ralph().into(),
                ExtensibleChildInformation::susan(),
            ],
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set)]
#[non_exhaustive]
pub struct ExtensibleChildInformation {
    name: ExtensibleName,
    #[rasn(tag(explicit(0)))]
    date_of_birth: ExtensibleDate,
    #[rasn(extension_addition)]
    sex: Option<Sex>,
}

#[derive(AsnType, Decode, Encode, Debug, Clone, Copy, PartialEq)]
#[rasn(enumerated)]
pub enum Sex {
    Male = 1,
    Female = 2,
    Unknown = 3,
}

impl ExtensibleChildInformation {
    pub fn susan() -> Self {
        Self {
            name: Name::susan().into(),
            date_of_birth: ExtensibleDate(String::from("19590717").try_into().unwrap()),
            sex: Some(Sex::Female),
        }
    }
}

impl From<ChildInformation> for ExtensibleChildInformation {
    fn from(info: ChildInformation) -> Self {
        Self {
            name: info.name.into(),
            date_of_birth: info.date_of_birth.into(),
            sex: None,
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(set)]
pub struct ChildInformationWithConstraints {
    name: NameWithConstraints,
    #[rasn(tag(explicit(0)))]
    date_of_birth: DateWithConstraints,
}

impl From<ChildInformation> for ChildInformationWithConstraints {
    fn from(info: ChildInformation) -> Self {
        Self {
            name: info.name.into(),
            date_of_birth: info.date_of_birth.into(),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
#[non_exhaustive]
pub struct ExtensibleName {
    pub given_name: ExtensibleNameString,
    #[rasn(size(1))]
    pub initial: ExtensibleNameString,
    pub family_name: ExtensibleNameString,
}

impl From<Name> for ExtensibleName {
    fn from(name: Name) -> Self {
        Self {
            given_name: name.given_name.into(),
            initial: name.initial.into(),
            family_name: name.family_name.into(),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
pub struct NameWithConstraints {
    pub given_name: NameString,
    #[rasn(size(1))]
    pub initial: NameString,
    pub family_name: NameString,
}

impl From<Name> for NameWithConstraints {
    fn from(name: Name) -> Self {
        Self {
            given_name: name.given_name.into(),
            initial: name.initial.into(),
            family_name: name.family_name.into(),
        }
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(
    tag(application, 3),
    delegate,
    from("0..=9"),
    size(8, extensible, "9..20")
)]
pub struct ExtensibleDate(pub VisibleString);

impl From<Date> for ExtensibleDate {
    fn from(name: Date) -> Self {
        Self(name.0)
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 3), delegate, from("0..=9"), size(8))]
pub struct DateWithConstraints(pub VisibleString);

impl From<Date> for DateWithConstraints {
    fn from(name: Date) -> Self {
        Self(name.0)
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(delegate, from("a..=z", "A..=Z", "-", "."), size("1..=64", extensible))]
pub struct ExtensibleNameString(pub VisibleString);

impl From<VisibleString> for ExtensibleNameString {
    fn from(name: VisibleString) -> Self {
        Self(name)
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(delegate, from("a..=z", "A..=Z", "-", "."), size("1..=64"))]
pub struct NameString(pub VisibleString);

impl From<VisibleString> for NameString {
    fn from(name: VisibleString) -> Self {
        Self(name)
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
pub struct InitialString {
    #[rasn(size(1))]
    pub initial: NameString,
}

macro_rules! test {
    ($($name:ident ( $codec:ident ) : $typ:ty = $value:expr => $expected:expr;)+) => {
        $(
            #[test]
            fn $name () {
                const EXPECTED: &[u8] = $expected;
                let default: $typ = $value;

                assert_eq!(
                    EXPECTED,
                    rasn::$codec::encode(&default).unwrap()
                );
                assert_eq!(
                    default,
                    rasn::$codec::decode(EXPECTED).unwrap()
                );
            }
        )+
    }
}

test! {
    unconstrained_ber(ber): PersonnelRecord = <_>::default() => &[
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

    unconstrained_aper(aper): PersonnelRecord = <_>::default() => &[
        0x80, 0x04, 0x4A, 0x6F, 0x68, 0x6E, 0x01, 0x50, 0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x01, 0x33, 0x08, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6F,
        0x72, 0x08, 0x31, 0x39, 0x37, 0x31, 0x30, 0x39, 0x31, 0x37, 0x04, 0x4D,
        0x61, 0x72, 0x79, 0x01, 0x54, 0x05, 0x53, 0x6D, 0x69, 0x74, 0x68, 0x02,
        0x05, 0x52, 0x61, 0x6C, 0x70, 0x68, 0x01, 0x54, 0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x08, 0x31, 0x39, 0x35, 0x37, 0x31, 0x31, 0x31, 0x31, 0x05,
        0x53, 0x75, 0x73, 0x61, 0x6E, 0x01, 0x42, 0x05, 0x4A, 0x6F, 0x6E, 0x65,
        0x73, 0x08, 0x31, 0x39, 0x35, 0x39, 0x30, 0x37, 0x31, 0x37,
    ];

    unconstrained_uper(uper): PersonnelRecord = <_>::default() => &[
        0x82, 0x4A, 0xDF, 0xA3, 0x70, 0x0D, 0x00, 0x5A, 0x7B, 0x74, 0xF4, 0xD0,
        0x02, 0x66, 0x11, 0x13, 0x4F, 0x2C, 0xB8, 0xFA, 0x6F, 0xE4, 0x10, 0xC5,
        0xCB, 0x76, 0x2C, 0x1C, 0xB1, 0x6E, 0x09, 0x37, 0x0F, 0x2F, 0x20, 0x35,
        0x01, 0x69, 0xED, 0xD3, 0xD3, 0x40, 0x10, 0x2D, 0x2C, 0x3B, 0x38, 0x68,
        0x01, 0xA8, 0x0B, 0x4F, 0x6E, 0x9E, 0x9A, 0x02, 0x18, 0xB9, 0x6A, 0xDD,
        0x8B, 0x16, 0x2C, 0x41, 0x69, 0xF5, 0xE7, 0x87, 0x70, 0x0C, 0x20, 0x59,
        0x5B, 0xF7, 0x65, 0xE6, 0x10, 0xC5, 0xCB, 0x57, 0x2C, 0x1B, 0xB1, 0x6E,
    ];

    constrained_uper_name_string(uper): NameString = VisibleString::try_from("John").unwrap().into() => &[0xC, 0xBA, 0xA3, 0xA4];
    constrained_aper_name_string(uper): NameString = VisibleString::try_from("John").unwrap().into() => &[0xC, 0x4A, 0x6F, 0x68, 0x6E];
    constrained_uper_date(uper): DateWithConstraints = DateWithConstraints(VisibleString::try_from("19710917").unwrap()) => &[0b00011001, 0b01110001, 0b00001001, 0b00010111];
    constrained_uper_name(uper): NameWithConstraints = Name {
        given_name: String::from("John").try_into().unwrap(),
        initial: String::from("P").try_into().unwrap(),
        family_name: String::from("Smith").try_into().unwrap(),
    }.into() => &[0xC, 0xBA, 0xA3, 0xA5, 0x11, 0x14, 0xA2, 0x4B, 0xE3];
    constrained_uper_initial(uper): InitialString = InitialString {
        initial: VisibleString::try_from("P").unwrap().into(),
    }.into() => &[0x44];
    constrained_aper_initial(uper): InitialString = InitialString {
        initial: VisibleString::try_from("P").unwrap().into(),
    }.into() => &[0x50];

    constrained_uper(uper): PersonnelRecordWithConstraints = <_>::default() => &[
        0x86, 0x5D, 0x51, 0xD2, 0x88, 0x8A, 0x51, 0x25, 0xF1, 0x80, 0x99, 0x84,
        0x44, 0xD3, 0xCB, 0x2E, 0x3E, 0x9B, 0xF9, 0x0c, 0xB8, 0x84, 0x8B, 0x86,
        0x73, 0x96, 0xE8, 0xA8, 0x8A, 0x51, 0x25, 0xF1, 0x81, 0x08, 0x9B, 0x93,
        0xD7, 0x1A, 0xA2, 0x29, 0x44, 0x97, 0xC6, 0x32, 0xAE, 0x22, 0x22, 0x22,
        0x98, 0x5C, 0xE5, 0x21, 0x88, 0x5D, 0x54, 0xC1, 0x70, 0xCA, 0xC8,
        0x38, 0xB8
    ];

    extensible_uper(uper): ExtensiblePersonnelRecord = <_>::default() => &[
        0x40, 0xCB, 0xAA, 0x3A, 0x51, 0x08, 0xA5, 0x12, 0x5F, 0x18, 0x03, 0x30,
        0x88, 0x9A, 0x79, 0x65, 0xC7, 0xD3, 0x7F, 0x20, 0xCB, 0x88, 0x48, 0xB8,
        0x19, 0xCE, 0x5B, 0xA2, 0xA1, 0x14, 0xA2, 0x4B, 0xE3, 0x01, 0x13, 0x72,
        0x7A, 0xE3, 0x54, 0x22, 0x94, 0x49, 0x7C, 0x61, 0x95, 0x71, 0x11, 0x18,
        0x22, 0x98, 0x5C, 0xE5, 0x21, 0x84, 0x2E, 0xAA, 0x60, 0xB8, 0x32, 0xB2,
        0x0E, 0x2E, 0x02, 0x02, 0x80,
    ];

    constrained_aper(aper): PersonnelRecordWithConstraints = <_>::default() => &[
        0x86, 0x4a, 0x6f, 0x68, 0x6e, 0x50, 0x10, 0x53, 0x6d, 0x69, 0x74, 0x68,
        0x01, 0x33, 0x08, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x19,
        0x71, 0x09, 0x17, 0x0c, 0x4d, 0x61, 0x72, 0x79, 0x54, 0x10, 0x53, 0x6d,
        0x69, 0x74, 0x68, 0x02, 0x10, 0x52, 0x61, 0x6c, 0x70, 0x68, 0x54, 0x10,
        0x53, 0x6d, 0x69, 0x74, 0x68, 0x19, 0x57, 0x11, 0x11, 0x10, 0x53, 0x75,
        0x73, 0x61, 0x6e, 0x42, 0x10, 0x4a, 0x6f, 0x6e, 0x65, 0x73, 0x19, 0x59,
        0x07, 0x17
    ];
}
