use syn::Fields;

use crate::config::{Config, VariantConfig};

pub fn derive_struct_impl(name: syn::Ident, generics: syn::Generics, container: syn::DataStruct, config: &Config) -> proc_macro2::TokenStream {
    let mut list = vec![];
    let crate_root = &config.crate_root;

    for field in container.fields.iter() {
        let lhs = field.ident.as_ref().map(|i| quote!(#i :));

        list.push(proc_macro2::TokenStream::from(quote!(#lhs <_>::decode(&mut decoder)?)));
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

pub fn derive_enum_impl(name: syn::Ident, generics: syn::Generics, container: syn::DataEnum, config: &Config) -> proc_macro2::TokenStream {

    let crate_root = &config.crate_root;
    let decode_with_tag = if config.enumerated {
        let variants = container.variants.iter()
            .map(|v| {
                let ident = &v.ident;
                quote!(i if i == (#name::#ident as isize).into() => #name::#ident,)
            });

        quote! {
            let integer = decoder.decode_enumerated(tag)?;

            Ok(match integer {
                #(#variants)*
                _ => return Err(#crate_root::error::Error::custom("Invalid enumerated disrciminant."))
            })

        }
    } else {
        quote! {
            Err(#crate_root::error::Error::custom("`CHOICE`-style enums cannot be implicitly tagged."))
        }
    };

    let decode = if config.choice {
        let idents = container.variants.iter().map(|v| &v.ident);
        let tags = container.variants.iter().enumerate().map(|(i, _)| {
            quote::format_ident!("TAG_{}", i)
        });

        let tag_consts = container.variants.iter().map(|v| {
            let tag = VariantConfig::new(&v).tag(crate_root);
            quote!(#tag => ())
        });

        let fields = container.variants.iter().map(|v| match &v.fields {
            syn::Fields::Unit => quote!(decoder.decode_null(<()>::TAG)?),
            _ => {
                let is_newtype = match &v.fields {
                    syn::Fields::Unnamed(_) => true,
                    _ => false
                };

                let decode_fields = v.fields.iter().map(|f| {
                    let ident = f.ident.as_ref().map(|i| quote!(#i :));
                    quote!(#ident <_>::decode(decoder)?)
                });

                if is_newtype {
                    quote!(( #(#decode_fields),* ))
                } else {
                    quote!({ #(#decode_fields),* })
                }
            }
        });

        Some(quote! {
            fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
                Ok(match decoder.peek_tag()? {
                    #(#tags => #name::#idents #fields,)*
                    _ => return Err(#crate_root::error::Error::custom("Invalid `CHOICE` discriminant.")),
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
