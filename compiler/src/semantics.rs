use std::{collections::BTreeMap, mem};

use failure::ensure;
use unwrap_to::unwrap_to;

use crate::{parser::*, registry::*, Result};

#[derive(Debug)]
pub struct SemanticChecker {
    pub imports: BTreeMap<ModuleReference, Vec<String>>,
    pub module: Module,
    pub table: GlobalSymbolTable,
    // object_sets: ValueRegistry,
    // objects: ValueRegistry,
    // classes: ValueRegistry,
}

impl SemanticChecker {
    pub fn new(module: Module) -> Self {
        let imports = BTreeMap::new();
        let table = GlobalSymbolTable::default();

        Self {
            imports,
            module,
            table,
        }
    }

    pub fn build(&mut self) -> Result<()> {
        debug!("Building {}", self.module.identifier);
        self.resolve_imports()?;
        self.resolve_assignments()?;
        self.resolve_type_aliases();
        debug!("Skipping resolving object identifiers");
        //self.values.resolve_object_identifiers();
        self.resolve_defined_values();
        Ok(())
    }

    pub fn resolve_assignments(&mut self) -> Result<()> {
        debug!("Resolving assignments");
        for assignment in mem::replace(&mut self.module.assignments, Vec::new()) {
            ensure!(
                !self.contains_assignment(&assignment.name),
                "{:?} was already defined.",
                assignment.name
            );

            //debug!("ASSIGNMENT KIND: {:#?}", assignment.kind);

            match assignment.kind {
                AssignmentType::Type(ty) => {
                    self.table.insert_type(assignment.name, ty);
                }
                AssignmentType::Value(ty, value) => {
                    self.table.insert_value(assignment.name, ty, value);
                }
                AssignmentType::ValueSet(ty, elements) => {
                    ensure!(
                        elements.set.len() != 0,
                        "{:?} is empty, empty element sets are not allowed.",
                        assignment.name
                    );

                    self.table.insert_value_set(assignment.name, ty, elements);
                }
                AssignmentType::Object(..) => {
                    // unimplemented!()
                    //self.objects.insert(assignment.name, (class, object));
                }
                AssignmentType::ObjectClass(..) => {
                    // unimplemented!()
                    //self.classes.insert(assignment.name, class);
                }
                AssignmentType::ObjectSet(..) => {
                    // unimplemented!()
                    //self.object_sets.insert(assignment.name, class);
                }
            }
        }

        Ok(())
    }

    fn contains_assignment(&self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub fn resolve_type_aliases(&mut self) {
        debug!("Resolving type aliases.");
        for t in self
            .table
            .values
            .iter_mut()
            .map(|(_, (t, _))| t)
            .filter(|t| t.is_referenced())
        {
            let reference = unwrap_to!(t.raw_type => RawType::Referenced);

            if reference.is_internal() {
                if let Some(original_type) = self.table.types.get(&reference.item) {
                    // TODO: How do constraints work across type alias?
                    // Might be defined in the spec
                    *t = original_type.clone();
                }
            } else {
                unimplemented!("External reference types not available yet");
            }
        }
    }

    pub fn resolve_defined_values(&mut self) {
        debug!("Resolving defined values");
        let frozen_map = self.table.values.clone();
        let get_value = |defined_value: &mut DefinedValue| {
            let value = match defined_value {
                DefinedValue::Simple(v) => v,
                DefinedValue::Parameterized(_, _) => {
                    unimplemented!("Parameterized defined values are not currently supported")
                }
            };

            let original_value = if value.is_internal() {
                match frozen_map.get(&*value.item).map(|(_, v)| v) {
                    Some(v) => v,
                    None => panic!("Couldn't find {:?} value", value.item),
                }
            } else {
                unimplemented!("External defines are not currently supported")
            };

            original_value.clone()
        };

        for value in self
            .table
            .values
            .iter_mut()
            .map(|(_, (_, v))| v)
            .filter(|v| v.is_defined())
        {
            let def = match value {
                Value::Defined(def) => def,
                _ => unreachable!(),
            };

            *value = get_value(def);
        }

        let defined_values_imports = self
            .module
            .imports
            .iter_mut()
            .map(|(k, _)| k)
            .filter_map(ModuleReference::as_identification_mut)
            .filter(|a| a.is_defined());

        for value in defined_values_imports {
            let mut def = match value {
                AssignedIdentifier::Defined(def) => def,
                _ => unreachable!(),
            };

            *value =
                AssignedIdentifier::ObjectIdentifier(get_value(&mut def).into_object_identifier());
        }
    }

    pub fn resolve_imports(&mut self) -> Result<()> {
        for (reference, items) in mem::replace(&mut self.module.imports, Vec::new()) {
            ensure!(
                !self.imports.contains_key(&reference),
                "{:?} was already imported.",
                reference
            );

            self.imports.insert(reference, items);
        }

        Ok(())
    }
}
