// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{SIZE_T, ULONG64};
use shared::guiddef::GUID;
use shared::ifdef::{NET_IFINDEX, NET_LUID, PNET_IFINDEX, PNET_LUID};
use shared::minwindef::{DWORD, ULONG};
use shared::nldef::{
    NL_INTERFACE_OFFLOAD_ROD, NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    NL_ROUTER_DISCOVERY_BEHAVIOR
};
use shared::ntdef::{BOOLEAN, CHAR, PSTR, PWSTR, WCHAR};
use shared::ws2def::{ADDRESS_FAMILY, ScopeLevelCount};
pub type NETIO_STATUS = DWORD;
pub type NETIOAPI_API = NETIO_STATUS;
STRUCT!{struct MIB_IPINTERFACE_ROW {
    Family: ADDRESS_FAMILY,
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    MaxReassemblySize: ULONG,
    InterfaceIdentifier: ULONG64,
    MinRouterAdvertisementInterval: ULONG,
    MaxRouterAdvertisementInterval: ULONG,
    AdvertisingEnabled: BOOLEAN,
    ForwardingEnabled: BOOLEAN,
    WeakHostSend: BOOLEAN,
    WeakHostReceive: BOOLEAN,
    UseAutomaticMetric: BOOLEAN,
    UseNeighborUnreachabilityDetection: BOOLEAN,
    ManagedAddressConfigurationSupported: BOOLEAN,
    OtherStatefulConfigurationSupported: BOOLEAN,
    AdvertiseDefaultRoute: BOOLEAN,
    RouterDiscoveryBehavior: NL_ROUTER_DISCOVERY_BEHAVIOR,
    DadTransmits: ULONG,
    BaseReachableTime: ULONG,
    RetransmitTime: ULONG,
    PathMtuDiscoveryTimeout: ULONG,
    LinkLocalAddressBehavior: NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    LinkLocalAddressTimeout: ULONG,
    ZoneIndices: [ULONG; ScopeLevelCount as usize],
    SitePrefixLength: ULONG,
    Metric: ULONG,
    NlMtu: ULONG,
    Connected: BOOLEAN,
    SupportsWakeUpPatterns: BOOLEAN,
    SupportsNeighborDiscovery: BOOLEAN,
    SupportsRouterDiscovery: BOOLEAN,
    ReachableTime: ULONG,
    TransmitOffload: NL_INTERFACE_OFFLOAD_ROD,
    ReceiveOffload: NL_INTERFACE_OFFLOAD_ROD,
    DisableDefaultRoutes: BOOLEAN,
}}
pub type PMIB_IPINTERFACE_ROW = *mut MIB_IPINTERFACE_ROW;
extern "system" {
    pub fn GetIpInterfaceEntry(
        Row: PMIB_IPINTERFACE_ROW,
    ) -> NETIOAPI_API;
    pub fn SetIpInterfaceEntry(
        Row: PMIB_IPINTERFACE_ROW,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceNameToLuidA(
        InterfaceName: *const CHAR,
        InterfaceLuid: *mut NET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceNameToLuidW(
        InterfaceName: *const WCHAR,
        InterfaceLuid: *mut NET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToNameA(
        InterfaceLuid: *const NET_LUID,
        InterfaceName: PSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToNameW(
        InterfaceLuid: *const NET_LUID,
        InterfaceName: PWSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToIndex(
        InterfaceLuid: *const NET_LUID,
        InterfaceIndex: PNET_IFINDEX,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceIndexToLuid(
        InterfaceIndex: NET_IFINDEX,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToAlias(
        InterfaceLuid: *const NET_LUID,
        InterfaceAlias: PWSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceAliasToLuid(
        InterfaceAlias: *const WCHAR,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToGuid(
        InterfaceLuid: *const NET_LUID,
        InterfaceGuid: *mut GUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceGuidToLuid(
        InterfaceGuid: *const GUID,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
}
