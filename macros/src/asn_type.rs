use itertools::Itertools;

use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    let tag = config.tag_for_struct(&container.fields);
    let field_groups = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| (i, FieldConfig::new(f, config)));

    let field_metadata = field_groups
        .clone()
        .filter(|(_, field)| field.is_not_extension())
        .map(|(i, field)| {
            let metadata = field.to_field_metadata(i);
            quote!(#metadata)
        })
        .collect::<Vec<_>>();

    let extension_metadata = field_groups
        .clone()
        .filter(|(_, field)| field.is_extension())
        .map(|(i, field)| {
            let metadata = field.to_field_metadata(i);
            quote!(#metadata)
        })
        .collect::<Vec<_>>();

    let all_optional_tags_are_unique: Vec<_> = field_groups
        .chunk_by(|(_, config)| config.is_option_or_default_type())
        .into_iter()
        .filter_map(|(key, fields)| key.then_some(fields))
        .map(|fields| {
            let tag_tree = fields.map(|(i, f)| f.tag_tree(i));
            let error_message = format!(
                "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                name
            );

            quote!({
                const LIST: &'static [#crate_root::types::TagTree] = &[#(#tag_tree),*];
                const TAG_TREE: #crate_root::types::TagTree = #crate_root::types::TagTree::Choice(LIST);
                const _: () = assert!(TAG_TREE.is_unique(), #error_message);
            })
        })
        .collect::<Vec<_>>();

    for param in generics.type_params_mut() {
        param
            .bounds
            .push(syn::TypeParamBound::Trait(syn::TraitBound {
                paren_token: Some(<_>::default()),
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: syn::parse_str("rasn::AsnType").unwrap(),
            }));
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let extended_fields_def = if config.constraints.extensible {
        let fields = (!extension_metadata.is_empty())
            .then_some(quote! {
                #crate_root::types::fields::Fields::from_static(&[#(#extension_metadata),*])
            })
            .unwrap_or(quote!(#crate_root::types::fields::Fields::empty()));

        quote!(Some(#fields))
    } else {
        quote!(None)
    };

    let constructed_impl = (!config.delegate).then(|| {
        quote! {
            #[automatically_derived]
            impl #impl_generics  #crate_root::types::Constructed for #name #ty_generics #where_clause {
                const FIELDS: #crate_root::types::fields::Fields = #crate_root::types::fields::Fields::from_static(&[
                    #(#field_metadata),*
                ]);
                const EXTENDED_FIELDS: Option<#crate_root::types::fields::Fields> = #extended_fields_def;
            }
        }
    });

    let constraints_def = config.constraints.const_static_def(crate_root);

    let alt_identifier = config.identifier.as_ref().map_or(
        quote!(),
        |id| quote!(const IDENTIFIER: Option<&'static str> = Some(#id);),
    );

    quote! {
        #constructed_impl

        #[automatically_derived]
        impl #impl_generics  #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::types::Tag = {
                #(#all_optional_tags_are_unique)*

                #tag
            };
            #alt_identifier
            #constraints_def
        }
    }
}
