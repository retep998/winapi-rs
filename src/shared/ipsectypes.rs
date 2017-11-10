// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::wchar_t;
use shared::basetsd::{UINT8, UINT16, UINT32, UINT64};
use shared::fwptypes::{FWP_IP_VERSION};
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












