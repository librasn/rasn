use crate::Tag;

/// A newtype wrapper that will explicitly tag its value with `T`'s tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Explicit<const TAG: Tag, V> {
    pub(crate) value: V,
}

/// A newtype wrapper that will implicitly tag its value with `T`'s tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Implicit<const TAG: Tag, V> {
    pub(crate) value: V,
}

macro_rules! tag_kind {
    ($($name:ident),+) => {
        $(

            impl<const TAG: Tag, V> $name<TAG, V>{
                /// Create a new implicitly wrapped value.
                pub fn new(value: V) -> Self {
                    Self { value, }
                }
            }

            impl<const TAG: Tag, V> From<V> for $name<TAG, V>
            {
                fn from(value: V) -> Self {
                    Self::new(value)
                }
            }

            impl<const TAG: Tag, V> core::ops::Deref for $name<TAG, V> {
                type Target = V;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl<const TAG: Tag, V> core::ops::DerefMut for $name<TAG, V> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }
        )+
    }
}

tag_kind!(Implicit, Explicit);
