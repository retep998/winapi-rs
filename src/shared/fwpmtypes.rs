// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! WFP Structures and Definitions
use ctypes::wchar_t;
use shared::basetsd::{INT32, UINT8, UINT16, UINT32, UINT64};
use shared::fwptypes::{
    FWP_ACTION_TYPE, FWP_AF, FWP_BYTE_ARRAY6, FWP_BYTE_ARRAY16, FWP_BYTE_BLOB,
    FWP_CLASSIFY_OPTION_TYPE, FWP_CONDITION_VALUE0, FWP_DATA_TYPE, FWP_DIRECTION,
    FWPM_DISPLAY_DATA0, FWP_FILTER_ENUM_TYPE, FWP_IP_VERSION, FWP_MATCH_TYPE, FWP_VALUE0
};
use shared::guiddef::GUID;
use shared::iketypes::{
    IKEEXT_AUTHENTICATION_METHOD_TYPE, IKEEXT_CREDENTIAL2, IKEEXT_EM_SA_STATE,
    IKEEXT_KEY_MODULE_TYPE, IKEEXT_MM_SA_STATE, IKEEXT_POLICY0, IKEEXT_POLICY1, IKEEXT_POLICY2,
    IKEEXT_PROPOSAL0, IKEEXT_QM_SA_STATE, IKEEXT_SA_ROLE
};
use shared::ipsectypes::{
    IPSEC_DOSP_OPTIONS0, IPSEC_FAILURE_POINT, IPSEC_KEYING_POLICY0, IPSEC_KEYING_POLICY1,
    IPSEC_SA_SPI, IPSEC_TRAFFIC_TYPE, IPSEC_TRANSPORT_POLICY0, IPSEC_TRANSPORT_POLICY1,
    IPSEC_TRANSPORT_POLICY2, IPSEC_TUNNEL_POLICY0, IPSEC_TUNNEL_POLICY1, IPSEC_TUNNEL_POLICY2,
    IPSEC_TUNNEL_POLICY3
};
use shared::minwindef::{BOOL, DWORD, FILETIME, UINT, ULONG};
use um::winnt::{LPWSTR, SID};

ENUM!{enum FWPM_CHANGE_TYPE {
    FWPM_CHANGE_ADD = 1,
    FWPM_CHANGE_DELETE = FWPM_CHANGE_ADD + 1,
    FWPM_CHANGE_TYPE_MAX = FWPM_CHANGE_DELETE + 1,
}}

pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_ADD: DWORD = 0x00000001;
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_DELETE: DWORD = 0x00000002;

ENUM!{enum FWPM_SERVICE_STATE {
    FWPM_SERVICE_STOPPED = 0,
    FWPM_SERVICE_START_PENDING = FWPM_SERVICE_STOPPED + 1,
    FWPM_SERVICE_STOP_PENDING = FWPM_SERVICE_START_PENDING + 1,
    FWPM_SERVICE_RUNNING = FWPM_SERVICE_STOP_PENDING + 1,
    FWPM_SERVICE_STATE_MAX = FWPM_SERVICE_RUNNING + 1,
}}

pub const FWPM_NET_EVENT_KEYWORD_INBOUND_MCAST: DWORD = 0x00000001;
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_BCAST: DWORD = 0x00000002;
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_DROP: DWORD = 0x00000004;
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_ALLOW: DWORD = 0x00000008;
pub const FWPM_NET_EVENT_KEYWORD_CLASSIFY_ALLOW: DWORD = 0x00000010;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_NONE: DWORD = 0x00000000;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_INBOUND: DWORD = 0x00000001;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_FORWARD: DWORD = 0x00000002;
pub const FWPM_ENGINE_OPTION_PACKET_BATCH_INBOUND: DWORD = 0x00000004;

ENUM!{enum FWPM_ENGINE_OPTION {
    FWPM_ENGINE_COLLECT_NET_EVENTS = 0,
    FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS = FWPM_ENGINE_COLLECT_NET_EVENTS + 1,
    FWPM_ENGINE_NAME_CACHE = FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS + 1,
    FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS = FWPM_ENGINE_NAME_CACHE + 1,
    FWPM_ENGINE_PACKET_QUEUING = FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS + 1,
    FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC = FWPM_ENGINE_PACKET_QUEUING + 1,
    FWPM_ENGINE_OPTION_MAX = FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC + 1,
}}

pub const FWPM_SESSION_FLAG_DYNAMIC: DWORD = 0x00000001;
pub const FWPM_SESSION_FLAG_RESERVED: DWORD = 0x10000000;

STRUCT!{struct FWPM_SESSION0 {
    sessionKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    txnWaitTimeoutInMSec: UINT32,
    processId: DWORD,
    sid: *mut SID,
    username: *mut wchar_t,
    kernelMode: BOOL,
}}

STRUCT!{struct FWPM_SESSION_ENUM_TEMPLATE0 {
    reserved: UINT64,
}}

pub const FWPM_PROVIDER_FLAG_PERSISTENT: DWORD = 0x00000001;
pub const FWPM_PROVIDER_FLAG_DISABLED: DWORD = 0x00000010;

STRUCT!{struct FWPM_PROVIDER0 {
    providerKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerData: FWP_BYTE_BLOB,
    serviceName: *mut wchar_t,
}}

STRUCT!{struct FWPM_PROVIDER_ENUM_TEMPLATE0 {
    reserved: UINT64,
}}

STRUCT!{struct FWPM_PROVIDER_CHANGE0 {
    changeType: FWPM_CHANGE_TYPE,
    providerKey: GUID,
}}

STRUCT!{struct FWPM_PROVIDER_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_PROVIDER_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

pub const FWPM_PROVIDER_CONTEXT_FLAG_PERSISTENT: DWORD = 0x00000001;
pub const FWPM_PROVIDER_CONTEXT_FLAG_DOWNLEVEL: DWORD = 0x00000002;

STRUCT!{struct FWPM_CLASSIFY_OPTION0 {
    type_: FWP_CLASSIFY_OPTION_TYPE,
    value: FWP_VALUE0,
}}

STRUCT!{struct FWPM_CLASSIFY_OPTIONS0 {
    numOptions: UINT32,
    options: *mut FWPM_CLASSIFY_OPTION0,
}}

ENUM!{enum FWPM_PROVIDER_CONTEXT_TYPE {
    FWPM_IPSEC_KEYING_CONTEXT = 0,
    FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT = FWPM_IPSEC_KEYING_CONTEXT + 1,
    FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT = FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT + 1,
    FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT = FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT + 1,
    FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT =  FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT + 1,
    FWPM_IPSEC_IKE_MM_CONTEXT = FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT + 1,
    FWPM_IPSEC_AUTHIP_MM_CONTEXT = FWPM_IPSEC_IKE_MM_CONTEXT + 1,
    FWPM_CLASSIFY_OPTIONS_CONTEXT = FWPM_IPSEC_AUTHIP_MM_CONTEXT + 1,
    FWPM_GENERAL_CONTEXT = FWPM_CLASSIFY_OPTIONS_CONTEXT + 1,
    FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT = FWPM_GENERAL_CONTEXT + 1,
    FWPM_IPSEC_IKEV2_MM_CONTEXT = FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT + 1,
    FWPM_IPSEC_DOSP_CONTEXT = FWPM_IPSEC_IKEV2_MM_CONTEXT + 1,
    FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT = FWPM_IPSEC_DOSP_CONTEXT + 1,
    FWPM_PROVIDER_CONTEXT_TYPE_MAX = FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT + 1,
}}

UNION2!{union FWPM_PROVIDER_CONTEXT0_u {
    [u32; 1] [u64; 1],
    keyingPolicy keyingPolicy_mut: *mut IPSEC_KEYING_POLICY0,
    ikeQmTransportPolicy ikeQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY0,
    ikeQmTunnelPolicy ikeQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY0,
    authipQmTransportPolicy authipQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY0,
    authipQmTunnelPolicy authipQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY0,
    ikeMmPolicy ikeMmPolicy_mut: *mut IKEEXT_POLICY0,
    authIpMmPolicy authIpMmPolicy_mut: *mut IKEEXT_POLICY0,
    dataBuffer dataBuffer_mut: *mut FWP_BYTE_BLOB,
    classifyOptions classifyOptions_mut: *mut FWPM_CLASSIFY_OPTIONS0,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT0 {
    providerContextKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    type_: FWPM_PROVIDER_CONTEXT_TYPE,
	u: FWPM_PROVIDER_CONTEXT0_u,
    providerContextId: UINT64,
}}

UNION2!{union FWPM_PROVIDER_CONTEXT1_u {
    [u32; 1] [u64; 1],
    keyingPolicy keyingPolicy_mut: *mut IPSEC_KEYING_POLICY0,
    ikeQmTransportPolicy ikeQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY1,
    ikeQmTunnelPolicy ikeQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY1,
    authipQmTransportPolicy authipQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY1,
    authipQmTunnelPolicy authipQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY1,
    ikeMmPolicy ikeMmPolicy_mut: *mut IKEEXT_POLICY1,
    authIpMmPolicy authIpMmPolicy_mut: *mut IKEEXT_POLICY1,
    dataBuffer dataBuffer_mut: *mut FWP_BYTE_BLOB,
    classifyOptions classifyOptions_mut: *mut FWPM_CLASSIFY_OPTIONS0,
    ikeV2QmTunnelPolicy ikeV2QmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY1,
    ikeV2MmPolicy ikeV2MmPolicy_mut: *mut IKEEXT_POLICY1,
    idpOptions idpOptions_mut: *mut IPSEC_DOSP_OPTIONS0,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT1 {
    providerContextKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    type_: FWPM_PROVIDER_CONTEXT_TYPE,
    u: FWPM_PROVIDER_CONTEXT1_u,
    providerContextId: UINT64,
}}

UNION2!{union FWPM_PROVIDER_CONTEXT2_u {
    [u32; 1] [u64; 1],
    keyingPolicy keyingPolicy_mut: *mut IPSEC_KEYING_POLICY1,
    ikeQmTransportPolicy ikeQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    ikeQmTunnelPolicy ikeQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY2,
    authipQmTransportPolicy authipQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    authipQmTunnelPolicy authipQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY2,
    ikeMmPolicy ikeMmPolicy_mut: *mut IKEEXT_POLICY2,
    authIpMmPolicy authIpMmPolicy_mut: *mut IKEEXT_POLICY2,
    dataBuffer dataBuffer_mut: *mut FWP_BYTE_BLOB,
    classifyOptions classifyOptions_mut: *mut FWPM_CLASSIFY_OPTIONS0,
    ikeV2QmTunnelPolicy ikeV2QmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY2,
    ikeV2QmTransportPolicy ikeV2QmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    ikeV2MmPolicy ikeV2MmPolicy_mut: *mut IKEEXT_POLICY2,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT2 {
    providerContextKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    type_: FWPM_PROVIDER_CONTEXT_TYPE,
	u: FWPM_PROVIDER_CONTEXT2_u,
    providerContextId: UINT64,
}}

UNION2!{union FWPM_PROVIDER_CONTEXT3_u {
    [u32; 1] [u64; 1],
    keyingPolicy keyingPolicy_mut: *mut IPSEC_KEYING_POLICY1,
    ikeQmTransportPolicy ikeQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    ikeQmTunnelPolicy ikeQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY3,
    authipQmTransportPolicy authipQmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    authipQmTunnelPolicy authipQmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY3,
    ikeMmPolicy ikeMmPolicy_mut: *mut IKEEXT_POLICY2,
    authIpMmPolicy authIpMmPolicy_mut: *mut IKEEXT_POLICY2,
    dataBuffer dataBuffer_mut: *mut FWP_BYTE_BLOB,
    classifyOptions classifyOptions_mut: *mut FWPM_CLASSIFY_OPTIONS0,
    ikeV2QmTunnelPolicy ikeV2QmTunnelPolicy_mut: *mut IPSEC_TUNNEL_POLICY3,
    ikeV2QmTransportPolicy ikeV2QmTransportPolicy_mut: *mut IPSEC_TRANSPORT_POLICY2,
    ikeV2MmPolicy ikeV2MmPolicy_mut: *mut IKEEXT_POLICY2,
    idpOptions idpOptions_mut: *mut IPSEC_DOSP_OPTIONS0,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT3 {
    providerContextKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    type_: FWPM_PROVIDER_CONTEXT_TYPE,
	u: FWPM_PROVIDER_CONTEXT3_u,
    providerContextId: UINT64,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    providerKey: *mut GUID,
    providerContextType: FWPM_PROVIDER_CONTEXT_TYPE,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT_CHANGE0 {
    changeType: FWPM_CHANGE_TYPE,
    providerContextKey: GUID,
    providerContextId: UINT64,
}}

STRUCT!{struct FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

pub const FWPM_SUBLAYER_FLAG_PERSISTENT: UINT = 0x00000001;

STRUCT!{struct FWPM_SUBLAYER0 {
    subLayerKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    weight: UINT16,
}}

STRUCT!{struct FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    providerKey: *mut GUID,
}}

STRUCT!{struct FWPM_SUBLAYER_CHANGE0 {
    changeType: FWPM_CHANGE_TYPE,
    subLayerKey: GUID,
}}

STRUCT!{struct FWPM_SUBLAYER_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_SUBLAYER_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

pub const FWPM_LAYER_FLAG_KERNEL: UINT = 0x00000001;
pub const FWPM_LAYER_FLAG_BUILTIN: UINT = 0x00000002;
pub const FWPM_LAYER_FLAG_CLASSIFY_MOSTLY: UINT = 0x00000004;
pub const FWPM_LAYER_FLAG_BUFFERED: UINT = 0x00000008;

ENUM!{enum FWPM_FIELD_TYPE {
    FWPM_FIELD_RAW_DATA = 0,
    FWPM_FIELD_IP_ADDRESS = FWPM_FIELD_RAW_DATA + 1,
    FWPM_FIELD_FLAGS = FWPM_FIELD_IP_ADDRESS + 1,
    FWPM_FIELD_TYPE_MAX = FWPM_FIELD_FLAGS + 1,
}}

STRUCT!{struct FWPM_FIELD0 {
    fieldKey: *mut GUID,
    type_: FWPM_FIELD_TYPE,
    dataType: FWP_DATA_TYPE,
}}

STRUCT!{struct FWPM_LAYER0 {
    layerKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    numFields: UINT32,
    field: *mut FWPM_FIELD0,
    defaultSubLayerKey: GUID,
    layerId: UINT16,
}}

STRUCT!{struct FWPM_LAYER_ENUM_TEMPLATE0 {
    reserved: UINT64,
}}

pub const FWPM_CALLOUT_FLAG_PERSISTENT: UINT = 0x00010000;
pub const FWPM_CALLOUT_FLAG_USES_PROVIDER_CONTEXT: UINT = 0x00020000;
pub const FWPM_CALLOUT_FLAG_REGISTERED: UINT = 0x00040000;

STRUCT!{struct FWPM_CALLOUT0 {
    calloutKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    applicableLayer: GUID,
    calloutId: UINT32,
}}

STRUCT!{struct FWPM_CALLOUT_ENUM_TEMPLATE0 {
    providerKey: *mut GUID,
    layerKey: GUID,
}}

STRUCT!{struct FWPM_CALLOUT_CHANGE0 {
    changeType: FWPM_CHANGE_TYPE,
    calloutKey: GUID,
    calloutId: UINT32,
}}

STRUCT!{struct FWPM_CALLOUT_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_CALLOUT_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

UNION2!{union FWPM_ACTION0_u {
    [u32; 4],
    filterType filterType_mut: GUID,
    calloutKey calloutKey_mut: GUID,
    bitmapIndex bitmapIndex_mut: UINT8,
}}

STRUCT!{struct FWPM_ACTION0 {
    type_: FWP_ACTION_TYPE,
    u: FWPM_ACTION0_u,
}}

STRUCT!{struct FWPM_FILTER_CONDITION0 {
    fieldKey: GUID,
    matchType: FWP_MATCH_TYPE,
    conditionValue: FWP_CONDITION_VALUE0,
}}

pub const FWPM_FILTER_FLAG_NONE: UINT = 0x00000000;
pub const FWPM_FILTER_FLAG_PERSISTENT: UINT = 0x00000001;
pub const FWPM_FILTER_FLAG_BOOTTIME: UINT = 0x00000002;
pub const FWPM_FILTER_FLAG_HAS_PROVIDER_CONTEXT: UINT = 0x00000004;
pub const FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT: UINT = 0x00000008;
pub const FWPM_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: UINT = 0x00000010;
pub const FWPM_FILTER_FLAG_DISABLED: UINT = 0x00000020;
pub const FWPM_FILTER_FLAG_INDEXED: UINT = 0x00000040;
pub const FWPM_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: UINT = 0x00000080;
pub const FWPM_FILTER_FLAG_SYSTEMOS_ONLY: UINT = 0x00000100;
pub const FWPM_FILTER_FLAG_GAMEOS_ONLY: UINT = 0x00000200;
pub const FWPM_FILTER_FLAG_SILENT_MODE: UINT = 0x00000400;
pub const FWPM_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: UINT = 0x00000800;

UNION2!{union FWPM_FILTER0_u {
    [u64; 2],
    rawContext rawContext_mut: UINT64,
    providerContextKey providerContextKey_mut: GUID,
}}


STRUCT!{struct FWPM_FILTER0 {
    filterKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    providerKey: *mut GUID,
    providerData: FWP_BYTE_BLOB,
    layerKey: GUID,
    subLayerKey: GUID,
    weight: FWP_VALUE0,
    numFilterConditions: UINT32,
    filterCondition: *mut FWPM_FILTER_CONDITION0,
    action: FWPM_ACTION0,
    u: FWPM_FILTER0_u,
	reserved: *mut GUID,
    filterId: UINT64,
    effectiveWeight: FWP_VALUE0,
}}

STRUCT!{struct FWPM_FILTER_ENUM_TEMPLATE0 {
    providerKey: *mut GUID,
    layerKey: GUID,
    enumType: FWP_FILTER_ENUM_TYPE,
    flags: UINT32,
    providerContextTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    numFilterConditions: UINT32,
    filterCondition: *mut FWPM_FILTER_CONDITION0,
    actionMask: UINT32,
    calloutKey: *mut GUID,
}}

STRUCT!{struct FWPM_FILTER_CHANGE0 {
    changeType: FWPM_CHANGE_TYPE,
    filterKey: GUID,
    filterId: UINT64,
}}

STRUCT!{struct FWPM_FILTER_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_FILTER_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

STRUCT!{struct FWPM_LAYER_STATISTICS0 {
    layerId: GUID,
    classifyPermitCount: UINT32,
    classifyBlockCount: UINT32,
    classifyVetoCount: UINT32,
    numCacheEntries: UINT32,
}}

STRUCT!{struct FWPM_STATISTICS0 {
    numLayerStatistics: UINT32,
    layerStatistics: *mut FWPM_LAYER_STATISTICS0,
    inboundAllowedConnectionsV4: UINT32,
    inboundBlockedConnectionsV4: UINT32,
    outboundAllowedConnectionsV4: UINT32,
    outboundBlockedConnectionsV4: UINT32,
    inboundAllowedConnectionsV6: UINT32,
    inboundBlockedConnectionsV6: UINT32,
    outboundAllowedConnectionsV6: UINT32,
    outboundBlockedConnectionsV6: UINT32,
    inboundActiveConnectionsV4: UINT32,
    outboundActiveConnectionsV4: UINT32,
    inboundActiveConnectionsV6: UINT32,
    outboundActiveConnectionsV6: UINT32,
    reauthDirInbound: UINT64,
    reauthDirOutbound: UINT64,
    reauthFamilyV4: UINT64,
    reauthFamilyV6: UINT64,
    reauthProtoOther: UINT64,
    reauthProtoIPv4: UINT64,
    reauthProtoIPv6: UINT64,
    reauthProtoICMP: UINT64,
    reauthProtoICMP6: UINT64,
    reauthProtoUDP: UINT64,
    reauthProtoTCP: UINT64,
    reauthReasonPolicyChange: UINT64,
    reauthReasonNewArrivalInterface: UINT64,
    reauthReasonNewNextHopInterface: UINT64,
    reauthReasonProfileCrossing: UINT64,
    reauthReasonClassifyCompletion: UINT64,
    reauthReasonIPSecPropertiesChanged: UINT64,
    reauthReasonMidStreamInspection: UINT64,
    reauthReasonSocketPropertyChanged: UINT64,
    reauthReasonNewInboundMCastBCastPacket: UINT64,
    reauthReasonEDPPolicyChanged: UINT64,
    reauthReasonLocalAddressUniFiltersChanged: UINT64,
    reauthReasonRemoteAddressUniFiltersChanged: UINT64,
    reauthReasonLocalPortUniFiltersChanges: UINT64,
    reauthReasonRemotePortUniFiltersChanges: UINT64,
    reauthReasonProxyHandleChanged: UINT64,
}}

pub const FWPM_NET_EVENT_FLAG_IP_PROTOCOL_SET: UINT = 0x00000001;
pub const FWPM_NET_EVENT_FLAG_LOCAL_ADDR_SET: UINT = 0x00000002;
pub const FWPM_NET_EVENT_FLAG_REMOTE_ADDR_SET: UINT = 0x00000004;
pub const FWPM_NET_EVENT_FLAG_LOCAL_PORT_SET: UINT = 0x00000008;
pub const FWPM_NET_EVENT_FLAG_REMOTE_PORT_SET: UINT = 0x00000010;
pub const FWPM_NET_EVENT_FLAG_APP_ID_SET: UINT = 0x00000020;
pub const FWPM_NET_EVENT_FLAG_USER_ID_SET: UINT = 0x00000040;
pub const FWPM_NET_EVENT_FLAG_SCOPE_ID_SET: UINT = 0x00000080;
pub const FWPM_NET_EVENT_FLAG_IP_VERSION_SET: UINT = 0x00000100;
pub const FWPM_NET_EVENT_FLAG_REAUTH_REASON_SET: UINT = 0x00000200;
pub const FWPM_NET_EVENT_FLAG_PACKAGE_ID_SET: UINT = 0x00000400;
pub const FWPM_NET_EVENT_FLAG_ENTERPRISE_ID_SET: UINT = 0x00000800;
pub const FWPM_NET_EVENT_FLAG_POLICY_FLAGS_SET: UINT = 0x00001000;
pub const FWPM_NET_EVENT_FLAG_EFFECTIVE_NAME_SET: UINT = 0x00002000;

UNION2!{union FWPM_NET_EVENT_HEADER0_u1 {
    [u32; 4],
    localAddrV4 localAddrV4_mut: UINT32,
    localAddrV6 localAddrV6_mut: FWP_BYTE_ARRAY16,
}}

UNION2!{union FWPM_NET_EVENT_HEADER0_u2 {
    [u32; 4],
    remoteAddrV4 remoteAddrV4_mut: UINT32,
    remoteAddrV6 remoteAddrV6_mut: FWP_BYTE_ARRAY16,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER0 {
    timeStamp: FILETIME,
    flags: UINT32,
    ipVersion: FWP_IP_VERSION,
    ipProtocol: UINT8,
    u1: FWPM_NET_EVENT_HEADER0_u1,
    u2: FWPM_NET_EVENT_HEADER0_u2,
    localPort: UINT16,
    remotePort: UINT16,
    scopeId: UINT32,
    appId: FWP_BYTE_BLOB,
    userId: *mut SID,
}}

UNION2!{union FWPM_NET_EVENT_HEADER1_u1 {
    [u32; 4],
    localAddrV4 localAddrV4_mut: UINT32,
    localAddrV6 localAddrV6_mut: FWP_BYTE_ARRAY16,
}}

UNION2!{union FWPM_NET_EVENT_HEADER1_u2 {
    [u32; 4],
    remoteAddrV4 remoteAddrV4_mut: UINT32,
    remoteAddrV6 remoteAddrV6_mut: FWP_BYTE_ARRAY16,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER1_s_s {
    reserved2: FWP_BYTE_ARRAY6,
    reserved3: FWP_BYTE_ARRAY6,
    reserved4: UINT32,
    reserved5: UINT32,
    reserved6: UINT16,
    reserved7: UINT32,
    reserved8: UINT32,
    reserved9: UINT16,
    reserved10: UINT64,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER1_s {
    reserved1: FWP_AF,
    s: FWPM_NET_EVENT_HEADER1_s_s,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER1 {
    timeStamp: FILETIME,
    flags: UINT32,
    ipVersion: FWP_IP_VERSION,
    ipProtocol: UINT8,
    u1: FWPM_NET_EVENT_HEADER1_u1,
    u2: FWPM_NET_EVENT_HEADER1_u2,
    localPort: UINT16,
    remotePort: UINT16,
    scopeId: UINT32,
    appId: FWP_BYTE_BLOB,
    userId: *mut SID,
    s: FWPM_NET_EVENT_HEADER1_s,
}}

UNION2!{union FWPM_NET_EVENT_HEADER2_u1 {
    [u32; 4],
    localAddrV4 localAddrV4_mut: UINT32,
    localAddrV6 localAddrV6_mut: FWP_BYTE_ARRAY16,
}}

UNION2!{union FWPM_NET_EVENT_HEADER2_u2 {
    [u32; 4],
    remoteAddrV4 remoteAddrV4_mut: UINT32,
    remoteAddrV6 remoteAddrV6_mut: FWP_BYTE_ARRAY16,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER2 {
    timeStamp: FILETIME,
    flags: UINT32,
    ipVersion: FWP_IP_VERSION,
    ipProtocol: UINT8,
    u1: FWPM_NET_EVENT_HEADER2_u1,
    u2: FWPM_NET_EVENT_HEADER2_u2,
    localPort: UINT16,
    remotePort: UINT16,
    scopeId: UINT32,
    appId: FWP_BYTE_BLOB,
    userId: *mut SID,
    addressFamily: FWP_AF,
    packageSid: *mut SID,
}}

UNION2!{union FWPM_NET_EVENT_HEADER3_u1 {
    [u32; 4],
    localAddrV4 localAddrV4_mut: UINT32,
    localAddrV6 localAddrV6_mut: FWP_BYTE_ARRAY16,
}}

UNION2!{union FWPM_NET_EVENT_HEADER3_u2 {
    [u32; 4],
    remoteAddrV4 remoteAddrV4_mut: UINT32,
    remoteAddrV6 remoteAddrV6_mut: FWP_BYTE_ARRAY16,
}}

STRUCT!{struct FWPM_NET_EVENT_HEADER3 {
    timeStamp: FILETIME,
    flags: UINT32,
    ipVersion: FWP_IP_VERSION,
    ipProtocol: UINT8,
    u1: FWPM_NET_EVENT_HEADER3_u1,
    u2: FWPM_NET_EVENT_HEADER3_u2,
    localPort: UINT16,
    remotePort: UINT16,
    scopeId: UINT32,
    appId: FWP_BYTE_BLOB,
    userId: *mut SID,
    addressFamily: FWP_AF,
    packageSid: *mut SID,
    enterpriseId: *mut wchar_t,
    policyFlags: UINT64,
    effectiveName: FWP_BYTE_BLOB,
}}

ENUM!{enum FWPM_NET_EVENT_TYPE {
    FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE = 0,
    FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE = FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE + 1,
    FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE = FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE + 1,
    FWPM_NET_EVENT_TYPE_CLASSIFY_DROP = FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE + 1,
    FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP = FWPM_NET_EVENT_TYPE_CLASSIFY_DROP + 1,
    FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP = FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP + 1,
    FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW = FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP + 1,
    FWPM_NET_EVENT_TYPE_CAPABILITY_DROP = FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW + 1,
    FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW = FWPM_NET_EVENT_TYPE_CAPABILITY_DROP + 1,
    FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC = FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW + 1,
    FWPM_NET_EVENT_TYPE_MAX = FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC + 1,
}}

pub const IKEEXT_CERT_HASH_LEN: UINT = 20;
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_BENIGN: UINT = 0x00000001;
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_MULTIPLE: UINT = 0x00000002;

STRUCT!{struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    failureErrorCode: UINT32,
    failurePoint: IPSEC_FAILURE_POINT,
    flags: UINT32,
    keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    mmState: IKEEXT_MM_SA_STATE,
    saRole: IKEEXT_SA_ROLE,
    mmAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    endCertHash: [UINT8; 20],
    mmId: UINT64,
    mmFilterId: UINT64,
}}

STRUCT!{struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    failureErrorCode: UINT32,
    failurePoint: IPSEC_FAILURE_POINT,
    flags: UINT32,
    keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    mmState: IKEEXT_MM_SA_STATE,
    saRole: IKEEXT_SA_ROLE,
    mmAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    endCertHash: [UINT8; 20],
    mmId: UINT64,
    mmFilterId: UINT64,
    localPrincipalNameForAuth: *mut wchar_t,
    remotePrincipalNameForAuth: *mut wchar_t,
    numLocalPrincipalGroupSids: UINT32,
    localPrincipalGroupSids: *mut LPWSTR,
    numRemotePrincipalGroupSids: UINT32,
    remotePrincipalGroupSids: *mut LPWSTR,
}}

STRUCT!{struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    failureErrorCode: UINT32,
    failurePoint: IPSEC_FAILURE_POINT,
    keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    qmState: IKEEXT_QM_SA_STATE,
    saRole: IKEEXT_SA_ROLE,
    saTrafficType: IPSEC_TRAFFIC_TYPE,
    // Single field anonymous union
    localSubNet: FWP_CONDITION_VALUE0,
    // Single field anonymous union
    remoteSubNet: FWP_CONDITION_VALUE0,
    qmFilterId: UINT64,
}}

pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_MULTIPLE: UINT = 0x00000001;
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_BENIGN: UINT = 0x00000002;

STRUCT!{struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    failureErrorCode: UINT32,
    failurePoint: IPSEC_FAILURE_POINT,
    flags: UINT32,
    emState: IKEEXT_EM_SA_STATE,
    saRole: IKEEXT_SA_ROLE,
    emAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    endCertHash: [UINT8; 20],
    mmId: UINT64,
    qmFilterId: UINT64,
}}

STRUCT!{struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    failureErrorCode: UINT32,
    failurePoint: IPSEC_FAILURE_POINT,
    flags: UINT32,
    emState: IKEEXT_EM_SA_STATE,
    saRole: IKEEXT_SA_ROLE,
    emAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    endCertHash: [UINT8; 20],
    mmId: UINT64,
    qmFilterId: UINT64,
    localPrincipalNameForAuth: *mut wchar_t,
    remotePrincipalNameForAuth: *mut wchar_t,
    numLocalPrincipalGroupSids: UINT32,
    localPrincipalGroupSids: *mut LPWSTR,
    numRemotePrincipalGroupSids: UINT32,
    remotePrincipalGroupSids: *mut LPWSTR,
    saTrafficType: IPSEC_TRAFFIC_TYPE,
}}

STRUCT!{struct FWPM_NET_EVENT_CLASSIFY_DROP0 {
    filterId: UINT64,
    layerId: UINT16,
}}

STRUCT!{struct FWPM_NET_EVENT_CLASSIFY_DROP1 {
    filterId: UINT64,
    layerId: UINT16,
    reauthReason: UINT32,
    originalProfile: UINT32,
    currentProfile: UINT32,
    msFwpDirection: UINT32,
    isLoopback: BOOL,
}}

STRUCT!{struct FWPM_NET_EVENT_CLASSIFY_DROP2 {
    filterId: UINT64,
    layerId: UINT16,
    reauthReason: UINT32,
    originalProfile: UINT32,
    currentProfile: UINT32,
    msFwpDirection: UINT32,
    isLoopback: BOOL,
    vSwitchId: FWP_BYTE_BLOB,
    vSwitchSourcePort: UINT32,
    vSwitchDestinationPort: UINT32,
}}

STRUCT!{struct FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    localMacAddr: FWP_BYTE_ARRAY6,
    remoteMacAddr: FWP_BYTE_ARRAY6,
    mediaType: UINT32,
    ifType: UINT32,
    etherType: UINT16,
    ndisPortNumber: UINT32,
    reserved: UINT32,
    vlanTag: UINT16,
    ifLuid: UINT64,
    filterId: UINT64,
    layerId: UINT16,
    reauthReason: UINT32,
    originalProfile: UINT32,
    currentProfile: UINT32,
    msFwpDirection: UINT32,
    isLoopback: BOOL,
    vSwitchId: FWP_BYTE_BLOB,
    vSwitchSourcePort: UINT32,
    vSwitchDestinationPort: UINT32,
}}

STRUCT!{struct FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    filterId: UINT64,
    layerId: UINT16,
    reauthReason: UINT32,
    originalProfile: UINT32,
    currentProfile: UINT32,
    msFwpDirection: UINT32,
    isLoopback: BOOL,
}}

STRUCT!{struct FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    failureStatus: INT32,
    direction: FWP_DIRECTION,
    spi: IPSEC_SA_SPI,
    filterId: UINT64,
    layerId: UINT16,
}}

UNION2!{union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_u1 {
    [u32; 4],
    publicHostV4Addr publicHostV4Addr_mut: UINT32,
    publicHostV6Addr publicHostV6Addr_mut: [UINT8; 16],
}}

UNION2!{union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_u2 {
    [u32; 4],
    internalHostV4Addr internalHostV4Addr_mut: UINT32,
    internalHostV6Addr internalHostV6Addr_mut: [UINT8; 16],
}}

STRUCT!{struct FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    ipVersion: FWP_IP_VERSION,
	u1: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_u1,
	u2: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_u2,
    failureStatus: INT32,
    direction: FWP_DIRECTION,
}}

ENUM!{enum FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT = 0,
    FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER =
        FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT + 1,
    FWPM_APPC_NETWORK_CAPABILITY_INTERNET_PRIVATE_NETWORK =
        FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER + 1,
}}

STRUCT!{struct FWPM_NET_EVENT_CAPABILITY_DROP0 {
    networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    filterId: UINT64,
    isLoopback: BOOL,
}}

STRUCT!{struct FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    filterId: UINT64,
    isLoopback: BOOL,
}}

UNION2!{union FWPM_NET_EVENT0_u {
    [u32; 1] [u64; 1],
    ikeMmFailure ikeMmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE0,
    ikeQmFailure ikeQmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    ikeEmFailure ikeEmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE0,
    classifyDrop classifyDrop_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP0,
    ipsecDrop ipsecDrop_mut: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    idpDrop idpDrop_mut: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}}

STRUCT!{struct FWPM_NET_EVENT0 {
    header: FWPM_NET_EVENT_HEADER0,
    type_: FWPM_NET_EVENT_TYPE,
	u: FWPM_NET_EVENT0_u,
}}

UNION2!{union FWPM_NET_EVENT1_u {
    [u32; 1] [u64; 1],
    ikeMmFailure ikeMmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    ikeQmFailure ikeQmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    ikeEmFailure ikeEmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    classifyDrop classifyDrop_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP1,
    ipsecDrop ipsecDrop_mut: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    idpDrop idpDrop_mut: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}}

STRUCT!{struct FWPM_NET_EVENT1 {
    header: FWPM_NET_EVENT_HEADER1,
    type_: FWPM_NET_EVENT_TYPE,
    u: FWPM_NET_EVENT1_u,
}}

UNION2!{union FWPM_NET_EVENT2_u {
    [u32; 1] [u64; 1],
    ikeMmFailure ikeMmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    ikeQmFailure ikeQmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    ikeEmFailure ikeEmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    classifyDrop classifyDrop_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    ipsecDrop ipsecDrop_mut: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    idpDrop idpDrop_mut: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    classifyAllow classifyAllow_mut: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    capabilityDrop capabilityDrop_mut: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    capabilityAllow capabilityAllow_mut: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    classifyDropMac classifyDropMac_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}}

STRUCT!{struct FWPM_NET_EVENT2 {
    header: FWPM_NET_EVENT_HEADER2,
    type_: FWPM_NET_EVENT_TYPE,
    u: FWPM_NET_EVENT2_u,
}}

UNION2!{union FWPM_NET_EVENT3_u {
    [u32; 1] [u64; 1],
    ikeMmFailure ikeMmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    ikeQmFailure ikeQmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    ikeEmFailure ikeEmFailure_mut: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    classifyDrop classifyDrop_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    ipsecDrop ipsecDrop_mut: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    idpDrop idpDrop_mut: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    classifyAllow classifyAllow_mut: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    capabilityDrop capabilityDrop_mut: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    capabilityAllow capabilityAllow_mut: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    classifyDropMac classifyDropMac_mut: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}}

STRUCT!{struct FWPM_NET_EVENT3 {
    header: FWPM_NET_EVENT_HEADER3,
    type_: FWPM_NET_EVENT_TYPE,
	u: FWPM_NET_EVENT3_u,
}}

STRUCT!{struct FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    startTime: FILETIME,
    endTime: FILETIME,
    numFilterConditions: UINT32,
    filterCondition: *mut FWPM_FILTER_CONDITION0,
}}

STRUCT!{struct FWPM_NET_EVENT_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_NET_EVENT_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

ENUM!{enum FWPM_SYSTEM_PORT_TYPE {
    FWPM_SYSTEM_PORT_RPC_EPMAP = 0,
    FWPM_SYSTEM_PORT_TEREDO = FWPM_SYSTEM_PORT_RPC_EPMAP + 1,
    FWPM_SYSTEM_PORT_IPHTTPS_IN = FWPM_SYSTEM_PORT_TEREDO + 1,
    FWPM_SYSTEM_PORT_IPHTTPS_OUT = FWPM_SYSTEM_PORT_IPHTTPS_IN + 1,
    FWPM_SYSTEM_PORT_TYPE_MAX = FWPM_SYSTEM_PORT_IPHTTPS_OUT + 1,
}}

STRUCT!{struct FWPM_SYSTEM_PORTS_BY_TYPE0 {
    type_: FWPM_SYSTEM_PORT_TYPE,
    numPorts: UINT32,
    ports: *mut UINT16,
}}

STRUCT!{struct FWPM_SYSTEM_PORTS0 {
    numTypes: UINT32,
    types: *mut FWPM_SYSTEM_PORTS_BY_TYPE0,
}}

UNION2!{union FWPM_CONNECTION0_u1 {
    [u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union FWPM_CONNECTION0_u2 {
    [u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

STRUCT!{struct FWPM_CONNECTION0 {
    connectionId: UINT64,
    ipVersion: FWP_IP_VERSION,
    u1: FWPM_CONNECTION0_u1,
    u2: FWPM_CONNECTION0_u2,
    providerKey: *mut GUID,
    ipsecTrafficModeType: IPSEC_TRAFFIC_TYPE,
    keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    mmCrypto: IKEEXT_PROPOSAL0,
    mmPeer: IKEEXT_CREDENTIAL2,
    emPeer: IKEEXT_CREDENTIAL2,
    bytesTransferredIn: UINT64,
    bytesTransferredOut: UINT64,
    bytesTransferredTotal: UINT64,
    startSysTime: FILETIME,
}}

pub const FWPM_CONNECTION_ENUM_FLAG_QUERY_BYTES_TRANSFERRED: UINT = 0x00000001;

STRUCT!{struct FWPM_CONNECTION_ENUM_TEMPLATE0 {
    connectionId: UINT64,
    flags: UINT32,
}}

STRUCT!{struct FWPM_CONNECTION_SUBSCRIPTION0 {
    enumTemplate: *mut FWPM_CONNECTION_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

ENUM!{enum FWPM_CONNECTION_EVENT_TYPE {
    FWPM_CONNECTION_EVENT_ADD = 0,
    FWPM_CONNECTION_EVENT_DELETE = FWPM_CONNECTION_EVENT_ADD + 1,
    FWPM_CONNECTION_EVENT_MAX = FWPM_CONNECTION_EVENT_DELETE + 1,
}}

ENUM!{enum FWPM_VSWITCH_EVENT_TYPE {
    FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER = 0,
    FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION =
        FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER + 1,
    FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION =
        FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION + 1,
    FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION = FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION + 1,
    FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER = FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION + 1,
    FWPM_VSWITCH_EVENT_MAX = FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER + 1,
}}

STRUCT!{struct FWPM_VSWITCH_EVENT0_u_positionInfo {
    numvSwitchFilterExtensions: ULONG,
    vSwitchFilterExtensions: *mut LPWSTR,
}}

STRUCT!{struct FWPM_VSWITCH_EVENT0_u_reorderInfo {
    inRequiredPosition: BOOL,
    numvSwitchFilterExtensions: ULONG,
    vSwitchFilterExtensions: *mut LPWSTR,
}}

UNION2!{union FWPM_VSWITCH_EVENT0_u {
    [u32; 3] [u64; 2],
    positionInfo positionInfo_mut: FWPM_VSWITCH_EVENT0_u_positionInfo,
    reorderInfo reorderInfo_mut: FWPM_VSWITCH_EVENT0_u_reorderInfo,
}}
	
STRUCT!{struct FWPM_VSWITCH_EVENT0 {
    eventType: FWPM_VSWITCH_EVENT_TYPE,
    vSwitchId: *mut wchar_t,
    u: FWPM_VSWITCH_EVENT0_u,
}}

STRUCT!{struct FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    flags: UINT32,
    sessionKey: GUID,
}}
