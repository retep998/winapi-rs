// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

// #include <iprtrmib.h>
// #include <ipexport.h>
// #include <iptypes.h>
// #include <tcpestats.h>

use ctypes::*;
use shared::minwindef::*;
use shared::basetsd::*;
use shared::inaddr::*;
use shared::in6addr::*;
use shared::ntdef::*;
use shared::ws2def::*;
use shared::guiddef::GUID;
use um::minwinbase::{
    OVERLAPPED, LPOVERLAPPED, 
};
use shared::iprtrmib::*;
use shared::ipexport::*;
use shared::iptypes::*;
use shared::tcpestats::*;
ENUM!{enum NET_ADDRESS_FORMAT {
    NET_ADDRESS_FORMAT_UNSPECIFIED = 0,
    NET_ADDRESS_DNS_NAME,
    NET_ADDRESS_IPV4,
    NET_ADDRESS_IPV6,
}}

// #if defined (_WS2DEF_) && defined (_WS2IPDEF_) && defined(_WINDNS_INCLUDED_)
#[cfg(all(feature = "ws2def", feature = "ws2ipdef"))]
mod ws2def_and_ws2ipdef {
    use super::*;

    STRUCT!{struct NET_NAMED_ADDRESS {
        Address: [WCHAR; DNS_MAX_NAME_BUFFER_LENGTH],
        Port: [WCHAR; 6],
    }}
    UNION!{union NET_ADDRESS {
        NamedAddress: NET_NAMED_ADDRESS,
        Ipv4Address: SOCKADDR_IN,
        Ipv6Address: SOCKADDR_IN6,
        IpAddress: SOCKADDR,
    }}
    STRUCT!{struct NET_ADDRESS_INFO {
        Format: NET_ADDRESS_FORMAT,
        Address: NET_ADDRESS,
    }}
    pub type PNET_ADDRESS_INFO = *mut NET_ADDRESS_INFO;
}
#[cfg(all(feature = "ws2def", feature = "ws2ipdef"))]
pub use ws2def_and_ws2ipdef::*;


extern "system" {
    pub fn GetNumberOfInterfaces(
        pdwNumIf: PDWORD
    ) -> DWORD;
    pub fn GetIfEntry(
        pIfRow: PMIB_IFROW,
    ) -> DWORD;
    pub fn GetIfTable(
        pIfTable: PMIB_IFTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetIpAddrTable(
        pIpAddrTable: PMIB_IPADDRTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetIpNetTable(
        IpNetTable: PMIB_IPNETTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetIpForwardTable(
        pIpForwardTable: PMIB_IPFORWARDTABLE,
        pdwSize: PULONG,
        bOrder: BOOL,
    ) -> DWORD;
    pub fn GetTcpTable(
        TcpTable: PMIB_TCPTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    // https://msdn.microsoft.com/en-us/library/windows/desktop/aa365928(v=vs.85).aspx
    pub fn GetExtendedTcpTable(
        pTcpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: TCP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetOwnerModuleFromTcpEntry(
        pTcpEntry: PMIB_TCPROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetUdpTable(
        UdpTable: PMIB_UDPTABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetExtendedUdpTable(
        pUdpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetOwnerModuleFromUdpEntry(
        pUdpEntry: PMIB_UDPROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetTcpTable2(
        TcpTable: PMIB_TCPTABLE2,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    // pub fn AllocateAndGetTcpExTableFromStack() -> DWORD;
    // pub fn AllocateAndGetUdpExTableFromStack() -> DWORD;
    pub fn GetTcp6Table(
        TcpTable: PMIB_TCP6TABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetTcp6Table2(
        TcpTable: PMIB_TCP6TABLE2,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    // pub fn GetPerTcpConnectionEStats() -> ULONG;
    // pub fn GetPerTcp6ConnectionEStats() -> ULONG;
    // pub fn SetPerTcp6ConnectionEStats() -> ULONG;
    pub fn GetOwnerModuleFromTcp6Entry(
        pTcpEntry: PMIB_TCP6ROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    pub fn GetUdp6Table(
        Udp6Table: PMIB_UDP6TABLE,
        SizePointer: PULONG,
        Order: BOOL,
    ) -> ULONG;
    pub fn GetOwnerModuleFromUdp6Entry(
        pUdpEntry: PMIB_UDP6ROW_OWNER_MODULE,
        Class: TCPIP_OWNER_MODULE_INFO_CLASS,
        pBuffer: PVOID,
        pdwSize: PDWORD,
    ) -> DWORD;
    // pub fn GetOwnerModuleFromPidAndInfo() -> DWORD;
    pub fn GetIpStatistics(
        Statistics: PMIB_IPSTATS,
    ) -> ULONG;
    pub fn GetIcmpStatistics(
        Statistics: PMIB_ICMP,
    ) -> ULONG;
    pub fn GetTcpStatistics(
        Statistics: PMIB_TCPSTATS,
    ) -> ULONG;
    pub fn GetUdpStatistics(
        Stats: PMIB_UDPSTATS,
    ) -> ULONG;
    pub fn SetIpStatisticsEx(
        Statistics: PMIB_IPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetIpStatisticsEx(
        Statistics: PMIB_IPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetIcmpStatisticsEx(
        Statistics: PMIB_ICMP_EX,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetTcpStatisticsEx(
        Statistics: PMIB_TCPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetUdpStatisticsEx(
        Statistics: PMIB_UDPSTATS,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetTcpStatisticsEx2(
        Statistics: PMIB_TCPSTATS2,
        Family: ULONG,
    ) -> ULONG;
    pub fn GetUdpStatisticsEx2(
        Statistics: PMIB_UDPSTATS2,
        Family: ULONG,
    ) -> ULONG;
    pub fn SetIfEntry(
        pIfRow: PMIB_IFROW,
    ) -> DWORD;
    pub fn CreateIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn SetIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn DeleteIpForwardEntry(
        pRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn SetIpStatistics(
        pIpStats: PMIB_IPSTATS,
    ) -> DWORD;
    pub fn SetIpTTL(
        nTTL: UINT,
    ) -> DWORD;
    pub fn CreateIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn SetIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn DeleteIpNetEntry(
        pArpEntry: PMIB_IPNETROW,
    ) -> DWORD;
    pub fn FlushIpNetTable(
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn CreateProxyArpEntry(
        dwAddress: DWORD,
        dwMask: DWORD,
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn DeleteProxyArpEntry(
        dwAddress: DWORD,
        dwMask: DWORD,
        dwIfIndex: DWORD,
    ) -> DWORD;
    pub fn SetTcpEntry(
        pTcpRow: PMIB_TCPROW,
    ) -> DWORD;
    pub fn GetInterfaceInfo(
        pIfTable: PIP_INTERFACE_INFO,
        dwOutBufLen: PULONG,
    ) -> DWORD;
    pub fn GetUniDirectionalAdapterInfo(
        pIPIfInfo: PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS,
        dwOutBufLen: PULONG,
    ) -> DWORD;
    // pub fn NhpAllocateAndGetInterfaceInfoFromStack() -> DWORD;
    pub fn GetBestInterface(
        dwDestAddr: IPAddr,
        pdwBestIfIndex: PDWORD,
    ) -> DWORD;
    pub fn GetBestInterfaceEx(
        pDestAddr: *mut sockaddr,
        pdwBestIfIndex: PDWORD,
    ) -> DWORD;
    pub fn GetBestRoute(
        dwDestAddr: DWORD,
        dwSourceAddr: DWORD,
        pBestRoute: PMIB_IPFORWARDROW,
    ) -> DWORD;
    pub fn NotifyAddrChange(
        Handle: PHANDLE,
        overlapped: LPOVERLAPPED,
    ) -> DWORD;
    pub fn NotifyRouteChange(
        Handle: PHANDLE,
        overlapped: LPOVERLAPPED,
    ) -> DWORD;
    pub fn CancelIPChangeNotify(
        notifyOverlapped: LPOVERLAPPED
    ) -> BOOL;
    pub fn GetAdapterIndex(
        AdapterName: LPWSTR,
        IfIndex: PULONG,
    ) -> DWORD;
    pub fn AddIPAddress(
        Address: IPAddr,
        IpMask: IPMask,
        IfIndex: DWORD,
        NTEContext: PULONG,
        NTEInstance: PULONG,
    ) -> DWORD;
    pub fn DeleteIPAddress(
        NTEContext: ULONG,
    ) -> DWORD;
    pub fn GetNetworkParams(
        pFixedInfo: PFIXED_INFO,
        pOutBufLen: PULONG,
    ) -> DWORD;
    pub fn GetAdaptersInfo(
        AdapterInfo: PIP_ADAPTER_INFO,
        SizePointer: PULONG,
    ) -> ULONG;
    // pub fn GetAdapterOrderMap(VOID) -> PIP_ADAPTER_ORDER_MAP;
    pub fn GetAdaptersAddresses(
        Family: ULONG,
        Flags: ULONG,
        Reserved: PVOID,
        AdapterAddresses: PIP_ADAPTER_ADDRESSES,
        SizePointer: PULONG,
    ) -> ULONG;
    pub fn GetPerAdapterInfo(
        IfIndex: ULONG,
        pPerAdapterInfo: PIP_PER_ADAPTER_INFO,
        pOutBufLen: PULONG,
    ) -> DWORD;
    pub fn IpReleaseAddress(
        AdapterInfo: PIP_ADAPTER_INDEX_MAP,
    ) -> DWORD;
    pub fn IpRenewAddress(
        AdapterInfo: PIP_ADAPTER_INDEX_MAP,
    ) -> DWORD;
    pub fn SendARP(
        DestIP: IPAddr,
        SrcIP: IPAddr,
        pMacAddr: PVOID,
        PhyAddrLen: PULONG,
    ) -> DWORD;
    pub fn GetRTTAndHopCount(
        DestIpAddress: IPAddr,
        HopCount: PULONG,
        MaxHops: ULONG,
        RTT: PULONG,
    ) -> BOOL;
    pub fn GetFriendlyIfIndex(
        IfIndex: DWORD,
    ) -> DWORD;
    pub fn EnableRouter(
        pHandle: *mut HANDLE,
        pOverlapped: *mut OVERLAPPED,
    ) -> DWORD;
    pub fn UnenableRouter(
        pOverlapped: *mut OVERLAPPED,
        lpdwEnableCount: LPDWORD,
    ) -> DWORD;
    pub fn DisableMediaSense(
        pHandle: *mut HANDLE,
        pOverLapped: *mut OVERLAPPED,
    ) -> DWORD;
    pub fn RestoreMediaSense(
        pOverlapped: *mut OVERLAPPED,
        lpdwEnableCount: LPDWORD,
    ) -> DWORD;
    pub fn GetIpErrorString(
        ErrorCode: IP_STATUS,
        Buffer: PWSTR,
        Size: PDWORD,
    ) -> DWORD;
    pub fn ResolveNeighbor(
        NetworkAddress: *mut SOCKADDR,
        PhysicalAddress: PVOID,
        PhysicalAddressLength: PULONG,
    ) -> ULONG;
    pub fn CreatePersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn CreatePersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn DeletePersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
    ) -> ULONG;
    pub fn DeletePersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
    ) -> ULONG;
    pub fn LookupPersistentTcpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    pub fn LookupPersistentUdpPortReservation(
        StartPort: USHORT,
        NumberOfPorts: USHORT,
        Token: PULONG64,
    ) -> ULONG;
    // #if defined (_WS2DEF_) && defined (_WS2IPDEF_) && defined(_WINDNS_INCLUDED_)
    #[cfg(all(feature = "ws2def", feature = "ws2ipdef"))]
    pub fn ParseNetworkString(
        NetworkString: *const *mut WCHAR,
        Types: DWORD,
        AddressInfo: PNET_ADDRESS_INFO,
        PortNumber: *mut USHORT,
        PrefixLength: *mut BYTE,
    ) -> DWORD;
}
