use alloc::borrow::Cow;

use crate::types::{Tag, TagTree};

#[derive(Debug, Clone)]
pub struct Fields {
    fields: Cow<'static, [Field]>,
}

impl Fields {
    pub const fn new(fields: Cow<'static, [Field]>) -> Self {
        Self { fields }
    }

    pub const fn empty() -> Self {
        Self::new(Cow::Borrowed(&[]))
    }

    pub const fn from_static(fields: &'static [Field]) -> Self {
        Self {
            fields: Cow::Borrowed(fields),
        }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn optional_and_default_fields(&self) -> impl Iterator<Item = Field> + '_ {
        self.iter().filter(Field::is_optional_or_default)
    }

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

    pub fn iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.fields.iter().cloned()
    }
}

impl From<Cow<'static, [Field]>> for Fields {
    fn from(fields: Cow<'static, [Field]>) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Field {
    pub tag: Tag,
    pub tag_tree: TagTree,
    pub presence: FieldPresence,
    #[cfg(feature = "jer")]
    pub name: &'static str,
}

#[cfg(not(feature = "jer"))]
impl Field {
    pub const fn new_required(tag: Tag, tag_tree: TagTree) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Required,
        }
    }

    pub const fn new_optional(tag: Tag, tag_tree: TagTree) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Optional,
        }
    }

    pub const fn new_default(tag: Tag, tag_tree: TagTree) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Default,
        }
    }
}

#[cfg(feature = "jer")]
impl Field {
    pub const fn new_required(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Required,
            name
        }
    }

    pub const fn new_optional(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Optional,
            name
        }
    }

    pub const fn new_default(tag: Tag, tag_tree: TagTree, name: &'static str) -> Self {
        Self {
            tag,
            tag_tree,
            presence: FieldPresence::Default,
            name
        }
    }
}

impl Field {
    pub const fn is_optional_or_default(&self) -> bool {
        self.presence.is_optional_or_default()
    }

    pub const fn is_not_optional_or_default(&self) -> bool {
        !self.is_optional_or_default()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum FieldPresence {
    Required,
    Optional,
    #[default]
    Default,
}

impl FieldPresence {
    pub const fn is_optional_or_default(&self) -> bool {
        matches!(self, Self::Optional | Self::Default)
    }
}
