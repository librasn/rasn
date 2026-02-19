use syn::spanned::Spanned;
use syn::{Token, parenthesized};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Class {
    Universal = 0,
    Application,
    Context,
    Private,
}

impl Class {
    fn from_ident(ident: &syn::Ident) -> syn::Result<Self> {
        Ok(match &*ident.to_string().to_lowercase() {
            "universal" => Class::Universal,
            "application" => Class::Application,
            "context" => Class::Context,
            "private" => Class::Private,
            s => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!(
                        "Class MUST BE `universal`, `application`, `context`, or `private`. Found: {s}",
                    ),
                ));
            }
        })
    }

    pub fn to_ident(self) -> syn::Ident {
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

impl Class {
    pub fn as_tokens(self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        match self {
            Self::Universal => quote!(#crate_root::types::Class::Universal),
            Self::Application => quote!(#crate_root::types::Class::Application),
            Self::Context => quote!(#crate_root::types::Class::Context),
            Self::Private => quote!(#crate_root::types::Class::Private),
        }
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
    },
}

impl Tag {
    pub fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Self> {
        let tag;
        let mut explicit = false;
        let content;
        parenthesized!(content in meta.input);
        if content.peek(syn::Lit) {
            tag = (Class::Context, content.parse::<syn::Lit>()?);
        } else if content.peek(syn::Ident) {
            let ident: syn::Ident = content.parse()?;
            if content.peek(syn::token::Paren) {
                if ident != syn::Ident::new("explicit", proc_macro2::Span::call_site()) {
                    return Err(syn::Error::new(
                        ident.span(),
                        "Invalid attribute literal provided to `rasn`, expected `rasn(tag(explicit(...)))`.",
                    ));
                }
                let explicit_content;
                parenthesized!(explicit_content in &content);
                if explicit_content.peek(syn::Ident)
                    && explicit_content.peek2(Token![,])
                    && explicit_content.peek3(syn::Lit)
                {
                    let ident = explicit_content.parse()?;

                    let _ = explicit_content.parse::<Token![,]>()?;
                    let lit: syn::Lit = explicit_content.parse()?;

                    let class = Class::from_ident(&ident)?;
                    explicit = true;
                    tag = (class, lit);
                } else if explicit_content.peek(syn::Lit) {
                    let lit = explicit_content.parse()?;
                    explicit = true;
                    tag = (Class::Context, lit);
                } else {
                    return Err(explicit_content.error("Expected meta items inside `explicit`"));
                }
                if !explicit_content.is_empty() {
                    return Err(explicit_content
                        .error("The `#[rasn(tag)]`attribute takes a maximum of two arguments."));
                }
            } else if content.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
                let lit: syn::Lit = content.parse()?;
                let class = Class::from_ident(&ident)?;
                tag = (class, lit);
            } else {
                return Err(content.error("The `#[rasn(tag)]`attribute must be a list."));
            }
        } else {
            return Err(content.error("The `#[rasn(tag)]`attribute must be a list."));
        }
        if !content.is_empty() {
            return Err(
                content.error("The `#[rasn(tag)]`attribute takes a maximum of two arguments.")
            );
        }

        Ok(Self::Value {
            class: tag.0,
            value: tag.1,
            explicit,
        })
    }

    pub fn from_fields(fields: &syn::Fields) -> syn::Result<Self> {
        Ok(match fields {
            syn::Fields::Unit => Self::Delegate {
                ty: syn::TypeTuple {
                    paren_token: <_>::default(),
                    elems: <_>::default(),
                }
                .into(),
            },
            syn::Fields::Named(_) => Self::SEQUENCE(),
            syn::Fields::Unnamed(_) => {
                if fields.iter().count() == 1 {
                    let ty = fields.iter().next().cloned().unwrap().ty;

                    Self::Delegate { ty }
                } else {
                    return Err(syn::Error::new(
                        fields.span(),
                        "Unnamed fields are not supported.",
                    ));
                }
            }
        })
    }

    pub fn is_explicit(&self) -> bool {
        match self {
            Self::Value { explicit, .. } => *explicit,
            Self::Delegate { .. } => false,
        }
    }

    pub fn to_tokens(&self, crate_root: &syn::Path) -> proc_macro2::TokenStream {
        match self {
            Self::Value { class, value, .. } => {
                let cls = class.as_tokens(crate_root);
                quote!(#crate_root::types::Tag::new(#cls, #value))
            }
            Self::Delegate { ty } => quote!(<#ty as #crate_root::AsnType>::TAG),
        }
    }

    pub fn to_attribute_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Value {
                class,
                value,
                explicit,
            } => {
                let class = class.to_ident();
                let mut attribute = quote!(#class, #value);

                if *explicit {
                    attribute = quote!(explicit(#attribute));
                }
                quote!(#[rasn(tag(#attribute))])
            }
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
