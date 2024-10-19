#![doc = include_str!("../README.md")]
#![no_std]
#![allow(clippy::too_many_arguments)]

extern crate alloc;

use alloc::boxed::Box;
use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum AircraftPdus {
    #[rasn(tag(explicit(context, 0)))]
    AbortUser(CpdlcUserAbortReason),
    #[rasn(tag(explicit(context, 1)))]
    AbortProvider(CpdlcProviderAbortReason),
    #[rasn(tag(explicit(context, 2)))]
    StartDown(StartDownMessage),
    #[rasn(tag(explicit(context, 3)))]
    Send(AtcDownlinkMessage),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CpdlcProviderAbortReason {
    #[rasn(identifier = "timer-expired")]
    TimerExpired = 0,
    #[rasn(identifier = "undefined-error")]
    UndefinedError = 1,
    #[rasn(identifier = "invalid-PDU")]
    InvalidPdu = 2,
    #[rasn(identifier = "protocol-error")]
    ProtocolError = 3,
    #[rasn(identifier = "communication-service-error")]
    CommunicationServiceError = 4,
    #[rasn(identifier = "communication-service-failure")]
    CommunicationServiceFailure = 5,
    #[rasn(identifier = "invalid-QOS-parameter")]
    InvalidQosParameter = 6,
    #[rasn(identifier = "expected-PDU-missing")]
    ExpectedPduMissing = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CpdlcUserAbortReason {
    Undefined = 0,
    #[rasn(identifier = "no-message-identification-numbers-available")]
    NoMessageIdentificationNumbersAvailable = 1,
    #[rasn(identifier = "duplicate-message-identification-numbers")]
    DuplicateMessageIdentificationNumbers = 2,
    #[rasn(identifier = "no-longer-next-data-authority")]
    NoLongerNextDataAuthority = 3,
    #[rasn(identifier = "current-data-authority-abort")]
    CurrentDataAuthorityAbort = 4,
    #[rasn(identifier = "commanded-termination")]
    CommandedTermination = 5,
    #[rasn(identifier = "invalid-response")]
    InvalidResponse = 6,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum DownlinkMessage {
    #[rasn(tag(explicit(context, 0)))]
    NoMessage(()),
    #[rasn(tag(explicit(context, 1)))]
    AtcDownlinkMessage(AtcDownlinkMessage),
}

/// Ground Generated Messages - Top level
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum GroundPdus {
    #[rasn(tag(explicit(context, 0)))]
    AbortUser(CpdlcUserAbortReason),
    #[rasn(tag(explicit(context, 1)))]
    AbortProvider(CpdlcProviderAbortReason),
    #[rasn(tag(explicit(context, 2)))]
    Startup(UplinkMessage),
    #[rasn(tag(explicit(context, 3)))]
    Send(AtcUplinkMessage),
    #[rasn(tag(explicit(context, 4)))]
    Forward(AtcForwardMessage),
    #[rasn(tag(explicit(context, 5)))]
    ForwardResponse(AtcForwardResponse),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum Mode {
    Cpdlc = 0,
    Dsc = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct StartDownMessage {
    #[rasn(default = "start_down_message_mode_default")]
    pub mode: Mode,
    #[rasn(identifier = "startDownlinkMessage")]
    pub start_downlink_message: DownlinkMessage,
}

impl StartDownMessage {
    pub fn new(mode: Mode, start_downlink_message: DownlinkMessage) -> Self {
        Self {
            mode,
            start_downlink_message,
        }
    }
}

fn start_down_message_mode_default() -> Mode {
    Mode::Cpdlc
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum UplinkMessage {
    #[rasn(tag(explicit(context, 0)))]
    NoMessage(()),
    #[rasn(tag(explicit(context, 1)))]
    AtcUplinkMessage(AtcUplinkMessage),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcForwardMessage {
    #[rasn(identifier = "forwardHeader")]
    pub forward_header: ForwardHeader,
    #[rasn(identifier = "forwardMessage")]
    pub forward_message: ForwardMessage,
}

impl AtcForwardMessage {
    pub fn new(forward_header: ForwardHeader, forward_message: ForwardMessage) -> Self {
        Self {
            forward_header,
            forward_message,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum AtcForwardResponse {
    Success = 0,
    #[rasn(identifier = "service-not-supported")]
    ServiceNotSupported = 1,
    #[rasn(identifier = "version-not-equal")]
    VersionNotEqual = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct AlgorithmIdentifier(pub ObjectIdentifier);

/// root is {icao-arc atn-algorithms(9)}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct CPDLCMessage(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ForwardHeader {
    #[rasn(identifier = "dateTime")]
    pub date_time: DateTimeGroup,
    #[rasn(identifier = "aircraftID")]
    pub aircraft_id: AircraftFlightIdentification,
    #[rasn(identifier = "aircraftAddress")]
    pub aircraft_address: AircraftAddress,
}

impl ForwardHeader {
    pub fn new(
        date_time: DateTimeGroup,
        aircraft_id: AircraftFlightIdentification,
        aircraft_address: AircraftAddress,
    ) -> Self {
        Self {
            date_time,
            aircraft_id,
            aircraft_address,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum ForwardMessage {
    #[rasn(tag(explicit(context, 0)))]
    UpElementIds(BitString),
    #[rasn(tag(explicit(context, 1)))]
    DownElementIds(BitString),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PmCpdlcProviderAbortReason {
    #[rasn(identifier = "timer-expired")]
    TimerExpired = 0,
    #[rasn(identifier = "undefined-error")]
    UndefinedError = 1,
    #[rasn(identifier = "invalid-PDU")]
    InvalidPdu = 2,
    #[rasn(identifier = "protocol-error")]
    ProtocolError = 3,
    #[rasn(identifier = "communication-service-error")]
    CommunicationServiceError = 4,
    #[rasn(identifier = "communication-service-failure")]
    CommunicationServiceFailure = 5,
    #[rasn(identifier = "invalid-QOS-parameter")]
    InvalidQosParameter = 6,
    #[rasn(identifier = "expected-PDU-missing")]
    ExpectedPduMissing = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PmCpdlcUserAbortReason {
    Undefined = 0,
    #[rasn(identifier = "no-message-identification-numbers-available")]
    NoMessageIdentificationNumbersAvailable = 1,
    #[rasn(identifier = "duplicate-message-identification-numbers")]
    DuplicateMessageIdentificationNumbers = 2,
    #[rasn(identifier = "no-longer-next-data-authority")]
    NoLongerNextDataAuthority = 3,
    #[rasn(identifier = "current-data-authority-abort")]
    CurrentDataAuthorityAbort = 4,
    #[rasn(identifier = "commanded-termination")]
    CommandedTermination = 5,
    #[rasn(identifier = "invalid-response")]
    InvalidResponse = 6,
    #[rasn(identifier = "time-out-of-synchronisation")]
    TimeOutOfSynchronisation = 7,
    #[rasn(identifier = "unknown-integrity-check")]
    UnknownIntegrityCheck = 8,
    #[rasn(identifier = "validation-failure")]
    ValidationFailure = 9,
    #[rasn(identifier = "unable-to-decode-message")]
    UnableToDecodeMessage = 10,
    #[rasn(identifier = "invalid-pdu")]
    InvalidPdu = 11,
    #[rasn(identifier = "invalid-CPDLC-message")]
    InvalidCpdlcMessage = 12,
}

/// Aircraft Generated Messages - Top level
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum ProtectedAircraftPDUs {
    #[rasn(tag(explicit(context, 0)))]
    AbortUser(PmCpdlcUserAbortReason),
    #[rasn(tag(explicit(context, 1)))]
    AbortProvider(PmCpdlcProviderAbortReason),
    #[rasn(tag(explicit(context, 2)))]
    StartDown(ProtectedStartDownMessage),
    #[rasn(tag(explicit(context, 3)))]
    Send(ProtectedDownlinkMessage),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ProtectedDownlinkMessage {
    #[rasn(tag(explicit(context, 0)), identifier = "algorithmIdentifier")]
    pub algorithm_identifier: Option<AlgorithmIdentifier>,
    #[rasn(tag(explicit(context, 1)), identifier = "protectedMessage")]
    pub protected_message: Option<CPDLCMessage>,
    #[rasn(tag(explicit(context, 2)), identifier = "integrityCheck")]
    pub integrity_check: BitString,
}

impl ProtectedDownlinkMessage {
    pub fn new(
        algorithm_identifier: Option<AlgorithmIdentifier>,
        protected_message: Option<CPDLCMessage>,
        integrity_check: BitString,
    ) -> Self {
        Self {
            algorithm_identifier,
            protected_message,
            integrity_check,
        }
    }
}

#[doc = " Ground Generated Messages - Top level"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum ProtectedGroundPdus {
    #[rasn(tag(explicit(context, 0)))]
    AbortUser(PmCpdlcUserAbortReason),
    #[rasn(tag(explicit(context, 1)))]
    AbortProvider(PmCpdlcProviderAbortReason),
    #[rasn(tag(explicit(context, 2)))]
    Startup(ProtectedUplinkMessage),
    #[rasn(tag(explicit(context, 3)))]
    Send(ProtectedUplinkMessage),
    #[rasn(tag(explicit(context, 4)))]
    Forward(AtcForwardMessage),
    #[rasn(tag(explicit(context, 5)))]
    ForwardResponse(AtcForwardResponse),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum ProtectedMode {
    Cpdlc = 0,
    Dsc = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ProtectedStartDownMessage {
    #[rasn(default = "protected_start_down_message_mode_default")]
    pub mode: ProtectedMode,
    #[rasn(identifier = "startDownlinkMessage")]
    pub start_downlink_message: ProtectedDownlinkMessage,
}

impl ProtectedStartDownMessage {
    pub fn new(mode: ProtectedMode, start_downlink_message: ProtectedDownlinkMessage) -> Self {
        Self {
            mode,
            start_downlink_message,
        }
    }
}

fn protected_start_down_message_mode_default() -> ProtectedMode {
    ProtectedMode::Cpdlc
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ProtectedUplinkMessage {
    #[rasn(tag(explicit(context, 0)), identifier = "algorithmIdentifier")]
    pub algorithm_identifier: Option<AlgorithmIdentifier>,
    #[rasn(tag(explicit(context, 1)), identifier = "protectedMessage")]
    pub protected_message: Option<CPDLCMessage>,
    #[rasn(tag(explicit(context, 2)), identifier = "integrityCheck")]
    pub integrity_check: BitString,
}

impl ProtectedUplinkMessage {
    pub fn new(
        algorithm_identifier: Option<AlgorithmIdentifier>,
        protected_message: Option<CPDLCMessage>,
        integrity_check: BitString,
    ) -> Self {
        Self {
            algorithm_identifier,
            protected_message,
            integrity_check,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcDownlinkMessage {
    pub header: AtcMessageHeader,
    #[rasn(identifier = "messageData")]
    pub message_data: AtcDownlinkMessageData,
}
impl AtcDownlinkMessage {
    pub fn new(header: AtcMessageHeader, message_data: AtcDownlinkMessageData) -> Self {
        Self {
            header,
            message_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AtcDownlinkMessageDataConstrainedData {
    #[rasn(size("1..=2"), identifier = "routeClearanceData")]
    pub route_clearance_data: Option<SequenceOf<RouteClearance>>,
}

impl AtcDownlinkMessageDataConstrainedData {
    pub fn new(route_clearance_data: Option<SequenceOf<RouteClearance>>) -> Self {
        Self {
            route_clearance_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcDownlinkMessageData {
    #[rasn(size("1..=5"), identifier = "elementIds")]
    pub element_ids: SequenceOf<AtcDownlinkMsgElementId>,
    #[rasn(identifier = "constrainedData")]
    pub constrained_data: Option<AtcDownlinkMessageDataConstrainedData>,
}

impl AtcDownlinkMessageData {
    pub fn new(
        element_ids: SequenceOf<AtcDownlinkMsgElementId>,
        constrained_data: Option<AtcDownlinkMessageDataConstrainedData>,
    ) -> Self {
        Self {
            element_ids,
            constrained_data,
        }
    }
}

#[doc = " Downlink message element"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum AtcDownlinkMsgElementId {
    #[rasn(tag(explicit(context, 0)))]
    DM0Null(()),
    #[rasn(tag(explicit(context, 1)))]
    DM1Null(()),
    #[rasn(tag(explicit(context, 2)))]
    DM2Null(()),
    #[rasn(tag(explicit(context, 3)))]
    DM3Null(()),
    #[rasn(tag(explicit(context, 4)))]
    DM4Null(()),
    #[rasn(tag(explicit(context, 5)))]
    DM5Null(()),
    #[rasn(tag(explicit(context, 6)))]
    DM6Level(Level),
    #[rasn(tag(explicit(context, 7)))]
    DM7LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 8)))]
    DM8Level(Level),
    #[rasn(tag(explicit(context, 9)))]
    DM9Level(Level),
    #[rasn(tag(explicit(context, 10)))]
    DM10Level(Level),
    #[rasn(tag(explicit(context, 11)))]
    DM11PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 12)))]
    DM12PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 13)))]
    DM13TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 14)))]
    DM14TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 15)))]
    DM15DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 16)))]
    DM16PositionDistanceSpecifiedDirection(PositionDistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 17)))]
    DM17TimeDistanceSpecifiedDirection(TimeDistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 18)))]
    DM18Speed(Speed),
    #[rasn(tag(explicit(context, 19)))]
    DM19SpeedSpeed(SpeedSpeed),
    #[rasn(tag(explicit(context, 20)))]
    DM20Null(()),
    #[rasn(tag(explicit(context, 21)))]
    DM21Frequency(Frequency),
    #[rasn(tag(explicit(context, 22)))]
    DM22Position(Position),
    #[rasn(tag(explicit(context, 23)))]
    DM23ProcedureName(ProcedureName),
    #[rasn(tag(explicit(context, 24)))]
    DM24RouteClearance(RouteClearanceIndex),
    #[rasn(tag(explicit(context, 25)))]
    DM25ClearanceType(ClearanceType),
    #[rasn(tag(explicit(context, 26)))]
    DM26PositionRouteClearance(PositionRouteClearanceIndex),
    #[rasn(tag(explicit(context, 27)))]
    DM27DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 28)))]
    DM28Level(Level),
    #[rasn(tag(explicit(context, 29)))]
    DM29Level(Level),
    #[rasn(tag(explicit(context, 30)))]
    DM30Level(Level),
    #[rasn(tag(explicit(context, 31)))]
    DM31Position(Position),
    #[rasn(tag(explicit(context, 32)))]
    DM32Level(Level),
    #[rasn(tag(explicit(context, 33)))]
    DM33Position(Position),
    #[rasn(tag(explicit(context, 34)))]
    DM34Speed(Speed),
    #[rasn(tag(explicit(context, 35)))]
    DM35Degrees(Degrees),
    #[rasn(tag(explicit(context, 36)))]
    DM36Degrees(Degrees),
    #[rasn(tag(explicit(context, 37)))]
    DM37Level(Level),
    #[rasn(tag(explicit(context, 38)))]
    DM38Level(Level),
    #[rasn(tag(explicit(context, 39)))]
    DM39Speed(Speed),
    #[rasn(tag(explicit(context, 40)))]
    DM40RouteClearance(RouteClearanceIndex),
    #[rasn(tag(explicit(context, 41)))]
    DM41Null(()),
    #[rasn(tag(explicit(context, 42)))]
    DM42Position(Position),
    #[rasn(tag(explicit(context, 43)))]
    DM43Time(Time),
    #[rasn(tag(explicit(context, 44)))]
    DM44Position(Position),
    #[rasn(tag(explicit(context, 45)))]
    DM45Position(Position),
    #[rasn(tag(explicit(context, 46)))]
    DM46Time(Time),
    #[rasn(tag(explicit(context, 47)))]
    DM47Code(Code),
    #[rasn(tag(explicit(context, 48)))]
    DM48PositionReport(Box<PositionReport>),
    #[rasn(tag(explicit(context, 49)))]
    DM49Speed(Speed),
    #[rasn(tag(explicit(context, 50)))]
    DM50SpeedSpeed(SpeedSpeed),
    #[rasn(tag(explicit(context, 51)))]
    DM51Null(()),
    #[rasn(tag(explicit(context, 52)))]
    DM52Null(()),
    #[rasn(tag(explicit(context, 53)))]
    DM53Null(()),
    #[rasn(tag(explicit(context, 54)))]
    DM54Level(Level),
    #[rasn(tag(explicit(context, 55)))]
    DM55Null(()),
    #[rasn(tag(explicit(context, 56)))]
    DM56Null(()),
    #[rasn(tag(explicit(context, 57)))]
    DM57RemainingFuelPersonsOnBoard(RemainingFuelPersonsOnBoard),
    #[rasn(tag(explicit(context, 58)))]
    DM58Null(()),
    #[rasn(tag(explicit(context, 59)))]
    DM59PositionRouteClearance(PositionRouteClearanceIndex),
    #[rasn(tag(explicit(context, 60)))]
    DM60DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 61)))]
    DM61Level(Level),
    #[rasn(tag(explicit(context, 62)))]
    DM62ErrorInformation(ErrorInformation),
    #[rasn(tag(explicit(context, 63)))]
    DM63Null(()),
    #[rasn(tag(explicit(context, 64)))]
    DM64FacilityDesignation(FacilityDesignation),
    #[rasn(tag(explicit(context, 65)))]
    DM65Null(()),
    #[rasn(tag(explicit(context, 66)))]
    DM66Null(()),
    #[rasn(tag(explicit(context, 67)))]
    DM67FreeText(FreeText),
    #[rasn(tag(explicit(context, 68)))]
    DM68FreeText(FreeText),
    #[rasn(tag(explicit(context, 69)))]
    DM69Null(()),
    #[rasn(tag(explicit(context, 70)))]
    DM70Degrees(Degrees),
    #[rasn(tag(explicit(context, 71)))]
    DM71Degrees(Degrees),
    #[rasn(tag(explicit(context, 72)))]
    DM72Level(Level),
    #[rasn(tag(explicit(context, 73)))]
    DM73Versionnumber(VersionNumber),
    #[rasn(tag(explicit(context, 74)))]
    DM74Null(()),
    #[rasn(tag(explicit(context, 75)))]
    DM75Null(()),
    #[rasn(tag(explicit(context, 76)))]
    DM76LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 77)))]
    DM77LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 78)))]
    DM78TimeDistanceToFromPosition(TimeDistanceToFromPosition),
    #[rasn(tag(explicit(context, 79)))]
    DM79AtisCode(ATISCode),
    #[rasn(tag(explicit(context, 80)))]
    DM80DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 81)))]
    DM81LevelTime(LevelTime),
    #[rasn(tag(explicit(context, 82)))]
    DM82Level(Level),
    #[rasn(tag(explicit(context, 83)))]
    DM83SpeedTime(SpeedTime),
    #[rasn(tag(explicit(context, 84)))]
    DM84Speed(Speed),
    #[rasn(tag(explicit(context, 85)))]
    DM85DistanceSpecifiedDirectionTime(DistanceSpecifiedDirectionTime),
    #[rasn(tag(explicit(context, 86)))]
    DM86DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 87)))]
    DM87Level(Level),
    #[rasn(tag(explicit(context, 88)))]
    DM88Level(Level),
    #[rasn(tag(explicit(context, 89)))]
    DM89UnitnameFrequency(UnitNameFrequency),
    #[rasn(tag(explicit(context, 90)))]
    DM90FreeText(FreeText),
    #[rasn(tag(explicit(context, 91)))]
    DM91FreeText(FreeText),
    #[rasn(tag(explicit(context, 92)))]
    DM92FreeText(FreeText),
    #[rasn(tag(explicit(context, 93)))]
    DM93FreeText(FreeText),
    #[rasn(tag(explicit(context, 94)))]
    DM94FreeText(FreeText),
    #[rasn(tag(explicit(context, 95)))]
    DM95FreeText(FreeText),
    #[rasn(tag(explicit(context, 96)))]
    DM96FreeText(FreeText),
    #[rasn(tag(explicit(context, 97)))]
    DM97FreeText(FreeText),
    #[rasn(tag(explicit(context, 98)))]
    DM98FreeText(FreeText),
    #[rasn(tag(explicit(context, 99)))]
    DM99Null(()),
    #[rasn(tag(explicit(context, 100)))]
    DM100Null(()),
    #[rasn(tag(explicit(context, 101)))]
    DM101Null(()),
    #[rasn(tag(explicit(context, 102)))]
    DM102Null(()),
    #[rasn(tag(explicit(context, 103)))]
    DM103Null(()),
    #[rasn(tag(explicit(context, 104)))]
    DM104PositionTime(PositionTime),
    #[rasn(tag(explicit(context, 105)))]
    DM105Airport(Airport),
    #[rasn(tag(explicit(context, 106)))]
    DM106Level(Level),
    #[rasn(tag(explicit(context, 107)))]
    DM107Null(()),
    #[rasn(tag(explicit(context, 108)))]
    DM108Null(()),
    #[rasn(tag(explicit(context, 109)))]
    DM109Time(Time),
    #[rasn(tag(explicit(context, 110)))]
    DM110Position(Position),
    #[rasn(tag(explicit(context, 111)))]
    DM111TimePosition(TimePosition),
    #[rasn(tag(explicit(context, 112)))]
    DM112Null(()),
    #[rasn(tag(explicit(context, 113)))]
    DM113SpeedTypeSpeedTypeSpeedTypeSpeed(SpeedTypeSpeedTypeSpeedTypeSpeed),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcMessageHeader {
    #[rasn(tag(explicit(context, 0)), identifier = "messageIdNumber")]
    pub message_id_number: MsgIdentificationNumber,
    #[rasn(tag(explicit(context, 1)), identifier = "messageRefNumber")]
    pub message_ref_number: Option<MsgReferenceNumber>,
    #[rasn(tag(explicit(context, 2)), identifier = "dateTime")]
    pub date_time: DateTimeGroup,
    #[rasn(
        tag(explicit(context, 3)),
        default = "atcmessage_header_logical_ack_default",
        identifier = "logicalAck"
    )]
    pub logical_ack: LogicalAck,
}

impl AtcMessageHeader {
    pub fn new(
        message_id_number: MsgIdentificationNumber,
        message_ref_number: Option<MsgReferenceNumber>,
        date_time: DateTimeGroup,
        logical_ack: LogicalAck,
    ) -> Self {
        Self {
            message_id_number,
            message_ref_number,
            date_time,
            logical_ack,
        }
    }
}

fn atcmessage_header_logical_ack_default() -> LogicalAck {
    LogicalAck::NotRequired
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcUplinkMessage {
    pub header: AtcMessageHeader,
    #[rasn(identifier = "messageData")]
    pub message_data: AtcUplinkMessageData,
}

impl AtcUplinkMessage {
    pub fn new(header: AtcMessageHeader, message_data: AtcUplinkMessageData) -> Self {
        Self {
            header,
            message_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AtcUplinkMessageDataConstrainedData {
    #[rasn(size("1..=2"), identifier = "routeClearanceData")]
    pub route_clearance_data: Option<SequenceOf<RouteClearance>>,
}

impl AtcUplinkMessageDataConstrainedData {
    pub fn new(route_clearance_data: Option<SequenceOf<RouteClearance>>) -> Self {
        Self {
            route_clearance_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtcUplinkMessageData {
    #[rasn(size("1..=5"), identifier = "elementIds")]
    pub element_ids: SequenceOf<AtcUplinkMsgElementId>,
    #[rasn(identifier = "constrainedData")]
    pub constrained_data: Option<AtcUplinkMessageDataConstrainedData>,
}

impl AtcUplinkMessageData {
    pub fn new(
        element_ids: SequenceOf<AtcUplinkMsgElementId>,
        constrained_data: Option<AtcUplinkMessageDataConstrainedData>,
    ) -> Self {
        Self {
            element_ids,
            constrained_data,
        }
    }
}

#[doc = " Uplink message element"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum AtcUplinkMsgElementId {
    #[rasn(tag(explicit(context, 0)))]
    UM0Null(()),
    #[rasn(tag(explicit(context, 1)))]
    UM1Null(()),
    #[rasn(tag(explicit(context, 2)))]
    UM2Null(()),
    #[rasn(tag(explicit(context, 3)))]
    UM3Null(()),
    #[rasn(tag(explicit(context, 4)))]
    UM4Null(()),
    #[rasn(tag(explicit(context, 5)))]
    UM5Null(()),
    #[rasn(tag(explicit(context, 6)))]
    UM6Level(Level),
    #[rasn(tag(explicit(context, 7)))]
    UM7Time(Time),
    #[rasn(tag(explicit(context, 8)))]
    UM8Position(Position),
    #[rasn(tag(explicit(context, 9)))]
    UM9Time(Time),
    #[rasn(tag(explicit(context, 10)))]
    UM10Position(Position),
    #[rasn(tag(explicit(context, 11)))]
    UM11Time(Time),
    #[rasn(tag(explicit(context, 12)))]
    UM12Position(Position),
    #[rasn(tag(explicit(context, 13)))]
    UM13TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 14)))]
    UM14PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 15)))]
    UM15TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 16)))]
    UM16PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 17)))]
    UM17TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 18)))]
    UM18PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 19)))]
    UM19Level(Level),
    #[rasn(tag(explicit(context, 20)))]
    UM20Level(Level),
    #[rasn(tag(explicit(context, 21)))]
    UM21TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 22)))]
    UM22PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 23)))]
    UM23Level(Level),
    #[rasn(tag(explicit(context, 24)))]
    UM24TimeLevel(TimeLevel),
    #[rasn(tag(explicit(context, 25)))]
    UM25PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 26)))]
    UM26LevelTime(LevelTime),
    #[rasn(tag(explicit(context, 27)))]
    UM27LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 28)))]
    UM28LevelTime(LevelTime),
    #[rasn(tag(explicit(context, 29)))]
    UM29LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 30)))]
    UM30LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 31)))]
    UM31LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 32)))]
    UM32LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 33)))]
    UM33Null(()),
    #[rasn(tag(explicit(context, 34)))]
    UM34Level(Level),
    #[rasn(tag(explicit(context, 35)))]
    UM35Level(Level),
    #[rasn(tag(explicit(context, 36)))]
    UM36Level(Level),
    #[rasn(tag(explicit(context, 37)))]
    UM37Level(Level),
    #[rasn(tag(explicit(context, 38)))]
    UM38Level(Level),
    #[rasn(tag(explicit(context, 39)))]
    UM39Level(Level),
    #[rasn(tag(explicit(context, 40)))]
    UM40Null(()),
    #[rasn(tag(explicit(context, 41)))]
    UM41Null(()),
    #[rasn(tag(explicit(context, 42)))]
    UM42PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 43)))]
    UM43PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 44)))]
    UM44PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 45)))]
    UM45PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 46)))]
    UM46PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 47)))]
    UM47PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 48)))]
    UM48PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 49)))]
    UM49PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 50)))]
    UM50PositionLevelLevel(PositionLevelLevel),
    #[rasn(tag(explicit(context, 51)))]
    UM51PositionTime(PositionTime),
    #[rasn(tag(explicit(context, 52)))]
    UM52PositionTime(PositionTime),
    #[rasn(tag(explicit(context, 53)))]
    UM53PositionTime(PositionTime),
    #[rasn(tag(explicit(context, 54)))]
    UM54PositionTimeTime(PositionTimeTime),
    #[rasn(tag(explicit(context, 55)))]
    UM55PositionSpeed(PositionSpeed),
    #[rasn(tag(explicit(context, 56)))]
    UM56PositionSpeed(PositionSpeed),
    #[rasn(tag(explicit(context, 57)))]
    UM57PositionSpeed(PositionSpeed),
    #[rasn(tag(explicit(context, 58)))]
    UM58PositionTimeLevel(PositionTimeLevel),
    #[rasn(tag(explicit(context, 59)))]
    UM59PositionTimeLevel(PositionTimeLevel),
    #[rasn(tag(explicit(context, 60)))]
    UM60PositionTimeLevel(PositionTimeLevel),
    #[rasn(tag(explicit(context, 61)))]
    UM61PositionLevelSpeed(PositionLevelSpeed),
    #[rasn(tag(explicit(context, 62)))]
    UM62TimePositionLevel(TimePositionLevel),
    #[rasn(tag(explicit(context, 63)))]
    UM63TimePositionLevelSpeed(TimePositionLevelSpeed),
    #[rasn(tag(explicit(context, 64)))]
    UM64DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 65)))]
    UM65PositionDistanceSpecifiedDirection(PositionDistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 66)))]
    UM66TimeDistanceSpecifiedDirection(TimeDistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 67)))]
    UM67Null(()),
    #[rasn(tag(explicit(context, 68)))]
    UM68Position(Position),
    #[rasn(tag(explicit(context, 69)))]
    UM69Time(Time),
    #[rasn(tag(explicit(context, 70)))]
    UM70Position(Position),
    #[rasn(tag(explicit(context, 71)))]
    UM71Time(Time),
    #[rasn(tag(explicit(context, 72)))]
    UM72Null(()),
    #[rasn(tag(explicit(context, 73)))]
    UM73DepartureClearance(Box<DepartureClearance>),
    #[rasn(tag(explicit(context, 74)))]
    UM74Position(Position),
    #[rasn(tag(explicit(context, 75)))]
    UM75Position(Position),
    #[rasn(tag(explicit(context, 76)))]
    UM76TimePosition(TimePosition),
    #[rasn(tag(explicit(context, 77)))]
    UM77PositionPosition(PositionPosition),
    #[rasn(tag(explicit(context, 78)))]
    UM78LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 79)))]
    UM79PositionRouteClearance(PositionRouteClearanceIndex),
    #[rasn(tag(explicit(context, 80)))]
    UM80RouteClearance(RouteClearanceIndex),
    #[rasn(tag(explicit(context, 81)))]
    UM81ProcedureName(ProcedureName),
    #[rasn(tag(explicit(context, 82)))]
    UM82DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 83)))]
    UM83PositionRouteClearance(PositionRouteClearanceIndex),
    #[rasn(tag(explicit(context, 84)))]
    UM84PositionProcedureName(PositionProcedureName),
    #[rasn(tag(explicit(context, 85)))]
    UM85RouteClearance(RouteClearanceIndex),
    #[rasn(tag(explicit(context, 86)))]
    UM86PositionRouteClearance(PositionRouteClearanceIndex),
    #[rasn(tag(explicit(context, 87)))]
    UM87Position(Position),
    #[rasn(tag(explicit(context, 88)))]
    UM88PositionPosition(PositionPosition),
    #[rasn(tag(explicit(context, 89)))]
    UM89TimePosition(TimePosition),
    #[rasn(tag(explicit(context, 90)))]
    UM90LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 91)))]
    UM91HoldClearance(HoldClearance),
    #[rasn(tag(explicit(context, 92)))]
    UM92PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 93)))]
    UM93Time(Time),
    #[rasn(tag(explicit(context, 94)))]
    UM94DirectionDegrees(DirectionDegrees),
    #[rasn(tag(explicit(context, 95)))]
    UM95DirectionDegrees(DirectionDegrees),
    #[rasn(tag(explicit(context, 96)))]
    UM96Null(()),
    #[rasn(tag(explicit(context, 97)))]
    UM97PositionDegrees(PositionDegrees),
    #[rasn(tag(explicit(context, 98)))]
    UM98DirectionDegrees(DirectionDegrees),
    #[rasn(tag(explicit(context, 99)))]
    UM99ProcedureName(ProcedureName),
    #[rasn(tag(explicit(context, 100)))]
    UM100TimeSpeed(TimeSpeed),
    #[rasn(tag(explicit(context, 101)))]
    UM101PositionSpeed(PositionSpeed),
    #[rasn(tag(explicit(context, 102)))]
    UM102LevelSpeed(LevelSpeed),
    #[rasn(tag(explicit(context, 103)))]
    UM103TimeSpeedSpeed(TimeSpeedSpeed),
    #[rasn(tag(explicit(context, 104)))]
    UM104PositionSpeedSpeed(PositionSpeedSpeed),
    #[rasn(tag(explicit(context, 105)))]
    UM105LevelSpeedSpeed(LevelSpeedSpeed),
    #[rasn(tag(explicit(context, 106)))]
    UM106Speed(Speed),
    #[rasn(tag(explicit(context, 107)))]
    UM107Null(()),
    #[rasn(tag(explicit(context, 108)))]
    UM108Speed(Speed),
    #[rasn(tag(explicit(context, 109)))]
    UM109Speed(Speed),
    #[rasn(tag(explicit(context, 110)))]
    UM110SpeedSpeed(SpeedSpeed),
    #[rasn(tag(explicit(context, 111)))]
    UM111Speed(Speed),
    #[rasn(tag(explicit(context, 112)))]
    UM112Speed(Speed),
    #[rasn(tag(explicit(context, 113)))]
    UM113Speed(Speed),
    #[rasn(tag(explicit(context, 114)))]
    UM114Speed(Speed),
    #[rasn(tag(explicit(context, 115)))]
    UM115Speed(Speed),
    #[rasn(tag(explicit(context, 116)))]
    UM116Null(()),
    #[rasn(tag(explicit(context, 117)))]
    UM117UnitNameFrequency(UnitNameFrequency),
    #[rasn(tag(explicit(context, 118)))]
    UM118PositionUnitNameFrequency(PositionUnitNameFrequency),
    #[rasn(tag(explicit(context, 119)))]
    UM119TimeUnitNameFrequency(TimeUnitNameFrequency),
    #[rasn(tag(explicit(context, 120)))]
    UM120UnitNameFrequency(UnitNameFrequency),
    #[rasn(tag(explicit(context, 121)))]
    UM121PositionUnitNameFrequency(PositionUnitNameFrequency),
    #[rasn(tag(explicit(context, 122)))]
    UM122TimeUnitNameFrequency(TimeUnitNameFrequency),
    #[rasn(tag(explicit(context, 123)))]
    UM123Code(Code),
    #[rasn(tag(explicit(context, 124)))]
    UM124Null(()),
    #[rasn(tag(explicit(context, 125)))]
    UM125Null(()),
    #[rasn(tag(explicit(context, 126)))]
    UM126Null(()),
    #[rasn(tag(explicit(context, 127)))]
    UM127Null(()),
    #[rasn(tag(explicit(context, 128)))]
    UM128Level(Level),
    #[rasn(tag(explicit(context, 129)))]
    UM129Level(Level),
    #[rasn(tag(explicit(context, 130)))]
    UM130Position(Position),
    #[rasn(tag(explicit(context, 131)))]
    UM131Null(()),
    #[rasn(tag(explicit(context, 132)))]
    UM132Null(()),
    #[rasn(tag(explicit(context, 133)))]
    UM133Null(()),
    #[rasn(tag(explicit(context, 134)))]
    UM134SpeedTypeSpeedTypeSpeedType(SpeedTypeSpeedTypeSpeedType),
    #[rasn(tag(explicit(context, 135)))]
    UM135Null(()),
    #[rasn(tag(explicit(context, 136)))]
    UM136Null(()),
    #[rasn(tag(explicit(context, 137)))]
    UM137Null(()),
    #[rasn(tag(explicit(context, 138)))]
    UM138Null(()),
    #[rasn(tag(explicit(context, 139)))]
    UM139Null(()),
    #[rasn(tag(explicit(context, 140)))]
    UM140Null(()),
    #[rasn(tag(explicit(context, 141)))]
    UM141Null(()),
    #[rasn(tag(explicit(context, 142)))]
    UM142Null(()),
    #[rasn(tag(explicit(context, 143)))]
    UM143Null(()),
    #[rasn(tag(explicit(context, 144)))]
    UM144Null(()),
    #[rasn(tag(explicit(context, 145)))]
    UM145Null(()),
    #[rasn(tag(explicit(context, 146)))]
    UM146Null(()),
    #[rasn(tag(explicit(context, 147)))]
    UM147Null(()),
    #[rasn(tag(explicit(context, 148)))]
    UM148Level(Level),
    #[rasn(tag(explicit(context, 149)))]
    UM149LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 150)))]
    UM150LevelTime(LevelTime),
    #[rasn(tag(explicit(context, 151)))]
    UM151Speed(Speed),
    #[rasn(tag(explicit(context, 152)))]
    UM152DistanceSpecifiedDirection(DistanceSpecifiedDirection),
    #[rasn(tag(explicit(context, 153)))]
    UM153Altimeter(Altimeter),
    #[rasn(tag(explicit(context, 154)))]
    UM154Null(()),
    #[rasn(tag(explicit(context, 155)))]
    UM155Position(Position),
    #[rasn(tag(explicit(context, 156)))]
    UM156Null(()),
    #[rasn(tag(explicit(context, 157)))]
    UM157Frequency(Frequency),
    #[rasn(tag(explicit(context, 158)))]
    UM158AtisCode(ATISCode),
    #[rasn(tag(explicit(context, 159)))]
    UM159ErrorInformation(ErrorInformation),
    #[rasn(tag(explicit(context, 160)))]
    UM160Facility(Facility),
    #[rasn(tag(explicit(context, 161)))]
    UM161Null(()),
    #[rasn(tag(explicit(context, 162)))]
    UM162Null(()),
    #[rasn(tag(explicit(context, 163)))]
    UM163FacilityDesignation(FacilityDesignation),
    #[rasn(tag(explicit(context, 164)))]
    UM164Null(()),
    #[rasn(tag(explicit(context, 165)))]
    UM165Null(()),
    #[rasn(tag(explicit(context, 166)))]
    UM166TrafficType(TrafficType),
    #[rasn(tag(explicit(context, 167)))]
    UM167Null(()),
    #[rasn(tag(explicit(context, 168)))]
    UM168Null(()),
    #[rasn(tag(explicit(context, 169)))]
    UM169FreeText(FreeText),
    #[rasn(tag(explicit(context, 170)))]
    UM170FreeText(FreeText),
    #[rasn(tag(explicit(context, 171)))]
    UM171VerticalRate(VerticalRate),
    #[rasn(tag(explicit(context, 172)))]
    UM172VerticalRate(VerticalRate),
    #[rasn(tag(explicit(context, 173)))]
    UM173VerticalRate(VerticalRate),
    #[rasn(tag(explicit(context, 174)))]
    UM174VerticalRate(VerticalRate),
    #[rasn(tag(explicit(context, 175)))]
    UM175Level(Level),
    #[rasn(tag(explicit(context, 176)))]
    UM176Null(()),
    #[rasn(tag(explicit(context, 177)))]
    UM177Null(()),
    #[rasn(tag(explicit(context, 178)))]
    UM178Null(()),
    #[rasn(tag(explicit(context, 179)))]
    UM179Null(()),
    #[rasn(tag(explicit(context, 180)))]
    UM180LevelLevel(LevelLevel),
    #[rasn(tag(explicit(context, 181)))]
    UM181ToFromPosition(ToFromPosition),
    #[rasn(tag(explicit(context, 182)))]
    UM182Null(()),
    #[rasn(tag(explicit(context, 183)))]
    UM183FreeText(FreeText),
    #[rasn(tag(explicit(context, 184)))]
    UM184TimeToFromPosition(TimeToFromPosition),
    #[rasn(tag(explicit(context, 185)))]
    UM185PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 186)))]
    UM186PositionLevel(PositionLevel),
    #[rasn(tag(explicit(context, 187)))]
    UM187FreeText(FreeText),
    #[rasn(tag(explicit(context, 188)))]
    UM188PositionSpeed(PositionSpeed),
    #[rasn(tag(explicit(context, 189)))]
    UM189Speed(Speed),
    #[rasn(tag(explicit(context, 190)))]
    UM190Degrees(Degrees),
    #[rasn(tag(explicit(context, 191)))]
    UM191Null(()),
    #[rasn(tag(explicit(context, 192)))]
    UM192LevelTime(LevelTime),
    #[rasn(tag(explicit(context, 193)))]
    UM193Null(()),
    #[rasn(tag(explicit(context, 194)))]
    UM194FreeText(FreeText),
    #[rasn(tag(explicit(context, 195)))]
    UM195FreeText(FreeText),
    #[rasn(tag(explicit(context, 196)))]
    UM196FreeText(FreeText),
    #[rasn(tag(explicit(context, 197)))]
    UM197FreeText(FreeText),
    #[rasn(tag(explicit(context, 198)))]
    UM198FreeText(FreeText),
    #[rasn(tag(explicit(context, 199)))]
    UM199FreeText(FreeText),
    #[rasn(tag(explicit(context, 200)))]
    UM200Null(()),
    #[rasn(tag(explicit(context, 201)))]
    UM201Null(()),
    #[rasn(tag(explicit(context, 202)))]
    UM202Null(()),
    #[rasn(tag(explicit(context, 203)))]
    UM203FreeText(FreeText),
    #[rasn(tag(explicit(context, 204)))]
    UM204FreeText(FreeText),
    #[rasn(tag(explicit(context, 205)))]
    UM205FreeText(FreeText),
    #[rasn(tag(explicit(context, 206)))]
    UM206FreeText(FreeText),
    #[rasn(tag(explicit(context, 207)))]
    UM207FreeText(FreeText),
    #[rasn(tag(explicit(context, 208)))]
    UM208FreeText(FreeText),
    #[rasn(tag(explicit(context, 209)))]
    UM209LevelPosition(LevelPosition),
    #[rasn(tag(explicit(context, 210)))]
    UM210Position(Position),
    #[rasn(tag(explicit(context, 211)))]
    UM211Null(()),
    #[rasn(tag(explicit(context, 212)))]
    UM212FacilityDesignationATISCode(FacilityDesignationATISCode),
    #[rasn(tag(explicit(context, 213)))]
    UM213FacilityDesignationAltimeter(FacilityDesignationAltimeter),
    #[rasn(tag(explicit(context, 214)))]
    UM214RunwayRVR(RunwayRVR),
    #[rasn(tag(explicit(context, 215)))]
    UM215DirectionDegrees(DirectionDegrees),
    #[rasn(tag(explicit(context, 216)))]
    UM216Null(()),
    #[rasn(tag(explicit(context, 217)))]
    UM217Null(()),
    #[rasn(tag(explicit(context, 218)))]
    UM218Null(()),
    #[rasn(tag(explicit(context, 219)))]
    UM219Level(Level),
    #[rasn(tag(explicit(context, 220)))]
    UM220Level(Level),
    #[rasn(tag(explicit(context, 221)))]
    UM221Degrees(Degrees),
    #[rasn(tag(explicit(context, 222)))]
    UM222Null(()),
    #[rasn(tag(explicit(context, 223)))]
    UM223Null(()),
    #[rasn(tag(explicit(context, 224)))]
    UM224Null(()),
    #[rasn(tag(explicit(context, 225)))]
    UM225Null(()),
    #[rasn(tag(explicit(context, 226)))]
    UM226Time(Time),
    #[rasn(tag(explicit(context, 227)))]
    UM227Null(()),
    #[rasn(tag(explicit(context, 228)))]
    UM228Position(Position),
    #[rasn(tag(explicit(context, 229)))]
    UM229Null(()),
    #[rasn(tag(explicit(context, 230)))]
    UM230Null(()),
    #[rasn(tag(explicit(context, 231)))]
    UM231Null(()),
    #[rasn(tag(explicit(context, 232)))]
    UM232Null(()),
    #[rasn(tag(explicit(context, 233)))]
    UM233Null(()),
    #[rasn(tag(explicit(context, 234)))]
    UM234Null(()),
    #[rasn(tag(explicit(context, 235)))]
    UM235Null(()),
    #[rasn(tag(explicit(context, 236)))]
    UM236Null(()),
    #[rasn(extension_addition, tag(explicit(context, 237)))]
    UM237Null(()),
}

/// unit = Hectopascal, Range (750.0..1250.0), resolution = 0.1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1"))]
pub struct ATISCode(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2..=7"))]
pub struct ATSRouteDesignator(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtwaLongTrackWaypoint {
    #[rasn(tag(explicit(context, 0)))]
    pub position: Position,
    #[rasn(tag(explicit(context, 1)), identifier = "aTWDistance")]
    pub a_twdistance: AtwDistance,
    #[rasn(tag(explicit(context, 2)))]
    pub speed: Option<Speed>,
    #[rasn(tag(explicit(context, 3)), identifier = "aTWLevels")]
    pub a_twlevels: Option<ATWLevelSequence>,
}

impl AtwaLongTrackWaypoint {
    pub fn new(
        position: Position,
        a_twdistance: AtwDistance,
        speed: Option<Speed>,
        a_twlevels: Option<ATWLevelSequence>,
    ) -> Self {
        Self {
            position,
            a_twdistance,
            speed,
            a_twlevels,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtwDistance {
    #[rasn(identifier = "atwDistanceTolerance")]
    pub atw_distance_tolerance: AtwDistanceTolerance,
    pub distance: Distance,
}

impl AtwDistance {
    pub fn new(atw_distance_tolerance: AtwDistanceTolerance, distance: Distance) -> Self {
        Self {
            atw_distance_tolerance,
            distance,
        }
    }
}
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum AtwDistanceTolerance {
    Plus = 0,
    Minus = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct AtwLevel {
    pub atw: AtwLevelTolerance,
    pub level: Level,
}

impl AtwLevel {
    pub fn new(atw: AtwLevelTolerance, level: Level) -> Self {
        Self { atw, level }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=2"))]
pub struct ATWLevelSequence(pub SequenceOf<AtwLevel>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum AtwLevelTolerance {
    At = 0,
    AtorAbove = 1,
    AtorBelow = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct AircraftAddress(pub FixedBitString<24usize>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2..=8"))]
pub struct AircraftFlightIdentification(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4"))]
pub struct Airport(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Altimeter {
    #[rasn(tag(explicit(context, 0)))]
    AltimeterEnglish(AltimeterEnglish),
    #[rasn(tag(explicit(context, 1)))]
    AltimeterMetric(AltimeterMetric),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("2200..=3200"))]
pub struct AltimeterEnglish(pub u16);

/// unit = Inches Mercury, Range (22.00 .. 32.00), resolution = 0.01
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("7500..=12500"))]
pub struct AltimeterMetric(pub u16);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ClearanceType {
    NoneSpecified = 0,
    Approach = 1,
    Departure = 2,
    Further = 3,
    #[rasn(identifier = "start-up")]
    StartUp = 4,
    Pushback = 5,
    Taxi = 6,
    #[rasn(identifier = "take-off")]
    TakeOff = 7,
    Landing = 8,
    Oceanic = 9,
    #[rasn(identifier = "en-route")]
    EnRoute = 10,
    Downstream = 11,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4"))]
pub struct Code(pub SequenceOf<CodeOctalDigit>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct CodeOctalDigit(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ControlledTime {
    pub time: Time,
    #[rasn(identifier = "timeTolerance")]
    pub time_tolerance: TimeTolerance,
}

impl ControlledTime {
    pub fn new(time: Time, time_tolerance: TimeTolerance) -> Self {
        Self {
            time,
            time_tolerance,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}

impl Date {
    pub fn new(year: Year, month: Month, day: Day) -> Self {
        Self { year, month, day }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DateTimeGroup {
    pub date: Date,
    pub timehhmmss: Timehhmmss,
}

impl DateTimeGroup {
    pub fn new(date: Date, timehhmmss: Timehhmmss) -> Self {
        Self { date, timehhmmss }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=31"))]
pub struct Day(pub u8);

/// unit = Day, Range (1..31), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=20"))]
pub struct DegreeIncrement(pub u8);

/// unit = Degree, Range (1..20), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Degrees {
    #[rasn(tag(explicit(context, 0)))]
    DegreesMagnetic(DegreesMagnetic),
    #[rasn(tag(explicit(context, 1)))]
    DegreesTrue(DegreesTrue),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=360"))]
pub struct DegreesMagnetic(pub u16);

/// unit = degree, Range (1..360), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=360"))]
pub struct DegreesTrue(pub u16);

/// unit = degree, Range (1..360), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DepartureClearance {
    #[rasn(tag(explicit(context, 0)), identifier = "aircraftFlightIdentification")]
    pub aircraft_flight_identification: AircraftFlightIdentification,
    #[rasn(tag(explicit(context, 1)), identifier = "clearanceLimit")]
    pub clearance_limit: Position,
    #[rasn(tag(explicit(context, 2)), identifier = "flightInformation")]
    pub flight_information: Option<FlightInformation>,
    #[rasn(tag(explicit(context, 3)), identifier = "furtherInstructions")]
    pub further_instructions: Option<FurtherInstructions>,
}

impl DepartureClearance {
    pub fn new(
        aircraft_flight_identification: AircraftFlightIdentification,
        clearance_limit: Position,
        flight_information: Option<FlightInformation>,
        further_instructions: Option<FurtherInstructions>,
    ) -> Self {
        Self {
            aircraft_flight_identification,
            clearance_limit,
            flight_information,
            further_instructions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=150"))]
pub struct DepartureMinimumInterval(pub u8);

/// unit = Minute, Range (0.1..15.0), resolution = 0.1
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum Direction {
    Left = 0,
    Right = 1,
    EitherSide = 2,
    North = 3,
    South = 4,
    East = 5,
    West = 6,
    NorthEast = 7,
    NorthWest = 8,
    SouthEast = 9,
    SouthWest = 10,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DirectionDegrees {
    pub direction: Direction,
    pub degrees: Degrees,
}

impl DirectionDegrees {
    pub fn new(direction: Direction, degrees: Degrees) -> Self {
        Self { direction, degrees }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Distance {
    #[rasn(tag(explicit(context, 0)))]
    DistanceNm(DistanceNm),
    #[rasn(tag(explicit(context, 1)))]
    DistanceKm(DistanceKm),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=8000"))]
pub struct DistanceKm(pub u16);

/// unit = Kilometer, Range (0..2000), resolution = 0.25
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=9999"))]
pub struct DistanceNm(pub u16);

/// unit = Nautical Mile, Range (0..999.9), resolution = 0.1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum DistanceSpecified {
    #[rasn(tag(explicit(context, 0)))]
    DistanceSpecifiedNm(DistanceSpecifiedNm),
    #[rasn(tag(explicit(context, 1)))]
    DistanceSpecifiedKm(DistanceSpecifiedKm),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DistanceSpecifiedDirection {
    #[rasn(identifier = "distanceSpecified")]
    pub distance_specified: DistanceSpecified,
    pub direction: Direction,
}

impl DistanceSpecifiedDirection {
    pub fn new(distance_specified: DistanceSpecified, direction: Direction) -> Self {
        Self {
            distance_specified,
            direction,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct DistanceSpecifiedDirectionTime {
    #[rasn(identifier = "distanceSpecifiedDirection")]
    pub distance_specified_direction: DistanceSpecifiedDirection,
    pub time: Time,
}

impl DistanceSpecifiedDirectionTime {
    pub fn new(distance_specified_direction: DistanceSpecifiedDirection, time: Time) -> Self {
        Self {
            distance_specified_direction,
            time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=500"))]
pub struct DistanceSpecifiedKm(pub u16);

/// unit = Kilometer, Range (1..500), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=250"))]
pub struct DistanceSpecifiedNm(pub u8);

/// unit = Nautical Mile, Range (1..250), resolution = 1
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ErrorInformation {
    UnrecognizedMsgReferenceNumber = 0,
    LogicalAcknowledgmentNotAccepted = 1,
    InsufficientResources = 2,
    InvalidMessageElementCombination = 3,
    InvalidMessageElement = 4,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Facility {
    #[rasn(tag(explicit(context, 0)))]
    NoFacility(()),
    #[rasn(tag(explicit(context, 1)))]
    FacilityDesignation(FacilityDesignation),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4..=8"))]
pub struct FacilityDesignation(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FacilityDesignationATISCode {
    #[rasn(identifier = "facilityDesignation")]
    pub facility_designation: FacilityDesignation,
    #[rasn(identifier = "aTISCode")]
    pub a_tiscode: ATISCode,
}

impl FacilityDesignationATISCode {
    pub fn new(facility_designation: FacilityDesignation, a_tiscode: ATISCode) -> Self {
        Self {
            facility_designation,
            a_tiscode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FacilityDesignationAltimeter {
    #[rasn(identifier = "facilityDesignation")]
    pub facility_designation: FacilityDesignation,
    pub altimeter: Altimeter,
}

impl FacilityDesignationAltimeter {
    pub fn new(facility_designation: FacilityDesignation, altimeter: Altimeter) -> Self {
        Self {
            facility_designation,
            altimeter,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum FacilityFunction {
    Center = 0,
    Approach = 1,
    Tower = 2,
    Final = 3,
    GroundControl = 4,
    ClearanceDelivery = 5,
    Departure = 6,
    Control = 7,
    Radio = 8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("3..=18"))]
pub struct FacilityName(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=5"))]
pub struct Fix(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FixName {
    #[rasn(tag(explicit(context, 0)))]
    pub name: Fix,
    #[rasn(tag(explicit(context, 1)))]
    pub latlon: Option<LatitudeLongitude>,
}

impl FixName {
    pub fn new(name: Fix, latlon: Option<LatitudeLongitude>) -> Self {
        Self { name, latlon }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum FlightInformation {
    #[rasn(tag(explicit(context, 0)))]
    RouteOfFlight(RouteInformation),
    #[rasn(tag(explicit(context, 1)))]
    LevelsOfFlight(LevelsOfFlight),
    #[rasn(tag(explicit(context, 2)))]
    RouteAndLevels(RouteAndLevels),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=256"))]
pub struct FreeText(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Frequency {
    #[rasn(tag(explicit(context, 0)))]
    FrequencyHf(FrequencyHf),
    #[rasn(tag(explicit(context, 1)))]
    FrequencyVhf(FrequencyVhf),
    #[rasn(tag(explicit(context, 2)))]
    FrequencyUhf(FrequencyUhf),
    #[rasn(tag(explicit(context, 3)))]
    FrequencySatChannel(FrequencySatChannel),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("2850..=28000"))]
pub struct FrequencyHf(pub u16);

/// unit = Kilohertz, Range (2850..28000), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("12"))]
pub struct FrequencySatChannel(pub NumericString);

/// Corresponds to a 12 digit telephone number
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("9000..=15999"))]
pub struct FrequencyUhf(pub u16);

/// unit = Megahertz, Range (225.000..399.975), resolution = 0.025
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("23600..=27398"))]
pub struct FrequencyVhf(pub u16);

/// unit = Megahertz, Range (118.000..136.990), resolution = 0.005
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct FurtherInstructions {
    #[rasn(tag(explicit(context, 0)))]
    pub code: Option<Code>,
    #[rasn(tag(explicit(context, 1)), identifier = "frequencyDeparture")]
    pub frequency_departure: Option<UnitNameFrequency>,
    #[rasn(tag(explicit(context, 2)), identifier = "clearanceExpiryTime")]
    pub clearance_expiry_time: Option<Time>,
    #[rasn(tag(explicit(context, 3)), identifier = "airportDeparture")]
    pub airport_departure: Option<Airport>,
    #[rasn(tag(explicit(context, 4)), identifier = "airportDestination")]
    pub airport_destination: Option<Airport>,
    #[rasn(tag(explicit(context, 5)), identifier = "timeDeparture")]
    pub time_departure: Option<TimeDeparture>,
    #[rasn(tag(explicit(context, 6)), identifier = "runwayDeparture")]
    pub runway_departure: Option<Runway>,
    #[rasn(tag(explicit(context, 7)), identifier = "revisionNumber")]
    pub revision_number: Option<RevisionNumber>,
    #[rasn(tag(explicit(context, 8)), identifier = "aTISCode")]
    pub a_tiscode: Option<ATISCode>,
}

impl FurtherInstructions {
    pub fn new(
        code: Option<Code>,
        frequency_departure: Option<UnitNameFrequency>,
        clearance_expiry_time: Option<Time>,
        airport_departure: Option<Airport>,
        airport_destination: Option<Airport>,
        time_departure: Option<TimeDeparture>,
        runway_departure: Option<Runway>,
        revision_number: Option<RevisionNumber>,
        a_tiscode: Option<ATISCode>,
    ) -> Self {
        Self {
            code,
            frequency_departure,
            clearance_expiry_time,
            airport_departure,
            airport_destination,
            time_departure,
            runway_departure,
            revision_number,
            a_tiscode,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct HoldClearance {
    #[rasn(tag(explicit(context, 0)))]
    pub position: Position,
    #[rasn(tag(explicit(context, 1)))]
    pub level: Level,
    #[rasn(tag(explicit(context, 2)))]
    pub degrees: Degrees,
    #[rasn(tag(explicit(context, 3)))]
    pub direction: Direction,
    #[rasn(tag(explicit(context, 4)), identifier = "legType")]
    pub leg_type: Option<LegType>,
}

impl HoldClearance {
    pub fn new(
        position: Position,
        level: Level,
        degrees: Degrees,
        direction: Direction,
        leg_type: Option<LegType>,
    ) -> Self {
        Self {
            position,
            level,
            degrees,
            direction,
            leg_type,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct HoldAtWaypoint {
    #[rasn(tag(explicit(context, 0)))]
    pub position: Position,
    #[rasn(tag(explicit(context, 1)))]
    pub holdatwaypointspeedlow: Option<Speed>,
    #[rasn(tag(explicit(context, 2)), identifier = "aTWlevel")]
    pub a_twlevel: Option<AtwLevel>,
    #[rasn(tag(explicit(context, 3)))]
    pub holdatwaypointspeedhigh: Option<Speed>,
    #[rasn(tag(explicit(context, 4)))]
    pub direction: Option<Direction>,
    #[rasn(tag(explicit(context, 5)))]
    pub degrees: Option<Degrees>,
    #[rasn(tag(explicit(context, 6)), identifier = "eFCtime")]
    pub e_fctime: Option<Time>,
    #[rasn(tag(explicit(context, 7)))]
    pub legtype: Option<LegType>,
}

impl HoldAtWaypoint {
    pub fn new(
        position: Position,
        holdatwaypointspeedlow: Option<Speed>,
        a_twlevel: Option<AtwLevel>,
        holdatwaypointspeedhigh: Option<Speed>,
        direction: Option<Direction>,
        degrees: Option<Degrees>,
        e_fctime: Option<Time>,
        legtype: Option<LegType>,
    ) -> Self {
        Self {
            position,
            holdatwaypointspeedlow,
            a_twlevel,
            holdatwaypointspeedhigh,
            direction,
            degrees,
            e_fctime,
            legtype,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=100"))]
pub struct Humidity(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum Icing {
    Reserved = 0,
    Light = 1,
    Moderate = 2,
    Severe = 3,
}

/// unit = Percent humidity, Range (0..100), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct InterceptCourseFrom {
    #[rasn(identifier = "fromSelection")]
    pub from_selection: InterceptCourseFromSelection,
    pub degrees: Degrees,
}

impl InterceptCourseFrom {
    pub fn new(from_selection: InterceptCourseFromSelection, degrees: Degrees) -> Self {
        Self {
            from_selection,
            degrees,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum InterceptCourseFromSelection {
    #[rasn(tag(explicit(context, 0)))]
    PublishedIdentifier(PublishedIdentifier),
    #[rasn(tag(explicit(context, 1)))]
    LatitudeLongitude(LatitudeLongitude),
    #[rasn(tag(explicit(context, 2)))]
    PlaceBearingPlaceBearing(PlaceBearingPlaceBearing),
    #[rasn(tag(explicit(context, 3)))]
    PlaceBearingDistance(PlaceBearingDistance),
}

/// unit = Minute, Range (0..59), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LatLonReportingPoints {
    #[rasn(tag(explicit(context, 0)))]
    LatitudeReportingPoints(LatitudeReportingPoints),
    #[rasn(tag(explicit(context, 1)))]
    LongitudeReportingPoints(LongitudeReportingPoints),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=59"))]
pub struct LatLonWholeMinutes(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Latitude {
    #[rasn(identifier = "latitudeType")]
    pub latitude_type: LatitudeType,
    #[rasn(identifier = "latitudeDirection")]
    pub latitude_direction: LatitudeDirection,
}

impl Latitude {
    pub fn new(latitude_type: LatitudeType, latitude_direction: LatitudeDirection) -> Self {
        Self {
            latitude_type,
            latitude_direction,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=90000"))]
pub struct LatitudeDegrees(pub u32);

/// unit = Degree, Range (0..90), resolution = 0.001
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LatitudeDegreesMinutes {
    #[rasn(identifier = "latitudeWholeDegrees")]
    pub latitude_whole_degrees: LatitudeWholeDegrees,
    #[rasn(identifier = "minutesLatLon")]
    pub minutes_lat_lon: MinutesLatLon,
}

impl LatitudeDegreesMinutes {
    pub fn new(
        latitude_whole_degrees: LatitudeWholeDegrees,
        minutes_lat_lon: MinutesLatLon,
    ) -> Self {
        Self {
            latitude_whole_degrees,
            minutes_lat_lon,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LatitudeDegreesMinutesSeconds {
    #[rasn(identifier = "latitudeWholeDegrees")]
    pub latitude_whole_degrees: LatitudeWholeDegrees,
    #[rasn(identifier = "latlonWholeMinutes")]
    pub latlon_whole_minutes: LatLonWholeMinutes,
    #[rasn(identifier = "secondsLatLon")]
    pub seconds_lat_lon: SecondsLatLon,
}

impl LatitudeDegreesMinutesSeconds {
    pub fn new(
        latitude_whole_degrees: LatitudeWholeDegrees,
        latlon_whole_minutes: LatLonWholeMinutes,
        seconds_lat_lon: SecondsLatLon,
    ) -> Self {
        Self {
            latitude_whole_degrees,
            latlon_whole_minutes,
            seconds_lat_lon,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum LatitudeDirection {
    North = 0,
    South = 1,
}

/// unit = Degree, Range (0..89), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LatitudeLongitude {
    #[rasn(tag(explicit(context, 0)))]
    pub latitude: Option<Latitude>,
    #[rasn(tag(explicit(context, 1)))]
    pub longitude: Option<Longitude>,
}

impl LatitudeLongitude {
    pub fn new(latitude: Option<Latitude>, longitude: Option<Longitude>) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LatitudeReportingPoints {
    #[rasn(identifier = "latitudeDirection")]
    pub latitude_direction: LatitudeDirection,
    #[rasn(identifier = "latitudeDegrees")]
    pub latitude_degrees: LatitudeDegrees,
}

impl LatitudeReportingPoints {
    pub fn new(latitude_direction: LatitudeDirection, latitude_degrees: LatitudeDegrees) -> Self {
        Self {
            latitude_direction,
            latitude_degrees,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LatitudeType {
    #[rasn(tag(explicit(context, 0)))]
    LatitudeDegrees(LatitudeDegrees),
    #[rasn(tag(explicit(context, 1)))]
    LatitudeDegreesMinutes(LatitudeDegreesMinutes),
    #[rasn(tag(explicit(context, 2)))]
    LatitudeDMS(LatitudeDegreesMinutesSeconds),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=89"))]
pub struct LatitudeWholeDegrees(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LegDistance {
    #[rasn(tag(explicit(context, 0)))]
    LegDistanceEnglish(LegDistanceEnglish),
    #[rasn(tag(explicit(context, 1)))]
    LegDistanceMetric(LegDistanceMetric),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=50"))]
pub struct LegDistanceEnglish(pub u8);

/// unit = Nautical Mile, Range (0..50), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=128"))]
pub struct LegDistanceMetric(pub u8);

/// unit = Kilometer, Range (1..128), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=10"))]
pub struct LegTime(pub u8);

/// unit = Minute, Range (0..10), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LegType {
    #[rasn(tag(explicit(context, 0)))]
    LegDistance(LegDistance),
    #[rasn(tag(explicit(context, 1)))]
    LegTime(LegTime),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Level {
    #[rasn(tag(explicit(context, 0)))]
    SingleLevel(LevelType),
    #[rasn(size("2"), tag(explicit(context, 1)))]
    BlockLevel(SequenceOf<LevelType>),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-60..=7000"))]
pub struct LevelFeet(pub i16);

/// unit = Feet, Range (-600..70000), resolution = 10
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("30..=700"))]
pub struct LevelFlightLevel(pub u16);

/// unit = Level (100 Feet), Range (030..700), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("100..=2500"))]
pub struct LevelFlightLevelMetric(pub u16);

/// unit = Level (10 Meters), Range (100..2500), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct LevelLevel(pub SequenceOf<Level>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-30..=25000"))]
pub struct LevelMeters(pub i16);

/// unit = Meter, Range (-30..25000), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LevelPosition {
    pub level: Level,
    pub position: Position,
}

impl LevelPosition {
    pub fn new(level: Level, position: Position) -> Self {
        Self { level, position }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LevelProcedureName {
    pub level: Level,
    #[rasn(identifier = "procedureName")]
    pub procedure_name: ProcedureName,
}

impl LevelProcedureName {
    pub fn new(level: Level, procedure_name: ProcedureName) -> Self {
        Self {
            level,
            procedure_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LevelSpeed {
    pub level: Level,
    pub speed: SpeedSpeed,
}

impl LevelSpeed {
    pub fn new(level: Level, speed: SpeedSpeed) -> Self {
        Self { level, speed }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LevelSpeedSpeed {
    pub level: Level,
    pub speeds: SpeedSpeed,
}

impl LevelSpeedSpeed {
    pub fn new(level: Level, speeds: SpeedSpeed) -> Self {
        Self { level, speeds }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LevelTime {
    pub level: Level,
    pub time: Time,
}

impl LevelTime {
    pub fn new(level: Level, time: Time) -> Self {
        Self { level, time }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LevelType {
    #[rasn(tag(explicit(context, 0)))]
    LevelFeet(LevelFeet),
    #[rasn(tag(explicit(context, 1)))]
    LevelMeters(LevelMeters),
    #[rasn(tag(explicit(context, 2)))]
    LevelFlightLevel(LevelFlightLevel),
    #[rasn(tag(explicit(context, 3)))]
    LevelFlightLevelMetric(LevelFlightLevelMetric),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LevelsOfFlight {
    #[rasn(tag(explicit(context, 0)))]
    Level(Level),
    #[rasn(tag(explicit(context, 1)))]
    ProcedureName(ProcedureName),
    #[rasn(tag(explicit(context, 2)))]
    LevelProcedureName(LevelProcedureName),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum LogicalAck {
    Required = 0,
    NotRequired = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Longitude {
    #[rasn(identifier = "longitudeType")]
    pub longitude_type: LongitudeType,
    #[rasn(identifier = "longitudeDirection")]
    pub longitude_direction: LongitudeDirection,
}

impl Longitude {
    pub fn new(longitude_type: LongitudeType, longitude_direction: LongitudeDirection) -> Self {
        Self {
            longitude_type,
            longitude_direction,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=180000"))]
pub struct LongitudeDegrees(pub u32);

/// unit = Degree, Range (0..180), resolution = 0.001
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LongitudeDegreesMinutes {
    #[rasn(identifier = "longitudeWholeDegrees")]
    pub longitude_whole_degrees: LongitudeWholeDegrees,
    #[rasn(identifier = "minutesLatLon")]
    pub minutes_lat_lon: MinutesLatLon,
}

impl LongitudeDegreesMinutes {
    pub fn new(
        longitude_whole_degrees: LongitudeWholeDegrees,
        minutes_lat_lon: MinutesLatLon,
    ) -> Self {
        Self {
            longitude_whole_degrees,
            minutes_lat_lon,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LongitudeDegreesMinutesSeconds {
    #[rasn(identifier = "longitudeWholeDegrees")]
    pub longitude_whole_degrees: LongitudeWholeDegrees,
    #[rasn(identifier = "latLonWholeMinutes")]
    pub lat_lon_whole_minutes: LatLonWholeMinutes,
    #[rasn(identifier = "secondsLatLon")]
    pub seconds_lat_lon: SecondsLatLon,
}

impl LongitudeDegreesMinutesSeconds {
    pub fn new(
        longitude_whole_degrees: LongitudeWholeDegrees,
        lat_lon_whole_minutes: LatLonWholeMinutes,
        seconds_lat_lon: SecondsLatLon,
    ) -> Self {
        Self {
            longitude_whole_degrees,
            lat_lon_whole_minutes,
            seconds_lat_lon,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum LongitudeDirection {
    East = 0,
    West = 1,
}

/// unit = Degree, Range (0..179), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct LongitudeReportingPoints {
    #[rasn(identifier = "longitudeDirection")]
    pub longitude_direction: LongitudeDirection,
    #[rasn(identifier = "longitudeDegrees")]
    pub longitude_degrees: LongitudeDegrees,
}

impl LongitudeReportingPoints {
    pub fn new(
        longitude_direction: LongitudeDirection,
        longitude_degrees: LongitudeDegrees,
    ) -> Self {
        Self {
            longitude_direction,
            longitude_degrees,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum LongitudeType {
    #[rasn(tag(explicit(context, 0)))]
    LongitudeDegrees(LongitudeDegrees),
    #[rasn(tag(explicit(context, 1)))]
    LongitudeDegreesMinutes(LongitudeDegreesMinutes),
    #[rasn(tag(explicit(context, 2)))]
    LongitudeDms(LongitudeDegreesMinutesSeconds),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=179"))]
pub struct LongitudeWholeDegrees(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=5999"))]
pub struct MinutesLatLon(pub u16);

/// unit = Minute, Range (0..59.99), resolution = 0.01
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=12"))]
pub struct Month(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=63"))]
pub struct MsgIdentificationNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=63"))]
pub struct MsgReferenceNumber(pub u8);

/// unit = 1 Month, Range (1..12), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Navaid {
    #[rasn(tag(explicit(context, 0)))]
    pub name: NavaidName,
    #[rasn(tag(explicit(context, 1)))]
    pub latlon: Option<LatitudeLongitude>,
}

impl Navaid {
    pub fn new(name: NavaidName, latlon: Option<LatitudeLongitude>) -> Self {
        Self { name, latlon }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=4"))]
pub struct NavaidName(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=1024"))]
pub struct PersonsOnBoard(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PlaceBearing {
    #[rasn(identifier = "publishedIdentifier")]
    pub published_identifier: PublishedIdentifier,
    pub degrees: Degrees,
}

impl PlaceBearing {
    pub fn new(published_identifier: PublishedIdentifier, degrees: Degrees) -> Self {
        Self {
            published_identifier,
            degrees,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PlaceBearingDistance {
    #[rasn(identifier = "publishedIdentifier")]
    pub published_identifier: PublishedIdentifier,
    pub degrees: Degrees,
    pub distance: Distance,
}

impl PlaceBearingDistance {
    pub fn new(
        published_identifier: PublishedIdentifier,
        degrees: Degrees,
        distance: Distance,
    ) -> Self {
        Self {
            published_identifier,
            degrees,
            distance,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct PlaceBearingPlaceBearing(pub SequenceOf<PlaceBearing>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Position {
    #[rasn(tag(explicit(context, 0)))]
    FixName(FixName),
    #[rasn(tag(explicit(context, 1)))]
    Navaid(Navaid),
    #[rasn(tag(explicit(context, 2)))]
    Airport(Airport),
    #[rasn(tag(explicit(context, 3)))]
    LatitudeLongitude(LatitudeLongitude),
    #[rasn(tag(explicit(context, 4)))]
    PlaceBearingDistance(PlaceBearingDistance),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionDegrees {
    pub position: Position,
    pub degrees: Degrees,
}

impl PositionDegrees {
    pub fn new(position: Position, degrees: Degrees) -> Self {
        Self { position, degrees }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionDistanceSpecifiedDirection {
    pub position: Position,
    #[rasn(identifier = "distanceSpecifiedDirection")]
    pub distance_specified_direction: DistanceSpecifiedDirection,
}

impl PositionDistanceSpecifiedDirection {
    pub fn new(
        position: Position,
        distance_specified_direction: DistanceSpecifiedDirection,
    ) -> Self {
        Self {
            position,
            distance_specified_direction,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionLevel {
    pub position: Position,
    pub level: Level,
}

impl PositionLevel {
    pub fn new(position: Position, level: Level) -> Self {
        Self { position, level }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionLevelLevel {
    pub position: Position,
    pub levels: LevelLevel,
}

impl PositionLevelLevel {
    pub fn new(position: Position, levels: LevelLevel) -> Self {
        Self { position, levels }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionLevelSpeed {
    pub positionlevel: PositionLevel,
    pub speed: Speed,
}

impl PositionLevelSpeed {
    pub fn new(positionlevel: PositionLevel, speed: Speed) -> Self {
        Self {
            positionlevel,
            speed,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct PositionPosition(pub SequenceOf<Position>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionProcedureName {
    pub position: Position,
    #[rasn(identifier = "procedureName")]
    pub procedure_name: ProcedureName,
}

impl PositionProcedureName {
    pub fn new(position: Position, procedure_name: ProcedureName) -> Self {
        Self {
            position,
            procedure_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionReport {
    #[rasn(tag(explicit(context, 0)))]
    pub position_current: Position,
    #[rasn(tag(explicit(context, 1)))]
    pub time_at_position_current: Time,
    #[rasn(tag(explicit(context, 2)))]
    pub level: Level,
    #[rasn(tag(explicit(context, 3)))]
    pub fix_next: Option<Position>,
    #[rasn(tag(explicit(context, 4)))]
    pub time_eta_at_fix_next: Option<Time>,
    #[rasn(tag(explicit(context, 5)))]
    pub fix_next_plus_one: Option<Position>,
    #[rasn(tag(explicit(context, 6)))]
    pub time_eta_at_destination: Option<Time>,
    #[rasn(tag(explicit(context, 7)), identifier = "remainingFuel")]
    pub remaining_fuel: Option<RemainingFuel>,
    #[rasn(tag(explicit(context, 8)))]
    pub temperature: Option<Temperature>,
    #[rasn(tag(explicit(context, 9)))]
    pub winds: Option<Winds>,
    #[rasn(tag(explicit(context, 10)))]
    pub turbulence: Option<Turbulence>,
    #[rasn(tag(explicit(context, 11)))]
    pub icing: Option<Icing>,
    #[rasn(tag(explicit(context, 12)))]
    pub speed: Option<Speed>,
    #[rasn(tag(explicit(context, 13)))]
    pub speed_ground: Option<SpeedGround>,
    #[rasn(tag(explicit(context, 14)), identifier = "verticalChange")]
    pub vertical_change: Option<VerticalChange>,
    #[rasn(tag(explicit(context, 15)), identifier = "trackAngle")]
    pub track_angle: Option<Degrees>,
    #[rasn(tag(explicit(context, 16)))]
    pub heading: Option<Degrees>,
    #[rasn(tag(explicit(context, 17)))]
    pub distance: Option<Distance>,
    #[rasn(tag(explicit(context, 18)))]
    pub humidity: Option<Humidity>,
    #[rasn(tag(explicit(context, 19)), identifier = "reportedWaypointPosition")]
    pub reported_waypoint_position: Option<Position>,
    #[rasn(tag(explicit(context, 20)), identifier = "reportedWaypointTime")]
    pub reported_waypoint_time: Option<Time>,
    #[rasn(tag(explicit(context, 21)), identifier = "reportedWaypointLevel")]
    pub reported_waypoint_level: Option<Level>,
}

impl PositionReport {
    pub fn new(
        position_current: Position,
        time_at_position_current: Time,
        level: Level,
        fix_next: Option<Position>,
        time_eta_at_fix_next: Option<Time>,
        fix_next_plus_one: Option<Position>,
        time_eta_at_destination: Option<Time>,
        remaining_fuel: Option<RemainingFuel>,
        temperature: Option<Temperature>,
        winds: Option<Winds>,
        turbulence: Option<Turbulence>,
        icing: Option<Icing>,
        speed: Option<Speed>,
        speed_ground: Option<SpeedGround>,
        vertical_change: Option<VerticalChange>,
        track_angle: Option<Degrees>,
        heading: Option<Degrees>,
        distance: Option<Distance>,
        humidity: Option<Humidity>,
        reported_waypoint_position: Option<Position>,
        reported_waypoint_time: Option<Time>,
        reported_waypoint_level: Option<Level>,
    ) -> Self {
        Self {
            position_current,
            time_at_position_current,
            level,
            fix_next,
            time_eta_at_fix_next,
            fix_next_plus_one,
            time_eta_at_destination,
            remaining_fuel,
            temperature,
            winds,
            turbulence,
            icing,
            speed,
            speed_ground,
            vertical_change,
            track_angle,
            heading,
            distance,
            humidity,
            reported_waypoint_position,
            reported_waypoint_time,
            reported_waypoint_level,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionRouteClearanceIndex {
    pub position: Position,
    #[rasn(identifier = "routeClearanceIndex")]
    pub route_clearance_index: RouteClearanceIndex,
}

impl PositionRouteClearanceIndex {
    pub fn new(position: Position, route_clearance_index: RouteClearanceIndex) -> Self {
        Self {
            position,
            route_clearance_index,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionSpeed {
    pub position: Position,
    pub speed: Speed,
}

impl PositionSpeed {
    pub fn new(position: Position, speed: Speed) -> Self {
        Self { position, speed }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionSpeedSpeed {
    pub position: Position,
    pub speeds: SpeedSpeed,
}

impl PositionSpeedSpeed {
    pub fn new(position: Position, speeds: SpeedSpeed) -> Self {
        Self { position, speeds }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionTime {
    pub position: Position,
    pub time: Time,
}

impl PositionTime {
    pub fn new(position: Position, time: Time) -> Self {
        Self { position, time }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionTimeLevel {
    #[rasn(identifier = "positionTime")]
    pub position_time: PositionTime,
    pub level: Level,
}

impl PositionTimeLevel {
    pub fn new(position_time: PositionTime, level: Level) -> Self {
        Self {
            position_time,
            level,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionTimeTime {
    pub position: Position,
    pub times: TimeTime,
}

impl PositionTimeTime {
    pub fn new(position: Position, times: TimeTime) -> Self {
        Self { position, times }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct PositionUnitNameFrequency {
    pub position: Position,
    pub unitname: UnitName,
    pub frequency: Frequency,
}

impl PositionUnitNameFrequency {
    pub fn new(position: Position, unitname: UnitName, frequency: Frequency) -> Self {
        Self {
            position,
            unitname,
            frequency,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=20"))]
pub struct Procedure(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ProcedureName {
    #[rasn(tag(explicit(context, 0)), identifier = "type")]
    pub r#type: ProcedureType,
    #[rasn(tag(explicit(context, 1)))]
    pub procedure: Procedure,
    #[rasn(tag(explicit(context, 2)))]
    pub transition: Option<ProcedureTransition>,
}

impl ProcedureName {
    pub fn new(
        r#type: ProcedureType,
        procedure: Procedure,
        transition: Option<ProcedureTransition>,
    ) -> Self {
        Self {
            r#type,
            procedure,
            transition,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=5"))]
pub struct ProcedureTransition(pub Ia5String);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum ProcedureType {
    Arrival = 0,
    Approach = 1,
    Departure = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum PublishedIdentifier {
    #[rasn(tag(explicit(context, 0)))]
    FixName(FixName),
    #[rasn(tag(explicit(context, 1)))]
    Navaid(Navaid),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RtaRequiredTimeArrival {
    #[rasn(tag(explicit(context, 0)))]
    pub position: Position,
    #[rasn(tag(explicit(context, 1)), identifier = "rTATime")]
    pub r_tatime: RtaTime,
    #[rasn(tag(explicit(context, 2)), identifier = "rTATolerance")]
    pub r_tatolerance: Option<RtaTolerance>,
}

impl RtaRequiredTimeArrival {
    pub fn new(position: Position, r_tatime: RtaTime, r_tatolerance: Option<RtaTolerance>) -> Self {
        Self {
            position,
            r_tatime,
            r_tatolerance,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RtaTime {
    pub time: Time,
    #[rasn(identifier = "timeTolerance")]
    pub time_tolerance: TimeTolerance,
}

impl RtaTime {
    pub fn new(time: Time, time_tolerance: TimeTolerance) -> Self {
        Self {
            time,
            time_tolerance,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=150"))]
pub struct RtaTolerance(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Rvr {
    #[rasn(tag(explicit(context, 0)))]
    RvrFeet(RVRFeet),
    #[rasn(tag(explicit(context, 1)))]
    RvrMeters(RVRMeters),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=6100"))]
pub struct RVRFeet(pub u16);

/// unit = Feet, Range (0..6100), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=1500"))]
pub struct RVRMeters(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct RemainingFuel(pub Time);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RemainingFuelPersonsOnBoard {
    #[rasn(identifier = "remainingFuel")]
    pub remaining_fuel: RemainingFuel,
    #[rasn(identifier = "personsOnBoard")]
    pub persons_on_board: PersonsOnBoard,
}

impl RemainingFuelPersonsOnBoard {
    pub fn new(remaining_fuel: RemainingFuel, persons_on_board: PersonsOnBoard) -> Self {
        Self {
            remaining_fuel,
            persons_on_board,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ReportingPoints {
    #[rasn(tag(explicit(context, 0)), identifier = "latLonReportingPoints")]
    pub lat_lon_reporting_points: LatLonReportingPoints,
    #[rasn(tag(explicit(context, 1)), identifier = "degreeIncrement")]
    pub degree_increment: Option<DegreeIncrement>,
}

impl ReportingPoints {
    pub fn new(
        lat_lon_reporting_points: LatLonReportingPoints,
        degree_increment: Option<DegreeIncrement>,
    ) -> Self {
        Self {
            lat_lon_reporting_points,
            degree_increment,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=16"))]
pub struct RevisionNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RouteAndLevels {
    #[rasn(identifier = "routeOfFlight")]
    pub route_of_flight: RouteInformation,
    #[rasn(identifier = "levelsOfFlight")]
    pub levels_of_flight: LevelsOfFlight,
}

impl RouteAndLevels {
    pub fn new(route_of_flight: RouteInformation, levels_of_flight: LevelsOfFlight) -> Self {
        Self {
            route_of_flight,
            levels_of_flight,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RouteClearance {
    #[rasn(tag(explicit(context, 0)), identifier = "airportDeparture")]
    pub airport_departure: Option<Airport>,
    #[rasn(tag(explicit(context, 1)), identifier = "airportDestination")]
    pub airport_destination: Option<Airport>,
    #[rasn(tag(explicit(context, 2)), identifier = "runwayDeparture")]
    pub runway_departure: Option<Runway>,
    #[rasn(tag(explicit(context, 3)), identifier = "procedureDeparture")]
    pub procedure_departure: Option<ProcedureName>,
    #[rasn(tag(explicit(context, 4)), identifier = "runwayArrival")]
    pub runway_arrival: Option<Runway>,
    #[rasn(tag(explicit(context, 5)), identifier = "procedureApproach")]
    pub procedure_approach: Option<ProcedureName>,
    #[rasn(tag(explicit(context, 6)), identifier = "procedureArrival")]
    pub procedure_arrival: Option<ProcedureName>,
    #[rasn(
        size("1..=128"),
        tag(explicit(context, 7)),
        identifier = "routeInformations"
    )]
    pub route_informations: Option<SequenceOf<RouteInformation>>,
    #[rasn(tag(explicit(context, 8)), identifier = "routeInformationAdditional")]
    pub route_information_additional: Option<RouteInformationAdditional>,
}

impl RouteClearance {
    pub fn new(
        airport_departure: Option<Airport>,
        airport_destination: Option<Airport>,
        runway_departure: Option<Runway>,
        procedure_departure: Option<ProcedureName>,
        runway_arrival: Option<Runway>,
        procedure_approach: Option<ProcedureName>,
        procedure_arrival: Option<ProcedureName>,
        route_informations: Option<SequenceOf<RouteInformation>>,
        route_information_additional: Option<RouteInformationAdditional>,
    ) -> Self {
        Self {
            airport_departure,
            airport_destination,
            runway_departure,
            procedure_departure,
            runway_arrival,
            procedure_approach,
            procedure_arrival,
            route_informations,
            route_information_additional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=2"))]
pub struct RouteClearanceIndex(pub u8);

/// Identifies the position of the RouteClearance data in the ASN.1 type for ATC
/// UplinkMessage, constrained Data, routeClearance Data ATC DownlinkMessage,
/// constrained Data, routeClearance Data.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum RouteInformation {
    #[rasn(tag(explicit(context, 0)))]
    PublishedIdentifier(PublishedIdentifier),
    #[rasn(tag(explicit(context, 1)))]
    LatitudeLongitude(LatitudeLongitude),
    #[rasn(tag(explicit(context, 2)))]
    PlaceBearingPlaceBearing(PlaceBearingPlaceBearing),
    #[rasn(tag(explicit(context, 3)))]
    PlaceBearingDistance(PlaceBearingDistance),
    #[rasn(tag(explicit(context, 4)))]
    AtsRouteDesignator(ATSRouteDesignator),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RouteInformationAdditional {
    #[rasn(
        size("1..=8"),
        tag(explicit(context, 0)),
        identifier = "aTWAlongTrackWaypoints"
    )]
    pub a_twalong_track_waypoints: Option<SequenceOf<AtwaLongTrackWaypoint>>,
    #[rasn(tag(explicit(context, 1)))]
    pub reportingpoints: Option<ReportingPoints>,
    #[rasn(
        size("1..=4"),
        tag(explicit(context, 2)),
        identifier = "interceptCourseFroms"
    )]
    pub intercept_course_froms: Option<SequenceOf<InterceptCourseFrom>>,
    #[rasn(
        size("1..=8"),
        tag(explicit(context, 3)),
        identifier = "holdAtWaypoints"
    )]
    pub hold_at_waypoints: Option<SequenceOf<HoldAtWaypoint>>,
    #[rasn(
        size("1..=32"),
        tag(explicit(context, 4)),
        identifier = "waypointSpeedLevels"
    )]
    pub waypoint_speed_levels: Option<SequenceOf<WaypointSpeedLevel>>,
    #[rasn(
        size("1..=32"),
        tag(explicit(context, 5)),
        identifier = "rTARequiredTimeArrivals"
    )]
    pub r_tarequired_time_arrivals: Option<SequenceOf<RtaRequiredTimeArrival>>,
}

impl RouteInformationAdditional {
    pub fn new(
        a_twalong_track_waypoints: Option<SequenceOf<AtwaLongTrackWaypoint>>,
        reportingpoints: Option<ReportingPoints>,
        intercept_course_froms: Option<SequenceOf<InterceptCourseFrom>>,
        hold_at_waypoints: Option<SequenceOf<HoldAtWaypoint>>,
        waypoint_speed_levels: Option<SequenceOf<WaypointSpeedLevel>>,
        r_tarequired_time_arrivals: Option<SequenceOf<RtaRequiredTimeArrival>>,
    ) -> Self {
        Self {
            a_twalong_track_waypoints,
            reportingpoints,
            intercept_course_froms,
            hold_at_waypoints,
            waypoint_speed_levels,
            r_tarequired_time_arrivals,
        }
    }
}

/// unit= Minute, Range (0.1..15.0), resolution = 0.1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Runway {
    pub direction: RunwayDirection,
    pub configuration: RunwayConfiguration,
}

impl Runway {
    pub fn new(direction: RunwayDirection, configuration: RunwayConfiguration) -> Self {
        Self {
            direction,
            configuration,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum RunwayConfiguration {
    Left = 0,
    Right = 1,
    Center = 2,
    None = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=36"))]
pub struct RunwayDirection(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct RunwayRVR {
    pub runway: Runway,
    #[rasn(identifier = "rVR")]
    pub r_vr: Rvr,
}

impl RunwayRVR {
    pub fn new(runway: Runway, r_vr: Rvr) -> Self {
        Self { runway, r_vr }
    }
}

/// unit = Meters (0..1500), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=59"))]
pub struct SecondsLatLon(pub u8);

/// unit = Second, Range (0.. 59), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum Speed {
    #[rasn(tag(explicit(context, 0)))]
    SpeedIndicated(SpeedIndicated),
    #[rasn(tag(explicit(context, 1)))]
    SpeedIndicatedMetric(SpeedIndicatedMetric),
    #[rasn(tag(explicit(context, 2)))]
    SpeedTrue(SpeedTrue),
    #[rasn(tag(explicit(context, 3)))]
    SpeedTrueMetric(SpeedTrueMetric),
    #[rasn(tag(explicit(context, 4)))]
    SpeedGround(SpeedGround),
    #[rasn(tag(explicit(context, 5)))]
    SpeedGroundMetric(SpeedGroundMetric),
    #[rasn(tag(explicit(context, 6)))]
    SpeedMach(SpeedMach),
}

/// unit = Kilometers/Hour, Range (0..800), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-50..=2000"))]
pub struct SpeedGround(pub i16);

/// unit = Knots, Range (-50..2000), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-100..=4000"))]
pub struct SpeedGroundMetric(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=400"))]
pub struct SpeedIndicated(pub u16);

/// unit = Knots, Range (0..400), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=800"))]
pub struct SpeedIndicatedMetric(pub u16);

/// unit = Kilometers/Hour, Range (-100..4000), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("500..=4000"))]
pub struct SpeedMach(pub u16);

/// unit = Mach Range (0.5 to 4.0), resolution = 0.001
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct SpeedSpeed(pub SequenceOf<Speed>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct SpeedTime {
    pub speed: Speed,
    pub time: Time,
}

impl SpeedTime {
    pub fn new(speed: Speed, time: Time) -> Self {
        Self { speed, time }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=2000"))]
pub struct SpeedTrue(pub u16);

/// unit = Knots, Range (0..2000), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=4000"))]
pub struct SpeedTrueMetric(pub u16);

/// unit = Kilometers/Hour, Range (0..4000), resolution = 1
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SpeedType {
    NoneSpecified = 0,
    Indicated = 1,
    #[rasn(identifier = "true")]
    True = 2,
    Ground = 3,
    Mach = 4,
    Approach = 5,
    Cruise = 6,
    Minimum = 7,
    Maximum = 8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("3"))]
pub struct SpeedTypeSpeedTypeSpeedType(pub SequenceOf<SpeedType>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct SpeedTypeSpeedTypeSpeedTypeSpeed {
    #[rasn(identifier = "speedTypes")]
    pub speed_types: SpeedTypeSpeedTypeSpeedType,
    pub speed: Speed,
}

impl SpeedTypeSpeedTypeSpeedTypeSpeed {
    pub fn new(speed_types: SpeedTypeSpeedTypeSpeedType, speed: Speed) -> Self {
        Self { speed_types, speed }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("-100..=100"))]
pub struct Temperature(pub i8);

/// unit = Degree Celsius, Range (-100..100), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Time {
    pub hours: TimeHours,
    pub minutes: TimeMinutes,
}

impl Time {
    pub fn new(hours: TimeHours, minutes: TimeMinutes) -> Self {
        Self { hours, minutes }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeDeparture {
    #[rasn(tag(explicit(context, 0)), identifier = "timeDepartureAllocated")]
    pub time_departure_allocated: Option<Time>,
    #[rasn(tag(explicit(context, 1)), identifier = "timeDepartureControlled")]
    pub time_departure_controlled: Option<ControlledTime>,
    #[rasn(
        tag(explicit(context, 2)),
        identifier = "timeDepartureClearanceExpected"
    )]
    pub time_departure_clearance_expected: Option<Time>,
    #[rasn(tag(explicit(context, 3)), identifier = "departureMinimumInterval")]
    pub departure_minimum_interval: Option<DepartureMinimumInterval>,
}

impl TimeDeparture {
    pub fn new(
        time_departure_allocated: Option<Time>,
        time_departure_controlled: Option<ControlledTime>,
        time_departure_clearance_expected: Option<Time>,
        departure_minimum_interval: Option<DepartureMinimumInterval>,
    ) -> Self {
        Self {
            time_departure_allocated,
            time_departure_controlled,
            time_departure_clearance_expected,
            departure_minimum_interval,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeDistanceSpecifiedDirection {
    pub time: Time,
    #[rasn(identifier = "distanceSpecifiedDirection")]
    pub distance_specified_direction: DistanceSpecifiedDirection,
}

impl TimeDistanceSpecifiedDirection {
    pub fn new(time: Time, distance_specified_direction: DistanceSpecifiedDirection) -> Self {
        Self {
            time,
            distance_specified_direction,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeDistanceToFromPosition {
    pub time: Time,
    pub distance: Distance,
    pub tofrom: ToFrom,
    pub position: Position,
}

impl TimeDistanceToFromPosition {
    pub fn new(time: Time, distance: Distance, tofrom: ToFrom, position: Position) -> Self {
        Self {
            time,
            distance,
            tofrom,
            position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=23"))]
pub struct TimeHours(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeLevel {
    pub time: Time,
    pub level: Level,
}

impl TimeLevel {
    pub fn new(time: Time, level: Level) -> Self {
        Self { time, level }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=59"))]
pub struct TimeMinutes(pub u8);

/// unit = Minute, Range (0..59), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimePosition {
    pub time: Time,
    pub position: Position,
}

impl TimePosition {
    pub fn new(time: Time, position: Position) -> Self {
        Self { time, position }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimePositionLevel {
    pub timeposition: TimePosition,
    pub level: Level,
}

impl TimePositionLevel {
    pub fn new(timeposition: TimePosition, level: Level) -> Self {
        Self {
            timeposition,
            level,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimePositionLevelSpeed {
    pub timeposition: TimePosition,
    pub levelspeed: LevelSpeed,
}

impl TimePositionLevelSpeed {
    pub fn new(timeposition: TimePosition, levelspeed: LevelSpeed) -> Self {
        Self {
            timeposition,
            levelspeed,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=59"))]
pub struct TimeSeconds(pub u8);

/// unit = Second, Range (0..59), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeSpeed {
    pub time: Time,
    pub speed: Speed,
}

impl TimeSpeed {
    pub fn new(time: Time, speed: Speed) -> Self {
        Self { time, speed }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeSpeedSpeed {
    pub time: Time,
    pub speedspeed: SpeedSpeed,
}

impl TimeSpeedSpeed {
    pub fn new(time: Time, speedspeed: SpeedSpeed) -> Self {
        Self { time, speedspeed }
    }
}
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct TimeTime(pub SequenceOf<Time>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeToFromPosition {
    pub time: Time,
    pub tofrom: ToFrom,
    pub position: Position,
}

impl TimeToFromPosition {
    pub fn new(time: Time, tofrom: ToFrom, position: Position) -> Self {
        Self {
            time,
            tofrom,
            position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum TimeTolerance {
    At = 0,
    AtorAfter = 1,
    AtorBefore = 2,
}

#[doc = " unit = Hour, Range (0..23), resolution = 1"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TimeUnitNameFrequency {
    pub time: Time,
    #[rasn(identifier = "unitName")]
    pub unit_name: UnitName,
    pub frequency: Frequency,
}

impl TimeUnitNameFrequency {
    pub fn new(time: Time, unit_name: UnitName, frequency: Frequency) -> Self {
        Self {
            time,
            unit_name,
            frequency,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Timehhmmss {
    pub hoursminutes: Time,
    pub seconds: TimeSeconds,
}

impl Timehhmmss {
    pub fn new(hoursminutes: Time, seconds: TimeSeconds) -> Self {
        Self {
            hoursminutes,
            seconds,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum ToFrom {
    To = 0,
    From = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct ToFromPosition {
    #[rasn(identifier = "toFrom")]
    pub to_from: ToFrom,
    pub position: Position,
}

impl ToFromPosition {
    pub fn new(to_from: ToFrom, position: Position) -> Self {
        Self { to_from, position }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum TrafficType {
    NoneSpecified = 0,
    OppositeDirection = 1,
    SameDirection = 2,
    Converging = 3,
    Crossing = 4,
    Diverging = 5,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum Turbulence {
    Light = 0,
    Moderate = 1,
    Severe = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct UnitName {
    #[rasn(tag(explicit(context, 0)), identifier = "facilityDesignation")]
    pub facility_designation: FacilityDesignation,
    #[rasn(tag(explicit(context, 1)), identifier = "facilityName")]
    pub facility_name: Option<FacilityName>,
    #[rasn(tag(explicit(context, 2)), identifier = "facilityFunction")]
    pub facility_function: FacilityFunction,
}

impl UnitName {
    pub fn new(
        facility_designation: FacilityDesignation,
        facility_name: Option<FacilityName>,
        facility_function: FacilityFunction,
    ) -> Self {
        Self {
            facility_designation,
            facility_name,
            facility_function,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct UnitNameFrequency {
    #[rasn(identifier = "unitName")]
    pub unit_name: UnitName,
    pub frequency: Frequency,
}

impl UnitNameFrequency {
    pub fn new(unit_name: UnitName, frequency: Frequency) -> Self {
        Self {
            unit_name,
            frequency,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VersionNumber(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct VerticalChange {
    pub direction: VerticalDirection,
    pub rate: VerticalRate,
}

impl VerticalChange {
    pub fn new(direction: VerticalDirection, rate: VerticalRate) -> Self {
        Self { direction, rate }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(enumerated)]
pub enum VerticalDirection {
    Up = 0,
    Down = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum VerticalRate {
    #[rasn(tag(explicit(context, 0)))]
    VerticalRateEnglish(VerticalRateEnglish),
    #[rasn(tag(explicit(context, 1)))]
    VerticalRateMetric(VerticalRateMetric),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=3000"))]
pub struct VerticalRateEnglish(pub u16);

/// unit = Feet/Minute, Range (0..30000), resolution = 10
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=1000"))]
pub struct VerticalRateMetric(pub u16);

/// unit = Meters/Minute, Range (0..10000), resolution = 10
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct WaypointSpeedLevel {
    #[rasn(tag(explicit(context, 0)))]
    pub position: Position,
    #[rasn(tag(explicit(context, 1)))]
    pub speed: Option<Speed>,
    #[rasn(tag(explicit(context, 2)), identifier = "aTWLevels")]
    pub a_twlevels: Option<ATWLevelSequence>,
}

impl WaypointSpeedLevel {
    pub fn new(
        position: Position,
        speed: Option<Speed>,
        a_twlevels: Option<ATWLevelSequence>,
    ) -> Self {
        Self {
            position,
            speed,
            a_twlevels,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=360"))]
pub struct WindDirection(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice)]
pub enum WindSpeed {
    #[rasn(tag(explicit(context, 0)))]
    WindSpeedEnglish(WindSpeedEnglish),
    #[rasn(tag(explicit(context, 1)))]
    WindSpeedMetric(WindSpeedMetric),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WindSpeedEnglish(pub u8);

/// unit = Knot, Range (0..255), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("0..=511"))]
pub struct WindSpeedMetric(pub u16);

/// unit = Degree, Range (1..360), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct Winds {
    pub direction: WindDirection,
    pub speed: WindSpeed,
}

impl Winds {
    pub fn new(direction: WindDirection, speed: WindSpeed) -> Self {
        Self { direction, speed }
    }
}

/// unit = Kilometer/Hour, Range (0..511), resolution = 1
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1996..=2095"))]
pub struct Year(pub u16);
