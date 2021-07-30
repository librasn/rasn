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
        let ty = &field.ty;
        let field_config = FieldConfig::new(field, config);

        let or_else = match field_config.default {
            Some(Some(ref path)) => quote! { .unwrap_or_else(#path) },
            Some(None) => quote! { .unwrap_or_default() },
            None => quote!(?),
        };
        let tag = field_config.tag(i);

        list.push(quote! {
            #lhs if <#ty as #crate_root::AsnType>::TAG.const_eq(&#crate_root::Tag::EOC) {
                <_>::decode(decoder)
            } else {
                <_>::decode_with_tag(decoder, #tag)
            } #or_else
        });
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
        let tags = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, _)| quote::format_ident!("TAG_{}", i));

        let tag_consts = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| VariantConfig::new(&v, config).tag(i));

        let variants = container.variants.iter().enumerate().map(|(i, v)| {
            let variant_config = VariantConfig::new(&v, config);
            let variant_tag = variant_config.tag(i);
            let ident = &v.ident;
            match &v.fields {
                syn::Fields::Unit => quote!(if let Ok(()) = decoder.decode_null(#variant_tag) { return Ok(#name::#ident) }),
                syn::Fields::Unnamed(_) => {
                    if v.fields.len() != 1 {
                        panic!("Tuple struct variants should contain only a single element.");
                    }
                    if config.automatic_tagging || variant_config.tag.is_some() {
                        quote!(if let Ok(value) = <_>::decode_with_tag(decoder, #variant_tag).map(#name::#ident) { return Ok(value) })
                    } else {
                        quote!(if let Ok(value) = <_>::decode(decoder).map(#name::#ident) { return Ok(value) })
                    }
                },
                syn::Fields::Named(_) => {
                    let decode_fields = v.fields.iter().map(|f| {
                        let field_config = FieldConfig::new(&f, config);
                        let ident = f.ident.as_ref().map(|i| quote!(#i :));
                        if config.automatic_tagging || field_config.tag.is_some() {
                            quote!(#ident <_>::decode_with_tag(decoder, #variant_tag)?)
                        } else {
                            quote!(#ident <_>::decode(decoder)?)
                        }
                    });

                    quote! {
                        let decode_fn = |decoder: &mut D| {
                            decoder.decode_sequence(#variant_tag, |decoder| {
                                Ok::<_, D::Error>(#name::#ident { #(#decode_fields),* })
                            })
                        };

                        if let Ok(value) = (decode_fn)(decoder) { return Ok(value) }
                    }
                }
            }
        });

        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                #(
                    const #tags: #crate_root::Tag = #tag_consts;
                )*

                #(#variants)*

                Err(#crate_root::de::Error::custom("Invalid `CHOICE` discriminant."))
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
