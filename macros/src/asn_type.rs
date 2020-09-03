use crate::config::Config;

pub fn derive_struct_impl(
    name: syn::Ident,
    generics: syn::Generics,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl <#generics> #crate_root::AsnType for #name <#generics> {
            const TAG: #crate_root::Tag = #crate_root::Tag::SEQUENCE;
        }

        #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[]));
    })
}

pub fn derive_enum_impl(
    name: syn::Ident,
    generics: syn::Generics,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl<#generics> #crate_root::AsnType for #name <#generics> {
            const TAG: #crate_root::Tag = #crate_root::Tag::EOC;
        }
    })
}
