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
