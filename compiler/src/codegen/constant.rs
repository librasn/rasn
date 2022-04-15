use failure::Fallible;

use super::{imports::Visibility, Backend, Rust};
use crate::parser::{Type, Value};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Constant {
    visibility: Visibility,
    name: String,
    ty: Type,
    value: Value,
}

impl Constant {
    pub fn new(visibility: Visibility, name: String, ty: Type, value: Value) -> Self {
        Self {
            visibility,
            name,
            ty,
            value,
        }
    }

    pub fn generate(self, backend: &mut Rust) -> Fallible<String> {
        use heck::ShoutySnakeCase;

        let (ty, _) = backend.generate_type(&self.ty, None)?;

        Ok(format!(
            "{vis}const {name}: {ty} = {value};",
            vis = self.visibility,
            name = self.name.to_shouty_snake_case(),
            ty = ty,
            value = backend.generate_value(&self.value)?,
        ))
    }
}
