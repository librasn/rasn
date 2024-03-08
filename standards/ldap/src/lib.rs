#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

use rasn::{types::*, Decode, Encode};

/// ID value of a corresponding request [`LdapMessage`].
///
/// The messageID of a request MUST have a non-zero value different from
/// the messageID of any other request in progress in the same LDAP
/// session.  The zero value is reserved for the unsolicited notification
/// message.
///
/// Typical clients increment a counter for each request.
///
/// A client MUST NOT send a request with the same messageID as an
/// earlier request in the same LDAP session unless it can be determined
/// that the server is no longer servicing the earlier request (e.g.,
/// after the final response is received, or a subsequent Bind
/// completes).  Otherwise, the behavior is undefined.  For this purpose,
/// note that Abandon and successfully abandoned operations do not send
/// responses.
pub type MessageId = u32;

/// A notational convenience to indicate that, although strings of `LdapString`
/// encode as [`OctetString`] types, the ISO10646 character set (a superset of
/// Unicode) is used, encoded following the UTF-8 RFC3629 algorithm.
pub type LdapString = OctetString;

/// A notational convenience to indicate that the permitted value of this string
/// is a (UTF-8 encoded) dotted-decimal representation of an `ObjectIdentifier`.
pub type LdapOid = OctetString;

/// The representation of a Distinguished Name (DN).
pub type LdapDn = LdapString;
/// The representation of a Relative Distinguished Name (RDN).
pub type RelativeLdapDn = LdapString;
/// An attribute type and zero or more options.
pub type AttributeDescription = LdapString;
/// An encoded attribute value. The attribute value is encoded according to the
/// LDAP-specific encoding definition of its corresponding syntax.  The
/// LDAP-specific encoding definitions for different syntaxes and attribute
/// types may be found in other documents and in particular [RFC 4517].
pub type AttributeValue = OctetString;
/// The value to compare in the assertion. The syntax of the `AssertionValue`
/// depends on the context of the LDAP operation being performed.
pub type AssertionValue = OctetString;
/// An identifier for a "Matching Rule".
pub type MatchingRuleId = LdapString;
/// references to one or more servers or services that may be accessed via LDAP
/// or other protocols.
pub type Referral = SequenceOf<Uri>;
/// An LDAP String restricted to URL characters.
pub type Uri = LdapString;
/// List of LDAP [`Control`]s.
pub type Controls = SequenceOf<Control>;
pub type AttributeSelection = SequenceOf<LdapString>;
pub type PartialAttributeList = SequenceOf<PartialAttribute>;
pub type AttributeList = SequenceOf<Attribute>;

/// The envelope for all LDAP operations.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct LdapMessage {
    pub message_id: MessageId,
    pub protocol_op: ProtocolOp,
    #[rasn(tag(0))]
    pub controls: Option<Controls>,
}

impl LdapMessage {
    /// LdapMessage constructor
    pub fn new(message_id: MessageId, protocol_op: ProtocolOp) -> Self {
        LdapMessage {
            message_id,
            protocol_op,
            controls: None,
        }
    }
}

/// The kind of operation in the [`LdapMessage`].
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum ProtocolOp {
    BindRequest(BindRequest),
    BindResponse(BindResponse),
    UnbindRequest(UnbindRequest),
    SearchRequest(SearchRequest),
    SearchResEntry(SearchResultEntry),
    SearchResDone(SearchResultDone),
    SearchResRef(SearchResultReference),
    ModifyRequest(ModifyRequest),
    ModifyResponse(ModifyResponse),
    AddRequest(AddRequest),
    AddResponse(AddResponse),
    DelRequest(DelRequest),
    DelResponse(DelResponse),
    ModDnRequest(ModifyDnRequest),
    ModDnResponse(ModifyDnResponse),
    CompareRequest(CompareRequest),
    CompareResponse(CompareResponse),
    AbandonRequest(AbandonRequest),
    ExtendedReq(ExtendedRequest),
    ExtendedResp(ExtendedResponse),
    IntermediateResponse(IntermediateResponse),
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct AttributeValueAssertion {
    pub attribute_desc: AttributeDescription,
    pub assertion_value: AssertionValue,
}

impl AttributeValueAssertion {
    /// AttributeValueAssertion constructor
    pub fn new(attribute_desc: AttributeDescription, assertion_value: AssertionValue) -> Self {
        Self {
            attribute_desc,
            assertion_value,
        }
    }
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct PartialAttribute {
    pub r#type: AttributeDescription,
    pub vals: SetOf<AttributeValue>,
}

impl PartialAttribute {
    /// PartialAttribute constructor
    pub fn new(r#type: AttributeDescription, vals: SetOf<AttributeValue>) -> Self {
        Self { r#type, vals }
    }
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct Attribute {
    pub r#type: AttributeDescription,
    pub vals: SetOf<AttributeValue>,
}

impl Attribute {
    /// Attribute constructor
    pub fn new(r#type: AttributeDescription, vals: SetOf<AttributeValue>) -> Self {
        Self { r#type, vals }
    }
}

/// The envelope for the result of any operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct LdapResult {
    /// The code indicating the status of the operation.
    pub result_code: ResultCode,
    pub matched_dn: LdapDn,
    pub diagnostic_message: LdapString,
    #[rasn(tag(3))]
    pub referral: Option<Referral>,
}

impl LdapResult {
    /// LdapResult constructor
    pub fn new(
        result_code: ResultCode,
        matched_dn: LdapDn,
        diagnostic_message: LdapString,
    ) -> Self {
        Self {
            result_code,
            matched_dn,
            diagnostic_message,
            referral: None,
        }
    }
}

#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ResultCode {
    Success = 0,
    OperationsError = 1,
    ProtocolError = 2,
    TimeLimitExceeded = 3,
    SizeLimitExceeded = 4,
    CompareFalse = 5,
    CompareTrue = 6,
    AuthMethodNotSupported = 7,
    StrongerAuthRequired = 8,
    Referral = 10,
    AdminLimitExceeded = 11,
    UnavailableCriticalExtension = 12,
    ConfidentialityRequired = 13,
    SaslBindInProgress = 14,
    NoSuchAttribute = 16,
    UndefinedAttributeType = 17,
    InappropriateMatching = 18,
    ConstraintViolation = 19,
    AttributeOrValueExists = 20,
    InvalidAttributeSyntax = 21,
    NoSuchObject = 32,
    AliasProblem = 33,
    InvalidDnSyntax = 34,
    AliasDereferencingProblem = 36,
    InappropriateAuthentication = 48,
    InvalidCredentials = 49,
    InsufficientAccessRights = 50,
    Busy = 51,
    Unavailable = 52,
    UnwillingToPerform = 53,
    LoopDetect = 54,
    NamingViolation = 64,
    ObjectClassViolation = 65,
    NotAllowedOnNonLeaf = 66,
    NotAllowedOnRdn = 67,
    EntryAlreadyExists = 68,
    ObjectClassModsProhibited = 69,
    AffectsMultipleDsas = 71,
    Other = 80,
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct Control {
    pub control_type: LdapOid,
    #[rasn(default)]
    pub criticality: bool,
    pub control_value: Option<OctetString>,
}

impl Control {
    /// Control constructor
    pub fn new(
        control_type: LdapOid,
        criticality: bool,
        control_value: Option<OctetString>,
    ) -> Self {
        Self {
            control_type,
            criticality,
            control_value,
        }
    }
}

/// Allow authentication information to be exchanged between the client
/// and server.
///
/// The Bind operation should be thought of as the "authenticate" operation.
/// Operational, authentication, and security-related semantics of this
/// operation are given in [RFC 4513].
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
#[rasn(tag(application, 0))]
pub struct BindRequest {
    /// A version number indicating the version of the protocol to be used at
    /// the LDAP message layer. This document describes version 3 of the
    /// protocol. There is no version negotiation. The client sets this field to
    /// the version it desires.  If the server does not support the specified
    /// version, it **must** respond with a [`BindResponse`]
    /// containing [`ResultCode::ProtocolError`].
    pub version: u8,
    /// If not empty, the name of the Directory object that the client wishes to
    /// bind as.  This field may take on a null value (a zero-length string) for
    /// the purposes of anonymous binds or when using SASL authentication. Where
    /// the server attempts to locate the named object, it **shall not** perform
    /// alias dereferencing.
    pub name: LdapDn,
    /// Information used in authentication.
    pub authentication: AuthenticationChoice,
}

impl BindRequest {
    /// BindRequest constructor
    pub fn new(version: u8, name: LdapDn, authentication: AuthenticationChoice) -> Self {
        Self {
            version,
            name,
            authentication,
        }
    }
}

/// Information used in authentication.
///
/// This type is extensible. Servers that do not support a choice supplied
/// by a client return a [`BindResponse`]
/// with [`ResultCode::AuthMethodNotSupported`].
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum AuthenticationChoice {
    #[rasn(tag(0))]
    Simple(OctetString),
    // 1 and 2 are reserved.
    #[rasn(tag(3))]
    Sasl(SaslCredentials),
}

#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct SaslCredentials {
    pub mechanism: LdapString,
    pub credentials: Option<OctetString>,
}

impl SaslCredentials {
    /// SaslCredentials constructor
    pub fn new(mechanism: LdapString, credentials: Option<OctetString>) -> Self {
        Self {
            mechanism,
            credentials,
        }
    }
}

/// An indication from the server of the status of the client's request
/// for authentication.
///
///
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
#[rasn(tag(application, 1))]
pub struct BindResponse {
    pub result_code: ResultCode,
    pub matched_dn: LdapDn,
    pub diagnostic_message: LdapString,
    #[rasn(tag(3))]
    pub referral: Option<Referral>,
    #[rasn(tag(7))]
    pub server_sasl_creds: Option<OctetString>,
}

impl BindResponse {
    /// BindResponse constructor
    pub fn new(
        result_code: ResultCode,
        matched_dn: LdapDn,
        diagnostic_message: LdapString,
        referral: Option<Referral>,
        server_sasl_creds: Option<OctetString>,
    ) -> Self {
        Self {
            result_code,
            matched_dn,
            diagnostic_message,
            referral,
            server_sasl_creds,
        }
    }
}

/// Request to terminate an LDAP session.
///
/// The Unbind operation is not the antithesis of the Bind operation as the name
/// implies. The naming of these operations are historical. The Unbind operation
/// should be thought of as the "quit" operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 2))]
pub struct UnbindRequest;

/// Used to request a server to return, subject to access controls and other
/// restrictions, a set of entries matching a complex search criterion. This can
/// be used to read attributes from a single entry, from entries immediately
/// subordinate to a particular entry, or from a whole subtree of entries.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 3))]
#[non_exhaustive]
pub struct SearchRequest {
    /// The name of the base object entry (or possibly the root) relative to
    /// which the search is to be performed.
    pub base_object: LdapDn,
    /// The scope of the search to be performed.
    pub scope: SearchRequestScope,
    /// An indicator as to whether or not alias entries are to be dereferenced
    /// during stages of the search operation.
    pub deref_aliases: SearchRequestDerefAliases,
    /// The maximum number of entries to be returned as a result of the search.
    ///
    /// A value of zero in this field indicates that no client-requested time
    /// limit restrictions are in effect for the Search. Servers may also
    /// enforce a maximum number of entries to return.
    pub size_limit: u32,
    /// The maximum time (in seconds) allowed for a search.
    ///
    /// A value of zero in this field indicates that no client-requested time
    /// limit restrictions are in effect for the Search. Servers may also
    /// enforce a maximum time limit for the Search.
    pub time_limit: u32,
    /// Whether search results are to contain both attribute descriptions and
    /// values, or just attribute descriptions.
    ///
    /// Setting this field to [`true`] causes only attribute descriptions (and
    /// not values) to be returned. Setting this field to [`false`] causes both
    /// attribute descriptions and values to be returned.
    pub types_only: bool,
    /// Defines the conditions that must be fulfilled in order for the search to
    /// match a given entry.
    pub filter: Filter,
    /// A selection list of the attributes to be returned from each entry that
    /// matches the search filter.
    ///
    /// Attributes that are subtypes of listed attributes are
    /// implicitly included.
    pub attributes: AttributeSelection,
}

impl SearchRequest {
    /// SearchRequest constructor
    pub fn new(
        base_object: LdapDn,
        scope: SearchRequestScope,
        deref_aliases: SearchRequestDerefAliases,
        size_limit: u32,
        time_limit: u32,
        types_only: bool,
        filter: Filter,
        attributes: AttributeSelection,
    ) -> Self {
        Self {
            base_object,
            scope,
            deref_aliases,
            size_limit,
            time_limit,
            types_only,
            filter,
            attributes,
        }
    }
}

/// The scope of the search to be performed.
#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SearchRequestScope {
    ///  The scope is constrained to the entry named by `base_object`.
    BaseObject = 0,
    /// The scope is constrained to the immediate subordinates of the entry
    /// named by `base_object`.
    SingleLevel = 1,
    /// The scope is constrained to the entry named by `base_object` and to all
    /// its subordinates.
    WholeSubtree = 2,
}

/// An indicator as to whether or not alias entries are to be dereferenced
/// during stages of the search operation.
///
/// The act of dereferencing an alias includes recursively dereferencing
/// aliases that refer to aliases.
///
/// Servers **must** detect looping while dereferencing aliases in order to
/// prevent denial-of-service attacks of this nature.
#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SearchRequestDerefAliases {
    /// Do not dereference aliases in searching or in locating the base object
    /// of the search.
    NeverDerefAliases = 0,
    /// While searching subordinates of the base object, dereference any alias
    /// within the search scope. Dereferenced objects become the vertices of
    /// further search scopes where the search operation is also applied.
    ///
    /// If the search scope is [`SearchRequestScope::WholeSubtree`], the search
    /// continues in the subtree(s) of any dereferenced object.
    ///
    /// If the search scope is [`SearchRequestScope::SingleLevel`], the search
    /// is applied to any dereferenced objects and is not applied to
    /// their subordinates.
    ///
    /// Servers *should* eliminate duplicate entries that arise due to alias
    /// dereferencing while searching.
    DerefInSearching = 1,
    /// Dereference aliases in locating the base object of the search, but not
    /// when searching subordinates of the base object.
    DerefFindingBaseObj = 2,
    /// Dereference aliases both in searching and in locating the base object of
    /// the search.
    DerefAlways = 3,
}

/// Defines the conditions that must be fulfilled in order for the search to
/// match a given entry.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum Filter {
    /// All matching rules must evaluate to `true`.
    #[rasn(tag(0))]
    And(SetOf<Filter>),
    /// Any matching rules must evaluate to `true`.
    #[rasn(tag(1))]
    Or(SetOf<Filter>),
    /// The matching rule must evaluate to `false`.
    #[rasn(tag(2))]
    Not(alloc::boxed::Box<Filter>),
    /// The filter is `true` when the EQUALITY rule returns `true` as applied to
    /// the attribute or subtype and the asserted value.
    #[rasn(tag(3))]
    EqualityMatch(AttributeValueAssertion),
    /// The filter is `true` when the SUBSTR rule returns `true` as applied to
    /// the attribute or subtype and the asserted value.
    ///
    /// Note that the AssertionValue in a substrings filter item conforms to the
    /// assertion syntax of the EQUALITY matching rule for the attribute type
    /// rather than to the assertion syntax of the SUBSTR matching rule for the
    /// attribute type.  Conceptually, the entire SubstringFilter is converted
    /// into an assertion value of the substrings matching rule prior to
    /// applying the rule.
    #[rasn(tag(4))]
    Substrings(SubstringFilter),
    /// The filter is `true` when the "ORDERING" rule returns `false` as applied
    /// to the attribute or subtype and the asserted value.
    #[rasn(tag(5))]
    GreaterOrEqual(AttributeValueAssertion),
    /// The filter is `true` when either the "ORDERING" or "EQUALITY" rule
    /// returns `true` as applied to the attribute or subtype and the
    /// asserted value.
    #[rasn(tag(6))]
    LessOrEqual(AttributeValueAssertion),
    /// The filter is `true` when there is an attribute or subtype of the
    /// specified attribute description present in an entry, `false` when no
    /// attribute or subtype of the specified attribute description is present
    /// in an entry, and "Undefined" otherwise.
    #[rasn(tag(7))]
    Present(AttributeDescription),
    /// The filter is `true` when there is a value of the attribute type or
    /// subtype for which some locally-defined approximate matching
    /// algorithm (e.g., spelling variations, phonetic match, etc.) returns
    /// `true`. If a value matches for equality, it also satisfies an
    /// approximate match. If approximate matching is not supported for the
    /// attribute, this filter item should be treated as an `EqualityMatch`.
    #[rasn(tag(8))]
    ApproxMatch(AttributeValueAssertion),
    /// The filter is evaluated as follows:
    ///
    #[rasn(tag(9))]
    ExtensibleMatch(MatchingRuleAssertion),
}

/// The SUBSTR matching rule for the attribute type or subtype.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct SubstringFilter {
    /// The type to match against.
    pub r#type: AttributeDescription,
    /// Substrings to match against.
    pub substrings: SequenceOf<SubstringChoice>,
}

impl SubstringFilter {
    /// SubstringFilter constructor
    pub fn new(r#type: AttributeDescription, substrings: SequenceOf<SubstringChoice>) -> Self {
        Self { r#type, substrings }
    }
}

/// Which part of the substring to match against.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(choice)]
#[non_exhaustive]
pub enum SubstringChoice {
    /// The start of a substrings filter, **must** be the first element.
    #[rasn(tag(0))]
    Initial(AssertionValue),
    /// An assertion in the middle of a substrings filter, **must not** be the
    /// first or last element.
    #[rasn(tag(1))]
    Any(AssertionValue),
    /// The end of a substrings filter, **must** be the last element.
    #[rasn(tag(2))]
    Final(AssertionValue),
}

/// Extensible match rule assertion.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub struct MatchingRuleAssertion {
    /// If the `matching_rule` is absent, the `type` field **MUST** be
    /// present, and an equality match is performed for that type.
    #[rasn(tag(1))]
    pub matching_rule: Option<MatchingRuleId>,
    /// The type match against if `matching_rule` is absent.
    #[rasn(tag(2))]
    pub r#type: Option<AttributeDescription>,
    /// The value to match against.
    ///
    /// If the `type` field is absent and the `matching_rule` is present,
    /// `match_value` is compared against all attributes in an entry that
    /// support that `matching_rule`.
    ///
    /// If the `type` field is present and the `matching_rule` is present, the
    /// `match_value` is compared against the specified attribute type and
    /// its subtypes.
    #[rasn(tag(3))]
    pub match_value: AssertionValue,
    /// If the dnAttributes field is set to `true`, the match is additionally
    /// applied against all the AttributeValueAssertions in an entry's
    /// distinguished name, and it evaluates to TRUE if there is at least one
    /// attribute or subtype in the distinguished name for which the filter
    /// item evaluates to `true`. The dnAttributes field is present to
    /// alleviate the need for multiple versions of generic matching rules
    /// (such as word matching), where one applies to entries and another
    /// applies to entries and DN attributes as well.
    #[rasn(tag(4), default)]
    pub dn_attributes: bool,
}

impl MatchingRuleAssertion {
    /// MatchingRuleAssertion constructor
    pub fn new(
        matching_rule: Option<MatchingRuleId>,
        r#type: Option<AttributeDescription>,
        match_value: AssertionValue,
        dn_attributes: bool,
    ) -> Self {
        Self {
            matching_rule,
            r#type,
            match_value,
            dn_attributes,
        }
    }
}

/// An entry found during the search.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 4))]
#[non_exhaustive]
pub struct SearchResultEntry {
    /// The name of the object found.
    pub object_name: LdapDn,
    /// The attributes associated with the object.
    pub attributes: PartialAttributeList,
}

impl SearchResultEntry {
    /// SearchResultEntry constructor
    pub fn new(object_name: LdapDn, attributes: PartialAttributeList) -> Self {
        Self {
            object_name,
            attributes,
        }
    }
}

/// Reference of servers containing the data required to continue the search.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 19))]
#[rasn(delegate)]
pub struct SearchResultReference(pub SequenceOf<Uri>);

/// The result of a search operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 5))]
#[rasn(delegate)]
pub struct SearchResultDone(pub LdapResult);

/// Allows a client to request that a modification of an entry be performed on
/// its behalf by a server.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 6))]
pub struct ModifyRequest {
    /// The name of the entry to be modified.
    pub object: LdapDn,
    /// A list of modifications to be performed on the entry. The entire list of
    /// modifications **must** be performed in the order they are listed as a
    /// single atomic operation. While individual modifications may violate
    /// certain aspects of the directory schema (such as the object class
    /// definition and Directory Information Tree (DIT) content rule), the
    /// resulting entry after the entire list of modifications is performed
    /// **must** conform to the requirements of the directory model and
    /// controlling schema. See [RFC 4512]
    pub changes: SequenceOf<ModifyRequestChanges>,
}

/// Modifications to be performed on an LDAP entry.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModifyRequestChanges {
    /// The type of modification being performed.
    pub operation: ChangeOperation,
    /// The attribute type or attribute type and values being modified.
    pub modification: PartialAttribute,
}

/// The type of modification being performed on an LDAP entry.
#[derive(AsnType, Encode, Decode, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
pub enum ChangeOperation {
    /// Add values listed to the modification attribute, creating the attribute
    /// if necessary.
    Add = 0,
    /// Delete values listed from the modification attribute. If no values are
    /// listed, or if all current values of the attribute are listed, the entire
    /// attribute is removed.
    Delete = 1,
    /// Replace all existing values of the modification attribute with the new
    /// values listed, creating the attribute if it did not already exist. A
    /// replace with no value will delete the entire attribute if it exists, and
    /// it is ignored if the attribute does not exist.
    Replace = 2,
}

/// The result of a [`ModifyRequest`] operation.
///
/// Due to the requirement for atomicity in applying the list of modifications
/// in the [`ModifyRequest`], the client may expect that no modifications of the
/// DIT have been performed if the [`ModifyResponse`] received indicates any
/// sort of error, and that all requested modifications have been performed if
/// the [`ModifyResponse`] indicates successful completion of the modify
/// operation. Whether or not the modification was applied cannot be determined
/// by the client if the [`ModifyResponse`] was not received (e.g., the LDAP
/// session was terminated or the modify operation was abandoned).
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 7))]
pub struct ModifyResponse(pub LdapResult);

/// Allows a client to request the addition of LDAP an entry into the directory.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 8))]
pub struct AddRequest {
    /// The name of the entry to be added.
    pub entry: LdapDn,
    /// The list of attributes that, along with those from the RDN, make up the
    /// content of the entry being added.  Clients *may* or *may not* include
    /// the RDN attribute(s) in this list. Clients **must not** supply
    /// `NO-USER-MODIFICATION` attributes such as the `createTimestamp` or
    /// `creatorsName` attributes, since the server maintains
    /// these automatically.
    pub attributes: AttributeList,
}

/// The result of a [`AddRequest`] operation.
///
/// A response of success indicates that the new entry has been added to
/// the Directory.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 9))]
pub struct AddResponse(pub LdapResult);

/// Allows a client to request the removal of an LDAP entry from the directory.
///
/// The request contains the name of the entry to be deleted. Only leaf entries
/// (those with no subordinate entries) can be deleted with this operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 10))]
pub struct DelRequest(pub LdapDn);

/// The result of a [`DelRequest`] operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 11))]
pub struct DelResponse(pub LdapResult);

/// Allows a client to change the Relative Distinguished Name (RDN) of an LDAP
/// entry in the directory and/or to move a subtree of entries to a new location
/// in the directory.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 12))]
pub struct ModifyDnRequest {
    /// The name of the entry to be changed.
    pub entry: LdapDn,
    /// The new RDN of the entry. The value of the old RDN is supplied when
    /// moving the entry to a new superior without changing its RDN. Attribute
    /// values of the new RDN not matching any attribute value of the entry are
    /// added to the entry, and an appropriate error is returned if this fails.
    pub new_rdn: RelativeLdapDn,
    /// Controls whether the old RDN attribute values are to be retained as
    /// attributes of the entry or deleted from the entry.
    pub delete_old_rdn: bool,
    /// If present, this is the name of an existing object entry that becomes
    /// the immediate superior (parent) of the existing entry.
    #[rasn(tag(0))]
    pub new_superior: Option<LdapDn>,
}

/// The result of a [`ModifyDnRequest`] operation.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 13))]
pub struct ModifyDnResponse(pub LdapResult);

/// Allows a client to compare an assertion value with the values of a
/// particular attribute in a particular entry in the directory.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 14))]
pub struct CompareRequest {
    /// The name of the entry to be compared.
    pub entry: LdapDn,
    /// The assertion to compare with.
    pub ava: AttributeValueAssertion,
}

/// The result of a [`CompareResponse`] operation.
///
/// The result code is set to [`CompareTrue`][ResultCode::CompareTrue],
/// [`CompareFalse`][ResultCode::CompareFalse], or an appropriate error.
/// `CompareTrue` indicates that the assertion value in the `ava` field matches
/// a value of the attribute or subtype according to the attribute's `EQUALITY`
/// matching rule. `CompareFalse` indicates that the assertion value in the
/// `ava` field and the values of the attribute or subtype did not match. Other
/// result codes indicate either that the result of the comparison was
/// "undefined", or that some error occurred.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 15))]
pub struct CompareResponse(pub LdapResult);

/// Allows a client to request that the server abandon an uncompleted operation.
///
/// The [`MessageId`] is that of an operation that was requested earlier at this
/// LDAP message layer. The [`AbandonRequest`] itself has its own `MessageId`.
/// This is distinct from the `MessageId` of the earlier operation
/// being abandoned.
///
/// There is no response defined in the Abandon operation.  Upon receipt of an
/// [`AbandonRequest`], the server *may* abandon the operation identified by the
/// `MessageId`. Since the client cannot tell the difference between a
/// successfully abandoned operation and an uncompleted operation, the
/// application of the `Abandon` operation is limited to uses where the client
/// does not require an indication of its outcome.
///
/// "Abandon", "Bind", "Unbind", and "StartTLS" operations cannot be abandoned.
///
/// In the event that a server receives an `AbandonRequest` on a search
/// operation in the midst of transmitting responses to the search, that server
/// **must** cease transmitting entry responses to the abandoned request
/// immediately, and it **must not** send the `SearchResultDone`. Of course, the
/// server **must** ensure that only properly encoded [`LdapMessage`] PDUs
/// are transmitted.
///
/// The ability to abandon other (particularly update) operations is at the
/// discretion of the server.
///
/// Clients should not send Abandon requests for the same operation multiple
/// times, and they **must** also be prepared to receive results from operations
/// they have abandoned (since these might have been in transit when the
/// abandon was requested or might not be able to be abandoned).
///
/// Servers MUST discard Abandon requests for messageIDs they do not recognize,
/// for operations that cannot be abandoned, and for operations that have
/// already been abandoned.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, tag(application, 16))]
pub struct AbandonRequest(pub MessageId);

/// Allows additional operations to be defined for services not already
/// available in the protocol.
///
/// The Extended operation allows clients to make requests and receive responses
/// with predefined syntaxes and semantics.  These may be defined in RFCs or be
/// private to particular implementations.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 23))]
pub struct ExtendedRequest {
    /// The unique [`LdapOid`] corresponding to the extended operation. Where
    /// the request name is not recognized, the server
    /// returns [`ResultCode::ProtocolError`].
    #[rasn(tag(0))]
    pub request_name: LdapOid,
    /// Information specific to the extended operation.
    #[rasn(tag(1))]
    pub request_value: Option<OctetString>,
}

/// The result of a [`CompareResponse`] operation.
///
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 24))]
pub struct ExtendedResponse {
    pub result_code: ResultCode,
    pub matched_dn: LdapDn,
    pub diagnostic_message: LdapString,
    #[rasn(tag(3))]
    pub referral: Option<Referral>,
    /// When present, contains an [`LdapOid`] that is unique for this extended
    /// operation or response. Will be absent whenever the server is unable or
    /// unwilling to determine the appropriate [`LdapOid`] to return, for
    /// instance, when the requestName cannot be parsed or its value is
    /// not recognized.
    #[rasn(tag(10))]
    pub response_name: Option<LdapOid>,
    /// Information specific to the extended operation.
    #[rasn(tag(11))]
    pub response_value: Option<OctetString>,
}

/// Provides a general mechanism for defining single-request/multiple-response
/// operations in LDAP.
///
/// This message is intended to be used in conjunction with the Extended
/// operation to define new single-request/multiple-response operations or in
/// conjunction with a control when extending existing LDAP operations in a way
/// that requires them to return [`IntermediateResponse`] information.
#[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(tag(application, 25))]
pub struct IntermediateResponse {
    /// The unique [`LdapOid`] corresponding to the extended operation.
    #[rasn(tag(0))]
    pub response_name: Option<LdapOid>,
    /// Information specific to the extended operation.
    #[rasn(tag(1))]
    pub response_value: Option<OctetString>,
}
