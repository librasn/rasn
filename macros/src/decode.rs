use syn::Fields;

use crate::config::Config;

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
    let decode = if config.enumerated {
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
        todo!()
    };

    proc_macro2::TokenStream::from(quote! {
        #[automatically_derived]
        impl #generics #crate_root::Decode for #name #generics {
            fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                #decode
            }
        }
    })
}
