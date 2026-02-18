use crate::{AsnType, types::Tag};

/// A newtype wrapper that will explicitly tag its value with `T`'s tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Explicit<T, V> {
    _tag: core::marker::PhantomData<T>,
    /// The inner value.
    pub value: V,
}

/// A newtype wrapper that will implicitly tag its value with `T`'s tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Implicit<T, V> {
    _tag: core::marker::PhantomData<T>,
    /// The inner value.
    pub value: V,
}

macro_rules! tag_kind {
    ($($name:ident),+) => {
        $(

            impl<T, V> $name<T, V>{
                /// Create a wrapper from `value`.
                pub fn new(value: V) -> Self {
                    Self {
                        value,
                        _tag: core::marker::PhantomData,
                    }
                }
            }

            impl<T, V> From<V> for $name<T, V> {
                fn from(value: V) -> Self {
                    Self::new(value)
                }
            }

            impl<T, V> core::ops::Deref for $name<T, V> {
                type Target = V;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl<T, V> core::ops::DerefMut for $name<T, V> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }
        )+
    }
}

tag_kind!(Implicit, Explicit);

impl<T: AsnType, V> AsnType for Implicit<T, V> {
    const TAG: Tag = T::TAG;
}

impl<T: AsnType, V> AsnType for Explicit<T, V> {
    const TAG: Tag = T::TAG;
}
