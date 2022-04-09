mod prefix;

use crate::parser::*;
pub use prefix::*;

#[derive(Clone, Debug, Derefable, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Type {
    #[deref]
    pub raw_type: RawType,
    pub name: Option<String>,
    pub constraints: Option<Vec<Constraint>>,
}

impl From<RawType> for Type {
    fn from(raw_type: RawType) -> Self {
        Type {
            raw_type,
            name: None,
            constraints: None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum RawType {
    Builtin(BuiltinType),
    ParameterizedReference(ReferenceType, Vec<Parameter>),
    Referenced(ReferenceType),
    ReferencedFromObject(FieldReference),
}

impl From<BuiltinType> for RawType {
    fn from(builtin: BuiltinType) -> Self {
        RawType::Builtin(builtin)
    }
}

impl From<ReferenceType> for RawType {
    fn from(reference: ReferenceType) -> Self {
        RawType::Referenced(reference)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum BuiltinType {
    Boolean,
    BitString(BTreeMap<String, Number>),
    CharacterString(CharacterStringType),
    Choice(ChoiceType),
    Enumeration(
        Vec<Enumeration>,
        Option<ExceptionIdentification>,
        Option<Vec<Enumeration>>,
    ),
    Integer(BTreeMap<String, Number>),
    Null,
    ObjectClassField(DefinedObjectClass, Vec<Field>),
    ObjectIdentifier,
    OctetString,
    Prefixed(Prefix, Box<Type>),
    Sequence(ComponentTypeList),
    SequenceOf(Box<Type>),
    Set(Set),
    SetOf(Box<Type>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ReferenceType {
    pub module: Option<String>,
    pub item: String,
}

impl ReferenceType {
    pub fn new(module: Option<String>, item: String) -> Self {
        Self { module, item }
    }

    pub fn is_internal(&self) -> bool {
        self.module.is_none()
    }
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.module.is_some() {
            write!(f, "{}.", self.module.as_ref().unwrap())?;
        }

        write!(f, "{}", self.item)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum CharacterStringType {
    Bmp,
    General,
    Graphic,
    Ia5,
    Iso646,
    Numeric,
    Printable,
    T61,
    Teletex,
    Universal,
    Unrestricted,
    Utf8,
    Videotex,
    Visible,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ChoiceType {
    pub alternatives: Vec<Type>,
    pub extension: Option<ExtensionAndException>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Enumeration {
    name: String,
    number: Option<Number>,
}

impl Enumeration {
    pub fn new(name: String, number: Option<Number>) -> Self {
        Self { name, number }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum DefinedObjectClass {
    AbstractSyntax,
    Reference(ReferenceType),
    TypeIdentifier,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ComponentTypeList {
    pub components: Option<Vec<ComponentType>>,
    pub extension: Option<Extension>,
}

impl ComponentTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ComponentType {
    Type {
        ty: Type,
        optional: bool,
        default: Option<Value>,
    },
    ComponentsOf(Type),
}

impl ComponentType {
    pub fn as_type(&self) -> Option<(&Type, &bool, &Option<Value>)> {
        match self {
            ComponentType::Type {
                ty,
                optional,
                default,
            } => Some((&ty, &optional, &default)),
            ComponentType::ComponentsOf(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Set {
    Extensible(ExtensionAndException, bool),
    Concrete(ComponentTypeList),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Extension {
    pub exception: Option<ExceptionIdentification>,
    pub additions: Vec<ExtensionAddition>,
    pub marker: ExtensionMarker,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ExtensionAddition {
    Component(ComponentType),
    Group(Option<i64>, Vec<ComponentType>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ExtensionMarker {
    Extensible,
    End(Vec<ComponentType>),
}
