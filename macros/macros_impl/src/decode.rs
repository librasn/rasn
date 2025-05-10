use quote::ToTokens;
use syn::Fields;

use crate::config::{map_to_inner_type, Config, FieldConfig};

#[allow(clippy::too_many_lines)]
pub fn derive_struct_impl(
    name: &syn::Ident,
    generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut list = vec![];
    let crate_root = &config.crate_root;
    let crate_root_literal = crate_root.to_token_stream().to_string();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_configs = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| FieldConfig::new(field, config, i))
        .collect::<Result<Vec<_>, _>>()?;

    let decode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;
        let field_count = field_configs.len();
        // For any field_count >= 2, use an iterator to create the appropriate number of PhantomData fields
        let phantom_data = (1..field_count)
            .map(|_| quote!(core::marker::PhantomData))
            .collect::<Vec<_>>();
        let map_quote = if field_count == 1 {
            quote!(Self)
        } else {
            quote!(|data| Self(data, #(#phantom_data),*))
        };
        let ok_quote = if field_count == 1 {
            quote!(Self(<#ty>::decode(decoder)?))
        } else {
            quote!(Self(<#ty>::decode(decoder)?, #(#phantom_data),*))
        };

        if config.tag.as_ref().is_some_and(|tag| tag.is_explicit()) {
            quote! {
                decoder.decode_explicit_prefix::<#ty>(tag).map(#map_quote)
            }
        } else {
            // NOTE: AsnType trait already implements correct delegate constraints, and those are passed here
            // We don't need to do double intersection here!
            quote! {
                match tag {
                    #crate_root::types::Tag::EOC => {
                        Ok(#ok_quote)
                    }
                    _ => {
                        <#ty as #crate_root::Decode>::decode_with_tag_and_constraints(
                            decoder,
                            tag,
                            constraints
                        ).map(#map_quote)
                    }
                }
            }
        }
    } else if config.set {
        if field_configs.is_empty() {
            return Err(syn::Error::new(
                name.span(),
                "struct without fields not allowed to be a `set`",
            ));
        }
        let field_names = field_configs
            .iter()
            .map(|config| config.field.ident.clone());
        let field_names2 = field_names.clone();
        let mut count_extended_fields: usize = 0;
        let mut count_root_fields: usize = 0;
        let required_field_names = field_configs
            .iter()
            .filter(|config| !config.is_option_type())
            .map(|config| config.field.ident.clone());

        let (field_type_names, field_type_defs): (Vec<_>, Vec<_>) = field_configs
            .iter()
            .map(|config| {
                let ty = map_to_inner_type(&config.field.ty).unwrap_or(&config.field.ty);
                let tag_attr = config.tag_derive();
                let constraints = config.constraints.attribute_tokens();
                let name = quote::format_ident!("Field{}", config.context);
                let ty = if config.extension_addition || config.extension_addition_group {
                    count_extended_fields += 1;
                    quote!(Option<#ty>)
                } else {
                    count_root_fields += 1;
                    quote!(#ty)
                };

                (
                    name.clone(),
                    quote! {
                        #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
                        #[rasn(delegate, crate_root = #crate_root_literal)]
                        #tag_attr
                        #constraints
                        pub struct #name(#ty);
                    },
                )
            })
            .unzip();

        let choice_name = quote::format_ident!("{}Fields", name);

        let choice_def = quote! {
            #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
            #[rasn(choice, crate_root = #crate_root_literal)]
            enum #choice_name {
                #(#field_type_names(#field_type_names)),*
            }
        };

        let set_init = match container.fields {
            syn::Fields::Unit => quote!(),
            syn::Fields::Unnamed(_) => quote!(( #(#field_names2),* )),
            syn::Fields::Named(_) => quote!({ #(#field_names2),* }),
        };

        let (field_const_defs, field_match_arms, field_set_arms): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(field_configs
            .iter()
            .enumerate()
            .zip(field_type_names)
            .map(|((context, config), field_name)| {
                let tag = config.tag();
                let const_name = quote::format_ident!("{}Const", field_name);
                let decode_impl = if config.extension_addition {
                    quote!(#field_name(decoder.decode_extension_addition()?))
                } else if config.extension_addition_group {
                    quote!(#field_name(decoder.decode_extension_addition_group()?))
                } else {
                    quote!(<_>::decode(decoder)?)
                };
                let ident = &config.field.ident;

                let set_field_impl = if config.extension_addition || config.extension_addition_group {
                    quote! {
                        if #ident.is_some() {
                            return Err(#crate_root::de::Error::duplicate_field(stringify!(#ident), codec))
                        } else {
                            #ident = value.0;
                        }
                    }
                } else {
                    quote! {
                        if #ident.replace(value.0).is_some() {
                            return Err(#crate_root::de::Error::duplicate_field(stringify!(#ident), codec))
                        }
                    }
                };

                (
                    quote!(const #const_name: #crate_root::types::Tag = #tag;),
                    quote!((#context, #const_name) => { #choice_name::#field_name(#decode_impl) }),
                    quote!(#choice_name::#field_name(value) => { #set_field_impl })
                )
            }));

        quote! {
            #choice_def
            let codec = decoder.codec();
            #(#field_type_defs)*

            decoder.decode_set::<#count_root_fields, #count_extended_fields, #choice_name, _, _, _>(tag, |decoder, index, tag| {
                    #(#field_const_defs)*

                    Ok(match (index, tag) {
                        #(#field_match_arms)*
                        _ => return Err(#crate_root::de::Error::unknown_field(index, tag, codec)),
                    })
                },
                |fields| {
                    #(let mut #field_names = None;)*

                    for field in fields {
                        match field {
                            #(#field_set_arms)*
                        }
                    }

                    #(let #required_field_names = #required_field_names.ok_or_else(|| #crate_root::de::Error::missing_field(stringify!(#required_field_names), codec))?;)*

                    Ok(Self #set_init)
                }
            )
        }
    } else {
        let mut all_fields_optional_or_default = true;
        let mut count_root_fields: usize = 0;
        let mut count_extended_fields: usize = 0;

        for field_config in &field_configs {
            if !field_config.is_option_or_default_type() {
                all_fields_optional_or_default = false;
            }
            if field_config.extension_addition || field_config.extension_addition_group {
                count_extended_fields += 1;
            } else {
                count_root_fields += 1;
            }

            let type_params: Vec<_> = generics
                .params
                .iter()
                .filter_map(|param| {
                    if let syn::GenericParam::Type(type_param) = param {
                        Some(type_param.ident.clone())
                    } else {
                        None
                    }
                })
                .collect();
            list.push(field_config.decode_field_def(name, &type_params)?);
        }

        let fields = match container.fields {
            Fields::Named(_) => quote!({ #(#list),* }),
            Fields::Unnamed(_) => quote!(( #(#list),* )),
            Fields::Unit => quote!(),
        };

        let initializer_fn = if all_fields_optional_or_default {
            let fields = match container.fields {
                Fields::Named(_) => {
                    let init_fields = field_configs.iter().map(|config| {
                        let default_fn = config.default_fn().unwrap_or(quote!(<_>::default));
                        let name = &config.field.ident;
                        quote!(#name : #default_fn ())
                    });

                    quote!({ #(#init_fields),* })
                }
                Fields::Unnamed(_) => {
                    let init_fields = field_configs.iter().map(|config| {
                        let default_fn = config.default_fn().unwrap_or(quote!(<_>::default));
                        quote!(#default_fn ())
                    });
                    quote!(( #(#init_fields),* ))
                }
                Fields::Unit => quote!(),
            };
            quote! {
                Some(|| {
                    Self #fields
                })
            }
        } else {
            quote!(None::<fn() -> Self>)
        };

        quote! {
            decoder.decode_sequence::<#count_root_fields, #count_extended_fields, _, _, _>(tag, #initializer_fn, |decoder| {
                Ok(Self #fields)
            })
        }
    };

    let decode_impl =
        if !config.delegate && config.tag.as_ref().is_some_and(|tag| tag.is_explicit()) {
            let tag = config.tag_for_struct(&container.fields);
            map_from_inner_type(
                tag,
                name,
                &generics,
                &container.fields,
                container.semi_token,
                None,
                config,
                true,
            )
        } else {
            decode_impl
        };

    Ok(quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag_and_constraints<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::types::Tag, constraints: #crate_root::types::Constraints) -> core::result::Result<Self, D::Error> {
                #decode_impl
            }
        }
    })
}

#[allow(clippy::too_many_arguments)]
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
            .map_or_else(|| quote!(#i), |ident| quote!(#ident));
        quote!(#name : inner.#name)
    });

    let decode_op = if is_explicit {
        quote!(decoder.decode_explicit_prefix::<#inner_name>(#tag)?)
    } else {
        quote!(<#inner_name>::decode_with_tag(decoder, #tag)?)
    };

    quote! {
        #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
        struct #inner_name #generics #fields #semi

        let inner = #decode_op;

        Ok(#outer_name { #(#map_from_inner),* })
    }
}
