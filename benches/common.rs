use chrono::{FixedOffset, TimeZone, Utc};
use rasn::{types::*, AsnType, Decode, Encode};

#[derive(AsnType, Decode, Encode)]
pub struct Bench {
    a: bool,
    b: Integer,
    // c: Real,
    d: (),
    e: BitString,
    f: OctetString,
    g: ObjectIdentifier,
    h: BenchEnum,
    i: EmptySequence,
    j: Vec<()>,
    // k: Set
    l: SetOf<()>,
    m: BenchChoice,
    n: Utf8String,
    o: UtcTime,
    p: GeneralizedTime,
}

#[derive(AsnType, Decode, Encode)]
pub struct EmptySequence {}

#[derive(AsnType, Clone, Copy, Decode, Debug, Encode, PartialEq)]
#[rasn(enumerated)]
pub enum BenchEnum {
    A,
    B,
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum BenchChoice {
    A,
}

pub fn bench_default() -> Bench {
    Bench {
        a: true,
        b: 12345678.into(),
        // c: 3.14159,
        d: (),
        e: BitString::from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]),
        f: OctetString::from([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77].repeat(10)),
        g: ObjectIdentifier::new(vec![1, 4, 123, 4325, 23, 1, 44, 22222]).unwrap(),
        h: BenchEnum::B,
        i: EmptySequence {},
        j: vec![(); 5],
        // k: {},
        l: SetOf::new(),
        m: BenchChoice::A,
        n: "a".repeat(40),
        o: Utc.with_ymd_and_hms(2018, 6, 13, 11, 1, 59).unwrap(),
        p: FixedOffset::east_opt(5 * 3600)
            .unwrap()
            .with_ymd_and_hms(2018, 6, 13, 11, 1, 58)
            .unwrap(),
    }
}
