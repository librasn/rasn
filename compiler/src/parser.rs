pub mod module;
pub mod object;
pub mod oid;
pub mod types;
pub mod values;

use std::{collections::BTreeMap, fmt, iter::Peekable};

use derefable::Derefable;
use pest::{
    iterators::{FlatPairs, Pair},
    Parser as _,
};
use variation::Variation;

use crate::Result;

pub use self::module::*;
pub use self::object::*;
pub use self::oid::*;
pub use self::types::*;
pub use self::values::*;
pub use asn1_pest::{Asn1Parser as Pest, Rule};

// First Vec is a Vec of Unions, containing a Vec of intersections.
type ElementSet = Vec<Vec<Element>>;

pub(crate) struct Parser<'a>(Peekable<FlatPairs<'a, Rule>>, &'a str);

impl<'a> Parser<'a> {
    /// Parse asn1 module into an Abstract Syntax Tree (AST) represented by the
    /// `Module` struct.
    pub fn parse(source: &'a str) -> Result<Module> {
        Self::new(Rule::ModuleDefinition, source)?.parse_module()
    }

    /// Copies the lexer output and parses the module's identifying information
    /// into an Abstract Syntax Tree (AST) represented by the
    /// `ModuleIdentifier` struct.
    pub fn parse_header(source: &'a str) -> Result<ModuleIdentifier> {
        let mut ast = Self::new(Rule::ModuleHeaderOnly, source)?;
        ast.take(Rule::ModuleHeaderOnly);

        ast.parse_module_identifier()
    }

    fn new(rule: Rule, source: &'a str) -> Result<Self> {
        let iter = Pest::parse(rule, source)?;

        Ok(Self(iter.flatten().peekable(), source))
    }

    fn next(&mut self) -> Pair<Rule> {
        self.0.next().unwrap()
    }

    fn peek(&mut self, rule: Rule) -> bool {
        self.rule_peek() == rule
    }

    fn rule_peek(&mut self) -> Rule {
        self.0.peek().map(|x| x.as_rule()).unwrap()
    }

    fn next_rule(&mut self) -> Rule {
        self.0.next().map(|x| x.as_rule()).unwrap()
    }

    /// Takes the next pair, and asserts that's its `Rule` matches `rule`.
    ///
    /// # Panics
    /// If `rule` doesn't match the next rule in the iterator.
    fn take(&mut self, rule: Rule) -> Pair<Rule> {
        let pair = self.0.next().unwrap();
        let expected = pair.as_rule();
        if rule != expected {
            eprintln!("Parse Error: {:?} != {:?}", expected, rule);
            eprintln!("===================LINE==================");
            eprintln!("{}", pair.as_str());
            eprintln!("===================REST==================");
            eprintln!("{:#?}", &self.0.clone().collect::<Vec<_>>()[..5]);
            eprintln!("=========================================");
            panic!("Parse Error: {:?} != {:?}", expected, rule);
            //::std::process::exit(-1);
        }

        pair
    }

    /// Look at the next pair and checks if its `Rule` matches `rule`, consumes the pair if it
    /// matches. Useful for checking for optional wrapper rules.
    fn look(&mut self, rule: Rule) -> Option<Pair<Rule>> {
        if self.peek(rule) {
            Some(self.take(rule))
        } else {
            None
        }
    }

    /// Calls `Ast::peek` using the provided `Rule` and calls `parse_fn` and returns `Some(T)`
    /// if `true`, otherwise returns `None`.
    fn optionally_parse<T>(&mut self, rule: Rule, parse_fn: &dyn Fn(&mut Self) -> T) -> Option<T> {
        if self.peek(rule) {
            Some((parse_fn)(self))
        } else {
            None
        }
    }

    fn parse_module(&mut self) -> Result<Module> {
        self.take(Rule::ModuleDefinition);

        let identifier = self.parse_module_identifier()?;
        let tag = self.parse_tag_default();
        //let extension = self.parse_extension_default();

        let (exports, imports, assignments) = if self.look(Rule::ModuleBody).is_some() {
            let exports = self.parse_exports();
            let imports = self.parse_imports();
            let assignments = self.parse_assignments();

            (exports, imports, assignments)
        } else {
            (Exports::All, Vec::new(), Vec::new())
        };

        self.take(Rule::EOI);

        Ok(Module {
            identifier,
            tag,
            extension: None,
            exports,
            imports,
            assignments,
        })
    }

    fn parse_module_identifier(&mut self) -> Result<ModuleIdentifier> {
        self.take(Rule::ModuleIdentifier);

        let mut module_identifier = ModuleIdentifier::new(self.parse_reference_identifier());

        if self.look(Rule::DefinitiveIdentification).is_some() {
            self.take(Rule::DefinitiveOID);

            while self.look(Rule::DefinitiveObjIdComponent).is_some() {
                let pair = self.next();

                let component = match pair.as_rule() {
                    Rule::NameForm => ObjIdComponent::Name(self.parse_identifier()),
                    Rule::DefinitiveNumberForm => {
                        ObjIdComponent::Number(Number::Literal(pair.as_str().parse()?))
                    }
                    Rule::DefinitiveNameAndNumberForm => {
                        let name = self.parse_identifier();
                        let number = self.take(Rule::DefinitiveNumberForm).as_str().parse()?;
                        ObjIdComponent::NameAndNumber(name, Number::Literal(number))
                    }
                    _ => unreachable!(),
                };

                module_identifier.identification.push(component);
            }
        }

        Ok(module_identifier)
    }

    pub fn parse_tag_default(&mut self) -> Tag {
        if let Some(pair) = self.look(Rule::TagDefault) {
            let raw = pair.as_str();

            if raw.contains("AUTOMATIC") {
                Tag::Automatic
            } else if raw.contains("IMPLICIT") {
                Tag::Implicit
            } else {
                Tag::Explicit
            }
        } else {
            Tag::Explicit
        }
    }

    pub fn parse_exports(&mut self) -> Exports {
        if self.look(Rule::Exports).is_some() && self.peek(Rule::SymbolList) {
            Exports::Symbols(self.parse_symbol_list())
        } else {
            Exports::All
        }
    }

    pub fn parse_symbol_list(&mut self) -> Vec<String> {
        self.take(Rule::SymbolList);
        let mut symbols = Vec::new();

        while self.look(Rule::Symbol).is_some() {
            match self.rule_peek() {
                // TODO(Erin): Support parameterization
                Rule::Reference => {
                    symbols.push(self.parse_reference());
                }
                Rule::ParameterizedReference => {
                    self.take(Rule::ParameterizedReference);
                    symbols.push(self.parse_reference());
                }
                r => unreachable!("Unexpected rule: {:?}", r),
            }
        }

        symbols
    }

    pub fn parse_imports(&mut self) -> Vec<(ModuleReference, Vec<String>)> {
        let mut imports = Vec::new();

        if self.look(Rule::Imports).is_some() {
            while self.look(Rule::SymbolsFromModule).is_some() {
                let symbol_list = self.parse_symbol_list();
                self.take(Rule::GlobalModuleReference);
                let module_name = self.parse_reference_identifier();

                let identification = if self.look(Rule::AssignedIdentifier).is_some() {
                    let identification = match self.rule_peek() {
                        Rule::ObjectIdentifierValue => AssignedIdentifier::ObjectIdentifier(
                            self.parse_object_identifier_value(),
                        ),
                        Rule::DefinedValue => {
                            AssignedIdentifier::Defined(self.parse_defined_value())
                        }
                        _ => unreachable!(),
                    };

                    Some(identification)
                } else {
                    None
                };

                imports.push((
                    ModuleReference::new(module_name, identification),
                    symbol_list,
                ));
            }
        }

        imports
    }

    pub fn parse_object_identifier_value(&mut self) -> ObjectIdentifier {
        self.take(Rule::ObjectIdentifierValue);
        let mut components = Vec::new();

        while self.look(Rule::ObjIdComponents).is_some() {
            let component = match self.rule_peek() {
                Rule::Identifier => ObjIdComponent::Name(self.parse_identifier()),
                Rule::NumberForm => {
                    self.take(Rule::NumberForm);
                    ObjIdComponent::Number(self.parse_number_or_defined_value())
                }
                Rule::NameAndNumberForm => {
                    self.take(Rule::NameAndNumberForm);
                    let name = self.parse_identifier();
                    self.take(Rule::NumberForm);
                    let number = self.parse_number_or_defined_value();

                    ObjIdComponent::NameAndNumber(name, number)
                }
                _ => unreachable!(),
            };

            components.push(component)
        }

        ObjectIdentifier::from_components(components)
    }

    fn parse_defined_value(&mut self) -> DefinedValue {
        self.take(Rule::DefinedValue);

        match self.rule_peek() {
            Rule::DefinedTypeReference => DefinedValue::Simple(self.parse_defined_type_reference()),
            Rule::valuereference => DefinedValue::Simple(self.parse_value_reference()),
            Rule::ParameterizedValue => unimplemented!(),
            _ => unreachable!(),
        }
    }

    fn parse_number_or_defined_value(&mut self) -> Number {
        self.take(Rule::NumberOrDefinedValue);

        match self.rule_peek() {
            Rule::number => Number::Literal(self.parse_number()),
            Rule::DefinedValue => Number::DefinedValue(self.parse_defined_value()),
            _ => unreachable!(),
        }
    }

    fn parse_assignments(&mut self) -> Vec<Assignment> {
        let mut assignments = Vec::new();

        while self.look(Rule::Assignment).is_some() {
            let assignment_type = self.next_rule();
            let ident = self.parse_reference();

            let parameter_list = if self.look(Rule::ParameterList).is_some() {
                let mut parameters = Vec::new();

                while self.look(Rule::Parameter).is_some() {
                    let governor = if self.look(Rule::ParamGovernor).is_some() {
                        match self.rule_peek() {
                            Rule::Governor => {
                                self.take(Rule::Governor);
                                let g = match self.rule_peek() {
                                    Rule::Type => ParamGovernor::Type(self.parse_type()),
                                    Rule::DefinedObjectClass => {
                                        ParamGovernor::Class(self.parse_defined_object_class())
                                    }
                                    _ => unreachable!(),
                                };

                                Some(g)
                            }
                            Rule::Reference => {
                                Some(ParamGovernor::Reference(self.parse_reference()))
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        None
                    };

                    parameters.push((governor, self.parse_reference()));
                }

                Some(parameters)
            } else {
                None
            };

            let kind = match assignment_type {
                Rule::TypeAssignment => AssignmentType::Type(self.parse_type()),
                Rule::ValueAssignment => {
                    AssignmentType::Value(self.parse_type(), self.parse_value())
                }
                Rule::ValueSetAssignment => {
                    AssignmentType::ValueSet(self.parse_type(), self.parse_value_set())
                }
                Rule::ObjectClassAssignment => {
                    AssignmentType::ObjectClass(self.parse_object_class())
                }
                Rule::ObjectAssignment => {
                    AssignmentType::Object(self.parse_defined_object_class(), self.parse_object())
                }
                Rule::ObjectSetAssignment => AssignmentType::ObjectSet(
                    self.parse_defined_object_class(),
                    self.parse_object_set(),
                ),
                _ => unreachable!(),
            };

            assignments.push(Assignment::new(ident, kind, parameter_list))
        }

        assignments
    }

    fn parse_type(&mut self) -> Type {
        self.take(Rule::Type);

        match self.rule_peek() {
            Rule::UnconstrainedType => self.parse_unconstrained_type().into(),
            Rule::ConstrainedType => {
                self.take(Rule::ConstrainedType);

                if self.peek(Rule::TypeWithConstraint) {
                    let is_set = self.take(Rule::TypeWithConstraint).as_str().contains("SET");

                    let constraint = if self.peek(Rule::Constraint) {
                        self.parse_constraint()
                    } else {
                        self.parse_size_constraint()
                    };

                    let inner_type = if self.peek(Rule::NamedType) {
                        self.parse_named_type()
                    } else {
                        self.parse_type()
                    };

                    let raw_type = if is_set {
                        RawType::Builtin(BuiltinType::SetOf(Box::new(inner_type)))
                    } else {
                        RawType::Builtin(BuiltinType::SequenceOf(Box::new(inner_type)))
                    };

                    Type {
                        raw_type,
                        name: None,
                        constraints: Some(vec![constraint]),
                    }
                } else {
                    let raw_type = self.parse_unconstrained_type();
                    let mut constraints = Vec::new();

                    while self.peek(Rule::Constraint) {
                        constraints.push(self.parse_constraint());
                    }

                    Type {
                        raw_type,
                        name: None,
                        constraints: Some(constraints),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_constraint(&mut self) -> Constraint {
        self.take(Rule::Constraint);
        self.take(Rule::ConstraintSpec);

        if self.look(Rule::GeneralConstraint).is_some() {
            match self.rule_peek() {
                Rule::TableConstraint => {
                    self.take(Rule::TableConstraint);

                    if self.look(Rule::ComponentRelationConstraint).is_some() {
                        let object_set = self.parse_defined_object_set();
                        let mut components = Vec::new();

                        while self.look(Rule::AtNotation).is_some() {
                            let mut component_ids = Vec::new();

                            if self.peek(Rule::Level) {
                                unimplemented!("Leveled constraints currently not supported");
                            } else {
                                self.take(Rule::ComponentIdList);

                                while self.peek(Rule::Identifier) {
                                    component_ids.push(self.parse_identifier());
                                }
                            }

                            components.push(component_ids);
                        }

                        Constraint::General(GeneralConstraint::Table(object_set, components))
                    } else {
                        let (set, extendable) = self.parse_object_set();
                        Constraint::General(GeneralConstraint::ObjectSet(set, extendable))
                    }
                }
                Rule::ContentsConstraint => unimplemented!(),
                Rule::UserDefinedConstraint => unimplemented!(),
                _ => unreachable!(),
            }
        } else {
            let is_extendable = self.take(Rule::ElementSetSpecs).as_str().contains("...");

            Constraint::ElementSet(self.parse_element_set_spec(), is_extendable)
        }
    }

    fn parse_size_constraint(&mut self) -> Constraint {
        self.take(Rule::SizeConstraint);
        self.parse_constraint()
    }

    fn parse_element_set_specs(&mut self) -> ElementSetSpec {
        let has_ellipsis = self.take(Rule::ElementSetSpecs).as_str().contains("...");
        let set = self.parse_element_set_spec();

        let extensible = if has_ellipsis {
            if self.peek(Rule::ElementSetSpec) {
                let with = self.parse_element_set_spec();

                Extensible::YesWith(with)
            } else {
                Extensible::Yes
            }
        } else {
            Extensible::No
        };

        ElementSetSpec { set, extensible }
    }

    fn parse_element_set_spec(&mut self) -> ElementSet {
        let mut element_set = Vec::new();

        if self.look(Rule::ElementSetSpec).is_none() {
            return element_set;
        }

        self.take(Rule::Unions);

        while self.look(Rule::Intersections).is_some() {
            let mut intersections = Vec::new();
            while self.look(Rule::IntersectionElements).is_some() {
                intersections.push(self.parse_elements());
                self.look(Rule::IntersectionMark);
            }

            element_set.push(intersections);
            self.look(Rule::UnionMark);
        }

        element_set
    }

    fn parse_unconstrained_type(&mut self) -> RawType {
        self.take(Rule::UnconstrainedType);

        if self.look(Rule::BuiltinType).is_some() {
            let pair = self.next();
            match pair.as_rule() {
                Rule::BooleanType => RawType::Builtin(BuiltinType::Boolean),
                Rule::BitStringType => {
                    let mut named_bits = BTreeMap::new();

                    if self.look(Rule::NamedBitList).is_some() {
                        while self.look(Rule::NamedBit).is_some() {
                            named_bits.insert(
                                self.parse_identifier(),
                                self.parse_number_or_defined_value(),
                            );
                        }
                    }

                    RawType::Builtin(BuiltinType::BitString(named_bits))
                }

                Rule::CharacterStringType => {
                    let pair = self.next();
                    let char_type = if pair.as_rule() == Rule::UnrestrictedCharacterStringType {
                        CharacterStringType::Unrestricted
                    } else {
                        match pair.as_str() {
                            "BMPString" => CharacterStringType::Bmp,
                            "GeneralString" => CharacterStringType::General,
                            "GraphicString" => CharacterStringType::Graphic,
                            "IA5String" => CharacterStringType::Ia5,
                            "ISO646String" => CharacterStringType::Iso646,
                            "NumericString" => CharacterStringType::Numeric,
                            "PrintableString" => CharacterStringType::Printable,
                            "TeletexString" => CharacterStringType::Teletex,
                            "T61String" => CharacterStringType::T61,
                            "UniversalString" => CharacterStringType::Universal,
                            "UTF8String" => CharacterStringType::Utf8,
                            "VideotexString" => CharacterStringType::Videotex,
                            "VisibleString" => CharacterStringType::Visible,
                            _ => unreachable!(),
                        }
                    };

                    RawType::Builtin(BuiltinType::CharacterString(char_type))
                }

                Rule::ChoiceType => {
                    self.take(Rule::AlternativeTypeLists);
                    self.take(Rule::AlternativeTypeList);
                    let mut alternatives = Vec::new();

                    while self.peek(Rule::NamedType) {
                        alternatives.push(self.parse_named_type());
                    }

                    let extension = self.parse_extension_and_exception();

                    RawType::Builtin(BuiltinType::Choice(ChoiceType {
                        alternatives,
                        extension,
                    }))
                }

                Rule::EnumeratedType => {
                    self.take(Rule::Enumerations);

                    let enumerations = self.parse_enumeration();

                    let exception_spec = if self.peek(Rule::ExceptionSpec) {
                        Some(self.parse_exception_spec())
                    } else {
                        None
                    };

                    let extended_enumerations = if self.peek(Rule::Enumeration) {
                        Some(self.parse_enumeration())
                    } else {
                        None
                    };

                    RawType::Builtin(BuiltinType::Enumeration(
                        enumerations,
                        exception_spec,
                        extended_enumerations,
                    ))
                }

                Rule::IntegerType => {
                    let mut named_numbers = BTreeMap::new();

                    if self.look(Rule::NamedNumberList).is_some() {
                        while self.look(Rule::NamedNumber).is_some() {
                            let ident = self.parse_identifier();

                            let value = match self.rule_peek() {
                                Rule::SignedNumber => Number::Literal(self.parse_signed_number()),
                                Rule::DefinedValue => {
                                    Number::DefinedValue(self.parse_defined_value())
                                }
                                _ => unreachable!(),
                            };

                            named_numbers.insert(ident, value);
                        }
                    }

                    RawType::Builtin(BuiltinType::Integer(named_numbers))
                }

                Rule::NullType => RawType::Builtin(BuiltinType::Null),

                Rule::ObjectClassFieldType => {
                    let class = self.parse_defined_object_class();

                    let field_name = self.parse_field_name();

                    RawType::Builtin(BuiltinType::ObjectClassField(class, field_name))
                }

                Rule::ObjectIdentifierType => RawType::Builtin(BuiltinType::ObjectIdentifier),
                Rule::OctetStringType => RawType::Builtin(BuiltinType::OctetString),

                Rule::PrefixedType => {
                    if self.look(Rule::TaggedType).is_some() {
                        self.take(Rule::Tag);

                        let encoding = if self.look(Rule::EncodingReference).is_some() {
                            Some(self.parse_encoding_reference())
                        } else {
                            None
                        };

                        let class = self.look(Rule::Class).and_then(|r| r.as_str().parse().ok());

                        let _pair = self.take(Rule::ClassNumber).as_str().to_owned();
                        let number = self.parse_number_or_defined_value();
                        let kind = match self.look(Rule::TagKind).map(|r| r.as_str()) {
                            Some("IMPLICIT") => TagKind::Implicit,
                            Some("EXPLICIT") => TagKind::Explicit,
                            _ => TagKind::Environment,
                        };
                        let r#type = Box::new(self.parse_type());

                        RawType::Builtin(BuiltinType::Prefixed(
                            Prefix::new(encoding, kind, class, number),
                            r#type,
                        ))
                    } else {
                        unimplemented!("Encoding prefixed types are not supported currently.")
                    }
                }

                Rule::SequenceType => {
                    RawType::Builtin(BuiltinType::Sequence(self.parse_component_type_lists()))
                }

                Rule::SequenceOfType => {
                    if self.peek(Rule::Type) {
                        RawType::Builtin(BuiltinType::SequenceOf(Box::new(self.parse_type())))
                    } else {
                        RawType::Builtin(BuiltinType::SequenceOf(Box::new(self.parse_named_type())))
                    }
                }

                Rule::SetType => {
                    if self.peek(Rule::ExtensionAndException) {
                        RawType::Builtin(BuiltinType::Set(Set::Extensible(
                            self.parse_extension_and_exception().unwrap(),
                            self.parse_optional_extension_marker(),
                        )))
                    } else if self.peek(Rule::ComponentTypeLists) {
                        RawType::Builtin(BuiltinType::Set(Set::Concrete(
                            self.parse_component_type_lists(),
                        )))
                    } else {
                        RawType::Builtin(BuiltinType::Set(Set::Concrete(ComponentTypeList::new())))
                    }
                }
                Rule::SetOfType => {
                    if self.peek(Rule::Type) {
                        RawType::Builtin(BuiltinType::SetOf(Box::new(self.parse_type())))
                    } else {
                        RawType::Builtin(BuiltinType::SetOf(Box::new(self.parse_named_type())))
                    }
                }

                r => unreachable!("Unexpected rule: {:?}", r),
            }
        } else {
            self.take(Rule::ReferencedType);

            match self.rule_peek() {
                Rule::DefinedType => {
                    self.take(Rule::DefinedType);

                    match self.rule_peek() {
                        Rule::DefinedTypeReference => self.parse_defined_type_reference().into(),
                        Rule::ParameterizedType => {
                            self.take(Rule::ParameterizedType);

                            let reference = self.parse_defined_type_reference();
                            let parameters = self.parse_actual_parameter_list();

                            RawType::ParameterizedReference(reference, parameters)
                        }
                        Rule::ParameterizedValueSet => unimplemented!(),

                        r => unreachable!("Unexpected rule: {:?}", r),
                    }
                }
                Rule::FromObject => RawType::ReferencedFromObject(self.parse_from_object()),
                _ => unreachable!(),
            }
        }
    }

    fn parse_actual_parameter_list(&mut self) -> Vec<Parameter> {
        self.take(Rule::ActualParameterList);
        let mut parameters = Vec::new();

        while self.look(Rule::ActualParameter).is_some() {
            let parameter = match self.rule_peek() {
                Rule::Type => Parameter::Type(self.parse_type()),
                Rule::Value => Parameter::Value(self.parse_value()),
                Rule::ValueSet => Parameter::ValueSet(self.parse_value_set()),
                Rule::DefinedObjectClass => {
                    Parameter::ObjectClass(self.parse_defined_object_class())
                }
                Rule::Object => Parameter::Object(self.parse_object()),
                Rule::ObjectSet => Parameter::ObjectSet(self.parse_object_set()),
                _ => unreachable!(),
            };

            parameters.push(parameter);
        }

        parameters
    }

    fn parse_value_set(&mut self) -> ElementSetSpec {
        self.take(Rule::ValueSet);

        self.parse_element_set_specs()
    }

    fn parse_value(&mut self) -> Value {
        self.take(Rule::Value);

        match self.rule_peek() {
            Rule::BuiltinValue => self.parse_builtin_value(),
            Rule::ReferencedValue => self.parse_referenced_value(),
            Rule::ObjectClassFieldType => unimplemented!(),
            _ => unreachable!(),
        }
    }

    fn parse_builtin_value(&mut self) -> Value {
        self.take(Rule::BuiltinValue);

        match self.rule_peek() {
            Rule::BitStringValue => {
                self.take(Rule::BitStringValue);

                let bitstring = match self.rule_peek() {
                    Rule::bstring => {
                        self.take(Rule::bstring);

                        let bitstring = self
                            .look(Rule::bits)
                            .map(|b| b.as_str().to_owned())
                            .unwrap_or_else(String::new);

                        BitString::Literal(bitstring)
                    }
                    Rule::hstring => {
                        self.take(Rule::hstring);

                        let bitstring = self
                            .look(Rule::hexes)
                            .and_then(|hex| u64::from_str_radix(hex.as_str(), 16).ok())
                            .map(|num| format!("{:b}", num))
                            .unwrap_or_else(String::new);

                        BitString::Literal(bitstring)
                    }

                    Rule::IdentifierList => {
                        self.take(Rule::IdentifierList);

                        let mut identifiers = Vec::new();
                        while self.peek(Rule::Identifier) {
                            identifiers.push(self.parse_identifier());
                        }

                        BitString::List(identifiers)
                    }

                    Rule::Value => {
                        unimplemented!("BitStrings with 'CONTAINING' aren't currently supported.")
                    }

                    _ => BitString::Literal(String::new()),
                };

                Value::BitString(bitstring)
            }
            Rule::IntegerValue => {
                self.take(Rule::IntegerValue);

                let value = match self.rule_peek() {
                    Rule::SignedNumber => IntegerValue::Literal(self.parse_signed_number()),
                    Rule::Identifier => IntegerValue::Identifier(self.parse_identifier()),
                    _ => unreachable!(),
                };

                Value::Integer(value)
            }

            Rule::ObjectIdentifierValue => {
                Value::ObjectIdentifier(self.parse_object_identifier_value())
            }

            Rule::SequenceValue => self.parse_sequence_value(),
            Rule::EnumeratedValue => self.parse_enumerated_value(),
            Rule::BooleanValue => self.parse_boolean_value(),

            e => unreachable!("Unexpected Rule {:?}", e),
        }
    }

    fn parse_referenced_value(&mut self) -> Value {
        self.take(Rule::ReferencedValue);

        match self.rule_peek() {
            Rule::DefinedValue => Value::Defined(self.parse_defined_value()),
            Rule::FromObject => Value::FromObject(self.parse_from_object()),
            _ => unreachable!(),
        }
    }

    fn parse_from_object(&mut self) -> FieldReference {
        self.take(Rule::FromObject);
        self.take(Rule::ReferencedObjects);

        let referenced_object = match self.rule_peek() {
            Rule::DefinedObject => self.parse_defined_object(),
            Rule::DefinedObjectSet => self.parse_defined_object_set(),
            Rule::ParameterizedObjectSet => self.parse_parameterized_object_set(),
            _ => unreachable!(),
        };

        FieldReference::new(referenced_object, self.parse_field_name())
    }

    fn parse_signed_number(&mut self) -> i64 {
        self.take(Rule::SignedNumber).as_str().parse().unwrap()
    }

    fn parse_number(&mut self) -> i64 {
        self.take(Rule::number).as_str().parse().unwrap()
    }

    fn parse_identifier(&mut self) -> String {
        self.parse_to_str(Rule::Identifier)
    }

    fn parse_value_reference(&mut self) -> ReferenceType {
        ReferenceType::new(None, self.parse_to_str(Rule::valuereference))
    }

    fn parse_reference(&mut self) -> String {
        const VALID_RULES: [Rule; 8] = [
            Rule::Reference,
            // These rules are also allowed, so that this can be called in parse_assignment.
            Rule::EncodingIdentifier,
            Rule::ReferenceIdentifier,
            Rule::typereference,
            Rule::valuereference,
            Rule::objectclassreference,
            Rule::objectreference,
            Rule::objectsetreference,
        ];

        let pair = self.next();

        let is_valid = VALID_RULES.into_iter().any(|rule| pair.as_rule() == *rule);

        if is_valid {
            pair.as_str().to_owned()
        } else {
            panic!("{:?} != {:?}", pair.as_rule(), VALID_RULES);
        }
    }

    fn parse_reference_identifier(&mut self) -> String {
        self.parse_to_str(Rule::ReferenceIdentifier)
    }

    fn parse_type_reference(&mut self) -> String {
        self.parse_to_str(Rule::typereference)
    }

    fn parse_encoding_identifier(&mut self) -> String {
        self.parse_to_str(Rule::EncodingIdentifier)
    }

    fn parse_module_reference(&mut self) -> String {
        self.parse_to_str(Rule::modulereference)
    }

    fn parse_object_reference(&mut self) -> String {
        self.parse_to_str(Rule::objectreference)
    }

    fn parse_object_set_reference(&mut self) -> String {
        self.parse_to_str(Rule::objectsetreference)
    }

    fn parse_to_str(&mut self, rule: Rule) -> String {
        self.take(rule).as_str().to_owned()
    }

    fn parse_value_field_reference(&mut self) -> String {
        self.parse_field_reference(Rule::valuefieldreference)
    }

    fn parse_value_set_field_reference(&mut self) -> String {
        self.parse_field_reference(Rule::valuesetfieldreference)
    }

    fn parse_object_field_reference(&mut self) -> String {
        self.parse_field_reference(Rule::objectfieldreference)
    }

    fn parse_object_set_field_reference(&mut self) -> String {
        self.parse_field_reference(Rule::objectsetfieldreference)
    }

    fn parse_type_field_reference(&mut self) -> String {
        self.parse_field_reference(Rule::typefieldreference)
    }

    fn parse_field_reference(&mut self, rule: Rule) -> String {
        self.take(rule).as_str().trim_matches('&').to_owned()
    }

    fn parse_encoding_reference(&mut self) -> String {
        self.take(Rule::encodingreference).as_str().to_owned()
    }

    fn parse_literal(&mut self) -> String {
        self.take(Rule::Literal).as_str().to_owned()
    }

    fn parse_component_type_lists(&mut self) -> ComponentTypeList {
        self.take(Rule::ComponentTypeLists);

        let mut component_list = ComponentTypeList {
            components: None,
            extension: None,
        };

        if self.peek(Rule::ComponentTypeList) {
            component_list.components = Some(self.parse_component_type_list());
        }

        if self.look(Rule::ComponentTypeExtension).is_some() {
            let exception = match self.parse_extension_and_exception().unwrap() {
                ExtensionAndException::Extension => None,
                ExtensionAndException::Exception(id) => Some(id),
            };
            let additions = self.parse_extension_additions();
            let marker = if self.look(Rule::ExtensionEndMarker).is_some() {
                ExtensionMarker::End(self.parse_component_type_list())
            } else {
                ExtensionMarker::Extensible
            };

            component_list.extension = Some(Extension {
                exception,
                additions,
                marker,
            });
        }

        component_list
    }

    fn parse_component_type_list(&mut self) -> Vec<ComponentType> {
        self.take(Rule::ComponentTypeList);

        let mut component_types = Vec::new();

        while self.peek(Rule::ComponentType) {
            component_types.push(self.parse_component_type());
        }

        component_types
    }

    fn parse_component_type(&mut self) -> ComponentType {
        let raw = self.take(Rule::ComponentType).as_str().to_owned();

        if raw.contains("COMPONENTS") {
            ComponentType::ComponentsOf(self.parse_type())
        } else {
            let ty = self.parse_named_type();
            let optional = raw.contains("OPTIONAL");
            let default = match raw.contains("DEFAULT") {
                true => Some(self.parse_value()),
                false => None,
            };

            ComponentType::Type {
                ty,
                optional,
                default,
            }
        }
    }

    fn parse_named_type(&mut self) -> Type {
        self.take(Rule::NamedType);
        let ident = self.parse_identifier();
        let mut ty = self.parse_type();
        ty.name = Some(ident);

        ty
    }

    fn parse_elements(&mut self) -> Element {
        self.take(Rule::Elements);
        match self.rule_peek() {
            Rule::ElementSetSpec => Element::ElementSet(self.parse_element_set_spec()),
            Rule::SubtypeElements => {
                self.take(Rule::SubtypeElements);
                let subtype = match self.rule_peek() {
                    Rule::Value => SubTypeElement::Value(self.parse_value()),
                    Rule::Type => SubTypeElement::Type(self.parse_type()),
                    Rule::SizeConstraint => SubTypeElement::Size(self.parse_size_constraint()),
                    Rule::ValueRange => {
                        self.take(Rule::ValueRange);
                        let is_low_inclusive =
                            self.take(Rule::LowerEndpoint).as_str().contains('<');
                        let low_value = if self.take(Rule::LowerEndValue).as_str().contains("MIN") {
                            RangeValue::Min(is_low_inclusive)
                        } else {
                            RangeValue::Value(self.parse_value(), is_low_inclusive)
                        };

                        let is_high_inclusive =
                            self.take(Rule::UpperEndpoint).as_str().contains('<');
                        let high_value = if self.take(Rule::UpperEndValue).as_str().contains("MAX")
                        {
                            RangeValue::Max(is_high_inclusive)
                        } else {
                            RangeValue::Value(self.parse_value(), is_high_inclusive)
                        };

                        SubTypeElement::Range(low_value, high_value)
                    }
                    Rule::InnerTypeConstraints => {
                        self.take(Rule::InnerTypeConstraints);

                        if self.peek(Rule::Constraint) {
                            SubTypeElement::Constraint(self.parse_constraint())
                        } else {
                            self.take(Rule::MultipleTypeConstraints);

                            if self.look(Rule::FullSpecification).is_some() {
                                SubTypeElement::FullSpec(self.parse_type_constraints())
                            } else {
                                self.take(Rule::PartialSpecification);
                                SubTypeElement::PartialSpec(self.parse_type_constraints())
                            }
                        }
                    }
                    e => unreachable!("{:?}", e),
                };

                Element::SubType(subtype)
            }
            Rule::ObjectSetElements => {
                self.take(Rule::ObjectSetElements);

                match self.rule_peek() {
                    Rule::Object => Element::Object(self.parse_object()),
                    Rule::DefinedObjectSet => Element::ObjectSet(self.parse_defined_object_set()),
                    Rule::ParameterizedObjectSet => {
                        Element::ObjectSet(self.parse_parameterized_object_set())
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_type_constraints(&mut self) -> BTreeMap<String, ComponentConstraint> {
        self.take(Rule::TypeConstraints);
        let mut map = BTreeMap::new();

        while self.look(Rule::NamedConstraint).is_some() {
            let name = self.parse_identifier();
            self.take(Rule::ComponentConstraint);

            let constraint = if self.peek(Rule::Constraint) {
                Some(self.parse_constraint())
            } else {
                None
            };

            let presence = if self.peek(Rule::PresenceConstraint) {
                let p = match self.next().as_str() {
                    "PRESENT" => Presence::Present,
                    "ABSENT" => Presence::Absent,
                    "OPTIONAL" => Presence::Optional,
                    _ => unreachable!(),
                };

                Some(p)
            } else {
                None
            };

            map.insert(name, ComponentConstraint::new(constraint, presence));
        }

        map
    }

    fn parse_defined_object_class(&mut self) -> DefinedObjectClass {
        self.take(Rule::DefinedObjectClass);

        match self.rule_peek() {
            Rule::ExternalObjectClassReference => {
                self.take(Rule::ExternalObjectClassReference);

                DefinedObjectClass::Reference(ReferenceType::new(
                    Some(self.parse_reference_identifier()),
                    self.parse_encoding_identifier(),
                ))
            }
            Rule::EncodingIdentifier => DefinedObjectClass::Reference(ReferenceType::new(
                None,
                self.parse_encoding_identifier(),
            )),
            Rule::UsefulObjectClassReference => {
                if self
                    .take(Rule::UsefulObjectClassReference)
                    .as_str()
                    .contains("ABSTRACT-SYNTAX")
                {
                    DefinedObjectClass::AbstractSyntax
                } else {
                    DefinedObjectClass::TypeIdentifier
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_field_name(&mut self) -> Vec<Field> {
        self.take(Rule::FieldName);
        let mut field_names = Vec::new();

        while self.peek(Rule::PrimitiveFieldName) {
            field_names.push(self.parse_primitive_field_name());
        }

        field_names
    }

    fn parse_primitive_field_name(&mut self) -> Field {
        self.take(Rule::PrimitiveFieldName);

        let rule = self.rule_peek();
        let kind = match rule {
            Rule::typefieldreference => FieldType::Type,
            Rule::valuefieldreference => FieldType::Value,
            Rule::valuesetfieldreference => FieldType::ValueSet,
            Rule::objectfieldreference => FieldType::Object,
            Rule::objectsetfieldreference => FieldType::ObjectSet,
            _ => unreachable!(),
        };

        Field::new(self.parse_field_reference(rule), kind)
    }

    fn parse_object_set(&mut self) -> (ElementSet, bool) {
        self.take(Rule::ObjectSet);

        let is_extendable = self.take(Rule::ObjectSetSpec).as_str().contains("...");

        (self.parse_element_set_spec(), is_extendable)
    }

    fn parse_object(&mut self) -> Object {
        self.take(Rule::Object);

        match self.rule_peek() {
            Rule::DefinedObject => Object::Reference(self.parse_defined_object()),
            Rule::ObjectDefn => {
                self.take(Rule::ObjectDefn);

                let mut tokens = Vec::new();

                if self.look(Rule::DefinedSyntax).is_some() {
                    while self.look(Rule::DefinedSyntaxToken).is_some() {
                        let token = if self.look(Rule::Setting).is_some() {
                            let setting = match self.rule_peek() {
                                Rule::Type => Setting::Type(self.parse_type()),
                                Rule::Value => Setting::Value(self.parse_value()),
                                Rule::ValueSet => Setting::ValueSet(self.parse_value_set()),
                                Rule::Object => Setting::Object(self.parse_object()),
                                Rule::ObjectSet => Setting::ObjectSet(self.parse_object_set().0),
                                _ => unreachable!(),
                            };

                            ObjectDefn::Setting(setting)
                        } else {
                            ObjectDefn::Literal(self.parse_literal())
                        };

                        tokens.push(token);
                    }
                } else {
                    unimplemented!("Default Syntax is not currently suppported")
                }

                Object::Def(tokens)
            }
            _ => unreachable!(),
        }
    }

    fn parse_defined_object(&mut self) -> ObjectReference {
        self.take(Rule::DefinedObject);

        let module = self.optionally_parse(Rule::modulereference, &Self::parse_module_reference);
        let object = self.parse_object_reference();
        let parameters = self.optionally_parse(
            Rule::ActualParameterList,
            &Self::parse_actual_parameter_list,
        );

        ObjectReference::Object(ReferenceType::new(module, object), parameters)
    }

    fn parse_defined_object_set(&mut self) -> ObjectReference {
        self.take(Rule::DefinedObjectSet);

        let module = self.optionally_parse(Rule::modulereference, &Self::parse_module_reference);
        let set = self.parse_object_set_reference();

        ObjectReference::Set(ReferenceType::new(module, set), None)
    }

    fn parse_parameterized_object_set(&mut self) -> ObjectReference {
        self.take(Rule::ParameterizedObjectSet);
        self.take(Rule::DefinedObjectSet);

        let module = self.optionally_parse(Rule::modulereference, &Self::parse_module_reference);
        let set = self.parse_object_set_reference();

        let parameters = self.parse_actual_parameter_list();

        ObjectReference::Set(ReferenceType::new(module, set), Some(parameters))
    }

    fn parse_sequence_value(&mut self) -> Value {
        self.take(Rule::SequenceValue);

        Value::Sequence(self.parse_component_value_list())
    }

    fn parse_component_value_list(&mut self) -> Vec<NamedValue> {
        self.take(Rule::ComponentValueList);

        let mut values = Vec::new();

        while self.look(Rule::NamedValue).is_some() {
            values.push(NamedValue(self.parse_identifier(), self.parse_value()));
        }

        values
    }

    fn parse_enumerated_value(&mut self) -> Value {
        self.take(Rule::EnumeratedValue);

        Value::Enumerated(self.parse_identifier())
    }

    fn parse_object_class(&mut self) -> ObjectClass {
        self.take(Rule::ObjectClass);

        match self.rule_peek() {
            Rule::ObjectClassDefn => ObjectClass::Def(self.parse_object_class_defn()),
            Rule::ParameterizedObjectClass => unimplemented!("ParameterizedObjectClass"),
            Rule::DefinedObjectClass => ObjectClass::Defined(self.parse_defined_object_class()),
            _ => unreachable!(),
        }
    }

    fn parse_object_class_defn(&mut self) -> ClassDefinition {
        self.take(Rule::ObjectClassDefn);
        let mut fields = Vec::new();

        while self.look(Rule::FieldSpec).is_some() {
            let field = match self.rule_peek() {
                Rule::FixedTypeValueFieldSpec => self.parse_fixed_type_value_field_spec(),
                Rule::VariableTypeValueFieldSpec => self.parse_variable_type_value_field_spec(),
                Rule::FixedTypeValueSetFieldSpec => self.parse_fixed_type_value_set_field_spec(),
                Rule::VariableTypeValueSetFieldSpec => {
                    self.parse_variable_type_value_set_field_spec()
                }
                Rule::ObjectFieldSpec => self.parse_object_field_spec(),
                Rule::TypeFieldSpec => self.parse_type_field_spec(),
                Rule::ObjectSetFieldSpec => self.parse_object_set_field_spec(),
                _ => unreachable!(),
            };

            fields.push(field);
        }

        let syntax = if self.look(Rule::WithSyntaxSpec).is_some() {
            self.take(Rule::SyntaxList);
            Some(self.parse_token_or_group_spec())
        } else {
            None
        };

        ClassDefinition::new(fields, syntax)
    }

    fn parse_token_or_group_spec(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.look(Rule::TokenOrGroupSpec).is_some() {
            match self.next_rule() {
                Rule::RequiredToken => {
                    let token = match self.rule_peek() {
                        Rule::Literal => Token::Literal(self.parse_literal()),
                        Rule::PrimitiveFieldName => Token::Field(self.parse_primitive_field_name()),
                        _ => unreachable!(),
                    };

                    tokens.push(token);
                }
                Rule::OptionalGroup => {
                    tokens.push(Token::OptionalGroup(self.parse_token_or_group_spec()));
                }
                _ => unreachable!(),
            }
        }

        tokens
    }

    fn parse_fixed_type_value_field_spec(&mut self) -> FieldSpec {
        let is_unique = self
            .take(Rule::FixedTypeValueFieldSpec)
            .as_str()
            .contains("UNIQUE");
        let ident = self.parse_value_field_reference();
        let ty = self.parse_type();
        let optionality = self.parse_value_optionality_spec();

        FieldSpec::FixedTypeValue(ident, ty, is_unique, optionality)
    }

    fn parse_variable_type_value_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::VariableTypeValueFieldSpec);

        let ident = self.parse_value_field_reference();
        let field_name = self.parse_field_name();
        let optionality = self.parse_value_optionality_spec();

        FieldSpec::VariableTypeValue(ident, field_name, optionality)
    }

    fn parse_fixed_type_value_set_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::FixedTypeValueSetFieldSpec);

        let ident = self.parse_value_set_field_reference();
        let ty = self.parse_type();
        let optionality = self.parse_value_set_optionality_spec();

        FieldSpec::FixedValueSet(ident, ty, optionality)
    }

    fn parse_variable_type_value_set_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::VariableTypeValueSetFieldSpec);
        let ident = self.parse_value_set_field_reference();
        let field = self.parse_field_name();
        let optionality = self.parse_value_optionality_spec();

        FieldSpec::VariableTypeValue(ident, field, optionality)
    }

    fn parse_object_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::ObjectFieldSpec);
        let ident = self.parse_object_field_reference();
        let class = self.parse_defined_object_class();
        let optionality = self.parse_object_optionality_spec();

        FieldSpec::ObjectField(ident, class, optionality)
    }

    fn parse_type_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::TypeFieldSpec);
        let ident = self.parse_type_field_reference();
        let optionality = self.parse_type_optionality_spec();

        FieldSpec::Type(ident, optionality)
    }

    fn parse_object_set_field_spec(&mut self) -> FieldSpec {
        self.take(Rule::ObjectSetFieldSpec);

        let ident = self.parse_object_set_field_reference();
        let class = self.parse_defined_object_class();
        let optionality = self.parse_object_set_optionality_spec();

        FieldSpec::ObjectSet(ident, class, optionality)
    }

    fn parse_value_optionality_spec(&mut self) -> Optionality<Value> {
        self.parse_optionality_spec(Rule::ValueOptionalitySpec, &Self::parse_value)
    }

    fn parse_value_set_optionality_spec(&mut self) -> Optionality<ElementSetSpec> {
        self.parse_optionality_spec(Rule::ValueSetOptionalitySpec, &Self::parse_value_set)
    }

    fn parse_object_optionality_spec(&mut self) -> Optionality<Object> {
        self.parse_optionality_spec(Rule::ObjectOptionalitySpec, &Self::parse_object)
    }

    fn parse_type_optionality_spec(&mut self) -> Optionality<Type> {
        self.parse_optionality_spec(Rule::TypeOptionalitySpec, &Self::parse_type)
    }

    fn parse_object_set_optionality_spec(&mut self) -> Optionality<(ElementSet, bool)> {
        self.parse_optionality_spec(Rule::ObjectSetOptionalitySpec, &Self::parse_object_set)
    }

    fn parse_optionality_spec<T>(
        &mut self,
        rule: Rule,
        parse_fn: &Fn(&mut Self) -> T,
    ) -> Optionality<T> {
        if !self.peek(rule) {
            Optionality::None
        } else {
            let pair = self.take(rule);

            if pair.as_str().contains("OPTIONAL") {
                Optionality::Optional
            } else {
                Optionality::Default((parse_fn)(self))
            }
        }
    }

    fn parse_defined_type_reference(&mut self) -> ReferenceType {
        self.take(Rule::DefinedTypeReference);

        let module = self.optionally_parse(Rule::modulereference, &Self::parse_module_reference);
        let item = self.parse_type_reference();

        ReferenceType::new(module, item)
    }

    fn parse_external_value_reference(&mut self) -> ReferenceType {
        self.take(Rule::ExternalValueReference);
        let module = self.parse_module_reference();
        let mut reference = self.parse_value_reference();

        reference.module = Some(module);

        reference
    }

    fn parse_extension_and_exception(&mut self) -> Option<ExtensionAndException> {
        if !self.peek(Rule::ExtensionAndException) {
            return None;
        }

        self.take(Rule::ExtensionAndException);

        if self.peek(Rule::ExceptionSpec) {
            Some(ExtensionAndException::Exception(
                self.parse_exception_spec(),
            ))
        } else {
            Some(ExtensionAndException::Extension)
        }
    }

    fn parse_extension_additions(&mut self) -> Vec<ExtensionAddition> {
        self.take(Rule::ExtensionAdditions);
        self.take(Rule::ExtensionAdditionList);
        let mut additions = Vec::new();

        while self.look(Rule::ExtensionAddition).is_some() {
            if self.peek(Rule::ComponentType) {
                additions.push(ExtensionAddition::Component(self.parse_component_type()))
            } else {
                let version = if self.peek(Rule::VersionNumber) {
                    Some(self.parse_number())
                } else {
                    None
                };

                let components = self.parse_component_type_list();

                additions.push(ExtensionAddition::Group(version, components));
            }
        }

        additions
    }

    /// Returns just whether or not the extension marker was present, while
    /// consuming its pair if it was present.
    fn parse_optional_extension_marker(&mut self) -> bool {
        self.look(Rule::OptionalExtensionMarker).is_some()
    }

    fn parse_boolean_value(&mut self) -> Value {
        Value::Boolean(self.take(Rule::BooleanValue).as_str().contains("TRUE"))
    }

    fn parse_enumeration(&mut self) -> Vec<Enumeration> {
        self.take(Rule::Enumeration);

        let mut enumerations = Vec::new();

        while self.look(Rule::EnumerationItem).is_some() {
            let (name, number) = if self.peek(Rule::NamedNumber) {
                let (name, number) = self.parse_named_number();
                (name, Some(number))
            } else {
                (self.parse_identifier(), None)
            };

            enumerations.push(Enumeration::new(name, number));
        }

        enumerations
    }

    fn parse_named_number(&mut self) -> (String, Number) {
        self.take(Rule::NamedNumber);

        let name = self.parse_identifier();

        let number = if self.peek(Rule::SignedNumber) {
            Number::Literal(self.parse_signed_number())
        } else {
            Number::DefinedValue(self.parse_defined_value())
        };

        (name, number)
    }

    fn parse_exception_spec(&mut self) -> ExceptionIdentification {
        self.take(Rule::ExceptionSpec);
        self.take(Rule::ExceptionIdentification);

        match self.rule_peek() {
            Rule::SignedNumber => ExceptionIdentification::Number(self.parse_signed_number()),
            Rule::DefinedValue => ExceptionIdentification::Reference(self.parse_defined_value()),
            Rule::Type => {
                ExceptionIdentification::Arbitrary(Box::new(self.parse_type()), self.parse_value())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub kind: AssignmentType,
    pub parameters: Option<Vec<(Option<ParamGovernor>, String)>>,
}

impl Assignment {
    fn new(
        name: String,
        kind: AssignmentType,
        parameters: Option<Vec<(Option<ParamGovernor>, String)>>,
    ) -> Self {
        Self {
            name,
            kind,
            parameters,
        }
    }
}

// First argument is always the identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum AssignmentType {
    Type(Type),
    Value(Type, Value),
    ValueSet(Type, ElementSetSpec),
    Object(DefinedObjectClass, Object),
    ObjectClass(ObjectClass),
    ObjectSet(DefinedObjectClass, (ElementSet, bool)),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Constraint {
    General(GeneralConstraint),
    ElementSet(ElementSet, bool),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum GeneralConstraint {
    Table(ObjectReference, Vec<Vec<String>>),
    ObjectSet(ElementSet, bool),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Element {
    SubType(SubTypeElement),
    ElementSet(ElementSet),
    Object(Object),
    ObjectSet(ObjectReference),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum SubTypeElement {
    Value(Value),
    Type(Type),
    Size(Constraint),
    Range(RangeValue, RangeValue),
    Constraint(Constraint),
    FullSpec(BTreeMap<String, ComponentConstraint>),
    PartialSpec(BTreeMap<String, ComponentConstraint>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Extensible {
    Yes,
    YesWith(ElementSet),
    No,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ElementSetSpec {
    pub set: ElementSet,
    pub extensible: Extensible,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ParamGovernor {
    Type(Type),
    Class(DefinedObjectClass),
    Reference(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ExtensionAndException {
    Extension,
    Exception(ExceptionIdentification),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum ExceptionIdentification {
    Number(i64),
    Reference(DefinedValue),
    Arbitrary(Box<Type>, Value),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum RangeValue {
    Min(bool),
    Max(bool),
    Value(Value, bool),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Token {
    Literal(String),
    Field(Field),
    OptionalGroup(Vec<Token>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Derefable)]
pub struct ParameterList(#[deref(mutable)] Vec<Parameter>);

impl fmt::Display for ParameterList {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("fmt::Display for ParameterList")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Parameter {
    Type(Type),
    Value(Value),
    ValueSet(ElementSetSpec),
    ObjectClass(DefinedObjectClass),
    Object(Object),
    ObjectSet((ElementSet, bool)),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Variation)]
pub enum Presence {
    Absent,
    Optional,
    Present,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ComponentConstraint {
    constraint: Option<Constraint>,
    presence: Option<Presence>,
}

impl ComponentConstraint {
    fn new(constraint: Option<Constraint>, presence: Option<Presence>) -> Self {
        Self {
            constraint,
            presence,
        }
    }
}
