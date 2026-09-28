use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(delegate, size("3"))]
struct FixedTeletex(TeletexString);

#[test]
fn teletex_preserves_t61_octets_in_per_and_ber() {
    // The T.61 non-spacing acute accent followed by 'e' remains encoded
    // octets, rather than a Unicode scalar or a known-multiplier index.
    let octets = [b'A', 0xc2, b'e'];
    let value = TeletexString::from_bytes(&octets).expect("T.61 octets");
    assert_eq!(value.to_bytes(), octets);
    let wire = [3, b'A', 0xc2, b'e'];
    assert_eq!(rasn::aper::encode(&value).unwrap(), wire);
    assert_eq!(rasn::uper::encode(&value).unwrap(), wire);
    assert_eq!(rasn::aper::decode::<TeletexString>(&wire).unwrap(), value);
    assert_eq!(rasn::uper::decode::<TeletexString>(&wire).unwrap(), value);
    assert_eq!(
        rasn::ber::encode(&value).unwrap(),
        [0x14, 3, b'A', 0xc2, b'e']
    );
    let fixed = FixedTeletex(value);
    assert_eq!(rasn::aper::encode(&fixed).unwrap(), wire);
    assert_eq!(rasn::uper::encode(&fixed).unwrap(), wire);
    assert_eq!(rasn::aper::decode::<FixedTeletex>(&wire).unwrap(), fixed);
    assert_eq!(rasn::uper::decode::<FixedTeletex>(&wire).unwrap(), fixed);
}
