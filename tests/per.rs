use rasn::{AsnType, Encode, Decode, Tag, types::Class, types};

#[derive(Decode, Debug, Encode, PartialEq, Eq)]
struct PersonnelRecord {
    name: Name,
    #[rasn(tag(0))]
    title: String,
    number: EmployeeNumber,
    #[rasn(tag(1))]
    date_of_hire: Date,
    #[rasn(tag(2))]
    name_of_spouse: Name,
    #[rasn(tag(3))]
    children: Vec<ChildInformation>,
}

impl AsnType for PersonnelRecord {
    const TAG: Tag = Tag::new(Class::Application, 0);
}

#[derive(AsnType, Debug, Decode, Encode, PartialEq, Eq)]
struct ChildInformation {
    name: Name,
    #[rasn(tag(0))]
    date_of_birth: Date
}

#[derive(Debug, Decode, Encode, PartialEq, Eq)]
struct Name {
    given_name: String,
    initial: String,
    family_name: String,
}

impl AsnType for Name {
    const TAG: Tag = Tag::new(Class::Application, 2);
}

#[derive(Debug, PartialEq, Eq)]
struct App2;
#[derive(Debug, PartialEq, Eq)]
struct App3;

impl AsnType for App2 {
    const TAG: Tag = Tag::new(Class::Application, 2);
}

impl AsnType for App3 {
    const TAG: Tag = Tag::new(Class::Application, 3);
}

type EmployeeNumber = types::Implicit<App2, u64>;
type Date = types::Implicit<App3, String>;

fn record() -> PersonnelRecord {
    PersonnelRecord {
        name: Name {
            given_name: "John".into(),
            initial: "P".into(),
            family_name: "Smith".into(),
        },
        title: "Director".into(),
        number: EmployeeNumber::new(51),
        date_of_hire: Date::new("19710917".into()),
        name_of_spouse: Name {
            given_name: "Mary".into(),
            initial: "T".into(),
            family_name: "Smith".into(),
        },
        children: vec![
            ChildInformation {
                name: Name {
                    given_name: "Ralph".into(),
                    initial: "T".into(),
                    family_name: "Smith".into(),
                },
                date_of_birth: Date::new("19571111".into()),
            },
            ChildInformation {
                name: Name {
                    given_name: "Susan".into(),
                    initial: "T".into(),
                    family_name: "Smith".into(),
                },
                date_of_birth: Date::new("19590717".into()),
            },
        ],
    }
}

#[test]
fn aligned() {
    let record = record();
    let bytes = rasn::aper::encode(&record).unwrap();

    assert_eq!(bytes.len(), 94);
    assert_eq!(bytes, vec![
        0x80, 0x04, 0x4A, 0x6F,
        0x68, 0x6E, 0x01, 0x50,
        0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x01, 0x33,
        0x08, 0x44, 0x69, 0x72,
        0x65, 0x63, 0x74, 0x6F,
        0x72, 0x08, 0x31, 0x39,
        0x37, 0x31, 0x30, 0x39,
        0x31, 0x37, 0x04, 0x4D,
        0x61, 0x72, 0x79, 0x01,
        0x54, 0x05, 0x53, 0x6D,
        0x69, 0x74, 0x68, 0x02,
        0x05, 0x52, 0x61, 0x6C,
        0x70, 0x68, 0x01, 0x54,
        0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x08, 0x31,
        0x39, 0x35, 0x37, 0x31,
        0x31, 0x31, 0x31, 0x05,
        0x53, 0x75, 0x73, 0x61,
        0x6E, 0x01, 0x42, 0x05,
        0x4A, 0x6F, 0x6E, 0x65,
        0x73, 0x08, 0x31, 0x39,
        0x35, 0x39, 0x30, 0x37,
        0x31, 0x37
    ]);

    let decoded = rasn::aper::decode(&bytes).unwrap();
    assert_eq!(record, decoded);
}
