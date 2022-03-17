use crate::{config::*, ext::GenericsExt};

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;
    let list: Vec<_> = container
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| FieldConfig::new(field, config).encode(i, true))
        .collect();

    generics.add_trait_bounds(crate_root, quote::format_ident!("Encode"));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;

        if config
            .tag
            .as_ref()
            .map(|tag| tag.explicit)
            .unwrap_or_default()
        {
            let encode = quote!(encoder.encode_explicit_prefix(tag, &self.0).map(drop));
            if config.option_type.is_option_type(&ty) {
                let none_variant = &config.option_type.none_variant;
                quote! {
                    if !matches!(&self.0, #none_variant) {
                        #encode
                    }
                }
            } else {
                encode
            }
        } else {
            quote!(<#ty as #crate_root::Encode>::encode_with_tag(&self.0, encoder, tag))
        }
    } else {
        let operation = config
            .set
            .then(|| quote!(encode_set))
            .unwrap_or_else(|| quote!(encode_sequence));

        let encode_impl = quote! {
            encoder.#operation(tag, |encoder| {
                #(#list)*

                Ok(())
            }).map(drop)
        };

        if config.tag.as_ref().map_or(false, |tag| tag.explicit) {
            map_to_inner_type(
                quote!(tag),
                &name,
                &container.fields,
                &generics,
                &crate_root,
                encode_impl,
            )
        } else {
            encode_impl
        }
    };

    let vars = fields_as_vars(&container.fields);
    quote! {
        impl #impl_generics  #crate_root::Encode for #name #ty_generics #where_clause {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag) -> core::result::Result<(), EN::Error> {
                #(#vars)*

                #encode_impl
            }
        }
    }
}

pub fn map_to_inner_type(
    tag: proc_macro2::TokenStream,
    name: &syn::Ident,
    fields: &syn::Fields,
    generics: &syn::Generics,
    crate_root: &syn::Path,
    encode_impl: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let inner_name = quote::format_ident!("Inner{}", name);
    let mut inner_generics = generics.clone();
    inner_generics.params.push(
        syn::LifetimeDef::new(syn::Lifetime::new("'inner", proc_macro2::Span::call_site())).into(),
    );

    let (field_defs, init_fields) = match &fields {
        syn::Fields::Named(_) => {
            let field_defs = fields.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                quote!(#name : &'inner #ty)
            });

            let init_fields = fields.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                quote!(#name : &#name)
            });

            fn wrap(
                fields: impl Iterator<Item = proc_macro2::TokenStream>,
            ) -> proc_macro2::TokenStream {
                quote!({ #(#fields),* })
            }

            (wrap(field_defs), wrap(init_fields))
        }
        syn::Fields::Unnamed(_) => {
            let field_defs = fields.iter().map(|field| {
                let ty = &field.ty;
                quote!(&'inner #ty)
            });

            let init_fields = fields.iter().enumerate().map(|(i, _)| {
                let i = syn::Index::from(i);
                quote!(&self.#i)
            });

            fn wrap(
                fields: impl Iterator<Item = proc_macro2::TokenStream>,
            ) -> proc_macro2::TokenStream {
                quote!((#(#fields),*))
            }

            (wrap(field_defs), wrap(init_fields))
        }
        syn::Fields::Unit => (quote!(;), quote!()),
    };

    let vars = fields_as_vars(&fields);
    let (impl_generics, ty_generics, where_clause) = inner_generics.split_for_impl();
    quote! {
        struct #inner_name #inner_generics #field_defs

            impl #impl_generics  #crate_root::AsnType for #inner_name #ty_generics #where_clause {
                const TAG: #crate_root::Tag = #crate_root::Tag::SEQUENCE;
            }

        impl #impl_generics  #crate_root::Encode for #inner_name #ty_generics #where_clause {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag) -> core::result::Result<(), EN::Error> {
                #(#vars)*

                #encode_impl
            }
        }

        let inner = #inner_name #init_fields;

        encoder.encode_explicit_prefix(#tag, &inner).map(drop)
    }
}

fn fields_as_vars(fields: &syn::Fields) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().enumerate().map(|(i, field)| {
        let self_name = field
            .ident
            .as_ref()
            .map(|ident| quote!(#ident))
            .unwrap_or_else(|| {
                let i = syn::Index::from(i);
                quote!(#i)
            });

        let name = field
            .ident
            .is_some()
            .then(|| self_name.clone())
            .unwrap_or_else(|| {
                let ident = format_ident!("i{}", i);
                quote!(#ident)
            });

        quote!(#[allow(unused)] let #name = &self.#self_name;)
    })
}
