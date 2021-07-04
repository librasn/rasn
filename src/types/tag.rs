pub(crate) use self::consts::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Class {
    Universal = 0,
    Application,
    Context,
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

/// An abstract representation of the tag octets used in BER, CER, and
/// DER to identify .
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Tag {
    pub class: Class,
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

    /// Checks that a slice of tags is a distinct set in a way that is `const`
    /// compatible.
    pub const fn is_distinct_set(set: &[Self]) -> bool {
        let mut index = 0;
        while index < set.len() {
            let needle = &set[index];
            let mut after = index + 1;
            while after < set.len() {
                if needle.const_eq(set[after]) && !needle.const_eq(Tag::EOC) {
                    return false;
                } else {
                    after += 1;
                }
            }

            index += 1;
        }

        true
    }

    pub(crate) const fn const_eq(self, rhs: Self) -> bool {
        self.class as u8 == rhs.class as u8 && self.value == rhs.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
