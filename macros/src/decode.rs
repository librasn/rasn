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
        // simple direct tag dispatch does not work because the contained type will potentially
        // match an arbitrary number of tags. Thus, the decode is split into two phases:
        //
        // - First, dispatch on tag matching. For newtype variants, the pattern arm is guarded by
        //   the value of `!<ContainedType as AsnType>::CHOICE`, which is const. This has the
        //   effect of compiling out the match arm if the newtype variant contains a choice enum.
        // - Then, in the fall-through arm, for every newtype variant, a speculative attempt
        //   to decode as the contained type is inserted, guarded by an if with the complimentary
        //   const condition `<ContainedType as AsnType>::CHOICE`. This compiles out if the newtype
        //   variant does *not* contain a choice enum.
        // - Decoding fails only if both tag dispatch fails, and if none of the newtype variants
        //   that contain a match enum are able to decode anything.

        let variants_with_config = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v, VariantConfig::new(&v, config)))
            .collect::<Vec<_>>();

        let tag_consts = variants_with_config.iter().map(|(i, _, vc)| vc.tag(*i));

        let tags = variants_with_config
            .iter()
            .map(|(i, _, _)| quote::format_ident!("TAG_{}", i))
            .collect::<Vec<_>>();

        let tag_match_arms = variants_with_config.iter().map(|(i, v, variant_config)| {
            let tag = variant_config.tag(*i);
            let ident = &v.ident;
            let pat = quote::format_ident!("TAG_{}", i);
            match &v.fields {
                syn::Fields::Unit => {
                    quote!(
                        #pat => { decoder.decode_null(#tag)?; #name::#ident }
                    )
                }
                syn::Fields::Unnamed(_) => {
                    let field = v
                        .fields
                        .iter()
                        .next()
                        .expect("Newtype variant had no field");
                    let ty = &field.ty;
                    // The pattern guard being inserted here is const, so this arm will get
                    // compiled out if the variant's contained type is a choice enum, and
                    // complementary code in the fall-through branch will decode the variant
                    // instead.
                    quote!(
                        #pat if !<#ty as #crate_root::AsnType>::CHOICE => {
                            #name::#ident ( <_>::decode_with_tag(decoder, #tag)? )
                        }
                    )
                }
                syn::Fields::Named(_) => {
                    let decode_fields = v.fields.iter().map(|f| {
                        let ident = f
                            .ident
                            .as_ref()
                            .expect("Struct variant field had no identifier");
                        let ty = &f.ty;
                        // Similarly, the if condition here is const, controlled by type of
                        // each field in this struct variant, so only the correct branch is
                        // compiled.
                        quote!(
                            #ident : if <#ty as #crate_root::AsnType>::CHOICE {
                                <_>::decode(decoder)?
                            } else {
                                <_>::decode_with_tag(decoder, #tag)?
                            }
                        )
                    });
                    quote!(
                        #pat => {
                            #name::#ident { #(#decode_fields),* }
                        }
                    )
                }
            }
        });

        let newtype_variant_trials = variants_with_config.iter().filter_map(|(_, v, _)| {
            if let syn::Fields::Unnamed(_) = &v.fields {
                let ident = &v.ident;
                let field = &v
                    .fields
                    .iter()
                    .next()
                    .expect("Newtype variant had no field");
                let ty = &field.ty;
                // And finally, this if condition is const and complimentary to the pattern guard
                // inserted above.
                Some(quote!(
                    if <#ty as #crate_root::AsnType>::CHOICE {
                        if let Ok(val) = #ty::decode(decoder) {
                            return Ok(#name::#ident (val));
                        }
                    }
                ))
            } else {
                None
            }
        });
        let error = format!(
            "Decoding field of type `{}`: Invalid `CHOICE` discriminant.",
            name.to_string(),
        );

        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                #(
                    const #tags: #crate_root::Tag = #tag_consts;
                )*

                let tag = decoder.peek_tag()?;
                Ok(match tag {
                    #(#tag_match_arms)*
                    _ => {
                        #(#newtype_variant_trials)*
                        return Err(#crate_root::de::Error::custom(#error))
                    },
                })
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
