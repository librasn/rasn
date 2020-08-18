use crate::tag::{Class, Tag};

/// A BER Identifier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Identifier {
    /// The ASN.1 tag.
    pub tag: Tag,
    /// Whether a type is using `constructed` or `primitive` encoding.
    pub is_constructed: bool,
}

impl Identifier {
    /// Instantiates a new instance of `Identifier` from its components.
    pub fn new(class: Class, is_constructed: bool, tag: u32) -> Self {
        Self {
            tag: Tag::new(class, tag),
            is_constructed,
        }
    }

    /// Instantiates a new tag from `self` with `tag` overwritten.
    pub fn tag(self, tag: u32) -> Self {
        Self {
            tag: self.tag.set_value(tag),
            is_constructed: self.is_constructed,
        }
    }
}

impl core::ops::Deref for Identifier {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}

impl From<Tag> for Identifier {
    fn from(tag: Tag) -> Self {
        Self {
            tag,
            is_constructed: match tag {
                Tag::SEQUENCE | Tag::SET | Tag::EXTERNAL => true,
                _ => false,
            },
        }
    }
}
