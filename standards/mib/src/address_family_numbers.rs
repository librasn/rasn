//! Internet assigned address family numbers.
use rasn::{AsnType, Decode, Encode};

/// The MIB definition of the internet assigned address family numbers. This is
/// encoded as an `INTEGER`.
#[derive(AsnType, Debug, Decode, Encode, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[rasn(delegate)]
pub struct AddressFamilyNumbers(pub u16);

impl AddressFamilyNumbers {
    /// None of the the rest.
    pub const NONE: Self = Self(0);
    /// IP version 4.
    pub const IPV4: Self = Self(1);
    /// IP version 6.
    pub const IPV6: Self = Self(2);
    /// NSAP
    pub const NSAP: Self = Self(3);
    /// 8-bit multidrop
    pub const HDLC: Self = Self(4);
    pub const BBN_1822: Self = Self(5);
    /// Includes all 802 media plus Ethernet 'conanical format'.
    pub const ALL_802: Self = Self(6);
    pub const E163: Self = Self(7);
    /// SMDS, Frame Relay, ATM
    pub const E164: Self = Self(8);
    /// Telex
    pub const F69: Self = Self(9);
    /// X.25, Frame Relay
    pub const X121: Self = Self(10);
    /// Internet Protocol Exchange.
    pub const IPX: Self = Self(11);
    /// Apple Talk.
    pub const APPLE_TALK: Self = Self(12);
    /// DEC Net Phase IV.
    pub const DEC_NET_IV: Self = Self(13);
    /// Banyan Vines.
    pub const BANYAN_VINES: Self = Self(14);
    /// E.164 with NSAP format subaddress.
    pub const E164_WITH_NSAP: Self = Self(15);
    /// Domain Name System.
    pub const DNS: Self = Self(16);
    /// Distinguished Name.
    pub const DISTINGUISHED_NAME: Self = Self(17);
    /// 16-bit quantity, per the AS number space.
    pub const AS_NUMBER: Self = Self(18);
    /// XTP over IP version 4
    pub const XTP_OVER_IPV4: Self = Self(19);
    /// XTP over IP version 6
    pub const XTP_OVER_IPV6: Self = Self(20);
    /// XTP native mode XTP
    pub const XTP_NATIVE_MODE_XTP: Self = Self(21);
    /// Fibre Channel World-Wide Port Name
    pub const FIBRE_CHANNEL_WWPN: Self = Self(22);
    /// Fibre Channel World-Wide Node Name
    pub const FIBRE_CHANNEL_WWNN: Self = Self(23);
    /// Gateway Identifier
    pub const GWID: Self = Self(24);
    /// AFI for L2VPN information
    pub const AFI: Self = Self(25);
    /// MPLS-TP Section Endpoint Identifier
    pub const MPLS_TP_SECTION_ENDPOINT_IDENTIFIER: Self = Self(26);
    /// MPLS-TP LSP Endpoint Identifier
    pub const MPLS_TP_LSP_ENDPOINT_IDENTIFIER: Self = Self(27);
    /// MPLS-TP Pseudowire Endpoint Identifier
    pub const MPLS_TP_PSEUDOWIRE_ENDPOINT_IDENTIFIER: Self = Self(28);
    /// MT IP: Multi-Topology IP version 4
    pub const MT_IP_MULTI_TOPOLOGY_IP_VERSION_4: Self = Self(29);
    /// MT IPv6: Multi-Topology IP version 6
    pub const MT_IPV6_MULTI_TOPOLOGY_IP_VERSION_6: Self = Self(30);
    /// BGP SFC
    pub const BGP_SFC: Self = Self(31);
    /// EIGRP Common Service Family
    pub const EIGRP_COMMON_SERVICE_FAMILY: Self = Self(16384);
    /// EIGRP IPv4 Service Family
    pub const EIGRP_IPV4_SERVICE_FAMILY: Self = Self(16385);
    /// EIGRP IPv6 Service Family
    pub const EIGRP_IPV6_SERVICE_FAMILY: Self = Self(16386);
    /// LISP Canonical Address Format (LCAF)
    pub const LISP_CANONICAL_ADDRESS_FORMAT: Self = Self(16387);
    /// BGP-LS
    pub const BGP_LS: Self = Self(16388);
    /// 48-bit MAC
    pub const FORTY_EIGHT_BIT_MAC: Self = Self(16389);
    /// 64-bit MAC
    pub const SIXTY_FOUR_BIT_MAC: Self = Self(16390);
    /// OUI
    pub const OUI: Self = Self(16391);
    /// MAC/24
    pub const MAC_24: Self = Self(16392);
    /// MAC/40
    pub const MAC_40: Self = Self(16393);
    /// IPv6/64
    pub const IPV6_64: Self = Self(16394);
    /// RBridge Port ID
    pub const R_BRIDGE_PORT_ID: Self = Self(16395);
    /// TRILL Nickname
    pub const TRILL_NICKNAME: Self = Self(16396);
    /// Universally Unique Identifier (UUID)
    pub const UNIVERSALLY_UNIQUE_IDENTIFIER: Self = Self(16397);
    /// Routing Policy AFI
    pub const ROUTING_POLICY_AFI: Self = Self(16398);
    /// Reserved for future use.
    pub const RESERVED: Self = Self(65535);
}
