use rasn::prelude::*;

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(tag(explicit(application, 1)), delegate)]
pub struct DelegateSequence(pub Sequence);

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
pub struct Sequence {
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
pub struct SequenceWithDefault {
    #[rasn(tag(explicit(application, 1)), default = "default_bool")]
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
pub struct SequenceWithMultiDefault {
    #[rasn(tag(explicit(application, 1)), default = "default_bool")]
    b: bool,
    #[rasn(tag(explicit(application, 2)), default = "default_bool")]
    b2: bool,
}
pub fn default_bool() -> bool {
    true
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(tag(explicit(application, 1)))]
pub struct InlineSequence {
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(tag(application, 1))]
pub struct SequenceField {
    #[rasn(tag(explicit(universal, 16)))]
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(set, tag(explicit(application, 1)))]
pub struct InlineSet {
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(set, tag(application, 1))]
pub struct SetField {
    #[rasn(tag(explicit(universal, 16)))]
    b: bool,
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(choice)]
pub enum DelegateChoice {
    #[rasn(tag(explicit(application, 1)))]
    Sequence(Sequence),
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(choice)]
pub enum InlineChoice {
    #[rasn(tag(explicit(application, 1)))]
    Sequence { b: bool },
}

#[derive(AsnType, Decode, Debug, Encode, PartialEq)]
#[rasn(choice, tag(explicit(application, 1)))]
pub enum WrappedChoice {
    Sequence { b: bool },
}

const _: () = assert!(Tag::const_eq(
    DelegateSequence::TAG,
    &Tag::new(Class::Application, 1)
));

const _: () = assert!(Tag::const_eq(DelegateSequence::TAG, &InlineSequence::TAG,));

const _: () = assert!(Tag::const_eq(DelegateSequence::TAG, &InlineSet::TAG,));

#[test]
fn works() {
    const EXPECTED: &[u8] = &[0x61, 0x5, 0x30, 0x3, 0x01, 0x1, 0xFF];
    // Note that the explicitly tagged field is dropped.
    // This makes it a sequence with 0 elements
    const EXPECTED_DEFAULT: &[u8] = &[0x30, 0x00];
    const EXPECTED_NOT_DEFAULT: &[u8] = &[0x30, 0x05, 0x61, 0x03, 0x01, 0x01, 0x00];
    // NOTE: The explicit tag number is just different
    const EXPECTED_MULTI_DEFAULT: &[u8] = &[0x30, 0x05, 0x62, 0x03, 0x01, 0x01, 0x00];

    let delegate_seq = DelegateSequence(Sequence { b: true });
    let inline_seq = InlineSequence { b: true };
    let field_seq = SequenceField { b: true };
    let field_set = SetField { b: true };
    let inline_set = InlineSet { b: true };
    let delegate_choice = DelegateChoice::Sequence(Sequence { b: true });
    let inline_choice = InlineChoice::Sequence { b: true };
    let wrapped_choice = WrappedChoice::Sequence { b: true };
    let delegate_seq_enc = rasn::der::encode(&delegate_seq).unwrap();
    let inline_seq_enc = rasn::der::encode(&inline_seq).unwrap();
    let field_seq_enc = rasn::der::encode(&field_seq).unwrap();
    let field_set_enc = rasn::der::encode(&field_set).unwrap();
    let inline_set_enc = rasn::der::encode(&inline_set).unwrap();
    let delegate_choice_enc = rasn::der::encode(&delegate_choice).unwrap();
    let inline_choice_enc = rasn::der::encode(&inline_choice).unwrap();
    let wrapped_choice_enc = rasn::der::encode(&wrapped_choice).unwrap();

    // Set the field to match the default value to have it dropped
    let sequence_default = SequenceWithDefault { b: true };
    let sequence_non_default = SequenceWithDefault { b: false };
    let sequence_default_enc = rasn::der::encode(&sequence_default).unwrap();
    let sequence_non_default_enc = rasn::der::encode(&sequence_non_default).unwrap();
    // Verify it correctly includes encoded fields
    let sequence_multi_default = SequenceWithMultiDefault { b: true, b2: false };
    let sequence_multi_default_enc = rasn::der::encode(&sequence_multi_default).unwrap();

    assert_eq!(sequence_non_default_enc, EXPECTED_NOT_DEFAULT);
    assert_eq!(sequence_default_enc, EXPECTED_DEFAULT);
    assert_eq!(sequence_multi_default_enc, EXPECTED_MULTI_DEFAULT);

    assert_eq!(delegate_seq_enc, EXPECTED);
    assert_eq!(inline_seq_enc, EXPECTED);
    assert_eq!(field_seq_enc, EXPECTED);
    assert_eq!(field_set_enc, EXPECTED);
    assert_eq!(inline_set_enc, EXPECTED);
    assert_eq!(delegate_choice_enc, EXPECTED);
    assert_eq!(inline_choice_enc, EXPECTED);
    assert_eq!(wrapped_choice_enc, EXPECTED);
    assert_eq!(delegate_seq, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(inline_seq, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(field_seq, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(field_set, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(inline_set, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(delegate_choice, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(inline_choice, rasn::der::decode(EXPECTED).unwrap());
    assert_eq!(wrapped_choice, rasn::der::decode(EXPECTED).unwrap());
}
