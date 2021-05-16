pub trait Visitor<'de> {
    type Value;

    /// Peek at the next available tag.
    fn peek_tag(&self) -> Result<Tag, Self::Error>;

    /// Decode a `BIT STRING` identified by `tag` from the available input.
    fn visit_bit_string<const TAG: Tag>(self, value: types::BitString) -> Result<Self::Value, Self::Error>;

    /// Decode a `BOOL` identified by `tag` from the available input.
    fn visit_bool<const TAG: Tag>(self, value: bool) -> Result<Self::Value, Self::Error>;

    /// Decode a `INTEGER` identified by `tag` from the available input.
    fn visit_integer(self, value: &'de [u8]) -> Result<Self::Value, Self::Error>;

    /// Decode `NULL` identified by `tag` from the available input.
    fn visit_null<const TAG: Tag>(self) -> Result<Self::Value, Self::Error>;

    /// Decode a `OBJECT IDENTIFIER` identified by `tag` from the available input.
    fn visit_object_identifier<const TAG: Tag>(
        self,
        value: types::ObjectIdentifier,
    ) -> Result<Self::Value, Self::Error>;

    /// Decode a `SEQUENCE` identified by `tag` from the available input. Returning
    /// a new `Decoder` containing the sequence's contents to be decoded.
    fn visit_sequence<const TAG: Tag>(self) -> Result<Self::Value, Self::Error>;

    /// Decode a `SEQUENCE OF D` where `D: Decode` identified by `tag` from the available input.
    fn visit_sequence_of<S: SequenceOf, const TAG: Tag>(&mut self) -> Result<S, Self::Error>;

    /// Decode a `SET` identified by `tag` from the available input. Returning
    /// a new `Decoder` containing the sequence's contents to be decoded.
    fn visit_set(self, value: S) -> Result<Self, Self::Error>;

    /// Decode a `SET OF D` where `D: Decode` identified by `tag` from the available input.
    fn visit_set_of<D: Decode + Ord>(&mut self, tag: Tag) -> Result<BTreeSet<D>, Self::Error>;

    /// Decode a `OCTET STRING` identified by `tag` from the available input.
    fn decode_octet_string(&mut self, tag: Tag) -> Result<Vec<u8>, Self::Error>;

    /// Decode a `UTF8 STRING` identified by `tag` from the available input.
    fn decode_utf8_string(&mut self, tag: Tag) -> Result<types::Utf8String, Self::Error>;

    /// Decode an ASN.1 value that has been explicitly prefixed with `tag` from the available input.
    fn decode_explicit_prefix<D: Decode>(&mut self, tag: Tag) -> Result<D, Self::Error>;

    /// Decode a `UtcTime` identified by `tag` from the available input.
    fn decode_utc_time(&mut self, tag: Tag) -> Result<types::UtcTime, Self::Error>;

    /// Decode a `GeneralizedTime` identified by `tag` from the available input.
    fn decode_generalized_time(&mut self, tag: Tag) -> Result<types::GeneralizedTime, Self::Error>;

    /// Decode a `OBJECT IDENTIFIER` identified by `tag` from the available input.
    /// This is a specialisation of [`Self::decode_object_identifier`] for
    /// formats where you can zero copy the input.
    fn decode_oid(self, value: &'de Oid) -> Result<Self::Value, Self::Error>;

    /// Decode an enumerated enum's discriminant identified by `tag` from the available input.
    fn decode_enumerated<const TAG: Tag>(&mut self, value: u32) -> Result<types::Integer, Self::Error>;
}
