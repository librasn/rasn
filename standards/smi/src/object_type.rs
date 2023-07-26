use core::convert::TryInto;

/// A managed Management Information Base (MIB) object.
pub trait ObjectType
where
    Self: TryInto<Self::Syntax>,
    <Self as TryInto<Self::Syntax>>::Error: rasn::enc::Error + core::fmt::Display,
{
    /// The version of SMI syntax that this type uses.
    type SmiSyntax: SmiSyntax;
    /// The abstract syntax for the object type. This must resolve to an
    /// instance of the SMI type.
    type Syntax: Into<Self::SmiSyntax>;
    /// Determines the access level of the object.
    const ACCESS: Access;
    /// The current status of the object.
    const STATUS: Status;
    /// The object identifier for the object.
    const VALUE: &'static rasn::types::Oid;

    /// Converts `self` into its SMI data type.
    fn into_object_syntax(self) -> Result<Self::SmiSyntax, <Self as TryInto<Self::Syntax>>::Error> {
        Ok(self.try_into().map_err(rasn::enc::Error::custom)?.into())
    }
}

/// A trait representing either a `v1` or `v2` SMI syntax.
pub trait SmiSyntax {}

impl SmiSyntax for crate::v1::ObjectSyntax {}
impl SmiSyntax for crate::v2::ObjectSyntax {}

/// The current access provided to the object.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    NotAccessible,
}

/// The current status of the object's implementation.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Status {
    Current,
    Deprecated,
    Obsolete,
}
