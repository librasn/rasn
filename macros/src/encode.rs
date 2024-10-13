use crate::{config::*, ext::GenericsExt};

pub fn derive_struct_impl(
    name: syn::Ident,
    mut generics: syn::Generics,
    container: syn::DataStruct,
    config: &Config,
) -> proc_macro2::TokenStream {
    let crate_root = &config.crate_root;

    // let list: Vec<_> = container
    //     .fields
    //     .iter()
    //     .enumerate()
    //     .map(|(i, field)| FieldConfig::new(field, config).encode(i, true))
    //     .collect();

    let mut field_encodings = Vec::with_capacity(container.fields.len());
    let mut number_optional_default_fields: usize = 0;
    let mut number_root_fields: usize = 0;
    let mut number_extended_fields: usize = 0;

    // Set logic for setting presence bits in compile time
    for (i, field) in container.fields.iter().enumerate() {
        let field_config = FieldConfig::new(field, config);
        let field_name = field_config
            .field
            .ident
            .as_ref()
            .map(|name| quote!(#name))
            .unwrap_or_else(|| quote!(#i));
        let field_type = &field_config.field.ty;
        let field_type_tokens = quote!(#field_type);
        // let field_opt_index_expr = quote! { #number_optional_default_fields };
        let mut field_encoding = field_config.encode(i, true);

        if field_config.is_option_type() && field_config.is_not_extension() {
            // Optional field
            field_encoding = quote! {
                // if self.#field_name.is_some() {
                    // root_bitfield[#field_opt_index_expr] = true;
                    // fields.set_field_present_by_index(#i);
                    // bitfield[#i] = true;
                // }
                #field_encoding
            };
            number_optional_default_fields += 1;
            number_root_fields += 1;
            field_encodings.push(field_encoding);
        } else if field_config.is_default_type() && field_config.is_not_extension() {
            // Default field
            let default_path = &field_config.default.expect(
                "Unexpected empty default path when constructing default field value presence",
            );
            let default_call = if let Some(default_fn) = default_path.as_ref() {
                quote! { #default_fn() }
            } else {
                quote! { <#field_type_tokens as Default>::default()  }
            };
            field_encoding = quote! {
                // if self.#field_name != #default_call {
                    // root_bitfield[#field_opt_index_expr] = true;
                    // bitfield[#i] = true;
                    // fields.set_field_present_by_index(#i);
                // }
                #field_encoding
            };
            number_optional_default_fields += 1;
            number_root_fields += 1;
            field_encodings.push(field_encoding);
        } else if field_config.is_extension() && field_config.is_option_type() {
            field_encoding = quote! {
                if self.#field_name.is_some() {
                    // extension_bitfield[#number_extended_fields] = true;
                    // fields.set_field_present_by_index(#i);
                    // bitfield[#i] = true;
                    // if let Some(ref mut ext_fields) = extended_fields {
                    //     ext_fields.set_field_present_by_index(#i);
                    // }
                }
                #field_encoding
            };
            number_extended_fields += 1;
            field_encodings.push(field_encoding);
        } else if field_config.is_extension() {
            // Extension field
            field_encoding = quote! {
                // extension_bitfield[#number_extended_fields] = true;
                // bitfield[#i] = true;
                // if let Some(ref mut ext_fields) = extended_fields {
                //     ext_fields.set_field_present_by_index(#i);
                // }
                // fields.set_field_present_by_index(#i);
                #field_encoding
            };
            number_extended_fields += 1;
            field_encodings.push(field_encoding);
        } else {
            field_encoding = quote! {
                // bitfield[#i] = true;
                #field_encoding
            };
            field_encodings.push(field_encoding);
            number_root_fields += 1;
        }
    }

    generics.add_trait_bounds(crate_root, quote::format_ident!("Encode"));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_impl = if config.delegate {
        let ty = &container.fields.iter().next().unwrap().ty;

        if let Some(tag) = config.tag.as_ref().filter(|tag| tag.is_explicit()) {
            let tag = tag.to_tokens(crate_root);
            let encode = quote!(encoder.encode_explicit_prefix(#tag, &self.0).map(drop));
            if is_option_type(ty) {
                quote! {
                    if &self.0.is_some() {
                        #encode
                    }
                }
            } else {
                encode
            }
        } else {
            quote!(
                match tag {
                    #crate_root::types::Tag::EOC => {
                        self.0.encode(encoder)
                    }
                    _ => {
                        <#ty as #crate_root::Encode>::encode_with_tag_and_constraints(
                            &self.0,
                            encoder,
                            tag,
                            <#ty as #crate_root::AsnType>::CONSTRAINTS.override_constraints(constraints)
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

        let field_count = container.fields.len();
        let encode_impl = quote! {
            // Bitmap of the presence of optional/default fields
            // let mut bitfield: [bool; #field_count] = [false; #field_count];
            // let mut fields: [Field; #field_count] = [Self::FIELDS.fields, Self::EXTENDED_FIELDS.fields];
            // let mut extended_fields = Self::EXTENDED_FIELDS;
            // let mut root_bitfield: [bool; #number_optional_default_fields] = [false; #number_optional_default_fields];
            // Bitmap of the presence of extension fields
            // let mut extension_bitfield: [bool; #number_extended_fields] = [false; #number_extended_fields];
            // let extended_count: usize = Self::EXTENDED_FIELDS.as_ref().map_or(0, |fields| fields.len());
            encoder.#operation::<#number_root_fields, #number_extended_fields, Self, _>(tag, |encoder| {
                #(#field_encodings; encoder.update_index();)*

                // encoder.set_presence_bits::<{#number_root_fields}, #number_extended_fields>(fields, extended_fields);
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
            fn encode_with_tag_and_constraints<'constraints, EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::types::Tag, constraints: #crate_root::types::Constraints<'constraints>) -> core::result::Result<(), EN::Error> {
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
