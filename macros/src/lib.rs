#[macro_use]
extern crate quote;

mod asn_type;
mod config;
mod decode;
mod encode;
mod tag;

use config::Config;

const CRATE_NAME: &str = "rasn";

/// Helper function print out the derive.
fn __print_stream(stream: proc_macro2::TokenStream) -> proc_macro::TokenStream {
    println!("{}", stream);
    stream.into()
}

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
    }
    .into()
}

#[proc_macro_derive(Encode, attributes(rasn))]
pub fn encode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);

    let name = input.ident;
    let generics = input.generics;

    match input.data {
        syn::Data::Struct(v) => encode::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(v) => encode::derive_enum_impl(name, generics, v, &config),
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
/// **Shared Attributes**
/// These attributes are available on containers, variants, and fields.
/// - *`tag([class], number)` — override the default tag with the one
/// specified 
#[proc_macro_derive(AsnType, attributes(rasn))]
pub fn asn_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    match input.data {
        syn::Data::Struct(v) => asn_type::derive_struct_impl(name, generics, v, &config).into(),
        syn::Data::Enum(v) => asn_type::derive_enum_impl(name, generics, v, &config).into(),
        _ => panic!("Union types are not supported."),
    }
}
