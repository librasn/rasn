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
            const TAG: $crate::rasn::types::Tag = <$network_type as $crate::rasn::AsnType>::TAG;
            const IDENTIFIER: $crate::rasn::types::Identifier = <$network_type as $crate::rasn::AsnType>::IDENTIFIER;
        }

        impl $crate::ObjectType for $name {
            type SmiSyntax = $crate::v2::ObjectSyntax;
            type Syntax = $network_type;
            const ACCESS: $crate::Access = $crate::Access::$access_variant;
            const STATUS: $crate::Status = $crate::Status::$status_variant;
            const VALUE: &'static $crate::rasn::types::Oid =
                $crate::rasn::types::Oid::const_new(&$const_oid);
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
            type Error = $crate::rasn::error::DecodeError;

            fn try_from(value: $crate::v2::Opaque) -> Result<Self, Self::Error> {
                $crate::rasn::ber::decode(value.as_ref())
            }
        }

        impl core::convert::TryFrom<$name> for $crate::v2::Opaque {
            type Error = $crate::rasn::error::EncodeError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                use $crate::v1::ToOpaque;
                value.to_opaque()
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag_and_constraints<'encoder, EN: $crate::rasn::Encoder<'encoder>>(
                &self,
                encoder: &mut EN,
                tag: $crate::rasn::types::Tag,
                constraints: $crate::rasn::types::Constraints,
                identifier: Identifier,
            ) -> Result<(), EN::Error> {
                self.to_opaque()
                    .map_err(|e| $crate::rasn::error::EncodeError::opaque_conversion_failed(e.to_string(), encoder.codec()))?
                    .encode_with_tag_and_constraints(encoder, tag, constraints, identifier)
            }
        }
        impl $crate::rasn::Decode for $name {
            fn decode_with_tag_and_constraints<D: $crate::rasn::Decoder>(
                decoder: &mut D,
                tag: $crate::rasn::types::Tag,
                constraints: $crate::rasn::types::Constraints,
            ) -> Result<Self, D::Error> {
                $crate::v2::Opaque::decode_with_tag_and_constraints(decoder, tag, constraints).and_then(|opaque| {
                    let decoder = &mut $crate::rasn::ber::de::Decoder::new(opaque.as_ref(), $crate::rasn::ber::de::DecoderOptions::ber());

                    Ok($crate::opaque_impls! { @decode(decoder) $($decode)+ })
                })
            }
        }
    };
    (@decode ($decoder:ident) ($ty:ty)) => {
        Self(<$ty as $crate::rasn::Decode>::decode($decoder)?)
    };
    (@decode ($decoder:ident) { $($field: ident : $ty:ty),* }) => {
        Self {
            $(
                $field: <$ty as $crate::rasn::Decode>::decode($decoder)?
            ),+
        }
    };
}

/// A declarative macro for generating SMI objects. This macro accepts a list
/// of statements preceeded by a struct-like definition
/// ```
/// rasn_smi::object_type! {
///     /// A textual description of the entity.  This value should include the
///     /// full name and version identification of the system's hardware type,
///     /// software operating-system, and networking software.
///     #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
///     pub struct Descr(pub rasn::types::OctetString);
///     access: ReadOnly,
///     status: Current,
///     value = [1, 3, 6, 1, 2, 1, 1, 1];
/// }
/// ```
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
            fn decode_with_tag_and_constraints<D: $crate::rasn::Decoder>(decoder: &mut D, tag: $crate::rasn::types::Tag, constraints: $crate::rasn::types::Constraints) -> Result<Self, D::Error> {
                use core::convert::TryFrom;

                let syntax = $crate::v2::ObjectSyntax::decode_with_tag_and_constraints(decoder, tag, constraints)?;

                <$typ>::try_from(syntax).map_err(|e|$crate::rasn::de::Error::custom(e, decoder.codec())).map(Self)
            }
        }

        impl $crate::rasn::Encode for $name {
            fn encode_with_tag_and_constraints<'encoder, EN: $crate::rasn::Encoder<'encoder>>(&self, encoder: &mut EN, tag: $crate::rasn::types::Tag, constraints: $crate::rasn::types::Constraints, identifier: $crate::rasn::types::Identifier) -> Result<(), EN::Error> {
                self.0.encode_with_tag_and_constraints(encoder, tag, constraints, identifier)
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
