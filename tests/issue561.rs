use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct Msg {
    #[rasn(identifier = "messageData")]
    pub message_data: MessageData,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct MessageData {
    #[rasn(size("1..=7"), identifier = "elementIds")]
    pub element_ids: SequenceOf<ElementId>,
    #[rasn(identifier = "constrainedData")]
    pub constrained_data: Option<ConstrainedData>,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice)]
pub enum ElementId {
    #[rasn(extension_addition, tag(context, 270), identifier = "uM270")]
    UM270(ClearanceTypeRAssignedTime),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ClearanceTypeRAssignedTime {
    #[rasn(identifier = "clearanceType")]
    pub clearance_type: ClearanceType,
    #[rasn(identifier = "assignedTime")]
    pub assigned_time: AssignedTime,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Copy)]
#[rasn(enumerated)]
pub enum ClearanceType {
    #[rasn(identifier = "approach")]
    Approach = 0,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AssignedTime(pub ());

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
pub struct ConstrainedData {
    #[rasn(identifier = "dummy")]
    pub dummy: Option<()>,
}

fn msg(constrained_data: Option<ConstrainedData>) -> Msg {
    Msg {
        message_data: MessageData {
            element_ids: vec![ElementId::UM270(ClearanceTypeRAssignedTime {
                clearance_type: ClearanceType::Approach,
                assigned_time: AssignedTime(()),
            })],
            constrained_data,
        },
    }
}

#[test]
fn xer_sequence_of_choice_uses_choice_identifier() {
    let encoded = String::from_utf8(rasn::xer::encode(&msg(None)).unwrap()).unwrap();
    assert_eq!(
        encoded,
        "<Msg><messageData><elementIds><uM270><clearanceType><approach /></clearanceType><assignedTime /></uM270></elementIds></messageData></Msg>"
    );
}

#[test]
fn xer_sequence_of_choice_does_not_borrow_following_field_name() {
    let value = msg(Some(ConstrainedData { dummy: Some(()) }));
    let encoded = String::from_utf8(rasn::xer::encode(&value).unwrap()).unwrap();
    assert_eq!(
        encoded,
        "<Msg><messageData><elementIds><uM270><clearanceType><approach /></clearanceType><assignedTime /></uM270></elementIds><constrainedData><dummy /></constrainedData></messageData></Msg>"
    );
}
