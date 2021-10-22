use crate::{config::*, ext::GenericsExt};
use quote::ToTokens;

pub struct Enum {
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub variants: syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    pub config: Config,
}

impl Enum {
    pub fn impl_asntype(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;

        let tag = self
            .config
            .tag
            .as_ref()
            .map(|t| t.to_tokens(crate_root))
            .unwrap_or_else(|| {
                self.config
                    .enumerated
                    .then(|| quote!(#crate_root::Tag::ENUMERATED))
                    .unwrap_or(quote!(#crate_root::Tag::EOC))
            });

        let error_message = format!(
            "{}'s variants is not unique, ensure that your variants's tags are correct.",
            self.name
        );

        let tag_tree = if self.config.choice {
            let field_tags = self
                .variants
                .iter()
                .enumerate()
                .map(|(i, v)| VariantConfig::new(v, &self.generics, &self.config).tag_tree(i));

            quote! {
                {
                    const VARIANT_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags)*];
                    const VARIANT_TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(VARIANT_LIST);
                    const _: () = assert!(VARIANT_TAG_TREE.is_unique(), #error_message);
                    VARIANT_TAG_TREE
                }
            }
        } else {
            quote!(#crate_root::TagTree::Leaf(#tag))
        };

        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics #crate_root::AsnType for #name #ty_generics #where_clause {
                const TAG: #crate_root::Tag = {
                    #tag
                };
                const TAG_TREE: #crate_root::TagTree = {
                    const LIST: &'static [#crate_root::TagTree] = &[#tag_tree];
                    const TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(LIST);
                    const _: () = assert!(TAG_TREE.is_unique(), #error_message);
                    TAG_TREE
                };
            }
        }
    }

    pub fn impl_encode(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let mut generics = self.generics.clone();
        generics.add_trait_bounds(crate_root, quote::format_ident!("Encode"));

        let name = &self.name;
        let encode = self.encode(&generics);
        let encode_with_tag = self.encode_with_tag();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl #impl_generics #crate_root::Encode for #name #ty_generics #where_clause {
                #encode
                #encode_with_tag
            }
        }
    }

    pub fn impl_decode(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let mut generics = self.generics.clone();
        generics.add_trait_bounds(&self.config.crate_root, quote::format_ident!("Decode"));
        let decode_with_tag = if self.config.enumerated {
            let variants = self.variants.iter().map(|v| {
                let ident = &v.ident;
                quote!(i if i == (Self::#ident as isize).into() => Self::#ident,)
            });

            let length = variants.clone().count();

            quote! {
                let integer = decoder.decode_enumerated(
                    tag,
                    #crate_root::types::constraints::Constraints::from(&[#crate_root::types::constraints::Range::new(0usize, #length).into()])
                )?;

                Ok(match integer {
                    #(#variants)*
                    _ => return Err(#crate_root::de::Error::custom("Invalid enumerated disrciminant."))
                })

            }
        } else {
            quote!(decoder.decode_explicit_prefix(tag))
        };

        let decode_op = if self.config.choice {
            let variants =
                self.variants.iter().enumerate().map(|(i, v)| {
                    VariantConfig::new(v, &generics, &self.config).decode(&self.name, i)
                });

            let name = syn::LitStr::new(&self.name.to_string(), proc_macro2::Span::call_site());
            quote! {
                #(#variants);*
                Err(#crate_root::de::Error::no_valid_choice(#name))
            }
        } else {
            let name = &self.name;
            quote!(Self::decode_with_tag(decoder, <#name as #crate_root::AsnType>::TAG))
        };

        let decode_impl = if self.config.has_explicit_tag() {
            let inner_name = quote::format_ident!("Inner{}", self.name);
            let generics = self.generics.clone();
            let variants = self.variants.clone();
            let mut config = self.config.clone();
            config.tag = None;
            let inner_type = Self {
                name: inner_name.clone(),
                generics,
                variants,
                config,
            };
            let asntype = inner_type.impl_asntype();
            let decode_impl = inner_type.impl_decode();

            let variant_mapping = inner_type.variants.iter().map(|variant| {
                let ident = &variant.ident;

                let fields = variant.fields.iter().enumerate().map(|(i, field)| {
                    field
                        .ident
                        .clone()
                        .unwrap_or_else(|| quote::format_ident!("i{}", i))
                });

                let fields = match variant.fields {
                    syn::Fields::Named(_) => quote!({ #(#fields),* }),
                    syn::Fields::Unnamed(_) => quote!(( #(#fields),* )),
                    syn::Fields::Unit => quote!(),
                };

                let name = &self.name;
                quote!(#inner_name::#ident #fields => #name::#ident #fields)
            });

            quote! {
                fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> core::result::Result<Self, D::Error> {
                    #inner_type
                    #asntype
                    #decode_impl

                    let value = decoder.decode_explicit_prefix::<#inner_name>(<Self as #crate_root::AsnType>::TAG)?;
                    Ok(match value {
                        #(#variant_mapping),*
                    })
                }
            }
        } else {
            quote! {
                fn decode<D: #crate_root::Decoder>(decoder: &mut D) -> core::result::Result<Self, D::Error> {
                    #decode_op
                }
            }
        };

        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! {
            #[automatically_derived]
            impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
                fn decode_with_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> core::result::Result<Self, D::Error> {
                    #decode_with_tag
                }

                #decode_impl
            }
        }
    }

    fn encode_with_tag(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let operation = if self.config.enumerated {
            let variance = self.variants.len();
            quote! {
                encoder.encode_enumerated(tag, #variance, *self as isize).map(drop)
            }
        } else {
            quote! {
                encoder.encode_explicit_prefix(tag, self).map(drop)
            }
        };

        quote! {
            fn encode_with_tag<EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag) -> core::result::Result<(), EN::Error> {
                #operation
            }
        }
    }

    fn encode(&self, generics: &syn::Generics) -> Option<proc_macro2::TokenStream> {
        if self.config.choice {
            Some(self.encode_choice(&generics))
        } else {
            None
        }
    }

    fn encode_choice(&self, generics: &syn::Generics) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let variants = self.variants.iter().enumerate().map(|(i, v)| {
            let ident = &v.ident;
            let name = &self.name;
            let variant_config = VariantConfig::new(v, &generics, &self.config);
            let variant_tag = variant_config.tag(i);

            match &v.fields {
                syn::Fields::Named(_) => {
                    let idents = v.fields.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote!(#ident)
                    });

                    let fields = v.fields.iter().enumerate().map(|(i, f)| {
                        let field_config = FieldConfig::new(f, &self.config);
                        field_config.encode(i, false)
                    });

                    let encode_impl = |tag| {
                        quote! {
                            encoder.encode_sequence(#tag, <_>::default(), |encoder| {
                                #(#fields)*
                                Ok(())
                            }).map(drop)
                        }
                    };

                    let encode_impl = if variant_config.has_explicit_tag() {
                        crate::encode::map_to_inner_type(
                            quote!(#variant_tag),
                            &ident,
                            &v.fields,
                            &generics,
                            &crate_root,
                            (encode_impl)(quote!(#crate_root::Tag::SEQUENCE)),
                        )
                    } else {
                        (encode_impl)(variant_tag)
                    };

                    quote!(#name::#ident { #(#idents),* } => { #encode_impl })
                }
                syn::Fields::Unnamed(_) => {
                    if v.fields.iter().count() != 1 {
                        panic!("Tuple variants must contain only a single element.");
                    }
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, value))
                    } else if variant_config.tag.is_some() || self.config.automatic_tags {
                        quote!(#crate_root::Encode::encode_with_tag(value, encoder, #variant_tag))
                    } else {
                        quote!(#crate_root::Encode::encode(value, encoder))
                    };

                    quote! {
                        #name::#ident(value) => {
                            #encode_operation.map(drop)
                        }
                    }
                }
                syn::Fields::Unit => {
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, &()))
                    } else {
                        quote!(encoder.encode_null(#variant_tag))
                    };

                    quote!(#name::#ident => #encode_operation.map(drop))
                }
            }
        });

        let encode_variants = quote! {
            match self {
                #(#variants),*
            }
        };

        let encode_impl = if self.config.has_explicit_tag() {
            let inner_name = quote::format_ident!("Inner{}", self.name);
            let encode_lifetime = syn::Lifetime::new("'inner", proc_macro2::Span::call_site());
            let mut inner_generics = generics.clone();

            inner_generics
                .params
                .push(syn::LifetimeDef::new(encode_lifetime.clone().into()).into());

            let variants = self
                .variants
                .iter()
                .cloned()
                .map(|mut variant| {
                    for field in variant.fields.iter_mut() {
                        field.ty = syn::Type::Reference(syn::TypeReference {
                            and_token: <_>::default(),
                            lifetime: encode_lifetime.clone().into(),
                            mutability: None,
                            elem: Box::from(field.ty.clone()),
                        });
                    }

                    variant
                })
                .collect();

            let mut inner_config = self.config.clone();
            inner_config.tag = None;
            let inner_enum = Self {
                name: inner_name,
                generics: inner_generics,
                variants,
                config: inner_config,
            };
            let impl_asntype = inner_enum.impl_asntype();
            let impl_encode = inner_enum.impl_encode();
            let inner_name = &inner_enum.name;
            let init_variants = self.variants.iter().map(|variant| {
                let ident = &variant.ident;
                let (def_fields, init_fields) = variant.fields.iter().enumerate().map(|(i, field)| {
                    let index = syn::Index::from(i);
                    let ident = field.ident.as_ref().map(|ident| quote!(#ident))
                        .unwrap_or_else(|| format_ident!("i{}", index).into_token_stream());
                    let name = field.ident.as_ref().map(|ident| quote!(#ident))
                        .unwrap_or_else(|| index.into_token_stream());

                    (quote!(#name : ref #ident), quote!(#name : #ident))
                }).unzip::<_, _, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>>();

                quote!(Self::#ident { #(#def_fields),* } => #inner_name::#ident { #(#init_fields),* })
            });

            let name = &self.name;
            quote! {
                #inner_enum
                #impl_asntype
                #impl_encode

                let value = match &self {
                    #(#init_variants),*
                };

                encoder.encode_explicit_prefix::<#inner_name>(<#name as #crate_root::AsnType>::TAG, &value)
            }
        } else {
            encode_variants
        };

        quote! {
            fn encode<E: #crate_root::Encoder>(&self, encoder: &mut E) -> core::result::Result<(), E::Error> {
                #encode_impl.map(drop)
            }
        }
    }
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        let generics = &self.generics;
        tokens.extend(quote! {
            enum #name #generics { #variants }
        });
    }
}
