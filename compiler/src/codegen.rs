mod constant;
mod constant_enum;
mod imports;
mod structs;

use std::{collections::HashSet, fmt, io::Write, mem};

use failure::Fallible as Result;
use heck::*;

use self::{
    constant::Constant,
    imports::*,
    structs::{Field as StructField, *},
};
use crate::codegen::constant_enum::{ConstantEnum, Triple};
use crate::{parser::*, semantics::SemanticChecker};

#[derive(Clone, Copy, Debug)]
pub enum TagEnvironment {
    Implicit,
    Explicit,
    Automatic,
}

impl fmt::Display for TagEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TagEnvironment::Implicit => "Implicit",
            TagEnvironment::Explicit => "Explicit",
            TagEnvironment::Automatic => "AUTOMATIC",
        };

        f.write_str(s)
    }
}

impl Default for TagEnvironment {
    fn default() -> Self {
        TagEnvironment::Automatic
    }
}

pub trait Backend: Default {
    fn tag_environment(&mut self, environment: TagEnvironment);
    fn generate_type(
        &mut self,
        ty: &Type,
        parent_prefix: Option<&Prefix>,
    ) -> Result<(String, Option<Prefix>)>;
    fn generate_value(&mut self, value: &Value) -> Result<String>;
    fn generate_value_assignment(&mut self, name: String, ty: Type, value: Value) -> Result<()>;
    fn generate_sequence(
        &mut self,
        name: &str,
        components: &ComponentTypeList,
        parent_prefix: Option<&Prefix>,
    ) -> Result<String>;
    fn generate_sequence_of(
        &mut self,
        name: &str,
        ty: &Type,
        parent_prefix: Option<&Prefix>,
    ) -> Result<String>;
    fn generate_set(
        &mut self,
        name: &str,
        components: &ComponentTypeList,
        parent_prefix: Option<&Prefix>,
    ) -> Result<String>;
    fn generate_field(
        &mut self,
        field: &ComponentType,
        parent_prefix: Option<&Prefix>,
    ) -> Result<StructField>;
    fn generate_builtin(
        &mut self,
        name: &str,
        builtin: &BuiltinType,
        parent_prefix: Option<&Prefix>,
    ) -> Result<(String, Option<Prefix>)>;
    fn write_prelude<W: Write>(&mut self, writer: &mut W) -> Result<()>;
    fn write_footer<W: Write>(&self, writer: &mut W) -> Result<()>;
}

#[derive(Default)]
pub struct Rust {
    environment: TagEnvironment,
    constant_enums: HashSet<ConstantEnum>,
    consts: HashSet<Constant>,
    structs: Vec<Struct>,
    prelude: HashSet<Import>,
}

impl Backend for Rust {
    fn tag_environment(&mut self, environment: TagEnvironment) {
        self.environment = environment;
    }
    /// As Rust doesn't allow you to have anonymous structs,
    /// `generate_sequence` returns the name of the struct and
    /// stores the definition seperately.
    fn generate_sequence(
        &mut self,
        name: &str,
        components: &ComponentTypeList,
        parent_prefix: Option<&Prefix>,
    ) -> Result<String> {
        let mut generated_struct = Struct::new(Visibility::Public, name);

        if let Some(prefix) = parent_prefix {
            generated_struct =
                generated_struct.add_rasn_attributes(vec![Rasn::Prefix(prefix.clone())]);
        }

        for field in components.components.as_ref().unwrap() {
            let field = self.generate_field(field, parent_prefix)?;

            generated_struct.add_field(field);
        }

        self.structs.push(generated_struct);

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "AsnType"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "Encode"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "Decode"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        Ok(name.to_camel_case())
    }

    fn generate_sequence_of(
        &mut self,
        name: &str,
        ty: &Type,
        _parent_prefix: Option<&Prefix>,
    ) -> Result<String> {
        let inner_type = match ty.raw_type {
            RawType::Referenced(ref reference) => &reference.item,
            _ => unimplemented!(),
        };

        Ok(format!("pub type {} = Vec<{}>;", name, inner_type))
    }

    fn generate_set(
        &mut self,
        name: &str,
        components: &ComponentTypeList,
        parent_prefix: Option<&Prefix>,
    ) -> Result<String> {
        let mut generated_struct = Struct::new(Visibility::Public, name);
        let mut attributes = vec![Rasn::Type("set")];

        if let Some(prefix) = parent_prefix {
            attributes.push(Rasn::Prefix(prefix.clone()));
        }

        generated_struct = generated_struct.add_rasn_attributes(attributes);

        for field in components.components.as_ref().unwrap() {
            let field = self.generate_field(field, parent_prefix)?;
            generated_struct.add_field(field);
        }

        self.structs.push(generated_struct);

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "AsnType"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "Encode"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        self.prelude.insert(Import::new(
            Visibility::Private,
            ["rasn", "Decode"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ));

        Ok(name.to_camel_case())
    }

    fn generate_field(
        &mut self,
        field: &ComponentType,
        parent_prefix: Option<&Prefix>,
    ) -> Result<StructField> {
        // Unwrap currently needed as i haven't created the simplified AST without
        // `ComponentsOf` yet.
        let (ty, optional, default) = field.as_type().unwrap();
        let (field_ty, prefix) = self.generate_type(&ty, parent_prefix)?;
        let mut builder = FieldBuilder::new(ty.name.as_ref().unwrap().to_snake_case(), field_ty)
            .optional(*optional)
            .visibility(Visibility::Public)
            .default_value(default.clone().and_then(|v| self.generate_value(&v).ok()));
        if let Some(prefix) = prefix {
            builder = builder.add_rasn_attribute(vec![Rasn::Prefix(prefix)]);
        }
        let field = builder.build();
        Ok(field)
    }

    fn generate_type(
        &mut self,
        ty: &Type,
        parent_prefix: Option<&Prefix>,
    ) -> Result<(String, Option<Prefix>)> {
        match ty.raw_type {
            RawType::Builtin(ref builtin) => self.generate_builtin(
                &ty.name.as_ref().unwrap_or(&String::from("Error")),
                builtin,
                parent_prefix,
            ),
            RawType::Referenced(ref reference) if reference.is_internal() => {
                Ok((reference.item.clone(), None))
            }
            ref raw => {
                warn!("UNKNOWN TYPE: {:?}", raw);
                Ok((String::from("UNIMPLEMENTED"), None))
            }
        }
    }

    fn generate_builtin(
        &mut self,
        name: &str,
        builtin: &BuiltinType,
        parent_prefix: Option<&Prefix>,
    ) -> Result<(String, Option<Prefix>)> {
        let (output, prefix) = match builtin {
            BuiltinType::Boolean => (String::from("bool"), None),
            BuiltinType::ObjectIdentifier => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["rasn", "types", "ObjectIdentifier"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                (String::from("ObjectIdentifier"), None)
            }

            BuiltinType::OctetString => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["rasn", "types", "OctetString"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                (String::from("OctetString"), None)
            }
            BuiltinType::Integer(named_numbers) => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["rasn", "types", "Integer"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                if named_numbers.len() > 0 {
                    let variants: HashSet<Triple> = named_numbers
                        .iter()
                        .map(|(key, value)| {
                            Triple::new(key.to_string(), String::from("Integer"), value.to_string())
                        })
                        .collect();
                    self.constant_enums.insert(ConstantEnum::new(
                        Visibility::Public,
                        name.to_string(),
                        variants,
                    ));
                }

                (String::from("Integer"), None)
            }
            BuiltinType::Sequence(components) => (
                self.generate_sequence(&name.to_camel_case(), components, parent_prefix)?,
                None,
            ),
            BuiltinType::Prefixed(prefix, ty) => {
                let (ty, _) = self.generate_type(&ty, parent_prefix)?;
                (ty, Some(prefix.clone()))
            }
            ref builtin => {
                warn!("UNKNOWN BUILTIN TYPE: {:?}", builtin);
                (String::from("UNIMPLEMENTED"), None)
            }
        };

        Ok((output, prefix))
    }

    fn write_prelude<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        let prelude = mem::replace(&mut self.prelude, HashSet::new());
        writer
            .write_all(itertools::join(prelude.iter().map(ToString::to_string), "\n").as_bytes())?;

        let consts = mem::replace(&mut self.consts, HashSet::new());
        writer.write_all(
            itertools::join(
                consts.into_iter().filter_map(|c| c.generate(self).ok()),
                "\n",
            )
            .as_bytes(),
        )?;

        let constant_enums = mem::replace(&mut self.constant_enums, HashSet::new());
        writer.write_all(
            itertools::join(
                constant_enums
                    .into_iter()
                    .filter_map(|c| Some(c.generate())),
                "\n",
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn write_footer<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(
            itertools::join(self.structs.iter().map(ToString::to_string), "\n").as_bytes(),
        )?;

        Ok(())
    }

    fn generate_value(&mut self, value: &Value) -> Result<String> {
        match value {
            _ => Ok(String::from("UNIMPLEMENTED")),
        }
    }

    fn generate_value_assignment(&mut self, name: String, ty: Type, value: Value) -> Result<()> {
        self.consts
            .insert(Constant::new(Visibility::Public, name, ty, value));
        Ok(())
    }
}

pub struct CodeGenerator<'a, W: Write, B: Backend> {
    backend: B,
    semantic_tree: SemanticChecker,
    writer: &'a mut W,
}

impl<'a, W: Write, B: Backend> CodeGenerator<'a, W, B> {
    pub fn new(semantic_tree: SemanticChecker, writer: &'a mut W) -> Self {
        Self {
            backend: B::default(),
            semantic_tree,
            writer,
        }
    }

    pub fn generate(mut self) -> Result<()> {
        let table = self.semantic_tree.table;

        for (name, (ty, value)) in table.values.clone().into_iter() {
            self.backend.generate_value_assignment(name, ty, value)?;
        }

        for (name, ty) in table.types.iter() {
            match &ty.raw_type {
                RawType::Builtin(BuiltinType::Sequence(components)) => {
                    self.backend.generate_sequence(&name, components, None)?;
                }
                RawType::Builtin(BuiltinType::SequenceOf(ty)) => {
                    write!(
                        self.writer,
                        "{}\n",
                        self.backend.generate_sequence_of(&name, &ty, None)?
                    )?;
                }
                RawType::Builtin(BuiltinType::Set(Set::Concrete(components))) => {
                    self.backend.generate_set(&name, components, None)?;
                }
                RawType::Builtin(BuiltinType::Prefixed(prefix, ty)) => {
                    let mut ty = ty.clone();
                    if ty.name.is_none() {
                        ty.name = Some(name.to_string());
                    }

                    self.backend.generate_type(&*ty, Some(prefix))?;
                }
                _ => {
                    write!(self.writer, "UNIMPLEMENTED {}\n", &name)?;
                }
            }
        }

        self.backend.write_prelude(self.writer)?;
        write!(self.writer, "\n\n")?;
        self.backend.write_footer(self.writer)?;

        Ok(())
    }
}
