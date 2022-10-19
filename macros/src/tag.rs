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

    pub fn to_ident(&self) -> syn::Ident {
        quote::format_ident!(
            "{}",
            match self {
                Self::Universal => "universal",
                Self::Application => "application",
                Self::Context => "context",
                Self::Private => "private",
            }
        )
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
pub enum Tag {
    Value {
        class: Class,
        value: syn::Lit,
        explicit: bool,
    },
    Delegate {
        ty: syn::Type,
    }
}

impl Tag {
    pub fn from_meta(item: &syn::Meta) -> Option<Self> {
        let mut tag = None;
        let mut explicit = false;
        match item {
            syn::Meta::List(list) => match list.nested.iter().count() {
                1 => match &list.nested[0] {
                    syn::NestedMeta::Lit(lit) => tag = Some((Class::Context, lit.clone())),
                    syn::NestedMeta::Meta(meta) => {
                        let list = match meta {
                                syn::Meta::List(list) => list,
                                _ => panic!("Invalid attribute literal provided to `rasn`, expected `rasn(tag(explicit(...)))`."),
                            };

                        if !list
                            .path
                            .is_ident(&syn::Ident::new("explicit", proc_macro2::Span::call_site()))
                        {
                            panic!("Invalid attribute literal provided to `rasn`, expected `rasn(tag(explicit(...)))`.");
                        }

                        let mut iter = list.nested.iter();
                        let first = iter.next();
                        let second = iter.next();

                        match (first, second) {
                            (
                                Some(syn::NestedMeta::Meta(syn::Meta::Path(path))),
                                Some(syn::NestedMeta::Lit(lit)),
                            ) => {
                                let class = Class::from_ident(
                                    path.get_ident().expect("Path must be a valid ident."),
                                );
                                let value = lit.clone();
                                explicit = true;
                                tag = Some((class, value));
                            }
                            (Some(syn::NestedMeta::Lit(lit)), None) => {
                                explicit = true;
                                tag = Some((Class::Context, lit.clone()));
                            }
                            _ => panic!("Expected meta items inside `explicit`"),
                        }
                    }
                },
                2 => {
                    let mut iter = list.nested.iter().take(2).fuse();
                    let class = iter.next().unwrap();
                    let value = iter.next().unwrap();

                    if let (
                        syn::NestedMeta::Meta(syn::Meta::Path(path)),
                        syn::NestedMeta::Lit(value),
                    ) = (class, value)
                    {
                        tag = Some((Class::from_ident(path.get_ident().unwrap()), value.clone()));
                    }
                }
                _ => panic!("The `#[rasn(tag)]`attribute takes a maximum of two arguments."),
            },
            _ => panic!("The `#[rasn(tag)]`attribute must be a list."),
        }

        tag.map(|(class, value)| Self::Value {
            class,
            value,
            explicit,
        })
    }

    pub fn from_fields(fields: &syn::Fields) -> Self {
        match fields {
            syn::Fields::Unit => Self::Delegate { ty: syn::TypeTuple { paren_token: <_>::default(), elems: <_>::default() }.into() },
            syn::Fields::Named(_) => Self::SEQUENCE(),
            syn::Fields::Unnamed(_) => {
                if fields.iter().count() != 1 {
                    panic!("Tuple-style enum variants must contain only a single field, switch to struct-style variants for multiple fields.");
                } else {
                    let ty = fields.iter().cloned().next().unwrap().ty;

                    Self::Delegate { ty }
                }
            }
        }
    }

    pub fn is_explicit(&self) -> bool {
        match self {
            Self::Value { explicit, .. } => *explicit,
            _ => false,
        }
    }

    pub fn to_tokens(&self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        match self {
            Self::Value { class, value, .. } => quote!(#crate_root::Tag::new(#class, #value)),
            Self::Delegate { ty } => quote!(<#ty as #crate_root::AsnType>::TAG),
        }
    }

    pub fn to_attribute_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Value { class, value, explicit } => {
                let class = class.to_ident();
                let mut attribute = quote!(#class, #value);

                if *explicit {
                    attribute = quote!(explicit(#attribute));
                }
                quote!(#[rasn(tag(#attribute))])
            },
            Self::Delegate { .. } => quote!(#[rasn(delegate)]),
        }

    }
}

macro_rules! consts {
    ($($name:ident = $value:expr),+) => {
        #[allow(missing_docs)]
        impl Tag {
            $(
                #[allow(dead_code, non_snake_case)]
                pub fn $name() -> Tag {
                    Self::Value {
                        class: Class::Universal,
                        value: syn::LitInt::new(stringify!($value), proc_macro2::Span::call_site()).into(),
                        explicit: false,
                    }
                }
            )+
        }
    }
}

consts! {
    EOC = 0,
    BOOL = 1,
    INTEGER = 2,
    BIT_STRING = 3,
    OCTET_STRING = 4,
    NULL = 5,
    OBJECT_IDENTIFIER = 6,
    OBJECT_DESCRIPTOR = 7,
    EXTERNAL = 8,
    REAL = 9,
    ENUMERATED = 10,
    EMBEDDED_PDV = 11,
    UTF8_STRING = 12,
    RELATIVE_OID = 13,
    SEQUENCE = 16,
    SET = 17,
    NUMERIC_STRING = 18,
    PRINTABLE_STRING = 19,
    TELETEX_STRING = 20,
    VIDEOTEX_STRING = 21,
    IA5_STRING = 22,
    UTC_TIME = 23,
    GENERALIZED_TIME = 24,
    GRAPHIC_STRING = 25,
    VISIBLE_STRING = 26,
    GENERAL_STRING = 27,
    UNIVERSAL_STRING = 28,
    CHARACTER_STRING = 29,
    BMP_STRING = 30
}

