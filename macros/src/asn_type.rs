use itertools::Itertools;

use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    generics: syn::Generics,
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
        .into_iter()
        .map(|(i, field)| {
            let metadata = field.to_field_metadata(i);
            quote!(#metadata)
        })
        .collect::<Vec<_>>();

    let all_optional_tags_are_unique: Vec<_> = field_groups
        .group_by(|(_, config)| config.is_option_or_default_type())
        .into_iter()
        .filter_map(|(key, fields)| key.then(|| fields))
        .map(|fields| {
            let tag_tree = fields.map(|(i, f)| f.tag_tree(i));
            let error_message = format!(
                "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                name
            );

            quote!({
                const LIST: &'static [#crate_root::TagTree] = &[#(#tag_tree),*];
                const TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(LIST);
                const _: () = assert!(TAG_TREE.is_unique(), #error_message);
            })
        })
        .collect::<Vec<_>>();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let constructed_impl = (!config.delegate).then(|| {
        quote! {
            #[automatically_derived]
            impl #impl_generics  #crate_root::types::Constructed for #name #ty_generics #where_clause {
                const FIELDS: #crate_root::types::fields::Fields = #crate_root::types::fields::Fields::from_static(&[
                    #(#field_metadata),*
                ]);
            }
        }
    });

    let constraints_def = config.constraints.const_static_def(&crate_root);

    quote! {
        #constructed_impl

        #[automatically_derived]
        impl #impl_generics  #crate_root::AsnType for #name #ty_generics #where_clause {
            const TAG: #crate_root::Tag = {
                #(#all_optional_tags_are_unique)*

                #tag
            };

            #constraints_def
        }
    }
}
