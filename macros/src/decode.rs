use syn::Fields;

use crate::config::*;

pub fn derive_struct_impl(
    name: syn::Ident,
    generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut list = vec![];
    let crate_root = &config.crate_root;

    for field in container.fields.iter() {
        let lhs = field.ident.as_ref().map(|i| quote!(#i :));
        let tag = FieldConfig::new(field, config).tag();

        list.push(proc_macro2::TokenStream::from(
            quote!(#lhs <_>::decode_with_tag(&mut decoder, #tag)?),
        ));
    }

    let fields = match container.fields {
        Fields::Named(_) => quote!({ #(#list),* }),
        Fields::Unnamed(_) => quote!(( #(#list),* )),
        Fields::Unit => quote!(),
    };

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Decode for #name #generics {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                let mut decoder = decoder.decode_sequence(tag)?;
                Ok(Self #fields)
            }
        }
    })
}

pub fn derive_enum_impl(
    name: syn::Ident,
    generics: syn::Generics,
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
            Err(#crate_root::de::Error::custom("`CHOICE`-style enums cannot be implicitly tagged."))
        }
    };

    let decode = if config.choice {
        let tags = container
            .variants
            .iter()
            .enumerate()
            .map(|(i, _)| quote::format_ident!("TAG_{}", i));
        let tags2 = tags.clone();

        let tag_consts = container
            .variants
            .iter()
            .map(|v| VariantConfig::new(&v, config).tag());
        let tag_consts2 = tag_consts.clone();

        let fields = container.variants.iter().map(|v| {
            let tag = VariantConfig::new(&v, config).tag();
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
                        quote!(#ident <_>::decode_with_tag(decoder, #tag)?)
                    });

                    if is_newtype {
                        quote!(#name::#ident ( #(#decode_fields),* ))
                    } else {
                        quote!(#name::#ident { #(#decode_fields),* })
                    }
                }
            }
        });

        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                #crate_root::sa::const_assert!(#crate_root::Tag::is_distinct_set(&[#(#tag_consts),*]));

                #(
                    const #tags: #crate_root::Tag = #tag_consts2;
                )*

                let tag = decoder.peek_tag()?;
                Ok(match tag {
                    #(#tags2 => #fields,)*
                    _ => return Err(#crate_root::de::Error::custom("Invalid `CHOICE` discriminant.")),
                })
            }
        })
    } else {
        None
    };

    proc_macro2::TokenStream::from(quote! {
        impl #generics #crate_root::Decode for #name #generics {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                #decode_with_tag
            }

            #decode
        }
    })
}
