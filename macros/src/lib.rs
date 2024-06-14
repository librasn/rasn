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

/// Helper function print out the derive.
fn __print_stream(stream: proc_macro2::TokenStream) -> proc_macro::TokenStream {
    println!("{}", stream);
    stream.into()
}

/// An automatic derive of the `Decode` trait.
///
/// Will automatically generate a decode implementation using the your
/// container's definition. See [`AsnType`](`asn_type_derive`) for information
/// on available attributes.
#[proc_macro_derive(Decode, attributes(rasn))]
pub fn decode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;
    let crate_root = &config.crate_root;

    match input.data {
        // Unit structs are treated as ASN.1 NULL values.
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => quote! {
            impl #crate_root::Decode for #name {
                fn decode_with_tag_and_constraints<D: #crate_root::Decoder>(
                    decoder: &mut D,
                    tag: #crate_root::Tag,
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
    }
    .into()
}

/// An automatic derive of the `Encode` trait.
///
/// Will automatically generate a encode implementation using the your
/// container's definition. See [`AsnType`](`asn_type_derive`) for information
/// on available attributes.
#[proc_macro_derive(Encode, attributes(rasn))]
pub fn encode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);

    let name = input.ident;
    let generics = input.generics;
    let crate_root = &config.crate_root;

    match input.data {
        // Unit structs are treated as ASN.1 NULL values.
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => quote! {
            impl #crate_root::Encode for #name {
                fn encode_with_tag_and_constraints<E: #crate_root::Encoder>(
                    &self,
                    encoder: &mut E,
                    tag: #crate_root::Tag,
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
        _ => todo!(),
    }
    .into()
}

/// An automatic derive of the `AsnType` trait.
///
/// This macro will automatically generate an implementation of `AsnType`,
/// and generate a *compile-time* check that all of your fields (if struct) or
/// variants (if a choice style enum) have distinct tags.
///
/// ##### Shared Attributes
/// These attributes are available on containers, variants, and fields.
/// - *`tag([class], number)`* — override the default tag with the one
///   specified with this attribute. E.g. `#[rasn(tag(context, 0))]`, you can also
///   wrapp `[class], number` in `explicit` to mark it as a explicit tag
///   (e.g.  `#[rasn(tag(explicit(0)))]`.)
///
/// ##### Container Attributes
/// - `crate_root` The path to the `rasn` library to use in the macro.
/// - `enumerated/choice` Use either `#[rasn(choice)]` or `#[rasn(enumerated)]`
/// - `delegate` Only available for newtype wrappers (e.g. `struct Delegate(T)`);
///   uses the inner `T` type for implementing the trait.
#[proc_macro_derive(AsnType, attributes(rasn))]
pub fn asn_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    match input.data {
        syn::Data::Struct(v) => asn_type::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => r#enum::Enum {
            name,
            generics,
            variants,
            config,
        }
        .impl_asntype(),
        _ => panic!("Union types are not supported."),
    }
    .into()
}
