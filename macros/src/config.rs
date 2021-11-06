use quote::ToTokens;
use syn::Path;

use crate::tag::Tag;

#[derive(Debug)]
pub struct Config {
    pub crate_root: Path,
    pub enumerated: bool,
    pub choice: bool,
    pub set: bool,
    pub automatic_tags: bool,
    pub option_type: OptionalEnum,
    pub delegate: bool,
    pub tag: Option<Tag>,
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
        let mut delegate = false;

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
            set,
            enumerated,
            tag,
            option_type,
            delegate,
            crate_root: crate_root.unwrap_or_else(|| {
                syn::LitStr::new(crate::CRATE_NAME, proc_macro2::Span::call_site())
                    .parse()
                    .unwrap()
            }),
        }
    }

    fn tag_tree_for_ty(&self, ty: &syn::Type) -> proc_macro2::TokenStream {
        let crate_root = &self.crate_root;

        quote!(if !<#ty as #crate_root::AsnType>::TAG.is_choice() {
            #crate_root::TagTree::Leaf(<#ty as #crate_root::AsnType>::TAG)
        } else {
            <#ty as #crate_root::AsnType>::TAG_TREE
        })
    }
}

#[derive(Debug)]
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

pub struct VariantConfig<'a> {
    variant: &'a syn::Variant,
    container_config: &'a Config,
    pub tag: Option<Tag>,
}

impl<'a> VariantConfig<'a> {
    pub fn new(variant: &'a syn::Variant, container_config: &'a Config) -> Self {
        let mut tag = None;
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
                }
            }
        }

        Self {
            variant,
            container_config,
            tag,
        }
    }

    pub fn decode(&self, name: &syn::Ident, context: usize) -> proc_macro2::TokenStream {
        let tag = self.tag(context);
        let ident = &self.variant.ident;
        let is_explicit = self.tag.as_ref().map_or(false, |tag| tag.explicit);
        let explicit_wrapper = |tag, decode| quote!(decoder.decode_sequence(#tag, #decode));

         match &self.variant.fields {
            syn::Fields::Unit => {
                let decode_op = if is_explicit {
                    (explicit_wrapper)(tag, quote!(<()>::decode))
                } else {
                    quote!(<()>::decode_with_tag(decoder, #tag))
                };

                quote!(if let Ok(()) = #decode_op { return Ok(#name::#ident) })
            }
            syn::Fields::Unnamed(_) => {
                if self.variant.fields.len() != 1 {
                    panic!("Tuple struct variants should contain only a single element.");
                }

                let decode_operation = if is_explicit {
                    (explicit_wrapper)(tag, quote!(|decoder| <_>::decode(decoder)))
                } else if self.container_config.automatic_tags || self.tag.is_some()
                {
                    quote!(<_>::decode_with_tag(decoder, #tag))
                } else {
                    quote!(<_>::decode(decoder))
                };

                quote! {
                    if let Ok(value) = #decode_operation.map(#name::#ident) { return Ok(value) }
                }
            }
            syn::Fields::Named(_) => {
                let decode_fields = self.variant.fields.iter().enumerate().map(|(i, field)| {
                    let field_config = FieldConfig::new(field, self.container_config);
                    field_config.decode(name, i)
                });

                let decode_op = if is_explicit {
                    (explicit_wrapper)(tag.clone(), quote!(decode_fn))
                } else {
                    quote!((decode_fn)(decoder))
                };

                quote! {
                    let decode_fn = |decoder: &mut D| {
                        decoder.decode_sequence(#tag, |decoder| {
                            Ok::<_, D::Error>(#name::#ident { #(#decode_fields),* })
                        })
                    };

                    if let Ok(value) = #decode_op { return Ok(value) }
                }
            }
        }
    }

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value, .. }) = &self.tag {
            quote!(#crate_root::Tag::new(#class, #value))
        } else if self.container_config.automatic_tags {
            quote!(#crate_root::Tag::new(#crate_root::types::Class::Context, #context as u32))
        } else {
            Tag::from_fields(&self.variant.fields, crate_root)
        }
    }

    pub fn tag_tree(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if self.tag.is_some() || self.container_config.automatic_tags {
            let tag = self.tag(context);
            quote!(#crate_root::TagTree::Leaf(#tag),)
        } else {
            let field_tags = self
                .variant
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| FieldConfig::new(f, self.container_config).tag_tree(i));

            match self.variant.fields {
                syn::Fields::Unit => {
                    quote!(#crate_root::TagTree::Leaf(<() as #crate_root::AsnType>::TAG),)
                }
                syn::Fields::Named(_) => {
                    quote!({
                        const FIELD_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags,)*];
                        const FIELD_TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(FIELD_LIST);
                        #crate_root::sa::const_assert!(FIELD_TAG_TREE.is_unique());
                        #crate_root::TagTree::Leaf(#crate_root::Tag::SEQUENCE)
                    },)
                }
                syn::Fields::Unnamed(_) => {
                    if self.variant.fields.iter().count() != 1 {
                        panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                    } else {
                        let ty = &self.variant.fields.iter().next().unwrap().ty;

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
}

impl<'a> FieldConfig<'a> {
    pub fn new(field: &'a syn::Field, container_config: &'a Config) -> Self {
        let mut default = None;
        let mut tag = None;
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
                        syn::Meta::List(list) => list
                            .nested
                            .iter()
                            .cloned()
                            .filter_map(unested_meta)
                            .map(|m| m.path().clone())
                            .next(),
                        _ => None,
                    });
                }
            }
        }

        Self {
            field,
            container_config,
            tag,
            default,
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

        let encode_op = if self.tag.is_some() || self.container_config.automatic_tags {
            if self.tag.as_ref().map_or(false, |tag| tag.explicit) {
                quote!(encoder.encode_explicit_prefix(#tag, &self.#field)?;)
            } else {
                quote!(#this #field.encode_with_tag(encoder, #tag)?;)
            }
        } else {
            quote!(#this #field.encode(encoder)?;)
        };

        if self.default.is_some() {
            let ty = &self.field.ty;
            let default_fn = match self.default.as_ref().unwrap() {
                Some(path) => quote!(#path),
                None => quote!(<#ty>::default()),
            };

            quote! {
                if #this #field != #default_fn {
                    #encode_op
                }
            }
        } else {
            encode_op
        }
    }

    pub fn decode(&self, name: &syn::Ident, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        let or_else = match self.default {
            Some(Some(ref path)) => quote! { .unwrap_or_else(#path) },
            Some(None) => quote! { .unwrap_or_default() },
            None => {
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
            }
        };

        let lhs = self.field.ident.as_ref().map(|i| quote!(#i :));
        let tag = self.tag(context);

        if self.tag.is_some() || self.container_config.automatic_tags {
            if self.tag.as_ref().map_or(false, |tag| tag.explicit) {
                quote!(#lhs decoder.decode_explicit_prefix(#tag) #or_else)
            } else {
                quote!(#lhs <_>::decode_with_tag(decoder, #tag) #or_else)
            }
        } else {
            quote!(#lhs <_>::decode(decoder) #or_else)
        }
    }

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value, .. }) = &self.tag {
            if self.container_config.automatic_tags {
                panic!("You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tags)]`")
            }
            quote!(#crate_root::Tag::new(#class, #value))
        } else if self.container_config.automatic_tags {
            quote!(#crate_root::Tag::new(#crate_root::types::Class::Context, #context as u32))
        } else {
            let ty = &self.field.ty;
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

    pub fn is_option_type(&self) -> bool {
        self.container_config
            .option_type
            .is_option_type(&self.field.ty)
    }
}

fn unested_meta(nm: syn::NestedMeta) -> Option<syn::Meta> {
    match nm {
        syn::NestedMeta::Meta(m) => Some(m),
        _ => None,
    }
}
