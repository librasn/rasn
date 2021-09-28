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
                let tag = FieldConfig::new(field, config).tag(i);
                let name = quote::format_ident!("Field{}", i);

                (
                    name.clone(),
                    quote! {
                        #[derive(#crate_root::Decode, #crate_root::Encode)]
                        #[rasn(delegate)]
                        pub struct #name(#ty);

                        impl #crate_root::AsnType for #name {
                            const TAG: Tag = #tag;
                        }
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

        quote! {
            #choice_def
            #(#field_type_defs)*

            decoder.decode_set::<#choice_name, _, _>(tag, |fields| {
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

                #(let #required_field_names = #required_field_names.ok_or_else(|| #crate_root::de::Error::missing_field("#required_field_names"))?;)*

                Ok(Self #set_init)
            })
        }
    } else {
        for (i, field) in container.fields.iter().enumerate() {
            let field_config = FieldConfig::new(field, config);

            list.push(field_config.decode(&name, i));
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

    quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> Result<Self, D::Error> {
                #decode_impl
            }
        }
    }
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
        let variants = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| VariantConfig::new(v, config).decode(&name, i));

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

    quote! {
        impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> Result<Self, D::Error> {
                #decode_with_tag
            }

            #decode
        }
    }
}
