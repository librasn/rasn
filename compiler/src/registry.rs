use std::{
    collections::BTreeMap,
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use unwrap_to::unwrap_to;

use crate::{parser::*, Result};

#[derive(Debug, Default)]
pub struct GlobalSymbolTable {
    pub types: SymbolTable<Type>,
    pub values: SymbolTable<(Type, Value)>,
    pub value_sets: SymbolTable<(Type, ElementSetSpec)>,
}

impl GlobalSymbolTable {
    pub fn contains_key(&self, key: &str) -> bool {
        self.types.contains_key(key)
            || self.values.contains_key(key)
            || self.value_sets.contains_key(key)
    }

    pub fn insert_type(&mut self, key: String, value: Type) -> Option<Type> {
        self.types.insert(key, value)
    }

    pub fn insert_value(&mut self, key: String, ty: Type, value: Value) -> Option<(Type, Value)> {
        self.values.insert(key, (ty, value))
    }

    pub fn insert_value_set(
        &mut self,
        key: String,
        ty: Type,
        set: ElementSetSpec,
    ) -> Option<(Type, ElementSetSpec)> {
        self.value_sets.insert(key, (ty, set))
    }
}

#[derive(Debug)]
pub struct SymbolTable<V, K: Ord = String> {
    map: BTreeMap<K, V>,
}

impl<K: Ord, V> Default for SymbolTable<V, K> {
    fn default() -> Self {
        Self {
            map: BTreeMap::default(),
        }
    }
}

impl<K: Ord, V> Deref for SymbolTable<V, K> {
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K: Ord, V> DerefMut for SymbolTable<V, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl SymbolTable<(Type, Value)> {
    pub fn resolve_object_identifiers(&mut self) {
        debug!("Resolving OIDs to full path.");
        let total_length = self
            .map
            .iter()
            .filter(|(_, (_, v))| v.is_object_identifier())
            .count();

        trace!("Total number of OIDs: {}", total_length);

        let mut absolute_oids: BTreeMap<String, ObjectIdentifier> = self
            .map
            .clone()
            .into_iter()
            .filter(|(_, (_, v))| {
                v.is_object_identifier() && unwrap_to!(v => Value::ObjectIdentifier).is_absolute()
            })
            .map(|(k, (_, v))| {
                (
                    k,
                    match v {
                        Value::ObjectIdentifier(s) => s,
                        _ => unreachable!(),
                    },
                )
            })
            .collect();

        trace!("Number of initial Absolute OIDs: {}", absolute_oids.len());

        while total_length > absolute_oids.len() {
            for (name, object_identifier) in self
                .get_object_identifiers_mut()
                .filter(|(_, o)| o.is_relative())
            {
                trace!("Attempting to canonicalise {}", object_identifier);
                object_identifier.replace(&absolute_oids);
                if object_identifier.is_absolute() {
                    trace!("{} is now absolute.", object_identifier);
                    absolute_oids.insert(name.clone(), object_identifier.clone());
                    trace!(
                        "New number of Absolute OIDs: {}, remaining {}",
                        absolute_oids.len(),
                        total_length - absolute_oids.len()
                    );
                }
            }
        }
    }

    fn get_object_identifiers_mut(
        &mut self,
    ) -> impl Iterator<Item = (&String, &mut ObjectIdentifier)> {
        self.map
            .iter_mut()
            .filter_map(|(k, (_, v))| v.as_object_identifier_mut().map(|v| (k, v)))
    }
}

impl SymbolTable<PathBuf, ModuleIdentifier> {
    pub fn new(dependencies: Option<PathBuf>) -> Result<Self> {
        let mut map = BTreeMap::new();

        if let Some(ref dependencies) = dependencies {
            for entry in fs::read_dir(&dependencies)? {
                let entry = entry?;

                if entry.file_type()?.is_dir() {
                    continue;
                }

                let path = entry.path();

                if path.extension().map(|x| x != "asn1").unwrap_or(true) {
                    continue;
                }

                let header = Parser::parse_header(&fs::read_to_string(&path)?)?;

                map.insert(header, path.to_owned());
            }
        }

        Ok(Self { map })
    }
}
