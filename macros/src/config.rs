use crate::tag::Tag;

pub struct Config {
    pub crate_root: syn::Path,
    pub enumerated: bool,
    pub choice: bool,
    pub automatic_tagging: bool,
    pub tag: Option<Tag>,
}

impl Config {
    pub fn from_attributes(input: &syn::DeriveInput) -> Self {
        let mut choice = false;
        let mut crate_root = None;
        let mut enumerated = false;
        let mut automatic_tagging = false;
        let mut tag = None;

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

        Self {
            automatic_tagging,
            choice,
            enumerated,
            tag,
            crate_root: crate_root.unwrap_or_else(|| {
                syn::LitStr::new(crate::CRATE_NAME, proc_macro2::Span::call_site())
                    .parse()
                    .unwrap()
            }),
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

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value }) = &self.tag {
            quote!(#crate_root::Tag::new(#crate_root::tag::Class::#class, #value))
        } else if self.container_config.automatic_tagging {
            quote!(#crate_root::Tag::new(#crate_root::tag::Class::Context, #context))
        } else {
            Tag::from_fields(&self.variant.fields, crate_root)
        }
    }
}

pub struct FieldConfig<'a> {
    field: &'a syn::Field,
    container_config: &'a Config,
    pub tag: Option<Tag>,
}

impl<'a> FieldConfig<'a> {
    pub fn new(field: &'a syn::Field, container_config: &'a Config) -> Self {
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
                }
            }
        }

        Self {
            field,
            container_config,
            tag,
        }
    }

    pub fn tag(&self, context: usize) -> proc_macro2::TokenStream {
        let crate_root = &self.container_config.crate_root;
        if let Some(Tag { class, value }) = &self.tag {
            if self.container_config.automatic_tagging {
                panic!("You can't use the `#[rasn(tag)]` with `#[rasn(automatic_tagging)]`.")
            }
            quote!(#crate_root::Tag::new(#crate_root::tag::Class::#class, #value))
        } else if self.container_config.automatic_tagging {
            quote!(#crate_root::Tag::new(#crate_root::tag::Class::Context, #context as u32))
        } else {
            let ty = &self.field.ty;
            quote!(<#ty>::TAG)
        }
    }
}
