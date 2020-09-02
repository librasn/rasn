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
            quote!(let value = self.#field.encode(encoder)?;),
        ));
    }

    let crate_root = &config.crate_root;
    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Encode for #name #generics {

            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<EN::Ok, EN::Error> {
                encoder.encode_sequence(tag, |encoder| {
                    #(#list)*
                    Ok(value)
                })
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
            encoder.encode_enumerated(tag, *self as isize)
        }
    } else {
        quote! {
            Err(#crate_root::error::Error::custom("CHOICE-style enums do not allow implicit tagging."))
        }
    };

    let encode = if config.choice {
        let variants = container.variants.iter().map(|v| {
            let ident = &v.ident;
            let tags = VariantConfig::new(&v).tag(crate_root);

            match &v.fields {
                syn::Fields::Named(_) => {
                    let fields = v.fields.iter().map(|v| {
                        let ident = v.ident.as_ref().unwrap();
                        quote!(#ident)
                    });
                    let fields2 = fields.clone();

                    quote!(#name::#ident { #(#fields),* } => {
                        encoder.encode_sequence(tag, |encoder| {
                            #(let value = #fields2.encode_with_tag(encoder, #tags)?);*
                            Ok(value)
                        })
                    })
                }
                syn::Fields::Unnamed(_) => {
                    if v.fields.iter().count() != 1 {
                        panic!("Tuple variants must contain only a single element.");
                    }
                    quote!(#name::#ident(value) => { #crate_root::Encode::encode(value, encoder) })
                }
                syn::Fields::Unit => quote!(#name::#ident => { encoder.encode_null(<()>::TAG) }),
            }
        });

        Some(quote! {
            fn encode<E: #crate_root::Encoder>(&self, encoder: &mut E) -> Result<E::Ok, E::Error> {
                match self {
                    #(#variants),*
                }
            }
        })
    } else {
        None
    };

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Encode for #name #generics {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<EN::Ok, EN::Error> {
                #encode_with_tag
            }

            #encode
        }
    })
}
