pub struct Config {
    pub crate_root: syn::Path,
    pub enumerated: bool,
    pub choice: bool,
}

impl Config {
    pub fn from_attributes(input: &syn::DeriveInput) -> Self {
        let mut choice = false;
        let mut crate_root = None;
        let mut enumerated = false;

        let mut iter = input.attrs.iter()
            .filter_map(|a| a.parse_meta().ok())
            .filter(|m| m.path().is_ident(crate::CRATE_NAME));

        while let Some(syn::Meta::List(list)) = iter.next() {
            for item in list.nested.iter().filter_map(|n| match n { syn::NestedMeta::Meta(m) => Some(m), _ => None }) {
                let path = item.path();
                if path.is_ident("crate_root") {
                    if let syn::Meta::NameValue(nv) = item {
                        crate_root = match &nv.lit {
                            syn::Lit::Str(s) => s.parse::<syn::Path>().ok(),
                            _ => None
                        };
                    }
                } else if path.is_ident("enumerated") {
                    enumerated = true;
                } else if path.is_ident("choice") {
                    choice = true;
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
            choice,
            enumerated,
            crate_root: crate_root.unwrap_or_else(|| syn::LitStr::new(crate::CRATE_NAME, proc_macro2::Span::call_site()).parse().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Class {
    Universal = 0,
    Application,
    Context,
    Private,
}

impl quote::ToTokens for Class {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        tokens.append(match self {
            Self::Universal => format_ident!("Universal"),
            Self::Application => format_ident!("Application"),
            Self::Context => format_ident!("Context"),
            Self::Private => format_ident!("Private"),
        });
    }
}

pub struct VariantConfig<'a> {
    variant: &'a syn::Variant,
    pub tag: Option<(Class, syn::Lit)>
}

impl<'a> VariantConfig<'a> {
    pub fn new(variant: &'a syn::Variant) -> Self {
        let mut tag = None;
        let mut iter = variant.attrs.iter()
            .filter_map(|a| a.parse_meta().ok())
            .filter(|m| m.path().is_ident(crate::CRATE_NAME));

        while let Some(syn::Meta::List(list)) = iter.next() {
            for item in list.nested.iter().filter_map(|n| match n { syn::NestedMeta::Meta(m) => Some(m), _ => None }) {
                let path = item.path();
                if path.is_ident("tag") {
                    match item {
                        syn::Meta::List(list) => {
                            match list.nested.iter().count() {
                                1 => {
                                    if let syn::NestedMeta::Lit(lit) = list.nested.iter().next().unwrap() {
                                        if core::matches!(lit, syn::Lit::Int(_)) {
                                            tag = Some((Class::Context, lit.clone()));
                                        }

                                    }
                                }
                                2 => {
                                    todo!()
                                }
                                _ => panic!("The `#[rasn(tag)]`attribute takes a maximum of two arguments.")
                            }
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        Self {
            variant,
            tag
        }
    }

    pub fn tag(&self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        if let Some((class, value)) = &self.tag {
            return quote!(#crate_root::Tag::new(#crate_root::tag::Class::#class, #value))
        }

        match self.variant.fields {
            syn::Fields::Unit => quote!(<()>::TAG),
            syn::Fields::Named(_) => quote!(#crate_root::Tag::SEQUENCE),
            syn::Fields::Unnamed(_) => {
                if self.variant.fields.iter().count() != 1 {
                    panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                } else {
                    let ty = &self.variant.fields.iter().next().unwrap().ty;

                    quote!(<#ty>::TAG)
                }
            }
        }
    }
}
