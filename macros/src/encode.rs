use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut list = vec![];
    for (i, field) in container.fields.iter().enumerate() {
        let field_config = FieldConfig::new(field, config);
        let tag = field_config.tag(i);
        let i = syn::Index::from(i);
        let field = field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));

        if field_config.tag.is_some() || config.automatic_tags {
            list.push(proc_macro2::TokenStream::from(
                quote!(self.#field.encode_with_tag(encoder, #tag)?;),
            ));
        } else {
            list.push(proc_macro2::TokenStream::from(
                quote!(self.#field.encode(encoder)?;),
            ));
        }
    }

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
                            ident: quote::format_ident!("Encode"),
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

    let encode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;
        quote!(<#ty as #crate_root::Encode>::encode_with_tag(&self.0, encoder, tag))
    } else {
        quote! {
            encoder.encode_sequence(tag, |encoder| {
                #(#list)*

                Ok(())
            }).map(drop)
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    proc_macro2::TokenStream::from(quote! {
        impl #impl_generics  #crate_root::Encode for #name #ty_generics #where_clause {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag) -> Result<(), EN::Error> {
                #encode_impl
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

    let encode_with_tag = if config.enumerated {
        quote! {
            encoder.encode_enumerated(tag, *self as isize).map(drop)
        }
    } else {
        let error = format!(
            "Encoding field of type `{}`: {}",
            name.to_string(),
            crate::CHOICE_ERROR_MESSAGE
        );
        quote! {
            Err::<(), _>(#crate_root::enc::Error::custom(#error))
        }
    };

    let encode = if config.choice {
        let variants = container.variants.iter().enumerate().map(|(i, v)| {
            let ident = &v.ident;
            let variant_config = VariantConfig::new(&v, &config);
            let variant_tag = variant_config.tag(i);

            match &v.fields {
                syn::Fields::Named(_) => {
                    let idents = v.fields.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote!(#ident)
                    });

                    let fields = v.fields.iter().map(|f| {
                        let field_config = FieldConfig::new(&f, config);
                        let ident = f.ident.as_ref().unwrap();
                        if field_config.tag.is_some() {
                            let tag = field_config.tag(i);
                            quote!(#crate_root::Encode::encode_with_tag(#ident, encoder, #tag)?;)
                        } else {
                            quote!(#crate_root::Encode::encode(#ident, encoder)?;)
                        }
                    });

                    quote!(#name::#ident { #(#idents),* } => {
                        encoder.encode_sequence(#variant_tag, |encoder| {
                            #(#fields)*
                            Ok(())
                        })
                    })
                }
                syn::Fields::Unnamed(_) => {
                    if v.fields.iter().count() != 1 {
                        panic!("Tuple variants must contain only a single element.");
                    }
                    if variant_config.tag.is_some() || config.automatic_tags {
                        let ty = v.fields.iter().next().unwrap();
                        quote! {
                            #name::#ident(value) => {
                                if <#ty as #crate_root::AsnType>::TAG.const_eq(&#crate_root::Tag::EOC) {
                                    encoder.encode_explicit_prefix(#variant_tag, value).map(drop)
                                } else {
                                    #crate_root::Encode::encode_with_tag(value, encoder, #variant_tag).map(drop)
                                }
                            }
                        }
                    } else {
                        quote!(#name::#ident(value) => { #crate_root::Encode::encode(value, encoder).map(drop) })
                    }
                }
                syn::Fields::Unit => quote!(#name::#ident => { encoder.encode_null(#variant_tag).map(drop) }),
            }
        });

        Some(quote! {
            fn encode<E: #crate_root::Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                match self {
                    #(#variants),*
                }.map(drop)
            }
        })
    } else {
        None
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
                            ident: quote::format_ident!("Encode"),
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
        #[automatically_derived]
        impl #impl_generics  #crate_root::Encode for #name #ty_generics #where_clause {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag) -> Result<(), EN::Error> {
                #encode_with_tag
            }

            #encode
        }
    })
}
