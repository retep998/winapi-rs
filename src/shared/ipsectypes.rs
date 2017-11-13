// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::wchar_t;
use shared::basetsd::{UINT8, UINT16, UINT32, UINT64};
use shared::fwptypes::{FWP_BYTE_ARRAY16, FWP_BYTE_BLOB, FWP_CONDITION_VALUE0, FWP_DIRECTION, FWPM_DISPLAY_DATA0, FWP_IP_VERSION, FWP_V6_ADDR_AND_MASK, IPSEC_VIRTUAL_IF_TUNNEL_INFO0};
use shared::guiddef::GUID;
use shared::minwindef::UINT;

STRUCT!{struct IPSEC_SA_LIFETIME0 {
    lifetimeSeconds: UINT32,
    lifetimeKilobytes: UINT32,
    lifetimePackets: UINT32,
}}

ENUM!{enum IPSEC_TRANSFORM_TYPE {
    IPSEC_TRANSFORM_AH = 1,
    IPSEC_TRANSFORM_ESP_AUTH = IPSEC_TRANSFORM_AH + 1,
    IPSEC_TRANSFORM_ESP_CIPHER = IPSEC_TRANSFORM_ESP_AUTH + 1,
    IPSEC_TRANSFORM_ESP_AUTH_AND_CIPHER = IPSEC_TRANSFORM_ESP_CIPHER + 1,
    IPSEC_TRANSFORM_ESP_AUTH_FW = IPSEC_TRANSFORM_ESP_AUTH_AND_CIPHER + 1,
    IPSEC_TRANSFORM_TYPE_MAX = IPSEC_TRANSFORM_ESP_AUTH_FW + 1,
}}

ENUM!{enum IPSEC_AUTH_TYPE {
    IPSEC_AUTH_MD5 = 0,
    IPSEC_AUTH_SHA_1 = IPSEC_AUTH_MD5 + 1,
    IPSEC_AUTH_SHA_256 = IPSEC_AUTH_SHA_1 + 1,
    IPSEC_AUTH_AES_128 = IPSEC_AUTH_SHA_256 + 1,
    IPSEC_AUTH_AES_192 = IPSEC_AUTH_AES_128 + 1,
    IPSEC_AUTH_AES_256 = IPSEC_AUTH_AES_192 + 1,
    IPSEC_AUTH_MAX = IPSEC_AUTH_AES_256 + 1,
}}

pub type IPSEC_AUTH_CONFIG = UINT8;

pub const IPSEC_AUTH_CONFIG_HMAC_MD5_96: UINT = 0;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_1_96: UINT = 1;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_256_128: UINT = 2;
pub const IPSEC_AUTH_CONFIG_GCM_AES_128: UINT = 3;
pub const IPSEC_AUTH_CONFIG_GCM_AES_192: UINT = 4;
pub const IPSEC_AUTH_CONFIG_GCM_AES_256: UINT = 5;
pub const IPSEC_AUTH_CONFIG_MAX: UINT = 6;

STRUCT!{struct IPSEC_AUTH_TRANSFORM_ID0 {
    authType: IPSEC_AUTH_TYPE,
    authConfig: IPSEC_AUTH_CONFIG,
}}

pub type IPSEC_CRYPTO_MODULE_ID = GUID;

STRUCT!{struct IPSEC_AUTH_TRANSFORM0 {
    authTransformId: IPSEC_AUTH_TRANSFORM_ID0,
    cryptoModuleId: *mut IPSEC_CRYPTO_MODULE_ID,
}}

ENUM!{enum IPSEC_CIPHER_TYPE {
    IPSEC_CIPHER_TYPE_DES = 1,
    IPSEC_CIPHER_TYPE_3DES = IPSEC_CIPHER_TYPE_DES + 1,
    IPSEC_CIPHER_TYPE_AES_128 = IPSEC_CIPHER_TYPE_3DES + 1,
    IPSEC_CIPHER_TYPE_AES_192 = IPSEC_CIPHER_TYPE_AES_128 + 1,
    IPSEC_CIPHER_TYPE_AES_256 = IPSEC_CIPHER_TYPE_AES_192 + 1,
    IPSEC_CIPHER_TYPE_MAX = IPSEC_CIPHER_TYPE_AES_256 + 1,
}}

pub type IPSEC_CIPHER_CONFIG = UINT8;

pub const IPSEC_CIPHER_CONFIG_CBC_DES: UINT = 1;
pub const IPSEC_CIPHER_CONFIG_CBC_3DES: UINT = 2;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_128: UINT = 3;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_192: UINT = 4;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_256: UINT = 5;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_128: UINT = 6;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_192: UINT = 7;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_256: UINT = 8;
pub const IPSEC_CIPHER_CONFIG_MAX: UINT = 9;

STRUCT!{struct IPSEC_CIPHER_TRANSFORM_ID0 {
    cipherType: IPSEC_CIPHER_TYPE,
    cipherConfig: IPSEC_CIPHER_CONFIG,
}}

STRUCT!{struct IPSEC_CIPHER_TRANSFORM0 {
    cipherTransformId: IPSEC_CIPHER_TRANSFORM_ID0,
    cryptoModuleId: *mut IPSEC_CRYPTO_MODULE_ID,
}}

STRUCT!{struct IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    authTransform: IPSEC_AUTH_TRANSFORM0,
    cipherTransform: IPSEC_CIPHER_TRANSFORM0,
}}

UNION2!{union IPSEC_SA_TRANSFORM0_u {
	[u32; 1] [u64; 1],
    ahTransform ahTransform_mut: *mut IPSEC_AUTH_TRANSFORM0,
    espAuthTransform espAuthTransform_mut: *mut IPSEC_AUTH_TRANSFORM0,
    espCipherTransform espCipherTransform_mut: *mut IPSEC_CIPHER_TRANSFORM0,
    espAuthAndCipherTransform espAuthAndCipherTransform_mut: *mut IPSEC_AUTH_AND_CIPHER_TRANSFORM0,
    espAuthFwTransform espAuthFwTransform_mut: *mut IPSEC_AUTH_TRANSFORM0,
}}

STRUCT!{struct IPSEC_SA_TRANSFORM0 {
    ipsecTransformType: IPSEC_TRANSFORM_TYPE,
	u: IPSEC_SA_TRANSFORM0_u,
}}

ENUM!{enum IPSEC_PFS_GROUP {
    IPSEC_PFS_NONE = 0,
    IPSEC_PFS_1 = IPSEC_PFS_NONE + 1,
    IPSEC_PFS_2 = IPSEC_PFS_1 + 1,
    IPSEC_PFS_2048 = IPSEC_PFS_2 + 1,
    IPSEC_PFS_14 = IPSEC_PFS_2048,
    IPSEC_PFS_ECP_256 = IPSEC_PFS_14 + 1,
    IPSEC_PFS_ECP_384 = IPSEC_PFS_ECP_256 + 1,
    IPSEC_PFS_MM = IPSEC_PFS_ECP_384 + 1,
    IPSEC_PFS_24 = IPSEC_PFS_MM + 1,
    IPSEC_PFS_MAX = IPSEC_PFS_24 + 1,
}}

STRUCT!{struct IPSEC_PROPOSAL0 {
    lifetime: IPSEC_SA_LIFETIME0,
    numSaTransforms: UINT32,
    saTransforms: *mut IPSEC_SA_TRANSFORM0,
    pfsGroup: IPSEC_PFS_GROUP,
}}

STRUCT!{struct IPSEC_SA_IDLE_TIMEOUT0 {
    idleTimeoutSeconds: UINT32,
    idleTimeoutSecondsFailOver: UINT32,
}}

UNION2!{union IPSEC_TRAFFIC_SELECTOR0_u1 {
	[u32; 4],
    startV4Address startV4Address_mut: UINT32,
    startV6Address startV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TRAFFIC_SELECTOR0_u2 {
	[u32; 4],
    endV4Address endV4Address_mut: UINT32,
    endV6Address endV6Address_mut: [UINT8; 16],
}}

STRUCT!{struct IPSEC_TRAFFIC_SELECTOR0 {
    protocolId: UINT8,
    portStart: UINT16,
    portEnd: UINT16,
    ipVersion: FWP_IP_VERSION,
    u1: IPSEC_TRAFFIC_SELECTOR0_u1,
    u2: IPSEC_TRAFFIC_SELECTOR0_u2,
}}

STRUCT!{struct IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    flags: UINT32,
    numLocalTrafficSelectors: UINT32,
    localTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0,
    numRemoteTrafficSelectors: UINT32,
    remoteTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0,
}}

pub const IPSEC_POLICY_FLAG_ND_SECURE: UINT = 0x00000002;
pub const IPSEC_POLICY_FLAG_ND_BOUNDARY: UINT = 0x00000004;
pub const IPSEC_POLICY_FLAG_CLEAR_DF_ON_TUNNEL: UINT = 0x00000008;
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_PEER_BEHIND_NAT: UINT = 0x00000010;
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_GENERAL_NAT_TRAVERSAL: UINT = 0x00000020;
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_SECOND_LIFETIME: UINT = 0x00000040;
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_BYTE_LIFETIME: UINT = 0x00000080;
pub const IPSEC_POLICY_FLAG_ENABLE_V6_IN_V4_TUNNELING: UINT = 0x00000100;
pub const IPSEC_POLICY_FLAG_ENABLE_SERVER_ADDR_ASSIGNMENT: UINT = 0x00000200;
pub const IPSEC_POLICY_FLAG_TUNNEL_ALLOW_OUTBOUND_CLEAR_CONNECTION: UINT = 0x00000400;
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ALREADY_SECURE_CONNECTION: UINT = 0x00000800;
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ICMPV6: UINT = 0x00001000;
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_DICTATE_KEY: UINT = 0x00002000;
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_NOTIFY_KEY: UINT = 0x00004000;
pub const IPSEC_POLICY_FLAG_RESERVED1: UINT = 0x00008000;
pub const IPSEC_POLICY_FLAG_SITE_TO_SITE_TUNNEL: UINT = 0x00010000;

//TODO: remove this placeholder
STRUCT!{struct IKEEXT_EM_POLICY0 {
    lol: UINT,
}}

STRUCT!{struct IPSEC_TRANSPORT_POLICY0 {
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    flags: UINT32,
    ndAllowClearTimeoutSeconds: UINT32,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY0,
}}

//TODO: remove
STRUCT!{struct IKEEXT_EM_POLICY1 {
    lol: UINT,
}}

STRUCT!{struct IPSEC_TRANSPORT_POLICY1 {
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    flags: UINT32,
    ndAllowClearTimeoutSeconds: UINT32,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY1,
}}

//TODO: remove
STRUCT!{struct IKEEXT_EM_POLICY2 {
    lol: UINT,
}}
	
STRUCT!{struct IPSEC_TRANSPORT_POLICY2 {
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    flags: UINT32,
    ndAllowClearTimeoutSeconds: UINT32,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY2,
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS0_u1 {
	[u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS0_u2 {
	[u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}


STRUCT!{struct IPSEC_TUNNEL_ENDPOINTS0 {
    ipVersion: FWP_IP_VERSION,
    u1: IPSEC_TUNNEL_ENDPOINTS0_u1,
    u2: IPSEC_TUNNEL_ENDPOINTS0_u2,
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINT0_u {
	[u32; 4],
    v4Address v4Address_mut: UINT32,
    v6Address v6Address_mut: [UINT8; 16],
}}

STRUCT!{struct IPSEC_TUNNEL_ENDPOINT0 {
    ipVersion: FWP_IP_VERSION,
    u: IPSEC_TUNNEL_ENDPOINT0_u,
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS2_u1 {
	[u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS2_u2 {
	[u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

STRUCT!{struct IPSEC_TUNNEL_ENDPOINTS2 {
    ipVersion: FWP_IP_VERSION,
	u1: IPSEC_TUNNEL_ENDPOINTS2_u1,
	u2: IPSEC_TUNNEL_ENDPOINTS2_u2,
    localIfLuid: UINT64,
    remoteFqdn: *mut wchar_t,
    numAddresses: UINT32,
    remoteAddresses: *mut IPSEC_TUNNEL_ENDPOINT0,
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS1_u1 {
	[u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TUNNEL_ENDPOINTS1_u2 {
	[u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

STRUCT!{struct IPSEC_TUNNEL_ENDPOINTS1 {
    ipVersion: FWP_IP_VERSION,
    u1: IPSEC_TUNNEL_ENDPOINTS1_u1,
    u2: IPSEC_TUNNEL_ENDPOINTS1_u2,
    localIfLuid: UINT64,
}}

STRUCT!{struct IPSEC_TUNNEL_POLICY0 {
    flags: UINT32,
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS0,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY0,
}}

STRUCT!{struct IPSEC_TUNNEL_POLICY1 {
    flags: UINT32,
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS1,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY1,
}}

STRUCT!{struct IPSEC_TUNNEL_POLICY2 {
    flags: UINT32,
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY2,
    fwdPathSaLifetime: UINT32,
}}

STRUCT!{struct IPSEC_TUNNEL_POLICY3 {
    flags: UINT32,
    numIpsecProposals: UINT32,
    ipsecProposals: *mut IPSEC_PROPOSAL0,
    tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    emPolicy: *mut IKEEXT_EM_POLICY2,
    fwdPathSaLifetime: UINT32,
    compartmentId: UINT32,
    numTrafficSelectorPolicy: UINT32,
    trafficSelectorPolicies: *mut IPSEC_TRAFFIC_SELECTOR_POLICY0,
}}

STRUCT!{struct IPSEC_KEYING_POLICY0 {
    numKeyMods: UINT32,
    keyModKeys: *mut GUID,
}}

STRUCT!{struct IPSEC_KEYING_POLICY1 {
    numKeyMods: UINT32,
    keyModKeys: *mut GUID,
    flags: UINT32,
}}

STRUCT!{struct IPSEC_AGGREGATE_SA_STATISTICS0 {
    activeSas: UINT32,
    pendingSaNegotiations: UINT32,
    totalSasAdded: UINT32,
    totalSasDeleted: UINT32,
    successfulRekeys: UINT32,
    activeTunnels: UINT32,
    offloadedSas: UINT32,
}}

STRUCT!{struct IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    invalidSpisOnInbound: UINT32,
    decryptionFailuresOnInbound: UINT32,
    authenticationFailuresOnInbound: UINT32,
    replayCheckFailuresOnInbound: UINT32,
    saNotInitializedOnInbound: UINT32,
}}

STRUCT!{struct IPSEC_AH_DROP_PACKET_STATISTICS0 {
    invalidSpisOnInbound: UINT32,
    authenticationFailuresOnInbound: UINT32,
    replayCheckFailuresOnInbound: UINT32,
    saNotInitializedOnInbound: UINT32,
}}

STRUCT!{struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    invalidSpisOnInbound: UINT32,
    decryptionFailuresOnInbound: UINT32,
    authenticationFailuresOnInbound: UINT32,
    udpEspValidationFailuresOnInbound: UINT32,
    replayCheckFailuresOnInbound: UINT32,
    invalidClearTextInbound: UINT32,
    saNotInitializedOnInbound: UINT32,
    receiveOverIncorrectSaInbound: UINT32,
    secureReceivesNotMatchingFilters: UINT32,
}}

STRUCT!{struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    invalidSpisOnInbound: UINT32,
    decryptionFailuresOnInbound: UINT32,
    authenticationFailuresOnInbound: UINT32,
    udpEspValidationFailuresOnInbound: UINT32,
    replayCheckFailuresOnInbound: UINT32,
    invalidClearTextInbound: UINT32,
    saNotInitializedOnInbound: UINT32,
    receiveOverIncorrectSaInbound: UINT32,
    secureReceivesNotMatchingFilters: UINT32,
    totalDropPacketsInbound: UINT32,
}}

STRUCT!{struct IPSEC_TRAFFIC_STATISTICS0 {
    encryptedByteCount: UINT64,
    authenticatedAHByteCount: UINT64,
    authenticatedESPByteCount: UINT64,
    transportByteCount: UINT64,
    tunnelByteCount: UINT64,
    offloadByteCount: UINT64,
}}

STRUCT!{struct IPSEC_TRAFFIC_STATISTICS1 {
    encryptedByteCount: UINT64,
    authenticatedAHByteCount: UINT64,
    authenticatedESPByteCount: UINT64,
    transportByteCount: UINT64,
    tunnelByteCount: UINT64,
    offloadByteCount: UINT64,
    totalSuccessfulPackets: UINT64,
}}

STRUCT!{struct IPSEC_STATISTICS0 {
    aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0,
    inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
    outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
}}

STRUCT!{struct IPSEC_STATISTICS1 {
    aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1,
    inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
    outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
}}

pub type IPSEC_SA_SPI = UINT32;

STRUCT!{struct IPSEC_SA_AUTH_INFORMATION0 {
    authTransform: IPSEC_AUTH_TRANSFORM0,
    authKey: FWP_BYTE_BLOB,
}}

STRUCT!{struct IPSEC_SA_CIPHER_INFORMATION0 {
    cipherTransform: IPSEC_CIPHER_TRANSFORM0,
    cipherKey: FWP_BYTE_BLOB,
}}

STRUCT!{struct IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    saCipherInformation: IPSEC_SA_CIPHER_INFORMATION0,
    saAuthInformation: IPSEC_SA_AUTH_INFORMATION0,
}}

UNION2!{union IPSEC_SA0_u {
    [u32; 1] [u64; 1],
    ahInformation ahInformation_mut: *mut IPSEC_SA_AUTH_INFORMATION0,
    espAuthInformation espAuthInformation_mut: *mut IPSEC_SA_AUTH_INFORMATION0,
    espCipherInformation espCipherInformation_mut: *mut IPSEC_SA_CIPHER_INFORMATION0,
    espAuthAndCipherInformation espAuthAndCipherInformation_mut: *mut IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0,
    espAuthFwInformation espAuthFwInformation_mut: *mut IPSEC_SA_AUTH_INFORMATION0,
}}

STRUCT!{struct IPSEC_SA0 {
    spi: IPSEC_SA_SPI,
    saTransformType: IPSEC_TRANSFORM_TYPE,
	u: IPSEC_SA0_u,
}}

STRUCT!{struct IPSEC_KEYMODULE_STATE0 {
    keyModuleKey: GUID,
    stateBlob: FWP_BYTE_BLOB,
}}

pub type IPSEC_TOKEN_HANDLE = UINT64;

ENUM!{enum IPSEC_TOKEN_TYPE {
    IPSEC_TOKEN_TYPE_MACHINE = 0,
    IPSEC_TOKEN_TYPE_IMPERSONATION = IPSEC_TOKEN_TYPE_MACHINE + 1,
    IPSEC_TOKEN_TYPE_MAX = IPSEC_TOKEN_TYPE_IMPERSONATION + 1,
}}

ENUM!{enum IPSEC_TOKEN_PRINCIPAL {
    IPSEC_TOKEN_PRINCIPAL_LOCAL = 0,
    IPSEC_TOKEN_PRINCIPAL_PEER = IPSEC_TOKEN_PRINCIPAL_LOCAL + 1,
    IPSEC_TOKEN_PRINCIPAL_MAX = IPSEC_TOKEN_PRINCIPAL_PEER + 1,
}}

ENUM!{enum IPSEC_TOKEN_MODE {
    IPSEC_TOKEN_MODE_MAIN = 0,
    IPSEC_TOKEN_MODE_EXTENDED = IPSEC_TOKEN_MODE_MAIN + 1,
    IPSEC_TOKEN_MODE_MAX = IPSEC_TOKEN_MODE_EXTENDED + 1,
}}

STRUCT!{struct IPSEC_TOKEN0 {
    type_: IPSEC_TOKEN_TYPE,
    principal: IPSEC_TOKEN_PRINCIPAL,
    mode: IPSEC_TOKEN_MODE,
    token: IPSEC_TOKEN_HANDLE,
}}

STRUCT!{struct IPSEC_ID0 {
    mmTargetName: *mut wchar_t,
    emTargetName: *mut wchar_t,
    numTokens: UINT32,
    tokens: *mut IPSEC_TOKEN0,
    explicitCredentials: UINT64,
    logonId: UINT64,
}}

pub const IPSEC_SA_BUNDLE_FLAG_ND_SECURE: UINT = 0x00000001;
pub const IPSEC_SA_BUNDLE_FLAG_ND_BOUNDARY: UINT = 0x00000002;
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_NAT_BOUNDARY: UINT = 0x00000004;
pub const IPSEC_SA_BUNDLE_FLAG_GUARANTEE_ENCRYPTION: UINT = 0x00000008;
pub const IPSEC_SA_BUNDLE_FLAG_NLB: UINT = 0x00000010;
pub const IPSEC_SA_BUNDLE_FLAG_NO_MACHINE_LUID_VERIFY: UINT = 0x00000020;
pub const IPSEC_SA_BUNDLE_FLAG_NO_IMPERSONATION_LUID_VERIFY: UINT = 0x00000040;
pub const IPSEC_SA_BUNDLE_FLAG_NO_EXPLICIT_CRED_MATCH: UINT = 0x00000080;
pub const IPSEC_SA_BUNDLE_FLAG_ALLOW_NULL_TARGET_NAME_MATCH: UINT = 0x00000200;
pub const IPSEC_SA_BUNDLE_FLAG_CLEAR_DF_ON_TUNNEL: UINT = 0x00000400;
pub const IPSEC_SA_BUNDLE_FLAG_ASSUME_UDP_CONTEXT_OUTBOUND: UINT = 0x00000800;
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_BOUNDARY: UINT = 0x00001000;
pub const IPSEC_SA_BUNDLE_FLAG_SUPPRESS_DUPLICATE_DELETION: UINT = 0x00002000;
pub const IPSEC_SA_BUNDLE_FLAG_PEER_SUPPORTS_GUARANTEE_ENCRYPTION: UINT = 0x00004000;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_INBOUND_CONNECTIONS: UINT = 0x00008000;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_OUTBOUND_CONNECTIONS: UINT = 0x00010000;
pub const IPSEC_SA_BUNDLE_FLAG_FORWARD_PATH_INITIATOR: UINT = 0x00020000;
pub const IPSEC_SA_BUNDLE_FLAG_ENABLE_OPTIONAL_ASYMMETRIC_IDLE: UINT = 0x0040000;
pub const IPSEC_SA_BUNDLE_FLAG_USING_DICTATED_KEYS: UINT = 0x00080000;
pub const IPSEC_SA_BUNDLE_FLAG_LOCALLY_DICTATED_KEYS: UINT = 0x00100000;
pub const IPSEC_SA_BUNDLE_FLAG_SA_OFFLOADED: UINT = 0x00200000;
pub const IPSEC_SA_BUNDLE_FLAG_IP_IN_IP_PKT: UINT = 0x00400000;

STRUCT!{struct IPSEC_SA_BUNDLE0 {
    flags: UINT32,
    lifetime: IPSEC_SA_LIFETIME0,
    idleTimeoutSeconds: UINT32,
    ndAllowClearTimeoutSeconds: UINT32,
    ipsecId: *mut IPSEC_ID0,
    napContext: UINT32,
    qmSaId: UINT32,
    numSAs: UINT32,
    saList: *mut IPSEC_SA0,
    keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    ipVersion: FWP_IP_VERSION,
	// Single field anonymous union
    peerV4PrivateAddress: UINT32,
    mmSaId: UINT64,
    pfsGroup: IPSEC_PFS_GROUP,
}}

STRUCT!{struct IPSEC_SA_BUNDLE1 {
    flags: UINT32,
    lifetime: IPSEC_SA_LIFETIME0,
    idleTimeoutSeconds: UINT32,
    ndAllowClearTimeoutSeconds: UINT32,
    ipsecId: *mut IPSEC_ID0,
    napContext: UINT32,
    qmSaId: UINT32,
    numSAs: UINT32,
    saList: *mut IPSEC_SA0,
    keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    ipVersion: FWP_IP_VERSION,
	// Single field anonymous union
    peerV4PrivateAddress: UINT32,
    mmSaId: UINT64,
    pfsGroup: IPSEC_PFS_GROUP,
    saLookupContext: GUID,
    qmFilterId: UINT64,
}}

ENUM!{enum IPSEC_TRAFFIC_TYPE {
    IPSEC_TRAFFIC_TYPE_TRANSPORT = 0,
    IPSEC_TRAFFIC_TYPE_TUNNEL = IPSEC_TRAFFIC_TYPE_TRANSPORT + 1,
    IPSEC_TRAFFIC_TYPE_MAX = IPSEC_TRAFFIC_TYPE_TUNNEL + 1,
}}

UNION2!{union IPSEC_TRAFFIC0_u1 {
	[u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TRAFFIC0_u2 {
	[u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TRAFFIC0_u3 {
	[u64; 1],
    ipsecFilterId ipsecFilterId_mut: UINT64,
    tunnelPolicyId tunnelPolicyId_mut: UINT64,
}}

STRUCT!{struct IPSEC_TRAFFIC0 {
    ipVersion: FWP_IP_VERSION,
	u1: IPSEC_TRAFFIC0_u1,
	u2: IPSEC_TRAFFIC0_u2,
    trafficType: IPSEC_TRAFFIC_TYPE,
	u3: IPSEC_TRAFFIC0_u3,
    remotePort: UINT16,
}}

UNION2!{union IPSEC_TRAFFIC1_u1 {
	[u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TRAFFIC1_u2 {
	[u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

UNION2!{union IPSEC_TRAFFIC1_u3 {
	[u64; 1],
    ipsecFilterId ipsecFilterId_mut: UINT64,
    tunnelPolicyId tunnelPolicyId_mut: UINT64,
}}

STRUCT!{struct IPSEC_TRAFFIC1 {
    ipVersion: FWP_IP_VERSION,
	u1: IPSEC_TRAFFIC1_u1,
	u2: IPSEC_TRAFFIC1_u2,
	u3: IPSEC_TRAFFIC1_u3,
    remotePort: UINT16,
    localPort: UINT16,
    ipProtocol: UINT8,
    localIfLuid: UINT64,
    realIfProfileId: UINT32,
}}

STRUCT!{struct IPSEC_V4_UDP_ENCAPSULATION0 {
    localUdpEncapPort: UINT16,
    remoteUdpEncapPort: UINT16,
}}

STRUCT!{struct IPSEC_GETSPI0 {
    inboundIpsecTraffic: IPSEC_TRAFFIC0,
    ipVersion: FWP_IP_VERSION,
	// Single field anonymous union
    inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    rngCryptoModuleID: *mut IPSEC_CRYPTO_MODULE_ID,
}}

STRUCT!{struct IPSEC_GETSPI1 {
    inboundIpsecTraffic: IPSEC_TRAFFIC1,
    ipVersion: FWP_IP_VERSION,
	// Single field anonymous union
    inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    rngCryptoModuleID: *mut IPSEC_CRYPTO_MODULE_ID,
}}

STRUCT!{struct IPSEC_SA_DETAILS0 {
    ipVersion: FWP_IP_VERSION,
    saDirection: FWP_DIRECTION,
    traffic: IPSEC_TRAFFIC0,
    saBundle: IPSEC_SA_BUNDLE0,
	// Single field anonymous union
    udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    transportFilter: *mut FWPM_FILTER0,
}}

STRUCT!{struct IPSEC_SA_DETAILS1 {
    ipVersion: FWP_IP_VERSION,
    saDirection: FWP_DIRECTION,
    traffic: IPSEC_TRAFFIC1,
    saBundle: IPSEC_SA_BUNDLE1,
	// Single field anonymous union
    udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    transportFilter: *mut FWPM_FILTER0,
    virtualIfTunnelInfo: IPSEC_VIRTUAL_IF_TUNNEL_INFO0,
}}

STRUCT!{struct IPSEC_SA_CONTEXT0 {
    saContextId: UINT64,
    inboundSa: *mut IPSEC_SA_DETAILS0,
    outboundSa: *mut IPSEC_SA_DETAILS0,
}}

STRUCT!{struct IPSEC_SA_CONTEXT1 {
    saContextId: UINT64,
    inboundSa: *mut IPSEC_SA_DETAILS1,
    outboundSa: *mut IPSEC_SA_DETAILS1,
}}

STRUCT!{struct IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    localSubNet: FWP_CONDITION_VALUE0,
    remoteSubNet: FWP_CONDITION_VALUE0,
}}

STRUCT!{struct IPSEC_SA_ENUM_TEMPLATE0 {
    saDirection: FWP_DIRECTION,
}}

STRUCT!{struct IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    enumTemplate: *mut IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
    flags: UINT32,
    sessionKey: GUID,
}}

ENUM!{enum IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    IPSEC_SA_CONTEXT_EVENT_ADD = 1,
    IPSEC_SA_CONTEXT_EVENT_DELETE = IPSEC_SA_CONTEXT_EVENT_ADD + 1,
    IPSEC_SA_CONTEXT_EVENT_MAX = IPSEC_SA_CONTEXT_EVENT_DELETE + 1,
}}

STRUCT!{struct IPSEC_SA_CONTEXT_CHANGE0 {
    changeType: IPSEC_SA_CONTEXT_EVENT_TYPE0,
    saContextId: UINT64,
}}

ENUM!{enum IPSEC_FAILURE_POINT {
    IPSEC_FAILURE_NONE = 0,
    IPSEC_FAILURE_ME = IPSEC_FAILURE_NONE + 1,
    IPSEC_FAILURE_PEER = IPSEC_FAILURE_ME + 1,
    IPSEC_FAILURE_POINT_MAX = IPSEC_FAILURE_PEER + 1,
}}

STRUCT!{struct IPSEC_ADDRESS_INFO0 {
    numV4Addresses: UINT32,
    v4Addresses: *mut UINT32,
    numV6Addresses: UINT32,
    v6Addresses: *mut FWP_BYTE_ARRAY16,
}}

pub const IPSEC_DOSP_FLAG_ENABLE_IKEV1: UINT = 0x00000001;
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV2: UINT = 0x00000002;
pub const IPSEC_DOSP_FLAG_DISABLE_AUTHIP: UINT = 0x00000004;
pub const IPSEC_DOSP_FLAG_DISABLE_DEFAULT_BLOCK: UINT = 0x00000008;
pub const IPSEC_DOSP_FLAG_FILTER_BLOCK: UINT = 0x00000010;
pub const IPSEC_DOSP_FLAG_FILTER_EXEMPT: UINT = 0x00000020;
pub const IPSEC_DOSP_DSCP_DISABLE_VALUE: UINT = 0xff;
pub const IPSEC_DOSP_RATE_LIMIT_DISABLE_VALUE: UINT = 0;

STRUCT!{struct IPSEC_DOSP_OPTIONS0 {
    stateIdleTimeoutSeconds: UINT32,
    perIPRateLimitQueueIdleTimeoutSeconds: UINT32,
    ipV6IPsecUnauthDscp: UINT8,
    ipV6IPsecUnauthRateLimitBytesPerSec: UINT32,
    ipV6IPsecUnauthPerIPRateLimitBytesPerSec: UINT32,
    ipV6IPsecAuthDscp: UINT8,
    ipV6IPsecAuthRateLimitBytesPerSec: UINT32,
    icmpV6Dscp: UINT8,
    icmpV6RateLimitBytesPerSec: UINT32,
    ipV6FilterExemptDscp: UINT8,
    ipV6FilterExemptRateLimitBytesPerSec: UINT32,
    defBlockExemptDscp: UINT8,
    defBlockExemptRateLimitBytesPerSec: UINT32,
    maxStateEntries: UINT32,
    maxPerIPRateLimitQueues: UINT32,
    flags: UINT32,
    numPublicIFLuids: UINT32,
    publicIFLuids: *mut UINT64,
    numInternalIFLuids: UINT32,
    internalIFLuids: *mut UINT64,
    publicV6AddrMask: FWP_V6_ADDR_AND_MASK,
    internalV6AddrMask: FWP_V6_ADDR_AND_MASK,
}}

STRUCT!{struct IPSEC_DOSP_STATISTICS0 {
    totalStateEntriesCreated: UINT64,
    currentStateEntries: UINT64,
    totalInboundAllowedIPv6IPsecUnauthPkts: UINT64,
    totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts: UINT64,
    totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts: UINT64,
    totalInboundOtherDiscardedIPv6IPsecUnauthPkts: UINT64,
    totalInboundAllowedIPv6IPsecAuthPkts: UINT64,
    totalInboundRatelimitDiscardedIPv6IPsecAuthPkts: UINT64,
    totalInboundOtherDiscardedIPv6IPsecAuthPkts: UINT64,
    totalInboundAllowedICMPv6Pkts: UINT64,
    totalInboundRatelimitDiscardedICMPv6Pkts: UINT64,
    totalInboundAllowedIPv6FilterExemptPkts: UINT64,
    totalInboundRatelimitDiscardedIPv6FilterExemptPkts: UINT64,
    totalInboundDiscardedIPv6FilterBlockPkts: UINT64,
    totalInboundAllowedDefBlockExemptPkts: UINT64,
    totalInboundRatelimitDiscardedDefBlockExemptPkts: UINT64,
    totalInboundDiscardedDefBlockPkts: UINT64,
    currentInboundIPv6IPsecUnauthPerIPRateLimitQueues: UINT64,
}}

STRUCT!{struct IPSEC_DOSP_STATE0 {
    publicHostV6Addr: [UINT8; 16],
    internalHostV6Addr: [UINT8; 16],
    totalInboundIPv6IPsecAuthPackets: UINT64,
    totalOutboundIPv6IPsecAuthPackets: UINT64,
    durationSecs: UINT32,
}}

STRUCT!{struct IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    publicV6AddrMask: FWP_V6_ADDR_AND_MASK,
    internalV6AddrMask: FWP_V6_ADDR_AND_MASK,
}}

pub const IPSEC_KEY_MANAGER_FLAG_DICTATE_KEY: UINT = 0x00000001;

STRUCT!{struct IPSEC_KEY_MANAGER0 {
    keyManagerKey: GUID,
    displayData: FWPM_DISPLAY_DATA0,
    flags: UINT32,
    keyDictationTimeoutHint: UINT8,
}}
