#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
use alloc::string::ToString;
pub mod address_family_numbers;

use alloc::{vec, vec::Vec};

use rasn::types::*;
use smi::{object_type, v2::*};

/// Used to model textual information taken from the NVT ASCII character set. By
/// convention, objects with this syntax are declared as having `SIZE (0...255)`
pub type DisplayString = OctetString;
/// This data type is used to model media addresses.  For many types of media,
/// this will be in a binary representation. For example, an ethernet address
/// would be represented as a string of 6 octets.
pub type PhysAddress = OctetString;

pub const MIB: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB;
pub const SYSTEM: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_SYSTEM;
pub const INTERFACES: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_INTERFACES;
pub const AT: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_AT;
pub const IP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_IP;
pub const ICMP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_ICMP;
pub const TCP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_TCP;
pub const UDP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_UDP;
pub const EGP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_EGP;
pub const TRANSMISSION: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_TRANSMISSION;
pub const SNMP: &Oid = Oid::ISO_IDENTIFIED_ORGANISATION_DOD_INTERNET_MGMT_MIB_SNMP;

/// The System Group
///
/// Implementation of the System group is mandatory for all systems. If an agent
/// is not configured to have a value for any of these variables, a string of
/// length 0 is returned.
pub mod system {
    use super::*;

    object_type! {
        /// A textual description of the entity.  This value should include the
        /// full name and version identification of the system's hardware type,
        /// software operating-system, and networking software.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Descr(pub OctetString);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 1];

        /// The vendor's authoritative identification of the network management
        /// subsystem contained in the entity. This value is allocated within
        /// the SMI enterprises subtree (1.3.6.1.4.1) and provides an easy and
        /// unambiguous means for determining `what kind of box' is being
        /// managed. For example, if vendor "Flintstones, Inc." was assigned the
        /// subtree `1.3.6.1.4.1.4242`, it could assign the identifier
        /// `1.3.6.1.4.1.4242.1.1` to its `Fred Router'.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ObjectId(pub ObjectName);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 2];

        /// The time (in hundredths of a second) since the network management
        /// portion of the system was last re-initialized.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct UpTime(pub TimeTicks);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 3];

        /// The textual identification of the contact person for this managed
        /// node, together with information on how to contact this person. If no
        /// contact information is known, the value is the zero-length string.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Contact(pub DisplayString);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 4];

        /// An administratively-assigned name for this managed node. By
        /// convention, this is the node's fully-qualified domain name.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Name(pub DisplayString);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 5];

        /// The physical location of this node (e.g., "telephone closet,
        /// 3rd floor").
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Location(pub DisplayString);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 6];

        /// A value which indicates the set of services that this entity
        /// primarily offers.
        ///
        /// The value is a sum.  This sum initially takes the value zero, Then,
        /// for each layer, **L**, in the range 1 through 7, that this node
        /// performs transactions for, 2 raised to (**L** - 1) is added to the
        /// sum.  For example, a node which performs primarily routing functions
        /// would have a value of 4 `2^(3-1)`. In contrast, a node which is a
        /// host offering application services would have a value of 72
        /// `2^(4-1) + 2^(7-1)`. Note that in the context of the Internet suite
        /// of protocols, values should be calculated accordingly:
        ///
        /// | layer | functionality                       |
        /// | ----- | ----------------------------------- |
        /// | 1     | physical (e.g., repeaters)          |
        /// | 2     | datalink/subnetwork (e.g., bridges) |
        /// | 3     | internet (e.g., IP gateways)        |
        /// | 4     | end-to-end  (e.g., IP hosts)        |
        /// | 7     | applications (e.g., mail relays)    |
        ///
        /// For systems including OSI protocols, layers 5 and 6 may also
        /// be counted.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Services(pub u8);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 7];

        /// The value of [`system::UpTime`] at the time of the most recent
        /// change in state or value of any instance of sysORID.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrLastChange(pub TimeTicks);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 8];

        /// The (conceptual) table listing the capabilities of the local SNMP
        /// application acting as a command responder with respect to various
        /// MIB modules. SNMP entities having dynamically-configurable support
        /// of MIB modules will have a dynamically-varying number of
        /// conceptual rows.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct OrTable(pub Vec<OrEntry>);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9];

        /// An entry in the [`OrTable`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrEntry {
            index: OrIndex,
            id: OrId,
            descr: OrDescr,
            up_time: OrUpTime,
        }
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9, 1];

        /// The auxiliary variable used for identifying instances of the
        /// columnar objects in the [`OrTable`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrIndex(pub u32);
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9, 1];

        /// The auxiliary variable used for identifying instances of the
        /// columnar objects in the [`OrTable`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrId(pub ObjectIdentifier);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9, 2];

        /// A textual description of the capabilities identified by the
        /// corresponding instance of [`OrId`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrDescr(pub DisplayString);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9, 3];

        /// The value of [`system::UpTime`] at the time this conceptual row was
        /// last instantiated.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OrUpTime(pub TimeTicks);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 1, 9, 4];
    }
}

/// The Interfaces Group
///
/// Implementation of the Interfaces group is mandatory for all systems.

pub mod interfaces {
    use super::*;

    /// An interface entry containing objects at the subnetwork layer and
    /// below for a particular interface.
    #[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
    pub struct Entry {
        pub index: Index,
        pub descr: Descr,
        pub r#type: Type,
        pub mtu: Mtu,
        pub speed: Speed,
        pub phys_address: PhysAddress,
        pub admin_status: AdminStatus,
        pub oper_status: OperStatus,
        pub last_change: LastChange,
        pub in_octets: InOctets,
        pub in_ucast_pkts: InUcastPkts,
        pub in_n_ucast_pkts: InNUcastPkts,
        pub in_discards: InDiscards,
        pub in_errors: InErrors,
        pub in_unknown_protos: InUnknownProtos,
        pub out_octets: OutOctets,
        pub out_ucast_pkts: OutUcastPkts,
        pub out_n_ucast_pkts: OutNUcastPkts,
        pub out_discards: OutDiscards,
        pub out_errors: OutErrors,
        pub out_q_len: OutQLen,
        pub specific: Specific,
    }

    smi::common_impls!(
        Entry,
        Opaque,
        ReadWrite,
        Current,
        [1, 3, 6, 1, 2, 1, 2, 2, 1]
    );

    impl core::convert::TryFrom<Opaque> for Entry {
        type Error = rasn::error::DecodeError;

        fn try_from(value: Opaque) -> Result<Self, Self::Error> {
            rasn::ber::decode(value.as_ref())
        }
    }

    impl core::convert::TryFrom<Entry> for Opaque {
        type Error = rasn::error::EncodeError;

        fn try_from(value: Entry) -> Result<Self, Self::Error> {
            value.to_opaque()
        }
    }

    impl rasn::Decode for Entry {
        fn decode_with_tag_and_constraints<D: rasn::Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            Opaque::decode_with_tag_and_constraints(decoder, tag, constraints).and_then(|opaque| {
                let decoder = &mut rasn::ber::de::Decoder::new(
                    opaque.as_ref(),
                    rasn::ber::de::DecoderOptions::ber(),
                );

                Ok(Self {
                    index: <_>::decode(decoder)?,
                    descr: <_>::decode(decoder)?,
                    r#type: <_>::decode(decoder)?,
                    mtu: <_>::decode(decoder)?,
                    speed: <_>::decode(decoder)?,
                    phys_address: <_>::decode(decoder)?,
                    admin_status: <_>::decode(decoder)?,
                    oper_status: <_>::decode(decoder)?,
                    last_change: <_>::decode(decoder)?,
                    in_octets: <_>::decode(decoder)?,
                    in_ucast_pkts: <_>::decode(decoder)?,
                    in_n_ucast_pkts: <_>::decode(decoder)?,
                    in_discards: <_>::decode(decoder)?,
                    in_errors: <_>::decode(decoder)?,
                    in_unknown_protos: <_>::decode(decoder)?,
                    out_octets: <_>::decode(decoder)?,
                    out_ucast_pkts: <_>::decode(decoder)?,
                    out_n_ucast_pkts: <_>::decode(decoder)?,
                    out_discards: <_>::decode(decoder)?,
                    out_errors: <_>::decode(decoder)?,
                    out_q_len: <_>::decode(decoder)?,
                    specific: <_>::decode(decoder).unwrap_or_default(),
                })
            })
        }
    }

    impl rasn::Encode for Entry {
        fn encode_with_tag_and_constraints<EN: rasn::Encoder>(
            &self,
            encoder: &mut EN,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<(), EN::Error> {
            self.to_opaque()
                .map_err(|e| {
                    rasn::error::EncodeError::opaque_conversion_failed(
                        e.to_string(),
                        encoder.codec(),
                    )
                })?
                .encode_with_tag_and_constraints(encoder, tag, constraints)
        }
    }

    object_type! {
        /// The number of network interfaces (regardless of their current state)
        /// present on this system.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Number(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 1];

        /// A list of interface entries.  The number of entries is given by the
        /// value of [`Number`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct Table(pub Vec<Entry>);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2];

        /// A unique value for each interface. Its value ranges between 1 and
        /// the value of [`Number`]. The value for each interface must remain
        /// constant at least from one re-initialization of the entity's network
        /// management system to the next re-initialization.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Index(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 1];

        /// A textual string containing information about the interface. This
        /// string should include the name of the manufacturer, the product name
        /// and the version of the hardware interface.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Descr(pub DisplayString);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 2];

        /// The type of interface, distinguished according to the physical/link
        /// protocol(s) immediately "below" the network layer in the
        /// protocol stack.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Type(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 3];

        impl Type {
            pub const OTHER: u64 = 1;
            pub const REGULAR1822: u64 = 2;
            pub const HDH1822: u64 = 3;
            pub const DDN_X25: u64 = 4;
            pub const RFC887_X25: u64 = 5;
            pub const ETHERNET_CSMACD: u64 = 6;
            pub const ISO88023_CSMACD: u64 = 7;
            pub const ISO88024_TOKEN_BUS: u64 = 8;
            pub const ISO88025_TOKEN_RING: u64 = 9;
            pub const ISO88026_MAN: u64 = 10;
            pub const STAR_LAN: u64 = 11;
            pub const PROTEON_10MBIT: u64 = 12;
            pub const PROTEON_80MBIT: u64 = 13;
            pub const HYPERCHANNEL: u64 = 14;
            pub const FDDI: u64 = 15;
            pub const LAPB: u64 = 16;
            pub const SDLC: u64 = 17;
            pub const T1_CARRIER: u64 = 18;
            pub const CEPT: u64 = 19;
            pub const BASIC_ISDN: u64 = 20;
            pub const PRIMARY_ISDN: u64 = 21;
            pub const PROP_POINT_TO_POINT_SERIAL: u64 = 22;
            pub const PPP: u64 = 23;
            pub const SOFTWARE_LOOPBACK: u64 = 24;
            pub const EON: u64 = 25;
            pub const ETHERNET_3MBIT: u64 = 26;
            pub const NSIP: u64 = 27;
            pub const SLIP: u64 = 28;
            pub const ULTRA: u64 = 29;
            pub const DS3: u64 = 30;
            pub const SIP: u64 = 31;
            pub const FRAME_RELAY: u64 = 32;
        }

        /// The size of the largest datagram which can be sent/received on the
        /// interface, specified in octets. For interfaces that are used for
        /// transmitting network datagrams, this is the size of the largest
        /// network datagram that can be sent on the interface.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Mtu(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 4];

        /// An estimate of the interface's current bandwidth in bits per second.
        /// For interfaces which do not vary in bandwidth or for those where no
        /// accurate estimation can be made, this object should contain the
        /// nominal bandwidth.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Speed(pub Gauge32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 5];

        /// The interface's address at the protocol layer immediately "below"
        /// the network layer in the protocol stack.  For interfaces which do
        /// not have such an address (e.g., a serial line), this object should
        /// contain an octet string of zero length.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PhysAddress(pub super::PhysAddress);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 6];

        /// The desired state of the interface. The [`Self::TESTING`] state
        /// indicates that no operational packets can be passed.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdminStatus(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 7];

        impl AdminStatus {
            pub const UP: u64 = 1;
            pub const DOWN: u64 = 2;
            pub const TESTING: u64 = 3;
        }

        /// The current operational state of the interface. The
        /// [`Self::TESTING`] state indicates that no operational packets can
        /// be passed.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OperStatus(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 8];

        impl OperStatus {
            pub const UP: u64 = 1;
            pub const DOWN: u64 = 2;
            pub const TESTING: u64 = 3;
        }

        /// The value of [`system::UpTime`] at the time the interface entered
        /// its current operational state.  If the current state was entered
        /// prior to the last re-initialization of the local network management
        /// subsystem, then this object contains a zero value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct LastChange(pub TimeTicks);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 9];

        /// The total number of octets received on the interface, including
        /// framing characters.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InOctets(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 10];

        /// The number of subnetwork-unicast packets delivered to a
        /// higher-layer protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUcastPkts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 11];

        /// The number of non-unicast (i.e., subnetwork-broadcast or
        /// subnetwork-multicast) packets delivered to a higher-layer protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InNUcastPkts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 12];

        /// The number of inbound packets which were chosen to be discarded even
        /// though no errors had been detected to prevent their being
        /// deliverable to a higher-layer protocol. One possible reason for
        /// discarding such a packet could be to free up buffer space.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDiscards(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 13];

        /// The number of inbound packets that contained errors preventing them
        /// from being deliverable to a higher-layer protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 14];

        /// The number of packets received via the interface which were
        /// discarded because of an unknown or unsupported protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUnknownProtos(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 15];

        /// The total number of octets transmitted out of the interface,
        /// including framing characters.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutOctets(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 16];

        /// The total number of packets that higher-level protocols requested be
        /// transmitted to a subnetwork-unicast address, including those that
        /// were discarded or not sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutUcastPkts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 17];

        /// The total number of packets that higher-level protocols requested be
        /// transmitted to a non-unicast (i.e., a subnetwork-broadcast or
        /// subnetwork-multicast) address, including those that were discarded
        /// or not sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutNUcastPkts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 18];

        /// The number of outbound packets which were chosen to be discarded
        /// even though no errors had been detected to prevent their being
        /// transmitted. One possible reason for discarding such a packet could
        /// be to free up buffer space.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDiscards(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 19];

        /// The number of outbound packets that could not be transmitted because
        /// of errors.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 20];

        /// The length of the output packet queue (in packets).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutQLen(pub Gauge32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 21];

        /// A reference to MIB definitions specific to the particular media
        /// being used to realize the interface.  For example, if the interface
        /// is realized by an ethernet, then the value of this object refers to
        /// a document defining objects specific to ethernet.  If this
        /// information is not present, its value should be set to the
        /// `OBJECT IDENTIFIER { 0 0 }`, which is a syntactically valid object
        /// identifier, and any conformant implementation of ASN.1 and BER must
        /// be able to generate and recognize this value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Specific(pub ObjectIdentifier);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 22];

        impl Default for Specific {
            fn default() -> Self {
                Self(ObjectIdentifier::new_unchecked(vec![0, 0].into()))
            }
        }
    }
}

/// The Address Translation Group
///
/// Implementation of the Address Translation group is mandatory for all
/// systems. Note however that this group is deprecated by MIB-II. That is, it
/// is being included solely for compatibility with MIB-I nodes, and will most
/// likely be excluded from MIB-III nodes.  From MIB-II and onwards, each
/// network protocol group contains its own address translation tables.
///
/// The Address Translation group contains one table which is the union across
/// all interfaces of the translation tables for converting a NetworkAddress
/// (e.g., an IP address) into a subnetwork-specific address.  For lack of a
/// better term, this document refers to such a subnetwork-specific address as a
/// "physical" address.
///
/// Examples of such translation tables are: for broadcast media where ARP is in
/// use, the translation table is equivalent to the ARP cache; or, on an X.25
/// network where non-algorithmic translation to X.121 addresses is required,
/// the translation table contains the NetworkAddress to X.121
/// address equivalences.
pub mod address {
    use super::*;

    object_type! {
        /// The Address Translation tables contain the NetworkAddress to
        /// "physical" address equivalences. Some interfaces do not use
        /// translation tables for determining address equivalences (e.g.,
        /// DDN-X.25 has an algorithmic method); if all interfaces are of this
        /// type, then the Address Translation table is empty, i.e., has
        /// zero entries.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct Table(pub Vec<Entry>);
        access: NotAccessible,
        status: Deprecated,
        value = [1, 3, 6, 1, 2, 1, 3, 1];

        /// Each entry contains one [`NetAddress`] to [`PhysAddress`] equivalence.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Entry {
            pub index: Index,
            pub phys_address: PhysAddress,
            pub net_address: NetAddress,
        }
        access: NotAccessible,
        status: Deprecated,
        value = [1, 3, 6, 1, 2, 1, 3, 1, 1];


        /// The interface on which this entry's equivalence is effective. The
        /// interface identified by a particular value of this index is the same
        /// interface as identified by the same value of [`interfaces::Index`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Index(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 3, 1, 1, 1];

        /// The media-dependent `physical' address.
        ///
        /// Setting this object to a null string (one of zero length) has the
        /// effect of invaliding the corresponding entry in the [`Table`]
        /// object. That is, it effectively disassociates the interface
        /// identified with said entry from the mapping identified with said
        /// entry. It is an implementation-specific matter as to whether the
        /// agent removes an invalidated entry from the table. Accordingly,
        /// management stations must be prepared to receive tabular information
        /// from agents that corresponds to entries not currently in use. Proper
        /// interpretation of such entries requires examination of the relevant
        /// [`PhysAddress`] object.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PhysAddress(pub super::PhysAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 3, 1, 1, 2];

        /// The NetworkAddress (e.g., the IP address) corresponding to the
        /// media-dependent [`PhysAddress`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetAddress(pub IpAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 3, 1, 1, 3];
    }
}

/// The Internet Protocol (IP) Group
///
/// Implementation of the IP group is mandatory for all systems.
pub mod ip {
    use super::*;

    object_type! {
        /// The indication of whether this entity is acting as an IP gateway in
        /// respect to the forwarding of datagrams received by, but not
        /// addressed to, this entity.  IP gateways forward datagrams. IP hosts
        /// do not (except those source-routed via the host).
        ///
        /// Note that for some managed nodes, this object may take on only a
        /// subset of the values possible. Accordingly, it is appropriate for an
        /// agent to return a `badValue` response if a management station
        /// attempts to change this object to an inappropriate value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Forwarding(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 1];

        impl Forwarding {
            pub const GATEWAY: u64 = 1;
            pub const HOST: u64 = 2;
        }

        /// The default value inserted into the "Time-To-Live" field of the IP
        /// header of datagrams originated at this entity, whenever a TTL value
        /// is not supplied by the transport layer protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct DefaultTtl(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 2];

        /// The total number of input datagrams received from interfaces,
        /// including those received in error.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InReceives(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 3];

        /// The number of input datagrams discarded due to errors in their IP
        /// headers, including bad checksums, version number mismatch, other
        /// format errors, time-to-live exceeded, errors discovered in
        /// processing their IP options, etc.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InHdrErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 4];

        /// The number of input datagrams discarded because the IP address in
        /// their IP header's destination field was not a valid address to be
        /// received at this entity. This count includes invalid addresses
        /// (e.g., 0.0.0.0) and addresses of unsupported Classes
        /// (e.g., Class E). For entities which are not IP Gateways and
        /// therefore do not forward datagrams, this counter includes datagrams
        /// discarded because the destination address was not a local address.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 5];

        /// The number of input datagrams for which this entity was not their
        /// final IP destination, as a result of which an attempt was made to
        /// find a route to forward them to that final destination. In entities
        /// which do not act as IP Gateways, this counter will include only
        /// those packets which were "Source-Routed" via this entity, and the
        /// Source-Route option processing was successful.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ForwDatagrams(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 6];

        /// The number of locally-addressed datagrams received successfully but
        /// discarded because of an unknown or unsupported protocol.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUnknownProtos(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 7];

        /// The number of input IP datagrams for which no problems were
        /// encountered to prevent their continued processing, but which were
        /// discarded (e.g., for lack of buffer space).  Note that this counter
        /// does not include any datagrams discarded while awaiting re-assembly.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDiscards(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 8];

        /// The total number of input datagrams successfully delivered to IP
        /// user-protocols (including ICMP).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDelivers(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 9];

        /// The total number of IP datagrams which local IP user-protocols
        /// (including ICMP) supplied to IP in requests for transmission. Note
        /// that this counter does not include any datagrams counted
        /// in [`ForwDatagrams`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutRequests(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 10];

        /// The number of output IP datagrams for which no problem was
        /// encountered to prevent their transmission to their destination, but
        /// which were discarded (e.g., for lack of buffer space).  Note that
        /// this counter would include datagrams counted in [`ForwDatagrams`] if
        /// any such packets met this (discretionary) discard criterion.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDiscards(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 11];

        /// The number of IP datagrams discarded because no route could be found
        /// to transmit them to their destination. Note that this counter
        /// includes any packets counted in [`ForwDatagrams`] which meet this
        /// `no-route' criterion.  Note that this includes any datagrams which a
        /// host cannot route because all of its default gateways are down.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutNoRoutes(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 12];

        /// The maximum number of seconds which received fragments are held
        /// while they are awaiting reassembly at this entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmTimeout(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 13];

        /// The number of IP fragments received which needed to be reassembled
        /// at this entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmReqds(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 14];

        /// The number of IP datagrams successfully re-assembled.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmOks(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 15];

        /// The number of failures detected by the IP re-assembly algorithm
        /// (for whatever reason: timed out, errors, etc). Note that this is
        /// not necessarily a count of discarded IP fragments since some
        /// algorithms (notably the algorithm in RFC 815) can lose track of the
        /// number of fragments by combining them as they are received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmFails(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 16];

        /// The number of IP datagrams that have been successfully fragmented at
        /// this entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragOks(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 17];

        /// The number of IP datagrams that have been discarded because they
        /// needed to be fragmented at this entity but could not be, e.g.,
        /// because their "Don't Fragment" flag was set.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragFails(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 18];

        /// The number of IP datagram fragments that have been generated as a
        /// result of fragmentation at this entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragCreates(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 19];

        /// The table of addressing information relevant to this entity's
        /// IP addresses.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct AddrTable(pub Vec<AddrEntry>);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20];

        /// The addressing information for one of this entity's IP addresses.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AddrEntry {
            pub ad_ent_addr: AdEntAddr,
            pub ad_ent_if_index: AdEntIfIndex,
            pub ad_ent_net_mask: AdEntNetMask,
            pub ad_ent_bcast_addr: AdEntBcastAddr,
        }
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1];

        /// The IP address to which this entry's addressing
        /// information pertains.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntAddr(pub IpAddress);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1, 1];

        /// The index value which uniquely identifies the interface to which
        /// this entry is applicable. The interface identified by a particular
        /// value of this index is the same interface as identified by the same
        /// value of [`interfaces::Index`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntIfIndex(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1, 2];

        /// The subnet mask associated with the IP address of this entry. The
        /// value of the mask is an IP address with all the network bits set to
        /// 1 and all the hosts bits set to 0.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntNetMask(pub IpAddress);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1, 3];

        /// The value of the least-significant bit in the IP broadcast address
        /// used for sending datagrams on the (logical) interface associated
        /// with the IP address of this entry.
        ///
        /// For example, when the Internet standard all-ones broadcast address
        /// is used, the value will be 1.  This value applies to both the subnet
        /// and network broadcasts addresses used by the entity on this
        /// (logical) interface.
        ///
        /// The value of the least-significant bit in the
        /// IP broadcast address used for sending datagrams on the (logical)
        /// interface associated with the IP address of this entry.
        ///
        /// For example, when the Internet standard all-ones broadcast address
        /// is used, the value will be 1. This value applies to both the subnet
        /// and network broadcasts addresses used by the entity on this
        /// (logical) interface.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntBcastAddr(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1, 4];

        /// The size of the largest IP datagram which this entity can
        /// re-assemble from incoming IP fragmented datagrams received on
        /// this interface.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntReasmMaxSize(pub u16);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 20, 1, 5];

        /// This entity's IP Routing table.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct RoutingTable(pub Vec<RouteEntry>);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21];

        /// A router to a particular destination.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteEntry {
            pub route_dest: RouteDest,
            pub route_if_index: RouteIfIndex,
            pub route_metric_1: RouteMetric1,
            pub route_metric_2: RouteMetric2,
            pub route_metric_3: RouteMetric3,
            pub route_metric_4: RouteMetric4,
            pub route_next_hop: RouteNextHop,
            pub route_type: RouteType,
            pub route_proto: RouteProto,
            pub route_age: RouteAge,
            pub route_mask: RouteMask,
            pub route_metric_5: RouteMetric5,
            pub route_info: RouteInfo,
        }
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1];

        /// The destination IP address of this route. An entry with a value of
        /// 0.0.0.0 is considered a default route. Multiple routes to a single
        /// destination can appear in the table, but access to such multiple
        /// entries is dependent on the table-access mechanisms defined by the
        /// network management protocol in use.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteDest(pub IpAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 1];

        /// The index value which uniquely identifies the local interface
        /// through which the next hop of this route should be reached. The
        /// interface identified by a particular value of this index is the same
        /// interface as identified by the same value of [`interfaces::Index`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteIfIndex(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 2];

        /// The primary routing metric for this route. The semantics of this
        /// metric are determined by the routing-protocol specified in the
        /// route's ipRouteProto value. If this metric is not used, its value
        /// should be set to -1.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric1(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 3];

        /// An alternate routing metric for this route. The semantics of this
        /// metric are determined by the routing-protocol specified in the
        /// route's [`RouteProto`] value.  If this metric is not used, its value
        /// should be set to -1.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric2(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 4];

        /// An alternate routing metric for this route. The semantics of this
        /// metric are determined by the routing-protocol specified in the
        /// route's [`RouteProto`] value.  If this metric is not used, its value
        /// should be set to -1.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric3(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 5];

        /// An alternate routing metric for this route. The semantics of this
        /// metric are determined by the routing-protocol specified in the
        /// route's [`RouteProto`] value.  If this metric is not used, its value
        /// should be set to -1.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric4(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 6];

        /// The IP address of the next hop of this route.  (In the case of a
        /// route bound to an interface which is realized via a broadcast media,
        /// the value of this field is the agent's IP address on
        /// that interface.)
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteNextHop(pub IpAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 7];

        /// The type of route.  Note that the values [`Self::DIRECT`] and
        /// [`Self::INDIRECT`] refer to the notion of direct and indirect
        /// routing in the IP architecture.
        ///
        /// Setting this object to the value [`Self::INVALID`] has the effect of
        /// invalidating the corresponding entry in the [`RouteTable`] object.
        /// That is, it effectively disassociates the destination identified
        /// with said entry from the route identified with said entry.  It is an
        /// implementation-specific matter as to whether the agent removes an
        /// invalidated entry from the table.  Accordingly, management stations
        /// must be prepared to receive tabular information from agents that
        /// corresponds to entries not currently in use.  Proper interpretation
        /// of such entries requires examination of the relevant
        /// [`RouteType`] object.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteType(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 8];

        impl RouteType {
            /// None of the other constants.
            pub const OTHER: u64 = 1;
            /// An invalidated route.
            pub const INVALID: u64 = 2;
            /// Route to directly connected (sub-)network
            pub const DIRECT: u64 = 3;
            /// Route to a non-local host/(sub-)network
            pub const REMOTE: u64 = 4;
        }

        /// The routing mechanism via which this route was learned. Inclusion of
        /// values for gateway routing protocols is not intended to imply that
        /// hosts should support those protocols.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteProto(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 9];

        impl RouteProto {
            pub const OTHER: u64 = 1;
            pub const LOCAL: u64 = 2;
            pub const NETMGMT: u64 = 3;
            pub const ICMP: u64 = 4;
            pub const EGP: u64 = 5;
            pub const GGP: u64 = 6;
            pub const HELLO: u64 = 7;
            pub const RIP: u64 = 8;
            pub const IS_IS: u64 = 9;
            pub const ES_IS: u64 = 10;
            pub const CISCO_IGRP: u64 = 11;
            pub const BBN_SPF_IGP: u64 = 12;
            pub const OIGP: u64 = 13;
        }

        /// The number of seconds since this route was last updated or otherwise
        /// determined to be correct. Note that no semantics of "too old" can be
        /// implied except through knowledge of the routing protocol by which
        /// the route was learned.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteAge(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 10];

        /// Indicate the mask to be logical-ANDed with the destination address
        /// before being compared to the value in the [`RouteDest`] field. For
        /// those systems that do not support arbitrary subnet masks, an agent
        /// constructs the value of the [`RouteMask`] by determining whether the
        /// value of the correspondent [`RouteDest`] field belong to a class-A,
        /// B, or C network, and then using one of:
        ///
        /// | mask          | network |
        /// | ------------- | ------- |
        /// | 255.0.0.0     | class-A |
        /// | 255.255.0.0   | class-B |
        /// | 255.255.255.0 | class-C |
        ///
        /// If the value of the ipRouteDest is 0.0.0.0 (a default route), then
        /// the mask value is also 0.0.0.0.  It should be noted that all IP
        /// routing subsystems implicitly use this mechanism.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMask(pub IpAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 11];

        /// An alternate routing metric for this route. The semantics of this
        /// metric are determined by the routing-protocol specified in the
        /// route's [`RouteProto`] value.  If this metric is not used, its value
        /// should be set to -1.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric5(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 12];

        /// A reference to MIB definitions specific to the particular routing
        /// protocol which is responsible for this route, as determined by the
        /// value specified in the route's [`RouteProto`] value.  If this
        /// information is not present, its value should be set to the
        /// `OBJECT IDENTIFIER { 0 0 }`, which is a syntactically valid object
        /// identifier, and any conformant implementation of ASN.1 and BER must
        /// be able to generate and recognize this value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteInfo(pub ObjectIdentifier);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 21, 1, 13];

        /// A reference to MIB definitions specific to the particular routing
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct NetToMediaTable(pub Vec<NetToMediaEntry>);
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22];

        /// A reference to MIB definitions specific to the particular routing
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetToMediaEntry {
            index: NetToMediaIndex,
            phys_address: NetToMediaPhysAddress,
            net_address: NetToMediaNetAddress,
            r#type: NetToMediaType,
        }
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22, 1];

        /// The interface on which this entry's equivalence is effective. The
        /// interface identified by a particular value of this index is the same
        /// interface as identified by the same value of [`interfaces::Index`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetToMediaIndex(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22, 1, 1];

        /// The media-dependent "physical" address.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetToMediaPhysAddress(pub PhysAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22, 1, 2];

        /// The [`IpAddress`] corresponding to the media-dependent [`NetToMediaPhysAddress`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetToMediaNetAddress(pub IpAddress);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22, 1, 3];

        /// The type of mapping.
        ///
        /// Setting this object to the value [`Self::INVALID`] has the effect of
        /// invalidating the corresponding entry in the ipNetToMediaTable. That
        /// is, it effectively disassociates the interface identified with said
        /// entry from the mapping identified with said entry. It is an
        /// implementation-specific matter as to whether the agent removes an
        /// invalidated entry from the table. Accordingly, management stations
        /// must be prepared to receive tabular information from agents that
        /// corresponds to entries not currently in use.  Proper interpretation
        /// of such entries requires examination of the relevant
        /// [`NetToMediaType`] object.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetToMediaType(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 22, 1, 4];

        /// The number of routing entries which were chosen to be discarded even
        /// though they are valid. One possible reason for discarding such an
        /// entry could be to free-up buffer space for other routing entries.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RoutingDiscards(pub Counter32);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 4, 23];
    }
}

/// The ICMP Group
///
/// Implementation of the ICMP group is mandatory for all systems.
pub mod icmp {
    use super::*;

    object_type! {
        /// The total number of ICMP messages which the entity received. Note
        /// that this counter includes all those counted by [`InErrors`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InMsgs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 1];

        /// The number of ICMP messages which the entity received but determined
        /// as having ICMP-specific errors (bad ICMP checksums,
        /// bad length, etc.).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 2];

        /// The number of ICMP "Destination Unreachable" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDestUnreachs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 3];

        /// The number of ICMP "Time Exceeded" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimeExcds(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 4];

        /// The number of ICMP "Parameter Problem" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InParmProbs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 5];

        /// The number of ICMP "Source Quench" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InSrcQuenchs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 6];

        /// The number of ICMP "Redirect" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InRedirects(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 7];

        /// The number of ICMP "Echo (request)" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InEchos(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 8];

        /// The number of ICMP "Echo Reply" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InEchosReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 9];

        /// The number of ICMP "Timestamp (request)" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimestamps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 10];

        /// The number of ICMP "Timestamp Reply" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimestampsReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 11];

        /// The number of ICMP "Address Mask (request)" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrMasks(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 12];

        /// The number of ICMP "Address Mask Reply" messages received.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrMasksReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 13];

        /// The total number of ICMP messages which this entity attempted to
        /// send. Note that this counter includes all those counted
        /// by [`OutErrors`].
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutMsgs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 14];

        /// The number of ICMP messages which this entity did not send due to
        /// problems discovered within ICMP such as a lack of buffers. This
        /// value should not include errors discovered outside the ICMP layer
        /// such as the inability of IP to route the resultant datagram. In some
        /// implementations there may be no types of error which contribute to
        /// this counter's value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 15];

        /// The number of ICMP "Destination Unreachable" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDestUnreachs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 16];

        /// The number of ICMP "Time Exceeded" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimeExcds(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 17];

        /// The number of ICMP "Parameter Problem" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutParmProbs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 18];

        /// The number of ICMP "Source Quench" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutSrcQuenchs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 19];

        /// The number of ICMP Redirect messages sent. For a host, this object
        /// will always be zero, since hosts do not send redirects.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutRedirects(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 20];

        /// The number of ICMP "Echo (request)" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutEchos(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 21];

        /// The number of ICMP "Echo Reply" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutEchosReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 22];

        /// The number of ICMP "Timestamp (request)" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimestamps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 23];

        /// The number of ICMP "Timestamp Reply" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimestampsReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 24];

        /// The number of ICMP "Address Mask (request)" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutAddrMasks(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 25];

        /// The number of ICMP "Address Mask Reply" messages sent.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutAddrMasksReps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 5, 26];
    }
}

/// The Transmission Control Protocol (TCP) Group
///
/// Implementation of the TCP group is mandatory for all systems that implement
/// the TCP.
///
/// Note that instances of object types that represent information about a
/// particular TCP connection are transient; they persist only as long as the
/// connection in question.
pub mod tcp {
    use super::*;

    object_type! {
        /// The algorithm used to determine the timeout value used for
        /// retransmitting unacknowledged octets.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoAlgorithm(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 1];

        impl RtoAlgorithm {
            /// Unknown
            pub const OTHER: u64 = 1;
            /// A constant RTO
            pub const CONSTANT: u64 = 2;
            /// MIL-STD-1778
            pub const RSRE: u64 = 3;
            /// Van Jacobson's algorithm
            pub const VANJ: u64 = 4;
        }

        /// The minimum value permitted by a TCP implementation for the
        /// retransmission timeout, measured in milliseconds.  More refined
        /// semantics for objects of this type depend upon the algorithm used to
        /// determine the retransmission timeout.  In particular, when the
        /// timeout algorithm is [`RtoAlgorithm::RSRE`], an object of this type
        /// has the semantics of the `LBOUND` quantity described in RFC 793.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoMin(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 2];

        /// The maximum value permitted by a TCP implementation for the
        /// retransmission timeout, measured in milliseconds.  More refined
        /// semantics for objects of this type depend upon the algorithm used to
        /// determine the retransmission timeout. In particular, when the
        /// timeout algorithm is [`RtoAlgorithm::RSRE`], an object of this type
        /// has the semantics of the `UBOUND` quantity described in RFC 793.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoMax(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 3];

        /// The limit on the total number of TCP connections the entity can
        /// support. In entities where the maximum number of connections is
        /// dynamic, this object should contain the value `-1`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct MaxConn(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 4];

        /// The number of times TCP connections have made a direct transition to
        /// the `SYN-SENT` state from the `CLOSED` state.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ActiveOpens(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 5];

        /// The number of times TCP connections have made a direct transition to
        /// the `SYN-RCVD` state from the `LISTEN` state.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PassiveOpens(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 6];

        /// The number of times TCP connections have made a direct transition to
        /// the `CLOSED` state from either the `SYN-SENT` state or the
        /// `SYN-RCVD` state, plus the number of times TCP connections have made
        /// a direct transition to the `LISTEN` state from the `SYN-RCVD` state.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AttemptsFails(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 7];

        /// The number of times TCP connections have made a direct transition to
        /// the `CLOSED` state from either the `ESTABLISHED` state or the
        /// `CLOSE-WAIT` state.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct EstabResets(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 8];

        /// The number of TCP connections for which the current state is either
        /// `ESTABLISHED` or `CLOSE-WAIT`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct CurrEstab(pub Gauge32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 9];

        /// The total number of segments received, including those received in
        /// error. This count includes segments received on currently
        /// established connections.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InSegs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 10];

        /// The total number of segments sent, including those on current
        /// connections but excluding those containing only
        /// retransmitted octets.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutSegs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 11];

        /// The total number of segments retransmitted - that is, the number of
        /// TCP segments transmitted containing one or more previously
        /// transmitted octets.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RetransSegs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 12];

        /// A table containing TCP connection-specific information.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct ConnTable(pub Vec<ConnEntry>);
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13];

        /// Information about a particular current TCP connection.  An object of
        /// this type is transient, in that it ceases to exist when (or soon
        /// after) the connection makes the transition to the `CLOSED` state.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnEntry {
            state: ConnState,
            local_address: ConnLocalAddress,
            local_port: ConnLocalPort,
            rem_address: ConnRemAddress,
            rem_port: ConnRemPort,

        }
        access: NotAccessible,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1];

        /// The state of this TCP connection.
        ///
        /// The only value which may be set by a management station is
        /// [`Self::DELETE_TCB`]. Accordingly, it is appropriate for an agent to
        /// return a `badValue` response if a management station attempts to set
        /// this object to any other value.
        ///
        /// If a management station sets this object to the value
        /// [`Self::DELETE_TCB`], then this has the effect of deleting the TCB
        /// (as defined in RFC 793) of the corresponding connection on the
        /// managed node, resulting in immediate termination of the connection.
        ///
        /// As an implementation-specific option, a RST segment may be sent from
        /// the managed node to the other TCP endpoint (note however that RST
        /// segments are not sent reliably).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnState(pub Integer);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1, 1];

        impl ConnState {
            pub const CLOSED: u64 = 1;
            pub const LISTEN: u64 = 2;
            pub const SYN_SENT: u64 = 3;
            pub const SYN_RECEIVED: u64 = 4;
            pub const ESTABLISHED: u64 = 5;
            pub const FIN_WAIT1: u64 = 6;
            pub const FIN_WAIT2: u64 = 7;
            pub const CLOSE_WAIT: u64 = 8;
            pub const LAST_ACK: u64 = 9;
            pub const CLOSING: u64 = 10;
            pub const TIME_WAIT: u64 = 11;
            pub const DELETE_TCB: u64 = 12;
        }

        /// The local IP address for this TCP connection.  In the case of a
        /// connection in the `LISTEN` state which is willing to accept
        /// connections for any IP interface associated with the node, the value
        /// 0.0.0.0 is used.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnLocalAddress(pub IpAddress);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1, 2];

        /// The local port number for this TCP connection.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnLocalPort(pub u16);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1, 3];

        /// The remote IP address for this TCP connection.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnRemAddress(pub IpAddress);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1, 4];

        /// The remote port number for this TCP connection.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnRemPort(pub u16);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 13, 1, 5];

        /// The total number of segments received in error
        /// (e.g., bad TCP checksums).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 14];

        /// The number of TCP segments sent containing the `RST` flag
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutRsts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 6, 15];
    }
}

/// The User Datagram Protocol (UDP) Group
///
/// Implementation of the UDP group is mandatory for all systems which implement
/// the UDP.
pub mod udp {
    use super::*;

    object_type! {
        /// The total number of UDP datagrams delivered to UDP users.
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of `sysUpTime`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDatagrams(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 1];

        /// The total number of received UDP datagrams for which there was no
        /// application at the destination port.
        ///
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of `sysUpTime`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NoPorts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 2];

        /// The number of received UDP datagrams that could not be delivered for
        /// reasons other than the lack of an application at the
        /// destination port.
        ///
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of sysUpTime.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 3];

        /// The total number of UDP datagrams sent from this entity.
        ///
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of sysUpTime.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDatagrams(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 4];

        /// The total number of UDP datagrams delivered to UDP users, for
        /// devices that can receive more than 1 million UDP datagrams
        /// per second.
        ///
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of sysUpTime.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct HcInDatagrams(pub Counter64);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 8];

        /// The total number of UDP datagrams sent from this entity, for devices
        /// that can transmit more than 1 million UDP datagrams per second.
        ///
        /// Discontinuities in the value of this counter can occur at
        /// re-initialization of the management system, and at other times as
        /// indicated by discontinuities in the value of sysUpTime.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct HcOutDatagrams(pub Counter64);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 7, 9];

    }
}

/// the EGP (Exterior Gateway Protocol) group
///
/// Implementation of the EGP group is mandatory for all
/// systems which implement the EGP.
pub mod egp {
    use super::*;

    object_type! {
        /// The number of EGP messages received without error.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 1];

        /// The number of EGP messages received that proved to be in error.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 2];

        /// The total number of locally generated EGP messages.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 3];

        /// The number of locally generated EGP messages not sent due to
        /// resource limitations within an EGP entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 4];

        /// The EGP neighbor table contains information about this entity's
        /// EGP neighbors.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct NeighTable(pub Vec<NeighEntry>);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5];

        /// Information about this entity's relationship with a particular
        /// EGP neighbor.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighEntry {
            state: NeighState,
            addr: NeighAddr,
            r#as: NeighAs,
            in_msgs: NeighInMsgs,
            in_errs: NeighInErrs,
            out_msgs: NeighOutMsgs,
            out_errs: NeighOutErrs,
            in_err_msgs: NeighInErrMsgs,
            out_err_msgs: NeighOutErrMsgs,
            state_ups: NeighStateUps,
            state_downs: NeighStateDowns,
            interval_hello: NeighIntervalHello,
            interval_poll: NeighIntervalPoll,
            mode: NeighMode,
            event_trigger: NeighEventTrigger,
        }
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1];

        /// The EGP state of the local system with respect to this entry's EGP
        /// neighbor. Each EGP state is represented by a value that is one
        /// greater than the numerical value associated with said state in
        /// RFC 904.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighState(pub Integer);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 1];

        impl NeighState {
            pub const IDLE: u64 = 1;
            pub const ACQUISITION: u64 = 2;
            pub const DOWN: u64 = 3;
            pub const UP: u64 = 4;
            pub const CEASE: u64 = 5;
        }

        /// The IP address of this entry's EGP neighbor.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighAddr(pub IpAddress);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 2];

        /// The autonomous system of this EGP peer. Zero should be specified if
        /// the autonomous system number of the neighbor is not yet known.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighAs(pub Integer);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 3];

        /// The number of EGP messages received without error from this
        /// EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighInMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 4];

        /// The number of EGP messages received from this EGP peer that proved
        /// to be in error (e.g., bad EGP checksum).
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighInErrs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 5];

        /// The number of locally generated EGP messages to this EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighOutMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 6];

        /// The number of locally generated EGP messages not sent to this EGP
        /// peer due to resource limitations within an EGP entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighOutErrs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 7];

        /// The number of EGP-defined error messages received from this
        /// EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighInErrMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 8];

        /// The number of EGP-defined error messages sent to this
        /// EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighOutErrMsgs(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 9];

        /// The number of EGP state transitions to the UP state with this
        /// EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighStateUps(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 10];

        /// The number of EGP state transitions from the UP state to any other
        /// state with this EGP peer.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighStateDowns(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 11];

        /// The interval between EGP Hello command retransmissions (in
        /// hundredths of a second).  This represents the t1 timer as defined in
        /// RFC 904.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighIntervalHello(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 12];

        /// The interval between EGP poll command retransmissions (in
        /// hundredths of a second). This represents the t3 timer as defined in
        /// RFC 904.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighIntervalPoll(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 13];

        /// The interval between EGP poll command retransmissions (in
        /// hundredths of a second). This represents the t3 timer as defined in
        /// RFC 904.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighMode(pub Integer);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 14];

        impl NeighMode {
            pub const ACTIVE: u64 = 1;
            pub const STOP: u64 = 2;
        }

        /// A control variable used to trigger operator-initiated "Start" and
        /// "Stop" events.  When read, this variable always returns the most
        /// recent value that egpNeighEventTrigger was set to. If it has not
        /// been set since the last initialization of the network management
        /// subsystem on the node, it returns a value of `stop'.
        ///
        /// When set, this variable causes a Start or Stop event on the
        /// specified neighbor, as specified on pages 8-10 of RFC 904.  Briefly,
        /// a Start event causes an Idle peer to begin neighbor acquisition and
        /// a non-Idle peer to reinitiate neighbor acquisition.  A stop event
        /// causes a non-Idle peer to return to the Idle state until a Start
        /// event occurs, either via egpNeighEventTrigger or otherwise.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighEventTrigger(pub Integer);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 5, 1, 15];

        impl NeighEventTrigger {
            pub const START: u64 = 1;
            pub const STOP: u64 = 2;
        }

        /// The autonomous system number of this EGP entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct As(pub Integer);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 8, 6];
    }
}

/// The Simple Network Management Protocol (SNMP) Group
///
/// Implementation of the SNMP group is mandatory for all systems which support
/// an SNMP protocol entity.  Some of the objects defined below will be
/// zero-valued in those SNMP implementations that are optimized to support only
/// those functions specific to either a management agent or a management
/// station. In particular, it should be observed that the objects below refer
/// to an SNMP entity, and there may be several SNMP entities residing on a
/// managed node (e.g., if the node is hosting acting as a management station).
pub mod snmp {
    use super::*;

    smi::object_type! {
        /// The total number of Messages delivered to the SNMP entity from the
        /// transport service.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InPkts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 1];

        /// The total number of SNMP Messages which were passed from the SNMP
        /// protocol entity to the transport service.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutPkts(pub Counter32);
        access: ReadOnly,
        status: Obsolete,
        value = [1, 3, 6, 1, 2, 1, 11, 2];

        /// The total number of SNMP Messages which were delivered to the SNMP
        /// protocol entity and were for an unsupported SNMP version.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InBadVersions(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 3];

        /// The total number of community-based SNMP messages (for example,
        /// SNMPv1) delivered to the SNMP entity which used an SNMP community
        /// name not known to said entity.  Also, implementations which
        /// authenticate community-based SNMP messages using check(s) in
        /// addition to matching the community name (for example, by also
        /// checking whether the message originated from a transport address
        /// allowed to use a specified community name) MAY include in this value
        /// the number of messages which failed the additional check(s). It is
        /// strongly recommended that the documentation for any security model
        /// which is used to authenticate community-based SNMP messages specify
        /// the precise conditions that contribute to this value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InBadCommunityNames(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 4];

        /// The total number of community-based SNMP messages (for example,
        /// SNMPv1) delivered to the SNMP entity which represented an SNMP
        /// operation that was not allowed for the SNMP community named in the
        /// message. The precise conditions under which this counter is
        /// incremented (if at all) depend on how the SNMP entity implements its
        /// access control mechanism and how its applications interact with that
        /// access control mechanism. It is strongly recommended that the
        /// documentation for any access control mechanism which is used to
        /// control access to and visibility of MIB instrumentation specify the
        /// precise conditions that contribute to this value.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InBadCommunityUses(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 5];

        /// The total number of ASN.1 or BER errors encountered by the SNMP
        /// entity when decoding received SNMP messages.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAsnParseErrs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 6];

        /// The total number of SNMP PDUs which were delivered to the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `tooBig`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTooBigs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 8];

        /// The total number of SNMP PDUs which were delivered to the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `noSuchName`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InNoSuchNames(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 9];

        /// The total number of SNMP PDUs which were delivered to the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `badValue`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InBadValues(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 10];

        /// The total number valid SNMP PDUs which were delivered to the SNMP
        /// protocol entity and for which the value of the error-status field is
        /// `readOnly`.  It should be noted that it is a protocol error to
        /// generate an SNMP PDU which contains the value `readOnly` in the
        /// error-status field, as such this object is provided as a means of
        /// detecting incorrect implementations of the SNMP.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InReadOnlys(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 11];

        /// The total number of SNMP PDUs which were delivered to the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `genErr`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InGenErrs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 12];

        /// The total number of MIB objects which have been retrieved
        /// successfully by the SNMP protocol entity as the result of receiving
        /// valid SNMP Get-Request and Get-Next PDUs.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTotalReqVars(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 13];

        /// The total number of MIB objects which have been altered
        /// successfully by the SNMP protocol entity as the result of receiving
        /// valid SNMP Set-Request PDUs.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTotalSetVars(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 14];

        /// The total number of SNMP Get-Request PDUs which have been accepted
        /// and processed by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InGetRequests(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 15];

        /// The total number of SNMP Get-Next PDUs which have been accepted and
        /// processed by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InGetNexts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 16];

        /// The total number of SNMP Set-Request PDUs which have been accepted
        /// and processed by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InSetRequests(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 17];

        /// The total number of SNMP Get-Response PDUs which have been accepted
        /// and processed by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InGetResponses(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 18];

        /// The total number of SNMP Trap PDUs which have been accepted and
        /// processed by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTraps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 19];

        /// The total number of SNMP PDUs which were generated by the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `tooBig`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTooBigs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 20];

        /// The total number of SNMP PDUs which were generated by the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `noSuchName`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutNoSuchNames(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 21];

        /// The total number of SNMP PDUs which were generated by the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `badValue`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutBadValues(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 22];

        /// The total number of SNMP PDUs which were generated by the SNMP
        /// protocol entity and for which the value of the error-status field
        /// is `genErr`.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutGenErrs(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 24];

        /// The total number of SNMP Get-Request PDUs which have been generated
        /// by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutGetRequests(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 25];

        /// The total number of SNMP Get-Next PDUs which have been generated by
        /// the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutGetNexts(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 26];

        /// The total number of SNMP Set-Request PDUs which have been generated
        /// by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutSetRequests(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 27];

        /// The total number of SNMP Get-Response PDUs which have been generated
        /// by the SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutGetResponses(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 28];

        /// The total number of SNMP Trap PDUs which have been generated by the
        /// SNMP protocol entity.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTraps(pub Counter32);
        access: ReadOnly,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 29];

        /// Indicates whether the SNMP agent process is permitted to generate
        /// authentication-failure traps.  The value of this object overrides
        /// any configuration information; as such, it provides a means whereby
        /// all authentication-failure traps may be disabled.
        ///
        /// Note that it is strongly recommended that this object be stored in
        /// non-volatile memory so that it remains constant between
        /// re-initializations of the network management system.
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct EnableAuthenTraps(pub Integer);
        access: ReadWrite,
        status: Current,
        value = [1, 3, 6, 1, 2, 1, 11, 30];
    }
}
