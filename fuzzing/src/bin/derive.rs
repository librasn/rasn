extern crate afl;

use std::str::FromStr;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let s = format!("#[derive(AsnType, Decode, Encode,)]\n{s}");
            if let Ok(tok) = proc_macro2::TokenStream::from_str(&s) {
                if let Ok(derive_input) = syn::parse2::<syn::DeriveInput>(tok) {
                    let _ = rasn_derive_impl::decode_derive_inner(derive_input.clone());
                    let _ = rasn_derive_impl::encode_derive_inner(derive_input.clone());
                    let _ = rasn_derive_impl::asn_type_derive_inner(derive_input);
                }
            }
        }
    });
}
