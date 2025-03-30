use crate::{ext::TypeExt, tag::Tag};
use proc_macro2::Span;
use quote::ToTokens;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{parenthesized, Ident, LitStr, Path, Token, Type, UnOp};

#[derive(Clone, Debug, Default)]
pub struct Constraints {
    pub extensible: bool,
    pub from: Option<Constraint<StringValue>>,
    pub size: Option<Constraint<Value>>,
    pub value: Option<Constraint<Value>>,
}

impl Constraints {
    pub fn const_static_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.const_expr(crate_root)
            .map(|expr| quote!(const CONSTRAINTS: #crate_root::types::Constraints = #expr;))
    }

    pub fn attribute_tokens(&self) -> Option<proc_macro2::TokenStream> {
        self.has_constraints().then(|| {
            let add_comma = |stream| quote!(#stream,);
            let size = self.size_attr().map(add_comma);
            let from = self.from_attr().map(add_comma);
            let extensible = self.extensible.then_some(quote!(#[non_exhaustive]));
            let value = self.value_attr().map(add_comma);

            quote! {
                #[rasn(#size #from #value)]
                #extensible
            }
        })
    }

    pub fn const_expr(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.has_constraints().then(|| {
            let add_comma = |stream| quote!(#stream,);
            let size = self.size_def(crate_root).map(add_comma);
            let from = self.from_def(crate_root).map(add_comma);
            let extensible = self.extensible_def(crate_root).map(add_comma);
            let value = self.value_def(crate_root).map(add_comma);

            quote! {
                #crate_root::types::Constraints::new(&[
                    #size
                    #value
                    #extensible
                    #from
                ])
            }
        })
    }

    fn size_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.size.as_ref().map(|value| {
            let extensible = value.extensible.is_some();
            let constraint = match value.constraint {
                Value::Range(Some(min), Some(max)) => {
                    if min > max {
                        return syn::Error::new(
                            Span::call_site(),
                            "Minimum size constraint must be less than or equal to maximum size constraint.",
                        )
                        .to_compile_error();
                    }
                    quote!(#crate_root::types::constraints::Bounded::const_new(#min as usize, #max as usize))
                }
                Value::Range(Some(min), None) => {
                    quote!(#crate_root::types::constraints::Bounded::start_from(#min as usize))
                }
                Value::Range(None, Some(max)) => {
                    quote!(#crate_root::types::constraints::Bounded::up_to(#max as usize))
                }
                Value::Range(None, None) => {
                    quote!(#crate_root::types::constraints::Bounded::const_new(usize::MIN, usize::MAX))
                }
                Value::Single(length) => {
                    quote!(#crate_root::types::constraints::Bounded::single_value(#length as usize))
                }
            };

            quote!(
                #crate_root::types::Constraint::Size(
                    #crate_root::types::constraints::Extensible::new(
                        #crate_root::types::constraints::Size::new(
                            #constraint
                        )
                    ).set_extensible(#extensible)
                )
            )
        })
    }

    fn size_attr(&self) -> Option<proc_macro2::TokenStream> {
        self.size.as_ref().map(|value| {

            let extensible = value.extensible.is_some().then_some(quote!(extensible));
            let constraint = match value.constraint {
                Value::Range(Some(min), Some(max)) => {
                    if min > max {
                        return syn::Error::new(
                            Span::call_site(),
                            "Minimum size constraint must be less than or equal to maximum size constraint.",
                        )
                        .to_compile_error();
                    }
                    let string = quote!(#min..=#max).to_string();
                    quote!(#string)
                }
                Value::Range(Some(min), None) => {
                    let string = quote!(#min..).to_string();
                    quote!(#string)
                }
                Value::Range(None, Some(max)) => {
                    let string = quote!(..=#max).to_string();
                    quote!(#string)
                }
                Value::Range(None, None) => {
                    quote!("..")
                }
                Value::Single(length) => {
                    quote!(#length)
                }
            };

            quote!(size(#constraint, #extensible))
        })
    }

    fn value_attr(&self) -> Option<proc_macro2::TokenStream> {
        self.value.as_ref().map(|value| {
            let extensible = value.extensible.is_some().then_some(quote!(extensible));
            let constraint = match value.constraint {
                Value::Range(Some(min), Some(max)) => {
                    quote!(#min..#max)
                }
                Value::Range(Some(min), None) => {
                    quote!(#min..)
                }
                Value::Range(None, Some(max)) => {
                    quote!(..#max)
                }
                Value::Range(None, None) => {
                    quote!(..)
                }
                Value::Single(value) => {
                    quote!(#value)
                }
            };

            quote!(value(#constraint, #extensible))
        })
    }

    fn value_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.value.as_ref().map(|value| {
            let extensible = value.extensible.is_some();
            let constraint = match value.constraint {
                Value::Range(Some(min), Some(max)) => {
                    quote!(#crate_root::types::constraints::Bounded::const_new(#min as i128, #max as i128))
                }
                Value::Range(Some(min), None) => {
                    quote!(#crate_root::types::constraints::Bounded::start_from(#min as i128))
                }
                Value::Range(None, Some(max)) => {
                    quote!(#crate_root::types::constraints::Bounded::up_to(#max as i128))
                }
                Value::Range(None, None) => {
                    quote!(#crate_root::types::constraints::Bounded::const_new(i128::MIN, i128::MAX))
                }
                Value::Single(length) => {
                    quote!(#crate_root::types::constraints::Bounded::single_value(#length as i128))
                }
            };

            quote!(
                #crate_root::types::Constraint::Value(
                    #crate_root::types::constraints::Extensible::new(
                        #crate_root::types::constraints::Value::new(
                            #constraint
                        )
                    ).set_extensible(#extensible)
                )
            )
        })
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.from.as_ref().map(|value| {
            let extensible = value.extensible.is_some();
            let StringValue(set) = &value.constraint;

            quote!(
                #crate_root::types::Constraint::PermittedAlphabet(
                    #crate_root::types::constraints::Extensible::new(
                        #crate_root::types::constraints::PermittedAlphabet::new(
                            &[#(#set,)*]
                        )
                    ).set_extensible(#extensible)
                )
            )
        })
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_attr(&self) -> Option<proc_macro2::TokenStream> {
        self.from.as_ref().map(|value| {
            let extensible = value.extensible.is_some().then_some(quote!(extensible));
            let StringValue(set) = &value.constraint;

            quote!(from(#(#set,)*, #extensible))
        })
    }

    fn extensible_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.extensible
            .then(|| quote!(#crate_root::types::Constraint::Extensible))
    }

    fn has_constraints(&self) -> bool {
        self.extensible || self.from.is_some() || self.size.is_some() || self.value.is_some()
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub crate_root: Path,
    pub identifier: Option<syn::LitStr>,
    pub enumerated: bool,
    pub choice: bool,
    pub set: bool,
    pub automatic_tags: bool,
    pub delegate: bool,
    pub tag: Option<Tag>,
    pub constraints: Constraints,
}

impl Config {
    pub fn from_attributes(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut choice = false;
        let mut set = false;
        let mut crate_root = None;
        let mut identifier = None;
        let mut enumerated = false;
        let mut automatic_tags = false;
        let mut tag = None;
        let mut from = None;
        let mut size = None;
        let mut value = None;
        let mut delegate = false;
        let mut extensible = false;

        for attr in &input.attrs {
            if attr.path().is_ident("non_exhaustive") {
                extensible = true;
            } else if attr.path().is_ident(crate::CRATE_NAME) {
                attr.parse_nested_meta(|meta| {
                    let path = &meta.path;
                    if path.is_ident("crate_root") {
                        if !meta.input.is_empty() {
                            let value = meta.value()?;

                            let s: LitStr = value.parse()?;
                            let root: Path = s.parse()?;
                            crate_root = Some(root);
                        }
                    } else if path.is_ident("identifier") {
                        let value = meta.value()?;
                        identifier = Some(value.parse()?);
                    } else if path.is_ident("enumerated") {
                        enumerated = true;
                    } else if path.is_ident("choice") {
                        choice = true;
                    } else if path.is_ident("set") {
                        set = true;
                    } else if path.is_ident("automatic_tags") {
                        automatic_tags = true;
                    } else if path.is_ident("tag") {
                        tag = Some(Tag::from_meta(&meta)?);
                    } else if path.is_ident("delegate") {
                        delegate = true;
                    } else if path.is_ident("from") {
                        from = Some(StringValue::from_meta(&meta)?);
                    } else if path.is_ident("size") {
                        size = Some(Value::from_meta(&meta)?);
                    } else if path.is_ident("value") {
                        value = Some(Value::from_meta(&meta)?);
                    } else {
                        return Err(meta.error(format!(
                            "unknown input provided: {}",
                            path.to_token_stream()
                        )));
                    }
                    Ok(())
                })?
            }
        }

        let is_enum = matches!(input.data, syn::Data::Enum(_));

        if !is_enum && (choice || enumerated) {
            return Err(syn::Error::new(
                input.ident.span(),
                "Structs cannot be annotated with `#[rasn(choice)]` or `#[rasn(enumerated)]`.",
            ));
        } else if is_enum && set {
            return Err(syn::Error::new(
                input.ident.span(),
                "Enums cannot be annotated with `#[rasn(set)]`.",
            ));
        } else if is_enum && ((choice && enumerated) || (!choice && !enumerated)) {
            return Err(syn::Error::new(
                input.ident.span(),
                "Enums must be annotated with either `#[rasn(choice)]` OR `#[rasn(enumerated)]`.",
            ));
        }

        let mut invalid_delegate = false;

        if is_enum && delegate {
            invalid_delegate = true;
        } else if delegate {
            if let syn::Data::Struct(data) = &input.data {
                // Delegate works also for tuple structs with multiple fields if fields beyond the first are phantom data type
                let fields = &data.fields;
                let first_is_phantom = fields.iter().next().is_some_and(|field| {
                    if let syn::Type::Path(type_path) = &field.ty {
                        if let Some(last_segment) = type_path.path.segments.last() {
                            return last_segment.ident == "PhantomData";
                        }
                    }
                    false
                });

                let non_phantom_fields_count = fields
                    .iter()
                    .filter(|field| {
                        if let syn::Type::Path(type_path) = &field.ty {
                            if let Some(last_segment) = type_path.path.segments.last() {
                                return last_segment.ident != "PhantomData";
                            }
                        }
                        true // The count of non-phantom fields should be 1
                    })
                    .count();

                invalid_delegate = first_is_phantom || non_phantom_fields_count != 1;
            }
        }

        if invalid_delegate {
            return Err(syn::Error::new(
                input.ident.span(),
                "`#[rasn(delegate)]` is only valid on single-field tuple structs. This does not count fields with `PhantomData` type. The first field must be a non-phantom field.",
            ));
        }

        Ok(Self {
            automatic_tags,
            choice,
            delegate,
            enumerated,
            set,
            tag,
            identifier,
            constraints: Constraints {
                extensible,
                from,
                size,
                value,
            },
            crate_root: crate_root.unwrap_or_else(|| {
                syn::LitStr::new(crate::CRATE_NAME, proc_macro2::Span::call_site())
                    .parse()
                    .unwrap()
            }),
        })
    }

    fn tag_tree_for_ty(&self, ty: &syn::Type) -> proc_macro2::TokenStream {
        let crate_root = &self.crate_root;
        let mut ty = ty.clone();
        ty.strip_lifetimes();

        quote!(<#ty as #crate_root::AsnType>::TAG_TREE)
    }

    pub fn has_explicit_tag(&self) -> bool {
        self.tag.as_ref().is_some_and(|tag| tag.is_explicit())
    }

    pub fn tag_for_struct(&self, fields: &syn::Fields) -> proc_macro2::TokenStream {
        let crate_root = &self.crate_root;
        self.tag
            .as_ref()
            .map(|t| t.to_tokens(crate_root))
            .or_else(|| {
                self.delegate.then(|| {
                    let mut ty = fields.iter().next().unwrap().ty.clone();
                    ty.strip_lifetimes();

                    quote!(<#ty as #crate_root::AsnType>::TAG)
                })
            })
            .or_else(|| {
                (fields == &syn::Fields::Unit).then(|| quote!(#crate_root::types::Tag::NULL))
            })
            .or_else(|| self.set.then(|| quote!(#crate_root::types::Tag::SET)))
            .unwrap_or(quote!(#crate_root::types::Tag::SEQUENCE))
    }
}

pub(crate) fn is_option_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Option"),
        syn::Type::Reference(syn::TypeReference { elem, .. }) => is_option_type(elem),
        _ => false,
    }
}

pub(crate) fn map_to_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .filter(|segment| segment.ident == "Option")
            .and_then(|segment| {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    args.args.first().and_then(|arg| {
                        if let syn::GenericArgument::Type(ty) = arg {
                            Some(ty)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }),
        _ => None,
    }
}

pub struct VariantConfig<'config> {
    pub variant: &'config syn::Variant,
    container_config: &'config Config,
    generics: &'config syn::Generics,
    pub tag: Option<Tag>,
    pub identifier: Option<LitStr>,
    pub extension_addition: bool,
    pub constraints: Constraints,
    pub context: usize,
}

impl<'config> VariantConfig<'config> {
    pub fn new(
        variant: &'config syn::Variant,
        generics: &'config syn::Generics,
        container_config: &'config Config,
        context: usize,
    ) -> syn::Result<Self> {
        let mut extensible = false;
        let mut identifier = None;
        let mut extension_addition = false;
        let mut from = None;
        let mut size = None;
        let mut tag = None;
        let mut value = None;

        for attr in &variant.attrs {
            if !attr.path().is_ident(crate::CRATE_NAME) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                let path = &meta.path;
                if path.is_ident("tag") {
                    tag = Some(Tag::from_meta(&meta)?);
                } else if path.is_ident("identifier") {
                    let value = meta.value()?;
                    identifier = Some(value.parse()?);
                } else if path.is_ident("size") {
                    size = Some(Value::from_meta(&meta)?);
                } else if path.is_ident("value") {
                    value = Some(Value::from_meta(&meta)?);
                } else if path.is_ident("from") {
                    from = Some(StringValue::from_meta(&meta)?);
                } else if path.is_ident("extensible") {
                    extensible = true;
                } else if path.is_ident("extension_addition") {
                    extension_addition = true;
                }

                Ok(())
            })?;
        }

        let fields = &variant.fields;

        if matches!(fields, syn::Fields::Unnamed(_)) && fields.len() != 1 {
            return Err(syn::Error::new(
                fields.span(),
                "Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.",
            ));
        }

        Ok(Self {
            container_config,
            extension_addition,
            generics,
            tag,
            identifier,
            variant,
            constraints: Constraints {
                extensible,
                from,
                size,
                value,
            },
            context,
        })
    }

    pub fn discriminant(&self) -> Option<isize> {
        self.variant
            .discriminant
            .as_ref()
            .and_then(|(_, expr)| match expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int.base10_parse().ok(),
                syn::Expr::Unary(syn::ExprUnary {
                    op: UnOp::Neg(_),
                    expr: e,
                    ..
                }) => {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(int),
                        ..
                    }) = e.deref()
                    {
                        int.base10_parse().map(|i: isize| -i).ok()
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    pub fn has_explicit_tag(&self) -> bool {
        self.tag.as_ref().is_some_and(|tag| tag.is_explicit())
    }

    pub fn decode(&self, name: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
        let context = self.context;
        let crate_root = &self.container_config.crate_root;
        let tag_tree = self.tag_tree()?;
        let ident = &self.variant.ident;
        let is_explicit = self.has_explicit_tag();
        let constraint_name = format_ident!("DECODE_CONSTRAINT_{}", context);
        let mut const_constraint = quote! {};

        let decode_op = match &self.variant.fields {
            syn::Fields::Unit => {
                let decode_op = if is_explicit {
                    quote!(decoder.decode_explicit_prefix::<()>(tag))
                } else {
                    quote!(<()>::decode_with_tag(decoder, tag))
                };

                quote!(#decode_op.map(|_| Self::#ident))
            }
            syn::Fields::Unnamed(_) => {
                // Assert already checked in FieldConfig
                assert_eq!(self.variant.fields.len(), 1);
                let constraints = self
                    .constraints
                    .const_expr(&self.container_config.crate_root);

                let field = FieldConfig::new(
                    self.variant.fields.iter().next().unwrap(),
                    self.container_config,
                    0,
                )?;
                let decode_operation = if is_explicit {
                    quote!(decoder.decode_explicit_prefix(tag))
                } else if self.container_config.automatic_tags || self.tag.is_some() {
                    if let Some(path) = field.default {
                        let path = path
                            .map(|path| quote!(#path))
                            .unwrap_or_else(|| quote!(<_>::default));
                        if let Some(constraints) = constraints {
                            const_constraint = quote! {
                                const #constraint_name: #crate_root::types::constraints::Constraints = #constraints;
                            };
                            quote!(decoder.decode_default_with_tag_and_constraints(tag, #path, #constraint_name))
                        } else {
                            quote!(decoder.decode_default_with_tag(tag, #path))
                        }
                    } else if let Some(constraints) = constraints {
                        const_constraint = quote! {
                            const #constraint_name: #crate_root::types::constraints::Constraints = #constraints;
                        };
                        quote!(<_>::decode_with_tag_and_constraints(decoder, tag, #constraint_name))
                    } else {
                        quote!(<_>::decode_with_tag(decoder, tag))
                    }
                } else if let Some(path) = field.default {
                    let path = path
                        .map(|path| quote!(#path))
                        .unwrap_or_else(|| quote!(<_>::default));
                    quote!(<_>::decode_default(decoder, <_>::TAG, #path))
                } else {
                    quote!(<_>::decode(decoder))
                };

                quote!(#decode_operation.map(Self::#ident))
            }
            syn::Fields::Named(_) => {
                let crate_root = &self.container_config.crate_root;
                let tag = self
                    .tag
                    .as_ref()
                    .map(|t| t.to_tokens(crate_root))
                    .unwrap_or(quote!(#crate_root::types::Tag::SEQUENCE));

                let decode_impl = super::decode::map_from_inner_type(
                    tag,
                    name,
                    self.generics,
                    &self.variant.fields,
                    Some(<_>::default()),
                    Some(quote!(Self::#ident)),
                    self.container_config,
                    is_explicit,
                );

                quote!({
                    let decode_fn = |decoder: &mut D| -> core::result::Result<_, D::Error> {
                        #decode_impl
                    };

                    (decode_fn)(decoder)
                })
            }
        };

        Ok(quote! {
            if #crate_root::types::TagTree::tag_contains(&tag, &[#tag_tree]) {
                #const_constraint
                return #decode_op
            }
        })
    }

    pub fn tag(&self) -> syn::Result<crate::tag::Tag> {
        Ok(if let Some(tag) = &self.tag {
            tag.clone()
        } else if self.container_config.automatic_tags {
            Tag::Value {
                class: crate::tag::Class::Context,
                value: syn::LitInt::new(&self.context.to_string(), proc_macro2::Span::call_site())
                    .into(),
                explicit: false,
            }
        } else {
            Tag::from_fields(&self.variant.fields)?
        })
    }

    pub fn tag_tree(&self) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.container_config.crate_root;
        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag()?.to_tokens(crate_root);
            Ok(quote!(#crate_root::types::TagTree::Leaf(#tag)))
        } else {
            let field_configs = self
                .variant
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| FieldConfig::new(f, self.container_config, i))
                .collect::<Result<Vec<_>, _>>()?;

            let field_tags = field_configs
                .iter()
                .filter(|f| f.is_option_type())
                .map(|f| f.tag_tree());

            Ok(match self.variant.fields {
                syn::Fields::Unit => {
                    quote!(#crate_root::types::TagTree::Leaf(<() as #crate_root::AsnType>::TAG))
                }
                syn::Fields::Named(_) => {
                    let error_message = format!(
                        "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                        self.variant.ident
                    );

                    quote!({
                        const FIELD_LIST: &'static [#crate_root::types::TagTree] = &[#(#field_tags),*];
                        const FIELD_TAG_TREE: #crate_root::types::TagTree = #crate_root::types::TagTree::Choice(FIELD_LIST);
                        const _: () = assert!(FIELD_TAG_TREE.is_unique(), #error_message);
                        #crate_root::types::TagTree::Leaf(#crate_root::types::Tag::SEQUENCE)
                    })
                }
                syn::Fields::Unnamed(_) => {
                    // Assert already checked in FieldConfig
                    assert_eq!(self.variant.fields.len(), 1);
                    let mut ty = self.variant.fields.iter().next().unwrap().ty.clone();
                    ty.strip_lifetimes();

                    quote!(<#ty as #crate_root::AsnType>::TAG_TREE)
                }
            })
        }
    }
}

#[derive(Debug)]
pub struct FieldConfig<'a> {
    pub field: &'a syn::Field,
    pub container_config: &'a Config,
    pub tag: Option<Tag>,
    pub identifier: Option<LitStr>,
    pub default: Option<Option<syn::Path>>,
    pub extension_addition: bool,
    pub extension_addition_group: bool,
    pub constraints: Constraints,
    pub context: usize,
}

pub enum FieldType {
    Required,
    Optional,
    Default,
}

impl<'a> FieldConfig<'a> {
    pub fn new(
        field: &'a syn::Field,
        container_config: &'a Config,
        context: usize,
    ) -> syn::Result<Self> {
        let mut default = None;
        let mut tag = None;
        let mut size = None;
        let mut identifier = None;
        let mut from = None;
        let mut value = None;
        let mut extensible = false;
        let mut extension_addition = false;
        let mut extension_addition_group = false;
        /*if !field.attrs.is_empty() {
            panic!("{:?}", field)
        }*/

        for attr in &field.attrs {
            if !attr.path().is_ident(crate::CRATE_NAME) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                let path = &meta.path;
                if path.is_ident("tag") {
                    if container_config.automatic_tags {
                        return Err(meta.error(
                            "You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tags)]`",
                        ));
                    }
                    tag = Some(Tag::from_meta(&meta)?);
                } else if path.is_ident("default") {
                    if meta.input.is_empty() || meta.input.peek(Token![,]) {
                        default = Some(None);
                    } else {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        default = Some(Some(s.parse()?));
                    }
                } else if path.is_ident("identifier") {
                    let value = meta.value()?;
                    identifier = Some(value.parse()?);
                } else if path.is_ident("size") {
                    size = Some(Value::from_meta(&meta)?);
                } else if path.is_ident("value") {
                    value = Some(Value::from_meta(&meta)?);
                } else if path.is_ident("from") {
                    from = Some(StringValue::from_meta(&meta)?);
                } else if path.is_ident("extensible") {
                    extensible = true;
                } else if path.is_ident("extension_addition") {
                    extension_addition = true;
                } else if path.is_ident("extension_addition_group") {
                    extension_addition_group = true;
                } else {
                    return Err(meta.error(format!(
                        "unknown field tag {:?}",
                        path.get_ident().map(ToString::to_string)
                    )));
                }
                Ok(())
            })?;
        }

        if extension_addition && extension_addition_group {
            return Err(syn::Error::new(field.span(), "field cannot be both `extension_addition` and `extension_addition_group`, choose one"));
        }

        Ok(Self {
            container_config,
            default,
            field,
            identifier,
            tag,
            extension_addition,
            extension_addition_group,
            constraints: Constraints {
                extensible,
                from,
                size,
                value,
            },
            context,
        })
    }

    pub fn encode(
        &self,
        use_self: bool,
        type_params: &[Ident],
    ) -> syn::Result<proc_macro2::TokenStream> {
        let context = self.context;
        let this = use_self.then(|| quote!(self.));
        let tag = self.tag();
        let i = syn::Index::from(context);
        let field = self
            .field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));
        let crate_root = &self.container_config.crate_root;
        let identifier = self
            .identifier
            .clone()
            .or(self
                .field
                .ident
                .as_ref()
                .map(|i| LitStr::new(&i.to_string(), Span::call_site())))
            .map(|i| quote!(#crate_root::types::Identifier(Some(#i))))
            .unwrap_or(quote!(#crate_root::types::Identifier::EMPTY));
        let mut ty = self.field.ty.clone();
        ty.strip_lifetimes();
        let default_fn = self.default_fn().map(|d| quote!(#d,));
        let has_generics = !type_params.is_empty() && {
            if let Type::Path(ref ty) = ty {
                ty.path.segments.iter().any(|seg| {
                    let type_string = seg.into_token_stream().to_string();
                    let type_parts: Vec<&str> = type_string.split(" ").collect();
                    type_params
                        .iter()
                        .any(|param| type_parts.contains(&param.to_string().as_str()))
                })
            } else {
                false
            }
        };
        let constraint_name = format_ident!("FIELD_CONSTRAINT_{}", context);
        let constraints = self
            .constraints
            .const_expr(&self.container_config.crate_root)
            .unwrap_or_else(|| quote!(#crate_root::types::Constraints::default()));
        let constraint_def = if has_generics {
            quote! {
                let #constraint_name: #crate_root::types::Constraints  = <#ty as #crate_root::AsnType>::CONSTRAINTS.intersect(const {#constraints});
            }
        } else {
            quote! {
                const #constraint_name : #crate_root::types::Constraints = <#ty as #crate_root::AsnType>::CONSTRAINTS.intersect(
                    #constraints
                );
            }
        };

        let encode = if self.tag.is_some() || self.container_config.automatic_tags {
            if self.tag.as_ref().is_some_and(|tag| tag.is_explicit()) {
                if self.default.is_some() {
                    // Note: encoder must be aware if the field is optional and present, so we should not do the presence check on this level
                    quote!(encoder.encode_default_with_explicit_prefix(#tag, &self.#field, #default_fn #identifier)?;)
                } else {
                    // Note: encoder must be aware if the field is optional and present, so we should not do the presence check on this level
                    quote!(encoder.encode_explicit_prefix(#tag, &self.#field, #identifier)?;)
                }
            } else if self.extension_addition {
                quote!(
                    #constraint_def
                    encoder.encode_extension_addition(
                        #tag,
                        #constraint_name,
                        &#this #field,
                        #identifier,
                    )?;
                )
            } else if self.extension_addition_group {
                quote!(encoder.encode_extension_addition_group(#this #field.as_ref(), #identifier)?;)
            } else {
                match (self.constraints.has_constraints(), self.default.is_some()) {
                    (true, true) => {
                        quote!(
                            #constraint_def
                            encoder.encode_default_with_tag_and_constraints(
                                #tag,
                                #constraint_name,
                                &#this #field,
                                #default_fn
                                #identifier,
                            )?;
                        )
                    }
                    (true, false) => {
                        quote!(
                            #constraint_def
                            #this #field.encode_with_tag_and_constraints(
                                encoder,
                                #tag,
                                #constraint_name,
                                #default_fn
                                #identifier,
                            )?;
                        )
                    }
                    (false, true) => {
                        quote!(encoder.encode_default_with_tag(#tag, &#this #field, #default_fn #identifier)?;)
                    }
                    (false, false) => {
                        quote!(#this #field.encode_with_tag_and_identifier(encoder, #tag, #identifier)?;)
                    }
                }
            }
        } else if self.extension_addition {
            quote!(
                #constraint_def
                encoder.encode_extension_addition(
                    #tag,
                    #constraint_name,
                    &#this #field,
                    #identifier,
                )?;
            )
        } else if self.extension_addition_group {
            quote!(encoder.encode_extension_addition_group(#this #field.as_ref(), #identifier)?;)
        } else {
            match (self.constraints.has_constraints(), self.default.is_some()) {
                (true, true) => {
                    quote!(
                        #constraint_def
                        encoder.encode_default_with_constraints(
                            #constraint_name,
                            &#this #field,
                            #default_fn
                            #identifier,
                        )?;
                    )
                }
                (true, false) => {
                    quote!(
                        #constraint_def
                        #this #field.encode_with_constraints_and_identifier(
                            encoder,
                            #constraint_name,
                            #identifier,
                        )?;
                    )
                }
                (false, true) => {
                    quote!(encoder.encode_default(&#this #field, #default_fn #identifier)?;)
                }
                (false, false) => quote!(#this #field.encode(encoder)?;),
            }
        };

        Ok(quote! {
            #encode
        })
    }

    pub fn decode_field_def(
        &self,
        name: &syn::Ident,
        type_params: &[Ident],
    ) -> syn::Result<proc_macro2::TokenStream> {
        let lhs = self.field.ident.as_ref().map(|i| quote!(#i :));
        let decode_op = self.decode(name, type_params)?;
        Ok(quote!(#lhs #decode_op))
    }

    pub fn decode(
        &self,
        name: &syn::Ident,
        type_params: &[Ident],
    ) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.container_config.crate_root;
        let ty = &self.field.ty;
        let ident = format!(
            "{}.{}",
            name,
            self.field
                .ident
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_else(|| self.context.to_string())
        );
        let or_else = quote!(.map_err(|error| #crate_root::de::Error::field_error(#ident, error.into(), decoder.codec()))?);
        let default_fn = self.default_fn();

        let tag = self.tag();
        let has_generics = !type_params.is_empty() && {
            if let Type::Path(ty) = ty {
                ty.path.segments.iter().any(|seg| {
                    let type_string = seg.into_token_stream().to_string();
                    let type_parts: Vec<&str> = type_string.split(" ").collect();
                    type_params
                        .iter()
                        .any(|param| type_parts.contains(&param.to_string().as_str()))
                })
            } else {
                false
            }
        };
        let constraint_name = format_ident!("CONSTRAINT_{}", self.context);
        let constraints = self.constraints.const_expr(crate_root);
        let constraint_def = if has_generics {
            quote! {
                let #constraint_name: #crate_root::types::Constraints  = <#ty as #crate_root::AsnType>::CONSTRAINTS.intersect(const {#constraints});
            }
        } else {
            quote! {
                const #constraint_name : #crate_root::types::Constraints = <#ty as #crate_root::AsnType>::CONSTRAINTS.intersect(
                    #constraints
                );
            }
        };
        let handle_extension = if self.is_not_option_or_default_type() {
            quote!(.ok_or_else(|| {
                #crate_root::de::Error::field_error(#ident, #crate_root::error::DecodeError::required_extension_not_present(#tag, decoder.codec()), decoder.codec())})?)
        } else if self.is_default_type() {
            quote!(.unwrap_or_else(#default_fn))
        } else {
            quote!()
        };

        let decode = if self.extension_addition_group {
            quote!(decoder.decode_extension_addition_group() #or_else)
        } else {
            match (
                (self.tag.is_some() || self.container_config.automatic_tags)
                    .then(|| self.tag.as_ref().is_some_and(|tag| tag.is_explicit())),
                self.default.as_ref().map(|path| {
                    path.as_ref()
                        .map_or(quote!(<_>::default), |path| quote!(#path))
                }),
                self.constraints.has_constraints(),
            ) {
                (Some(true), _, _) => {
                    if self.is_option_type() {
                        quote!(
                            decoder.decode_optional_with_explicit_prefix(#tag)?
                        )
                    } else if self.is_default_type() {
                        quote!(
                            decoder.decode_optional_with_explicit_prefix(#tag)?.unwrap_or_else(#default_fn)
                        )
                    } else {
                        // False positive
                        quote!(decoder.decode_explicit_prefix(#tag) #or_else)
                    }
                }
                (Some(false), Some(path), true) => {
                    quote!({
                        #constraint_def
                        decoder.decode_default_with_tag_and_constraints(
                            #tag,
                            #path,
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (Some(false), Some(path), false) => {
                    quote!(decoder.decode_default_with_tag(#tag, #path) #or_else)
                }
                (Some(false), None, true) => {
                    quote!({
                        #constraint_def
                        <_>::decode_with_tag_and_constraints(
                            decoder,
                            #tag,
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (Some(false), None, false) => {
                    quote!(<_>::decode_with_tag(decoder, #tag) #or_else)
                }
                (None, Some(path), true) => {
                    quote!({
                        #constraint_def
                        decoder.decode_default_with_constraints(
                            #path,
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (None, Some(path), false) => {
                    quote!(decoder.decode_default(#path) #or_else)
                }
                (None, None, true) => {
                    quote!({
                        #constraint_def
                        <_>::decode_with_constraints(
                            decoder,
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (None, None, false) => {
                    quote!(<_>::decode(decoder) #or_else)
                }
            }
        };

        Ok(if self.extension_addition {
            match (
                (self.tag.is_some() || self.container_config.automatic_tags)
                    .then(|| self.tag.as_ref().is_some_and(|tag| tag.is_explicit())),
                self.default.as_ref().map(|path| {
                    path.as_ref()
                        .map_or(quote!(<_>::default), |path| quote!(#path))
                }),
                self.constraints.has_constraints(),
            ) {
                (Some(true), _, constraints) => {
                    let or_else = if self.is_option_type() {
                        quote!(.ok())
                    } else if self.is_default_type() {
                        quote!(.ok().unwrap_or_else(#default_fn))
                    } else {
                        // False positive
                        #[allow(clippy::redundant_clone)]
                        or_else.clone()
                    };
                    if constraints {
                        {
                            quote!(
                                #constraint_def
                                decoder.decode_extension_addition_with_explicit_tag_and_constraints(
                                    #tag,
                                    #constraint_name
                                    ) #or_else #handle_extension)
                        }
                    } else {
                        quote!(decoder.decode_extension_addition_with_explicit_tag_and_constraints(
                            #tag,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS) #or_else #handle_extension)
                    }
                }
                (Some(false), Some(path), true) => {
                    quote!(
                        #constraint_def
                        decoder.decode_extension_addition_with_default_and_tag_and_constraints(
                            #tag,
                            #path,
                            #constraint_name
                            ) #or_else
                    )
                }
                (Some(false), None, true) => {
                    quote!(
                        {
                            #constraint_def
                            <_>::decode_extension_addition_with_tag_and_constraints(
                                decoder,
                                #tag,
                                #constraint_name
                            ) #or_else #handle_extension
                        }
                    )
                }
                (Some(false), Some(path), false) => {
                    quote!(decoder.decode_extension_addition_with_default_and_tag(#tag, #path) #or_else)
                }
                (Some(false), None, false) => {
                    quote!(<_>::decode_extension_addition_with_tag(decoder, #tag) #or_else #handle_extension)
                }
                (None, Some(path), true) => {
                    quote!(
                        {
                        #constraint_def
                        decoder.decode_extension_addition_with_default_and_constraints(
                            #path,
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (None, Some(path), false) => {
                    quote!(
                        decoder.decode_extension_addition_with_default(
                            #path,
                        ) #or_else
                    )
                }
                (None, None, true) => {
                    quote!({
                        #constraint_def
                        decoder.decode_extension_addition_with_constraints(
                            #constraint_name,
                        ) #or_else }
                    )
                }
                (None, None, false) => {
                    quote! {
                        decoder.decode_extension_addition() #or_else #handle_extension
                    }
                }
            }
        } else {
            quote!({
                #decode
            })
        })
    }

    pub fn default_fn(&self) -> Option<proc_macro2::TokenStream> {
        let ty = &self.field.ty;
        self.default.as_ref().map(|default_fn| match default_fn {
            Some(path) => quote!(#path),
            None => quote!(<#ty>::default),
        })
    }

    pub fn tag_derive(&self) -> proc_macro2::TokenStream {
        if let Some(tag) = &self.tag {
            tag.to_attribute_tokens()
        } else if self.container_config.automatic_tags {
            let context = syn::Index::from(self.context);
            quote!(#[rasn(tag(context, #context))])
        } else {
            quote!()
        }
    }

    pub fn tag(&self) -> proc_macro2::TokenStream {
        let context = self.context;
        let crate_root = &self.container_config.crate_root;
        if let Some(tag) = &self.tag {
            let tag = tag.to_tokens(crate_root);
            quote!(#tag)
        } else if self.container_config.automatic_tags {
            quote!(#crate_root::types::Tag::new(#crate_root::types::Class::Context, #context as u32))
        } else {
            let mut ty = self.field.ty.clone();
            ty.strip_lifetimes();
            quote!(<#ty as #crate_root::AsnType>::TAG)
        }
    }

    pub fn tag_tree(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let ty = &self.field.ty;

        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag();
            quote!(#crate_root::types::TagTree::Leaf(#tag))
        } else {
            self.container_config.tag_tree_for_ty(ty)
        }
    }

    pub fn to_field_metadata(&self) -> proc_macro2::TokenStream {
        let context = self.context;
        let crate_root = &self.container_config.crate_root;
        let tag = self.tag();
        let tag_tree = self.tag_tree();
        let name = self
            .identifier
            .clone()
            .or(self
                .field
                .ident
                .as_ref()
                .map(|id| syn::LitStr::new(&id.to_string(), proc_macro2::Span::call_site())))
            .unwrap_or(syn::LitStr::new("", proc_macro2::Span::call_site()));

        let constructor = quote::format_ident!(
            "{}",
            match self.field_type() {
                FieldType::Required => "new_required",
                FieldType::Optional => "new_optional",
                FieldType::Default => "new_default",
            }
        );

        quote!({ #crate_root::types::fields::Field::#constructor(#context, #tag, #tag_tree, #name) })
    }

    pub fn field_type(&self) -> FieldType {
        if self.is_option_type() {
            FieldType::Optional
        } else if self.is_default_type() {
            FieldType::Default
        } else {
            FieldType::Required
        }
    }

    pub fn is_extension(&self) -> bool {
        self.extension_addition || self.extension_addition_group
    }

    pub fn is_not_extension(&self) -> bool {
        !self.is_extension()
    }

    pub fn is_option_type(&self) -> bool {
        is_option_type(&self.field.ty)
    }

    pub fn is_default_type(&self) -> bool {
        self.default.is_some()
    }

    pub fn is_option_or_default_type(&self) -> bool {
        self.is_default_type() || self.is_option_type()
    }

    pub fn is_not_option_or_default_type(&self) -> bool {
        !self.is_option_or_default_type()
    }
}

#[derive(Clone, Debug)]
pub struct Constraint<T> {
    pub constraint: T,
    pub extensible: Option<Vec<T>>,
}

impl<T> From<T> for Constraint<T> {
    fn from(constraint: T) -> Self {
        Self {
            constraint,
            extensible: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringValue(pub Vec<u32>);

impl StringValue {
    fn from_meta(item: &syn::meta::ParseNestedMeta) -> syn::Result<Constraint<StringValue>> {
        let mut values = Vec::new();
        let mut extensible: Option<_> = None;

        enum StringRange {
            Single(u32),
            Range(u32, u32),
        }

        fn parse_character(string: &str) -> Option<u32> {
            string.chars().map(u32::from).next()
        }

        let content;
        parenthesized!(content in item.input);
        while !content.is_empty() {
            let (span, string) = if content.peek(syn::LitStr) {
                let str: syn::LitStr = content.parse()?;

                (str.span(), str.value())
            } else if content.peek(syn::Ident) {
                let path: syn::Path = content.parse()?;
                (path.span(), path.require_ident()?.to_string())
            } else {
                return Err(content.error(format!("Unsupported meta item: {:?}", content)));
            };
            if string == "extensible" {
                extensible = Some(Vec::new());
                skip_comma(&content);
                continue;
            }

            if string.len() == 1 {
                values.push(parse_character(&string).map(StringRange::Single).unwrap());
                skip_comma(&content);
                continue;
            }

            let Some((start, mut end)) = string.split_once("..") else {
                return Err(syn::Error::new(span, format!("unknown format: {string}, must be a single character or range of characters (`..`, `..=`)")));
            };

            let Some(start) = parse_character(start) else {
                return Err(syn::Error::new(
                    span,
                    format!("start of range was an invalid character: {start}"),
                ));
            };

            let is_inclusive = end.starts_with('=');
            if is_inclusive {
                end = &end[1..];
            }

            let Some(end) = parse_character(end) else {
                return Err(syn::Error::new(
                    span,
                    format!("end of range was an invalid character: {end}"),
                ));
            };

            if let Some(extensible_values) = extensible.as_mut() {
                extensible_values.push(StringRange::Range(start, end + is_inclusive as u32));
            } else {
                values.push(StringRange::Range(start, end + is_inclusive as u32));
            }
            skip_comma(&content);
        }
        let into_flat_set = |constraints: Vec<_>| {
            let mut set = constraints
                .iter()
                .flat_map(|from| match from {
                    StringRange::Single(value) => vec![*value],
                    StringRange::Range(start, end) => (*start..*end).collect::<Vec<u32>>(),
                })
                .collect::<Vec<u32>>();
            set.sort();
            set.dedup();
            set
        };

        Ok(Constraint {
            constraint: Self((into_flat_set)(values)),
            extensible: extensible.map(|values| vec![Self((into_flat_set)(values))]),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Single(i128),
    Range(Option<i128>, Option<i128>),
}

impl Value {
    fn from_meta(item: &syn::meta::ParseNestedMeta) -> syn::Result<Constraint<Value>> {
        let mut extensible = None;
        let mut constraint = None;

        // Attempts to parse either size or value constraint value.
        // These constraints are i128 types - proc macros might add i128 suffix so we need to remove that.
        // Also check if the value is a valid number in general, noting the underscore separator as well.
        fn parse_character(string: &str) -> Option<i128> {
            let filtered: String = string
                .chars()
                .filter(|&c| !c.is_whitespace() && c != '_')
                .collect();
            // Remove the "i128" suffix if it exists.
            if filtered.ends_with("i128") {
                filtered[..filtered.len() - "i128".len()].parse().ok()
            } else {
                filtered.parse().ok()
            }
        }

        let content;
        parenthesized!(content in item.input);
        if content.is_empty() {
            return Err(content.error("Missing content inside `value()`"));
        }
        while !content.is_empty() {
            let (span, string) = if content.peek(syn::LitStr) {
                let str: syn::LitStr = content.parse()?;
                (str.span(), str.value())
            } else if content.peek(syn::Ident) {
                let ident: syn::Ident = content.parse()?;
                (ident.span(), ident.to_string())
            } else if content.peek(syn::LitInt) {
                let int: syn::LitInt = content.parse()?;
                constraint = Some(int.base10_parse().map(Value::Single)?);
                skip_comma(&content);
                continue;
            } else {
                return Err(content.error(format!("Value Unsupported meta item: {:?}", content)));
            };

            if string == "extensible" {
                extensible = Some(Vec::new());
                skip_comma(&content);
                continue;
            }

            let value = if let Some(number) = parse_character(&string) {
                Value::Single(number)
            } else {
                let Some((start, mut end)) = string.split_once("..") else {
                    return Err(syn::Error::new(span, format!("unknown format: {string}, must be a single value or range of values (`..`, `..=`)")));
                };

                let start_parsed = parse_character(start);
                if start_parsed.is_none() && !start.is_empty() {
                    return Err(syn::Error::new(
                        span,
                        format!("start of the range constraint was an invalid value: {start:?}"),
                    ));
                }
                let is_inclusive = end.starts_with('=');
                if is_inclusive {
                    end = &end[1..];
                }

                let end_parsed = parse_character(end).map(|end| end - (!is_inclusive) as i128);
                if end_parsed.is_none() && !end.is_empty() {
                    return Err(syn::Error::new(
                        span,
                        format!("end of the range constraint was an invalid value: {end:?}"),
                    ));
                }
                Value::Range(start_parsed, end_parsed)
            };

            if let Some(extensible_values) = extensible.as_mut() {
                extensible_values.push(value)
            } else if constraint.is_none() {
                constraint = Some(value);
            } else {
                return Err(syn::Error::new(
                    span,
                    "Multiple non-extensible value constraints are not permitted.",
                ));
            }
            skip_comma(&content);
        }

        Ok(Constraint {
            constraint: constraint.unwrap(),
            extensible,
        })
    }
}

fn skip_comma(content: &syn::parse::ParseBuffer) {
    if content.peek(Token![,]) {
        let _: Token![,] = content.parse().unwrap();
    }
}
