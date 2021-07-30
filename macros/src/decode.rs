use syn::Fields;

use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut list = vec![];
    let crate_root = &config.crate_root;

    for (i, field) in container.fields.iter().enumerate() {
        let lhs = field.ident.as_ref().map(|i| quote!(#i :));
        let field_config = FieldConfig::new(field, config);

        let or_else = match field_config.default {
            Some(Some(ref path)) => quote! { .unwrap_or_else(#path) },
            Some(None) => quote! { .unwrap_or_default() },
            None => quote!(?),
        };

        if field_config.choice {
            list.push(proc_macro2::TokenStream::from(
                quote!(#lhs <_>::decode(decoder) #or_else),
            ));
        } else {
            let tag = field_config.tag(i);
            list.push(proc_macro2::TokenStream::from(
                quote!(#lhs <_>::decode_with_tag(decoder, #tag) #or_else),
            ));
        }
    }

    let fields = match container.fields {
        Fields::Named(_) => quote!({ #(#list),* }),
        Fields::Unnamed(_) => quote!(( #(#list),* )),
        Fields::Unit => quote!(),
    };

    for param in generics.type_params_mut() {
        param.colon_token = Some(Default::default());
        param.bounds = {
            let mut punct = syn::punctuated::Punctuated::new();
            punct.push(
                syn::TraitBound {
                    paren_token: None,
                    modifier: syn::TraitBoundModifier::None,
                    lifetimes: None,
                    path: {
                        let mut path = crate_root.clone();
                        path.segments.push(syn::PathSegment {
                            ident: quote::format_ident!("Decode"),
                            arguments: syn::PathArguments::None,
                        });

                        path
                    },
                }
                .into(),
            );

            punct
        };
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let decode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;
        quote! {
            <#ty as #crate_root::Decode>::decode_with_tag(decoder, tag).map(Self)
        }
    } else {
        quote! {
            decoder.decode_sequence(tag, |decoder| {
                Ok(Self #fields)
            })
        }
    };

    proc_macro2::TokenStream::from(quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> Result<Self, D::Error> {
                #decode_impl
            }
        }
    })
}

pub fn derive_enum_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataEnum,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    let decode_with_tag = if config.enumerated {
        let variants = container.variants.iter().map(|v| {
            let ident = &v.ident;
            quote!(i if i == (#name::#ident as isize).into() => #name::#ident,)
        });

        quote! {
            let integer = decoder.decode_enumerated(tag)?;

            Ok(match integer {
                #(#variants)*
                _ => return Err(#crate_root::de::Error::custom("Invalid enumerated disrciminant."))
            })

        }
    } else {
        let error = format!(
            "Decoding field of type `{}`: {}",
            name.to_string(),
            crate::CHOICE_ERROR_MESSAGE
        );

        quote! {
            Err(#crate_root::de::Error::custom(#error))
        }
    };

    let decode = if config.choice {
        // To handle CHOICE enums that have newtype variants which themselves contain CHOICE enums,
        // the decode is split into two phases:
        //
        // - Dispatch on tag matching for all variants not marked #[rasn(choice)]. For struct
        //   variants, the tag is SEQUENCE. For unit variants, they are either explicitly tagged,
        //   or one untagged unit variant is allowed for "not present". Finally, for
        //   non-explicitly-tagged newtype variants, the tag of the contained type is used.
        // - For newtype variants that contain a CHOICE enum, direct tag dispatch does not work
        //   because the contained type will potentially match an arbitrary number of tags. Thus,
        //   if tag dispatch fails, and there are any newtype variants marked #[rasn(choice)], just
        //   speculatively try to decode as each such variant, and fail only if none of them are
        //   able to decode anything.

        let all_variants_with_config = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v, VariantConfig::new(&v, config)))
            .collect::<Vec<_>>();

        let non_choice_variants_with_config = all_variants_with_config
            .iter()
            .filter(|(_, _, vc)| !vc.choice)
            .collect::<Vec<_>>();

        let non_choice_tag_consts = non_choice_variants_with_config
            .iter()
            .map(|(i, _, vc)| vc.tag(*i));
        let non_choice_tags = non_choice_variants_with_config
            .iter()
            .map(|(i, _, _)| quote::format_ident!("TAG_{}", i))
            .collect::<Vec<_>>();

        let fields = non_choice_variants_with_config
            .iter()
            .map(|(i, v, variant_config)| {
                let tag = variant_config.tag(*i);
                let ident = &v.ident;
                match &v.fields {
                    syn::Fields::Unit => quote!({ decoder.decode_null(#tag)?; #name::#ident}),
                    _ => {
                        let is_newtype = match &v.fields {
                            syn::Fields::Unnamed(_) => true,
                            _ => false,
                        };

                        let decode_fields = v.fields.iter().map(|f| {
                            let ident = f.ident.as_ref().map(|i| quote!(#i :));
                            if variant_config.choice {
                                quote!(#ident <_>::decode(decoder)?)
                            } else {
                                quote!(#ident <_>::decode_with_tag(decoder, #tag)?)
                            }
                        });

                        if is_newtype {
                            quote!(#name::#ident ( #(#decode_fields),* ))
                        } else {
                            quote!(#name::#ident { #(#decode_fields),* })
                        }
                    }
                }
            });

        let choice_variant_trials = all_variants_with_config
            .iter()
            .filter(|(_, _, vc)| vc.choice)
            .map(|(_, v, _)| {
                let ident = &v.ident;
                let field = &v.fields.iter().next().expect("Choice variant had no field");
                let field_type = &field.ty;
                quote!(
                    if let Ok(val) = #field_type::decode(decoder) {
                        return Ok(#name::#ident (val));
                    }
                )
            });
        let error = format!(
            "Decoding field of type `{}`: Invalid `CHOICE` discriminant.",
            name.to_string(),
        );
        let no_matching_tag_arm = quote!(
            #(#choice_variant_trials)*
            return Err(#crate_root::de::Error::custom(#error))
        );

        let decode_impl = if non_choice_tags.is_empty() {
            no_matching_tag_arm
        } else {
            quote! {
                #(
                    const #non_choice_tags: #crate_root::Tag = #non_choice_tag_consts;
                )*

                let tag = decoder.peek_tag()?;
                Ok(match tag {
                    #(#non_choice_tags => #fields,)*
                    _ => { #no_matching_tag_arm },
                })
            }
        };
        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                #decode_impl
            }
        })
    } else {
        None
    };

    let crate_root = &config.crate_root;

    for param in generics.type_params_mut() {
        param.colon_token = Some(Default::default());
        param.bounds = {
            let mut punct = syn::punctuated::Punctuated::new();
            punct.push(
                syn::TraitBound {
                    paren_token: None,
                    modifier: syn::TraitBoundModifier::None,
                    lifetimes: None,
                    path: {
                        let mut path = crate_root.clone();
                        path.segments.push(syn::PathSegment {
                            ident: quote::format_ident!("Decode"),
                            arguments: syn::PathArguments::None,
                        });

                        path
                    },
                }
                .into(),
            );

            punct
        };
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    proc_macro2::TokenStream::from(quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> Result<Self, D::Error> {
                #decode_with_tag
            }

            #decode
        }
    })
}
