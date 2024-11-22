use crate::{config::*, ext::GenericsExt};

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;

    let mut field_encodings = Vec::with_capacity(container.fields.len());
    let mut number_root_fields: usize = 0;
    let mut number_extended_fields: usize = 0;

    // Count the number of root and extended fields so that encoder can know the number of fields in advance
    for (i, field) in container.fields.iter().enumerate() {
        let field_config = FieldConfig::new(field, config);
        let field_encoding = field_config.encode(i, true);

        if field_config.is_extension() {
            number_extended_fields += 1;
        } else {
            number_root_fields += 1;
        }

        field_encodings.push(quote! {
            #field_encoding
        });
    }

    generics.add_trait_bounds(crate_root, quote::format_ident!("Encode"));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;

        if let Some(tag) = config.tag.as_ref().filter(|tag| tag.is_explicit()) {
            let tag = tag.to_tokens(crate_root);
            // Note: encoder must be aware if the field is optional and present, so we should not do the presence check on this level
            quote!(encoder.encode_explicit_prefix(#tag, &self.0).map(drop))
        } else {
            let constraint_name = quote::format_ident!("effective_constraint");
            let constraint_def = if generics.params.is_empty() {
                quote! {
                    let #constraint_name: #crate_root::types::Constraints  = const {<#ty as #crate_root::AsnType>::CONSTRAINTS}.intersect(constraints);
                }
            } else {
                quote! {
                    let #constraint_name: #crate_root::types::Constraints  = <#ty as #crate_root::AsnType>::CONSTRAINTS.intersect(constraints);
                }
            };
            quote!(
                #constraint_def
                match tag {
                    #crate_root::types::Tag::EOC => {
                        self.0.encode(encoder)
                    }
                    _ => {
                        <#ty as #crate_root::Encode>::encode_with_tag_and_constraints(
                            &self.0,
                            encoder,
                            tag,
                            #constraint_name
                        )
                    }
                }
            )
        }
    } else {
        let operation = config
            .set
            .then(|| quote!(encode_set))
            .unwrap_or_else(|| quote!(encode_sequence));

        let encode_impl = quote! {
            // In order to avoid unnecessary allocations, we provide the constant field counts to the encoder when encoding sequences and sets.
            encoder.#operation::<#number_root_fields, #number_extended_fields, Self, _>(tag, |encoder| {
                #(#field_encodings)*
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
        #[allow(clippy::mutable_key_type)]
        impl #impl_generics  #crate_root::Encode for #name #ty_generics #where_clause {
            fn encode_with_tag_and_constraints<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::types::Tag, constraints: #crate_root::types::Constraints) -> core::result::Result<(), EN::Error> {
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
        .push(syn::LifetimeParam::new(lifetime.clone()).into());

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
                let name_prefixed = format_ident!("__rasn_field_{}", name);
                quote!(#name : &#name_prefixed)
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

    let tag = tag.to_tokens(crate_root);
    let inner_impl = if is_explicit {
        quote!(encoder.encode_explicit_prefix(#tag, &inner).map(drop))
    } else {
        quote!(inner.encode_with_tag(encoder, #tag))
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
            .as_ref()
            .map(|ident| {
                let prefixed = format_ident!("__rasn_field_{}", ident);
                quote!(#prefixed)
            })
            .unwrap_or_else(|| {
                let ident = format_ident!("i{}", i);
                quote!(#ident)
            });

        quote!(#[allow(unused)] let #name = &self.#self_name;)
    })
}
