use std::fmt;

pub struct Struct {
    name: String,
    fields: Vec<Field>,
    attributes: Vec<Attribute>,
}

impl Struct {
    pub fn new<I: Into<String>>(name: I) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
            attributes: vec![Attribute::Derive(vec![
                Derive::Serialize,
                Derive::Deserialize,
            ])],
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.attributes.is_empty() {
            itertools::join(self.attributes.iter().map(ToString::to_string), "\n").fmt(f)?;
            writeln!(f)?;
        }

        writeln!(f, "struct {} {{", self.name)?;

        if !self.fields.is_empty() {
            itertools::join(self.fields.iter().map(ToString::to_string), "\n").fmt(f)?;
            writeln!(f)?;
        }

        writeln!(f, "}}")
    }
}

pub struct Field {
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

        write!(f, "{}: {},", self.name, ty)
    }
}

#[derive(Default)]
pub struct FieldBuilder {
    name: String,
    ty: String,
    optional: bool,
    default_value: Option<String>,
}

impl FieldBuilder {
    pub fn new<I: Into<String>>(name: I, ty: I) -> Self {
        let name = name.into();
        let ty = ty.into();

        Self {
            name,
            ty,
            ..Self::default()
        }
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn default_value(mut self, default_value: Option<String>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn build(self) -> Field {
        let mut attributes = Vec::new();

        if let Some(default_value) = self.default_value {
            attributes.push(Attribute::Serde(Serde::Default(default_value)));
        }

        Field {
            attributes,
            name: self.name,
            optional: self.optional,
            ty: self.ty,
        }
    }
}

pub enum Attribute {
    Serde(Serde),
    Derive(Vec<Derive>),
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
        };

        attribute.fmt(f)
    }
}

pub enum Serde {
    Default(String),
}

pub enum Derive {
    Serialize,
    Deserialize,
}

impl fmt::Display for Derive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let derive = match self {
            Derive::Serialize => "Serialize",
            Derive::Deserialize => "Deserialize",
        };

        derive.fmt(f)
    }
}
