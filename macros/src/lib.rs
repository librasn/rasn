use syn::{parse_macro_input, DeriveInput};

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
    let derive_input = parse_macro_input!(input as DeriveInput);

    rasn_derive_impl::decode_derive_inner(derive_input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// An automatic derive of the `Encode` trait.
///
/// Will automatically generate a encode implementation using the your
/// container's definition. See [`AsnType`](`asn_type_derive`) for information
/// on available attributes.
#[proc_macro_derive(Encode, attributes(rasn))]
pub fn encode_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    rasn_derive_impl::encode_derive_inner(derive_input)
        .unwrap_or_else(syn::Error::into_compile_error)
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
///   uses the inner `T` type for implementing the trait. Tuple-struct can have more than one field if other fields are `PhantomData` types.
#[proc_macro_derive(AsnType, attributes(rasn))]
pub fn asn_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    rasn_derive_impl::asn_type_derive_inner(derive_input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
