use std::fmt;

use variation::Variation;

use super::object::FieldReference;
use super::oid::ObjectIdentifier;
use super::types::ReferenceType;
use super::ParameterList;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Value {
    BitString(BitString),
    Boolean(bool),
    Defined(DefinedValue),
    Enumerated(String),
    FromObject(FieldReference),
    Integer(IntegerValue),
    Object(Vec<String>),
    ObjectClassField,
    ObjectIdentifier(ObjectIdentifier),
    Sequence(Vec<NamedValue>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum IntegerValue {
    Literal(i64),
    Identifier(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Number {
    Literal(i64),
    DefinedValue(DefinedValue),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Literal(num) => num.fmt(f),
            Number::DefinedValue(defined_value) => defined_value.fmt(f),
        }
    }
}

impl From<i64> for Number {
    fn from(num: i64) -> Self {
        Number::Literal(num)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NamedValue(pub String, pub Value);

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum DefinedValue {
    Simple(ReferenceType),
    /// Paramaterized value
    Parameterized(ReferenceType, ParameterList),
}

impl fmt::Display for DefinedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefinedValue::Simple(reference) => reference.fmt(f),
            DefinedValue::Parameterized(reference, parameters) => {
                write!(f, "{}{}", reference, parameters)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum BitString {
    Literal(String),
    List(Vec<String>),
    Containing(Box<Value>),
}
