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
        .or_else(|| config.set.then(|| quote!(#crate_root::Tag::SET)))
        .unwrap_or(quote!(#crate_root::Tag::SEQUENCE));

    let field_groups = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| (i, FieldConfig::new(f, config)))
        .group_by(|(_, config)| config.is_option_type());

    let all_optional_tags_are_unique = field_groups
        .into_iter()
        .filter_map(|(key, fields)| key.then(|| fields))
        .map(|fields| {
            let tag_tree = fields.map(|(i, f)| f.tag_tree(i));
            quote!({
                const LIST: &'static [#crate_root::TagTree] = &[#(#tag_tree),*];
                const TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(LIST);
                #crate_root::sa::const_assert!(TAG_TREE.is_unique());
            })
        });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #impl_generics  #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::Tag = {
                #(#all_optional_tags_are_unique)*

                #tag
            };
        }
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

    let tag_tree = if config.choice {
        let field_tags = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| VariantConfig::new(v, config).tag_tree(i));

        quote! {
            {
                const VARIANT_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags)*];
                const VARIANT_TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(VARIANT_LIST);
                #crate_root::sa::const_assert!(VARIANT_TAG_TREE.is_unique());
                VARIANT_TAG_TREE
            }
        }
    } else {
        quote!(#crate_root::TagTree::Leaf(#tag))
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    proc_macro2::TokenStream::from(quote! {
        impl #impl_generics #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::Tag = {
                #tag
            };
            const TAG_TREE: #crate_root::TagTree = {
                const LIST: &'static [#crate_root::TagTree] = &[#tag_tree];
                const TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(LIST);
                #crate_root::sa::const_assert!(TAG_TREE.is_unique());
                TAG_TREE
            };
        }

    })
}
