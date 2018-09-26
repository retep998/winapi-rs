// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! TCP/IP specific information for use by WinSock2 compatible applications.
use ctypes::{c_int, c_short};
use shared::in6addr::IN6_ADDR;
use shared::inaddr::IN_ADDR;
use shared::minwindef::{DWORD, UINT, ULONG, USHORT};
use shared::ws2def::{
    ADDRESS_FAMILY, IOC_IN, IOC_OUT, IOC_VOID, SCOPE_ID, SOCKADDR, SOCKADDR_IN, SOCKADDR_STORAGE,
    SOCKET_ADDRESS,
};
use um::winnt::SHORT;
STRUCT!{struct sockaddr_in6_old {
    sin6_family: SHORT,
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: IN6_ADDR,
}}
UNION!{union sockaddr_gen {
    [u32; 6],
    Address Address_mut: SOCKADDR,
    AddressIn AddressIn_mut: SOCKADDR_IN,
    AddressIn6 AddressIn6_mut: sockaddr_in6_old,
}}
STRUCT!{struct INTERFACE_INFO {
    iiFlags: ULONG,
    iiAddress: sockaddr_gen,
    iiBroadcastAddress: sockaddr_gen,
    iiNetmask: sockaddr_gen,
}}
pub type LPINTERFACE_INFO = *mut INTERFACE_INFO;
STRUCT!{struct INTERFACE_INFO_EX {
    iiFlags: ULONG,
    iiAddress: SOCKET_ADDRESS,
    iiBroadcastAddress: SOCKET_ADDRESS,
    iiNetmask: SOCKET_ADDRESS,
}}
pub type LPINTERFACE_INFO_EX = *mut INTERFACE_INFO_EX;
pub const IFF_UP: ULONG = 0x00000001;
pub const IFF_BROADCAST: ULONG = 0x00000002;
pub const IFF_LOOPBACK: ULONG = 0x00000004;
pub const IFF_POINTTOPOINT: ULONG = 0x00000008;
pub const IFF_MULTICAST: ULONG = 0x00000010;
ENUM!{enum PMTUD_STATE {
    IP_PMTUDISC_NOT_SET,
    IP_PMTUDISC_DO,
    IP_PMTUDISC_DONT,
    IP_PMTUDISC_PROBE,
    IP_PMTUDISC_MAX,
}}
pub type PPMTUD_STATE = *mut PMTUD_STATE;
pub const IP_OPTIONS: c_int = 1;
pub const IP_HDRINCL: c_int = 2;
pub const IP_TOS: c_int = 3;
pub const IP_TTL: c_int = 4;
pub const IP_MULTICAST_IF: c_int = 9;
pub const IP_MULTICAST_TTL: c_int = 10;
pub const IP_MULTICAST_LOOP: c_int = 11;
pub const IP_ADD_MEMBERSHIP: c_int = 12;
pub const IP_DROP_MEMBERSHIP: c_int = 13;
pub const IP_DONTFRAGMENT: c_int = 14;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 15;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 16;
pub const IP_BLOCK_SOURCE: c_int = 17;
pub const IP_UNBLOCK_SOURCE: c_int = 18;
pub const IP_PKTINFO: c_int = 19;
pub const IP_HOPLIMIT: c_int = 21;
pub const IP_RECEIVE_BROADCAST: c_int = 22;
pub const IP_RECVIF: c_int = 24;
pub const IP_RECVDSTADDR: c_int = 25;
pub const IP_IFLIST: c_int = 28;
pub const IP_ADD_IFLIST: c_int = 29;
pub const IP_DEL_IFLIST: c_int = 30;
pub const IP_UNICAST_IF: c_int = 31;
pub const IP_RTHDR: c_int = 32;
pub const IP_GET_IFLIST: c_int = 33;
pub const IP_RECVRTHDR: c_int = 38;
pub const IP_TCLASS: c_int = 39;
pub const IP_RECVTCLASS: c_int = 40;
pub const IP_ORIGINAL_ARRIVAL_IF: c_int = 47;
pub const IP_ECN: c_int = 50;
pub const IP_PKTINFO_EX: c_int = 51;
pub const IP_WFP_REDIRECT_RECORDS: c_int = 60;
pub const IP_WFP_REDIRECT_CONTEXT: c_int = 70;
pub const IP_MTU_DISCOVER: c_int = 71;
pub const IP_RECVTOS: c_int = 40;
pub const IP_RECVTTL: c_int = 21;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: c_int = -1;
UNION!{union SOCKADDR_IN6_LH_u {
    [u32; 1],
    sin6_scope_id sin6_scope_id_mut: ULONG,
    sin6_scope_struct sin6_scope_struct_mut: SCOPE_ID,
}}
STRUCT!{struct SOCKADDR_IN6_LH {
    sin6_family: ADDRESS_FAMILY,
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: IN6_ADDR,
    u: SOCKADDR_IN6_LH_u,
}}
pub type PSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
pub type LPSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
STRUCT!{struct SOCKADDR_IN6_W2KSP1 {
    sin6_family: c_short,
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: IN6_ADDR,
    sin6_scope_id: ULONG,
}}
pub type PSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
pub type LPSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
pub type SOCKADDR_IN6 = SOCKADDR_IN6_LH;
pub type PSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
pub type LPSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
STRUCT!{struct SOCKADDR_INET {
    Ipv4: SOCKADDR_IN,
    Ipv6: SOCKADDR_IN6,
    si_family: ADDRESS_FAMILY,
}}
pub type PSOCKADDR_INET = *mut SOCKADDR_INET;
STRUCT!{struct SOCKADDR_IN6_PAIR {
    SourceAddress: PSOCKADDR_IN6,
    DestinationAddress: PSOCKADDR_IN6,
}}
pub type PSOCKADDR_IN6_PAIR = *mut SOCKADDR_IN6_PAIR;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: usize = 64;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: usize = 8;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: usize = 104;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: usize = 96;
pub const IN6ADDR_6TO4PREFIX_LENGTH: usize = 16;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: usize = 32;
#[inline]
pub fn IN6_IS_ADDR_UNSPECIFIED(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    (words[0] == 0) && (words[1] == 0) && (words[2] == 0) && (words[3] == 0) && (words[4] == 0) &&
    (words[5] == 0) && (words[6] == 0) && (words[7] == 0)
}
#[inline]
pub fn IN6_IS_ADDR_LOOPBACK(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    (words[0] == 0) && (words[1] == 0) && (words[2] == 0) && (words[3] == 0) && (words[4] == 0) &&
    (words[5] == 0) && (words[6] == 0) && (words[7] == 0x0100)
}
#[inline]
pub fn IN6_IS_ADDR_MULTICAST(a: &IN6_ADDR) -> bool {
    unsafe { a.u.Byte()[0] == 0xff }
}
#[inline]
pub fn IN6_IS_ADDR_EUI64(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    ((bytes[0] & 0xe0) != 0) && !IN6_IS_ADDR_MULTICAST(a)
}
#[inline]
pub fn IN6_IS_ADDR_SUBNET_ROUTER_ANYCAST(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    IN6_IS_ADDR_EUI64(a) && (words[4] == 0) && (words[5] == 0) && (words[6] == 0) &&
    (words[7] == 0)
}
#[inline]
pub fn IN6_IS_ADDR_SUBNET_RESERVED_ANYCAST(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    IN6_IS_ADDR_EUI64(a) && (words[4] == 0xfffd) && (words[5] == 0xffff) && (words[6] == 0xffff) &&
    ((words[7] & 0x80ff) == 0x80ff)
}
#[inline]
pub fn IN6_IS_ADDR_ANYCAST(a: &IN6_ADDR) -> bool {
    IN6_IS_ADDR_SUBNET_RESERVED_ANYCAST(a) || IN6_IS_ADDR_SUBNET_ROUTER_ANYCAST(a)
}
#[inline]
pub fn IN6_IS_ADDR_LINKLOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    (bytes[0] == 0xfe) && ((bytes[1] & 0xc0) == 0x80)
}
#[inline]
pub fn IN6_IS_ADDR_SITELOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    (bytes[0] == 0xfe) && ((bytes[1] & 0xc0) == 0xc0)
}
#[inline]
pub fn IN6_IS_ADDR_GLOBAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    let High = (bytes[0] & 0xf0) as ULONG;
    (High != 0) && (High != 0xf0)
}
#[inline]
pub fn IN6_IS_ADDR_V4MAPPED(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    (words[0] == 0) && (words[1] == 0) && (words[2] == 0) && (words[3] == 0) && (words[4] == 0) &&
    (words[5] == 0xffff)
}
#[inline]
pub fn IN6_IS_ADDR_V4COMPAT(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    let bytes = unsafe { a.u.Byte() };
    (words[0] == 0) && (words[1] == 0) && (words[2] == 0) && (words[3] == 0) && (words[4] == 0) &&
    (words[5] == 0) && !((words[6] == 0) && (bytes[14] == 0) && ((bytes[15] == 0) ||
    (bytes[15] == 1)))
}
#[inline]
pub fn IN6_IS_ADDR_V4TRANSLATED(a: &IN6_ADDR) -> bool {
    let words = unsafe { a.u.Word() };
    (words[0] == 0) && (words[1] == 0) && (words[2] == 0) && (words[3] == 0) &&
    (words[4] == 0xffff) && (words[5] == 0)
}
#[inline]
pub fn IN6_IS_ADDR_MC_NODELOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    IN6_IS_ADDR_MULTICAST(a) && ((bytes[1] & 0xf) == 1)
}
#[inline]
pub fn IN6_IS_ADDR_MC_LINKLOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    IN6_IS_ADDR_MULTICAST(a) && ((bytes[1] & 0xf) == 2)
}
#[inline]
pub fn IN6_IS_ADDR_MC_SITELOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    IN6_IS_ADDR_MULTICAST(a) && ((bytes[1] & 0xf) == 5)
}
#[inline]
pub fn IN6_IS_ADDR_MC_ORGLOCAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    IN6_IS_ADDR_MULTICAST(a) && ((bytes[1] & 0xf) == 8)
}
#[inline]
pub fn IN6_IS_ADDR_MC_GLOBAL(a: &IN6_ADDR) -> bool {
    let bytes = unsafe { a.u.Byte() };
    IN6_IS_ADDR_MULTICAST(a) && ((bytes[1] & 0xf) == 0xe)
}
macro_rules! _IO {
    ($x:expr, $y:expr) => { IOC_VOID | ($x << 8) | $y }
}
macro_rules! _IOR {
    ($x:expr, $y:expr) => { IOC_OUT | 0x40000 | ($x << 8) | $y }
}
macro_rules! _IOW {
    ($x:expr, $y:expr) => { IOC_IN | 0x40000 | ($x << 8) | $y }
}
pub const SIO_GET_INTERFACE_LIST: ULONG = _IOR!(0x74, 127);
pub const SIO_GET_INTERFACE_LIST_EX: ULONG = _IOR!(0x74, 126);
pub const SIO_SET_MULTICAST_FILTER: ULONG = _IOW!(0x74, 125);
pub const SIO_GET_MULTICAST_FILTER: ULONG = _IOW!(0x74, 124 | IOC_IN);
pub const SIOCSIPMSFILTER: ULONG = SIO_SET_MULTICAST_FILTER;
pub const SIOCGIPMSFILTER: ULONG = SIO_GET_MULTICAST_FILTER;
pub const SIOCSMSFILTER: ULONG = _IOW!(0x74, 126);
pub const SIOCGMSFILTER: ULONG = _IOW!(0x74, 127 | IOC_IN);
pub const SIO_IDEAL_SEND_BACKLOG_QUERY: ULONG = _IOR!(0x74, 123);
pub const SIO_IDEAL_SEND_BACKLOG_CHANGE: ULONG = _IO!(0x74, 122);
pub const MCAST_JOIN_GROUP: c_int = 41;
pub const MCAST_LEAVE_GROUP: c_int = 42;
pub const MCAST_BLOCK_SOURCE: c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: c_int = 44;
pub const MCAST_JOIN_SOURCE_GROUP: c_int = 45;
pub const MCAST_LEAVE_SOURCE_GROUP: c_int = 46;
ENUM!{enum MULTICAST_MODE_TYPE {
    MCAST_INCLUDE = 0,
    MCAST_EXCLUDE,
}}
STRUCT!{struct IP_MREQ {
    imr_multiaddr: IN_ADDR,
    imr_interface: IN_ADDR,
}}
pub type PIP_MREQ = *mut IP_MREQ;
STRUCT!{struct IP_MREQ_SOURCE {
    imr_multiaddr: IN_ADDR,
    imr_sourceaddr: IN_ADDR,
    imr_interface: IN_ADDR,
}}
pub type PIP_MREQ_SOURCE = *mut IP_MREQ_SOURCE;
STRUCT!{struct IP_MSFILTER {
    imsf_multiaddr: IN_ADDR,
    imsf_interface: IN_ADDR,
    imsf_fmode: MULTICAST_MODE_TYPE,
    imsf_numsrc: ULONG,
    imsf_slist: [IN_ADDR; 1],
}}
pub type PIP_MSFILTER = *mut IP_MSFILTER;
pub const IPV6_HOPOPTS: c_int = 1;
pub const IPV6_HDRINCL: c_int = 2;
pub const IPV6_UNICAST_HOPS: c_int = 4;
pub const IPV6_MULTICAST_IF: c_int = 9;
pub const IPV6_MULTICAST_HOPS: c_int = 10;
pub const IPV6_MULTICAST_LOOP: c_int = 11;
pub const IPV6_ADD_MEMBERSHIP: c_int = 12;
pub const IPV6_JOIN_GROUP: c_int = IPV6_ADD_MEMBERSHIP;
pub const IPV6_DROP_MEMBERSHIP: c_int = 13;
pub const IPV6_LEAVE_GROUP: c_int = IPV6_DROP_MEMBERSHIP;
pub const IPV6_DONTFRAG: c_int = 14;
pub const IPV6_PKTINFO: c_int = 19;
pub const IPV6_HOPLIMIT: c_int = 21;
pub const IPV6_PROTECTION_LEVEL: c_int = 23;
pub const IPV6_RECVIF: c_int = 24;
pub const IPV6_RECVDSTADDR: c_int = 25;
pub const IPV6_CHECKSUM: c_int = 26;
pub const IPV6_V6ONLY: c_int = 27;
pub const IPV6_IFLIST: c_int = 28;
pub const IPV6_ADD_IFLIST: c_int = 29;
pub const IPV6_DEL_IFLIST: c_int = 30;
pub const IPV6_UNICAST_IF: c_int = 31;
pub const IPV6_RTHDR: c_int = 32;
pub const IPV6_GET_IFLIST: c_int = 33;
pub const IPV6_RECVRTHDR: c_int = 38;
pub const IPV6_TCLASS: c_int = 39;
pub const IPV6_RECVTCLASS: c_int = 40;
pub const IPV6_ECN: c_int = 50;
pub const IPV6_PKTINFO_EX: c_int = 51;
pub const IPV6_WFP_REDIRECT_RECORDS: c_int = 60;
pub const IPV6_WFP_REDIRECT_CONTEXT: c_int = 70;
pub const IPV6_MTU_DISCOVER: c_int = 71;
pub const IP_UNSPECIFIED_HOP_LIMIT: c_int = -1;
pub const IP_PROTECTION_LEVEL: c_int = IPV6_PROTECTION_LEVEL;
pub const PROTECTION_LEVEL_UNRESTRICTED: UINT = 10;
pub const PROTECTION_LEVEL_EDGERESTRICTED: UINT = 20;
pub const PROTECTION_LEVEL_RESTRICTED: UINT = 30;
pub const PROTECTION_LEVEL_DEFAULT: UINT = -1i32 as u32;
STRUCT!{struct IPV6_MREQ {
    ipv6mr_multiaddr: IN6_ADDR,
    ipv6mr_interface: ULONG,
}}
pub type PIPV6_MREQ = *mut IPV6_MREQ;
STRUCT!{struct GROUP_REQ {
    gr_interface: ULONG,
    gr_group: SOCKADDR_STORAGE,
}}
pub type PGROUP_REQ = *mut GROUP_REQ;
STRUCT!{struct GROUP_SOURCE_REQ {
    gsr_interface: ULONG,
    gsr_group: SOCKADDR_STORAGE,
    gsr_source: SOCKADDR_STORAGE,
}}
pub type PGROUP_SOURCE_REQ = *mut GROUP_SOURCE_REQ;
STRUCT!{struct GROUP_FILTER {
    gf_interface: ULONG,
    gf_group: SOCKADDR_STORAGE,
    gf_fmode: MULTICAST_MODE_TYPE,
    gf_numsrc: ULONG,
    gf_slist: [SOCKADDR_STORAGE; 1],
}}
pub type PGROUP_FILTER = *mut GROUP_FILTER;
STRUCT!{struct IN_PKTINFO {
    ipi_addr: IN_ADDR,
    ipi_ifindex: ULONG,
}}
pub type PIN_PKTINFO = *mut IN_PKTINFO;
STRUCT!{struct IN6_PKTINFO {
    ipi6_addr: IN6_ADDR,
    ipi6_ifindex: ULONG,
}}
pub type PIN6_PKTINFO = *mut IN6_PKTINFO;
STRUCT!{struct IN_PKTINFO_EX {
    pkt_info: IN_PKTINFO,
    scope_id: SCOPE_ID,
}}
pub type PIN_PKTINFO_EX = *mut IN_PKTINFO_EX;
STRUCT!{struct IN6_PKTINFO_EX {
    pkt_info: IN6_PKTINFO,
    scope_id: SCOPE_ID,
}}
pub type PIN6_PKTINFO_EX = *mut IN6_PKTINFO_EX;
pub const INET_ADDRSTRLEN: usize = 22;
pub const INET6_ADDRSTRLEN: usize = 65;
pub const TCP_OFFLOAD_NO_PREFERENCE: DWORD = 0;
pub const TCP_OFFLOAD_NOT_PREFERRED: DWORD = 1;
pub const TCP_OFFLOAD_PREFERRED: DWORD = 2;
pub const TCP_EXPEDITED_1122: DWORD = 0x0002;
pub const TCP_KEEPALIVE: DWORD = 3;
pub const TCP_MAXSEG: DWORD = 4;
pub const TCP_MAXRT: DWORD = 5;
pub const TCP_STDURG: DWORD = 6;
pub const TCP_NOURG: DWORD = 7;
pub const TCP_ATMARK: DWORD = 8;
pub const TCP_NOSYNRETRIES: DWORD = 9;
pub const TCP_TIMESTAMPS: DWORD = 10;
pub const TCP_OFFLOAD_PREFERENCE: DWORD = 11;
pub const TCP_CONGESTION_ALGORITHM: DWORD = 12;
pub const TCP_DELAY_FIN_ACK: DWORD = 13;
pub const TCP_MAXRTMS: DWORD = 14;
pub const TCP_FASTOPEN: DWORD = 15;
pub const TCP_KEEPCNT: DWORD = 16;
