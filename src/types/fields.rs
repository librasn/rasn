//! Representing all fields for a `SEQUENCE` or `SET` type.

use crate::types::{Tag, TagTree};

/// Represents all of the values that make up a given value in ASN.1.
#[derive(Debug, Clone, Copy)]
pub struct Fields<const N: usize> {
    fields: [Field; N],
    has_required: bool,
    number_optional_default: usize,
}

impl<const N: usize> Fields<N> {
    /// Creates a set of fields from a static set.
    pub const fn from_static(fields: [Field; N]) -> Self {
        let mut i = 0;
        let (has_required, number_optional_default) = {
            let mut required = false;
            let mut number_opts = 0;
            while i < fields.len() {
                if fields[i].is_not_optional_or_default() {
                    required = true;
                } else {
                    number_opts += 1;
                }
                i += 1;
            }
            (required, number_opts)
        };
        Self {
            fields,
            has_required,
            number_optional_default,
        }
    }

    /// Returns the number of fields.
    pub const fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the set doesn't contain any fields.
    pub const fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns whether the set contains any fields.
    pub const fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
    /// Checks if any field is required.
    pub const fn has_required_field(&self) -> bool {
        self.has_required
    }

    /// Returns an iterator over all fields which are [FieldPresence::Optional] or
    /// [FieldPresence::Default].
    // pub fn optional_and_default_fields(&self) -> impl Iterator<Item = Field> + '_ {
    //     self.iter().filter(Field::is_optional_or_default)
    // }

    /// Returns the number of fields which are [FieldPresence::Optional] or
    /// [FieldPresence::Default].
    pub const fn number_of_optional_and_default_fields(&self) -> usize {
        self.number_optional_default
    }

    /// Returns the canonical sorted version of `self`.
    pub const fn canonised(&self) -> Self {
        self.canonical_sort()
    }

    /// Sorts the fields by their canonical tag order in constant matter.
    pub const fn canonical_sort(mut self) -> Self {
        let len = self.fields.len();
        let mut i = 0;
        while i < len {
            let mut j = i + 1;
            while j < len {
                let tag_i = self.fields[i].tag_tree.smallest_tag();
                let tag_j = self.fields[j].tag_tree.smallest_tag();
                if tag_i.const_cmp(&tag_j) as usize == core::cmp::Ordering::Greater as usize {
                    let temp = self.fields[i];
                    self.fields[i] = self.fields[j];
                    self.fields[j] = temp;
                }
                j += 1;
            }
            i += 1;
        }
        self
    }

    /// Returns an iterator over all fields.
    pub fn iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.fields.iter().copied()
    }

    /// Returns an iterator over identifiers for all fields.
    pub fn identifiers(&self) -> impl Iterator<Item = &str> + '_ {
        self.fields.iter().map(|f| f.name)
    }
}

impl<const N: usize> core::ops::Deref for Fields<N> {
    type Target = [Field];

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

/// Represents a field in a `SET` or `SEQUENCE` type.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Field {
    /// The index of the field (order number in `SEQUENCE` or `SET`).
    pub index: usize,
    /// The tag for the field.
    pub tag: Tag,
    /// The tree of tags for the field, if the field is a `CHOICE` type.
    pub tag_tree: TagTree,
    /// The presence requirement of the field.
    pub presence: FieldPresence,
    /// The name of the field.
    pub name: &'static str,
}

impl Field {
    /// Creates a new field with [FieldPresence::Required] from the given values.
    pub const fn new_required(
        index: usize,
        tag: Tag,
        tag_tree: TagTree,
        name: &'static str,
    ) -> Self {
        Self {
            index,
            tag,
            tag_tree,
            presence: FieldPresence::Required,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Required] from `T::AsnType`.
    pub const fn new_required_type<T: crate::types::AsnType>(
        index: usize,
        name: &'static str,
    ) -> Self {
        Self {
            index,
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Required,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Optional] from the given values.
    pub const fn new_optional(
        index: usize,
        tag: Tag,
        tag_tree: TagTree,
        name: &'static str,
    ) -> Self {
        Self {
            index,
            tag,
            tag_tree,
            presence: FieldPresence::Optional,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Optional] from `T::AsnType`.
    pub const fn new_optional_type<T: crate::types::AsnType>(
        index: usize,
        name: &'static str,
    ) -> Self {
        Self {
            index,
            tag: T::TAG,
            tag_tree: T::TAG_TREE,
            presence: FieldPresence::Optional,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Default] from the given values.
    pub const fn new_default(
        index: usize,
        tag: Tag,
        tag_tree: TagTree,
        name: &'static str,
    ) -> Self {
        Self {
            index,
            tag,
            tag_tree,
            presence: FieldPresence::Default,
            name,
        }
    }

    /// Creates a new field with [FieldPresence::Default] from `T::AsnType`.
    pub const fn new_default_type<T: crate::types::AsnType>(
        index: usize,
        name: &'static str,
    ) -> Self {
        Self {
            index,
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
