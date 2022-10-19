use alloc::borrow::Cow;

use crate::types::{TagTree};

#[derive(Debug, Clone)]
pub struct Fields {
    fields: Cow<'static, [Field]>,
}

impl Fields {
    pub const fn new(fields: Cow<'static, [Field]>) -> Self {
        Self { fields }
    }

    pub const fn from_static(fields: &'static [Field]) -> Self {
        Self { fields: Cow::Borrowed(fields) }
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
        self.fields.to_mut().sort_by(|a, b| a.tag.smallest_tag().cmp(&b.tag.smallest_tag()));
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

#[derive(Debug, Clone)]
pub struct Field {
    pub tag: TagTree,
    pub presence: FieldPresence,
}

impl Field {
    pub const fn new_required(tag: TagTree) -> Self {
        Self {
            tag,
            presence: FieldPresence::Required,
        }
    }

    pub const fn new_optional(tag: TagTree) -> Self {
        Self {
            tag,
            presence: FieldPresence::Optional,
        }
    }

    pub const fn new_default(tag: TagTree) -> Self {
        Self {
            tag,
            presence: FieldPresence::Default,
        }
    }

    pub const fn is_optional_or_default(&self) -> bool {
        self.presence.is_optional_or_default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
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
