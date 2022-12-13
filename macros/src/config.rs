use quote::ToTokens;
use syn::{Lit, NestedMeta, Path};

use crate::{ext::TypeExt, tag::Tag};

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
    pub extensible: bool,
    pub from: Option<Vec<StringValue>>,
    pub size: Option<Value>,
    pub value: Option<Value>,
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
            extensible,
            from,
            option_type,
            set,
            size,
            value,
            tag,
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
    variant: &'config syn::Variant,
    container_config: &'config Config,
    generics: &'config syn::Generics,
    pub tag: Option<Tag>,
    pub extension_addition: bool,
}

impl<'config> VariantConfig<'config> {
    pub fn new(
        variant: &'config syn::Variant,
        generics: &'config syn::Generics,
        container_config: &'config Config,
    ) -> Self {
        let mut tag = None;
        let mut extension_addition = false;
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
        }
    }

    pub fn has_explicit_tag(&self) -> bool {
        self.tag.as_ref().map_or(false, |tag| tag.is_explicit())
    }

    pub fn decode(&self, name: &syn::Ident, context: usize) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let crate_root = &self.container_config.crate_root;
        let tag = self.tag(context).to_tokens(&self.container_config.crate_root);
        let ident = &self.variant.ident;
        let const_name = quote::format_ident!("{ident}{context}");
        let is_explicit = self.has_explicit_tag();
        let const_name_def = quote!(const #const_name : #crate_root::Tag = #tag;);

        match &self.variant.fields {
            syn::Fields::Unit => {
                let decode_op = if is_explicit {
                    quote!(decoder.decode_explicit_prefix::<()>(#tag))
                } else {
                    quote!(<()>::decode_with_tag(decoder, #tag))
                };

                (const_name_def, quote!(#const_name => #decode_op.map(|_| Self::#ident)))
            }
            syn::Fields::Unnamed(_) => {
                if self.variant.fields.len() != 1 {
                    panic!("Tuple struct variants should contain only a single element.");
                }

                let field = FieldConfig::new(self.variant.fields.iter().next().unwrap(), self.container_config);
                let decode_operation = if is_explicit {
                    quote!(decoder.decode_explicit_prefix(#tag))
                } else if self.container_config.automatic_tags || self.tag.is_some() {
                    if let Some(path) = field.default {
                        let path = path.map(|path| quote!(#path)).unwrap_or_else(|| quote!(<_>::default));
                        quote!(decoder.decode_default_with_tag(#tag, #path))
                    } else {
                        quote!(<_>::decode_with_tag(decoder, #tag))
                    }
                } else {
                    if let Some(path) = field.default {
                        let path = path.map(|path| quote!(#path)).unwrap_or_else(|| quote!(<_>::default));
                        quote!(<_>::decode_default(decoder, <_>::TAG, #path))
                    } else {
                        quote!(<_>::decode(decoder))
                    }
                };

                (const_name_def, quote!(#const_name => #decode_operation.map(Self::#ident)))
            }
            syn::Fields::Named(_) => {
                let crate_root = &self.container_config.crate_root;
                let tag = self
                    .tag
                    .as_ref()
                    .map(|t| t.to_tokens(crate_root))
                    .unwrap_or(quote!(#crate_root::Tag::SEQUENCE));

                let decode_impl = super::decode::map_from_inner_type(
                    tag.clone(),
                    &name,
                    &self.generics,
                    &self.variant.fields,
                    Some(<_>::default()),
                    Some(quote!(Self::#ident)),
                    &self.container_config,
                    is_explicit,
                );

                (const_name_def, quote! {
                    #const_name => {
                        let decode_fn = |decoder: &mut D| -> core::result::Result<_, D::Error> {
                            #decode_impl
                        };

                        (decode_fn)(decoder)
                    }
                })
            }
        }
    }

    pub fn tag(&self, context: usize) -> crate::tag::Tag {
        if let Some(tag) = &self.tag {
            tag.clone()
        } else if self.container_config.automatic_tags {
            Tag::Value { class: crate::tag::Class::Context, value: syn::LitInt::new(&context.to_string(), proc_macro2::Span::call_site()).into(), explicit: false }
        } else {
            Tag::from_fields(&self.variant.fields)
        }
    }

    pub fn tag_tree(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag(context).to_tokens(crate_root);
            quote!(#crate_root::TagTree::Leaf(#tag),)
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
                    quote!(#crate_root::TagTree::Leaf(<() as #crate_root::AsnType>::TAG),)
                }
                syn::Fields::Named(_) => {
                    let error_message = format!(
                        "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                        self.variant.ident
                    );

                    quote!({
                        const FIELD_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags,)*];
                        const FIELD_TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(FIELD_LIST);
                        const _: () = assert!(FIELD_TAG_TREE.is_unique(), #error_message);
                        #crate_root::TagTree::Leaf(#crate_root::Tag::SEQUENCE)
                    },)
                }
                syn::Fields::Unnamed(_) => {
                    if self.variant.fields.iter().count() != 1 {
                        panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                    } else {
                        let mut ty = self.variant.fields.iter().next().unwrap().ty.clone();
                        ty.strip_lifetimes();

                        quote!(<#ty as #crate_root::AsnType>::TAG_TREE,)
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
    pub size: Option<Value>,
}

pub enum FieldType {
    Required,
    Optional,
    Default
}

impl<'a> FieldConfig<'a> {
    pub fn new(field: &'a syn::Field, container_config: &'a Config) -> Self {
        let mut default = None;
        let mut tag = None;
        let mut size = None;
        let mut iter = field
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
                } else {
                    panic!("unknown field tag {:?}", path.get_ident().map(ToString::to_string));
                }
            }
        }

        Self {
            container_config,
            default,
            field,
            size,
            tag,
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
        ty.strip_lifetimes();
        let default_fn = self.default.as_ref().map(|default_fn| match default_fn {
            Some(path) => quote!(#path),
            None => quote!(<#ty>::default),
        });

        let constraints = self.size.is_some().then(|| self.constraints_def());

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
            } else {
                match (constraints.is_some(), self.default.is_some()) {
                    (true, true) => {
                        quote!(encoder.encode_default_with_tag_and_constraints(#tag, constraints, &#this #field, #default_fn)?;)
                    },
                    (true, false) => {
                        quote!(encoder.encode_with_tag_and_constraints(#tag, constraints, &#this #field, #default_fn)?;)
                    },
                    (false, true) => {
                        quote!(encoder.encode_default_with_tag(#tag, &#this #field, #default_fn)?;)
                    },
                    (false, false) => quote!(#this #field.encode_with_tag(encoder, #tag)?;),
                }
            }
        } else {
            match (constraints.is_some(), self.default.is_some()) {
                (true, true) => quote!(encoder.encode_default_with_constraints(constraints, &#this #field, #default_fn)?;),
                (true, false) => quote!(#this #field.encode_with_constraints(encoder, constraints)?;),
                (false, true) => quote!(encoder.encode_default(&#this #field, #default_fn)?;),
                (false, false) => quote!(#this #field.encode(encoder)?;),
            }
        };

        quote! {
            #constraints
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
        let or_else = {
            let ident = format!(
                "{}.{}",
                name,
                self.field
                    .ident
                    .as_ref()
                    .map(|ident| ident.to_string())
                    .unwrap_or_else(|| context.to_string())
            );
            quote!(.map_err(|error| #crate_root::de::Error::field_error(#ident, error))?)
        };

        let tag = self.tag(context);
        let constraints = self.constraints_def();

        let decode = if self.tag.is_some() || self.container_config.automatic_tags {
            if self.tag.as_ref().map_or(false, |tag| tag.is_explicit()) {
                quote!(decoder.decode_explicit_prefix(#tag) #or_else)
            } else {
                if let Some(path) = &self.default {
                    let path = path.as_ref().map(|path| quote!(#path)).unwrap_or_else(|| quote!(<_>::default));
                    if constraints.is_some() {
                        quote!(decoder.decode_default_with_tag_and_constraints(#tag, #path, constraints) #or_else)
                    } else {
                        quote!(decoder.decode_default_with_tag(#tag, #path) #or_else)
                    }
                } else {
                    if constraints.is_some() {
                        quote!(<_>::decode_with_tag_and_constraints(#tag, constraints) #or_else)
                    } else {
                        quote!(<_>::decode_with_tag(decoder, #tag) #or_else)
                    }
                }
            }
        } else if let Some(path) = self.default.as_ref() {
            match (constraints.is_some(), path) {
                (true, Some(path)) => quote!(decoder.decode_default_with_tag_and_constraints(#tag, constraints, #path) #or_else),
                (true, None) => quote!(decoder.decode_default_with_constraints(constraints, <_>::default) #or_else),
                (false, Some(path)) => quote!(decoder.decode_default_with_tag(#tag, #path) #or_else),
                (false, None) => quote!(decoder.decode_default(<_>::default) #or_else),
            }
        } else {
            if constraints.is_some() {
                quote!(<_>::decode_with_constraints(decoder, constraints) #or_else)
            } else {
                quote!(<_>::decode(decoder) #or_else)
            }
        };

        quote!({
            #constraints
            #decode
        })
    }

    pub fn tag_derive(&self, context: usize) -> proc_macro2::TokenStream {
        if let Some(tag) = &self.tag
        {
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

        let constructor = quote::format_ident!("{}", match self.field_type() {
            FieldType::Required => "new_required",
            FieldType::Optional => "new_optional",
            FieldType::Default => "new_default",
        });

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

    fn size_def(&self) -> Option<proc_macro2::TokenStream> {
        let crate_root = &self.container_config.crate_root;
        self.size.as_ref().map(|value| match value {
            Value::Range(Some(min), Some(max)) => {
                quote!(#crate_root::types::constraints::Size::new(#crate_root::types::constraints::Range::new(#min as usize, #max as usize)))
            }
            Value::Range(Some(min), None) => {
                quote!(#crate_root::types::constraints::Size::new(#crate_root::types::constraints::Range::start_from(#min as usize)))
            }
            Value::Range(None, Some(max)) => {
                quote!(#crate_root::types::constraints::Size::new(#crate_root::types::constraints::Range::up_to(#max as usize)))
            }
            Value::Range(None, None) => {
                quote!(#crate_root::types::constraints::Size::new(#crate_root::types::constraints::Range::new(usize::MIN, usize::MAX)))
            }
            Value::Single(length) => {
                quote!(#crate_root::types::constraints::Size::new(#crate_root::types::constraints::Range::single_value(#length as usize)))
            }
        })
    }

    fn constraints_def(&self) -> Option<proc_macro2::TokenStream> {
        let crate_root = &self.container_config.crate_root;
        let size_constraint = self.size_def();
        let ty = &self.field.ty;

       size_constraint.is_some().then(|| quote! {
            let constraints: &[_] = &[#crate_root::types::Constraint::Size(#size_constraint)];
            let constraints: #crate_root::types::Constraints = #crate_root::types::Constraints::new(constraints);
            let constraints = #crate_root::types::Constraints::override_constraints(<#ty>::CONSTRAINTS, constraints);
        })
    }

}

#[derive(Clone, Debug)]
pub enum StringValue {
    Single(u32),
    Range(u32, u32),
}

impl StringValue {
    fn from_meta(item: &syn::Meta) -> Vec<StringValue> {
        let mut values = Vec::new();

        fn parse_character(string: &str) -> Option<u32> {
            (string.len() == 1).then_some(0).and_then(|_| {
                string.chars()
                    .next()
                    .map(u32::from)
            })
        }

        let syn::Meta::List(list) = item else {
            panic!("Unsupported meta item: {:?}", item);
        };

        for item in &list.nested {
            let NestedMeta::Lit(Lit::Str(string)) = item else {
                panic!("Unsupported meta item: {:?}", item);
            };

            let string = string.value();

            if string.len() == 1 {
                values.push(parse_character(&string).map(StringValue::Single).unwrap());
                continue;
            }

            let Some((start, mut end)) = string.split_once("..") else {
                panic!("unknown format: {string}, must be a single character or range of characters (`..`, `..=`)")
            };

            let Some(start) = parse_character(start) else {
                panic!("start of range was an invalid character: {start}")
            };

            let is_inclusive = end.starts_with("=");
            if is_inclusive {
                end = &end[1..];
            }

            let Some(end) = parse_character(end) else {
                panic!("end of range was an invalid character: {end}")
            };

            values.push(StringValue::Range(start, end + is_inclusive as u32));
        }

        values
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Single(i128),
    Range(Option<i128>, Option<i128>),
}

impl Value {
    fn from_meta(item: &syn::Meta) -> Value {
        fn parse_character(string: &str) -> Option<i128> {
            string.parse().ok()
        }

        let syn::Meta::List(list) = item else {
            panic!("Unsupported meta item: {:?}", item);
        };

        let Some(item) = list.nested.iter().next() else {
            panic!("value required");
        };

        let string = match item {
            NestedMeta::Lit(Lit::Str(string)) => string.value(),
            NestedMeta::Lit(Lit::Int(int)) => {
                return int.base10_parse().map(Value::Single).unwrap();
            }
            _ => panic!("Unsupported meta item: {item:?}"),
        };

        if string.len() == 1 {
            return parse_character(&string).map(Value::Single).unwrap();
        }

        let Some((start, mut end)) = string.split_once("..") else {
            panic!("unknown format: {string}, must be a single character or range of characters (`..`, `..=`)")
        };

        let start = parse_character(start);
        let is_inclusive = end.starts_with("=");
        if is_inclusive {
            end = &end[1..];
        }

        let end = parse_character(end).map(|end| end + is_inclusive as i128);

        Value::Range(start, end)
    }
}
