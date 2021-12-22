macro_rules! from_impls {
    ($(impl $({$($generics:tt)+})? $from:ty => $typ:ty: ($value: pat) -> $variant:expr);+ $(;)?)  => {
        $(
            impl $(<$($generics)+>)? From<$from> for $typ {
                fn from($value: $from) -> Self {
                    $variant
                }
            }
        )+
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! common_impls {
    ($name:ident, $network_type:ty, $access_variant:ident, $status_variant:ident, $const_oid:expr) => {
        impl $crate::rasn::AsnType for $name {
            const TAG: Tag = <$network_type as $crate::rasn::AsnType>::TAG;
        }

        impl $crate::ObjectType for $name {
            type SmiSyntax = $crate::v2::ObjectSyntax;
            type Syntax = $network_type;
            const ACCESS: $crate::Access = $crate::Access::$access_variant;
            const STATUS: $crate::Status = $crate::Status::$status_variant;
            const VALUE: $crate::rasn::types::ConstOid = $crate::rasn::types::ConstOid(&$const_oid);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! delegate_impls {
    ($name:ident, $rust_type:ty) => {
        impl From<$rust_type> for $name {
            fn from(value: $rust_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $rust_type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl core::ops::Deref for $name {
            type Target = $rust_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! opaque_impls {
    ($name:ident $($decode:tt)+) => {
        impl core::convert::TryFrom<$crate::v2::Opaque> for $name {
            type Error = $crate::rasn::ber::de::Error;

            fn try_from(value: $crate::v2::Opaque) -> Result<Self, Self::Error> {
                $crate::rasn::ber::decode(value.as_ref())
            }
        }

        impl core::convert::TryFrom<$name> for $crate::v2::Opaque {
            type Error = $crate::rasn::ber::enc::Error;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                use $crate::v1::ToOpaque;
                value.to_opaque()
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag<EN: $crate::rasn::Encoder>(
                &self,
                encoder: &mut EN,
                tag: Tag,
            ) -> Result<(), EN::Error> {
                self.to_opaque()
                    .map_err($crate::rasn::enc::Error::custom)?
                    .encode_with_tag(encoder, tag)
            }
        }
        impl $crate::rasn::Decode for $name {
            fn decode_with_tag<D: $crate::rasn::Decoder>(
                decoder: &mut D,
                tag: Tag,
            ) -> Result<Self, D::Error> {
                $crate::v2::Opaque::decode_with_tag(decoder, tag).and_then(|opaque| {
                    let decoder = &mut $crate::rasn::ber::de::Decoder::new(opaque.as_ref(), $crate::rasn::ber::de::DecoderOptions::ber());

                    Ok($crate::opaque_impls! { @decode(decoder) $($decode)+ })
                })
            }
        }
    };
    (@decode ($decoder:ident) ($ty:ty)) => {
        Self(<$ty as $crate::rasn::Decode>::decode($decoder).map_err($crate::rasn::de::Error::custom)?)
    };
    (@decode ($decoder:ident) { $($field: ident : $ty:ty),* }) => {
        Self {
            $(
                $field: <$ty as $crate::rasn::Decode>::decode($decoder).map_err($crate::rasn::de::Error::custom)?
            ),+
        }
    };
}

/// A declartive macro for generating SMI objects. This macro accepts a list
/// of statements preceeded by a struct-like definition
#[macro_export]
macro_rules! object_type {
    (
        $(#[$($tree:tt)+])*
        $vi: vis struct $name: ident { $($field_vi:vis $field:ident : $field_ty:ty),* $(,)? }
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        $vi struct $name { $($field_vi $field: $field_ty),+ }

        $crate::common_impls!($name, $crate::v2::Opaque, $access_variant, $status_variant, $const_oid);
        $crate::opaque_impls!($name { $($field : $field_ty),+ });

        $crate::object_type! {
            $($rest)*
        }
    };

    (
        $(#[$($tree:tt)+])*
        $vi: vis struct $name: ident ($fv: vis $typ: ty);
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        #[repr(transparent)]
        pub struct $name($fv $typ);

        $crate::common_impls!($name, $typ, $access_variant, $status_variant,  $const_oid);
        $crate::delegate_impls!($name, $typ);

        impl $crate::rasn::Decode for $name {
            fn decode_with_tag<D: $crate::rasn::Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
                use core::convert::TryFrom;

                let syntax = $crate::v2::ObjectSyntax::decode_with_tag(decoder, tag)?;

                <$typ>::try_from(syntax).map_err($crate::rasn::de::Error::custom).map(Self)
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag<EN: $crate::rasn::Encoder>(&self, encoder: &mut EN, tag: Tag) -> Result<(), EN::Error> {
                self.0.encode_with_tag(encoder, tag)
            }
        }

        $crate::object_type! {
            $($rest)*
        }
    };
    (
        $(#[$($tree:tt)+])*
        $vi: vis opaque struct $name: ident ($fv: vis $typ: ty);
        access: $access_variant: ident,
        status: $status_variant: ident,
        value = $const_oid: expr;

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        #[derive(Clone)]
        #[repr(transparent)]
        pub struct $name($fv $typ);

        $crate::common_impls!($name, $crate::v2::Opaque, $access_variant, $status_variant, $const_oid);
        $crate::delegate_impls!($name, $typ);
        $crate::opaque_impls!($name ($typ));

        $crate::object_type! {
            $($rest)*
        }
    };

    (
        $(#[$($tree:tt)+])*
        impl $header:tt $(for $typ:path)? {
            $($block:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$($tree)+])*
        impl $header $(for $typ)? {
            $($block)*
        }

        $crate::object_type! {
            $($rest)*
        }
    };

    () => {};
}
