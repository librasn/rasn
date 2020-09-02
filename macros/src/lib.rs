#[macro_use] extern crate quote;

mod config;
mod asn_type;
mod decode;
mod encode;

use config::Config;

const CRATE_NAME: &str = "rasn";

/// Helper function print out the derive.
fn _print_stream(stream: proc_macro2::TokenStream) -> proc_macro::TokenStream {
    println!("{}", stream);
    stream.into()
}

#[proc_macro_derive(Decode, attributes(rasn))]
pub fn decode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let config = Config::from_attributes(&input);
    let name = input.ident;
    let generics = input.generics;

    _print_stream(match input.data {
        syn::Data::Struct(v) => decode::derive_struct_impl(name, generics, v, &config),
        syn::Data::Enum(v) => decode::derive_enum_impl(name, generics, v, &config),
        _ => panic!("Union types are not supported."),
    }.into())
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
    }.into()
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
