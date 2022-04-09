use std::str::FromStr;

use variation::Variation;

use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ObjectClass {
    Def(ClassDefinition),
    Defined(DefinedObjectClass),
    Parameterized(DefinedObjectClass, Option<Vec<Parameter>>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ClassDefinition {
    fields: Vec<FieldSpec>,
    syntax: Option<Vec<Token>>,
}

impl ClassDefinition {
    pub fn new(fields: Vec<FieldSpec>, syntax: Option<Vec<Token>>) -> Self {
        Self { fields, syntax }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum FieldSpec {
    FixedTypeValue(String, Type, bool, Optionality<Value>),
    VariableTypeValue(String, Vec<Field>, Optionality<Value>),
    FixedValueSet(String, Type, Optionality<ElementSetSpec>),
    ObjectField(String, DefinedObjectClass, Optionality<Object>),
    Type(String, Optionality<Type>),
    ObjectSet(String, DefinedObjectClass, Optionality<(ElementSet, bool)>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Optionality<T> {
    Optional,
    Default(T),
    None,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Field {
    name: String,
    kind: FieldType,
}

impl Field {
    pub fn new(name: String, kind: FieldType) -> Self {
        Self { name, kind }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum FieldType {
    Type,
    Value,
    ValueSet,
    Object,
    ObjectSet,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Class {
    Universal,
    Application,
    Context,
    Private,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Class {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "APPLICATION" => Ok(Class::Application),
            "CONTEXT" => Ok(Class::Context),
            "PRIVATE" => Ok(Class::Private),
            "UNIVERSAL" => Ok(Class::Universal),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Object {
    Def(Vec<ObjectDefn>),
    Reference(ObjectReference),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ObjectReference {
    Object(ReferenceType, Option<Vec<Parameter>>),
    Set(ReferenceType, Option<Vec<Parameter>>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FieldReference {
    object: ObjectReference,
    fields: Vec<Field>,
}

impl FieldReference {
    pub fn new(object: ObjectReference, fields: Vec<Field>) -> Self {
        Self { object, fields }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ObjectDefn {
    Setting(Setting),
    Literal(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Setting {
    Type(Type),
    Value(Value),
    ValueSet(ElementSetSpec),
    Object(Object),
    ObjectSet(ElementSet),
}
