use itertools::Itertools;

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
        .or(config.delegate.then(|| {
            let ty = &container.fields.iter().next().unwrap().ty;

            quote!(<#ty as #crate_root::AsnType>::TAG)
        }))
        .unwrap_or(quote!(#crate_root::Tag::SEQUENCE));

    let field_groups = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| (i, FieldConfig::new(f, config)))
        .group_by(|(_, config)| config.field.ty == config.container_config.option_type.path);
    let field_asserts = field_groups.into_iter().filter_map(|(key, fields)| key.then(|| fields)).map(|fields| {
        let fields = fields.map(|(i, f)| f.tag(i));
        quote!(#crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#fields),*]));)
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #impl_generics  #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::Tag = #tag;
        }

        const _: () = {
            #(#field_asserts)*;
        };
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
        .unwrap_or_else(|| {
            config
                .enumerated
                .then(|| quote!(#crate_root::Tag::ENUMERATED))
                .unwrap_or(quote!(#crate_root::Tag::EOC))
        });

    let variant_tags = container
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| VariantConfig::new(v, config).tag(i));

    let asserts = if config.choice {
        let field_asserts = container.variants.iter().map(|v| {
            if v.fields.len() > 1 {
                let field_tags = v.fields.iter().enumerate().map(|(i, f)| FieldConfig::new(f, config).tag(i));

                quote! {
                    #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#field_tags),*]));
                }
            } else {
                quote!()
            }
        });

        Some(quote! {
            #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#variant_tags),*]));

            #(#field_asserts)*
        })
    } else {
        None
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let choice_const = if config.choice {
        quote!(
            const CHOICE: bool = true;
        )
    } else {
        quote!()
    };

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #impl_generics #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::Tag = {
                const _: () = { #asserts };

                #tag
            };
            #choice_const
        }

    })
}
