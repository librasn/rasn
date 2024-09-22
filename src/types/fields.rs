//! Representing all fields for a `SEQUENCE` or `SET` type.

use alloc::borrow::Cow;

use crate::types::{Tag, TagTree};

/// Represents all of the values that make up a given value in ASN.1.
#[derive(Debug, Clone)]
pub struct Fields {
    fields: Cow<'static, [Field]>,
}

impl Fields {
    /// Creates a new set of fields from a given value.
    pub const fn new(fields: Cow<'static, [Field]>) -> Self {
        Self { fields }
    }

    /// Creates an empty set of fields.
    pub const fn empty() -> Self {
        Self::new(Cow::Borrowed(&[]))
    }

    /// Creates a set of fields from a static set.
    pub const fn from_static(fields: &'static [Field]) -> Self {
        Self {
            fields: Cow::Borrowed(fields),
        }
    }

    /// Returns the number of fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the set doesn't contain any fields.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns whether the set contains any fields.
    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Returns an iterator over all fields which are [FieldPresence::Optional] or
    /// [FieldPresence::Default].
    pub fn optional_and_default_fields(&self) -> impl Iterator<Item = Field> + '_ {
        self.iter().filter(Field::is_optional_or_default)
    }

    /// Returns the number of fields which are [FieldPresence::Optional] or
    /// [FieldPresence::Default].
    pub fn number_of_optional_and_default_fields(&self) -> usize {
        self.optional_and_default_fields().count()
    }

    /// Returns the canonical sorted version of `self`.
    pub fn canonised(mut self) -> Self {
        self.canonical_sort();
        self
    }

    /// Sorts the fields by their canonical tag order.
    pub fn canonical_sort(&mut self) {
        self.fields
            .to_mut()
            .sort_by(|a, b| a.tag_tree.smallest_tag().cmp(&b.tag_tree.smallest_tag()));
    }

    /// Returns an iterator over all fields.
    pub fn iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.fields.iter().cloned()
    }

    /// Returns an iterator over identifiers for all fields.
    pub fn identifiers(&self) -> impl Iterator<Item = &str> + '_ {
        self.fields.iter().map(|f| f.name)
    }
}

impl Default for Fields {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<Cow<'static, [Field]>> for Fields {
    fn from(fields: Cow<'static, [Field]>) -> Self {
        Self::new(fields)
    }
}

/// Represents a field in a `SET` or `SEQUENCE` type.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Field {
    /// The tag for the field.
    pub tag: Tag,
    /// The tree of tags for the field, if the field is a `CHOICE` type.
    pub tag_tree: TagTree,
    /// The presence of the field.
    pub presence: FieldPresence,
    /// The name of the field.
    pub name: &'static str,
}

impl Field {
    /// Creates a new field with [FieldPresence::Required] from the given values.
    pub const fn new_required(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Required,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Required] from `T::AsnType`.
    pub const fn new_required_type<T: crate::types::AsnType>(name: &'static str) -> Self {
        Self {
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Required,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Optional] from the given values.
    pub const fn new_optional(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Optional,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Optional] from `T::AsnType`.
    pub const fn new_optional_type<T: crate::types::AsnType>(name: &'static str) -> Self {
        Self {
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Optional,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Default] from the given values.
    pub const fn new_default(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Default,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Default] from `T::AsnType`.
    pub const fn new_default_type<T: crate::types::AsnType>(name: &'static str) -> Self {
        Self {
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Default,
            name,
        }
    }
}

impl Field {
    /// Returns whether the field is [FieldPresence::Optional] or [FieldPresence::Default].
    pub const fn is_optional_or_default(&self) -> bool {
        self.presence.is_optional_or_default()
    }

    /// Returns whether the field is [FieldPresence::Required].
    pub const fn is_not_optional_or_default(&self) -> bool {
        !self.is_optional_or_default()
    }
}

/// The presence of a field in constructed type, used to determine whether to
/// expect a given field when encoding or decoding, and how to react when it
/// is not present.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum FieldPresence {
    /// Value for the field is required, and will cause an error if not found.
    Required,
    /// Value for the field is optional, and will return none value if not found.
    Optional,
    /// Value for the field is default, and will return a default value if not found.
    #[default]
    Default,
}

impl FieldPresence {
    /// Returns whether the current values matches [FieldPresence::Optional] or
    /// [FieldPresence::Default].
    pub const fn is_optional_or_default(&self) -> bool {
        matches!(self, Self::Optional | Self::Default)
    }
}
