use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(delegate, from("a..=b"), size("1..=2", extensible))]
struct Letters(Ia5String);

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
struct Wrapped {
    before: bool,
    letters: Letters,
}

#[test]
fn size_extensions_retain_the_effective_permitted_alphabet() {
    let value = Wrapped {
        before: true,
        letters: Letters(Ia5String::try_from("aba").unwrap()),
    };
    let aper = [0xc0, 0x03, 0x40];
    let uper = [0xc0, 0xd0];
    assert_eq!(rasn::aper::encode(&value).unwrap(), aper);
    assert_eq!(rasn::uper::encode(&value).unwrap(), uper);
    assert_eq!(rasn::aper::decode::<Wrapped>(&aper).unwrap(), value);
    assert_eq!(rasn::uper::decode::<Wrapped>(&uper).unwrap(), value);
    let invalid = Wrapped {
        before: true,
        letters: Letters(Ia5String::try_from("ccc").unwrap()),
    };
    assert!(rasn::aper::encode(&invalid).is_err());
    assert!(rasn::uper::encode(&invalid).is_err());
}
