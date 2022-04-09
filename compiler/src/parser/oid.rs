use std::{
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
};

use derefable::Derefable;
use variation::Variation;

use super::values::Number;

#[derive(Clone, Debug, Derefable, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ObjectIdentifier(#[deref(mutable)] Vec<ObjIdComponent>);

impl ObjectIdentifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_components(components: Vec<ObjIdComponent>) -> Self {
        Self(components)
    }

    pub fn is_absolute(&self) -> bool {
        !self.is_relative()
    }

    pub fn is_relative(&self) -> bool {
        for component in &self.0 {
            if component.is_reserved_name_form() {
                continue;
            } else if component.is_name() {
                return true;
            }
        }

        false
    }

    pub fn replace(&mut self, map: &BTreeMap<String, ObjectIdentifier>) {
        for (name, id) in map {
            while let Ok(index) = self.0.binary_search(&ObjIdComponent::Name(name.clone())) {
                self.0.splice(index..(index + 1), id.0.iter().cloned());
            }
        }
    }
}

impl fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ ")?;
        for component in &self.0 {
            write!(f, "{} ", component)?;
        }

        write!(f, "}}")
    }
}

#[derive(Clone, Debug, Eq, PartialOrd, Ord, Variation)]
pub enum ObjIdComponent {
    Name(String),
    Number(Number),
    NameAndNumber(String, Number),
}

impl ObjIdComponent {
    pub fn is_reserved_name_form(&self) -> bool {
        match self {
            ObjIdComponent::NameAndNumber(name, _) | ObjIdComponent::Name(name) => match &**name {
                "ITU-T" | "itu-t" | "ccitt" | "ISO" | "iso" | "Joint-ISO-ITU-T"
                | "joint-iso-itu-t" | "joint-iso-ccitt" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl fmt::Display for ObjIdComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjIdComponent::Name(name) => name.fmt(f),
            ObjIdComponent::Number(number) => number.fmt(f),
            ObjIdComponent::NameAndNumber(name, number) => write!(f, "{}({})", name, number),
        }
    }
}

impl Hash for ObjIdComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ObjIdComponent::Name(s) => s.hash(state),
            ObjIdComponent::Number(n) => n.hash(state),
            ObjIdComponent::NameAndNumber(s, _) if self.is_reserved_name_form() => s.hash(state),
            ObjIdComponent::NameAndNumber(s, n) => {
                s.hash(state);
                n.hash(state);
            }
        }
    }
}

impl PartialEq for ObjIdComponent {
    fn eq(&self, rhs: &Self) -> bool {
        if self.is_reserved_name_form() {
            let self_name = match self {
                ObjIdComponent::Name(name) | ObjIdComponent::NameAndNumber(name, _) => name,
                _ => unreachable!(),
            };

            if rhs.is_reserved_name_form() {
                let rhs_name = match self {
                    ObjIdComponent::Name(name) | ObjIdComponent::NameAndNumber(name, _) => name,
                    _ => unreachable!(),
                };

                self_name.to_lowercase() == rhs_name.to_lowercase()
            } else {
                false
            }
        } else {
            match (self, rhs) {
                (ObjIdComponent::Name(lhs), ObjIdComponent::Name(rhs)) => lhs == rhs,
                (ObjIdComponent::Number(lhs), ObjIdComponent::Number(rhs)) => lhs == rhs,
                (
                    ObjIdComponent::NameAndNumber(lhs_s, lhs_n),
                    ObjIdComponent::NameAndNumber(rhs_s, rhs_n),
                ) => lhs_s == rhs_s && lhs_n == rhs_n,
                _ => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn two_oids() -> (ObjectIdentifier, ObjectIdentifier) {
        (
            ObjectIdentifier::from_components(vec![
                ObjIdComponent::NameAndNumber("joint-iso-itu-t".into(), 2.into()),
                ObjIdComponent::NameAndNumber("ds".into(), 5.into()),
                ObjIdComponent::NameAndNumber("module".into(), 1.into()),
                ObjIdComponent::NameAndNumber("usefulDefinitions".into(), 0.into()),
                ObjIdComponent::Number(3.into()),
            ]),
            ObjectIdentifier::from_components(vec![
                ObjIdComponent::Name("joint-iso-itu-t".into()),
                ObjIdComponent::NameAndNumber("ds".into(), 5.into()),
                ObjIdComponent::NameAndNumber("module".into(), 1.into()),
                ObjIdComponent::NameAndNumber("usefulDefinitions".into(), 0.into()),
                ObjIdComponent::Number(3.into()),
            ]),
        )
    }

    #[test]
    fn oid_partialeq() {
        let (a, b) = two_oids();
        assert_eq!(a, b);
        assert_eq!(b, a);
    }

    #[test]
    fn oid_hash() {
        let (a, b) = two_oids();

        let mut a_hasher = DefaultHasher::new();
        a.hash(&mut a_hasher);

        let mut b_hasher = DefaultHasher::new();
        b.hash(&mut b_hasher);

        assert_eq!(a_hasher.finish(), b_hasher.finish());
    }
}
