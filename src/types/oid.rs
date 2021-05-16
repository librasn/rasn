use alloc::vec::Vec;
use core::ops;

const fn is_valid_oid(slice: &[u32]) -> bool {
    slice.len() >= 2 && slice[0] < 2
}

/// A temporary workaround for [`Oid`] not currently being `const` compatible.
pub struct ConstOid(&'static [u32]);

impl AsRef<[u32]> for ConstOid {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}

impl ops::Deref for ConstOid {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Oid> for ConstOid {
    fn as_ref(&self) -> &Oid {
        unsafe { Oid::new_unchecked(self.0) }
    }
}

/// A reference to a global unique identifier that identifies an concept, such
/// as a organisation, or encoding rules.
#[derive(Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Oid([u32]);

impl Oid {
    pub const ITU_T: ConstOid = ConstOid(&[0]);
    pub const ISO: ConstOid = ConstOid(&[1]);
    pub const JOINT_ISO_ITU_T: ConstOid = ConstOid(&[2]);
    pub const ISO_IDENTIFIED_ORGANISATION: ConstOid = ConstOid(&[1, 3]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD: ConstOid = ConstOid(&[1, 3, 6]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET: ConstOid = ConstOid(&[1, 3, 6, 1]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY: ConstOid =
        ConstOid(&[1, 3, 6, 1, 1]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT: ConstOid = ConstOid(&[1, 3, 6, 1, 2]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_EXPERIMENTAL: ConstOid =
        ConstOid(&[1, 3, 6, 1, 3]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE: ConstOid =
        ConstOid(&[1, 3, 6, 1, 4]);
    pub const ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES: ConstOid =
        ConstOid(&[1, 3, 6, 1, 3, 1]);

    /// Creates a new reference to a object identifier from `slice`.
    ///
    /// Returns `None` if `vec` contains less than two components or the first
    /// component is greater than 1.
    /// ```
    /// use rasn::types::Oid;
    ///
    /// let internet = Oid::new(&[1, 3, 6, 1]).unwrap();
    /// ```
    pub fn new(slice: &[u32]) -> Option<&Oid> {
        if is_valid_oid(slice) {
            Some(unsafe { Self::new_unchecked(slice) })
        } else {
            None
        }
    }

    /// Creates a new mutable reference to a object identifier from `slice`.
    ///
    /// Returns `None` if `vec` contains less than two components or the first
    /// component is greater than 1.
    /// ```
    /// use rasn::types::Oid;
    ///
    /// let internet = Oid::new(&[1, 3, 6, 1]).unwrap();
    /// ```
    pub fn new_mut(slice: &mut [u32]) -> Option<&mut Oid> {
        if is_valid_oid(slice) {
            Some(unsafe { Self::new_unchecked_mut(slice) })
        } else {
            None
        }
    }

    /// Creates a new reference to a object identifier from `slice`.
    ///
    /// # Safety
    /// This allows you to create potentially invalid object identifiers which
    /// may affect encoding validity.
    pub unsafe fn new_unchecked(slice: &[u32]) -> &Oid {
        &*(slice as *const [u32] as *const Oid)
    }

    /// Creates a new object identifier from `slice`.
    ///
    /// # Safety
    /// This allows you to create potentially invalid object identifiers which
    /// may affect encoding validity.
    pub unsafe fn new_unchecked_mut(slice: &mut [u32]) -> &mut Oid {
        &mut *(slice as *mut [u32] as *mut Oid)
    }
}

impl alloc::borrow::ToOwned for Oid {
    type Owned = ObjectIdentifier;

    fn to_owned(&self) -> Self::Owned {
        Self::Owned::new_unchecked(self.0.to_owned())
    }
}

impl AsRef<[u32]> for Oid {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}

impl PartialEq<[u32]> for Oid {
    fn eq(&self, rhs: &[u32]) -> bool {
        &self.0 == rhs
    }
}

impl<const N: usize> PartialEq<[u32; N]> for Oid {
    fn eq(&self, rhs: &[u32; N]) -> bool {
        &self.0 == rhs
    }
}

impl PartialEq<Oid> for [u32] {
    fn eq(&self, rhs: &Oid) -> bool {
        self == &rhs.0
    }
}

impl<const N: usize> PartialEq<Oid> for [u32; N] {
    fn eq(&self, rhs: &Oid) -> bool {
        self == &rhs.0
    }
}

impl ops::Deref for Oid {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Oid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A global unique identifier that identifies an concept, such as a
/// organisation, or encoding rules. The "owned" version of [`Oid`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObjectIdentifier(Vec<u32>);

impl ObjectIdentifier {
    /// Creates a new object identifier from `vec`.
    ///
    /// Returns `None` if `vec` contains less than two components or the first
    /// component is greater than 1.
    pub fn new(vec: Vec<u32>) -> Option<Self> {
        is_valid_oid(&vec).then(|| Self(vec))
    }

    /// Creates a new object identifier from `vec`.
    ///
    /// # Safety
    /// This allows you to create potentially invalid object identifiers which
    /// may affect encoding validity.
    pub const fn new_unchecked(vec: Vec<u32>) -> Self {
        Self(vec)
    }
}

impl AsRef<[u32]> for ObjectIdentifier {
    fn as_ref(&self) -> &[u32] {
        self.0.as_ref()
    }
}

impl alloc::borrow::Borrow<Oid> for ObjectIdentifier {
    fn borrow(&self) -> &Oid {
        &*self
    }
}

impl<'a> From<&'a Oid> for ObjectIdentifier {
    fn from(oid: &'a Oid) -> Self {
        alloc::borrow::ToOwned::to_owned(oid)
    }
}

impl ops::Deref for ObjectIdentifier {
    type Target = Oid;

    fn deref(&self) -> &Self::Target {
        unsafe { Oid::new_unchecked(&self.0) }
    }
}

impl ops::DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Oid::new_unchecked_mut(&mut self.0) }
    }
}

impl<const N: usize> PartialEq<ObjectIdentifier> for [u32; N] {
    fn eq(&self, rhs: &ObjectIdentifier) -> bool {
        self == &**rhs
    }
}

impl PartialEq<[u32]> for ObjectIdentifier {
    fn eq(&self, rhs: &[u32]) -> bool {
        &*self == rhs
    }
}

#[cfg(test)]
mod test {
    use super::ObjectIdentifier;

    #[test]
    fn transmute() {
        let mut oid = ObjectIdentifier::new_unchecked(alloc::vec![1, 3, 6]);

        assert_eq!([1u32, 3, 6][..], *oid);
        oid.reverse();
        assert_eq!([6u32, 3, 1][..], *oid);
    }
}
