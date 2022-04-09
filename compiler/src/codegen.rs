mod constant;
mod imports;
mod structs;

use std::{collections::HashSet, fmt, io::Write, mem};

use failure::Fallible as Result;
use heck::*;

use self::{constant::Constant, imports::*, structs::*};
use crate::{
    parser::*,
    semantics::SemanticChecker,
};

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
    fn generate_type(&mut self, ty: &Type) -> Result<String>;
    fn generate_value(&mut self, value: &Value) -> Result<String>;
    fn generate_value_assignment(&mut self, name: String, ty: Type, value: Value) -> Result<()>;
    fn generate_sequence(&mut self, name: &str, fields: &ComponentTypeList) -> Result<String>;
    fn generate_sequence_of(&mut self, name: &str, ty: &Type) -> Result<String>;
    fn generate_builtin(&mut self, builtin: &BuiltinType) -> Result<String>;
    fn write_prelude<W: Write>(&mut self, writer: &mut W) -> Result<()>;
    fn write_footer<W: Write>(&self, writer: &mut W) -> Result<()>;
}

#[derive(Default)]
pub struct Rust {
    environment: TagEnvironment,
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
    fn generate_sequence(&mut self, name: &str, components: &ComponentTypeList) -> Result<String> {
        let mut generated_struct = Struct::new(name);

        for field in components.components.as_ref().unwrap() {
            // Unwrap currently needed as i haven't created the simplified AST without
            // `ComponentsOf` yet.
            let (ty, optional, default) = field.as_type().unwrap();
            let field = FieldBuilder::new(ty.name.as_ref().unwrap().to_snake_case(), self.generate_type(&ty)?)
                .optional(*optional)
                .default_value(default.clone().and_then(|v| self.generate_value(&v).ok()))
                .build();

            generated_struct.add_field(field);
        }

        self.structs.push(generated_struct);

        Ok(name.to_camel_case())
    }

    fn generate_sequence_of(&mut self, name: &str, ty: &Type) -> Result<String> {
        let inner_type = match ty.raw_type {
            RawType::Referenced(ref reference) => &reference.item,
            _ => unimplemented!(),
        };

        Ok(format!("pub type {} = Vec<{}>;", name, inner_type))
    }

    fn generate_type(&mut self, ty: &Type) -> Result<String> {
        match ty.raw_type {
            RawType::Builtin(ref builtin) => self.generate_builtin(builtin),
            RawType::Referenced(ref reference) if reference.is_internal() => {
                Ok(reference.item.clone())
            }
            ref raw => {
                warn!("UNKNOWN TYPE: {:?}", raw);
                Ok(String::from("UNIMPLEMENTED"))
            }
        }
    }

    fn generate_builtin(&mut self, builtin: &BuiltinType) -> Result<String> {
        let output = match builtin {
            BuiltinType::Boolean => String::from("bool"),
            BuiltinType::ObjectIdentifier => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["asn1", "types", "ObjectIdentifier"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                String::from("ObjectIdentifier")
            }

            BuiltinType::OctetString => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["asn1", "types", "OctetString"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                String::from("OctetString")
            }
            BuiltinType::Integer(_) => {
                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["asn1", "types", "Integer"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                String::from("Integer")
            },
            BuiltinType::Prefixed(prefix, ty) => {

                let kind = match prefix.kind {
                    TagKind::Implicit => TagEnvironment::Implicit,
                    TagKind::Explicit => TagEnvironment::Explicit,
                    TagKind::Environment => self.environment
                };

                self.prelude.insert(Import::new(
                    Visibility::Private,
                    ["asn1", "types", &*kind.to_string()]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ));

                let generated_ty = self.generate_type(&ty)?;

                format!(
                    "{kind}<{class}, {number}, {ty}>",
                    kind = kind,
                    class = prefix.class.unwrap_or(Class::Context),
                    number = prefix.number,
                    ty = generated_ty
                )
            }
            ref builtin => {
                warn!("UNKNOWN BUILTIN TYPE: {:?}", builtin);
                String::from("UNIMPLEMENTED")
            }
        };

        Ok(output)
    }

    fn write_prelude<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        let prelude = mem::replace(&mut self.prelude, HashSet::new());
        writer.write(itertools::join(prelude.iter().map(ToString::to_string), "\n").as_bytes())?;

        /*
        let consts = mem::replace(&mut self.consts, HashSet::new());
        writer.write(
            itertools::join(
                consts.into_iter().filter_map(|c| c.generate(self).ok()),
                "\n",
            )
            .as_bytes(),
        )?;
        */
        Ok(())
    }

    fn write_footer<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write(
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
                    self.backend.generate_sequence(&name, components)?;
                }
                RawType::Builtin(BuiltinType::SequenceOf(ty)) => {
                    write!(
                        self.writer,
                        "{}\n",
                        self.backend.generate_sequence_of(&name, &ty)?
                    )?;
                }
                RawType::Builtin(BuiltinType::Prefixed(prefix, ty)) => {
                    write!(
                        self.writer,
                        "{}\n",
                        self.backend.generate_sequence_of(&name, &ty)?
                    )?;
                }
                _ => {}
            }
        }

        self.backend.write_prelude(self.writer)?;
        write!(self.writer, "\n\n")?;
        self.backend.write_footer(self.writer)?;

        Ok(())
    }
}
