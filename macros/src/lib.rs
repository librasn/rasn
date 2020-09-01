#[macro_use] extern crate quote;

mod config;
mod asn_type;
mod decode;

use config::Config;

const CRATE_NAME: &str = "rasn";

#[proc_macro_derive(Decode, attributes(rasn))]
pub fn decode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    match input.data {
        syn::Data::Struct(v) => decode::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(v) => decode::derive_enum_impl(name, generics, v, &config),
        _ => panic!("Union types are not supported."),
    }.into()
}

#[proc_macro_derive(Encode, attributes(rasn))]
pub fn encode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);

    let name = input.ident;
    let generics = input.generics;
    // let metas = input.attrs.into_iter().filter_map(|a| a.parse_meta().ok());

    proc_macro::TokenStream::from(match input.data {
        syn::Data::Struct(v) => decode_derive_struct_impl(name, generics, v, &config),
        _ => todo!(),
    })
}

fn decode_derive_struct_impl(name: syn::Ident, generics: syn::Generics, container: syn::DataStruct, config: &Config) -> proc_macro2::TokenStream {
    let mut list = vec![];
    for (i, field) in container.fields.iter().enumerate() {
        let i = syn::Index::from(i);
        let field = field.ident.as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));

        list.push(proc_macro2::TokenStream::from(quote!(let value = self.#field.encode(encoder)?;)));
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

#[proc_macro_derive(AsnType, attributes(rasn))]
pub fn asn_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    match input.data {
        syn::Data::Struct(_) => asn_type::derive_struct_impl(name, generics, &config).into(),
        syn::Data::Enum(_) => asn_type::derive_enum_impl(name, generics, &config).into(),
        _ => panic!("Union types are not supported."),
    }
}
