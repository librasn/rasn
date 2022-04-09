use std::fmt;

use variation::Variation;

use super::{Assignment, DefinedValue, ObjectIdentifier};

#[derive(Debug)]
pub struct Module {
    pub identifier: ModuleIdentifier,
    pub tag: Tag,
    pub extension: Option<()>,
    pub exports: Exports,
    pub imports: Vec<(ModuleReference, Vec<String>)>,
    pub assignments: Vec<Assignment>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ModuleIdentifier {
    pub name: String,
    pub identification: ObjectIdentifier,
    // iri: Option<Iri>
}

impl ModuleIdentifier {
    pub fn new(name: String) -> Self {
        Self {
            name,
            identification: ObjectIdentifier::new(),
        }
    }
}

impl fmt::Display for ModuleIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.name, self.identification)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Tag {
    Explicit,
    Implicit,
    Automatic,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Exports {
    All,
    Symbols(Vec<String>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ModuleReference {
    name: String,
    identification: Option<AssignedIdentifier>,
}

impl ModuleReference {
    pub fn new(name: String, identification: Option<AssignedIdentifier>) -> Self {
        Self {
            name,
            identification,
        }
    }

    pub fn as_identifier(&self) -> Option<ModuleIdentifier> {
        Some(ModuleIdentifier {
            name: self.name.clone(),
            identification: match self.identification.as_ref()? {
                AssignedIdentifier::ObjectIdentifier(oid) => oid.clone(),
                _ => return None,
            },
        })
    }

    pub fn has_identification(&self) -> bool {
        self.identification.is_some()
    }

    pub fn as_identification_mut(&mut self) -> Option<&mut AssignedIdentifier> {
        match self.identification {
            Some(ref mut id) => Some(id),
            _ => None,
        }
    }

    pub fn identification_uses_defined_value(&self) -> bool {
        self.identification
            .as_ref()
            .map(|i| i.is_defined())
            .unwrap_or(false)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum AssignedIdentifier {
    ObjectIdentifier(ObjectIdentifier),
    Defined(DefinedValue),
}
