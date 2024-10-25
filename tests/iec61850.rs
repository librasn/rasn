use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(choice)]
enum IEC61850SpecificProtocol {
    GoosePdu(IECGoosePdu),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(tag(application, 1))]
struct IECGoosePdu {
    #[rasn(tag(0))]
    gocb_ref: VisibleString,
    #[rasn(tag(1))]
    time_allowed_to_live: Integer,
    #[rasn(tag(2))]
    dat_set: VisibleString,
    #[rasn(tag(3))]
    go_id: Option<VisibleString>,
    #[rasn(tag(4))]
    t: UtcTime,
    #[rasn(tag(5))]
    st_num: Integer,
    #[rasn(tag(6))]
    sq_num: Integer,
    #[rasn(tag(7))]
    #[rasn(default)]
    simulation: bool,
    #[rasn(tag(8))]
    conf_rev: Integer,
    #[rasn(tag(9))]
    #[rasn(default)]
    nds_com: bool,
    #[rasn(tag(10))]
    num_dat_set_entries: Integer,
    #[rasn(tag(11))]
    all_data: Vec<Data>,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(choice)]
enum Data {
    #[rasn(tag(1))]
    Array(DataSequence),
    #[rasn(tag(2))]
    Structure(DataSequence),
    #[rasn(tag(3))]
    Boolean(bool),
    #[rasn(tag(4))]
    BitString(BitString),
    #[rasn(tag(5))]
    Integer(Integer),
    #[rasn(tag(6))]
    Unsigned(Integer),
    #[rasn(tag(7))]
    FloatingPoint(FloatingPoint),
    #[rasn(tag(9))]
    OctetString(OctetString),
    #[rasn(tag(10))]
    VisibleString(VisibleString),
    #[rasn(tag(11))]
    GeneralizedTime(GeneralizedTime),
    #[rasn(tag(12))]
    BinaryTime(TimeOfDay),
    #[rasn(tag(13))]
    Bcd(Integer),
    #[rasn(tag(14))]
    BooleanArray(BitString),
    #[rasn(tag(15))]
    ObjId(ObjectIdentifier),
    #[rasn(tag(16))]
    MMSString(MMSString),
    #[rasn(tag(17))]
    UtcTime(UtcTime),
}

type DataSequence = Vec<Data>;

type FloatingPoint = OctetString;

type UtcTime = OctetString;

type MMSString = Utf8String;

type TimeOfDay = OctetString;

#[test]
fn it_works() {
    let data = IEC61850SpecificProtocol::GoosePdu(IECGoosePdu {
        gocb_ref: String::from("").try_into().unwrap(),
        time_allowed_to_live: 200.into(),
        dat_set: String::from("").try_into().unwrap(),
        go_id: Some(String::from("events").try_into().unwrap()),
        t: UtcTime::from(vec![]),
        st_num: 1.into(),
        sq_num: 1.into(),
        simulation: true,
        conf_rev: 1.into(),
        nds_com: false,
        num_dat_set_entries: 4.into(),
        all_data: vec![
            Data::Boolean(true),
            Data::Boolean(true),
            Data::Integer(200.into()),
        ],
    });

    let bin = rasn::ber::encode(&data).unwrap();
    assert_eq!(data, rasn::ber::decode(&bin).unwrap());
}
