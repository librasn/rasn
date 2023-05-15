use syn::Fields;

use crate::{config::*, ext::GenericsExt};

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut list = vec![];
    let crate_root = &config.crate_root;
    generics.add_trait_bounds(crate_root, quote::format_ident!("Decode"));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let decode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;

        if config
            .tag
            .as_ref()
            .map(|tag| tag.is_explicit())
            .unwrap_or_default()
        {
            quote! {
                decoder.decode_explicit_prefix::<#ty>(tag).map(Self)
            }
        } else {
            quote! {
                <#ty as #crate_root::Decode>::decode_with_tag_and_constraints(decoder, tag, constraints).map(Self)
            }
        }
    } else if config.set {
        let field_names = container.fields.iter().map(|field| field.ident.clone());
        let field_names2 = field_names.clone();
        let field_names3 = field_names.clone();
        let required_field_names = container
            .fields
            .iter()
            .filter(|field| !FieldConfig::new(field, config).is_option_type())
            .map(|field| field.ident.clone());
        let (field_type_names, field_type_defs): (Vec<_>, Vec<_>) = container
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let ty = config
                    .option_type
                    .map_to_inner_type(&field.ty)
                    .unwrap_or(&field.ty);
                let tag_attr = FieldConfig::new(field, config).tag_derive(i);
                let name = quote::format_ident!("Field{}", i);

                (
                    name.clone(),
                    quote! {
                        #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
                        #[rasn(delegate)]
                        #tag_attr
                        pub struct #name(#ty);
                    },
                )
            })
            .unzip();

        let choice_name = quote::format_ident!("{}Fields", name);

        let choice_def = quote! {
            #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
            #[rasn(choice)]
            enum #choice_name {
                #(#field_type_names(#field_type_names)),*
            }
        };

        let set_init = match container.fields {
            syn::Fields::Unit => quote!(),
            syn::Fields::Unnamed(_) => quote!(( #(#field_names3),* )),
            syn::Fields::Named(_) => quote!({ #(#field_names3),* }),
        };

        let (field_const_defs, field_match_arms): (Vec<_>, Vec<_>) = container.fields
            .iter()
            .enumerate()
            .zip(field_type_names.clone())
            .map(|((context, field), field_name)| {
                let config = FieldConfig::new(field, config);
                let tag = config.tag(context);
                let const_name = quote::format_ident!("{}Const", field_name);
                (
                    quote!(const #const_name: Tag = #tag;),
                    quote!((#context, #const_name) => { #choice_name::#field_name(<_>::decode(decoder)?) }),
                )
            })
            .unzip();

        quote! {
            #choice_def
            #(#field_type_defs)*

            decoder.decode_set::<#choice_name, _, _, _>(tag, |decoder, index, tag| {
                    #(#field_const_defs)*

                    Ok(match (index, tag) {
                        #(#field_match_arms)*
                        _ => return Err(#crate_root::de::Error::custom("Unknown field provided.")),
                    })
                },
                |fields| {
                    #(let mut #field_names = None;)*

                    for field in fields {
                        match field {
                            #(#choice_name::#field_type_names(value) => {
                                if #field_names2.replace(value.0).is_some() {
                                    return Err(rasn::de::Error::duplicate_field(stringify!(#field_names2)))
                                }
                            })*
                        }
                    }

                    #(let #required_field_names = #required_field_names.ok_or_else(|| #crate_root::de::Error::missing_field(stringify!(#required_field_names)))?;)*

                    Ok(Self #set_init)
                }
            )
        }
    } else {
        for (i, field) in container.fields.iter().enumerate() {
            let field_config = FieldConfig::new(field, config);

            list.push(field_config.decode_field_def(&name, i));
        }

        let fields = match container.fields {
            Fields::Named(_) => quote!({ #(#list),* }),
            Fields::Unnamed(_) => quote!(( #(#list),* )),
            Fields::Unit => quote!(),
        };

        quote! {
            decoder.decode_sequence(tag, |decoder| {
                Ok(Self #fields)
            })
        }
    };

    let decode_impl =
        if !config.delegate && config.tag.as_ref().map_or(false, |tag| tag.is_explicit()) {
            let tag = config.tag_for_struct(&container.fields);
            map_from_inner_type(
                tag,
                &name,
                &generics,
                &container.fields,
                container.semi_token,
                None,
                &config,
                true,
            )
        } else {
            decode_impl
        };

    quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag_and_constraints<'constraints, D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag, constraints: #crate_root::types::Constraints<'constraints>) -> core::result::Result<Self, D::Error> {
                #decode_impl
            }
        }
    }
}

pub fn map_from_inner_type(
    tag: proc_macro2::TokenStream,
    name: &syn::Ident,
    generics: &syn::Generics,
    fields: &Fields,
    semi: Option<syn::Token![;]>,
    outer_name: Option<proc_macro2::TokenStream>,
    config: &Config,
    is_explicit: bool,
) -> proc_macro2::TokenStream {
    let inner_name = quote::format_ident!("Inner{}", name);
    let crate_root = &config.crate_root;
    let outer_name = outer_name.unwrap_or(quote!(Self));

    let map_from_inner = fields.iter().enumerate().map(|(i, field)| {
        let name = field
            .ident
            .as_ref()
            .map(|ident| quote!(#ident))
            .unwrap_or_else(|| quote!(#i));
        quote!(#name : inner.#name)
    });

    let decode_fields = fields.iter().enumerate().map(|(i, field)| {
        let field_config = FieldConfig::new(field, config);
        field_config.decode_field_def(name, i)
    });

    let decode_op = if is_explicit {
        quote!(decoder.decode_explicit_prefix::<#inner_name>(#tag)?)
    } else {
        quote!(<#inner_name>::decode_with_tag(decoder, #tag)?)
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        #[derive(#crate_root::AsnType)]
        struct #inner_name #generics #fields #semi

        impl #impl_generics #crate_root::Decode for #inner_name #ty_generics #where_clause {
            fn decode_with_tag_and_constraints<'constraints, D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag, _: #crate_root::types::Constraints<'constraints>) -> core::result::Result<Self, D::Error> {
                decoder.decode_sequence(tag, |decoder| {
                    Ok::<_, D::Error>(#inner_name { #(#decode_fields),* })
                })
            }
        }

        let inner = #decode_op;

        Ok(#outer_name { #(#map_from_inner),* })
    }
}
