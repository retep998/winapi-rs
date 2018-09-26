// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains API definitions for the traffic control interface
use shared::basetsd::ULONG64;
use shared::guiddef::LPGUID;
use shared::minwindef::{PULONG, UCHAR, ULONG, USHORT};
use shared::ntddndis::NETWORK_ADDRESS_LIST;
use shared::qos::{FLOWSPEC, QOS_OBJECT_HDR};
use um::winnt::{BOOLEAN, HANDLE, LPSTR, LPWSTR, PHANDLE, PVOID, WCHAR};
pub const CURRENT_TCI_VERSION: ULONG = 0x0002;
pub const TC_NOTIFY_IFC_UP: ULONG = 1;
pub const TC_NOTIFY_IFC_CLOSE: ULONG = 2;
pub const TC_NOTIFY_IFC_CHANGE: ULONG = 3;
pub const TC_NOTIFY_PARAM_CHANGED: ULONG = 4;
pub const TC_NOTIFY_FLOW_CLOSE: ULONG = 5;
pub const TC_INVALID_HANDLE: HANDLE = 0 as HANDLE;
pub const MAX_STRING_LENGTH: usize = 256;
FN!{stdcall TCI_NOTIFY_HANDLER(
    ClRegCtx: HANDLE,
    ClIfcCtx: HANDLE,
    Event: ULONG,
    SubCode: HANDLE,
    BufSize: ULONG,
    Buffer: PVOID,
) -> ()}
FN!{stdcall TCI_ADD_FLOW_COMPLETE_HANDLER(
    ClFlowCtx: HANDLE,
    Status: ULONG,
) -> ()}
FN!{stdcall TCI_MOD_FLOW_COMPLETE_HANDLER(
    ClFlowCtx: HANDLE,
    Status: ULONG,
) -> ()}
FN!{stdcall TCI_DEL_FLOW_COMPLETE_HANDLER(
    ClFlowCtx: HANDLE,
    Status: ULONG,
) -> ()}
STRUCT!{struct TCI_CLIENT_FUNC_LIST {
    ClNotifyHandler: TCI_NOTIFY_HANDLER,
    ClAddFlowCompleteHandler: TCI_ADD_FLOW_COMPLETE_HANDLER,
    ClModifyFlowCompleteHandler: TCI_MOD_FLOW_COMPLETE_HANDLER,
    ClDeleteFlowCompleteHandler: TCI_DEL_FLOW_COMPLETE_HANDLER,
}}
pub type PTCI_CLIENT_FUNC_LIST = *mut TCI_CLIENT_FUNC_LIST;
STRUCT!{struct ADDRESS_LIST_DESCRIPTOR {
    MediaType: ULONG,
    AddressList: NETWORK_ADDRESS_LIST,
}}
pub type PADDRESS_LIST_DESCRIPTOR = *mut ADDRESS_LIST_DESCRIPTOR;
STRUCT!{struct TC_IFC_DESCRIPTOR {
    Length: ULONG,
    pInterfaceName: LPWSTR,
    pInterfaceID: LPWSTR,
    AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}}
pub type PTC_IFC_DESCRIPTOR = *mut TC_IFC_DESCRIPTOR;
STRUCT!{struct TC_SUPPORTED_INFO_BUFFER {
    InstanceIDLength: USHORT,
    InstanceID: [WCHAR; MAX_STRING_LENGTH],
    InterfaceLuid: ULONG64,
    AddrListDesc: ADDRESS_LIST_DESCRIPTOR,
}}
pub type PTC_SUPPORTED_INFO_BUFFER = *mut TC_SUPPORTED_INFO_BUFFER;
STRUCT!{struct TC_GEN_FILTER {
    AddressType: USHORT,
    PatternSize: ULONG,
    Pattern: PVOID,
    Mask: PVOID,
}}
pub type PTC_GEN_FILTER = *mut TC_GEN_FILTER;
STRUCT!{struct TC_GEN_FLOW {
    SendingFlowspec: FLOWSPEC,
    ReceivingFlowspec: FLOWSPEC,
    TcObjectsLength: ULONG,
    TcObjects: [QOS_OBJECT_HDR; 1],
}}
pub type PTC_GEN_FLOW = *mut TC_GEN_FLOW;
STRUCT!{struct IP_PATTERN_S_un_S_un_ports {
    s_srcport: USHORT,
    s_dstport: USHORT,
}}
STRUCT!{struct IP_PATTERN_S_un_S_un_icmp {
    s_type: UCHAR,
    s_code: UCHAR,
    filler: USHORT,
}}
UNION!{union IP_PATTERN_S_un {
    [u32; 1],
    S_un_ports S_un_ports_mut: IP_PATTERN_S_un_S_un_ports,
    S_un_icmp S_un_icmp_mut: IP_PATTERN_S_un_S_un_icmp,
    S_Spi S_Spi_mut: ULONG,
}}
STRUCT!{struct IP_PATTERN {
    Reserved1: ULONG,
    Reserved2: ULONG,
    SrcAddr: ULONG,
    DstAddr: ULONG,
    S_un: IP_PATTERN_S_un,
    ProtocolId: UCHAR,
    Reserved3: [UCHAR; 3],
}}
pub type PIP_PATTERN = *mut IP_PATTERN;
STRUCT!{struct IPX_PATTERN_s {
    NetworkAddress: ULONG,
    NodeAddress: [UCHAR; 6],
    Socket: USHORT,
}}
STRUCT!{struct IPX_PATTERN {
    Src: IPX_PATTERN_s,
    Dest: IPX_PATTERN_s,
}}
pub type PIPX_PATTERN = *mut IPX_PATTERN;
STRUCT!{struct ENUMERATION_BUFFER {
    Length: ULONG,
    OwnerProcessId: ULONG,
    FlowNameLength: USHORT,
    FlowName: [WCHAR; MAX_STRING_LENGTH],
    pFlow: PTC_GEN_FLOW,
    NumberOfFilters: ULONG,
    GenericFilter: [TC_GEN_FILTER; 1],
}}
pub type PENUMERATION_BUFFER = *mut ENUMERATION_BUFFER;
extern "system" {
    pub fn TcRegisterClient(
        TciVersion: ULONG,
        ClRegCtx: HANDLE,
        ClientHandlerList: PTCI_CLIENT_FUNC_LIST,
        pClientHandle: PHANDLE,
    ) -> ULONG;
    pub fn TcEnumerateInterfaces(
        ClientHandle: HANDLE,
        pBufferSize: PULONG,
        InterfaceBuffer: PTC_IFC_DESCRIPTOR,
    ) -> ULONG;
    pub fn TcOpenInterfaceA(
        pInterfaceName: LPSTR,
        ClientHandle: HANDLE,
        ClIfcCtx: HANDLE,
        pIfcHandle: PHANDLE,
    ) -> ULONG;
    pub fn TcOpenInterfaceW(
        pInterfaceName: LPWSTR,
        ClientHandle: HANDLE,
        ClIfcCtx: HANDLE,
        pIfcHandle: PHANDLE,
    ) -> ULONG;
    pub fn TcCloseInterface(
        IfcHandle: HANDLE,
    ) -> ULONG;
    pub fn TcQueryInterface(
        IfcHandle: HANDLE,
        pGuidParam: LPGUID,
        NotifyChange: BOOLEAN,
        pBufferSize: PULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcSetInterface(
        IfcHandle: HANDLE,
        pGuidParam: LPGUID,
        BufferSize: ULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcQueryFlowA(
        pFlowName: LPSTR,
        pGuidParam: LPGUID,
        pBufferSize: PULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcQueryFlowW(
        pFlowName: LPWSTR,
        pGuidParam: LPGUID,
        pBufferSize: PULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcSetFlowA(
        pFlowName: LPSTR,
        pGuidParam: LPGUID,
        BufferSize: ULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcSetFlowW(
        pFlowName: LPWSTR,
        pGuidParam: LPGUID,
        BufferSize: ULONG,
        Buffer: PVOID,
    ) -> ULONG;
    pub fn TcAddFlow(
        IfcHandle: HANDLE,
        ClFlowCtx: HANDLE,
        Flags: ULONG,
        pGenericFlow: PTC_GEN_FLOW,
        pFlowHandle: PHANDLE,
    ) -> ULONG;
    pub fn TcGetFlowNameA(
        FlowHandle: HANDLE,
        StrSize: ULONG,
        pFlowName: LPSTR,
    ) -> ULONG;
    pub fn TcGetFlowNameW(
        FlowHandle: HANDLE,
        StrSize: ULONG,
        pFlowName: LPWSTR,
    ) -> ULONG;
    pub fn TcModifyFlow(
        FlowHandle: HANDLE,
        pGenericFlow: PTC_GEN_FLOW,
    ) -> ULONG;
    pub fn TcAddFilter(
        FlowHandle: HANDLE,
        pGenericFilter: PTC_GEN_FILTER,
        pFilterHandle: PHANDLE,
    ) -> ULONG;
    pub fn TcDeregisterClient(
        ClientHandle: HANDLE,
    ) -> ULONG;
    pub fn TcDeleteFlow(
        FlowHandle: HANDLE,
    ) -> ULONG;
    pub fn TcDeleteFilter(
        FilterHandle: HANDLE,
    ) -> ULONG;
    pub fn TcEnumerateFlows(
        IfcHandle: HANDLE,
        pEnumHandle: PHANDLE,
        pFlowCount: PULONG,
        pBufSize: PULONG,
        Buffer: PENUMERATION_BUFFER,
    ) -> ULONG;
}
