use itertools::Itertools;

use crate::config::*;

pub fn derive_struct_impl(
    name: &syn::Ident,
    generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> syn::Result<proc_macro2::TokenStream> {
    let name_literal = name.to_string();
    let crate_root = &config.crate_root;
    let tag = config.tag_for_struct(&container.fields);
    let field_configs = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| FieldConfig::new(f, config, i))
        .collect::<Result<Vec<_>, _>>()?;

    let field_metadata = field_configs
        .iter()
        .filter(|field| field.is_not_extension())
        .map(|field| {
            let metadata = field.to_field_metadata();
            quote!(#metadata)
        })
        .collect::<Vec<_>>();

    let extension_metadata = field_configs
        .iter()
        .filter(|field| field.is_extension())
        .map(|field| {
            let metadata = field.to_field_metadata();
            quote!(#metadata)
        })
        .collect::<Vec<_>>();
    // TODO use partition above?

    let all_optional_tags_are_unique: Vec<_> = field_configs
        .iter()
        .chunk_by(|config| config.is_option_or_default_type())
        .into_iter()
        .filter_map(|(key, fields)| key.then_some(fields))
        .map(|fields| {
            let error_message = format!(
                "{}'s fields is not a valid \
                        order of ASN.1 tags, ensure that your field's tags and \
                        OPTIONALs are correct.",
                name
            );

            let tag_tree = fields.map(|f| f.tag_tree()).collect::<Vec<_>>();

            Ok::<_, syn::Error>(quote!({
                const LIST: &'static [#crate_root::types::TagTree] = &[#(#tag_tree),*];
                const TAG_TREE: #crate_root::types::TagTree = #crate_root::types::TagTree::Choice(LIST);
                const _: () = assert!(TAG_TREE.is_unique(), #error_message);
            }))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let extended_fields_def = if config.constraints.extensible {
        (!extension_metadata.is_empty())
            .then_some(quote! {
                Some(#crate_root::types::fields::Fields::from_static([#(#extension_metadata),*]))
            })
            .unwrap_or(quote!(None))
    } else {
        quote!(None)
    };
    let root_field_count = field_metadata.len();
    let extension_field_count = extension_metadata.len();
    let extensible = config.constraints.extensible;

    let constructed_impl = (!config.delegate).then(|| {
        quote! {
            #[automatically_derived]
            impl #impl_generics  #crate_root::types::Constructed<#root_field_count, #extension_field_count> for #name #ty_generics #where_clause {
                const FIELDS: #crate_root::types::fields::Fields<#root_field_count> = #crate_root::types::fields::Fields::from_static([
                    #(#field_metadata),*
                ]);
                const IS_EXTENSIBLE: bool = #extensible;
                const EXTENDED_FIELDS: Option<#crate_root::types::fields::Fields<#extension_field_count>> = #extended_fields_def;
            }
        }
    });
    let constraints = config
        .constraints
        .const_expr(crate_root)
        .unwrap_or_else(|| quote!(#crate_root::types::Constraints::default()));
    // Intersect the constraints of the wrapped type with the constraints of the container
    let constraints_def = if config.delegate {
        let inner_type = match container.fields {
            syn::Fields::Unnamed(ref fields) => &fields.unnamed.first().unwrap().ty,
            _ => {
                return Err(syn::Error::new_spanned(
                    &container.fields,
                    "Delegate is only supported for newtype pattern",
                ));
            }
        };
        Some(quote! {
            const CONSTRAINTS: #crate_root::types::Constraints =<#inner_type as #crate_root::AsnType>::CONSTRAINTS.intersect(#constraints);
        })
    } else {
        config.constraints.const_static_def(crate_root)
    };

    let alt_identifier = config.identifier.as_ref().map_or(
        quote!(const IDENTIFIER: #crate_root::types::Identifier = #crate_root::types::Identifier(Some(#name_literal));),
        |id| quote!(const IDENTIFIER: #crate_root::types::Identifier = #crate_root::types::Identifier(Some(#id));),
    );

    Ok(quote! {
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
    })
}
