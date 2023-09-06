use quote::ToTokens;
use syn::{Lit, NestedMeta, Path};

use crate::{ext::TypeExt, tag::Tag};

#[derive(Clone, Debug, Default)]
pub struct Constraints {
    pub extensible: bool,
    pub from: Option<Constraint<StringValue>>,
    pub size: Option<Constraint<Value>>,
    pub value: Option<Constraint<Value>>,
}

impl Constraints {
    pub fn const_static_def(&self, crate_root: &syn::Path) -> Option<proc_macro2::TokenStream> {
        self.const_expr(crate_root).map(
            |expr| quote!(const CONSTRAINTS: #crate_root::types::Constraints<'static> = #expr;),
        )
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
    pub enumerated: bool,
    pub choice: bool,
    pub set: bool,
    pub automatic_tags: bool,
    pub option_type: OptionalEnum,
    pub delegate: bool,
    pub tag: Option<Tag>,
    pub constraints: Constraints,
}

impl Config {
    pub fn from_attributes(input: &syn::DeriveInput) -> Self {
        let mut choice = false;
        let mut set = false;
        let mut crate_root = None;
        let mut enumerated = false;
        let mut automatic_tags = false;
        let mut tag = None;
        let mut option = None;
        let mut from = None;
        let mut size = None;
        let mut value = None;
        let mut delegate = false;
        let extensible = input
            .attrs
            .iter()
            .any(|a| a.path.is_ident("non_exhaustive"));

        let mut iter = input
            .attrs
            .iter()
            .filter(|a| a.path.is_ident(crate::CRATE_NAME))
            .map(|a| a.parse_meta().unwrap());

        while let Some(syn::Meta::List(list)) = iter.next() {
            for item in list.nested.iter().filter_map(|n| match n {
                syn::NestedMeta::Meta(m) => Some(m),
                _ => None,
            }) {
                let path = item.path();

                if path.is_ident("crate_root") {
                    if let syn::Meta::NameValue(nv) = item {
                        crate_root = match &nv.lit {
                            syn::Lit::Str(s) => s.parse::<syn::Path>().ok(),
                            _ => None,
                        };
                    }
                } else if path.is_ident("enumerated") {
                    enumerated = true;
                } else if path.is_ident("choice") {
                    choice = true;
                } else if path.is_ident("set") {
                    set = true;
                } else if path.is_ident("automatic_tags") {
                    automatic_tags = true;
                } else if path.is_ident("option_type") {
                    if let syn::Meta::List(list) = item {
                        let filter_into_paths = |nm: &_| match nm {
                            syn::NestedMeta::Meta(meta) => Some(meta.path().clone()),
                            _ => None,
                        };
                        let mut iter = list
                            .nested
                            .iter()
                            .take(3)
                            .filter_map(filter_into_paths)
                            .fuse();

                        let path = iter.next();
                        let some_variant = iter.next();
                        let none_variant = iter.next();
                        option = Some((path, some_variant, none_variant));
                    }
                } else if path.is_ident("tag") {
                    tag = Tag::from_meta(item);
                } else if path.is_ident("delegate") {
                    delegate = true;
                } else if path.is_ident("from") {
                    from = Some(StringValue::from_meta(item));
                } else if path.is_ident("size") {
                    size = Some(Value::from_meta(item));
                } else if path.is_ident("value") {
                    value = Some(Value::from_meta(item));
                } else {
                    panic!("unknown input provided: {}", path.to_token_stream());
                }
            }
        }

        let is_enum = matches!(input.data, syn::Data::Enum(_));

        if !is_enum && (choice || enumerated) {
            panic!("Structs cannot be annotated with `#[rasn(choice)]` or `#[rasn(enumerated)]`.");
        } else if is_enum && set {
            panic!("Enums cannot be annotated with `#[rasn(set)]`.");
        } else if is_enum && ((choice && enumerated) || (!choice && !enumerated)) {
            panic!(
                "Enums must be annotated with either `#[rasn(choice)]` OR `#[rasn(enumerated)]`."
            )
        }

        let mut invalid_delegate = false;

        if is_enum && delegate {
            invalid_delegate = true;
        } else if delegate {
            if let syn::Data::Struct(data) = &input.data {
                invalid_delegate = data.fields.len() != 1;
            }
        }

        if invalid_delegate {
            panic!("`#[rasn(delegate)]` is only valid on single-unit structs.");
        }

        let option_type = {
            let (path, some_variant, none_variant) = option.unwrap_or((None, None, None));

            OptionalEnum {
                path: path
                    .and_then(|path| path.get_ident().cloned())
                    .unwrap_or_else(|| syn::Ident::new("Option", proc_macro2::Span::call_site())),
                some_variant: syn::TypePath {
                    path: some_variant.unwrap_or_else(|| {
                        Path::from(syn::Ident::new("Some", proc_macro2::Span::call_site()))
                    }),
                    qself: None,
                }
                .into(),
                none_variant: syn::TypePath {
                    path: none_variant.unwrap_or_else(|| {
                        Path::from(syn::Ident::new("None", proc_macro2::Span::call_site()))
                    }),
                    qself: None,
                }
                .into(),
            }
        };

        Self {
            automatic_tags,
            choice,
            delegate,
            enumerated,
            option_type,
            set,
            tag,
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
        }
    }

    fn tag_tree_for_ty(&self, ty: &syn::Type) -> proc_macro2::TokenStream {
        let crate_root = &self.crate_root;
        let mut ty = ty.clone();
        ty.strip_lifetimes();

        quote!(<#ty as #crate_root::AsnType>::TAG_TREE)
    }

    pub fn has_explicit_tag(&self) -> bool {
        self.tag.as_ref().map_or(false, |tag| tag.is_explicit())
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
            .or_else(|| self.set.then(|| quote!(#crate_root::Tag::SET)))
            .unwrap_or(quote!(#crate_root::Tag::SEQUENCE))
    }
}

#[derive(Clone, Debug)]
pub struct OptionalEnum {
    pub path: syn::Ident,
    pub some_variant: syn::Type,
    pub none_variant: syn::Type,
}

impl OptionalEnum {
    pub(crate) fn is_option_type(&self, ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map_or(false, |segment| segment.ident == self.path),
            _ => false,
        }
    }

    pub(crate) fn map_to_inner_type<'ty>(&self, ty: &'ty syn::Type) -> Option<&'ty syn::Type> {
        match ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .filter(|segment| segment.ident == self.path)
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
}

pub struct VariantConfig<'config> {
    pub variant: &'config syn::Variant,
    container_config: &'config Config,
    generics: &'config syn::Generics,
    pub tag: Option<Tag>,
    pub extension_addition: bool,
    pub constraints: Constraints,
}

impl<'config> VariantConfig<'config> {
    pub fn new(
        variant: &'config syn::Variant,
        generics: &'config syn::Generics,
        container_config: &'config Config,
    ) -> Self {
        let mut extensible = false;
        let mut extension_addition = false;
        let mut from = None;
        let mut size = None;
        let mut tag = None;
        let mut value = None;

        let mut iter = variant
            .attrs
            .iter()
            .filter_map(|a| a.parse_meta().ok())
            .filter(|m| m.path().is_ident(crate::CRATE_NAME));

        while let Some(syn::Meta::List(list)) = iter.next() {
            for item in list.nested.iter().filter_map(|n| match n {
                syn::NestedMeta::Meta(m) => Some(m),
                _ => None,
            }) {
                let path = item.path();
                if path.is_ident("tag") {
                    tag = Tag::from_meta(item);
                } else if path.is_ident("size") {
                    size = Some(Value::from_meta(item));
                } else if path.is_ident("value") {
                    value = Some(Value::from_meta(item));
                } else if path.is_ident("from") {
                    from = Some(StringValue::from_meta(item));
                } else if path.is_ident("extensible") {
                    extensible = true;
                } else if path.is_ident("extension_addition") {
                    extension_addition = true;
                }
            }
        }

        Self {
            container_config,
            extension_addition,
            generics,
            tag,
            variant,
            constraints: Constraints {
                extensible,
                from,
                size,
                value,
            },
        }
    }

    pub fn discriminant(&self) -> Option<usize> {
        self.variant
            .discriminant
            .as_ref()
            .and_then(|(_, expr)| match expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int.base10_parse().ok(),
                _ => None,
            })
    }

    pub fn has_explicit_tag(&self) -> bool {
        self.tag.as_ref().map_or(false, |tag| tag.is_explicit())
    }

    pub fn decode(&self, name: &syn::Ident, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let tag_tree = self.tag_tree(context);
        let ident = &self.variant.ident;
        let is_explicit = self.has_explicit_tag();

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
                if self.variant.fields.len() != 1 {
                    panic!("Tuple struct variants should contain only a single element.");
                }
                let constraints = self
                    .constraints
                    .const_expr(&self.container_config.crate_root);

                let field = FieldConfig::new(
                    self.variant.fields.iter().next().unwrap(),
                    self.container_config,
                );
                let decode_operation = if is_explicit {
                    quote!(decoder.decode_explicit_prefix(tag))
                } else if self.container_config.automatic_tags || self.tag.is_some() {
                    if let Some(path) = field.default {
                        let path = path
                            .map(|path| quote!(#path))
                            .unwrap_or_else(|| quote!(<_>::default));
                        if let Some(constraints) = constraints {
                            quote!(decoder.decode_default_with_tag_and_constraints(tag, #path, #constraints))
                        } else {
                            quote!(decoder.decode_default_with_tag(tag, #path))
                        }
                    } else if let Some(constraints) = constraints {
                        quote!(<_>::decode_with_tag_and_constraints(decoder, tag, #constraints))
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
                    .unwrap_or(quote!(#crate_root::Tag::SEQUENCE));

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

        quote! {
            if #crate_root::TagTree::tag_contains(&tag, &[#tag_tree]) {
                return #decode_op
            }
        }
    }

    pub fn tag(&self, context: usize) -> crate::tag::Tag {
        if let Some(tag) = &self.tag {
            tag.clone()
        } else if self.container_config.automatic_tags {
            Tag::Value {
                class: crate::tag::Class::Context,
                value: syn::LitInt::new(&context.to_string(), proc_macro2::Span::call_site())
                    .into(),
                explicit: false,
            }
        } else {
            Tag::from_fields(&self.variant.fields)
        }
    }

    pub fn tag_tree(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag(context).to_tokens(crate_root);
            quote!(#crate_root::TagTree::Leaf(#tag))
        } else {
            let field_tags = self
                .variant
                .fields
                .iter()
                .enumerate()
                .filter(|(_, f)| FieldConfig::new(f, self.container_config).is_option_type())
                .map(|(i, f)| FieldConfig::new(f, self.container_config).tag_tree(i));

            match self.variant.fields {
                syn::Fields::Unit => {
                    quote!(#crate_root::TagTree::Leaf(<() as #crate_root::AsnType>::TAG))
                }
                syn::Fields::Named(_) => {
                    let error_message = format!(
                        "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                        self.variant.ident
                    );

                    quote!({
                        const FIELD_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags),*];
                        const FIELD_TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(FIELD_LIST);
                        const _: () = assert!(FIELD_TAG_TREE.is_unique(), #error_message);
                        #crate_root::TagTree::Leaf(#crate_root::Tag::SEQUENCE)
                    })
                }
                syn::Fields::Unnamed(_) => {
                    if self.variant.fields.iter().count() != 1 {
                        panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                    } else {
                        let mut ty = self.variant.fields.iter().next().unwrap().ty.clone();
                        ty.strip_lifetimes();

                        quote!(<#ty as #crate_root::AsnType>::TAG_TREE)
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FieldConfig<'a> {
    pub field: &'a syn::Field,
    pub container_config: &'a Config,
    pub tag: Option<Tag>,
    pub default: Option<Option<syn::Path>>,
    pub extension_addition: bool,
    pub extension_addition_group: bool,
    pub constraints: Constraints,
}

pub enum FieldType {
    Required,
    Optional,
    Default,
}

impl<'a> FieldConfig<'a> {
    pub fn new(field: &'a syn::Field, container_config: &'a Config) -> Self {
        let mut default = None;
        let mut tag = None;
        let mut size = None;
        let mut from = None;
        let mut value = None;
        let mut extensible = false;
        let mut extension_addition = false;
        let mut extension_addition_group = false;
        let mut iter = field
            .attrs
            .iter()
            .map(|a| a.parse_meta().unwrap())
            .filter(|m| m.path().is_ident(crate::CRATE_NAME));

        while let Some(syn::Meta::List(list)) = iter.next() {
            for item in list.nested.iter().filter_map(|n| match n {
                syn::NestedMeta::Meta(m) => Some(m),
                _ => None,
            }) {
                let path = item.path();
                if path.is_ident("tag") {
                    tag = Tag::from_meta(item);
                } else if path.is_ident("default") {
                    default = Some(match item {
                        syn::Meta::NameValue(value) => match &value.lit {
                            syn::Lit::Str(lit_str) => lit_str.parse().map(Some).unwrap(),
                            _ => panic!("Unsupported type for default."),
                        },
                        _ => None,
                    });
                } else if path.is_ident("size") {
                    size = Some(Value::from_meta(item));
                } else if path.is_ident("value") {
                    value = Some(Value::from_meta(item));
                } else if path.is_ident("from") {
                    from = Some(StringValue::from_meta(item));
                } else if path.is_ident("extensible") {
                    extensible = true;
                } else if path.is_ident("extension_addition") {
                    extension_addition = true;
                } else if path.is_ident("extension_addition_group") {
                    extension_addition_group = true;
                } else {
                    panic!(
                        "unknown field tag {:?}",
                        path.get_ident().map(ToString::to_string)
                    );
                }
            }
        }

        if extension_addition && extension_addition_group {
            panic!("field cannot be both `extension_addition` and `extension_addition_group`, choose one");
        }

        Self {
            container_config,
            default,
            field,
            tag,
            extension_addition,
            extension_addition_group,
            constraints: Constraints {
                extensible,
                from,
                size,
                value,
            },
        }
    }

    pub fn encode(&self, context: usize, use_self: bool) -> proc_macro2::TokenStream {
        let this = use_self.then(|| quote!(self.));
        let tag = self.tag(context);
        let i = syn::Index::from(context);
        let field = self
            .field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));
        let mut ty = self.field.ty.clone();
        let crate_root = &self.container_config.crate_root;
        ty.strip_lifetimes();
        let default_fn = self.default.as_ref().map(|default_fn| match default_fn {
            Some(path) => quote!(#path),
            None => quote!(<#ty>::default),
        });

        let encode = if self.tag.is_some() || self.container_config.automatic_tags {
            if self.tag.as_ref().map_or(false, |tag| tag.is_explicit()) {
                let encode = quote!(encoder.encode_explicit_prefix(#tag, &self.#field)?;);
                if self.is_option_type() {
                    let none = &self.container_config.option_type.none_variant;
                    quote! {
                        if !matches!(&#this #field, #none) {
                            #encode
                        }
                    }
                } else {
                    encode
                }
            } else if self.extension_addition {
                let constraints = self
                    .constraints
                    .const_expr(&self.container_config.crate_root)
                    .unwrap_or_else(|| quote!(<_>::default()));
                quote!(
                    encoder.encode_extension_addition(
                        #tag,
                        <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        &#this #field
                    )?;
                )
            } else if self.extension_addition_group {
                quote!(encoder.encode_extension_addition_group(#this #field.as_ref())?;)
            } else {
                let constraints = self
                    .constraints
                    .const_expr(&self.container_config.crate_root)
                    .unwrap_or_else(|| quote!(<_>::default()));
                match (self.constraints.has_constraints(), self.default.is_some()) {
                    (true, true) => {
                        quote!(
                            encoder.encode_default_with_tag_and_constraints(
                                #tag,
                                <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                                &#this #field,
                                #default_fn
                            )?;
                        )
                    }
                    (true, false) => {
                        quote!(
                            #this #field.encode_with_tag_and_constraints(
                                encoder,
                                #tag,
                                <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                                #default_fn
                            )?;
                        )
                    }
                    (false, true) => {
                        quote!(encoder.encode_default_with_tag(#tag, &#this #field, #default_fn)?;)
                    }
                    (false, false) => quote!(#this #field.encode_with_tag(encoder, #tag)?;),
                }
            }
        } else if self.extension_addition {
            let constraints = self
                .constraints
                .const_expr(&self.container_config.crate_root)
                .unwrap_or_else(|| quote!(<_>::default()));
            quote!(
                encoder.encode_extension_addition(
                    #tag,
                    <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                    &#this #field
                )?;
            )
        } else if self.extension_addition_group {
            quote!(encoder.encode_extension_addition_group(#this #field.as_ref())?;)
        } else {
            match (self.constraints.has_constraints(), self.default.is_some()) {
                (true, true) => {
                    let constraints = self
                        .constraints
                        .const_expr(&self.container_config.crate_root);
                    quote!(
                        encoder.encode_default_with_constraints(
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                            &#this #field,
                            #default_fn
                        )?;
                    )
                }
                (true, false) => {
                    let constraints = self
                        .constraints
                        .const_expr(&self.container_config.crate_root);
                    quote!(
                        #this #field.encode_with_constraints(
                            encoder,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        )?;
                    )
                }
                (false, true) => quote!(encoder.encode_default(&#this #field, #default_fn)?;),
                (false, false) => quote!(#this #field.encode(encoder)?;),
            }
        };

        quote! {
            #encode
        }
    }

    pub fn decode_field_def(&self, name: &syn::Ident, context: usize) -> proc_macro2::TokenStream {
        let lhs = self.field.ident.as_ref().map(|i| quote!(#i :));
        let decode_op = self.decode(name, context);
        quote!(#lhs #decode_op)
    }

    pub fn decode(&self, name: &syn::Ident, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let ty = &self.field.ty;
        let ident = format!(
            "{}.{}",
            name,
            self.field
                .ident
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_else(|| context.to_string())
        );

        let or_else = quote!(.map_err(|error| #crate_root::de::Error::field_error(#ident, error))?);
        let default_fn = self.default.as_ref().map(|default_fn| match default_fn {
            Some(path) => quote!(#path),
            None => quote!(<#ty>::default),
        });

        let tag = self.tag(context);
        let constraints = self.constraints.const_expr(crate_root);
        let handle_extension = if self.is_not_option_or_default_type() {
            quote!(.ok_or_else(|| #crate_root::de::Error::field_error(#ident, "extension required but not present"))?)
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
                    .then(|| self.tag.as_ref().map_or(false, |tag| tag.is_explicit())),
                self.default.as_ref().map(|path| {
                    path.as_ref()
                        .map_or(quote!(<_>::default), |path| quote!(#path))
                }),
                self.constraints.has_constraints(),
            ) {
                (Some(true), _, _) => {
                    let or_else = if self.is_option_type() {
                        quote!(.ok())
                    } else if self.is_default_type() {
                        quote!(.ok().unwrap_or_else(#default_fn))
                    } else {
                        // False positive
                        #[allow(clippy::redundant_clone)]
                        or_else.clone()
                    };

                    quote!(decoder.decode_explicit_prefix(#tag) #or_else)
                }
                (Some(false), Some(path), true) => {
                    quote!(
                        decoder.decode_default_with_tag_and_constraints(
                            #tag,
                            #path,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (Some(false), Some(path), false) => {
                    quote!(decoder.decode_default_with_tag(#tag, #path) #or_else)
                }
                (Some(false), None, true) => {
                    quote!(
                        <_>::decode_with_tag_and_constraints(
                            decoder,
                            #tag,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (Some(false), None, false) => {
                    quote!(<_>::decode_with_tag(decoder, #tag) #or_else)
                }
                (None, Some(path), true) => {
                    quote!(
                        decoder.decode_default_with_constraints(
                            #path,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (None, Some(path), false) => {
                    quote!(decoder.decode_default(#path) #or_else)
                }
                (None, None, true) => {
                    quote!(
                        <_>::decode_with_constraints(
                            decoder,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (None, None, false) => {
                    quote!(<_>::decode(decoder) #or_else)
                }
            }
        };

        if self.extension_addition {
            match (
                self.default.as_ref().map(|path| {
                    path.as_ref()
                        .map_or(quote!(<_>::default), |path| quote!(#path))
                }),
                self.constraints.has_constraints(),
            ) {
                (Some(path), true) => {
                    quote!(
                        decoder.decode_extension_addition_with_default_and_constraints(
                            #path,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (Some(path), false) => {
                    quote!(
                        decoder.decode_extension_addition_with_default(
                            #path,
                        ) #or_else
                    )
                }
                (None, true) => {
                    quote!(
                        decoder.decode_extension_addition_with_constraints(
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(#constraints),
                        ) #or_else
                    )
                }
                (None, false) => {
                    quote! {
                        decoder.decode_extension_addition() #or_else #handle_extension
                    }
                }
            }
        } else {
            quote!({
                #decode
            })
        }
    }

    pub fn tag_derive(&self, context: usize) -> proc_macro2::TokenStream {
        if let Some(tag) = &self.tag {
            if self.container_config.automatic_tags {
                panic!("You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tags)]`")
            }

            tag.to_attribute_tokens()
        } else if self.container_config.automatic_tags {
            let context = syn::Index::from(context);
            quote!(#[rasn(tag(context, #context))])
        } else {
            quote!()
        }
    }

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(tag) = &self.tag {
            if self.container_config.automatic_tags {
                panic!("You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tags)]`")
            }
            let tag = tag.to_tokens(crate_root);
            quote!(#tag)
        } else if self.container_config.automatic_tags {
            quote!(#crate_root::Tag::new(#crate_root::types::Class::Context, #context as u32))
        } else {
            let mut ty = self.field.ty.clone();
            ty.strip_lifetimes();
            quote!(<#ty as #crate_root::AsnType>::TAG)
        }
    }

    pub fn tag_tree(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let ty = &self.field.ty;

        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag(context);
            quote!(#crate_root::TagTree::Leaf(#tag))
        } else {
            self.container_config.tag_tree_for_ty(ty)
        }
    }

    pub fn to_field_metadata(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let tag = self.tag(context);
        let tag_tree = self.tag_tree(context);

        let constructor = quote::format_ident!(
            "{}",
            match self.field_type() {
                FieldType::Required => "new_required",
                FieldType::Optional => "new_optional",
                FieldType::Default => "new_default",
            }
        );

        quote!({ #crate_root::types::fields::Field::#constructor(#tag, #tag_tree) })
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
        self.container_config
            .option_type
            .is_option_type(&self.field.ty)
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
    fn from_meta(item: &syn::Meta) -> Constraint<StringValue> {
        let mut values = Vec::new();
        let mut extensible: Option<_> = None;

        enum StringRange {
            Single(u32),
            Range(u32, u32),
        }

        fn parse_character(string: &str) -> Option<u32> {
            string.chars().map(u32::from).next()
        }

        let syn::Meta::List(list) = item else {
            panic!("Unsupported meta item: {:?}", item);
        };

        for item in &list.nested {
            let string = match item {
                NestedMeta::Lit(Lit::Str(string)) => string.value(),
                NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap().to_string(),
                item => panic!("Unsupported meta item: {:?}", item),
            };

            if string == "extensible" {
                extensible = Some(Vec::new());
                continue;
            }

            if string.len() == 1 {
                values.push(parse_character(&string).map(StringRange::Single).unwrap());
                continue;
            }

            let Some((start, mut end)) = string.split_once("..") else {
                panic!("unknown format: {string}, must be a single character or range of characters (`..`, `..=`)")
            };

            let Some(start) = parse_character(start) else {
                panic!("start of range was an invalid character: {start}")
            };

            let is_inclusive = end.starts_with('=');
            if is_inclusive {
                end = &end[1..];
            }

            let Some(end) = parse_character(end) else {
                panic!("end of range was an invalid character: {end}")
            };

            if let Some(extensible_values) = extensible.as_mut() {
                extensible_values.push(StringRange::Range(start, end + is_inclusive as u32));
            } else {
                values.push(StringRange::Range(start, end + is_inclusive as u32));
            }
        }

        let into_flat_set = |constraints: Vec<_>| {
            use rayon::prelude::*;
            let mut set = constraints
                .iter()
                .flat_map(|from| match from {
                    StringRange::Single(value) => vec![*value],
                    StringRange::Range(start, end) => (*start..*end).into_par_iter().collect(),
                })
                .collect::<Vec<u32>>();
            set.sort();
            set.dedup();
            set
        };

        Constraint {
            constraint: Self((into_flat_set)(values)),
            extensible: extensible.map(|values| vec![Self((into_flat_set)(values))]),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Single(i128),
    Range(Option<i128>, Option<i128>),
}

impl Value {
    fn from_meta(item: &syn::Meta) -> Constraint<Value> {
        let mut extensible = None;
        let mut constraint = None;

        fn parse_character(string: &str) -> Option<i128> {
            string.parse().ok()
        }

        let syn::Meta::List(list) = item else {
            panic!("Unsupported meta item: {:?}", item);
        };

        for item in list.nested.iter() {
            let string = match item {
                NestedMeta::Lit(Lit::Str(string)) => string.value(),
                NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap().to_string(),
                NestedMeta::Lit(Lit::Int(int)) => {
                    constraint = Some(int.base10_parse().map(Value::Single).unwrap());
                    continue;
                }
                _ => panic!("Unsupported meta item: {item:?}"),
            };

            if string == "extensible" {
                extensible = Some(Vec::new());
                continue;
            }

            let value = if let Some(number) = parse_character(&string) {
                Value::Single(number)
            } else {
                let Some((start, mut end)) = string.split_once("..") else {
                    panic!("unknown format: {string}, must be a single character or range of characters (`..`, `..=`)")
                };

                let start = parse_character(start);
                let is_inclusive = end.starts_with('=');
                if is_inclusive {
                    end = &end[1..];
                }

                let end = parse_character(end).map(|end| end - (!is_inclusive) as i128);
                Value::Range(start, end)
            };

            if let Some(extensible_values) = extensible.as_mut() {
                extensible_values.push(value)
            } else if constraint.is_none() {
                constraint = Some(value);
            } else {
                panic!("Multiple non-extensible value constraints are not permitted.");
            }
        }

        Constraint {
            constraint: constraint.unwrap(),
            extensible,
        }
    }
}
