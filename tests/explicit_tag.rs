use rasn::{AsnType, Decode, Encode, types::{SetOf, OctetString}};

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct PresentationAddress {
    #[rasn(tag(0))]
    pub p_selector: Option<OctetString>,
    #[rasn(tag(explicit(1)))]
    pub s_selector: Option<OctetString>,
    #[rasn(tag(explicit(2)))]
    pub t_selector: Option<OctetString>,
    #[rasn(tag(explicit(3)))]
    pub n_addresses: SetOf<OctetString>,
}


