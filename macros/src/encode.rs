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

        if let Some(tag) = config.tag.as_ref().filter(|tag| tag.is_explicit()) {
            let tag = tag.to_tokens(crate_root);
            let encode = quote!(encoder.encode_explicit_prefix(#tag, &self.0).map(drop));
            if config.option_type.is_option_type(ty) {
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
            quote!(
                <#ty as #crate_root::Encode>::encode_with_tag_and_constraints(
                    &self.0,
                    encoder,
                    tag,
                    <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(constraints)
                )
            )
        }
    } else {
        let operation = config
            .set
            .then(|| quote!(encode_set))
            .unwrap_or_else(|| quote!(encode_sequence));

        let encode_impl = quote! {
            encoder.#operation::<Self, _>(tag, |encoder| {
                #(#list)*

                Ok(())
            }).map(drop)
        };

        if config.tag.as_ref().map_or(false, |tag| tag.is_explicit()) {
            map_to_inner_type(
                config.tag.clone().unwrap(),
                &name,
                &container.fields,
                &generics,
                crate_root,
                true,
            )
        } else {
            encode_impl
        }
    };

    let vars = fields_as_vars(&container.fields);
    quote! {
        impl #impl_generics  #crate_root::Encode for #name #ty_generics #where_clause {
            fn encode_with_tag_and_constraints<'constraints, EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag, constraints: #crate_root::types::Constraints<'constraints>) -> core::result::Result<(), EN::Error> {
                #(#vars)*

                #encode_impl
            }
        }
    }
}

pub fn map_to_inner_type(
    tag: crate::tag::Tag,
    name: &syn::Ident,
    fields: &syn::Fields,
    generics: &syn::Generics,
    crate_root: &syn::Path,
    is_explicit: bool,
) -> proc_macro2::TokenStream {
    let inner_name = quote::format_ident!("Inner{}", name);
    let mut inner_generics = generics.clone();
    let lifetime = syn::Lifetime::new(
        &format!("'inner{}", uuid::Uuid::new_v4().as_u128()),
        proc_macro2::Span::call_site(),
    );
    inner_generics
        .params
        .push(syn::LifetimeDef::new(lifetime.clone()).into());

    let (field_defs, init_fields) = match &fields {
        syn::Fields::Named(_) => {
            let field_defs = fields.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let attrs = &field.attrs;
                let ty = &field.ty;
                quote!(#(#attrs)* #name : &#lifetime #ty)
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
                quote!(&#lifetime #ty)
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

    let inner_impl = if is_explicit {
        let tag = tag.to_tokens(crate_root);
        quote!(encoder.encode_explicit_prefix(#tag, &inner).map(drop))
    } else {
        quote!(inner.encode(encoder))
    };

    quote! {
        #[derive(#crate_root::AsnType, #crate_root::Encode)]
        struct #inner_name #inner_generics #field_defs

        let inner = #inner_name #init_fields;

        #inner_impl
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
