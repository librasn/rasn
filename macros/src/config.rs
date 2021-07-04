use syn::Path;

use crate::tag::Tag;

#[derive(Debug)]
pub struct Config {
    pub crate_root: Path,
    pub enumerated: bool,
    pub choice: bool,
    pub automatic_tagging: bool,
    pub option_type: OptionalEnum,
    pub tag: Option<Tag>,
}

impl Config {
    pub fn from_attributes(input: &syn::DeriveInput) -> Self {
        let mut choice = false;
        let mut crate_root = None;
        let mut enumerated = false;
        let mut automatic_tagging = false;
        let mut tag = None;
        let mut option = None;

        let mut iter = input
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
                } else if path.is_ident("automatic_tagging") {
                    automatic_tagging = true;
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
                }
            }
        }

        let is_enum = match input.data {
            syn::Data::Enum(_) => true,
            _ => false,
        };

        if is_enum && ((choice && enumerated) || (!choice && !enumerated)) {
            panic!("A Rust enum must be marked either `choice` OR `enumerated`")
        }

        let option_type = {
            let (path, some_variant, none_variant) = option.unwrap_or((None, None, None));

            OptionalEnum {
                path: syn::TypePath {
                    path: path.unwrap_or_else(|| {
                        Path::from(syn::Ident::new("Option", proc_macro2::Span::call_site()))
                    }),
                    qself: None,
                }
                .into(),
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
            automatic_tagging,
            choice,
            enumerated,
            tag,
            option_type,
            crate_root: crate_root.unwrap_or_else(|| {
                syn::LitStr::new(crate::CRATE_NAME, proc_macro2::Span::call_site())
                    .parse()
                    .unwrap()
            }),
        }
    }
}

#[derive(Debug)]
pub struct OptionalEnum {
    pub path: syn::Type,
    pub some_variant: syn::Type,
    pub none_variant: syn::Type,
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

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value }) = &self.tag {
            quote!(#crate_root::Tag::new(#class, #value))
        } else if self.container_config.automatic_tagging {
            quote!(#crate_root::Tag::new(#crate_root::types::Class::Context, #context))
        } else {
            Tag::from_fields(&self.variant.fields, crate_root)
        }
    }
}

#[derive(Debug)]
pub struct FieldConfig<'a> {
    pub field: &'a syn::Field,
    pub container_config: &'a Config,
    pub choice: bool,
    pub tag: Option<Tag>,
}

impl<'a> FieldConfig<'a> {
    pub fn new(field: &'a syn::Field, container_config: &'a Config) -> Self {
        let mut tag = None;
        let mut choice = false;
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
                } else if path.is_ident("choice") {
                    choice = true;
                }
            }
        }

        Self {
            field,
            container_config,
            choice,
            tag,
        }
    }

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value }) = &self.tag {
            if self.container_config.automatic_tagging {
                panic!("You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tagging)]`")
            }
            quote!(#crate_root::Tag::new(#class, #value))
        } else if self.container_config.automatic_tagging {
            quote!(#crate_root::Tag::new(#crate_root::types::Class::Context, #context as u32))
        } else {
            let ty = &self.field.ty;
            quote!(<#ty as #crate_root::AsnType>::TAG)
        }
    }
}
