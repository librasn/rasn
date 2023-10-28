use crate::types::{Class, Tag};

/// A BER Identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    /// The ASN.1 tag.
    pub tag: Tag,
    /// Whether a type is using `constructed` or `primitive` encoding.
    pub(crate) is_constructed: bool,
}

impl Identifier {
    /// Instantiates a new instance of `Identifier` from its components.
    #[must_use]
    pub fn new(class: Class, is_constructed: bool, tag: u32) -> Self {
        Self {
            tag: Tag::new(class, tag),
            is_constructed,
        }
    }

    /// Instantiates a new instance of `Identifier` from its components.
    #[must_use]
    pub fn from_tag(tag: Tag, is_constructed: bool) -> Self {
        Self {
            tag,
            is_constructed,
        }
    }

    /// Instantiates a new tag from `self` with `tag` overwritten.
    #[must_use]
    pub fn tag(self, tag: u32) -> Self {
        Self {
            tag: self.tag.set_value(tag),
            is_constructed: self.is_constructed,
        }
    }

    /// Returns whether the identifier is for a type that is using
    /// "constructed" encoding.
    #[must_use]
    pub fn is_constructed(&self) -> bool {
        self.is_constructed
    }

    /// Returns whether the identifier is for a type that is using
    /// "primitive" encoding.
    #[must_use]
    pub fn is_primitive(&self) -> bool {
        !self.is_constructed()
    }
}

impl core::ops::Deref for Identifier {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
