//! In the ALIGNED variant, the count of a SEQUENCE OF is a constrained whole
//! number (ITU-T X.691 (02/2021) §20.6, §11.9.3.3): a bit-field for a range
//! of 255 at most, which the components follow without padding, one aligned
//! octet for a range of 256 and two for a larger one (§11.5.7), and an
//! unconstrained length from an upper bound of 64K (§11.9.3.5).
use rasn::prelude::*;

/// `Booleans ::= SEQUENCE (SIZE (1..12)) OF BOOLEAN`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=12"))]
struct Booleans(SequenceOf<bool>);

/// `Item ::= SEQUENCE { a OCTET STRING (SIZE (3)), ... }`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Item {
    a: FixedOctetString<3>,
}

/// `Items ::= SEQUENCE (SIZE (1..12)) OF Item`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=12"))]
struct Items(SequenceOf<Item>);

/// `Small ::= SEQUENCE (SIZE (1..8)) OF BOOLEAN`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
struct Small(SequenceOf<bool>);

/// `Between ::= SEQUENCE { first BOOLEAN, list Small, last BOOLEAN }`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct Between {
    first: bool,
    list: Small,
    last: bool,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(enumerated)]
enum Criticality {
    Reject = 0,
    Ignore = 1,
    Notify = 2,
}

/// `Field ::= SEQUENCE { id INTEGER (0..65535), criticality Criticality, value OPEN TYPE }`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
struct Field {
    #[rasn(value("0..=65535"))]
    id: u16,
    criticality: Criticality,
    value: Any,
}

/// `Message ::= SEQUENCE { fields SEQUENCE (SIZE (0..65535)) OF Field, ... }`,
/// as the messages of NGAP, S1AP, F1AP and E1AP hold their protocol IEs.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Message {
    #[rasn(size("0..=65535"))]
    fields: SequenceOf<Field>,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(enumerated)]
#[non_exhaustive]
enum TypeOfError {
    NotUnderstood = 0,
    Missing = 1,
}

/// `Diagnostic ::= SEQUENCE { criticality Criticality, id INTEGER (0..65535),
/// typeOfError TypeOfError, extensions SEQUENCE (SIZE (1..65535)) OF Field OPTIONAL, ... }`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Diagnostic {
    criticality: Criticality,
    #[rasn(value("0..=65535"))]
    id: u16,
    type_of_error: TypeOfError,
    #[rasn(size("1..=65535"))]
    extensions: Option<SequenceOf<Field>>,
}

/// The `CriticalityDiagnostics` of NGAP: `SEQUENCE { procedureCode INTEGER (0..255) OPTIONAL,
/// triggeringMessage Criticality OPTIONAL, procedureCriticality Criticality OPTIONAL,
/// list SEQUENCE (SIZE (1..256)) OF Diagnostic OPTIONAL,
/// extensions SEQUENCE (SIZE (1..65535)) OF Field OPTIONAL, ... }`,
/// with `Criticality` in the place of its `TriggeringMessage`, which has three values too.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct Diagnostics {
    procedure_code: Option<u8>,
    triggering_message: Option<Criticality>,
    procedure_criticality: Option<Criticality>,
    #[rasn(size("1..=256"))]
    list: Option<SequenceOf<Diagnostic>>,
    #[rasn(size("1..=65535"))]
    extensions: Option<SequenceOf<Field>>,
}

/// `Connections ::= SEQUENCE (SIZE (1..65536)) OF BOOLEAN`
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=65536"))]
struct Connections(SequenceOf<bool>);

fn round_trip<T>(value: &T, expected: &[u8])
where
    T: Decode + Encode + PartialEq + core::fmt::Debug,
{
    assert_eq!(rasn::aper::encode(value).unwrap(), expected);
    assert_eq!(&rasn::aper::decode::<T>(expected).unwrap(), value);
}

#[test]
fn components_follow_a_bit_field_count_without_padding() {
    // The reproduction of the issue: a count of four bits, then the
    // components.
    round_trip(&Booleans(vec![true, false, true]), &[0b0010_1010]);
    // Each item starts in the octet that the count ends in, with its
    // extension bit, and aligns its own octets.
    round_trip(
        &Items(vec![
            Item {
                a: [0x01, 0x02, 0x03].into(),
            },
            Item {
                a: [0x04, 0x05, 0x06].into(),
            },
        ]),
        &[0x10, 0x01, 0x02, 0x03, 0x00, 0x04, 0x05, 0x06],
    );

    // A count of three bits: 010, then 111.
    round_trip(&Small(vec![true; 3]), &[0b0101_1100]);
    // The same list between two components: 1, 010, 111, 1.
    round_trip(
        &Between {
            first: true,
            list: Small(vec![true; 3]),
            last: true,
        },
        &[0b1010_1111],
    );
}

#[test]
fn a_count_of_two_octets_is_octet_aligned() {
    // The extension bit, padding, the count in two octets, then the field:
    // its identifier in two octets, its criticality, and its open type.
    round_trip(
        &Message {
            fields: vec![Field {
                id: 85,
                criticality: Criticality::Reject,
                value: Any::new(vec![0x00, 0x07]),
            }],
        },
        &[0x00, 0x00, 0x01, 0x00, 0x55, 0x00, 0x02, 0x00, 0x07],
    );
}

#[test]
fn a_count_of_one_octet_is_octet_aligned() {
    // A range of 256: the count is one aligned octet (§11.5.7.2). These are
    // the octets of the issue, for a list of one diagnostic.
    round_trip(
        &Diagnostics {
            procedure_code: None,
            triggering_message: None,
            procedure_criticality: None,
            list: Some(vec![Diagnostic {
                criticality: Criticality::Notify,
                id: 10,
                type_of_error: TypeOfError::Missing,
                extensions: None,
            }]),
            extensions: None,
        },
        &[0x08, 0x00, 0x20, 0x00, 0x0a, 0x40],
    );
}

#[test]
fn a_count_with_an_upper_bound_of_64k_is_unconstrained() {
    // One aligned length octet, as for a list without a size constraint.
    round_trip(&Connections(vec![true]), &[0x01, 0x80]);
}
