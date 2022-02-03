use rasn::prelude::*;

#[derive(AsnType)]
#[rasn(tag(explicit(application, 1)), delegate)]
pub struct ExplicitDelegate(pub u32);

const _: () = assert!(Tag::const_eq(ExplicitDelegate::TAG, &Tag::new(Class::Application, 1)));
