use alloc::vec::Vec;
use core::ops;

pub(crate) const MAX_OID_FIRST_OCTET: u32 = 2;
pub(crate) const MAX_OID_SECOND_OCTET: u32 = 39;

const fn is_valid_oid(slice: &[u32]) -> bool {
    slice.len() >= 2 && slice[0] <= MAX_OID_FIRST_OCTET && slice[1] <= MAX_OID_SECOND_OCTET
}

/// A temporary workaround for [`Oid`] not currently being `const` compatible.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstOid(pub &'static [u32]);

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
        Oid::new_unchecked(self.0)
    }
}

impl PartialEq<[u32]> for ConstOid {
    fn eq(&self, rhs: &[u32]) -> bool {
        self.0 == rhs
    }
}

impl PartialEq<Oid> for ConstOid {
    fn eq(&self, rhs: &Oid) -> bool {
        *self.0 == rhs.0
    }
}

/// A reference to a global unique identifier that identifies an concept, such
/// as a organisation, or encoding rules.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Oid([u32]);

impl Oid {
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
            Some(Self::new_unchecked(slice))
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
            Some(Self::new_unchecked_mut(slice))
        } else {
            None
        }
    }

    /// Creates a new reference to a object identifier from `slice`.
    ///
    /// # Safety
    /// This allows you to create potentially invalid object identifiers which
    /// may affect encoding validity.
    pub fn new_unchecked(slice: &[u32]) -> &Oid {
        unsafe { &*(slice as *const [u32] as *const Oid) }
    }

    /// Creates a new object identifier from `slice`.
    ///
    /// # Safety
    /// This allows you to create potentially invalid object identifiers which
    /// may affect encoding validity.
    pub fn new_unchecked_mut(slice: &mut [u32]) -> &mut Oid {
        unsafe { &mut *(slice as *mut [u32] as *mut Oid) }
    }
}

macro_rules! oids {
    ($($name:ident => $($num:literal),+ $(,)?);+ $(;)?) => {
        impl Oid {
            $(
                pub const $name: ConstOid = ConstOid(&[$($num),+]);
            )+
        }
    }
}

// ITU-T object identifiers
oids! {
    ITU_T => 0;
    ITU_T_DATA_PSS_UCL_PILOT => 0, 9, 2342, 19200300, 100;
    ITU_T_DATA_PSS_UCL_PILOT_ATTRIBUTE_TYPE => 0, 9, 2342, 19200300, 100, 1;
}

// ISO object identifiers
oids! {
    ISO => 1;
    ISO_IDENTIFIED_ORGANISATION => 1, 3;
    ISO_IDENTIFIED_ORGANISATION_DOD => 1, 3, 6;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET => 1, 3, 6, 1;

    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY => 1, 3, 6, 1, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_DIRECTORY_X509 => 1, 3, 6, 1, 1, 15;

    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT => 1, 3, 6, 1, 2;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB => 1, 3, 6, 1, 2, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_SYSTEM => 1, 3, 6, 1, 2, 1, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_INTERFACES => 1, 3, 6, 1, 2, 1, 2;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_AT => 1, 3, 6, 1, 2, 1, 3;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_IP => 1, 3, 6, 1, 2, 1, 4;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_ICMP => 1, 3, 6, 1, 2, 1, 5;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_TCP => 1, 3, 6, 1, 2, 1, 6;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_UDP => 1, 3, 6, 1, 2, 1, 7;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_EGP => 1, 3, 6, 1, 2, 1, 8;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_CMOT => 1, 3, 6, 1, 2, 1, 9;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_TRANSMISSION => 1, 3, 6, 1, 2, 1, 10;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_SNMP => 1, 3, 6, 1, 2, 1, 11;

    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_EXPERIMENTAL => 1, 3, 6, 1, 3;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE => 1, 3, 6, 1, 4;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES => 1, 3, 6, 1, 3, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_WALL => 1, 3, 6, 1, 3, 1, 1466;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_WALL_DYN_EXT => 1, 3, 6, 1, 3, 1, 1466, 101, 119;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_WALL_ATTR => 1, 3, 6, 1, 3, 1, 1466, 101, 120;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_WALL_MATCH => 1, 3, 6, 1, 3, 1, 1466, 109, 114;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_WALL_SYNTAX => 1, 3, 6, 1, 3, 1, 1466, 115, 121, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_OPEN_LDAP_LDAP => 1, 3, 6, 1, 3, 1, 4203, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_OPEN_LDAP_LDAP_ATTRIBUTES => 1, 3, 6, 1, 3, 1, 4203, 1, 3;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_PRIVATE_ENTERPRISES_OPEN_LDAP_LDAP_CONTROLS => 1, 3, 6, 1, 3, 1, 4203, 1, 10;

    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SECURITY => 1, 3, 6, 1, 5;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2 => 1, 3, 6, 1, 6;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_DOMAINS => 1, 3, 6, 1, 6, 1;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_PROXIES => 1, 3, 6, 1, 6, 2;
    ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_SNMP_V2_MODULES => 1, 3, 6, 1, 6, 3;
}

// Joint ISO-ITU-T object identifiers
oids! {
    JOINT_ISO_ITU_T => 2;

    JOINT_ISO_ITU_T_DS => 2, 5;
    JOINT_ISO_ITU_T_DS_MODULE => 2, 5, 1;

    JOINT_ISO_ITU_T_DS_MODULE_USEFUL_DEFINITIONS => 2, 5, 1, 0, 8;
    JOINT_ISO_ITU_T_DS_MODULE_INFORMATION_FRAMEWORK => 2, 5, 1, 1, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_ABSTRACT_SERVICE => 2, 5, 1, 2, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DISTRIBUTED_OPERATIONS => 2, 5, 1, 3, 8;
    JOINT_ISO_ITU_T_DS_MODULE_PROTOCOL_OBJECT_IDENTIFIERS => 2, 5, 1, 4, 8;
    JOINT_ISO_ITU_T_DS_MODULE_SELECTED_ATTRIBUTE_TYPES => 2, 5, 1, 5, 8;
    JOINT_ISO_ITU_T_DS_MODULE_SELECTED_OBJECT_CLASSES => 2, 5, 1, 6, 8;
    JOINT_ISO_ITU_T_DS_MODULE_AUTHENTICATION_FRAMEWORK => 2, 5, 1, 7, 8;
    JOINT_ISO_ITU_T_DS_MODULE_ALGORITHM_OBJECT_IDENTIFIERS => 2, 5, 1, 8, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_OBJECT_IDENTIFIERS => 2, 5, 1, 9, 8;
    JOINT_ISO_ITU_T_DS_MODULE_UPPER_BOUNDS => 2, 5, 1, 10, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DAP => 2, 5, 1, 11, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DSP => 2, 5, 1, 12, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DISTRIBUTED_DIRECTORY_OIDS => 2, 5, 1, 13, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_SHADOW_OIDS => 2, 5, 1, 14, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_SHADOW_ABSTRACT_SERVICE => 2, 5, 1, 15, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DISP => 2, 5, 1, 16, 7;
    JOINT_ISO_ITU_T_DS_MODULE_DOP => 2, 5, 1, 17, 7;
    JOINT_ISO_ITU_T_DS_MODULE_OP_BINDING_MANAGEMENT => 2, 5, 1, 18, 8;
    JOINT_ISO_ITU_T_DS_MODULE_OP_BINDING_OIDS => 2, 5, 1, 19, 8;
    JOINT_ISO_ITU_T_DS_MODULE_HIERARCHICAL_OPERATIONAL_BINDINGS => 2, 5, 1, 20, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DSA_OPERATIONAL_ATTRIBUTE_TYPES => 2, 5, 1, 22, 8;
    JOINT_ISO_ITU_T_DS_MODULE_SCHEMA_ADMINISTRATION => 2, 5, 1, 23, 8;
    JOINT_ISO_ITU_T_DS_MODULE_BASIC_ACCESS_CONTROL => 2, 5, 1, 24, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_OPERATIONAL_BINDING_TYPES => 2, 5, 1, 25, 8;
    JOINT_ISO_ITU_T_DS_MODULE_CERTIFICATE_EXTENSIONS => 2, 5, 1, 26, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_MANAGEMENT => 2, 5, 1, 27, 8;
    JOINT_ISO_ITU_T_DS_MODULE_ENHANCED_SECURITY => 2, 5, 1, 28, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_SECURITY_EXCHANGES => 2, 5, 1, 29, 8;
    JOINT_ISO_ITU_T_DS_MODULE_IDM_PROTOCOL_SPECIFICATION => 2, 5, 1, 30, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_IDM_PROTOCOLS => 2, 5, 1, 31, 8;
    JOINT_ISO_ITU_T_DS_MODULE_ATTRIBUTE_CERTIFICATE_DEFINITIONS => 2, 5, 1, 32, 8;
    JOINT_ISO_ITU_T_DS_MODULE_SERVICE_ADMINISTRATION => 2, 5, 1, 33, 8;
    JOINT_ISO_ITU_T_DS_MODULE_LDAP_ATTRIBUTES => 2, 5, 1, 34, 8;
    JOINT_ISO_ITU_T_DS_MODULE_COMMON_PROTOCOL_SPECIFICATION => 2, 5, 1, 35, 8;
    JOINT_ISO_ITU_T_DS_MODULE_OSI_PROTOCOL_SPECIFICATION => 2, 5, 1, 36, 8;
    JOINT_ISO_ITU_T_DS_MODULE_DIRECTORY_OSI_PROTOCOLS => 2, 5, 1, 37, 8;
    JOINT_ISO_ITU_T_DS_MODULE_LDAP_SYSTEM_SCHEMA => 2, 5, 1, 38, 8;
    JOINT_ISO_ITU_T_DS_MODULE_PASSWORD_POLICY => 2, 5, 1, 39, 8;
    JOINT_ISO_ITU_T_DS_MODULE_PKI_PMI_EXTERNAL_DATA_TYPES => 2, 5, 1, 40, 8;
    JOINT_ISO_ITU_T_DS_MODULE_EXTENSION_ATTRIBUTES => 2, 5, 1, 41, 8;
    JOINT_ISO_ITU_T_DS_MODULE_PKI_PMI_WRAPPER => 2, 5, 1, 42, 8;
    JOINT_ISO_ITU_T_DS_MODULE_AVL_MANAGEMENT => 2, 5, 1, 43, 8;
    JOINT_ISO_ITU_T_DS_MODULE_TRUST_BROKER_PROTOCOL => 2, 5, 1, 44, 8;

    JOINT_ISO_ITU_T_DS_SERVICE_ELEMENT => 2, 5, 2;
    JOINT_ISO_ITU_T_DS_APPLICATION_CONTEXT => 2, 5, 3;
    JOINT_ISO_ITU_T_DS_ATTRIBUTE_TYPE => 2, 5, 4;
    JOINT_ISO_ITU_T_DS_SYNTAX_VENDOR => 2, 5, 5;
    JOINT_ISO_ITU_T_DS_OBJECT_CLASS => 2, 5, 6;
    JOINT_ISO_ITU_T_DS_ATTRIBUTE_SET => 2, 5, 7;
    JOINT_ISO_ITU_T_DS_ALGORITHM => 2, 5, 8;
    JOINT_ISO_ITU_T_DS_ABSTRACT_SYNTAX => 2, 5, 9;
    JOINT_ISO_ITU_T_DS_OBJECT => 2, 5, 10;
    JOINT_ISO_ITU_T_DS_PORT => 2, 5, 11;
    JOINT_ISO_ITU_T_DS_DSA_OPERATIONAL_ATTRIBUTE => 2, 5, 12;
    JOINT_ISO_ITU_T_DS_MATCHING_RULE => 2, 5, 13;
    JOINT_ISO_ITU_T_DS_KNOWLEDGE_MATCHING_RULE => 2, 5, 14;
    JOINT_ISO_ITU_T_DS_NAME_FORM => 2, 5, 15;
    JOINT_ISO_ITU_T_DS_GROUP => 2, 5, 16;
    JOINT_ISO_ITU_T_DS_SUBENTRY => 2, 5, 17;
    JOINT_ISO_ITU_T_DS_OPERATIONAL_ATTRIBUTE_TYPE => 2, 5, 18;
    JOINT_ISO_ITU_T_DS_OPERATIONAL_BINDING => 2, 5, 19;
    JOINT_ISO_ITU_T_DS_SCHEMA_OBJECT_CLASS => 2, 5, 20;
    JOINT_ISO_ITU_T_DS_SCHEMA_OPERATIONAL_ATTRIBUTE => 2, 5, 21;
    JOINT_ISO_ITU_T_DS_ADMINISTRATIVE_ROLES => 2, 5, 23;
    JOINT_ISO_ITU_T_DS_ACCESS_CONTROL_ATTRIBUTE => 2, 5, 24;
    JOINT_ISO_ITU_T_DS_ROS_OBJECT => 2, 5, 25;
    JOINT_ISO_ITU_T_DS_CONTRACT => 2, 5, 26;
    JOINT_ISO_ITU_T_DS_PACKAGE => 2, 5, 27;
    JOINT_ISO_ITU_T_DS_ACCESS_CONTROL_SCHEMES => 2, 5, 28;
    JOINT_ISO_ITU_T_DS_CERTIFICATE_EXTENSION => 2, 5, 29;
    JOINT_ISO_ITU_T_DS_MANAGEMENT_OBJECT => 2, 5, 30;
    JOINT_ISO_ITU_T_DS_ATTRIBUTE_VALUE_CONTEXT => 2, 5, 31;
    JOINT_ISO_ITU_T_DS_SECURITY_EXCHANGE => 2, 5, 32;
    JOINT_ISO_ITU_T_DS_IDM_PROTOCOL => 2, 5, 33;
    JOINT_ISO_ITU_T_DS_PROBLEM => 2, 5, 34;
    JOINT_ISO_ITU_T_DS_NOTIFICATION => 2, 5, 35;
    JOINT_ISO_ITU_T_DS_MATCHING_RESTRICTION => 2, 5, 36;
    JOINT_ISO_ITU_T_DS_CONTROL_ATTRIBUTE_TYPE => 2, 5, 37;
    JOINT_ISO_ITU_T_DS_KEY_PURPOSES => 2, 5, 38;
    JOINT_ISO_ITU_T_DS_PASSWORD_QUALITY => 2, 5, 39;
    JOINT_ISO_ITU_T_DS_ATTRIBUTE_SYNTAX => 2, 5, 40;
    JOINT_ISO_ITU_T_DS_AV_RESTRICTION => 2, 5, 41;
    JOINT_ISO_ITU_T_DS_CMS_CONTENT_TYPE => 2, 5, 42;

    JOINT_ISO_ITU_T_REGISTRATION_PROCEDURES_MODULE_DIRECTORY_DEFS => 2, 17, 1, 2;
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
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
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
        Oid::new_unchecked(&self.0)
    }
}

impl ops::DerefMut for ObjectIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Oid::new_unchecked_mut(&mut self.0)
    }
}

impl<const N: usize> PartialEq<ObjectIdentifier> for [u32; N] {
    fn eq(&self, rhs: &ObjectIdentifier) -> bool {
        self == &**rhs
    }
}

impl PartialEq<ObjectIdentifier> for Oid {
    fn eq(&self, rhs: &ObjectIdentifier) -> bool {
        self == &**rhs
    }
}

impl PartialEq<ObjectIdentifier> for ConstOid {
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
