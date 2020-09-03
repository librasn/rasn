use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    let tag = config
        .tag
        .as_ref()
        .map(|t| t.to_tokens(crate_root))
        .unwrap_or(quote!(#crate_root::Tag::SEQUENCE));

    let field_tags = container
        .fields
        .iter()
        .map(|f| FieldConfig::new(f, config).tag());

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl <#generics> #crate_root::AsnType for #name <#generics> {
            const TAG: #crate_root::Tag = #tag;
        }

        #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#field_tags),*]));
    })
}

pub fn derive_enum_impl(
    name: syn::Ident,
    generics: syn::Generics,
    container: syn::DataEnum,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    let tag = config
        .tag
        .as_ref()
        .map(|t| t.to_tokens(crate_root))
        .unwrap_or(quote!(#crate_root::Tag::EOC));

    let variant_tags = container
        .variants
        .iter()
        .map(|v| VariantConfig::new(v, config).tag());

    let asserts = if config.choice {
        let field_asserts = container.variants.iter().map(|v| {
            let field_tags = v.fields.iter().map(|f| FieldConfig::new(f, config).tag());
            quote! {
                #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#field_tags),*]));
            }
        });

        Some(quote! {
            #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#variant_tags),*]));

            #(#field_asserts)*
        })
    } else {
        None
    };

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl<#generics> #crate_root::AsnType for #name <#generics> {
            const TAG: #crate_root::Tag = #tag;
        }

        #asserts
    })
}
