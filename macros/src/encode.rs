use crate::config::{Config, VariantConfig};

pub fn derive_struct_impl(
    name: syn::Ident,
    generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut list = vec![];
    for (i, field) in container.fields.iter().enumerate() {
        let i = syn::Index::from(i);
        let field = field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));

        list.push(proc_macro2::TokenStream::from(
            quote!(self.#field.encode(encoder)?;),
        ));
    }

    let crate_root = &config.crate_root;
    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Encode for #name #generics {

            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
                encoder.encode_sequence(tag, |encoder| {
                    #(#list)*

                    Ok(())
                }).map(drop)
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

    let encode_with_tag = if config.enumerated {
        quote! {
            encoder.encode_enumerated(tag, *self as isize).map(drop)
        }
    } else {
        quote! {
            Err(#crate_root::enc::Error::custom("CHOICE-style enums do not allow implicit tagging."))
        }
    };

    let encode = if config.choice {
        let variants = container.variants.iter().map(|v| {
            let ident = &v.ident;
            let tag = VariantConfig::new(&v).tag(crate_root);

            match &v.fields {
                syn::Fields::Named(_) => {
                    let fields = v.fields.iter().map(|v| {
                        let ident = v.ident.as_ref().unwrap();
                        quote!(#ident)
                    });
                    let fields2 = fields.clone();

                    quote!(#name::#ident { #(#fields),* } => {
                        encoder.encode_sequence(tag, |encoder| {
                            #(#crate_root::Encode::encode_with_tag(#fields2, encoder, #tag)?);*

                            Ok(())
                        })
                    })
                }
                syn::Fields::Unnamed(_) => {
                    if v.fields.iter().count() != 1 {
                        panic!("Tuple variants must contain only a single element.");
                    }
                    quote!(#name::#ident(value) => { #crate_root::Encode::encode_with_tag(value, encoder, #tag) })
                }
                syn::Fields::Unit => quote!(#name::#ident => { encoder.encode_null(#tag) }),
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

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Encode for #name #generics {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
                #encode_with_tag
            }

            #encode
        }
    })
}
