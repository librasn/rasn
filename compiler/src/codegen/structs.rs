use crate::codegen::imports::Visibility;
use crate::parser::{Prefix, TagKind};
use std::fmt;

pub struct Struct {
    visibility: Visibility,
    name: String,
    fields: Vec<Field>,
    attributes: Vec<Attribute>,
}

impl Struct {
    pub fn new<I: Into<String>>(visibility: Visibility, name: I) -> Self {
        Self {
            visibility,
            name: name.into(),
            fields: Vec::new(),
            attributes: vec![Attribute::Derive(vec![
                Derive::AsnType,
                Derive::Encode,
                Derive::Decode,
                Derive::PartialEq,
                Derive::PartialOrd,
                Derive::Eq,
                Derive::Ord,
                Derive::Debug,
            ])],
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn add_rasn_attributes(mut self, attributes: Vec<Rasn>) -> Self {
        self.attributes.push(Attribute::Rasn(attributes));
        self
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.attributes.is_empty() {
            itertools::join(self.attributes.iter().map(ToString::to_string), "\n").fmt(f)?;
            writeln!(f)?;
        }

        writeln!(f, "{}struct {} {{", self.visibility, self.name)?;

        if !self.fields.is_empty() {
            itertools::join(self.fields.iter().map(ToString::to_string), "\n").fmt(f)?;
            writeln!(f)?;
        }

        writeln!(f, "}}")
    }
}

pub struct Field {
    visibility: Visibility,
    attributes: Vec<Attribute>,
    name: String,
    optional: bool,
    // TODO: Replace with stricter type.
    ty: String,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.attributes.is_empty() {
            itertools::join(self.attributes.iter().map(ToString::to_string), "\n").fmt(f)?;
            writeln!(f)?;
        }

        let ty = if self.optional {
            format!("Option<{}>", self.ty)
        } else {
            self.ty.clone()
        };

        write!(f, "\t{}{}: {},", self.visibility, self.name, ty)
    }
}

#[derive(Default)]
pub struct FieldBuilder {
    visibility: Visibility,
    name: String,
    ty: String,
    optional: bool,
    default_value: Option<String>,
    attributes: Vec<Attribute>,
}

impl FieldBuilder {
    pub fn new<I: Into<String>>(name: I, ty: I) -> Self {
        let name = name.into();
        let ty = ty.into();

        Self {
            visibility: Visibility::Private,
            name,
            ty,
            ..Self::default()
        }
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn default_value(mut self, default_value: Option<String>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn add_rasn_attribute(mut self, attributes: Vec<Rasn>) -> Self {
        self.attributes.push(Attribute::Rasn(attributes));
        self
    }

    pub fn build(mut self) -> Field {
        if let Some(default_value) = self.default_value {
            self.attributes
                .push(Attribute::Serde(Serde::Default(default_value)));
        }

        Field {
            visibility: self.visibility,
            attributes: self.attributes,
            name: self.name,
            optional: self.optional,
            ty: self.ty,
        }
    }
}

pub enum Attribute {
    Serde(Serde),
    Derive(Vec<Derive>),
    Rasn(Vec<Rasn>),
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let attribute = match self {
            Attribute::Serde(serde) => match serde {
                Serde::Default(default) => format!("#[serde(default = {})]", default),
            },
            Attribute::Derive(defaults) => format!(
                "#[derive({})]",
                itertools::join(defaults.iter().map(ToString::to_string), ", ")
            ),
            Attribute::Rasn(defaults) => format!(
                "#[rasn({})]",
                itertools::join(defaults.iter().map(ToString::to_string), ", ")
            ),
        };

        attribute.fmt(f)
    }
}

pub enum Serde {
    Default(String),
}

pub enum Derive {
    AsnType,
    Encode,
    Decode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Debug,
}

impl fmt::Display for Derive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let derive = match self {
            Derive::AsnType => "AsnType",
            Derive::Encode => "Encode",
            Derive::Decode => "Decode",
            Derive::PartialEq => "PartialEq",
            Derive::PartialOrd => "PartialOrd",
            Derive::Eq => "Eq",
            Derive::Ord => "Ord",
            Derive::Debug => "Debug",
        };

        derive.fmt(f)
    }
}

pub enum Rasn {
    Type(&'static str),
    Prefix(Prefix),
}

impl fmt::Display for Rasn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rasn = match self {
            Rasn::Type(type_name) => type_name.to_string(),
            Rasn::Prefix(prefix) => match prefix.kind {
                TagKind::Explicit => format!(
                    "tag(explicit({class}{number}))",
                    class = prefix
                        .class
                        .map(|x| x.to_string().to_lowercase() + ", ")
                        .unwrap_or(String::from("")),
                    number = prefix.number
                ),
                TagKind::Implicit => format!(
                    "tag(implicit({class}{number}))",
                    class = prefix
                        .class
                        .map(|x| x.to_string().to_lowercase() + ", ")
                        .unwrap_or(String::from("")),
                    number = prefix.number
                ),
                _ => format!(
                    "tag({class}{number})",
                    class = prefix
                        .class
                        .map(|x| x.to_string().to_lowercase() + ", ")
                        .unwrap_or(String::from("")),
                    number = prefix.number
                ),
            },
        };

        rasn.fmt(f)
    }
}
