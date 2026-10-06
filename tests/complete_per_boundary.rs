use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
struct Nested {
    before: bool,
    empty: (),
    after: bool,
}

#[test]
fn complete_empty_values_consume_one_octet_and_preserve_stream_remainders() {
    for (encode, decode, remainder) in [
        (
            rasn::aper::encode::<()> as fn(&()) -> Result<Vec<u8>, rasn::error::EncodeError>,
            rasn::aper::decode::<bool> as fn(&[u8]) -> Result<bool, rasn::error::DecodeError>,
            rasn::aper::decode_with_remainder::<()>
                as fn(&[u8]) -> Result<((), &[u8]), rasn::error::DecodeError>,
        ),
        (
            rasn::uper::encode::<()>,
            rasn::uper::decode::<bool>,
            rasn::uper::decode_with_remainder::<()>,
        ),
    ] {
        let mut stream = encode(&()).unwrap();
        assert_eq!(stream, [0]);
        assert!(remainder(&stream).unwrap().1.is_empty());
        stream.push(0x80);
        let (_, rest) = remainder(&stream).unwrap();
        assert_eq!(rest, [0x80]);
        assert!(decode(rest).unwrap());
    }
}

#[test]
fn complete_decoding_checks_empty_octets_and_trailing_data() {
    for decode in [rasn::aper::decode::<()>, rasn::uper::decode::<()>] {
        assert!(decode(&[]).is_err());
        assert!(decode(&[1]).is_err());
        assert!(decode(&[0, 0]).is_err());
        assert!(decode(&[0]).is_ok());
    }
    for decode in [rasn::aper::decode::<bool>, rasn::uper::decode::<bool>] {
        assert!(decode(&[0, 0xff]).is_err());
        assert!(!decode(&[1]).unwrap());
        assert!(decode(&[0xff]).unwrap());
        assert!(!decode(&[0]).unwrap());
        assert!(decode(&[0x80]).unwrap());
    }
    let value = Nested {
        before: true,
        empty: (),
        after: true,
    };
    assert_eq!(rasn::aper::encode(&value).unwrap(), [0xc0]);
    assert_eq!(rasn::aper::decode::<Nested>(&[0xc0]).unwrap(), value);
    assert_eq!(rasn::uper::encode(&value).unwrap(), [0xc0]);
    assert_eq!(rasn::uper::decode::<Nested>(&[0xc0]).unwrap(), value);
}

#[test]
fn constrained_decoding_checks_complete_encoding_boundaries() {
    for decode in [
        rasn::aper::decode_with_constraints::<()>,
        rasn::uper::decode_with_constraints::<()>,
    ] {
        for wire in [&[][..], &[1][..], &[0, 0][..]] {
            assert!(decode(&Constraints::default(), wire).is_err(), "{wire:?}");
        }
        assert!(decode(&Constraints::default(), &[0]).is_ok());
    }
    for decode in [
        rasn::aper::decode_with_constraints::<bool>,
        rasn::uper::decode_with_constraints::<bool>,
    ] {
        assert!(!decode(&Constraints::default(), &[1]).unwrap());
        assert!(decode(&Constraints::default(), &[0xff]).unwrap());
        assert!(decode(&Constraints::default(), &[0, 0xff]).is_err());
        assert!(decode(&Constraints::default(), &[0x80]).unwrap());
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct WithNull {
    root: bool,
    #[rasn(extension_addition)]
    value: Option<()>,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
struct NullGroup {
    value: (),
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct WithGroup {
    root: bool,
    #[rasn(extension_addition_group)]
    value: Option<NullGroup>,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct WithBool {
    root: bool,
    #[rasn(extension_addition)]
    value: Option<bool>,
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
enum Choice {
    Root(bool),
    #[rasn(extension_addition)]
    Null(()),
}

fn extension_wire(aligned: bool, contents: &[u8]) -> Vec<u8> {
    // Extension bit, TRUE root field, normally small bitmap length of one,
    // presence bit, then the unconstrained open-type length and contents.
    let mut bits = vec![
        true, true, false, false, false, false, false, false, false, true,
    ];
    if aligned {
        bits.resize(16, false);
    }
    for octet in core::iter::once(contents.len() as u8).chain(contents.iter().copied()) {
        bits.extend((0..8).rev().map(|shift| octet & (1 << shift) != 0));
    }
    let mut wire = vec![0; bits.len().div_ceil(8)];
    for (index, bit) in bits.into_iter().enumerate() {
        if bit {
            wire[index / 8] |= 0x80 >> (index % 8);
        }
    }
    wire
}

#[test]
fn extension_open_types_check_complete_encoding_boundaries() {
    for aligned in [true, false] {
        for contents in [&[][..], &[1][..], &[0, 0][..]] {
            let wire = extension_wire(aligned, contents);
            let result = if aligned {
                rasn::aper::decode::<WithNull>(&wire)
            } else {
                rasn::uper::decode::<WithNull>(&wire)
            };
            assert!(result.is_err(), "aligned={aligned}, contents={contents:?}");
            let result = if aligned {
                rasn::aper::decode::<WithGroup>(&wire)
            } else {
                rasn::uper::decode::<WithGroup>(&wire)
            };
            assert!(
                result.is_err(),
                "group aligned={aligned}, contents={contents:?}"
            );
        }
        let wire = extension_wire(aligned, &[0]);
        let value = WithNull {
            root: true,
            value: Some(()),
        };
        if aligned {
            assert_eq!(rasn::aper::decode::<WithNull>(&wire).unwrap(), value);
            assert_eq!(
                rasn::aper::decode::<WithGroup>(&wire).unwrap(),
                WithGroup {
                    root: true,
                    value: Some(NullGroup { value: () })
                },
            );
        } else {
            assert_eq!(rasn::uper::decode::<WithNull>(&wire).unwrap(), value);
            assert_eq!(
                rasn::uper::decode::<WithGroup>(&wire).unwrap(),
                WithGroup {
                    root: true,
                    value: Some(NullGroup { value: () })
                },
            );
        }
        let wire = extension_wire(aligned, &[0xff]);
        let boolean = if aligned {
            rasn::aper::decode::<WithBool>(&wire)
        } else {
            rasn::uper::decode::<WithBool>(&wire)
        }
        .unwrap();
        assert_eq!(boolean.value, Some(true));
        for contents in [&[][..], &[0, 0xff][..]] {
            let wire = extension_wire(aligned, contents);
            let result = if aligned {
                rasn::aper::decode::<WithBool>(&wire)
            } else {
                rasn::uper::decode::<WithBool>(&wire)
            };
            assert!(
                result.is_err(),
                "BOOL aligned={aligned}, contents={contents:?}"
            );
        }
    }
    for decode in [rasn::aper::decode::<Choice>, rasn::uper::decode::<Choice>] {
        assert_eq!(decode(&[0x80, 1, 0]).unwrap(), Choice::Null(()));
        for wire in [&[0x80, 0][..], &[0x80, 1, 1][..], &[0x80, 2, 0, 0][..]] {
            assert!(decode(wire).is_err(), "{wire:?}");
        }
    }
}
