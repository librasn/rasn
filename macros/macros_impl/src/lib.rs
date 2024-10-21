#[macro_use]
extern crate quote;

mod asn_type;
mod config;
mod decode;
mod encode;
mod r#enum;
mod ext;
mod tag;

use config::Config;
use syn::DataStruct;

const CRATE_NAME: &str = "rasn";

pub fn decode_derive_inner(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse2(input)?;
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;
    let crate_root = &config.crate_root;

    Ok(match input.data {
        // Unit structs are treated as ASN.1 NULL values.
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => quote! {
            impl #crate_root::Decode for #name {
                fn decode_with_tag_and_constraints<D: #crate_root::Decoder>(
                    decoder: &mut D,
                    tag: #crate_root::types::Tag,
                    _: #crate_root::prelude::Constraints,
                ) -> Result<Self, D::Error> {
                    decoder.decode_null(tag).map(|_| #name)
                }
            }
        },
        syn::Data::Struct(v) => decode::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => r#enum::Enum {
            name,
            generics,
            variants,
            config,
        }
        .impl_decode(),
        _ => panic!("Union types are not supported."),
    })
}
pub fn encode_derive_inner(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse2(input)?;
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;
    let crate_root = &config.crate_root;

    Ok(match input.data {
        // Unit structs are treated as ASN.1 NULL values.
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => quote! {
            impl #crate_root::Encode for #name {
                fn encode_with_tag_and_constraints<E: #crate_root::Encoder>(
                    &self,
                    encoder: &mut E,
                    tag: #crate_root::types::Tag,
                    _: #crate_root::prelude::Constraints,
                ) -> Result<(), E::Error> {
                    encoder.encode_null(tag).map(drop)
                }
            }
        },
        syn::Data::Struct(v) => encode::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => r#enum::Enum {
            name,
            generics,
            variants,
            config,
        }
        .impl_encode(),
        _ => panic!("Union types are not supported."),
    })
}

pub fn asn_type_derive_inner(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let input = syn::parse2(input)?;
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    Ok(match input.data {
        syn::Data::Struct(v) => asn_type::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => r#enum::Enum {
            name,
            generics,
            variants,
            config,
        }
        .impl_asntype(),
        _ => panic!("Union types are not supported."),
    })
}
