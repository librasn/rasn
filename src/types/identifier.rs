/// The identifier of an ASN.1 type in an ASN.1 module. In most cases,
/// the identifier is the human-readable type name as defined in the ASN.1 module.
/// For built-in ASN.1 types, the XML tag names as defined in ITU-T X.680 (2021/02) §12.36
/// are used. Some ASN.1 types, most notably open types, do not have a consistent identifier.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Identifier(pub Option<&'static str>);

impl Identifier {
    /// Empty identifier value
    pub const EMPTY: Self = Self(None);
    /// Identifier for the built-in Bit String type
    pub const BIT_STRING: Self = Self(Some("BIT_STRING"));
    /// Identifier for the built-in Bool type
    pub const BOOL: Self = Self(Some("BOOLEAN"));
    /// Identifier for the built-in Choice type
    pub const CHOICE: Self = Self(Some("CHOICE"));
    /// Identifier for the built-in Date type
    pub const DATE: Self = Self(Some("DATE"));
    /// Identifier for the built-in Date Time type
    pub const DATE_TIME: Self = Self(Some("DATE_TIME"));
    /// Identifier for the built-in Duration type
    pub const DURATION: Self = Self(Some("DURATION"));
    /// Identifier for the built-in Embedded PDV type
    pub const EMBEDDED_PDV: Self = Self(Some("SEQUENCE"));
    /// Identifier for the built-in Enumerated type
    pub const ENUMERATED: Self = Self(Some("ENUMERATED"));
    /// Identifier for the built-in External type
    pub const EXTERNAL: Self = Self(Some("SEQUENCE"));
    /// Identifier for the built-in `Instance_Of` type
    pub const INSTANCE_OF: Self = Self(Some("SEQUENCE"));
    /// Identifier for the built-in Integer type
    pub const INTEGER: Self = Self(Some("INTEGER"));
    /// Identifier for the built-in Iri type
    pub const IRI: Self = Self(Some("OID_IRI"));
    /// Identifier for the built-in Null type
    pub const NULL: Self = Self(Some("NULL"));
    /// Identifier for the built-in `Object_Identifier` type
    pub const OBJECT_IDENTIFIER: Self = Self(Some("OBJECT_IDENTIFIER"));
    /// Identifier for the built-in Octet String type
    pub const OCTET_STRING: Self = Self(Some("OCTET_STRING"));
    /// Identifier for the built-in Real type
    pub const REAL: Self = Self(Some("REAL"));
    /// Identifier for the built-in Relative Iri type
    pub const RELATIVE_IRI: Self = Self(Some("RELATIVE_OID_IRI"));
    /// Identifier for the built-in Relative Oid type
    pub const RELATIVE_OID: Self = Self(Some("RELATIVE_OID"));
    /// Identifier for the built-in Sequence type
    pub const SEQUENCE: Self = Self(Some("SEQUENCE"));
    /// Identifier for the built-in Sequence Of type
    pub const SEQUENCE_OF: Self = Self(Some("SEQUENCE_OF"));
    /// Identifier for the built-in Set type
    pub const SET: Self = Self(Some("SET"));
    /// Identifier for the built-in Set Of type
    pub const SET_OF: Self = Self(Some("SET_OF"));
    /// Identifier for the built-in Time type
    pub const TIME: Self = Self(Some("TIME"));
    /// Identifier for the built-in Time Of Day type
    pub const TIME_OF_DAY: Self = Self(Some("TIME_OF_DAY"));
    /// Identifier for the built-in Unrestricted Character String type
    pub const UNRESTRICTED_CHARACTER_STRING: Self = Self(Some("SEQUENCE"));
    /// Identifier for the built-in Bmp String type
    pub const BMP_STRING: Self = Self(Some("BMPString"));
    /// Identifier for the built-in Ia5 String type
    pub const IA5_STRING: Self = Self(Some("IA5String"));
    /// Identifier for the built-in General String type
    pub const GENERAL_STRING: Self = Self(Some("GeneralString"));
    /// Identifier for the built-in Graphic String type
    pub const GRAPHIC_STRING: Self = Self(Some("GraphicString"));
    /// Identifier for the built-in Numeric String type
    pub const NUMERIC_STRING: Self = Self(Some("NumericString"));
    /// Identifier for the built-in Printable String type
    pub const PRINTABLE_STRING: Self = Self(Some("PrintableString"));
    /// Identifier for the built-in Teletex String type
    pub const TELETEX_STRING: Self = Self(Some("TeletexString"));
    /// Identifier for the built-in Visible String type
    pub const VISIBLE_STRING: Self = Self(Some("VisibleString"));
    /// Identifier for the built-in Utf8 String type
    pub const UTF8_STRING: Self = Self(Some("UTF8String"));
    /// Identifier for the built-in Generalized Time type
    pub const GENERALIZED_TIME: Self = Self(Some("GeneralizedTime"));
    /// Identifier for the built-in Utc Time type
    pub const UTC_TIME: Self = Self(Some("UTCTime"));

    /// Returns a reference of `self` if the identifier is not empty, or `other` if it is.
    #[must_use]
    pub fn or(&self, other: Self) -> Self {
        if self.0.is_none() { other } else { *self }
    }

    /// Returns the undelying string or panics if identifier is empty.
    ///
    /// ## Panics
    /// Panics if the self value equals None.
    #[must_use]
    pub fn unwrap(&self) -> &'static str {
        self.0.unwrap()
    }
}
