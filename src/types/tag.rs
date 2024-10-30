#![allow(clippy::upper_case_acronyms)]

pub(crate) use self::consts::*;
use alloc::string::ToString;

/// The class of tag identifying its category.
///
/// The order of the variants is equal to the canonical type order for tags,
/// which allows us to use [Ord] to get the canonical ordering.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Class {
    /// Types defined in X.680.
    Universal = 0,
    /// Application specific types.
    Application,
    /// Context specific types (e.g. fields in a struct)
    Context,
    /// Private types.
    Private,
}

impl Class {
    /// Instantiate a `Class` from a u8.
    ///
    /// # Panics
    /// If `value` is greater than 3.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Class::Universal,
            1 => Class::Application,
            2 => Class::Context,
            3 => Class::Private,
            num => panic!("'{}' is not a valid class.", num),
        }
    }

    /// Returns whether the given class is universal.
    pub fn is_universal(self) -> bool {
        self == Class::Universal
    }
}

impl core::fmt::Display for Class {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(match self {
            Self::Universal => "universal",
            Self::Application => "application",
            Self::Context => "context",
            Self::Private => "private",
        })
    }
}

/// An abstract representation of an ASN.1 tag that uniquely identifies a type
/// within a ASN.1 module for codecs.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Tag {
    /// The class of the tag.
    pub class: Class,
    /// The sub-class of the tag.
    pub value: u32,
}
impl Tag {
    /// Constant implementation for [Ord] for [Tag].
    pub const fn const_cmp(&self, other: &Tag) -> core::cmp::Ordering {
        if (self.class as u8) < (other.class as u8) {
            core::cmp::Ordering::Less
        } else if (self.class as u8) > (other.class as u8) {
            core::cmp::Ordering::Greater
        } else {
            // Classes are equal, compare values
            if self.value < other.value {
                core::cmp::Ordering::Less
            } else if self.value > other.value {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Equal
            }
        }
    }
}

/// Implement display for Tag; represents `class` as string and `value` as number.
impl core::fmt::Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(match self.class {
            Class::Universal => "Universal",
            Class::Application => "Application",
            Class::Context => "Context",
            Class::Private => "Private",
        })?;
        f.write_str(" ")?;
        f.write_str(&self.value.to_string())
    }
}

macro_rules! consts {
    ($($name:ident = $value:expr),+) => {
        #[allow(missing_docs)]
        impl Tag {
            $(
                pub const $name: Tag = Tag::new(Class::Universal, $value);
            )+
        }

        #[allow(non_camel_case_types)]
        pub mod consts {
            use super::*;

            $(
                #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
                pub struct $name;

                impl crate::types::AsnType for $name {
                    const TAG: Tag = Tag::$name;
                }
            )+
        }

    }
}

consts! {
    EOC = 0,
    BOOL = 1,
    INTEGER = 2,
    BIT_STRING = 3,
    OCTET_STRING = 4,
    NULL = 5,
    OBJECT_IDENTIFIER = 6,
    OBJECT_DESCRIPTOR = 7,
    EXTERNAL = 8,
    REAL = 9,
    ENUMERATED = 10,
    EMBEDDED_PDV = 11,
    UTF8_STRING = 12,
    RELATIVE_OID = 13,
    SEQUENCE = 16,
    SET = 17,
    NUMERIC_STRING = 18,
    PRINTABLE_STRING = 19,
    TELETEX_STRING = 20,
    VIDEOTEX_STRING = 21,
    IA5_STRING = 22,
    UTC_TIME = 23,
    GENERALIZED_TIME = 24,
    GRAPHIC_STRING = 25,
    VISIBLE_STRING = 26,
    GENERAL_STRING = 27,
    UNIVERSAL_STRING = 28,
    CHARACTER_STRING = 29,
    BMP_STRING = 30,
    DATE = 31
}

impl Tag {
    /// The `Tag` constant to use to represent `CHOICE` type's `AsnType::TAG`.
    pub const CHOICE: Self = Self::EOC;

    /// Create a new tag from `class` and `value`.
    pub const fn new(class: Class, value: u32) -> Self {
        Self { class, value }
    }

    /// Create a new `APPLICATION` tag from `value`.
    pub const fn new_application(value: u32) -> Self {
        Self::new(Class::Application, value)
    }

    /// Create a new `CONTEXT` tag from `value`.
    pub const fn new_context(value: u32) -> Self {
        Self::new(Class::Context, value)
    }

    /// Create a new `PRIVATE` tag from `value`.
    pub const fn new_private(value: u32) -> Self {
        Self::new(Class::Private, value)
    }

    /// Set the value of the tag.
    pub fn set_value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    #[doc(hidden)]
    pub const fn const_eq(self, rhs: &Self) -> bool {
        self.class as u8 == rhs.class as u8 && self.value == rhs.value
    }

    #[doc(hidden)]
    pub const fn const_less_than(self, rhs: Self) -> bool {
        (self.class as u8) < (rhs.class as u8) && self.value < rhs.value
    }

    /// Returns whether `Tag` is defined as `Tag::EOC`, and thus is an invalid
    /// tag and must be CHOICE structure.
    pub const fn is_choice(&self) -> bool {
        self.const_eq(&Tag::CHOICE)
    }
}

/// The root or node in tree representing all of potential tags in a ASN.1 type.
///
/// For most types this is only ever one level deep, except for CHOICE enums
/// which will contain a set of nodes, that either point to a `Leaf` or another
/// level of `Choice`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TagTree {
    /// The end of branch in the tree.
    Leaf(Tag),
    /// A branch in the tree.
    Choice(&'static [TagTree]),
}

impl TagTree {
    /// Returns an empty tree.
    pub const fn empty() -> Self {
        Self::Choice(&[])
    }

    /// Returns the tag with the smallest possible value from the tree.
    pub const fn smallest_tag(&self) -> Tag {
        match self {
            Self::Leaf(tag) => *tag,
            Self::Choice(tree) => {
                let mut i = 0;
                let mut tag: Tag = Tag::new_private(u32::MAX);

                while i < tree.len() {
                    let next_tag = tree[i].smallest_tag();
                    if next_tag.const_less_than(tag) {
                        tag = next_tag;
                    }

                    i += 1;
                }

                tag
            }
        }
    }

    /// Returns whether a given `TagTree` only contains unique entries.
    pub const fn is_unique(&self) -> bool {
        match self {
            Self::Choice(tree) => Self::is_unique_set(tree),
            Self::Leaf(_) => true,
        }
    }

    /// Checks whether a given set of nodes only contains unique entries.
    pub(crate) const fn is_unique_set(nodes: &'static [Self]) -> bool {
        let mut index = 0;

        while index < nodes.len() {
            match &nodes[index] {
                TagTree::Choice(inner_tags) => {
                    if !Self::is_unique_set(inner_tags) {
                        return false;
                    }

                    let mut inner_index = 0;
                    while inner_index < inner_tags.len() {
                        if Self::tree_contains(
                            &inner_tags[inner_index],
                            nodes.split_at(index + 1).1,
                        ) {
                            return false;
                        }

                        inner_index += 1;
                    }
                }

                TagTree::Leaf(tag) => {
                    // We're at the last element so there's nothing more to
                    // compare to.
                    if index + 1 == nodes.len() {
                        return true;
                    }

                    if Self::tag_contains(tag, nodes.split_at(index + 1).1) {
                        return false;
                    }
                }
            }

            index += 1;
        }

        true
    }

    /// Whether any `Leaf` in `needle` matches any `Leaf`s in `nodes`.
    const fn tree_contains(needle: &TagTree, nodes: &'static [TagTree]) -> bool {
        match needle {
            TagTree::Choice(inner_tags) => {
                let mut inner_index = 0;
                while inner_index < inner_tags.len() {
                    if Self::tree_contains(&inner_tags[inner_index], nodes) {
                        return true;
                    }

                    inner_index += 1;
                }
                false
            }

            TagTree::Leaf(tag) => {
                if Self::tag_contains(tag, nodes) {
                    return true;
                }

                false
            }
        }
    }

    /// Whether `needle` matches any `Leaf`s in `nodes`.
    pub const fn tag_contains(needle: &Tag, nodes: &[TagTree]) -> bool {
        let mut index = 0;

        while index < nodes.len() {
            match &nodes[index] {
                TagTree::Choice(nodes) => {
                    if Self::tag_contains(needle, nodes) {
                        return true;
                    }
                }

                TagTree::Leaf(tag) => {
                    if tag.const_eq(needle) {
                        return true;
                    }
                }
            }

            index += 1;
        }

        false
    }
}

impl PartialOrd for TagTree {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TagTree {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.smallest_tag().cmp(&other.smallest_tag())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _EXPECTED: TagTree = TagTree::Choice(&[
        TagTree::Leaf(Tag::CHOICE),
        TagTree::Leaf(Tag::BIT_STRING),
        TagTree::Choice(&[
            TagTree::Leaf(Tag::new(Class::Application, 0)),
            TagTree::Leaf(Tag::new(Class::Application, 1)),
        ]),
        TagTree::Choice(&[
            TagTree::Leaf(Tag::new(Class::Context, 0)),
            TagTree::Leaf(Tag::new(Class::Context, 2)),
        ]),
        TagTree::Leaf(Tag::new(Class::Private, 0)),
        TagTree::Leaf(Tag::new(Class::Private, 1)),
    ]);

    const _INVALID_FLAT: TagTree = TagTree::Choice(&[
        TagTree::Leaf(Tag::BIT_STRING),
        TagTree::Leaf(Tag::new(Class::Application, 0)),
        TagTree::Leaf(Tag::new(Class::Application, 0)),
        TagTree::Leaf(Tag::new(Class::Context, 0)),
        TagTree::Leaf(Tag::new(Class::Context, 0)),
        TagTree::Leaf(Tag::new(Class::Private, 0)),
        TagTree::Leaf(Tag::new(Class::Private, 0)),
    ]);

    const _INVALID_NESTED: TagTree = TagTree::Choice(&[
        TagTree::Leaf(Tag::CHOICE),
        TagTree::Leaf(Tag::BIT_STRING),
        TagTree::Choice(&[
            TagTree::Leaf(Tag::new(Class::Application, 0)),
            TagTree::Leaf(Tag::new(Class::Application, 1)),
        ]),
        TagTree::Choice(&[
            TagTree::Choice(&[TagTree::Leaf(Tag::new(Class::Application, 0))]),
            TagTree::Leaf(Tag::new(Class::Application, 0)),
            TagTree::Leaf(Tag::new(Class::Context, 2)),
        ]),
        TagTree::Leaf(Tag::new(Class::Private, 1)),
        TagTree::Leaf(Tag::new(Class::Private, 1)),
    ]);

    #[test]
    fn is_unique() {
        const _: () = assert!(_EXPECTED.is_unique());
        const _: () = assert!(!_INVALID_FLAT.is_unique());
        const _: () = assert!(!_INVALID_NESTED.is_unique());
    }

    #[test]
    fn canonical_ordering() {
        let mut tags = [
            Tag::CHOICE,
            Tag::new(Class::Application, 0),
            Tag::BIT_STRING,
            Tag::new(Class::Application, 1),
            Tag::new(Class::Private, 1),
            Tag::new(Class::Private, 0),
            Tag::new(Class::Context, 2),
            Tag::new(Class::Context, 0),
        ];
        let expected = [
            Tag::CHOICE,
            Tag::BIT_STRING,
            Tag::new(Class::Application, 0),
            Tag::new(Class::Application, 1),
            Tag::new(Class::Context, 0),
            Tag::new(Class::Context, 2),
            Tag::new(Class::Private, 0),
            Tag::new(Class::Private, 1),
        ];

        tags.sort();

        assert_eq!(tags, expected);
    }
}
