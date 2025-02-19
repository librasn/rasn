use itertools::Itertools;
use proc_macro2::Span;
use quote::ToTokens;
use syn::LitStr;

use crate::config::*;

pub struct Enum<'a> {
    pub name: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    pub config: &'a Config,
}

impl Enum<'_> {
    #[allow(clippy::too_many_lines)]
    pub fn impl_asntype(&self) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.config.crate_root;

        let tag = self.config.tag.as_ref().map_or_else(
            || {
                self.config
                    .enumerated
                    .then(|| quote!(#crate_root::types::Tag::ENUMERATED))
                    .unwrap_or(quote!(#crate_root::types::Tag::EOC))
            },
            |t| t.to_tokens(crate_root),
        );

        let error_message = format!(
            "{}'s variants is not unique, ensure that your variants' tags are correct.",
            self.name
        );

        let variant_configs: Vec<_> = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| VariantConfig::new(v, self.generics, self.config, i))
            .collect::<Result<Vec<_>, _>>()?;

        let field_tags = if self.config.choice {
            variant_configs
                .iter()
                .map(|config| config.tag_tree())
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };

        let tag_tree = if self.config.choice {
            quote! {
                {
                    let variant_list: &'static [#crate_root::types::TagTree] = const { &[#(#field_tags),*] };
                    let variant_tag_tree: #crate_root::types::TagTree = #crate_root::types::TagTree::Choice(variant_list);
                    assert!(variant_tag_tree.is_unique(), #error_message);
                    variant_tag_tree
                }
            }
        } else {
            quote!(#crate_root::types::TagTree::Leaf(#tag))
        };

        let name = &self.name;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let return_val = self
            .config
            .tag
            .is_some()
            .then(|| quote!(#crate_root::types::TagTree::Leaf(Self::TAG)))
            .unwrap_or_else(|| quote!(tag_tree));

        let identifiers = variant_configs
            .iter()
            .map(|v| {
                v.identifier.clone().unwrap_or_else(|| {
                    syn::LitStr::new(&v.variant.ident.to_string(), proc_macro2::Span::call_site())
                })
            })
            .collect_vec();

        let constraints_def = self.config.constraints.const_static_def(crate_root);

        let (base_variants, extended_variants): (Vec<_>, Vec<_>) = variant_configs
            .iter()
            .zip(field_tags)
            .partition_map(|(config, field_tag)| {
                if config.extension_addition {
                    either::Right(field_tag)
                } else {
                    either::Left(field_tag)
                }
            });

        let extensible = self.config.constraints.extensible;
        let extended_const_variants = extensible
            .then(|| quote!(Some(&[#(#extended_variants),*])))
            .unwrap_or(quote!(None));

        // Check count of the root components in the choice
        // https://github.com/XAMPPRocky/rasn/issues/168
        // Choice index starts from zero, so we need to reduce variance by one
        let variant_count = if self.variants.is_empty() {
            0
        } else {
            self.variants.len() - 1
        };

        let variance_constraint = Constraints {
            extensible: false,
            from: None,
            size: None,
            value: Some(Constraint::from(Value::Range(
                Some(0),
                Some(variant_count as i128),
            ))),
        }
        .const_expr(crate_root);

        let choice_impl = self.config.choice.then(|| quote! {
            impl #impl_generics #crate_root::types::Choice for #name #ty_generics #where_clause {
                const VARIANTS: &'static [#crate_root::types::TagTree] = &[
                    #(#base_variants),*
                ];
                const VARIANCE_CONSTRAINT: #crate_root::types::Constraints = #variance_constraint;
                const EXTENDED_VARIANTS: Option<&'static [#crate_root::types::TagTree]> = #extended_const_variants;
                const IDENTIFIERS: &'static [&'static str] = &[
                    #(#identifiers),*
                ];
            }
        });

        let enumerated_impl = self.config.enumerated.then(|| {
            let (variants, extended_variants): (Vec<_>, Vec<_>) = variant_configs.iter()
                .partition(|config| !config.extension_addition);

            let discriminants = variants.iter().enumerate().map(|(i, config)| {
                let discriminant = config.discriminant().unwrap_or(i as isize);
                let variant = &config.variant.ident;
                quote!((Self::#variant, #discriminant))
            });
            let extended_discriminants = extended_variants.iter().enumerate().map(|(i, config)| {
                let discriminant = config.discriminant().unwrap_or(i as isize);
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

                    const IDENTIFIERS: &'static [&'static str] = &[
                        #(#identifiers),*
                    ];
                }
            }
        });

        let alt_identifier = self.config.identifier.as_ref().map_or(
            quote!(),
            |id| quote!(const IDENTIFIER: Option<&'static str> = Some(#id);),
        );

        Ok(quote! {
            impl #impl_generics #crate_root::AsnType for #name #ty_generics #where_clause {
                const TAG: #crate_root::types::Tag = {
                    #tag
                };
                const TAG_TREE: #crate_root::types::TagTree = const {
                    let list: &'static [#crate_root::types::TagTree] = const {
                        &[#tag_tree]
                    };
                    let tag_tree: #crate_root::types::TagTree = #crate_root::types::TagTree::Choice(list);
                    assert!(tag_tree.is_unique(), #error_message);
                    #return_val
                };
                #alt_identifier
                #constraints_def
            }

            #choice_impl
            #enumerated_impl
        })
    }

    pub fn impl_encode(&mut self) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.config.crate_root;

        let name = &self.name;
        let encode = if self.config.choice {
            Some(self.encode_choice()?)
        } else {
            None
        };
        let encode_with_tag = self.encode_with_tag();
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        Ok(quote! {
            #[automatically_derived]
            impl #impl_generics #crate_root::Encode for #name #ty_generics #where_clause {
                #encode
                #encode_with_tag
            }
        })
    }

    pub fn impl_decode(&self) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.config.crate_root;

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
            let variants = self.variants.clone();
            let inner_type = InnerEnum {
                name: inner_name,
                generics: self.generics.clone(),
                variants,
            };
            let inner_name = &inner_type.name;

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
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let decode_choice_impl = if self.config.choice {
            let decode_ops: Vec<proc_macro2::TokenStream> = self
                .variants
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    VariantConfig::new(v, self.generics, self.config, i)
                        .map(|config| config.decode(self.name))
                })
                .collect::<Result<Result<Vec<_>, _>, _>>()??;

            let str_name = syn::LitStr::new(&self.name.to_string(), proc_macro2::Span::call_site());
            let from_tag = quote! {
                #(#decode_ops)*

                Err(#crate_root::de::Error::no_valid_choice(#str_name, decoder.codec()))
            };
            Some(quote! {
                #[automatically_derived]
                impl #impl_generics #crate_root::types::DecodeChoice for #name #ty_generics #where_clause {
                    fn from_tag<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::types::Tag) -> core::result::Result<Self, D::Error> {
                        use #crate_root::de::Decode;
                        #from_tag
                    }
                }
            })
        } else {
            None
        };

        Ok(quote! {
            #decode_choice_impl

            #[automatically_derived]
            impl #impl_generics #crate_root::Decode for #name #ty_generics #where_clause {
                fn decode_with_tag_and_constraints<D: #crate_root::Decoder>(decoder: &mut D, tag: #crate_root::types::Tag, constraints: #crate_root::types::Constraints) -> core::result::Result<Self, D::Error> {
                    #decode_with_tag
                }

                #decode_impl
            }
        })
    }

    fn encode_with_tag(&self) -> proc_macro2::TokenStream {
        let crate_root = &self.config.crate_root;
        let operation = if self.config.enumerated {
            quote! {
                encoder.encode_enumerated(tag, self, identifier).map(drop)
            }
        } else {
            quote! {
                encoder.encode_explicit_prefix(tag, self, identifier).map(drop)
            }
        };

        quote! {
            fn encode_with_tag_and_constraints<'encoder, EN: #crate_root::Encoder<'encoder>>(&self, encoder: &mut EN, tag: #crate_root::types::Tag, constraints: #crate_root::types::Constraints, identifier: Option<&'static str>) -> core::result::Result<(), EN::Error> {
                #operation
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn encode_choice(&self) -> syn::Result<proc_macro2::TokenStream> {
        let crate_root = &self.config.crate_root;

        let (variant_configs, (variant_tags, tag_tokens)) = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let variant_config = VariantConfig::new(v, self.generics, self.config, i)?;
                let variant_tag = variant_config.tag()?;
                let tag_tokens = variant_tag.to_tokens(crate_root);
                Ok((variant_config, (variant_tag, tag_tokens)))
            })
            .collect::<syn::Result<(Vec<_>, (Vec<_>, Vec<_>))>>()?;

        let tags = self
            .variants
            .iter()
            .zip(tag_tokens.iter())
            .map(|(v, tag_tokens)| {
                let name = &self.name;
                let ident = &v.ident;

                match &v.fields {
                    syn::Fields::Named(_) => quote!(#name::#ident { .. } => #tag_tokens),
                    syn::Fields::Unnamed(_) => quote!(#name::#ident (_) => #tag_tokens),
                    syn::Fields::Unit => quote!(#name::#ident => #tag_tokens),
                }
            });
        let mut variant_constraints: Vec<proc_macro2::TokenStream> = vec![];
        let variants = self.variants.iter().enumerate().map(|(i, v)| {
            let ident = &v.ident;
            let name = &self.name;
            let variant_config = &variant_configs[i];
            let variant_tag = &variant_tags[i];
            let variant_identifier =  variant_config
            .identifier
            .clone()
            .unwrap_or(LitStr::new(&variant_config.variant.ident.to_string(), Span::call_site()));

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
                    let encode_impl: proc_macro2::TokenStream = crate::encode::map_to_inner_type(
                        variant_tag,
                        ident,
                        &v.fields,
                        self.generics,
                        crate_root,
                        variant_config.has_explicit_tag(),
                        variant_identifier,
                    );

                    quote!(#name::#ident { #(#idents: #idents_prefixed),* } => { #encode_impl.map(|_| #tag_tokens) })
                }
                syn::Fields::Unnamed(_) => {
                    // Assert already checked in FieldConfig
                    assert_eq!(v.fields.len(), 1, "Tuple variants must contain only a single element.");
                    let constraints = variant_config
                        .constraints
                        .const_expr(&self.config.crate_root);
                    let constraint_name = format_ident!("VARIANT_CONSTRAINT_{}", i);
                    let variant_tag = &tag_tokens[i];
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, value, Some(#variant_identifier)))
                    } else if variant_config.tag.is_some() || self.config.automatic_tags {
                        if let Some(constraints) = constraints {
                            variant_constraints.push(quote! {
                                const #constraint_name: #crate_root::types::constraints::Constraints = #constraints;
                            });
                            quote!(#crate_root::Encode::encode_with_tag_and_constraints(value, encoder, #variant_tag, #constraint_name, Some(#variant_identifier)))
                        } else {
                            quote!(#crate_root::Encode::encode_with_tag_and_identifier(value, encoder, #variant_tag, Some(#variant_identifier)))
                        }
                    } else if let Some(constraints) = constraints {
                            variant_constraints.push(quote! {
                                const #constraint_name: #crate_root::types::constraints::Constraints = #constraints;
                            });
                            quote!(#crate_root::Encode::encode_with_constraints_and_identifier(value, encoder, #constraint_name, Some(#variant_identifier)))
                        } else {
                            quote!(#crate_root::Encode::encode_with_identifier(value, encoder, #variant_identifier))
                    };

                    quote! {
                        #name::#ident(value) => {
                            #encode_operation.map(|_| #variant_tag)
                        }
                    }
                }
                syn::Fields::Unit => {
                    let variant_tag = &tag_tokens[i];
                    let encode_operation = if variant_config.has_explicit_tag() {
                        quote!(encoder.encode_explicit_prefix(#variant_tag, &(), Some(#variant_identifier)))
                    } else {
                        quote!(encoder.encode_null(#variant_tag, Some(#variant_identifier)))
                    };

                    quote!(#name::#ident => #encode_operation.map(|_| #variant_tag))
                }
            }
        });

        let encode_variants = quote! {
            encoder.encode_choice::<Self>(
                Self::CONSTRAINTS,
                match self {
                    #(#tags),*
                },
                |encoder| match self {
                    #(#variants),*
                },
                Self::IDENTIFIER,
            )
        };

        let encode_impl = if self.config.has_explicit_tag() {
            let inner_name = quote::format_ident!("Inner{}", self.name);
            let encode_lifetime = syn::Lifetime::new("'inner", proc_macro2::Span::call_site());
            let mut inner_generics = self.generics.clone();
            let inner_identifier = self
                .config
                .identifier
                .clone()
                .unwrap_or(LitStr::new(&self.name.to_string(), Span::call_site()));

            inner_generics
                .params
                .push(syn::LifetimeParam::new(encode_lifetime.clone()).into());

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

            let inner_enum = InnerEnum {
                name: inner_name,
                generics: inner_generics,
                variants,
            };
            let inner_name = &inner_enum.name;
            let init_variants = self.variants.iter().map(|variant| {
                let ident = &variant.ident;
                let (def_fields, init_fields) = variant.fields.iter().enumerate().map(|(i, field)| {
                    let index = syn::Index::from(i);
                    let ident = field.ident.as_ref().map_or_else(|| format_ident!("i{}", index).into_token_stream(), |ident| quote!(#ident));
                    let name = field.ident.as_ref().map_or_else(|| index.into_token_stream(), |ident| quote!(#ident));

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

                encoder.encode_explicit_prefix::<#inner_name>(<#name as #crate_root::AsnType>::TAG, &value, Some(#inner_identifier))
            }
        } else {
            encode_variants
        };

        Ok(quote! {
            fn encode<'encoder, E: #crate_root::Encoder<'encoder>>(&self, encoder: &mut E) -> core::result::Result<(), E::Error> {
                #(#variant_constraints)*
                #encode_impl.map(drop)
            }
        })
    }
}

impl ToTokens for Enum<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        let generics = self.generics;
        tokens.extend(quote! {
            enum #name #generics { #variants }
        });
    }
}

struct InnerEnum {
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    generics: syn::Generics,
}

impl ToTokens for InnerEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        let generics = &self.generics;
        tokens.extend(quote! {
            enum #name #generics { #variants }
        });
    }
}
