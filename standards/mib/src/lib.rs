#![no_std]

use rasn::{*, types::*};
use smi::*;

pub mod system {
    use super::*;

    smi::object_type! {
        pub struct Descr(pub OctetString);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 1, 1];

        pub struct ObjectId(pub ObjectName);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 1, 2];

        pub struct UpTime(pub TimeTicks);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 1, 3];
    }
}

pub mod interfaces {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
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
        }
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Number(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct Table(pub Vec<Entry>);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Index(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Descr(pub OctetString);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Type(pub Integer);
        access: ReadOnly,
        status: Mandatory,
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
            pub const PROP_POINT_TO_POINT_SERIAL: u64 = 21;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Mtu(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Speed(pub Gauge);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PhysAddress(pub OctetString);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 6];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdminStatus(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 7];

        impl AdminStatus {
            pub const UP: u64 = 1;
            pub const DOWN: u64 = 2;
            pub const TESTING: u64 = 3;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OperStatus(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 8];

        impl OperStatus {
            pub const UP: u64 = 1;
            pub const DOWN: u64 = 2;
            pub const TESTING: u64 = 3;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct LastChange(pub TimeTicks);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 9];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InOctets(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 10];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUcastPkts(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 11];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InNUcastPkts(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 12];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDiscards(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 13];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 14];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUnknownProtos(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 15];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutOctets(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 16];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutUcastPkts(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 17];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutNUcastPkts(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 18];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDiscards(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 19];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 20];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutQLen(pub Gauge);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 2, 1, 21];
    }
}

pub mod address {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct Table(pub Vec<Entry>);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 3, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Entry {
            pub index: Index,
            pub phys_address: PhysAddress,
            pub net_address: NetAddress,
        }
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 3, 1, 1];


        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Index(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 3, 1, 1, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PhysAddress(pub OctetString);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 3, 1, 1, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NetAddress(pub NetworkAddress);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 3, 1, 1, 3];
    }
}

pub mod ip {
    use super::*;

    smi::object_type! {

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AddrEntry {
            pub ad_ent_addr: AdEntAddr,
            pub ad_ent_if_index: AdEntIfIndex,
            pub ad_ent_net_mask: AdEntNetMask,
            pub ad_ent_bcast_addr: AdEntBcastAddr,
        }
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20, 1];

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
        }
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct Forwarding(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 1];

        impl Forwarding {
            pub const GATEWAY: u64 = 1;
            pub const HOST: u64 = 2;
        }


        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct DefaultTtl(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InReceives(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InHdrErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InForwDatagrams(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 6];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InUnknownProtos(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 7];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDiscards(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 8];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDelivers(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 9];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutRequests(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 10];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDiscards(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 11];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutNoRoutes(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 12];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmTimeout(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 13];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmReqds(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 14];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmOks(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 15];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ReasmFails(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 16];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragOks(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 17];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragFails(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 18];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct FragCreates(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 19];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct AddrTable(pub Vec<AddrEntry>);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntAddr(pub IpAddress);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20, 1, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntIfIndex(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20, 1, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntNetMask(pub IpAddress);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20, 1, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AdEntBcastAddr(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 20, 1, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct RoutingTable(pub Vec<RouteEntry>);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteDest(pub IpAddress);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteIfIndex(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric1(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric2(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric3(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteMetric4(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 6];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteNextHop(pub IpAddress);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 7];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteType(pub Integer);
        access: ReadWrite,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 8];

        impl RouteType {
            pub const OTHER: u64 = 1;
            pub const INVALID: u64 = 2;
            pub const DIRECT: u64 = 3;
            pub const REMOTE: u64 = 4;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteProto(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 9];

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

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RouteAge(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 4, 21, 1, 10];
    }
}

pub mod icmp {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InMsgs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDestUnreachs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimeExcds(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InParmProbs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InSrcQuenchs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 6];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InRedirects(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 7];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InEchos(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 8];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InEchosReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 9];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimestamps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 10];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InTimestampsReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 11];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrMasks(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 12];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InAddrMasksReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 13];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutMsgs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 14];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 15];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDestUnreachs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 16];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimeExcds(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 17];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutParmProbs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 18];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutSrcQuenchs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 19];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutRedirects(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 20];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutEchos(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 21];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutEchosReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 22];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimestamps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 23];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutTimestampsReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 24];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutAddrMasks(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 25];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutAddrMasksReps(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 5, 26];
    }
}

pub mod tcp {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoAlgorithm(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 1];

        impl RtoAlgorithm {
            pub const OTHER: u64 = 1;
            pub const CONSTANT: u64 = 2;
            pub const RSRE: u64 = 3;
            pub const VANJ: u64 = 4;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoMin(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RtoMax(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct MaxConn(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ActiveOpens(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct PassiveOpens(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 6];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct AttemptsFails(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 7];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct EstabResets(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 8];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct CurrEstab(pub Gauge);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 9];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InSegs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 10];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutSegs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 11];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct RetransSegs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 12];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct ConnTable(pub Vec<ConnEntry>);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnEntry {
            state: ConnState,
            local_address: ConnLocalAddress,
            local_port: ConnLocalPort,
            rem_address: ConnRemAddress,
            rem_port: ConnRemPort,

        }
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnState(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1, 1];

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
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnLocalAddress(pub IpAddress);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnLocalPort(pub u16);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnRemAddress(pub IpAddress);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct ConnRemPort(pub u16);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 6, 13, 1, 5];
    }
}

pub mod udp {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InDatagrams(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 7, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NoPorts(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 7, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 7, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutDatagrams(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 7, 4];
    }
}

pub mod egp {
    use super::*;

    smi::object_type! {
        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InMsgs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct InErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 2];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutMsgs(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 3];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct OutErrors(pub Counter);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 4];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub opaque struct NeighTable(pub Vec<NeighEntry>);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 5];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighEntry {
            state: NeighState,
            addr: NeighAddr,

        }
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 5, 1];

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighState(pub Integer);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 5, 1, 1];

        impl NeighState {
            pub const IDLE: u64 = 1;
            pub const ACQUISITION: u64 = 2;
            pub const DOWN: u64 = 3;
            pub const UP: u64 = 4;
            pub const CEASE: u64 = 5;
        }

        #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
        pub struct NeighAddr(pub IpAddress);
        access: ReadOnly,
        status: Mandatory,
        value = [1, 3, 6, 1, 2, 1, 2, 8, 5, 1, 2];
    }
}
