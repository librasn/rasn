#![doc = include_str!("../README.md")]
#![no_std]

use rasn::prelude::*;

pub type ValueName = String;
pub type Value = String;
pub type DateTime = rasn::types::GeneralizedTime;
pub type AnyUri = StringWithNoCrlFht;

/// An alert message.
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct Alert {
    /// Unambiguous identification of the message from all messages from this
    /// sender, in a format defined by the sender and identified in the "sender"
    /// field below.
    pub identifier: IdentifierString,

    /// The globally unambiguous identification of the sender. This
    /// specification does not define the root of a global identification tree
    /// (there is no international agreement on such a root), so it relies on
    /// human-readable text to define globally and unambiguously the sender. An
    /// internet domain name or use of "iri:/ITU-T/..." is possible, but the
    /// choice needs to be clearly stated in human-readable form.
    pub sender: String,
    pub sent: DateTime,
    pub status: AlertStatus,
    pub msg_type: AlertMessageType,

    /// Not standardized human-readable identification of the source of
    /// the alert.
    pub source: Option<String>,
    pub scope: AlertScope,

    /// Not standardized human-readable restrictions on the distribution of the
    /// alert message.
    pub restriction: Option<String>,
    /// A space separated list of addresses for private messages.
    pub addresses: Option<String>,
    /// Sequence codes for special handling.
    pub code_list: SequenceOf<String>,
    /// Not standardized human-readable clarifying text for the alert.
    pub note: Option<String>,
    /// Space-separated references to earlier messages.
    pub references: Option<String>,
    /// Space-separated references to related incidents.
    pub incidents: Option<String>,
    pub info_list: SequenceOf<AlertInformation>,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum AlertStatus {
    Actual,
    Draft,
    Exercise,
    System,
    Test,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum AlertMessageType {
    Ack,
    Alert,
    Cancel,
    Error,
    Update,
}

#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum AlertScope {
    Private,
    Public,
    Restricted,
}

fn default_alert_language() -> Language {
    Language("en-US".try_into().unwrap())
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct AlertEventCode {
    pub value_name: ValueName,
    pub value: Value,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct AlertParameter {
    value_name: ValueName,
    value: Value,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct AlertInformation {
    /// The language used in this value.
    #[rasn(default = "default_alert_language")]
    language: Language,
    #[rasn(size("1.."))]
    category_list: SequenceOf<InformationCategory>,

    /// Not standardized human-readable text describing the type of the event.
    event: String,
    #[rasn(size("0.."))]
    response_type_list: SequenceOf<InformationResponseType>,
    urgency: HowUrgent,
    severity: HowSevere,
    certainty: HowCertain,

    /// Not standardized human-readable text describing the intended audience
    /// for the message.
    audience: Option<String>,

    #[rasn(size("0.."))]
    event_code_list: SequenceOf<AlertEventCode>,
    effective: Option<DateTime>,
    onset: Option<DateTime>,
    expires: Option<DateTime>,

    /// Not standardized human-readable name of the authority issuing
    /// the message.
    sender_name: Option<String>,

    /// Not standardized human-readable short statement (headline) of the alert.
    #[rasn(size("1..160"), extensible)]
    headline: Option<String>,

    /// Not standardized human-readable extended description of the event.
    description: Option<String>,
    /// Not standardized human-readable recommended action.
    instruction: Option<String>,
    web: Option<AnyUri>,
    /// Not standardized human-readable contact details for follow-up.
    contact: Option<String>,
    #[rasn(size("0.."))]
    parameter_list: SequenceOf<AlertParameter>,
    #[rasn(size("0.."))]
    resource_list: SequenceOf<ResourceFile>,
    #[rasn(size("0.."))]
    area_list: SequenceOf<Area>,
}

/// The category of the subject event of the alert message.
#[derive(AsnType, Copy, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum InformationCategory {
    /// Chemical, Biological, Radiological, Nuclear or High-Yield Explosive
    /// threat or attack
    Cbrne,
    /// Pollution and other environmental
    Env,
    /// Fire suppression and rescue
    Fire,
    /// Geophysical
    Geo,
    /// Medical and public health
    Health,
    /// Utility, telecommunication, other non-transport infrastructure
    Infra,
    /// Meteorological
    Met,
    /// Other events
    Other,
    /// Rescue and recovery
    Rescue,
    /// General emergency and public safety
    Safety,
    /// Law enforcement, military, homeland and local/private security
    Security,
    /// Public and private transportation
    Transport,
}

#[derive(AsnType, Copy, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum InformationResponseType {
    Assess,
    Evacuate,
    Execute,
    Monitor,
    None,
    Prepare,
    Shelter,
}

#[derive(AsnType, Copy, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum HowUrgent {
    Expected,
    Future,
    Immediate,
    Past,
    Unknown,
}

#[derive(AsnType, Copy, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum HowSevere {
    Extreme,
    Minor,
    Moderate,
    Severe,
    Unknown,
}

#[derive(AsnType, Copy, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum HowCertain {
    Likely,
    Observed,
    Possible,
    Unknown,
    Unlikely,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct ResourceFile {
    resource_desc: String,
    mime_type: Option<String>,
    size: Option<Integer>,
    uri: Option<AnyUri>,
    deref_uri: Option<String>,
    digest: Option<String>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct Area {
    area_desc: String,
    polygon_list: SequenceOf<String>,
    circle_list: SequenceOf<String>,
    #[rasn(size("0.."))]
    geocode_list: SequenceOf<Geocode>,
    altitude: Option<String>,
    ceiling: Option<String>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(automatic_tags)]
pub struct Geocode {
    value_name: ValueName,
    value: Value,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(from(
    "\u{0009}",
    "\u{000A}",
    "\u{000D}",
    "\u{0020}..\u{D7FF}",
    "\u{E000}..\u{FFFD}",
    "\u{10000}..\u{10FFFD}"
))]
#[rasn(delegate)]
pub struct String(Utf8String);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(size(1))]
#[rasn(delegate)]
pub struct StringChar(String);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(from("\u{0020}", "\u{002C}"))]
#[rasn(delegate)]
pub struct SpaceAndComma(Utf8String);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct IdentifierString(StringChar);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(from("a..z", "A..Z", "-", "0..9"))]
#[rasn(delegate)]
pub struct Language(VisibleString);

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(from("\u{20}..\u{D7FF}", "\u{E000}..\u{FFFD}", "\u{10000}..\u{100000}"))]
#[rasn(delegate)]
pub struct StringWithNoCrlFht(Utf8String);
