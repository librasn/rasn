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
    } else if config.set {
        let field_names = container.fields.iter().map(|field| field.ident.clone());
        let field_names2 = field_names.clone();
        let field_names3 = field_names.clone();
        let required_field_names = container.fields.iter().filter(|field| !FieldConfig::new(field, config).is_option_type()).map(|field| field.ident.clone());
        let (field_type_names, field_type_defs): (Vec<_>, Vec<_>) = container.fields.iter().enumerate().map(|(i, field)| {
            let ty = config.option_type.map_to_inner_type(&field.ty).unwrap_or(&field.ty);
            let tag = FieldConfig::new(field, config).tag(i);
            let name = quote::format_ident!("Field{}", i);

            (name.clone(), quote! {
                #[derive(#crate_root::Decode, #crate_root::Encode)]
                #[rasn(delegate)]
                pub struct #name(#ty);

                impl #crate_root::AsnType for #name {
                    const TAG: Tag = #tag;
                }
            })
        }).unzip();

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

        quote! {
            #choice_def
            #(#field_type_defs)*

            decoder.decode_set::<#choice_name, _, _>(tag, |fields| {
                #(let mut #field_names = None;)*

                for field in fields {
                    match field {
                        #(#choice_name::#field_type_names(value) => { #field_names2 = Some(value.0) })*
                    }
                }

                #(let #required_field_names = #required_field_names.ok_or_else(|| #crate_root::de::Error::missing_field("#required_field_names"))?;)*

                Ok(Self #set_init)
            })
        }
    } else {
        for (i, field) in container.fields.iter().enumerate() {
            let lhs = field.ident.as_ref().map(|i| quote!(#i :));
            let field_config = FieldConfig::new(field, config);

            let or_else = match field_config.default {
                Some(Some(ref path)) => quote! { .unwrap_or_else(#path) },
                Some(None) => quote! { .unwrap_or_default() },
                None => {
                    let ident = format!("{}.{}", name, field.ident.as_ref().map(|ident| ident.to_string()).unwrap_or_else(|| i.to_string()));
                    quote!(.map_err(|error| #crate_root::de::Error::field_error(#ident, error))?)
                }
            };
            let tag = field_config.tag(i);

            if field_config.tag.is_some() || config.automatic_tags {
                list.push(quote!(#lhs <_>::decode_with_tag(decoder, #tag) #or_else));
            } else {
                list.push(quote!(#lhs <_>::decode(decoder) #or_else));
            }
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
        quote! {
            decoder.decode_explicit_prefix(tag)
        }
    };

    let decode = if config.choice {
        let variants = container.variants.iter().enumerate().map(|(i, v)| {
            let variant_config = VariantConfig::new(&v, config);
            let variant_tag = variant_config.tag(i);
            let ident = &v.ident;
            match &v.fields {
                syn::Fields::Unit => quote!(if let Ok(()) = <()>::decode_with_tag(decoder, #variant_tag) { return Ok(#name::#ident) }),
                syn::Fields::Unnamed(_) => {
                    if v.fields.len() != 1 {
                        panic!("Tuple struct variants should contain only a single element.");
                    }

                    let decode_operation = if config.automatic_tags || variant_config.tag.is_some() {
                        quote!(<_>::decode_with_tag(decoder, #variant_tag))
                    } else {
                        quote!(<_>::decode(decoder))
                    };

                    quote!{
                        if let Ok(value) = #decode_operation.map(#name::#ident) { return Ok(value) }
                    }
                },
                syn::Fields::Named(_) => {
                    let decode_fields = v.fields.iter().enumerate().map(|(i, field)| {
                        let field_config = FieldConfig::new(&field, config);
                        let lhs = field.ident.as_ref().map(|i| quote!(#i :));
                        let ident = format!("{}.{}", name, field.ident.as_ref().map(|ident| ident.to_string()).unwrap_or_else(|| i.to_string()));
                        let map_field_error = quote!(.map_err(|error| #crate_root::de::Error::field_error(#ident, error)));
                        if config.automatic_tags || field_config.tag.is_some() {
                            quote!(#lhs <_>::decode_with_tag(decoder, #variant_tag) #map_field_error?)
                        } else {
                            quote!(#lhs <_>::decode(decoder) #map_field_error?)
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

        let name = syn::LitStr::new(&name.to_string(), proc_macro2::Span::call_site());
        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                #(#variants);*
                Err(#crate_root::de::Error::no_valid_choice(#name))
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
