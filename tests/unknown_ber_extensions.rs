//! BER-family extension handling must preserve known fields and value boundaries.
use rasn::prelude::*;

#[derive(AsnType, Decode, Debug, PartialEq)]
#[non_exhaustive]
struct Sequence {
    #[rasn(tag(0))]
    a: u32,
    #[rasn(tag(1), extension_addition)]
    b: Option<u32>,
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[rasn(set)]
#[non_exhaustive]
struct Set {
    #[rasn(tag(0))]
    a: u32,
    #[rasn(tag(1), extension_addition)]
    b: Option<u32>,
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[non_exhaustive]
struct Empty {}

#[derive(AsnType, Decode, Debug, PartialEq)]
struct Outer<T> {
    inner: T,
    after: u32,
}

fn definite(tag: u8, contents: &[u8]) -> Vec<u8> {
    assert!(contents.len() < 128);
    let mut encoded = vec![tag, contents.len() as u8];
    encoded.extend_from_slice(contents);
    encoded
}

fn indefinite(tag: u8, contents: &[u8]) -> Vec<u8> {
    let mut encoded = vec![tag, 0x80];
    encoded.extend_from_slice(contents);
    encoded.extend_from_slice(&[0, 0]);
    encoded
}

#[test]
fn sequence_skips_multiple_unknown_additions_with_known_addition_present_or_absent() {
    for known in [false, true] {
        let mut contents = vec![0x80, 1, 1];
        if known {
            contents.extend_from_slice(&[0x81, 1, 2]);
        }
        contents.extend_from_slice(&[0x82, 1, 3, 0x83, 1, 4]);
        let expected = Sequence {
            a: 1,
            b: known.then_some(2),
        };
        assert_eq!(
            rasn::ber::decode::<Sequence>(&definite(0x30, &contents)).unwrap(),
            expected
        );
        assert_eq!(
            rasn::der::decode::<Sequence>(&definite(0x30, &contents)).unwrap(),
            expected
        );
        assert_eq!(
            rasn::cer::decode::<Sequence>(&indefinite(0x30, &contents)).unwrap(),
            expected
        );
    }
}

#[test]
fn empty_extensible_sequence_uses_default_initializer_and_skips_unknowns() {
    for contents in [&[][..], &[0x82, 1, 3][..]] {
        assert_eq!(
            rasn::ber::decode::<Empty>(&definite(0x30, contents)).unwrap(),
            Empty {}
        );
        assert_eq!(
            rasn::ber::decode::<Empty>(&indefinite(0x30, contents)).unwrap(),
            Empty {}
        );
        assert_eq!(
            rasn::der::decode::<Empty>(&definite(0x30, contents)).unwrap(),
            Empty {}
        );
        assert_eq!(
            rasn::cer::decode::<Empty>(&indefinite(0x30, contents)).unwrap(),
            Empty {}
        );
    }
}

#[test]
fn nested_unknown_indefinite_values_preserve_sibling_and_top_level_remainder() {
    // The unknown [2] contains an indefinite SEQUENCE, and its own EOC must
    // not be mistaken for the enclosing value's EOC.
    let contents = [0x80, 1, 1, 0xa2, 0x80, 0x30, 0x80, 2, 1, 7, 0, 0, 0, 0];
    for (tag, is_set) in [(0x30, false), (0x31, true)] {
        for nested in [definite(tag, &contents), indefinite(tag, &contents)] {
            let mut outer_contents = nested;
            outer_contents.extend_from_slice(&[2, 1, 9]);
            let mut input = definite(0x30, &outer_contents);
            input.extend_from_slice(&[2, 1, 42]);
            if is_set {
                let (value, remaining) =
                    rasn::ber::decode_with_remainder::<Outer<Set>>(&input).unwrap();
                assert_eq!(value.inner, Set { a: 1, b: None });
                assert_eq!(value.after, 9);
                assert_eq!(remaining, [2, 1, 42]);
            } else {
                let (value, remaining) =
                    rasn::ber::decode_with_remainder::<Outer<Sequence>>(&input).unwrap();
                assert_eq!(value.inner, Sequence { a: 1, b: None });
                assert_eq!(value.after, 9);
                assert_eq!(remaining, [2, 1, 42]);
            }
        }
    }
}

#[test]
fn definite_unknown_contents_are_opaque_like_any() {
    // The outer unknown TLV is complete; its payload is not recursively
    // interpreted without a schema (the same contract used by Any).
    let contents = [0x80, 1, 1, 0xa2, 2, 2, 1];
    assert_eq!(
        rasn::ber::decode::<Sequence>(&definite(0x30, &contents)).unwrap(),
        Sequence { a: 1, b: None }
    );
    assert_eq!(
        rasn::der::decode::<Sequence>(&definite(0x30, &contents)).unwrap(),
        Sequence { a: 1, b: None }
    );
    assert_eq!(
        rasn::cer::decode::<Sequence>(&indefinite(0x30, &contents)).unwrap(),
        Sequence { a: 1, b: None }
    );
}

#[test]
fn set_skips_unknown_members_in_every_position() {
    for contents in [
        &[0x82, 1, 3, 0x80, 1, 1, 0x81, 1, 2][..],
        &[0x80, 1, 1, 0x82, 1, 3, 0x81, 1, 2][..],
        &[0x80, 1, 1, 0x81, 1, 2, 0x82, 1, 3][..],
    ] {
        assert_eq!(
            rasn::ber::decode::<Set>(&definite(0x31, contents)).unwrap(),
            Set { a: 1, b: Some(2) }
        );
    }
}

#[derive(AsnType, Decode, Encode, Debug, PartialEq)]
#[rasn(choice)]
enum KnownChoice {
    #[rasn(tag(0))]
    Number(u32),
    #[rasn(tag(1))]
    Flag(bool),
}

#[derive(AsnType, Debug, PartialEq)]
#[rasn(set)]
#[non_exhaustive]
struct ChoiceSet {
    item: KnownChoice,
}

// Supply the member CHOICE directly so both its tag tree and its existing
// decoder participate in SET dispatch.
impl Decode for ChoiceSet {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: &rasn::types::Constraints,
    ) -> Result<Self, D::Error> {
        use rasn::de::Error;

        let codec = decoder.codec();
        decoder.decode_set::<1, 0, KnownChoice, Self, _, _>(
            tag,
            |decoder, _, _| KnownChoice::decode(decoder),
            |items| {
                if items.len() != 1 {
                    return Err(D::Error::custom("expected one CHOICE member", codec));
                }
                Ok(Self {
                    item: items.into_iter().next().unwrap(),
                })
            },
        )
    }
}

#[test]
fn set_matches_every_alternative_of_a_known_choice() {
    for (known, expected) in [
        ([0x80, 1, 7], KnownChoice::Number(7)),
        ([0x81, 1, 0xff], KnownChoice::Flag(true)),
    ] {
        assert_eq!(
            rasn::ber::decode::<ChoiceSet>(&definite(0x31, &known))
                .unwrap()
                .item,
            expected
        );
        let contents = [0x82, 1, 3, known[0], known[1], known[2]];
        assert_eq!(
            rasn::ber::decode::<ChoiceSet>(&definite(0x31, &contents)).unwrap(),
            ChoiceSet { item: expected }
        );
    }
}

#[derive(Debug, PartialEq)]
struct ConsumingMember(u32);

impl AsnType for ConsumingMember {
    const TAG: Tag = Tag::INTEGER;
}

impl Decode for ConsumingMember {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        _: Tag,
        _: &rasn::types::Constraints,
    ) -> Result<Self, D::Error> {
        use rasn::de::Error;

        // A custom decoder may consume a value before deciding whether its
        // tag or contents are valid. Error handling must preserve that value's
        // original boundary and distinguish its tag from the following one.
        let codec = decoder.codec();
        let encoded = Any::decode(decoder)?;
        match encoded.as_bytes() {
            [2, 1, number] => Ok(Self(u32::from(*number))),
            [2, ..] => Err(D::Error::custom("invalid known SET member", codec)),
            _ => Err(D::Error::custom("unrecognized SET member", codec)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ConsumingSet(u32);

impl AsnType for ConsumingSet {
    const TAG: Tag = Tag::SET;
}

impl rasn::types::Constructed<1, 0> for ConsumingSet {
    const IS_EXTENSIBLE: bool = true;
    const FIELDS: rasn::types::fields::Fields<1> = rasn::types::fields::Fields::from_static([
        rasn::types::fields::Field::new_required_type::<ConsumingMember>(0, "number"),
    ]);
}

impl Decode for ConsumingSet {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: &rasn::types::Constraints,
    ) -> Result<Self, D::Error> {
        use rasn::de::Error;

        let codec = decoder.codec();
        decoder.decode_set::<1, 0, ConsumingMember, Self, _, _>(
            tag,
            |decoder, _, _| ConsumingMember::decode(decoder),
            |items| {
                if items.len() != 1 {
                    return Err(D::Error::custom("expected one INTEGER member", codec));
                }
                Ok(Self(items.into_iter().next().unwrap().0))
            },
        )
    }
}

#[test]
fn consuming_unknown_set_member_errors_preserve_known_members_and_boundaries() {
    for unknown in [vec![0x82, 1, 3], indefinite(0xa2, &[2, 1, 3])] {
        let mut contents = unknown.clone();
        contents.extend_from_slice(&[2, 1, 7]);
        contents.extend_from_slice(&unknown);
        for inner in [definite(0x31, &contents), indefinite(0x31, &contents)] {
            let mut outer_contents = inner;
            outer_contents.extend_from_slice(&[2, 1, 9]);
            let mut input = definite(0x30, &outer_contents);
            input.extend_from_slice(&[2, 1, 42]);
            let (value, remaining) =
                rasn::ber::decode_with_remainder::<Outer<ConsumingSet>>(&input).unwrap();
            assert_eq!(value.inner, ConsumingSet(7));
            assert_eq!(value.after, 9);
            assert_eq!(remaining, [2, 1, 42]);
        }
    }
}

#[test]
fn consuming_known_set_member_errors_are_propagated_unchanged() {
    // The malformed known INTEGER is followed by an unknown member and a
    // valid INTEGER. Neither may conceal the original decoder error.
    let contents = [2, 0, 0x82, 1, 3, 2, 1, 7];
    for input in [definite(0x31, &contents), indefinite(0x31, &contents)] {
        let error = rasn::ber::decode::<ConsumingSet>(&input).unwrap_err();
        assert!(matches!(
            *error.kind,
            rasn::error::DecodeErrorKind::Custom { ref msg }
                if msg == "invalid known SET member"
        ));
    }
}

#[test]
fn unknown_indefinite_nesting_obeys_the_enclosing_decoders_depth_budget() {
    // The default depth budget is 128; the enclosing SEQUENCE or SET uses
    // one level before handing its remaining budget to the skip operation.
    for depth in [127, 128] {
        let mut unknown = vec![2, 1, 7];
        for _ in 0..depth {
            unknown = indefinite(0xa2, &unknown);
        }
        let mut contents = vec![0x80, 1, 1];
        contents.extend_from_slice(&unknown);
        let results = [
            rasn::ber::decode::<Sequence>(&indefinite(0x30, &contents)).map(|_| ()),
            rasn::ber::decode::<Set>(&indefinite(0x31, &contents)).map(|_| ()),
        ];
        for result in results {
            if depth == 127 {
                result.unwrap();
            } else {
                assert!(matches!(
                    *result.unwrap_err().kind,
                    rasn::error::DecodeErrorKind::ExceedsMaxParseDepth
                ));
            }
        }
    }
}

// Open fields cannot be classified by a finite tag tree. A handwritten
// decoder must retain control rather than having all its values skipped.
#[derive(Debug, PartialEq)]
struct OpenSet(Vec<Any>);

impl AsnType for OpenSet {
    const TAG: Tag = Tag::SET;
}

impl rasn::types::Constructed<1, 0> for OpenSet {
    const IS_EXTENSIBLE: bool = true;
    const FIELDS: rasn::types::fields::Fields<1> = rasn::types::fields::Fields::from_static([
        rasn::types::fields::Field::new_required_type::<Any>(0, "values"),
    ]);
}

impl Decode for OpenSet {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: &rasn::types::Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_set::<1, 0, Any, Self, _, _>(
            tag,
            |decoder, _, _| Any::decode(decoder),
            |values| Ok(Self(values)),
        )
    }
}

#[test]
fn set_with_an_empty_tag_tree_keeps_its_existing_field_decoder() {
    let input = [0x31, 6, 0x80, 1, 1, 0x82, 1, 3];
    assert_eq!(
        rasn::ber::decode::<OpenSet>(&input).unwrap(),
        OpenSet(vec![Any::new(vec![0x80, 1, 1]), Any::new(vec![0x82, 1, 3]),])
    );
}

#[derive(Debug, PartialEq)]
struct NestedOpenSet(Vec<Any>);

impl AsnType for NestedOpenSet {
    const TAG: Tag = Tag::SET;
}

impl rasn::types::Constructed<1, 0> for NestedOpenSet {
    const IS_EXTENSIBLE: bool = true;
    const FIELDS: rasn::types::fields::Fields<1> =
        rasn::types::fields::Fields::from_static([rasn::types::fields::Field::new_required(
            0,
            Tag::EOC,
            rasn::types::TagTree::Choice(&[
                rasn::types::TagTree::Leaf(Tag::INTEGER),
                rasn::types::TagTree::Choice(&[]),
            ]),
            "values",
        )]);
}

impl Decode for NestedOpenSet {
    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
        decoder: &mut D,
        tag: Tag,
        _: &rasn::types::Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_set::<1, 0, Any, Self, _, _>(
            tag,
            |decoder, _, _| Any::decode(decoder),
            |values| Ok(Self(values)),
        )
    }
}

#[test]
fn set_with_a_nested_empty_tag_tree_keeps_its_existing_field_decoder() {
    let input = [0x31, 6, 2, 1, 1, 0x82, 1, 3];
    assert_eq!(
        rasn::ber::decode::<NestedOpenSet>(&input).unwrap(),
        NestedOpenSet(vec![Any::new(vec![2, 1, 1]), Any::new(vec![0x82, 1, 3])])
    );
}

#[test]
fn malformed_unknown_frames_are_rejected() {
    for unknown in [
        &[0x82, 2, 3][..],                            // Truncated payload.
        &[0x82][..],                                  // Missing length.
        &[0x82, 0x82, 1][..],                         // Truncated long-form length.
        &[0xa2, 0x80, 2, 1, 7][..],                   // Missing EOC.
        &[0xa2, 0x80, 0x30, 0x80, 2, 1, 7, 0, 0][..], // Missing outer EOC.
        &[0x82, 0x80, 0, 0][..],                      // Indefinite primitive.
    ] {
        let mut contents = vec![0x80, 1, 1];
        contents.extend_from_slice(unknown);
        assert!(rasn::ber::decode::<Sequence>(&definite(0x30, &contents)).is_err());
        assert!(rasn::ber::decode::<Set>(&definite(0x31, &contents)).is_err());
        assert!(rasn::ber::decode::<Sequence>(&indefinite(0x30, &contents)).is_err());
        assert!(rasn::ber::decode::<Set>(&indefinite(0x31, &contents)).is_err());
    }
}

#[test]
fn der_rejects_indefinite_unknown_additions() {
    let contents = [0x80, 1, 1, 0xa2, 0x80, 2, 1, 7, 0, 0];
    assert!(rasn::der::decode::<Sequence>(&definite(0x30, &contents)).is_err());
    assert!(rasn::der::decode::<Set>(&definite(0x31, &contents)).is_err());
}

#[derive(AsnType, Decode, Debug, PartialEq)]
struct ClosedSequence {
    #[rasn(tag(0))]
    a: u32,
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[rasn(set)]
struct ClosedSet {
    #[rasn(tag(0))]
    a: u32,
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[non_exhaustive]
struct MiddleExtensions {
    #[rasn(tag(0))]
    a: u32,
    #[rasn(tag(1), extension_addition)]
    b: Option<u32>,
    #[rasn(tag(3))]
    c: Option<u32>,
}

#[test]
fn nonextensible_values_and_middle_extension_regions_remain_strict() {
    let contents = [0x80, 1, 1, 0x82, 1, 3];
    assert!(rasn::ber::decode::<ClosedSequence>(&definite(0x30, &contents)).is_err());
    assert!(rasn::ber::decode::<ClosedSet>(&definite(0x31, &contents)).is_err());
    // Skipping the tail here would silently discard the known optional c.
    let contents = [0x80, 1, 1, 0x81, 1, 2, 0x82, 1, 3, 0x83, 1, 4];
    assert!(rasn::ber::decode::<MiddleExtensions>(&definite(0x30, &contents)).is_err());
}

#[test]
fn duplicate_known_set_members_and_missing_required_roots_remain_errors() {
    for contents in [
        &[0x80, 1, 1, 0x82, 1, 3, 0x80, 1, 1][..],
        &[0x80, 1, 1, 0x81, 1, 2, 0x82, 1, 3, 0x81, 1, 2][..],
        &[0x82, 1, 3][..],
    ] {
        assert!(rasn::ber::decode::<Set>(&definite(0x31, contents)).is_err());
    }
    assert!(rasn::ber::decode::<Sequence>(&[0x30, 3, 0x82, 1, 3]).is_err());
}

#[test]
fn malformed_known_members_are_never_treated_as_unknown() {
    for contents in [
        &[0x80, 0, 0x82, 1, 3][..],
        &[0x80, 1, 1, 0x81, 0, 0x82, 1, 3][..],
    ] {
        assert!(rasn::ber::decode::<Sequence>(&definite(0x30, contents)).is_err());
        assert!(rasn::ber::decode::<Set>(&definite(0x31, contents)).is_err());
        assert!(rasn::der::decode::<Sequence>(&definite(0x30, contents)).is_err());
        assert!(rasn::der::decode::<Set>(&definite(0x31, contents)).is_err());
        assert!(rasn::cer::decode::<Sequence>(&indefinite(0x30, contents)).is_err());
        assert!(rasn::cer::decode::<Set>(&indefinite(0x31, contents)).is_err());
    }
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[non_exhaustive]
struct ExplicitAddition {
    #[rasn(tag(0))]
    a: u32,
    #[rasn(tag(explicit(1)), extension_addition)]
    b: Option<u32>,
}

#[test]
fn malformed_known_explicit_additions_cannot_be_hidden_by_unknown_additions() {
    for bad_known in [
        &[0xa1, 3, 1, 1, 0xff][..],
        &[0xa1, 0x80, 1, 1, 0xff, 0, 0][..],
    ] {
        let mut contents = vec![0x80, 1, 1];
        contents.extend_from_slice(bad_known);
        contents.extend_from_slice(&[0x82, 1, 3]);
        assert!(rasn::ber::decode::<ExplicitAddition>(&definite(0x30, &contents)).is_err());
        assert!(rasn::ber::decode::<ExplicitAddition>(&indefinite(0x30, &contents)).is_err());
    }
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[rasn(delegate, tag(explicit(4)))]
struct ExplicitValue(u32);

#[test]
fn explicit_wrappers_remain_strict() {
    assert!(rasn::ber::decode::<ExplicitValue>(&[0xa4, 6, 2, 1, 1, 0x82, 1, 3]).is_err());
    assert!(rasn::ber::decode::<ExplicitValue>(&[0xa4, 0x80, 2, 1, 1, 0x82, 1, 3, 0, 0,]).is_err());
}
