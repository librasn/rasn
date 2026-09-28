//! Published X.691 (02/2021) 11.9.3.4, 19.8 and 19.9 vectors.
use rasn::prelude::*;
use rasn::types::{
    Constructed, Identifier,
    fields::{Field, Fields},
};

#[derive(AsnType, Debug, Decode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Root {
    a: bool,
}

#[derive(AsnType, Debug, Decode, PartialEq)]
#[rasn(set, automatic_tags)]
#[non_exhaustive]
struct RootSet {
    a: bool,
}

#[derive(AsnType, Debug, Decode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct One {
    a: bool,
    #[rasn(extension_addition)]
    b: Option<bool>,
}

#[derive(AsnType, Debug, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct Outer<T> {
    before: bool,
    inner: T,
    after: bool,
}

// Avoid duplicating 128 derived fields. This encodes exactly one root BOOLEAN
// and N optional BOOLEAN additions, with only the last addition present.
struct Additions<const N: usize>;
impl<const N: usize> AsnType for Additions<N> {
    const TAG: Tag = Tag::SEQUENCE;
}
impl<const N: usize> Constructed<1, N> for Additions<N> {
    const IS_EXTENSIBLE: bool = true;
    const FIELDS: Fields<1> = Fields::from_static([Field::new_required(
        0,
        Tag::new_context(0),
        TagTree::Leaf(Tag::new_context(0)),
        "a",
    )]);
    const EXTENDED_FIELDS: Option<Fields<N>> = Some({
        let mut fields =
            [Field::new_optional(0, Tag::BOOL, TagTree::Leaf(Tag::BOOL), "addition"); N];
        let mut i = 0;
        while i < N {
            let tag = Tag::new_context(i as u32 + 1);
            fields[i] = Field::new_optional(i, tag, TagTree::Leaf(tag), "addition");
            i += 1;
        }
        Fields::from_static(fields)
    });
}
impl<const N: usize> Encode for Additions<N> {
    fn encode_with_tag_and_constraints<'b, E: rasn::Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _: Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder.encode_sequence::<1, N, Self, _>(
            tag,
            |e| {
                true.encode_with_tag(e, Tag::new_context(0))?;
                for i in 0..N {
                    e.encode_extension_addition(
                        Tag::new_context(i as u32 + 1),
                        Constraints::NONE,
                        (i == N - 1).then_some(true),
                        Identifier::EMPTY,
                    )?;
                }
                Ok(())
            },
            identifier,
        )?;
        Ok(())
    }
}

// Outer before=true, root a=true, last addition=true, after=false.
// n<=64: 0 + (n-1) as six bits. n>64: 1 + unconstrained n determinant.
fn expected(aper: bool, n: usize) -> Vec<u8> {
    match (aper, n) {
        (true, 64) => vec![0xef, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0x40, 1, 0x80, 0],
        (false, 64) => vec![0xef, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0x60, 0],
        (true, 65) => vec![0xf0, 0x41, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 1, 0x80, 0],
        (false, 65) => vec![0xf4, 0x10, 0, 0, 0, 0, 0, 0, 0, 0x08, 0x0c, 0],
        (true, 128) => vec![
            0xf0, 0x80, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0x80, 0,
        ],
        (false, 128) => {
            let mut bytes = vec![0xf8, 0x08];
            bytes.extend_from_slice(&[0; 16]);
            bytes.extend_from_slice(&[0x10, 0x18, 0]);
            bytes
        }
        _ => unreachable!(),
    }
}

fn check_decode(n: usize) {
    for aper in [true, false] {
        let wire = expected(aper, n);
        let root: Outer<Root> = if aper {
            rasn::aper::decode(&wire)
        } else {
            rasn::uper::decode(&wire)
        }
        .unwrap();
        let set: Outer<RootSet> = if aper {
            rasn::aper::decode(&wire)
        } else {
            rasn::uper::decode(&wire)
        }
        .unwrap();
        let one: Outer<One> = if aper {
            rasn::aper::decode(&wire)
        } else {
            rasn::uper::decode(&wire)
        }
        .unwrap();
        assert_eq!(
            root,
            Outer {
                before: true,
                inner: Root { a: true },
                after: false
            }
        );
        assert_eq!(
            set,
            Outer {
                before: true,
                inner: RootSet { a: true },
                after: false
            }
        );
        assert_eq!(
            one,
            Outer {
                before: true,
                inner: One { a: true, b: None },
                after: false
            }
        );
    }
}

fn check_encode<const N: usize>() {
    let value = Outer {
        before: true,
        inner: Additions::<N>,
        after: false,
    };
    let mut valid = true;
    for aper in [true, false] {
        let actual = if aper {
            rasn::aper::encode(&value)
        } else {
            rasn::uper::encode(&value)
        }
        .unwrap();
        eprintln!(
            "APER={aper} n={N} actual={actual:02x?} expected={:02x?}",
            expected(aper, N)
        );
        valid &= actual == expected(aper, N);
    }
    assert!(valid, "n={N} encoding disagrees with X.691");
}

#[test]
fn decode_n64() {
    check_decode(64);
}
#[test]
fn decode_n65() {
    check_decode(65);
}
#[test]
fn decode_n128() {
    check_decode(128);
}
#[test]
fn encode_n64() {
    check_encode::<64>();
}
#[test]
fn encode_n65() {
    check_encode::<65>();
}
#[test]
fn encode_n128() {
    check_encode::<128>();
}

// A single unknown open type carries a 16K fragment, then a zero final length.
// Both valid cases keep the trailing outer BOOLEAN; shortened payloads fail.
#[test]
fn skip_fragmented_unknown_and_reject_truncation() {
    for aper in [true, false] {
        // before/ext/a = 111, small length = 0000000, bitmap = 1.
        let mut bits = vec![
            true, true, true, false, false, false, false, false, false, false, true,
        ];
        if aper {
            while !bits.len().is_multiple_of(8) {
                bits.push(false);
            }
        }
        let mut bytes = vec![0xc1];
        bytes.resize(1 + 16384, 0x80);
        bytes.push(0); // final determinant required even for exact 16K
        for byte in bytes {
            for shift in (0..8).rev() {
                bits.push(byte & (1 << shift) != 0);
            }
        }
        bits.push(true); // after
        while !bits.len().is_multiple_of(8) {
            bits.push(false);
        }
        let wire: Vec<u8> = bits
            .chunks(8)
            .map(|b| b.iter().fold(0, |v, &bit| (v << 1) | u8::from(bit)))
            .collect();
        let value: Outer<Root> = if aper {
            rasn::aper::decode(&wire)
        } else {
            rasn::uper::decode(&wire)
        }
        .unwrap();
        assert!(value.before && value.inner.a && value.after);
        for cut in [wire.len() / 2, wire.len() - 2] {
            let truncated: Result<Outer<Root>, _> = if aper {
                rasn::aper::decode(&wire[..cut])
            } else {
                rasn::uper::decode(&wire[..cut])
            };
            assert!(truncated.is_err(), "APER={aper} cut={cut}");
        }
    }
}

// Independently emit the large-form bitmap fragments, then a single open type.
fn fragmented_bitmap_wire(aper: bool, n: usize) -> Vec<u8> {
    fn byte(bits: &mut Vec<bool>, value: u8, aper: bool) {
        if aper {
            while !bits.len().is_multiple_of(8) {
                bits.push(false);
            }
        }
        for shift in (0..8).rev() {
            bits.push(value & (1 << shift) != 0);
        }
    }
    let mut bits = vec![true, true, true, true]; // before/ext/root/large
    let mut offset = 0;
    while n - offset >= 16384 {
        let blocks = ((n - offset) / 16384).min(4);
        byte(&mut bits, 0xc0 | blocks as u8, aper);
        for i in offset..offset + blocks * 16384 {
            bits.push(i == n - 1);
        }
        offset += blocks * 16384;
    }
    let tail = n - offset;
    if tail >= 128 {
        byte(&mut bits, 0x80 | (tail >> 8) as u8, aper);
    }
    byte(&mut bits, tail as u8, aper);
    for i in offset..n {
        bits.push(i == n - 1);
    }
    byte(&mut bits, 1, aper);
    byte(&mut bits, 0x80, aper);
    bits.push(false); // after
    while !bits.len().is_multiple_of(8) {
        bits.push(false);
    }
    bits.chunks(8)
        .map(|b| b.iter().fold(0, |v, &bit| (v << 1) | u8::from(bit)))
        .collect()
}

#[test]
fn decode_fragmented_extension_bitmaps() {
    for aper in [true, false] {
        for n in [16384, 16385, 65536, 65537] {
            let wire = fragmented_bitmap_wire(aper, n);
            let value: Outer<Root> = if aper {
                rasn::aper::decode(&wire)
            } else {
                rasn::uper::decode(&wire)
            }
            .unwrap();
            assert!(value.before && value.inner.a && !value.after);
            for cut in [wire.len() / 2, wire.len() - 2] {
                let truncated: Result<Outer<Root>, _> = if aper {
                    rasn::aper::decode(&wire[..cut])
                } else {
                    rasn::uper::decode(&wire[..cut])
                };
                assert!(truncated.is_err(), "APER={aper} n={n} cut={cut}");
            }
        }
    }
}

#[test]
fn encode_fragmented_extension_bitmap() {
    // This deliberately enormous schema needs room for its compile-time field arrays.
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let value = Outer {
                before: true,
                inner: Additions::<16384>,
                after: false,
            };
            for aper in [true, false] {
                let wire = if aper {
                    rasn::aper::encode(&value)
                } else {
                    rasn::uper::encode(&value)
                }
                .unwrap();
                assert_eq!(wire, fragmented_bitmap_wire(aper, 16384));
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
