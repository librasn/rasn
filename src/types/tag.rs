pub(crate) use self::consts::*;

/// The class of tag identifying its category.
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

    pub fn is_universal(self) -> bool {
        self == Class::Universal
    }
}

/// An abstract representation of an ASN.1 tag that uniquely identifies a type
/// within a ASN.1 module for codecs.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Tag {
    /// The class of the tag.
    pub class: Class,
    /// The sub-class of the tag.
    pub value: u32,
}

macro_rules! consts {
    ($($name:ident = $value:expr),+) => {
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
    BMP_STRING = 30
}

impl Tag {
    pub const fn new(class: Class, value: u32) -> Self {
        Self { class, value }
    }

    pub fn set_value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    #[doc(hidden)]
    pub const fn const_eq(self, rhs: &Self) -> bool {
        self.class as u8 == rhs.class as u8 && self.value == rhs.value
    }
}

/// The root or node in tree reprensenting all of potential tags in a ASN.1 type.
/// For most types this is only ever one level deep, except for CHOICE enums
/// which will contain a set of nodes, that either point to a `Leaf` or another
/// level of `Choice`.
#[derive(Debug)]
pub enum TagTree {
    Leaf(Tag),
    Choice(&'static [TagTree]),
}

impl TagTree {
    /// Checks whether a given set of nodes only contains unique entries.
    pub const fn is_unique(nodes: &'static [Self]) -> bool {
        let mut index = 0;

        while index < nodes.len() {
            match &nodes[index] {
                TagTree::Choice(inner_tags) => {
                    if !Self::is_unique(inner_tags) {
                        return false;
                    }

                    let mut inner_index = 0;
                    while inner_index < inner_tags.len() {
                        if Self::tree_contains(
                            &inner_tags[inner_index],
                            konst::slice::slice_from(&nodes, index + 1),
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

                    if Self::tag_contains(
                        tag,
                        konst::slice::slice_from(&nodes, index + 1),
                    ) {
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
    const fn tag_contains(needle: &Tag, nodes: &'static [TagTree]) -> bool {
        let mut index = 0;

        while index < nodes.len() {
            match &nodes[index] {
                TagTree::Choice(nodes) => {
                    if Self::tag_contains(needle, nodes) {
                        return true;
                    }
                }

                TagTree::Leaf(tag) => {
                    if tag.const_eq(&needle) {
                        return true;
                    }
                }
            }

            index += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: &'static [TagTree] = &[
        TagTree::Leaf(Tag::EOC),
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
    ];

    const INVALID_FLAT: &'static [TagTree] = &[
        TagTree::Leaf(Tag::BIT_STRING),
        TagTree::Leaf(Tag::new(Class::Application, 0)),
        TagTree::Leaf(Tag::new(Class::Application, 0)),
        TagTree::Leaf(Tag::new(Class::Context, 0)),
        TagTree::Leaf(Tag::new(Class::Context, 0)),
        TagTree::Leaf(Tag::new(Class::Private, 0)),
        TagTree::Leaf(Tag::new(Class::Private, 0)),
    ];

    const INVALID_NESTED: &'static [TagTree] = &[
        TagTree::Leaf(Tag::EOC),
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
    ];

    #[test]
    fn is_unique() {
        let _ = EXPECTED;
        let _ = INVALID_FLAT;
        let _ = INVALID_NESTED;

        crate::sa::const_assert!(TagTree::is_unique(EXPECTED));
        crate::sa::const_assert!(!TagTree::is_unique(INVALID_FLAT));
        crate::sa::const_assert!(!TagTree::is_unique(INVALID_NESTED));
    }

    #[test]
    fn canonical_ordering() {
        let mut tags = [
            Tag::EOC,
            Tag::new(Class::Application, 0),
            Tag::BIT_STRING,
            Tag::new(Class::Application, 1),
            Tag::new(Class::Private, 1),
            Tag::new(Class::Private, 0),
            Tag::new(Class::Context, 2),
            Tag::new(Class::Context, 0),
        ];
        let expected = [
            Tag::EOC,
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
