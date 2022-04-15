use failure::Fallible;
use heck::CamelCase;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use super::imports::Visibility;

#[derive(Clone, Debug, Eq)]
pub struct Triple {
    pub key: String,
    pub value_type: String,
    pub value: String,
}

impl Triple {
    pub fn new(key: String, value_type: String, value: String) -> Self {
        Self {
            key,
            value_type,
            value,
        }
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Hash for Triple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.key.as_bytes());
    }
}

#[derive(Clone, Debug, Eq)]
pub struct ConstantEnum {
    visibility: Visibility,
    name: String,
    variants: HashSet<Triple>,
}

impl PartialEq for ConstantEnum {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for ConstantEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

impl ConstantEnum {
    pub fn new(visibility: Visibility, name: String, variants: HashSet<Triple>) -> Self {
        Self {
            visibility,
            name,
            variants,
        }
    }

    pub fn generate(self) -> String {
        format!(
            r#"
#[non_exhaustive]
{vis}struct {name};
impl {name} {{
{variants}
}}"#,
            vis = self.visibility,
            name = self.name.to_camel_case(),
            variants = self
                .variants
                .iter()
                .map(|triple| format!(
                    "\t{}{}: {} = {};",
                    self.visibility,
                    triple.key.to_camel_case(),
                    triple.value_type.to_camel_case(),
                    triple.value
                ))
                .join("\n"),
        )
    }
}
