use itertools::Itertools;
use quote::ToTokens;

use crate::{config::*, ext::GenericsExt};

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
                    const VARIANT_LIST: &'static [#crate_root::TagTree] = &[#(#field_tags),*];
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
        let return_val = self
            .config
            .tag
            .is_some()
            .then(|| quote!(#crate_root::types::TagTree::Leaf(Self::TAG)))
            .unwrap_or_else(|| quote!(TAG_TREE));

        let (base_variants, extended_variants): (Vec<_>, Vec<_>) = self
            .variants
            .iter()
            .enumerate()
            .partition_map(|(i, variant)| {
                let config = VariantConfig::new(variant, &self.generics, &self.config);
                let tag_tree = config.tag_tree(i);
                match config.extension_addition {
                    false => either::Left(tag_tree),
                    true => either::Right(tag_tree),
                }
            });

        let constraints_def = self.config.constraints.const_static_def(crate_root);

        let choice_impl = self.config.choice.then(|| quote! {
            impl #impl_generics #crate_root::types::Choice for #name #ty_generics #where_clause {
                const VARIANTS: &'static [#crate_root::types::TagTree] = &[
                    #(#base_variants),*
                ];
                const EXTENDED_VARIANTS: &'static [#crate_root::types::TagTree] = &[
                    #(#extended_variants),*
                ];

            }
        });

        let extensible = self.config.constraints.extensible;

        let enumerated_impl = self.config.enumerated.then(|| {
            let (variants, extended_variants): (Vec<_>, Vec<_>) = self.variants.iter()
                .map(|variant| VariantConfig::new(variant, &self.generics, &self.config))
                .partition(|config| !config.extension_addition);

            let discriminants = variants.iter().enumerate().map(|(i, config)| {
                let discriminant = config.discriminant().unwrap_or(i) as isize;
                let variant = &config.variant.ident;
                quote!((Self::#variant, #discriminant))
            });
            let extended_discriminants = extended_variants.iter().enumerate().map(|(i, config)| {
                let discriminant = config.discriminant().unwrap_or(i) as isize;
                let variant = &config.variant.ident;
                quote!((Self::#variant, #discriminant))
            });

            let variants = variants.iter().map(|config| config.variant.ident.clone());
            let extended_variant_idents = extended_variants.iter().map(|config| config.variant.ident.clone());
            let extended_variants = extensible
                .then(|| quote!(Some(&[#(Self::#extended_variant_idents,)*])))
                .unwrap_or(quote!(None));
            let extended_discriminants = (!extended_variants.is_empty())
                .then(|| quote!(Some(&[#(#extended_discriminants,)*])))
                .unwrap_or(quote!(None));

            quote! {
                impl #impl_generics #crate_root::types::Enumerated for #name #ty_generics #where_clause {
                    const VARIANTS: &'static [Self] = &[#(Self::#variants,)*];
                    const EXTENDED_VARIANTS: Option<&'static [Self]> = #extended_variants;

                    const DISCRIMINANTS: &'static [(Self, isize)] = &[#(#discriminants,)*];
                    const EXTENDED_DISCRIMINANTS: Option<&'static [(Self, isize)]> = #extended_discriminants;
                }
            }
        });

        quote! {
            impl #impl_generics #crate_root::AsnType for #name #ty_generics #where_clause {
                const TAG: #crate_root::Tag = {
                    #tag
                };
                const TAG_TREE: #crate_root::TagTree = {
                    const LIST: &'static [#crate_root::TagTree] = &[#tag_tree];
                    const TAG_TREE: #crate_root::TagTree = #crate_root::TagTree::Choice(LIST);
                    const _: () = assert!(TAG_TREE.is_unique(), #error_message);
                    #return_val
                };

                #constraints_def
            }

            #choice_impl
            #enumerated_impl
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
            quote!(decoder.decode_enumerated(tag))
        } else {
            quote!(decoder.decode_explicit_prefix(tag))
        };

        let decode_op = if self.config.choice {
            quote!(decoder.decode_choice(Self::CONSTRAINTS))
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

                    #[derive(#crate_root::AsnType, #crate_root::Decode)]
                    #[rasn(choice)]
                    #inner_type

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

        let decode_choice_impl = self.config.choice.then(|| {
            let decode_ops = self.variants.iter()
                .enumerate()
                .map(|(i, v)| VariantConfig::new(v, &generics, &self.config).decode(&self.name, i));

            let str_name = syn::LitStr::new(&self.name.to_string(), proc_macro2::Span::call_site());
            let from_tag = quote! {
                #(#decode_ops)*

                Err(#crate_root::de::Error::no_valid_choice(#str_name))
            };
            quote! {
                #[automatically_derived]
                impl #impl_generics #crate_root::types::DecodeChoice for #name #ty_generics #where_clause {
                    fn from_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag) -> core::result::Result<Self, D::Error> {
                        use #crate_root::de::Decode;
                        #from_tag
                    }
                }
            }
        });

        quote! {
            #decode_choice_impl

            #[automatically_derived]
            impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
                fn decode_with_tag_and_constraints<'constraints, D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::Tag, constraints: #crate_root::types::Constraints<'constraints>) -> core::result::Result<Self, D::Error> {
                    #decode_with_tag
                }

                #decode_impl
            }
        }
    }

    fn encode_with_tag(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let operation = if self.config.enumerated {
            quote! {
                encoder.encode_enumerated(tag, self).map(drop)
            }
        } else {
            quote! {
                encoder.encode_explicit_prefix(tag, self).map(drop)
            }
        };

        quote! {
            fn encode_with_tag_and_constraints<'constraints, EN: #crate_root::Encoder>(&self, encoder: &mut EN, tag: #crate_root::Tag, constraints: #crate_root::types::Constraints<'constraints>) -> core::result::Result<(), EN::Error> {
                #operation
            }
        }
    }

    fn encode(&self, generics: &syn::Generics) -> Option<proc_macro2::TokenStream> {
        if self.config.choice {
            Some(self.encode_choice(generics))
        } else {
            None
        }
    }

    fn encode_choice(&self, generics: &syn::Generics) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let variants = self.variants.iter().enumerate().map(|(i, v)| {
            let ident = &v.ident;
            let name = &self.name;
            let variant_config = VariantConfig::new(v, generics, &self.config);
            let variant_tag = variant_config.tag(i);

            match &v.fields {
                syn::Fields::Named(_) => {
                    let idents = v.fields.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote!(#ident)
                    });
                    let idents_prefixed = v.fields.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        let ident_prefixed = format_ident!("__rasn_field_{}", ident);
                        quote!(#ident_prefixed)
                    });

                    let tag_tokens = variant_tag.to_tokens(crate_root);
                    let encode_impl = crate::encode::map_to_inner_type(
                        variant_tag,
                        ident,
                        &v.fields,
                        generics,
                        crate_root,
                        variant_config.has_explicit_tag(),
                    );

                    quote!(#name::#ident { #(#idents: #idents_prefixed),* } => { #encode_impl.map(|_| #tag_tokens) })
                }
                syn::Fields::Unnamed(_) => {
                    if v.fields.iter().count() != 1 {
                        panic!("Tuple variants must contain only a single element.");
                    }
                    let constraints = variant_config
                        .constraints
                        .const_expr(&self.config.crate_root);
                    let variant_tag = variant_tag.to_tokens(crate_root);
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, value))
                    } else if variant_config.tag.is_some() || self.config.automatic_tags {
                        if let Some(constraints) = constraints {
                            quote!(#crate_root::Encode::encode_with_tag_and_constraints(value, encoder, #variant_tag, #constraints))
                        } else {
                            quote!(#crate_root::Encode::encode_with_tag(value, encoder, #variant_tag))
                        }
                    } else if let Some(constraints) = constraints {
                            quote!(#crate_root::Encode::encode_with_constraints(value, encoder, #constraints))
                        } else {
                            quote!(#crate_root::Encode::encode(value, encoder))
                    };

                    quote! {
                        #name::#ident(value) => {
                            #encode_operation.map(|_| #variant_tag)
                        }
                    }
                }
                syn::Fields::Unit => {
                    let variant_tag = variant_tag.to_tokens(crate_root);
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, &()))
                    } else {
                        quote!(encoder.encode_null(#variant_tag))
                    };

                    quote!(#name::#ident => #encode_operation.map(|_| #variant_tag))
                }
            }
        });

        let encode_variants = quote! {
            encoder.encode_choice::<Self>(Self::CONSTRAINTS, |encoder| match self {
                #(#variants),*
            })
        };

        let encode_impl = if self.config.has_explicit_tag() {
            let inner_name = quote::format_ident!("Inner{}", self.name);
            let encode_lifetime = syn::Lifetime::new("'inner", proc_macro2::Span::call_site());
            let mut inner_generics = generics.clone();

            inner_generics
                .params
                .push(syn::LifetimeDef::new(encode_lifetime.clone()).into());

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
                #[derive(#crate_root::AsnType, #crate_root::Encode)]
                #[rasn(choice)]
                #inner_enum

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
