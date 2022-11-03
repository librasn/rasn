use pretty_assertions::assert_eq;
use rasn::prelude::*;

#[derive(AsnType, Encode, Debug, PartialEq)]
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

impl rasn::Decode for PersonnelRecord {
    fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::Tag,
        constraints: rasn::types::Constraints<'constraints>,
    ) -> core::result::Result<Self, D::Error> {
        enum PersonnelRecordFields {
            Field0(Field0),
            Field1(Field1),
            Field2(Field2),
            Field3(Field3),
            Field4(Field4),
            Field5(Field5),
        }
        impl rasn::AsnType for PersonnelRecordFields {
            const TAG: rasn::Tag = { rasn::Tag::EOC };
            const TAG_TREE: rasn::TagTree = {
                const LIST: &'static [rasn::TagTree] = &[{
                    const VARIANT_LIST: &'static [rasn::TagTree] = &[
                        <Field0 as rasn::AsnType>::TAG_TREE,
                        <Field1 as rasn::AsnType>::TAG_TREE,
                        <Field2 as rasn::AsnType>::TAG_TREE,
                        <Field3 as rasn::AsnType>::TAG_TREE,
                        <Field4 as rasn::AsnType>::TAG_TREE,
                        <Field5 as rasn::AsnType>::TAG_TREE,
                    ];
                    const VARIANT_TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(VARIANT_LIST);
                    const _: () = if !VARIANT_TAG_TREE.is_unique() {
                        {
                            panic!("PersonnelRecordFields's variants is not un
ique, ensure that your variants's tags are correct.")
                        }
                    };
                    VARIANT_TAG_TREE
                }];
                const TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(LIST);
                const _: () = if !TAG_TREE.is_unique() {
                    {
                        panic!("PersonnelRecordFields's variants is not unique
, ensure that your variants's tags are correct.")
                    }
                };
                TAG_TREE
            };
        }
        #[automatically_derived]
        impl rasn::Decode for PersonnelRecordFields {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_explicit_prefix(tag)
            }
            fn decode<D: rasn::Decoder>(decoder: &mut D) -> core::result::Result<Self, D::Error> {
                if let Ok(value) = <_>::decode(decoder).map(Self::Field0) {
                    return Ok(value);
                };
                if let Ok(value) = <_>::decode(decoder).map(Self::Field1) {
                    return Ok(value);
                };
                if let Ok(value) = <_>::decode(decoder).map(Self::Field2) {
                    return Ok(value);
                };
                if let Ok(value) = <_>::decode(decoder).map(Self::Field3) {
                    return Ok(value);
                };
                if let Ok(value) = <_>::decode(decoder).map(Self::Field4) {
                    return Ok(value);
                };
                if let Ok(value) = <_>::decode(decoder).map(Self::Field5) {
                    return Ok(value);
                }
                Err(rasn::de::Error::no_valid_choice("PersonnelRecordFields"))
            }
        }
        #[automatically_derived]
        impl rasn::Encode for PersonnelRecordFields {
            fn encode<E: rasn::Encoder>(
                &self,
                encoder: &mut E,
            ) -> core::result::Result<(), E::Error> {
                match self {
                    PersonnelRecordFields::Field0(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                    PersonnelRecordFields::Field1(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                    PersonnelRecordFields::Field2(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                    PersonnelRecordFields::Field3(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                    PersonnelRecordFields::Field4(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                    PersonnelRecordFields::Field5(value) => {
                        rasn::Encode::encode(value, encoder).map(drop)
                    }
                }
                .map(drop)
            }
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                encoder.encode_explicit_prefix(tag, self).map(drop)
            }
        }
        pub struct Field0(Name);
        #[automatically_derived]
        impl rasn::AsnType for Field0 {
            const TAG: rasn::Tag = { <Name as rasn::AsnType>::TAG };
        }
        impl rasn::Decode for Field0 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                <Name as rasn::Decode>::decode_with_tag_and_constraints(decoder, tag, constraints)
                    .map(Self)
            }
        }
        impl rasn::Encode for Field0 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                <Name as rasn::Encode>::encode_with_tag_and_constraints(
                    &self.0,
                    encoder,
                    tag,
                    constraints,
                )
            }
        }
        pub struct Field1(VisibleString);
        #[automatically_derived]
        impl rasn::AsnType for Field1 {
            const TAG: rasn::Tag = { rasn::Tag::new(rasn::types::Class::Context, 0) };
        }
        impl rasn::Decode for Field1 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder
                    .decode_explicit_prefix::<VisibleString>(tag)
                    .map(Self)
            }
        }
        impl rasn::Encode for Field1 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                encoder
                    .encode_explicit_prefix(rasn::Tag::new(rasn::types::Class::Context, 0), &self.0)
                    .map(drop)
            }
        }
        pub struct Field2(EmployeeNumber);
        #[automatically_derived]
        impl rasn::AsnType for Field2 {
            const TAG: rasn::Tag = { <EmployeeNumber as rasn::AsnType>::TAG };
        }
        impl rasn::Decode for Field2 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                <EmployeeNumber as rasn::Decode>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self)
            }
        }
        impl rasn::Encode for Field2 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                <EmployeeNumber as rasn::Encode>::encode_with_tag_and_constraints(
                    &self.0,
                    encoder,
                    tag,
                    constraints,
                )
            }
        }
        pub struct Field3(Date);
        #[automatically_derived]
        impl rasn::AsnType for Field3 {
            const TAG: rasn::Tag = { rasn::Tag::new(rasn::types::Class::Context, 1) };
        }
        impl rasn::Decode for Field3 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_explicit_prefix::<Date>(tag).map(Self)
            }
        }
        impl rasn::Encode for Field3 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                encoder
                    .encode_explicit_prefix(rasn::Tag::new(rasn::types::Class::Context, 1), &self.0)
                    .map(drop)
            }
        }
        pub struct Field4(Name);
        #[automatically_derived]
        impl rasn::AsnType for Field4 {
            const TAG: rasn::Tag = { rasn::Tag::new(rasn::types::Class::Context, 2) };
        }
        impl rasn::Decode for Field4 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_explicit_prefix::<Name>(tag).map(Self)
            }
        }
        impl rasn::Encode for Field4 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                encoder
                    .encode_explicit_prefix(rasn::Tag::new(rasn::types::Class::Context, 2), &self.0)
                    .map(drop)
            }
        }
        pub struct Field5(Vec<ChildInformation>);
        #[automatically_derived]
        impl rasn::AsnType for Field5 {
            const TAG: rasn::Tag = { rasn::Tag::new(rasn::types::Class::Context, 3) };
        }
        impl rasn::Decode for Field5 {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                <Vec<ChildInformation> as rasn::Decode>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self)
            }
        }
        impl rasn::Encode for Field5 {
            fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let i0 = &self.0;
                <Vec<ChildInformation> as rasn::Encode>::encode_with_tag_and_constraints(
                    &self.0,
                    encoder,
                    tag,
                    constraints,
                )
            }
        }
        decoder.decode_set::<PersonnelRecordFields, _, _, _>(
            tag,
            constraints,
            |decoder, index, tag| {
                const Field0Const: Tag = <Name as rasn::AsnType>::TAG;
                const Field1Const: Tag = rasn::Tag::new(rasn::types::Class::Context, 0);
                const Field2Const: Tag = <EmployeeNumber as rasn::AsnType>::TAG;
                const Field3Const: Tag = rasn::Tag::new(rasn::types::Class::Context, 1);
                const Field4Const: Tag = rasn::Tag::new(rasn::types::Class::Context, 2);
                const Field5Const: Tag = rasn::Tag::new(rasn::types::Class::Context, 3);
                Ok(match (index, tag) {
                    (0usize, Field0Const) => PersonnelRecordFields::Field0(<_>::decode(decoder)?),
                    (1usize, Field1Const) => PersonnelRecordFields::Field1(<_>::decode(decoder)?),
                    (2usize, Field2Const) => PersonnelRecordFields::Field2(<_>::decode(decoder)?),
                    (3usize, Field3Const) => PersonnelRecordFields::Field3(<_>::decode(decoder)?),
                    (4usize, Field4Const) => PersonnelRecordFields::Field4(<_>::decode(decoder)?),
                    (5usize, Field5Const) => PersonnelRecordFields::Field5(<_>::decode(decoder)?),
                    _ => return Err(rasn::de::Error::custom("Unknown field provided.")),
                })
            },
            |fields| {
                let mut name = None;
                let mut title = None;
                let mut number = None;
                let mut date_of_hire = None;
                let mut name_of_spouse = None;
                let mut children = None;
                for field in fields {
                    match field {
                        PersonnelRecordFields::Field0(value) => {
                            if name.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("name"));
                            }
                        }
                        PersonnelRecordFields::Field1(value) => {
                            if title.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("title"));
                            }
                        }
                        PersonnelRecordFields::Field2(value) => {
                            if number.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("number"));
                            }
                        }
                        PersonnelRecordFields::Field3(value) => {
                            if date_of_hire.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("date_of_hire"));
                            }
                        }
                        PersonnelRecordFields::Field4(value) => {
                            if name_of_spouse.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("name_of_spouse"));
                            }
                        }
                        PersonnelRecordFields::Field5(value) => {
                            if children.replace(value.0).is_some() {
                                return Err(rasn::de::Error::duplicate_field("children"));
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| rasn::de::Error::missing_field("name"))?;
                let title = title.ok_or_else(|| rasn::de::Error::missing_field("title"))?;
                let number = number.ok_or_else(|| rasn::de::Error::missing_field("number"))?;
                let date_of_hire =
                    date_of_hire.ok_or_else(|| rasn::de::Error::missing_field("date_of_hire"))?;
                let name_of_spouse = name_of_spouse
                    .ok_or_else(|| rasn::de::Error::missing_field("name_of_spouse"))?;
                let children =
                    children.ok_or_else(|| rasn::de::Error::missing_field("children"))?;
                Ok(Self {
                    name,
                    title,
                    number,
                    date_of_hire,
                    name_of_spouse,
                    children,
                })
            },
        )
    }
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
            number: <_>::default(),
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

impl Default for EmployeeNumber {
    fn default() -> Self {
        Self(51.into())
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 3), delegate)]
pub struct Date(pub VisibleString);

#[test]
fn name_uper() {
    const EXPECTED: &[u8] = &[
        0b00000100,
        0b10010101,
        0b10111111,
        0b01000110,
        0b11100000,
        0b00011010,
        0b00000000,
        0b10110100,
        0b11110110,
        0b11101001,
        0b11101001,
        0b10100000
    ];

    let default = Name {
        given_name: String::from("John").into(),
        initial: String::from("P").into(),
        family_name: String::from("Smith").into(),
    };

    assert_eq!(
        EXPECTED,
        rasn::uper::encode(&default).unwrap()
    );
    assert_eq!(
        default,
        rasn::uper::decode(EXPECTED).unwrap()
    );
}

#[test]
fn title_uper() {
    const EXPECTED: &[u8] = &[
        0b00001000,
        0b10001001,
        0b10100111,
        0b10010110,
        0b01011100,
        0b01111101,
        0b00110111,
        0b11110010,
    ];

    let default = VisibleString::from("Director");

    assert_eq!(
        EXPECTED,
        rasn::uper::encode(&default).unwrap()
    );
    assert_eq!(
        default,
        rasn::uper::decode(EXPECTED).unwrap()
    );
}

#[test]
fn employee_number_uper() {
    const EXPECTED: &[u8] = &[1, 0b00110011];
    let default = <EmployeeNumber>::default();

    assert_eq!(
        EXPECTED,
        rasn::uper::encode(&default).unwrap()
    );
    assert_eq!(
        default,
        rasn::uper::decode(EXPECTED).unwrap()
    );
}

macro_rules! test {
    ($($codec:ident : $typ:ty = $expected:expr;)+) => {
        $(
            #[test]
            fn $codec () {
                const EXPECTED: &[u8] = $expected;
                let default = <$typ>::default();

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
    ber: PersonnelRecord = &[
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

    aper: PersonnelRecord = &[
        0x80, 0x04, 0x4A, 0x6F, 0x68, 0x6E, 0x01, 0x50, 0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x01, 0x33, 0x08, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6F,
        0x72, 0x08, 0x31, 0x39, 0x37, 0x31, 0x30, 0x39, 0x31, 0x37, 0x04, 0x4D,
        0x61, 0x72, 0x79, 0x01, 0x54, 0x05, 0x53, 0x6D, 0x69, 0x74, 0x68, 0x02,
        0x05, 0x52, 0x61, 0x6C, 0x70, 0x68, 0x01, 0x54, 0x05, 0x53, 0x6D, 0x69,
        0x74, 0x68, 0x08, 0x31, 0x39, 0x35, 0x37, 0x31, 0x31, 0x31, 0x31, 0x05,
        0x53, 0x75, 0x73, 0x61, 0x6E, 0x01, 0x42, 0x05, 0x4A, 0x6F, 0x6E, 0x65,
        0x73, 0x08, 0x31, 0x39, 0x35, 0x39, 0x30, 0x37, 0x31, 0x37,
    ];

    uper: PersonnelRecord = &[
        0x82, 0x4A, 0xDF, 0xA3, 0x70, 0x0D, 0x00, 0x5A, 0x7B, 0x74, 0xF4, 0xD0,
        0x02, 0x66, 0x11, 0x13, 0x4F, 0x2C, 0xB8, 0xFA, 0x6F, 0xE4, 0x10, 0xC5,
        0xCB, 0x76, 0x2C, 0x1C, 0xB1, 0x6E, 0x09, 0x37, 0x0F, 0x2F, 0x20, 0x35,
        0x01, 0x69, 0xED, 0xD3, 0xD3, 0x40, 0x10, 0x2D, 0x2C, 0x3B, 0x38, 0x68,
        0x01, 0xA8, 0x0B, 0x4F, 0x6E, 0x9E, 0x9A, 0x02, 0x18, 0xB9, 0x6A, 0xDD,
        0x8B, 0x16, 0x2C, 0x41, 0x69, 0xF5, 0xE7, 0x87, 0x70, 0x0C, 0x20, 0x59,
        0x5B, 0xF7, 0x65, 0xE6, 0x10, 0xC5, 0xCB, 0x57, 0x2C, 0x1B, 0xB1, 0x6E,
    ];
}
