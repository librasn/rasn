#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Class {
    Universal = 0,
    Application,
    Context,
    Private,
}

impl Class {
    fn from_ident(ident: &syn::Ident) -> Self {
        match &*ident.to_string().to_lowercase() {
            "universal" => Class::Universal,
            "application" => Class::Application,
            "context" => Class::Context,
            "private" => Class::Private,
            s => panic!(
                "Class MUST BE `universal`, `application`, `context`, or `private`. Found: {}",
                s
            ),
        }
    }
}

impl quote::ToTokens for Class {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        tokens.append_all(match self {
            Self::Universal => quote!(rasn::types::Class::Universal),
            Self::Application => quote!(rasn::types::Class::Application),
            Self::Context => quote!(rasn::types::Class::Context),
            Self::Private => quote!(rasn::types::Class::Private),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub class: Class,
    pub value: syn::Lit,
}

impl Tag {
    pub fn from_meta(item: &syn::Meta) -> Option<Self> {
        let mut tag = None;
        match item {
            syn::Meta::List(list) => match list.nested.iter().count() {
                1 => {
                    if let syn::NestedMeta::Lit(lit) = list.nested.iter().next().unwrap() {
                        tag = Some((Class::Context, lit.clone()));
                    }
                }
                2 => {
                    let mut iter = list.nested.iter().take(2).fuse();
                    let class = iter.next().unwrap();
                    let value = iter.next().unwrap();

                    if let (syn::NestedMeta::Meta(syn::Meta::Path(path)), syn::NestedMeta::Lit(value)) =
                        (class, value)
                    {
                        tag = Some((Class::from_ident(path.get_ident().unwrap()), value.clone()));
                    }
                }
                _ => panic!("The `#[rasn(tag)]`attribute takes a maximum of two arguments."),
            },
            _ => panic!("The `#[rasn(tag)]`attribute must be a list."),
        }

        tag.map(|(class, value)| Self { class, value })
    }

    pub fn from_fields(fields: &syn::Fields, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        match fields {
            syn::Fields::Unit => quote!(<() as #crate_root::AsnType>::TAG),
            syn::Fields::Named(_) => quote!(#crate_root::Tag::SEQUENCE),
            syn::Fields::Unnamed(_) => {
                if fields.iter().count() != 1 {
                    panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                } else {
                    let ty = &fields.iter().next().unwrap().ty;

                    quote!(<#ty as #crate_root::AsnType>::TAG)
                }
            }
        }
    }

    pub fn to_tokens(&self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        let class = &self.class;
        let value = &self.value;

        quote!(#crate_root::Tag::new(#class, #value))
    }
}
