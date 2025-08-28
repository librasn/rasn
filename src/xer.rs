//! XML Encoding Rules.

pub mod de;
pub mod enc;

const BOOLEAN_TRUE_TAG: &str = "true";
const BOOLEAN_FALSE_TAG: &str = "false";
const PLUS_INFINITY_TAG: &str = "PLUS-INFINITY";
const MINUS_INFINITY_TAG: &str = "MINUS-INFINITY";
const NAN_TAG: &str = "NOT-A-NUMBER";
const PLUS_INFINITY_VALUE: &str = "INF";
const MINUS_INFINITY_VALUE: &str = "-INF";
const NAN_VALUE: &str = "NaN";

/// Attempts to decode `T` from `input` using XER.
/// # Errors
/// Returns error specific to XER decoder if decoding is not possible.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut de::Decoder::new(input)?)
}

/// Attempts to encode `value` to XER.
/// # Errors
/// Returns error specific to XER encoder if encoding is not possible.
pub fn encode<T: crate::Encode>(
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut encoder = enc::Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.finish())
}

#[cfg(test)]
mod tests {
    use core::f64;

    use bitvec::bitvec;
    use bitvec::order::Msb0;

    use crate::prelude::*;
    use crate::xer::{decode, encode};

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    #[non_exhaustive]
    struct NestedTestA {
        wine: bool,
        grappa: OctetString,
        inner: InnerTestA,
        #[rasn(extension_addition)]
        oid: Option<ObjectIdentifier>,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct InnerTestA {
        hidden: Option<bool>,
    }

    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate", identifier = "Enum-Sequence")]
    struct EnumSequence {
        #[rasn(identifier = "enum-field")]
        enum_field: EnumType,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate", identifier = "Deep-Sequence")]
    struct DeepSequence {
        nested: NestedTestA,
        recursion: RecursiveChoice,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(choice, crate_root = "crate")]
    enum RecursiveChoice {
        Leaf,
        Fruit(bool),
        Recursion(Box<RecursiveChoice>),
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct SequenceWithChoice {
        recursion: RecursiveChoice,
        nested: NestedTestA,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq, Copy, Clone, Eq, Hash)]
    #[rasn(enumerated, automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum EnumType {
        #[rasn(identifier = "eins")]
        First,
        #[rasn(identifier = "zwei")]
        Second,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[rasn(crate_root = "crate")]
    enum ChoiceType {
        #[rasn(identifier = "enum")]
        EnumVariant(EnumType),
        #[allow(non_camel_case_types)]
        nested(InnerTestA),
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags, delegate)]
    #[rasn(crate_root = "crate")]
    struct ChoiceDelegate(pub ChoiceType);

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    struct DefaultSequence {
        #[rasn(identifier = "bool-df", default = "bool_default")]
        bool_with_default: bool,
        recursion: Vec<DefaultSequence>,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    pub struct SequenceWithSequenceOf {
        ids: SequenceOfIntegers,
        flag: bool,
        int: Integer,
        enum_val: EnumType,
    }

    #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
    #[rasn(automatic_tags)]
    #[rasn(crate_root = "crate")]
    pub struct SequenceWithSetOf {
        ids: SetOfIntegers,
        flag: bool,
        int: Integer,
        enum_val: EnumType,
    }

    fn bool_default() -> bool {
        bool::default()
    }

    type SequenceOfChoices = Vec<ChoiceType>;
    type SequenceOfDelegateChoices = Vec<ChoiceDelegate>;
    type SequenceOfEnumSequences = Vec<EnumSequence>;
    type SequenceOfBitStrings = Vec<BitString>;
    type SequenceOfEnums = Vec<EnumType>;
    type SequenceOfNulls = Vec<()>;
    type SequenceOfIntegers = Vec<i32>;
    type SequenceOfSequenceOfSequences = Vec<Vec<InnerTestA>>;

    type SetOfEnumSequences = SetOf<EnumSequence>;
    type SetOfEnums = SetOf<EnumType>;
    type SetOfBools = SetOf<bool>;
    type SetOfIntegers = SetOf<i32>;
    type SetOfSequenceOfSequences = SetOf<Vec<InnerTestA>>;

    macro_rules! round_trip {
        ($test_name:ident, $type:ident, $value:expr, $expected_ty:literal, $expected_val:literal) => {
            #[test]
            fn $test_name() {
                #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
                #[rasn(automatic_tags, delegate)]
                #[rasn(crate_root = "crate")]
                struct DelegateType(pub $type);

                #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
                #[rasn(automatic_tags, delegate)]
                #[rasn(crate_root = "crate")]
                struct NestedDelegateType(pub DelegateType);

                #[derive(AsnType, Debug, Encode, Decode, PartialEq)]
                #[rasn(automatic_tags, delegate, identifier = "Alias")]
                #[rasn(crate_root = "crate")]
                struct AliasDelegateType(pub $type);

                let input = $value;
                let encoded = String::from_utf8(encode(&input).unwrap()).unwrap();
                let expected = String::from("<")
                    + $expected_ty
                    + ">"
                    + $expected_val
                    + "</"
                    + $expected_ty
                    + ">";
                let decoded = decode::<$type>(expected.as_bytes()).unwrap();
                assert_eq!(input, decoded);
                assert_eq!(encoded, expected);

                let input = DelegateType($value);
                let encoded = String::from_utf8(encode(&input).unwrap()).unwrap();
                let expected = String::from("<DelegateType>") + $expected_val + "</DelegateType>";
                let decoded = decode::<DelegateType>(expected.as_bytes()).unwrap();
                assert_eq!(input, decoded);
                assert_eq!(encoded, expected);

                let input = NestedDelegateType(DelegateType($value));
                let encoded = String::from_utf8(encode(&input).unwrap()).unwrap();
                let expected =
                    String::from("<NestedDelegateType>") + $expected_val + "</NestedDelegateType>";
                let decoded = decode::<NestedDelegateType>(expected.as_bytes()).unwrap();
                assert_eq!(input, decoded);
                assert_eq!(encoded, expected);

                let input = AliasDelegateType($value);
                let encoded = String::from_utf8(encode(&input).unwrap()).unwrap();
                let expected = String::from("<Alias>") + $expected_val + "</Alias>";
                let decoded = decode::<AliasDelegateType>(expected.as_bytes()).unwrap();
                assert_eq!(input, decoded);
                assert_eq!(encoded, expected);
            }
        };
    }

    round_trip!(boolean_true, bool, true, "BOOLEAN", "<true />");
    round_trip!(boolean_false, bool, false, "BOOLEAN", "<false />");
    round_trip!(integer_sml, Integer, Integer::from(1), "INTEGER", "1");
    round_trip!(integer_neg, Integer, Integer::from(-2), "INTEGER", "-2");
    round_trip!(integer_u8, u8, 212, "INTEGER", "212");
    round_trip!(
        integer_i64,
        i64,
        -2_141_247_653_269_i64,
        "INTEGER",
        "-2141247653269"
    );
    round_trip!(positive_real, f64, 1.1234, "REAL", "1.1234");
    round_trip!(negative_real, f64, -1.1234, "REAL", "-1.1234");
    round_trip!(
        empty_element_infinity,
        f64,
        f64::INFINITY,
        "REAL",
        "<PLUS-INFINITY />"
    );
    round_trip!(
        empty_element_neg_infinity,
        f64,
        f64::NEG_INFINITY,
        "REAL",
        "<MINUS-INFINITY />"
    );
    round_trip!(
        bit_string,
        BitString,
        bitvec![u8, Msb0; 1,0,1,1,0,0,1],
        "BIT_STRING",
        "1011001"
    );
    round_trip!(
        octet_string,
        OctetString,
        OctetString::from([255u8, 0, 8, 10].to_vec()),
        "OCTET_STRING",
        "FF00080A"
    );
    round_trip!(
        ia5_string,
        Ia5String,
        Ia5String::from_iso646_bytes(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35]).unwrap(),
        "IA5String",
        "012345"
    );
    round_trip!(
        numeric_string,
        NumericString,
        NumericString::from_bytes(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35]).unwrap(),
        "NumericString",
        "012345"
    );
    round_trip!(
        utf8_string,
        Utf8String,
        "012345".to_string(),
        "UTF8String",
        "012345"
    );
    round_trip!(
        object_identifier,
        ObjectIdentifier,
        ObjectIdentifier::from(Oid::const_new(&[1, 654, 2, 1])),
        "OBJECT_IDENTIFIER",
        "1.654.2.1"
    );
    round_trip!(
        sequence,
        InnerTestA,
        InnerTestA {
            hidden: Some(false)
        },
        "InnerTestA",
        "<hidden><false /></hidden>"
    );
    round_trip!(
        enumerated,
        EnumType,
        EnumType::First,
        "EnumType",
        "<eins />"
    );
    round_trip!(
        choice,
        ChoiceType,
        ChoiceType::nested(InnerTestA { hidden: Some(true) }),
        "ChoiceType",
        "<nested><hidden><true /></hidden></nested>"
    );
    round_trip!(
        choice_with_none_value,
        ChoiceType,
        ChoiceType::nested(InnerTestA { hidden: None }),
        "ChoiceType",
        "<nested />"
    );
    round_trip!(
        enum_in_choice,
        ChoiceType,
        ChoiceType::EnumVariant(EnumType::Second),
        "ChoiceType",
        "<enum><zwei /></enum>"
    );
    round_trip!(
        choice_recursion,
        RecursiveChoice,
        RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Fruit(true))))))))),
        "RecursiveChoice",
        "<Recursion><Recursion><Recursion><Recursion><Fruit><true /></Fruit></Recursion></Recursion></Recursion></Recursion>"
    );
    round_trip!(
        recursive_choice_eventually_empty,
        RecursiveChoice,
        RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Recursion(Box::new(RecursiveChoice::Leaf)))))))),
        "RecursiveChoice",
        "<Recursion><Recursion><Recursion><Recursion><Leaf /></Recursion></Recursion></Recursion></Recursion>"
    );
    round_trip!(
        deep_sequence,
        DeepSequence,
        DeepSequence { nested: NestedTestA { wine: true, grappa: vec![0, 1, 2, 3].into(), inner: InnerTestA { hidden: Some(false) }, oid: None }, recursion: RecursiveChoice::Leaf },
        "Deep-Sequence",
        "<nested><wine><true /></wine><grappa>00010203</grappa><inner><hidden><false /></hidden></inner></nested><recursion><Leaf /></recursion>"
    );
    round_trip!(
        extended_sequence,
        NestedTestA,
        NestedTestA {
            wine: true,
            grappa: vec![0, 1, 2, 3].into(),
            inner: InnerTestA {
                hidden: Some(false),
            },
            oid: Some(ObjectIdentifier::from(Oid::const_new(&[1, 8270, 4, 1]))),
        },
        "NestedTestA",
        "<wine><true /></wine><grappa>00010203</grappa><inner><hidden><false /></hidden></inner><oid>1.8270.4.1</oid>"
    );
    round_trip!(
        sequence_with_defaults,
        DefaultSequence,
        DefaultSequence { bool_with_default: false, recursion: vec![DefaultSequence { bool_with_default: true, recursion: vec![] }] },
        "DefaultSequence",
        "<recursion><DefaultSequence><bool-df><true /></bool-df><recursion /></DefaultSequence></recursion>"
    );
    round_trip!(
        extended_sequence_with_inner_none,
        NestedTestA,
        NestedTestA {
            wine: true,
            grappa: vec![0, 1, 2, 3].into(),
            inner: InnerTestA { hidden: None },
            oid: Some(ObjectIdentifier::from(Oid::const_new(&[1, 8270, 4, 1])))
        },
        "NestedTestA",
        "<wine><true /></wine><grappa>00010203</grappa><inner /><oid>1.8270.4.1</oid>"
    );
    round_trip!(
        extensible_sequence_without_extensions,
        NestedTestA,
        NestedTestA {
            wine: true,
            grappa: vec![0, 1, 2, 3].into(),
            inner: InnerTestA { hidden: None },
            oid: None
        },
        "NestedTestA",
        "<wine><true /></wine><grappa>00010203</grappa><inner />"
    );
    round_trip!(
        sequence_of_nulls,
        SequenceOfNulls,
        vec![(), (), ()],
        "SEQUENCE_OF",
        "<NULL /><NULL /><NULL />"
    );
    round_trip!(
        sequence_of_choices,
        SequenceOfChoices,
        vec![
            ChoiceType::EnumVariant(EnumType::First),
            ChoiceType::EnumVariant(EnumType::Second)
        ],
        "SEQUENCE_OF",
        "<enum><eins /></enum><enum><zwei /></enum>"
    );
    round_trip!(
        sequence_of_delegate_choices,
        SequenceOfDelegateChoices,
        vec![
            ChoiceDelegate(ChoiceType::EnumVariant(EnumType::First)),
            ChoiceDelegate(ChoiceType::EnumVariant(EnumType::Second))
        ],
        "SEQUENCE_OF",
        "<enum><eins /></enum><enum><zwei /></enum>"
    );
    round_trip!(
        sequence_of_enums,
        SequenceOfEnums,
        vec![EnumType::First, EnumType::Second],
        "SEQUENCE_OF",
        "<eins /><zwei />"
    );
    round_trip!(
        sequence_of_bitstrings,
        SequenceOfBitStrings,
        vec![bitvec![u8, Msb0; 1,0,1,1,0,0,1]],
        "SEQUENCE_OF",
        "<BIT_STRING>1011001</BIT_STRING>"
    );
    round_trip!(
        sequence_of_integers,
        SequenceOfIntegers,
        vec![-1, 2, 3],
        "SEQUENCE_OF",
        "<INTEGER>-1</INTEGER><INTEGER>2</INTEGER><INTEGER>3</INTEGER>"
    );
    round_trip!(
        sequence_of_sequence_of_sequences,
        SequenceOfSequenceOfSequences,
        vec![vec![InnerTestA { hidden: Some(true) }, InnerTestA { hidden: Some(false) }], vec![InnerTestA { hidden: None }], vec![]],
        "SEQUENCE_OF",
        "<SEQUENCE_OF><InnerTestA><hidden><true /></hidden></InnerTestA><InnerTestA><hidden><false /></hidden></InnerTestA></SEQUENCE_OF><SEQUENCE_OF><InnerTestA /></SEQUENCE_OF><SEQUENCE_OF />"
    );
    round_trip!(
        sequence_of_enum_sequences,
        SequenceOfEnumSequences,
        vec![
            EnumSequence {
                enum_field: EnumType::First
            },
            EnumSequence {
                enum_field: EnumType::Second
            }
        ],
        "SEQUENCE_OF",
        "<Enum-Sequence><enum-field><eins /></enum-field></Enum-Sequence><Enum-Sequence><enum-field><zwei /></enum-field></Enum-Sequence>"
    );
    round_trip!(
        set_of_bools,
        SetOfBools,
        SetOf::from_vec(vec![true]),
        "SET_OF",
        "<true />"
    );
    round_trip!(
        set_of_enums,
        SetOfEnums,
        SetOf::from_vec(vec![EnumType::Second]),
        "SET_OF",
        "<zwei />"
    );
    round_trip!(
        set_of_integers,
        SetOfIntegers,
        SetOf::from_vec(vec![-1]),
        "SET_OF",
        "<INTEGER>-1</INTEGER>"
    );
    round_trip!(
        set_of_sequence_of_sequences,
        SetOfSequenceOfSequences,
        SetOf::from_vec(vec![vec![InnerTestA { hidden: None }]]),
        "SET_OF",
        "<SEQUENCE_OF><InnerTestA /></SEQUENCE_OF>"
    );
    round_trip!(
        set_of_enum_sequences,
        SetOfEnumSequences,
        SetOf::from_vec(vec![EnumSequence {
            enum_field: EnumType::Second
        }]),
        "SET_OF",
        "<Enum-Sequence><enum-field><zwei /></enum-field></Enum-Sequence>"
    );
    round_trip!(
        sequence_with_element_after_choice,
        SequenceWithChoice,
        SequenceWithChoice {
            recursion: RecursiveChoice::Leaf,
            nested: NestedTestA {
                wine: true,
                grappa: vec![0, 1, 2, 3].into(),
                inner: InnerTestA {
                    hidden: Some(false),
                },
                oid: Some(ObjectIdentifier::from(Oid::const_new(&[1, 8270, 4, 1]))),
            }
        },
        "SequenceWithChoice",
        "<recursion><Leaf /></recursion><nested><wine><true /></wine><grappa>00010203</grappa><inner><hidden><false /></hidden></inner><oid>1.8270.4.1</oid></nested>"
    );
    round_trip!(
        sequence_with_element_after_sequence_of,
        SequenceWithSequenceOf,
        SequenceWithSequenceOf {
            ids: vec![42, 13],
            flag: false,
            int: Integer::from(12),
            enum_val: EnumType::First
        },
        "SequenceWithSequenceOf",
        "<ids><INTEGER>42</INTEGER><INTEGER>13</INTEGER></ids><flag><false /></flag><int>12</int><enum_val><eins /></enum_val>"
    );
    round_trip!(
        sequence_with_element_after_set_of,
        SequenceWithSetOf,
        SequenceWithSetOf {
            ids: SetOf::from_vec(vec![42, 13]),
            flag: false,
            int: Integer::from(12),
            enum_val: EnumType::First
        },
        "SequenceWithSetOf",
        "<ids><INTEGER>42</INTEGER><INTEGER>13</INTEGER></ids><flag><false /></flag><int>12</int><enum_val><eins /></enum_val>"
    );

    #[test]
    fn set_of_round_trip() {
        let first = EnumSequence {
            enum_field: EnumType::First,
        };
        let second = EnumSequence {
            enum_field: EnumType::Second,
        };
        let value = SetOf::<EnumSequence>::from_vec(vec![first.clone(), second.clone()]);
        let encoded = crate::xer::encode(&value).unwrap();
        let decoded: SetOf<EnumSequence> = crate::xer::decode(&encoded).unwrap();

        assert!(String::from_utf8(encoded.clone())
            .unwrap()
            .contains("<Enum-Sequence><enum-field><zwei /></enum-field></Enum-Sequence>"));
        assert!(String::from_utf8(encoded)
            .unwrap()
            .contains("<Enum-Sequence><enum-field><eins /></enum-field></Enum-Sequence>"));
        assert!(decoded.contains(&first));
        assert!(decoded.contains(&second));
    }
}
