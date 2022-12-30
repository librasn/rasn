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

    let field_metadata = field_groups.clone()
        .into_iter()
        .map(|(i, field)| {
            let metadata = field.to_field_metadata(i);
            quote!(#metadata)
        }).collect::<Vec<_>>();

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

    let size = config.size.as_ref().map(|size| {
        let extensible = size.extensible;

        match size.constraint {
            Value::Single(value) => quote!{
                #crate_root::types::Constraint::Size(
                    #crate_root::types::constraints::Size::new(
                        #crate_root::types::constraints::Range::single_value(#value as usize)
                    )
                    .into()
                    .set_extensible(#extensible)
                ),
            },
            Value::Range(start, end) => {
                let range = match (start, end) {
                    (Some(start), Some(end)) => quote! {
                        #crate_root::types::constraints::Range::const_new(#start as usize, #end as usize)
                    },
                    (Some(start), None) => quote! {
                        #crate_root::types::constraints::Range::start_from(#start as usize)
                    },
                    (None, Some(end)) => quote! {
                        #crate_root::types::constraints::Range::up_to(#end as usize)
                    },
                    (None, None) => return quote!(),
                };

                quote!(
                    #crate_root::types::Constraint::Size(
                        #crate_root::types::constraints::Size::new(#range)
                        .into()
                        .set_extensible(#extensible)
                    ),
                )
            }
        }
    });

    let from_constraint = config.from.as_ref().map(|from| {
        let extensible = from.extensible;
        let mut set = from.constraint.iter().flat_map(|from| match from {
            StringValue::Single(value) => vec![*value],
            StringValue::Range(start, end) => (*start..*end).into_iter().collect(),
        }).collect::<Vec<u32>>();
        set.sort();
        set.dedup();

        quote!(
            #crate_root::types::constraints::Constraint::PermittedAlphabet(
                #crate_root::types::constraints::PermittedAlphabet::new(&[#(#set,)*])
                    .into()
                    .set_extensible(#extensible)
            ),
        )
    });

    let constraints_def = (size.is_some() || from_constraint.is_some()).then(|| quote! {
        const CONSTRAINTS: #crate_root::types::Constraints<'static> = #crate_root::types::Constraints::new(&[
            #size
            #from_constraint
        ]);
    });

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
