use syn::Fields;

use crate::{config::*, ext::GenericsExt};
#[allow(clippy::too_many_lines)]

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
                match tag {
                    #crate_root::Tag::EOC => {
                        Ok(Self(<#ty>::decode(decoder)?))
                    }
                    _ => {
                        <#ty as #crate_root::Decode>::decode_with_tag_and_constraints(
                            decoder,
                            tag,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(constraints),
                        ).map(Self)
                    }
                }
            }
        }
    } else if config.set {
        let field_names = container.fields.iter().map(|field| field.ident.clone());
        let field_names2 = field_names.clone();
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
                let config = FieldConfig::new(field, config);
                let tag_attr = config.tag_derive(i);
                let constraints = config.constraints.attribute_tokens();
                let name = quote::format_ident!("Field{}", i);
                let ty = if config.extension_addition || config.extension_addition_group {
                    quote!(Option<#ty>)
                } else {
                    quote!(#ty)
                };

                (
                    name.clone(),
                    quote! {
                        #[derive(#crate_root::AsnType, #crate_root::Decode, #crate_root::Encode)]
                        #[rasn(delegate)]
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
            #[rasn(choice)]
            enum #choice_name {
                #(#field_type_names(#field_type_names)),*
            }
        };

        let set_init = match container.fields {
            syn::Fields::Unit => quote!(),
            syn::Fields::Unnamed(_) => quote!(( #(#field_names2),* )),
            syn::Fields::Named(_) => quote!({ #(#field_names2),* }),
        };

        let (field_const_defs, field_match_arms, field_set_arms): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(container.fields
            .iter()
            .enumerate()
            .zip(field_type_names)
            .map(|((context, field), field_name)| {
                let config = FieldConfig::new(field, config);
                let tag = config.tag(context);
                let const_name = quote::format_ident!("{}Const", field_name);
                let decode_impl = if config.extension_addition {
                    quote!(#field_name(decoder.decode_extension_addition()?))
                } else if config.extension_addition_group {
                    quote!(#field_name(decoder.decode_extension_addition_group()?))
                } else {
                    quote!(<_>::decode(decoder)?)
                };
                let ident = &field.ident;

                let set_field_impl = if config.extension_addition || config.extension_addition_group {
                    quote! {
                        if #ident.is_some() {
                            return Err(rasn::de::Error::duplicate_field(stringify!(#ident), codec))
                        } else {
                            #ident = value.0;
                        }
                    }
                } else {
                    quote! {
                        if #ident.replace(value.0).is_some() {
                            return Err(rasn::de::Error::duplicate_field(stringify!(#ident), codec))
                        }
                    }
                };

                (
                    quote!(const #const_name: #crate_root::Tag = #tag;),
                    quote!((#context, #const_name) => { #choice_name::#field_name(#decode_impl) }),
                    quote!(#choice_name::#field_name(value) => { #set_field_impl })
                )
            }));

        quote! {
            #choice_def
            let codec = decoder.codec();
            #(#field_type_defs)*

            decoder.decode_set::<#choice_name, _, _, _>(tag, |decoder, index, tag| {
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
        for (i, field) in container.fields.iter().enumerate() {
            let field_config = FieldConfig::new(field, config);

            if !field_config.is_option_or_default_type() {
                all_fields_optional_or_default = false;
            }

            list.push(field_config.decode_field_def(&name, i));
        }

        let fields = match container.fields {
            Fields::Named(_) => quote!({ #(#list),* }),
            Fields::Unnamed(_) => quote!(( #(#list),* )),
            Fields::Unit => quote!(),
        };

        let initializer_fn = if all_fields_optional_or_default {
            let fields = match container.fields {
                Fields::Named(_) => {
                    let init_fields = container.fields.iter().map(|field| {
                        let default_fn = FieldConfig::new(field, config)
                            .default_fn()
                            .unwrap_or(quote!(<_>::default));
                        let name = &field.ident;
                        quote!(#name : #default_fn ())
                    });

                    quote!({ #(#init_fields),* })
                }
                Fields::Unnamed(_) => {
                    let init_fields = container.fields.iter().map(|field| {
                        let default_fn = FieldConfig::new(field, config)
                            .default_fn()
                            .unwrap_or(quote!(<_>::default));
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
            decoder.decode_sequence(tag, #initializer_fn, |decoder| {
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
                config,
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
            .map(|ident| quote!(#ident))
            .unwrap_or_else(|| quote!(#i));
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
