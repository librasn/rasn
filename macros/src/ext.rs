pub trait TypeExt {
    fn strip_lifetimes(&mut self);
}

impl TypeExt for syn::Type {
    fn strip_lifetimes(&mut self) {
        match self {
            syn::Type::Reference(ref mut reference) => {
                reference.lifetime = None;
            }
            _ => {}
        }
    }
}

pub trait GenericsExt {
    fn add_trait_bounds(&mut self, crate_root: &syn::Path, r#trait: syn::Ident);
}

impl GenericsExt for syn::Generics {
    fn add_trait_bounds(&mut self, crate_root: &syn::Path, ident: syn::Ident) {
        for param in self.type_params_mut() {
            param.colon_token = Some(Default::default());
            param.bounds = {
                let mut punct = syn::punctuated::Punctuated::new();
                punct.push(
                    syn::TraitBound {
                        paren_token: None,
                        modifier: syn::TraitBoundModifier::None,
                        lifetimes: None,
                        path: {
                            let mut path = crate_root.clone();
                            path.segments.push(syn::PathSegment {
                                ident: ident.clone(),
                                arguments: syn::PathArguments::None,
                            });

                            path
                        },
                    }
                    .into(),
                );

                punct
            };
        }
    }
}
