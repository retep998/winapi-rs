// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! IKE Structures and Definitions
use ctypes::wchar_t;
use shared::basetsd::{UINT32, UINT64, UINT8};
use shared::fwptypes::{FWP_BYTE_ARRAY16, FWP_BYTE_BLOB, FWP_CONDITION_VALUE0, FWP_IP_VERSION};
use shared::guiddef::GUID;
use shared::ipsectypes::IPSEC_V4_UDP_ENCAPSULATION0;
use shared::minwindef::{BYTE, UINT, ULONG};
use shared::winerror::{ERROR_IPSEC_IKE_NEG_STATUS_BEGIN, ERROR_IPSEC_IKE_NEG_STATUS_END};
use um::winnt::{LPSTR, LPWSTR};

ENUM!{enum IKEEXT_KEY_MODULE_TYPE {
    IKEEXT_KEY_MODULE_IKE = 0,
    IKEEXT_KEY_MODULE_AUTHIP = IKEEXT_KEY_MODULE_IKE + 1,
    IKEEXT_KEY_MODULE_IKEV2 = IKEEXT_KEY_MODULE_AUTHIP + 1,
    IKEEXT_KEY_MODULE_MAX = IKEEXT_KEY_MODULE_IKEV2 + 1,
}}

ENUM!{enum IKEEXT_AUTHENTICATION_METHOD_TYPE {
    IKEEXT_PRESHARED_KEY = 0,
    IKEEXT_CERTIFICATE = IKEEXT_PRESHARED_KEY + 1,
    IKEEXT_KERBEROS = IKEEXT_CERTIFICATE + 1,
    IKEEXT_ANONYMOUS = IKEEXT_KERBEROS + 1,
    IKEEXT_SSL = IKEEXT_ANONYMOUS + 1,
    IKEEXT_NTLM_V2 = IKEEXT_SSL + 1,
    IKEEXT_IPV6_CGA = IKEEXT_NTLM_V2 + 1,
    IKEEXT_CERTIFICATE_ECDSA_P256 = IKEEXT_IPV6_CGA + 1,
    IKEEXT_CERTIFICATE_ECDSA_P384 = IKEEXT_CERTIFICATE_ECDSA_P256 + 1,
    IKEEXT_SSL_ECDSA_P256 = IKEEXT_CERTIFICATE_ECDSA_P384 + 1,
    IKEEXT_SSL_ECDSA_P384 = IKEEXT_SSL_ECDSA_P256 + 1,
    IKEEXT_EAP = IKEEXT_SSL_ECDSA_P384 + 1,
    IKEEXT_RESERVED = IKEEXT_EAP + 1,
    IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX = IKEEXT_RESERVED + 1,
}}

ENUM!{enum IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    IKEEXT_IMPERSONATION_NONE = 0,
    IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL = IKEEXT_IMPERSONATION_NONE + 1,
    IKEEXT_IMPERSONATION_MAX = IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL + 1,
}}

STRUCT!{struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    presharedKey: FWP_BYTE_BLOB,
}}

pub const IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY: UINT = 0x00000001;
pub const IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY: UINT = 0x00000002;

STRUCT!{struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    presharedKey: FWP_BYTE_BLOB,
    flags: UINT32,
}}

pub const IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING: UINT = 0x00000001;
pub const IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD: UINT = 0x00000002;
pub const IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE: UINT = 0x00000004;
pub const IKEEXT_CERT_FLAG_INTERMEDIATE_CA: UINT = 0x00000008;
pub const IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE: UINT = 0x00000010;
pub const IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND: UINT = 0x00000020;
pub const IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE: UINT = 0x00000040;
pub const IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE: UINT = 0x00000080;
pub const IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE: UINT = 0x00000100;

STRUCT!{struct IKEEXT_CERT_ROOT_CONFIG0 {
    certData: FWP_BYTE_BLOB,
    flags: UINT32,
}}

pub const IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY: UINT = 0x00000001;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK: UINT = 0x00000002;
pub const IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG: UINT = 0x00000004;
pub const IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION: UINT = 0x00000008;
pub const IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP: UINT = 0x00000010;
pub const IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE: UINT = 0x00000020;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD: UINT = 0x00000040;

ENUM!{enum IKEEXT_CERT_CONFIG_TYPE {
    IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST = 0,
    IKEEXT_CERT_CONFIG_ENTERPRISE_STORE = IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST + 1,
    IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE = IKEEXT_CERT_CONFIG_ENTERPRISE_STORE + 1,
    IKEEXT_CERT_CONFIG_UNSPECIFIED = IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE + 1,
    IKEEXT_CERT_CONFIG_TYPE_MAX = IKEEXT_CERT_CONFIG_UNSPECIFIED + 1,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION0_u1_s {
    inboundRootArraySize: UINT32,
    inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION0_u1 {
    [u32; 2] [u64; 2],
    s s_mut: IKEEXT_CERTIFICATE_AUTHENTICATION0_u1_s,
    inboundEnterpriseStoreConfig inboundEnterpriseStoreConfig_mut: *mut IKEEXT_CERT_ROOT_CONFIG0,
    inboundTrustedRootStoreConfig inboundTrustedRootStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION0_u2_s {
    outboundRootArraySize: UINT32,
    outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION0_u2 {
    [u32; 2] [u64; 2],
    s s_mut: IKEEXT_CERTIFICATE_AUTHENTICATION0_u2_s,
    outboundEnterpriseStoreConfig outboundEnterpriseStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
    outboundTrustedRootStoreConfig outboundTrustedRootStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u1: IKEEXT_CERTIFICATE_AUTHENTICATION0_u1,
    outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u2: IKEEXT_CERTIFICATE_AUTHENTICATION0_u2,    
    flags: UINT32,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION1_u1_s {
    inboundRootArraySize: UINT32,
    inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION1_u1 {
    [u32; 2] [u64; 2],
    s s_mut: IKEEXT_CERTIFICATE_AUTHENTICATION1_u1_s,
    inboundEnterpriseStoreConfig inboundEnterpriseStoreConfig_mut: *mut IKEEXT_CERT_ROOT_CONFIG0,
    inboundTrustedRootStoreConfig inboundTrustedRootStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION1_u2_s {
    outboundRootArraySize: UINT32,
    outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION1_u2 {
    [u32; 2] [u64; 2],
    s s_mut: IKEEXT_CERTIFICATE_AUTHENTICATION1_u2_s,
    outboundEnterpriseStoreConfig outboundEnterpriseStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
    outboundTrustedRootStoreConfig outboundTrustedRootStoreConfig_mut:
        *mut IKEEXT_CERT_ROOT_CONFIG0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u1: IKEEXT_CERTIFICATE_AUTHENTICATION1_u1,
    outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u2: IKEEXT_CERTIFICATE_AUTHENTICATION1_u2,
    flags: UINT32,
    localCertLocationUrl: FWP_BYTE_BLOB,
}}

ENUM!{enum IKEEXT_CERT_CRITERIA_NAME_TYPE {
    IKEEXT_CERT_CRITERIA_DNS = 0,
    IKEEXT_CERT_CRITERIA_UPN = IKEEXT_CERT_CRITERIA_DNS + 1,
    IKEEXT_CERT_CRITERIA_RFC822 = IKEEXT_CERT_CRITERIA_UPN + 1,
    IKEEXT_CERT_CRITERIA_CN = IKEEXT_CERT_CRITERIA_RFC822 + 1,
    IKEEXT_CERT_CRITERIA_OU = IKEEXT_CERT_CRITERIA_CN + 1,
    IKEEXT_CERT_CRITERIA_O = IKEEXT_CERT_CRITERIA_OU + 1,
    IKEEXT_CERT_CRITERIA_DC = IKEEXT_CERT_CRITERIA_O + 1,
    IKEEXT_CERT_CRITERIA_NAME_TYPE_MAX = IKEEXT_CERT_CRITERIA_DC + 1,
}}

STRUCT!{struct IKEEXT_CERT_EKUS0 {
    numEku: ULONG,
    eku: *mut LPSTR,
}}

STRUCT!{struct IKEEXT_CERT_NAME0 {
    nameType: IKEEXT_CERT_CRITERIA_NAME_TYPE,
    certName: LPWSTR,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_CRITERIA0 {
    certData: FWP_BYTE_BLOB,
    certHash: FWP_BYTE_BLOB,
    eku: *mut IKEEXT_CERT_EKUS0,
    name: *mut IKEEXT_CERT_NAME0,
    flags: UINT32,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s1 {
    inboundRootArraySize: UINT32,
    inboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s2 {
    inboundEnterpriseStoreArraySize: UINT32,
    inboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s3 {
    inboundRootStoreArraySize: UINT32,
    inboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION2_u1 {
    [u32; 2] [u64; 2],
    s1 s1_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s1,
    s2 s2_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s2,
    s3 s3_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u1_s3,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s1 {
    outboundRootArraySize: UINT32,
    outboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s2 {
    outboundEnterpriseStoreArraySize: UINT32,
    outboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s3 {
    outboundRootStoreArraySize: UINT32,
    outboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}}

UNION!{union IKEEXT_CERTIFICATE_AUTHENTICATION2_u2 {
    [u32; 2] [u64; 2],
    s1 s1_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s1,
    s2 s2_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s2,
    s3 s3_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2_u2_s3,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u1: IKEEXT_CERTIFICATE_AUTHENTICATION2_u1,
    outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    u2: IKEEXT_CERTIFICATE_AUTHENTICATION2_u2,
    flags: UINT32,
    localCertLocationUrl: FWP_BYTE_BLOB,
}}

STRUCT!{struct IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    keyContainerName: *mut wchar_t,
    cspName: *mut wchar_t,
    cspType: UINT32,
    cgaModifier: FWP_BYTE_ARRAY16,
    cgaCollisionCount: BYTE,
}}

pub const IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: UINT = 0x00000001;
pub const IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: UINT = 0x00000002;

STRUCT!{struct IKEEXT_KERBEROS_AUTHENTICATION0 {
    flags: UINT32,
}}

pub const IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR: UINT = 0x00000004;

STRUCT!{struct IKEEXT_KERBEROS_AUTHENTICATION1 {
    flags: UINT32,
    proxyServer: *mut wchar_t,
}}

pub const IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: UINT = 0x00000001;

STRUCT!{struct IKEEXT_RESERVED_AUTHENTICATION0 {
    flags: UINT32,
}}

pub const IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: UINT = 0x00000001;

STRUCT!{struct IKEEXT_NTLM_V2_AUTHENTICATION0 {
    flags: UINT32,
}}

pub const IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY: UINT = 0x00000001;
pub const IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY: UINT = 0x00000002;

STRUCT!{struct IKEEXT_EAP_AUTHENTICATION0 {
    flags: UINT32,
}}

UNION!{union IKEEXT_AUTHENTICATION_METHOD0_u {
    [u32; 8] [u64; 7],
    presharedKeyAuthentication presharedKeyAuthentication_mut:
        IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    certificateAuthentication certificateAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    kerberosAuthentication kerberosAuthentication_mut: IKEEXT_KERBEROS_AUTHENTICATION0,
    ntlmV2Authentication ntlmV2Authentication_mut: IKEEXT_NTLM_V2_AUTHENTICATION0,
    sslAuthentication sslAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    cgaAuthentication cgaAuthentication_mut: IKEEXT_IPV6_CGA_AUTHENTICATION0,
}}

STRUCT!{struct IKEEXT_AUTHENTICATION_METHOD0 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    u: IKEEXT_AUTHENTICATION_METHOD0_u,
}}

UNION!{union IKEEXT_AUTHENTICATION_METHOD1_u {
    [u32; 9] [u64; 9],
    presharedKeyAuthentication presharedKeyAuthentication_mut:
        IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    certificateAuthentication certificateAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    kerberosAuthentication kerberosAuthentication_mut: IKEEXT_KERBEROS_AUTHENTICATION0,
    ntlmV2Authentication ntlmV2Authentication_mut: IKEEXT_NTLM_V2_AUTHENTICATION0,
    sslAuthentication sslAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    cgaAuthentication cgaAuthentication_mut: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    eapAuthentication eapAuthentication_mut: IKEEXT_EAP_AUTHENTICATION0,
}}

STRUCT!{struct IKEEXT_AUTHENTICATION_METHOD1 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    u: IKEEXT_AUTHENTICATION_METHOD1_u,
}}

UNION!{union IKEEXT_AUTHENTICATION_METHOD2_u {
    [u32; 9] [u64; 9],
    presharedKeyAuthentication presharedKeyAuthentication_mut:
        IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    certificateAuthentication certificateAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    kerberosAuthentication kerberosAuthentication_mut: IKEEXT_KERBEROS_AUTHENTICATION1,
    reservedAuthentication reservedAuthentication_mut: IKEEXT_RESERVED_AUTHENTICATION0,
    ntlmV2Authentication ntlmV2Authentication_mut: IKEEXT_NTLM_V2_AUTHENTICATION0,
    sslAuthentication sslAuthentication_mut: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    cgaAuthentication cgaAuthentication_mut: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    eapAuthentication eapAuthentication_mut: IKEEXT_EAP_AUTHENTICATION0,
}}

STRUCT!{struct IKEEXT_AUTHENTICATION_METHOD2 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    u: IKEEXT_AUTHENTICATION_METHOD2_u,
}}

ENUM!{enum IKEEXT_CIPHER_TYPE {
    IKEEXT_CIPHER_DES = 0,
    IKEEXT_CIPHER_3DES = IKEEXT_CIPHER_DES + 1,
    IKEEXT_CIPHER_AES_128 = IKEEXT_CIPHER_3DES + 1,
    IKEEXT_CIPHER_AES_192 = IKEEXT_CIPHER_AES_128 + 1,
    IKEEXT_CIPHER_AES_256 = IKEEXT_CIPHER_AES_192 + 1,
    IKEEXT_CIPHER_AES_GCM_128_16ICV = IKEEXT_CIPHER_AES_256 + 1,
    IKEEXT_CIPHER_AES_GCM_256_16ICV = IKEEXT_CIPHER_AES_GCM_128_16ICV + 1,
    IKEEXT_CIPHER_TYPE_MAX = IKEEXT_CIPHER_AES_GCM_256_16ICV + 1,
}}

STRUCT!{struct IKEEXT_CIPHER_ALGORITHM0 {
    algoIdentifier: IKEEXT_CIPHER_TYPE,
    keyLen: UINT32,
    rounds: UINT32,
}}

ENUM!{enum IKEEXT_INTEGRITY_TYPE {
    IKEEXT_INTEGRITY_MD5 = 0,
    IKEEXT_INTEGRITY_SHA1 = IKEEXT_INTEGRITY_MD5 + 1,
    IKEEXT_INTEGRITY_SHA_256 = IKEEXT_INTEGRITY_SHA1 + 1,
    IKEEXT_INTEGRITY_SHA_384 = IKEEXT_INTEGRITY_SHA_256 + 1,
    IKEEXT_INTEGRITY_TYPE_MAX = IKEEXT_INTEGRITY_SHA_384 + 1,
}}

STRUCT!{struct IKEEXT_INTEGRITY_ALGORITHM0 {
    algoIdentifier: IKEEXT_INTEGRITY_TYPE,
}}

ENUM!{enum IKEEXT_DH_GROUP {
    IKEEXT_DH_GROUP_NONE = 0,
    IKEEXT_DH_GROUP_1 = IKEEXT_DH_GROUP_NONE + 1,
    IKEEXT_DH_GROUP_2 = IKEEXT_DH_GROUP_1 + 1,
    IKEEXT_DH_GROUP_14 = IKEEXT_DH_GROUP_2 + 1,
    IKEEXT_DH_GROUP_2048 = IKEEXT_DH_GROUP_14,
    IKEEXT_DH_ECP_256 = IKEEXT_DH_GROUP_2048 + 1,
    IKEEXT_DH_ECP_384 = IKEEXT_DH_ECP_256 + 1,
    IKEEXT_DH_GROUP_24 = IKEEXT_DH_ECP_384 + 1,
    IKEEXT_DH_GROUP_MAX = IKEEXT_DH_GROUP_24 + 1,
}}

STRUCT!{struct IKEEXT_PROPOSAL0 {
    cipherAlgorithm: IKEEXT_CIPHER_ALGORITHM0,
    integrityAlgorithm: IKEEXT_INTEGRITY_ALGORITHM0,
    maxLifetimeSeconds: UINT32,
    dhGroup: IKEEXT_DH_GROUP,
    quickModeLimit: UINT32,
}}

pub const IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS: UINT = 0x00000001;
pub const IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY: UINT = 0x00000002;
pub const IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY: UINT = 0x00000004;
pub const IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH: UINT = 0x00000008;
pub const IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED: UINT = 0x00000010;
pub const IKEEXT_POLICY_FLAG_SITE_TO_SITE: UINT = 0x00000020;

STRUCT!{struct IKEEXT_POLICY0 {
    softExpirationTime: UINT32,
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    numIkeProposals: UINT32,
    ikeProposals: *mut IKEEXT_PROPOSAL0,
    flags: UINT32,
    maxDynamicFilters: UINT32,
}}

STRUCT!{struct IKEEXT_POLICY1 {
    softExpirationTime: UINT32,
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    numIkeProposals: UINT32,
    ikeProposals: *mut IKEEXT_PROPOSAL0,
    flags: UINT32,
    maxDynamicFilters: UINT32,
    retransmitDurationSecs: UINT32,
}}

STRUCT!{struct IKEEXT_POLICY2 {
    softExpirationTime: UINT32,
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    numIkeProposals: UINT32,
    ikeProposals: *mut IKEEXT_PROPOSAL0,
    flags: UINT32,
    maxDynamicFilters: UINT32,
    retransmitDurationSecs: UINT32,
}}

STRUCT!{struct IKEEXT_EM_POLICY0 {
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}}

STRUCT!{struct IKEEXT_EM_POLICY1 {
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}}

STRUCT!{struct IKEEXT_EM_POLICY2 {
    numAuthenticationMethods: UINT32,
    authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}}

pub const IKEEXT_ERROR_CODE_COUNT: UINT = ERROR_IPSEC_IKE_NEG_STATUS_END
    - ERROR_IPSEC_IKE_NEG_STATUS_BEGIN;

STRUCT!{struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    currentActiveMainModes: UINT32,
    totalMainModesStarted: UINT32,
    totalSuccessfulMainModes: UINT32,
    totalFailedMainModes: UINT32,
    totalResponderMainModes: UINT32,
    currentNewResponderMainModes: UINT32,
    currentActiveQuickModes: UINT32,
    totalQuickModesStarted: UINT32,
    totalSuccessfulQuickModes: UINT32,
    totalFailedQuickModes: UINT32,
    totalAcquires: UINT32,
    totalReinitAcquires: UINT32,
    currentActiveExtendedModes: UINT32,
    totalExtendedModesStarted: UINT32,
    totalSuccessfulExtendedModes: UINT32,
    totalFailedExtendedModes: UINT32,
    totalImpersonationExtendedModes: UINT32,
    totalImpersonationMainModes: UINT32,
}}

STRUCT!{struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    currentActiveMainModes: UINT32,
    totalMainModesStarted: UINT32,
    totalSuccessfulMainModes: UINT32,
    totalFailedMainModes: UINT32,
    totalResponderMainModes: UINT32,
    currentNewResponderMainModes: UINT32,
    currentActiveQuickModes: UINT32,
    totalQuickModesStarted: UINT32,
    totalSuccessfulQuickModes: UINT32,
    totalFailedQuickModes: UINT32,
    totalAcquires: UINT32,
    totalReinitAcquires: UINT32,
    currentActiveExtendedModes: UINT32,
    totalExtendedModesStarted: UINT32,
    totalSuccessfulExtendedModes: UINT32,
    totalFailedExtendedModes: UINT32,
    totalImpersonationExtendedModes: UINT32,
    totalImpersonationMainModes: UINT32,
}}

STRUCT!{struct IKEEXT_KEYMODULE_STATISTICS0 {
    v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    errorFrequencyTable: [UINT32; 97],
    mainModeNegotiationTime: UINT32,
    quickModeNegotiationTime: UINT32,
    extendedModeNegotiationTime: UINT32,
}}

STRUCT!{struct IKEEXT_KEYMODULE_STATISTICS1 {
    v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    errorFrequencyTable: [UINT32; 97],
    mainModeNegotiationTime: UINT32,
    quickModeNegotiationTime: UINT32,
    extendedModeNegotiationTime: UINT32,
}}

STRUCT!{struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    totalSocketReceiveFailures: UINT32,
    totalSocketSendFailures: UINT32,
}}

STRUCT!{struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    totalSocketReceiveFailures: UINT32,
    totalSocketSendFailures: UINT32,
}}

STRUCT!{struct IKEEXT_COMMON_STATISTICS0 {
    v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    totalPacketsReceived: UINT32,
    totalInvalidPacketsReceived: UINT32,
    currentQueuedWorkitems: UINT32,
}}

STRUCT!{struct IKEEXT_COMMON_STATISTICS1 {
    v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    totalPacketsReceived: UINT32,
    totalInvalidPacketsReceived: UINT32,
    currentQueuedWorkitems: UINT32,
}}

STRUCT!{struct IKEEXT_STATISTICS0 {
    ikeStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    authipStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    commonStatistics: IKEEXT_COMMON_STATISTICS0,
}}

STRUCT!{struct IKEEXT_STATISTICS1 {
    ikeStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    authipStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    ikeV2Statistics: IKEEXT_KEYMODULE_STATISTICS1,
    commonStatistics: IKEEXT_COMMON_STATISTICS1,
}}


UNION!{union IKEEXT_TRAFFIC0_u1 {
    [u32; 4],
    localV4Address localV4Address_mut: UINT32,
    localV6Address localV6Address_mut: [UINT8; 16],
}}

UNION!{union IKEEXT_TRAFFIC0_u2 {
    [u32; 4],
    remoteV4Address remoteV4Address_mut: UINT32,
    remoteV6Address remoteV6Address_mut: [UINT8; 16],
}}

STRUCT!{struct IKEEXT_TRAFFIC0 {
    ipVersion: FWP_IP_VERSION,
    u1: IKEEXT_TRAFFIC0_u1,
    u2: IKEEXT_TRAFFIC0_u2,
    authIpFilterId: UINT64,
}}

pub type IKEEXT_COOKIE = UINT64;

STRUCT!{struct IKEEXT_COOKIE_PAIR0 {
    initiator: IKEEXT_COOKIE,
    responder: IKEEXT_COOKIE,
}}

pub const IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT: UINT = 0x00000001;

STRUCT!{struct IKEEXT_CERTIFICATE_CREDENTIAL0 {
    subjectName: FWP_BYTE_BLOB,
    certHash: FWP_BYTE_BLOB,
    flags: UINT32,
}}

STRUCT!{struct IKEEXT_NAME_CREDENTIAL0 {
    principalName: *mut wchar_t,
}}

UNION!{union IKEEXT_CREDENTIAL0_u {
    [u32; 1] [u64; 1],
    presharedKey presharedKey_mut: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    certificate certificate_mut: *mut IKEEXT_CERTIFICATE_CREDENTIAL0,
    name name_mut: *mut IKEEXT_NAME_CREDENTIAL0,
}}

STRUCT!{struct IKEEXT_CREDENTIAL0 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    u: IKEEXT_CREDENTIAL0_u,
}}

STRUCT!{struct IKEEXT_CREDENTIAL_PAIR0 {
    localCredentials: IKEEXT_CREDENTIAL0,
    peerCredentials: IKEEXT_CREDENTIAL0,
}}

STRUCT!{struct IKEEXT_CREDENTIALS0 {
    numCredentials: UINT32,
    credentials: *mut IKEEXT_CREDENTIAL_PAIR0,
}}

STRUCT!{struct IKEEXT_SA_DETAILS0 {
    saId: UINT64,
    keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    ipVersion: FWP_IP_VERSION,
    // Single field anonymous union
    v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    ikeTraffic: IKEEXT_TRAFFIC0,
    ikeProposal: IKEEXT_PROPOSAL0,
    cookiePair: IKEEXT_COOKIE_PAIR0,
    ikeCredentials: IKEEXT_CREDENTIALS0,
    ikePolicyKey: GUID,
    virtualIfTunnelId: UINT64,
}}

STRUCT!{struct IKEEXT_CERTIFICATE_CREDENTIAL1 {
    subjectName: FWP_BYTE_BLOB,
    certHash: FWP_BYTE_BLOB,
    flags: UINT32,
    certificate: FWP_BYTE_BLOB,
}}

UNION!{union IKEEXT_CREDENTIAL1_u {
    [u32; 1] [u64; 1],
    presharedKey presharedKey_mut: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    certificate certificate_mut: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    name name_mut: *mut IKEEXT_NAME_CREDENTIAL0,
}}

STRUCT!{struct IKEEXT_CREDENTIAL1 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    u: IKEEXT_CREDENTIAL1_u,
}}

STRUCT!{struct IKEEXT_CREDENTIAL_PAIR1 {
    localCredentials: IKEEXT_CREDENTIAL1,
    peerCredentials: IKEEXT_CREDENTIAL1,
}}

STRUCT!{struct IKEEXT_CREDENTIALS1 {
    numCredentials: UINT32,
    credentials: *mut IKEEXT_CREDENTIAL_PAIR1,
}}

STRUCT!{struct IKEEXT_SA_DETAILS1 {
    saId: UINT64,
    keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    ipVersion: FWP_IP_VERSION,
    // Single field anonymous union
    v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    ikeTraffic: IKEEXT_TRAFFIC0,
    ikeProposal: IKEEXT_PROPOSAL0,
    cookiePair: IKEEXT_COOKIE_PAIR0,
    ikeCredentials: IKEEXT_CREDENTIALS1,
    ikePolicyKey: GUID,
    virtualIfTunnelId: UINT64,
    correlationKey: FWP_BYTE_BLOB,
}}

UNION!{union IKEEXT_CREDENTIAL2_u {
    [u32; 1] [u64; 1],
    presharedKey presharedKey_mut: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    certificate certificate_mut: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    name name_mut: *mut IKEEXT_NAME_CREDENTIAL0,
}}

STRUCT!{struct IKEEXT_CREDENTIAL2 {
    authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    u: IKEEXT_CREDENTIAL2_u,
}}

STRUCT!{struct IKEEXT_CREDENTIAL_PAIR2 {
    localCredentials: IKEEXT_CREDENTIAL2,
    peerCredentials: IKEEXT_CREDENTIAL2,
}}

STRUCT!{struct IKEEXT_CREDENTIALS2 {
    numCredentials: UINT32,
    credentials: *mut IKEEXT_CREDENTIAL_PAIR2,
}}

STRUCT!{struct IKEEXT_SA_DETAILS2 {
    saId: UINT64,
    keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    ipVersion: FWP_IP_VERSION,
    // Single field anonymous union
    v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
    ikeTraffic: IKEEXT_TRAFFIC0,
    ikeProposal: IKEEXT_PROPOSAL0,
    cookiePair: IKEEXT_COOKIE_PAIR0,
    ikeCredentials: IKEEXT_CREDENTIALS2,
    ikePolicyKey: GUID,
    virtualIfTunnelId: UINT64,
    correlationKey: FWP_BYTE_BLOB,
}}

STRUCT!{struct IKEEXT_SA_ENUM_TEMPLATE0 {
    localSubNet: FWP_CONDITION_VALUE0,
    remoteSubNet:FWP_CONDITION_VALUE0,
    localMainModeCertHash: FWP_BYTE_BLOB,
}}

ENUM!{enum IKEEXT_MM_SA_STATE {
    IKEEXT_MM_SA_STATE_NONE = 0,
    IKEEXT_MM_SA_STATE_SA_SENT = IKEEXT_MM_SA_STATE_NONE + 1,
    IKEEXT_MM_SA_STATE_SSPI_SENT = IKEEXT_MM_SA_STATE_SA_SENT + 1,
    IKEEXT_MM_SA_STATE_FINAL = IKEEXT_MM_SA_STATE_SSPI_SENT + 1,
    IKEEXT_MM_SA_STATE_FINAL_SENT = IKEEXT_MM_SA_STATE_FINAL + 1,
    IKEEXT_MM_SA_STATE_COMPLETE = IKEEXT_MM_SA_STATE_FINAL_SENT + 1,
    IKEEXT_MM_SA_STATE_MAX = IKEEXT_MM_SA_STATE_COMPLETE + 1,
}}

ENUM!{enum IKEEXT_QM_SA_STATE {
    IKEEXT_QM_SA_STATE_NONE = 0,
    IKEEXT_QM_SA_STATE_INITIAL = IKEEXT_QM_SA_STATE_NONE + 1,
    IKEEXT_QM_SA_STATE_FINAL = IKEEXT_QM_SA_STATE_INITIAL + 1,
    IKEEXT_QM_SA_STATE_COMPLETE = IKEEXT_QM_SA_STATE_FINAL + 1,
    IKEEXT_QM_SA_STATE_MAX = IKEEXT_QM_SA_STATE_COMPLETE + 1,
}}

ENUM!{enum IKEEXT_EM_SA_STATE {
    IKEEXT_EM_SA_STATE_NONE = 0,
    IKEEXT_EM_SA_STATE_SENT_ATTS = IKEEXT_EM_SA_STATE_NONE + 1,
    IKEEXT_EM_SA_STATE_SSPI_SENT = IKEEXT_EM_SA_STATE_SENT_ATTS + 1,
    IKEEXT_EM_SA_STATE_AUTH_COMPLETE = IKEEXT_EM_SA_STATE_SSPI_SENT + 1,
    IKEEXT_EM_SA_STATE_FINAL = IKEEXT_EM_SA_STATE_AUTH_COMPLETE + 1,
    IKEEXT_EM_SA_STATE_COMPLETE = IKEEXT_EM_SA_STATE_FINAL + 1,
    IKEEXT_EM_SA_STATE_MAX = IKEEXT_EM_SA_STATE_COMPLETE + 1,
}}

ENUM!{enum IKEEXT_SA_ROLE {
    IKEEXT_SA_ROLE_INITIATOR = 0,
    IKEEXT_SA_ROLE_RESPONDER = IKEEXT_SA_ROLE_INITIATOR + 1,
    IKEEXT_SA_ROLE_MAX = IKEEXT_SA_ROLE_RESPONDER + 1,
}}
