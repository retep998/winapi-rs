// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Security Support Provider Interface Prototypes and structure definitions
pub type SEC_WCHAR = ::WCHAR;
pub type SEC_CHAR = ::CHAR;
pub type SECURITY_STATUS = ::LONG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecHandle {
    pub dwLower: ::ULONG_PTR,
    pub dwUpper: ::ULONG_PTR,
}
pub type PSecHandle = *mut SecHandle;
pub const SEC_DELETED_HANDLE: ::ULONG_PTR = 2;
pub type CredHandle = SecHandle;
pub type PCredHandle = PSecHandle;
pub type CtxtHandle = SecHandle;
pub type PCtxtHandle = PSecHandle;
pub type SECURITY_INTEGER = ::LARGE_INTEGER;
pub type PSECURITY_INTEGER = *mut ::LARGE_INTEGER;
pub type TimeStamp = SECURITY_INTEGER;
pub type PTimeStamp = *mut SECURITY_INTEGER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SECURITY_STRING {
    pub Length: ::c_ushort,
    pub MaximumLength: ::c_ushort,
    pub Buffer: *mut ::c_ushort,
}
pub type PSECURITY_STRING = *mut SECURITY_STRING;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgInfoW {
    pub fCapabilities: ::c_ulong,
    pub wVersion: ::c_ushort,
    pub wRPCID: ::c_ushort,
    pub cbMaxToken: ::c_ulong,
    pub Name: *mut SEC_WCHAR,
    pub Comment: *mut SEC_WCHAR,
}
pub type PSecPkgInfoW = *mut SecPkgInfoW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgInfoA {
    pub fCapabilities: ::c_ulong,
    pub wVersion: ::c_ushort,
    pub wRPCID: ::c_ushort,
    pub cbMaxToken: ::c_ulong,
    pub Name: *mut SEC_CHAR,
    pub Comment: *mut SEC_CHAR,
}
pub type PSecPkgInfoA = *mut SecPkgInfoA;
pub const SECPKG_FLAG_INTEGRITY: ::c_ulong = 0x00000001;
pub const SECPKG_FLAG_PRIVACY: ::c_ulong = 0x00000002;
pub const SECPKG_FLAG_TOKEN_ONLY: ::c_ulong = 0x00000004;
pub const SECPKG_FLAG_DATAGRAM: ::c_ulong = 0x00000008;
pub const SECPKG_FLAG_CONNECTION: ::c_ulong = 0x00000010;
pub const SECPKG_FLAG_MULTI_REQUIRED: ::c_ulong = 0x00000020;
pub const SECPKG_FLAG_CLIENT_ONLY: ::c_ulong = 0x00000040;
pub const SECPKG_FLAG_EXTENDED_ERROR: ::c_ulong = 0x00000080;
pub const SECPKG_FLAG_IMPERSONATION: ::c_ulong = 0x00000100;
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: ::c_ulong = 0x00000200;
pub const SECPKG_FLAG_STREAM: ::c_ulong = 0x00000400;
pub const SECPKG_FLAG_NEGOTIABLE: ::c_ulong = 0x00000800;
pub const SECPKG_FLAG_GSS_COMPATIBLE: ::c_ulong = 0x00001000;
pub const SECPKG_FLAG_LOGON: ::c_ulong = 0x00002000;
pub const SECPKG_FLAG_ASCII_BUFFERS: ::c_ulong = 0x00004000;
pub const SECPKG_FLAG_FRAGMENT: ::c_ulong = 0x00008000;
pub const SECPKG_FLAG_MUTUAL_AUTH: ::c_ulong = 0x00010000;
pub const SECPKG_FLAG_DELEGATION: ::c_ulong = 0x00020000;
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: ::c_ulong = 0x00040000;
pub const SECPKG_FLAG_RESTRICTED_TOKENS: ::c_ulong = 0x00080000;
pub const SECPKG_FLAG_NEGO_EXTENDER: ::c_ulong = 0x00100000;
pub const SECPKG_FLAG_NEGOTIABLE2: ::c_ulong = 0x00200000;
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: ::c_ulong = 0x00400000;
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: ::c_ulong = 0x00800000;
pub const SECPKG_ID_NONE: ::c_ulong = 0xFFFF;
pub const SECPKG_CALLFLAGS_APPCONTAINER: ::c_ulong = 0x00000001;
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: ::c_ulong = 0x00000002;
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: ::c_ulong = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecBuffer {
    pub cbBuffer: ::c_ulong,
    pub BufferType: ::c_ulong,
    pub pvBuffer: *mut ::c_void,
}
pub type PSecBuffer = *mut SecBuffer;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecBufferDesc {
    pub ulVersion: ::c_ulong,
    pub cBuffers: ::c_ulong,
    pub pBuffers: PSecBuffer,
}
pub type PSecBufferDesc = *mut SecBufferDesc;
pub const SECBUFFER_VERSION: ::c_ulong = 0;
pub const SECBUFFER_EMPTY: ::c_ulong = 0;
pub const SECBUFFER_DATA: ::c_ulong = 1;
pub const SECBUFFER_TOKEN: ::c_ulong = 2;
pub const SECBUFFER_PKG_PARAMS: ::c_ulong = 3;
pub const SECBUFFER_MISSING: ::c_ulong = 4;
pub const SECBUFFER_EXTRA: ::c_ulong = 5;
pub const SECBUFFER_STREAM_TRAILER: ::c_ulong = 6;
pub const SECBUFFER_STREAM_HEADER: ::c_ulong = 7;
pub const SECBUFFER_NEGOTIATION_INFO: ::c_ulong = 8;
pub const SECBUFFER_PADDING: ::c_ulong = 9;
pub const SECBUFFER_STREAM: ::c_ulong = 10;
pub const SECBUFFER_MECHLIST: ::c_ulong = 11;
pub const SECBUFFER_MECHLIST_SIGNATURE: ::c_ulong = 12;
pub const SECBUFFER_TARGET: ::c_ulong = 13;
pub const SECBUFFER_CHANNEL_BINDINGS: ::c_ulong = 14;
pub const SECBUFFER_CHANGE_PASS_RESPONSE: ::c_ulong = 15;
pub const SECBUFFER_TARGET_HOST: ::c_ulong = 16;
pub const SECBUFFER_ALERT: ::c_ulong = 17;
pub const SECBUFFER_APPLICATION_PROTOCOLS: ::c_ulong = 18;
pub const SECBUFFER_ATTRMASK: ::c_ulong = 0xF0000000;
pub const SECBUFFER_READONLY: ::c_ulong = 0x80000000;
pub const SECBUFFER_READONLY_WITH_CHECKSUM: ::c_ulong = 0x10000000;
pub const SECBUFFER_RESERVED: ::c_ulong = 0x60000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_NEGOTIATION_INFO {
    pub Size: ::c_ulong,
    pub NameLength: ::c_ulong,
    pub Name: *mut SEC_WCHAR,
    pub Reserved: *mut ::c_void,
}
pub type PSEC_NEGOTIATION_INFO = *mut SEC_NEGOTIATION_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_CHANNEL_BINDINGS {
    pub dwInitiatorAddrType: ::c_ulong,
    pub cbInitiatorLength: ::c_ulong,
    pub dwInitiatorOffset: ::c_ulong,
    pub dwAcceptorAddrType: ::c_ulong,
    pub cbAcceptorLength: ::c_ulong,
    pub dwAcceptorOffset: ::c_ulong,
    pub cbApplicationDataLength: ::c_ulong,
    pub dwApplicationDataOffset: ::c_ulong,
}
pub type PSEC_CHANNEL_BINDINGS = *mut SEC_CHANNEL_BINDINGS;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    SecApplicationProtocolNegotiationExt_None,
    SecApplicationProtocolNegotiationExt_NPN,
    SecApplicationProtocolNegotiationExt_ALPN,
}
pub use self::SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT::*;
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_APPLICATION_PROTOCOL_LIST {
    pub ProtoNegoExt: ::SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolListSize: ::c_ushort,
    pub ProtocolList: [::c_uchar; 0],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_APPLICATION_PROTOCOLS {
    pub ProtocolListsSize: ::c_ulong,
    pub ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; 0],
}
pub type PSEC_APPLICATION_PROTOCOLS = *mut SEC_APPLICATION_PROTOCOLS;
pub const SECURITY_NATIVE_DREP: ::c_ulong = 0x00000010;
pub const SECURITY_NETWORK_DREP: ::c_ulong = 0x00000000;
pub const SECPKG_CRED_INBOUND: ::c_ulong = 0x00000001;
pub const SECPKG_CRED_OUTBOUND: ::c_ulong = 0x00000002;
pub const SECPKG_CRED_BOTH: ::c_ulong = 0x00000003;
pub const SECPKG_CRED_DEFAULT: ::c_ulong = 0x00000004;
pub const SECPKG_CRED_RESERVED: ::c_ulong = 0xF0000000;
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: ::c_ulong = 0x00000010;
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: ::c_ulong = 0x00000020;
pub const ISC_REQ_DELEGATE: ::c_ulong = 0x00000001;
pub const ISC_REQ_MUTUAL_AUTH: ::c_ulong = 0x00000002;
pub const ISC_REQ_REPLAY_DETECT: ::c_ulong = 0x00000004;
pub const ISC_REQ_SEQUENCE_DETECT: ::c_ulong = 0x00000008;
pub const ISC_REQ_CONFIDENTIALITY: ::c_ulong = 0x00000010;
pub const ISC_REQ_USE_SESSION_KEY: ::c_ulong = 0x00000020;
pub const ISC_REQ_PROMPT_FOR_CREDS: ::c_ulong = 0x00000040;
pub const ISC_REQ_USE_SUPPLIED_CREDS: ::c_ulong = 0x00000080;
pub const ISC_REQ_ALLOCATE_MEMORY: ::c_ulong = 0x00000100;
pub const ISC_REQ_USE_DCE_STYLE: ::c_ulong = 0x00000200;
pub const ISC_REQ_DATAGRAM: ::c_ulong = 0x00000400;
pub const ISC_REQ_CONNECTION: ::c_ulong = 0x00000800;
pub const ISC_REQ_CALL_LEVEL: ::c_ulong = 0x00001000;
pub const ISC_REQ_FRAGMENT_SUPPLIED: ::c_ulong = 0x00002000;
pub const ISC_REQ_EXTENDED_ERROR: ::c_ulong = 0x00004000;
pub const ISC_REQ_STREAM: ::c_ulong = 0x00008000;
pub const ISC_REQ_INTEGRITY: ::c_ulong = 0x00010000;
pub const ISC_REQ_IDENTIFY: ::c_ulong = 0x00020000;
pub const ISC_REQ_NULL_SESSION: ::c_ulong = 0x00040000;
pub const ISC_REQ_MANUAL_CRED_VALIDATION: ::c_ulong = 0x00080000;
pub const ISC_REQ_RESERVED1: ::c_ulong = 0x00100000;
pub const ISC_REQ_FRAGMENT_TO_FIT: ::c_ulong = 0x00200000;
pub const ISC_REQ_FORWARD_CREDENTIALS: ::c_ulong = 0x00400000;
pub const ISC_REQ_NO_INTEGRITY: ::c_ulong = 0x00800000;
pub const ISC_REQ_USE_HTTP_STYLE: ::c_ulong = 0x01000000;
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: ::c_ulong = 0x20000000;
pub const ISC_REQ_CONFIDENTIALITY_ONLY: ::c_ulong = 0x40000000;
pub const ISC_RET_DELEGATE: ::c_ulong = 0x00000001;
pub const ISC_RET_MUTUAL_AUTH: ::c_ulong = 0x00000002;
pub const ISC_RET_REPLAY_DETECT: ::c_ulong = 0x00000004;
pub const ISC_RET_SEQUENCE_DETECT: ::c_ulong = 0x00000008;
pub const ISC_RET_CONFIDENTIALITY: ::c_ulong = 0x00000010;
pub const ISC_RET_USE_SESSION_KEY: ::c_ulong = 0x00000020;
pub const ISC_RET_USED_COLLECTED_CREDS: ::c_ulong = 0x00000040;
pub const ISC_RET_USED_SUPPLIED_CREDS: ::c_ulong = 0x00000080;
pub const ISC_RET_ALLOCATED_MEMORY: ::c_ulong = 0x00000100;
pub const ISC_RET_USED_DCE_STYLE: ::c_ulong = 0x00000200;
pub const ISC_RET_DATAGRAM: ::c_ulong = 0x00000400;
pub const ISC_RET_CONNECTION: ::c_ulong = 0x00000800;
pub const ISC_RET_INTERMEDIATE_RETURN: ::c_ulong = 0x00001000;
pub const ISC_RET_CALL_LEVEL: ::c_ulong = 0x00002000;
pub const ISC_RET_EXTENDED_ERROR: ::c_ulong = 0x00004000;
pub const ISC_RET_STREAM: ::c_ulong = 0x00008000;
pub const ISC_RET_INTEGRITY: ::c_ulong = 0x00010000;
pub const ISC_RET_IDENTIFY: ::c_ulong = 0x00020000;
pub const ISC_RET_NULL_SESSION: ::c_ulong = 0x00040000;
pub const ISC_RET_MANUAL_CRED_VALIDATION: ::c_ulong = 0x00080000;
pub const ISC_RET_RESERVED1: ::c_ulong = 0x00100000;
pub const ISC_RET_FRAGMENT_ONLY: ::c_ulong = 0x00200000;
pub const ISC_RET_FORWARD_CREDENTIALS: ::c_ulong = 0x00400000;
pub const ISC_RET_USED_HTTP_STYLE: ::c_ulong = 0x01000000;
pub const ISC_RET_NO_ADDITIONAL_TOKEN: ::c_ulong = 0x02000000;
pub const ISC_RET_REAUTHENTICATION: ::c_ulong = 0x08000000;
pub const ISC_RET_CONFIDENTIALITY_ONLY: ::c_ulong = 0x40000000;
pub const ASC_REQ_DELEGATE: ::c_ulong = 0x00000001;
pub const ASC_REQ_MUTUAL_AUTH: ::c_ulong = 0x00000002;
pub const ASC_REQ_REPLAY_DETECT: ::c_ulong = 0x00000004;
pub const ASC_REQ_SEQUENCE_DETECT: ::c_ulong = 0x00000008;
pub const ASC_REQ_CONFIDENTIALITY: ::c_ulong = 0x00000010;
pub const ASC_REQ_USE_SESSION_KEY: ::c_ulong = 0x00000020;
pub const ASC_REQ_SESSION_TICKET: ::c_ulong = 0x00000040;
pub const ASC_REQ_ALLOCATE_MEMORY: ::c_ulong = 0x00000100;
pub const ASC_REQ_USE_DCE_STYLE: ::c_ulong = 0x00000200;
pub const ASC_REQ_DATAGRAM: ::c_ulong = 0x00000400;
pub const ASC_REQ_CONNECTION: ::c_ulong = 0x00000800;
pub const ASC_REQ_CALL_LEVEL: ::c_ulong = 0x00001000;
pub const ASC_REQ_EXTENDED_ERROR: ::c_ulong = 0x00008000;
pub const ASC_REQ_STREAM: ::c_ulong = 0x00010000;
pub const ASC_REQ_INTEGRITY: ::c_ulong = 0x00020000;
pub const ASC_REQ_LICENSING: ::c_ulong = 0x00040000;
pub const ASC_REQ_IDENTIFY: ::c_ulong = 0x00080000;
pub const ASC_REQ_ALLOW_NULL_SESSION: ::c_ulong = 0x00100000;
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: ::c_ulong = 0x00200000;
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: ::c_ulong = 0x00400000;
pub const ASC_REQ_FRAGMENT_TO_FIT: ::c_ulong = 0x00800000;
pub const ASC_REQ_FRAGMENT_SUPPLIED: ::c_ulong = 0x00002000;
pub const ASC_REQ_NO_TOKEN: ::c_ulong = 0x01000000;
pub const ASC_REQ_PROXY_BINDINGS: ::c_ulong = 0x04000000;
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: ::c_ulong = 0x10000000;
pub const ASC_RET_DELEGATE: ::c_ulong = 0x00000001;
pub const ASC_RET_MUTUAL_AUTH: ::c_ulong = 0x00000002;
pub const ASC_RET_REPLAY_DETECT: ::c_ulong = 0x00000004;
pub const ASC_RET_SEQUENCE_DETECT: ::c_ulong = 0x00000008;
pub const ASC_RET_CONFIDENTIALITY: ::c_ulong = 0x00000010;
pub const ASC_RET_USE_SESSION_KEY: ::c_ulong = 0x00000020;
pub const ASC_RET_SESSION_TICKET: ::c_ulong = 0x00000040;
pub const ASC_RET_ALLOCATED_MEMORY: ::c_ulong = 0x00000100;
pub const ASC_RET_USED_DCE_STYLE: ::c_ulong = 0x00000200;
pub const ASC_RET_DATAGRAM: ::c_ulong = 0x00000400;
pub const ASC_RET_CONNECTION: ::c_ulong = 0x00000800;
pub const ASC_RET_CALL_LEVEL: ::c_ulong = 0x00002000;
pub const ASC_RET_THIRD_LEG_FAILED: ::c_ulong = 0x00004000;
pub const ASC_RET_EXTENDED_ERROR: ::c_ulong = 0x00008000;
pub const ASC_RET_STREAM: ::c_ulong = 0x00010000;
pub const ASC_RET_INTEGRITY: ::c_ulong = 0x00020000;
pub const ASC_RET_LICENSING: ::c_ulong = 0x00040000;
pub const ASC_RET_IDENTIFY: ::c_ulong = 0x00080000;
pub const ASC_RET_NULL_SESSION: ::c_ulong = 0x00100000;
pub const ASC_RET_ALLOW_NON_USER_LOGONS: ::c_ulong = 0x00200000;
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: ::c_ulong = 0x00400000;
pub const ASC_RET_FRAGMENT_ONLY: ::c_ulong = 0x00800000;
pub const ASC_RET_NO_TOKEN: ::c_ulong = 0x01000000;
pub const ASC_RET_NO_ADDITIONAL_TOKEN: ::c_ulong = 0x02000000;
pub const SECPKG_CRED_ATTR_NAMES: ::c_ulong = 1;
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: ::c_ulong = 2;
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: ::c_ulong = 3;
pub const SECPKG_CRED_ATTR_CERT: ::c_ulong = 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_NamesW {
    pub sUserName: *mut SEC_WCHAR,
}
pub type PSecPkgCredentials_NamesW = *mut SecPkgCredentials_NamesW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_NamesA {
    pub sUserName: *mut SEC_CHAR,
}
pub type PSecPkgCredentials_NamesA = *mut SecPkgCredentials_NamesA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_SSIProviderW {
    pub sProviderName: *mut SEC_WCHAR,
    pub ProviderInfoLength: ::c_ulong,
    pub ProviderInfo: *mut ::c_char,
}
pub type PSecPkgCredentials_SSIProviderW = *mut SecPkgCredentials_SSIProviderW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_SSIProviderA {
    pub sProviderName: *mut SEC_CHAR,
    pub ProviderInfoLength: ::c_ulong,
    pub ProviderInfo: *mut ::c_char,
}
pub type PSecPkgCredentials_SSIProviderA = *mut SecPkgCredentials_SSIProviderA;
pub const KDC_PROXY_SETTINGS_V1: ::ULONG = 1;
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_KdcProxySettingsW {
    pub Version: ::ULONG,
    pub Flags: ::ULONG,
    pub ProxyServerOffset: ::USHORT,
    pub ProxyServerLength: ::USHORT,
    pub ClientTlsCredOffset: ::USHORT,
    pub ClientTlsCredLength: ::USHORT,
}
pub type PSecPkgCredentials_KdcProxySettingsW = *mut SecPkgCredentials_KdcProxySettingsW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgCredentials_Cert {
    pub EncodedCertSize: ::c_ulong,
    pub EncodedCert: *mut ::c_uchar,
}
pub type PSecPkgCredentials_Cert = *mut SecPkgCredentials_Cert;
pub const SECPKG_ATTR_SIZES: ::c_ulong = 0;
pub const SECPKG_ATTR_NAMES: ::c_ulong = 1;
pub const SECPKG_ATTR_LIFESPAN: ::c_ulong = 2;
pub const SECPKG_ATTR_DCE_INFO: ::c_ulong = 3;
pub const SECPKG_ATTR_STREAM_SIZES: ::c_ulong = 4;
pub const SECPKG_ATTR_KEY_INFO: ::c_ulong = 5;
pub const SECPKG_ATTR_AUTHORITY: ::c_ulong = 6;
pub const SECPKG_ATTR_PROTO_INFO: ::c_ulong = 7;
pub const SECPKG_ATTR_PASSWORD_EXPIRY: ::c_ulong = 8;
pub const SECPKG_ATTR_SESSION_KEY: ::c_ulong = 9;
pub const SECPKG_ATTR_PACKAGE_INFO: ::c_ulong = 10;
pub const SECPKG_ATTR_USER_FLAGS: ::c_ulong = 11;
pub const SECPKG_ATTR_NEGOTIATION_INFO: ::c_ulong = 12;
pub const SECPKG_ATTR_NATIVE_NAMES: ::c_ulong = 13;
pub const SECPKG_ATTR_FLAGS: ::c_ulong = 14;
pub const SECPKG_ATTR_USE_VALIDATED: ::c_ulong = 15;
pub const SECPKG_ATTR_CREDENTIAL_NAME: ::c_ulong = 16;
pub const SECPKG_ATTR_TARGET_INFORMATION: ::c_ulong = 17;
pub const SECPKG_ATTR_ACCESS_TOKEN: ::c_ulong = 18;
pub const SECPKG_ATTR_TARGET: ::c_ulong = 19;
pub const SECPKG_ATTR_AUTHENTICATION_ID: ::c_ulong = 20;
pub const SECPKG_ATTR_LOGOFF_TIME: ::c_ulong = 21;
pub const SECPKG_ATTR_NEGO_KEYS: ::c_ulong = 22;
pub const SECPKG_ATTR_PROMPTING_NEEDED: ::c_ulong = 24;
pub const SECPKG_ATTR_UNIQUE_BINDINGS: ::c_ulong = 25;
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: ::c_ulong = 26;
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: ::c_ulong = 27;
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: ::c_ulong = 30;
pub const SECPKG_ATTR_NEGO_PKG_INFO: ::c_ulong = 31;
pub const SECPKG_ATTR_NEGO_STATUS: ::c_ulong = 32;
pub const SECPKG_ATTR_CONTEXT_DELETED: ::c_ulong = 33;
pub const SECPKG_ATTR_DTLS_MTU: ::c_ulong = 34;
pub const SECPKG_ATTR_DATAGRAM_SIZES: ::c_ulong = SECPKG_ATTR_STREAM_SIZES;
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: ::c_ulong = 128;
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: ::c_ulong = 35;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_SubjectAttributes {
    pub AttributeInfo: *mut ::c_void,
}
pub type PSecPkgContext_SubjectAttributes = *mut SecPkgContext_SubjectAttributes;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: ::c_ulong = 0x1;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: ::c_ulong = 0x2;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum SECPKG_CRED_CLASS {
    SecPkgCredClass_None = 0,
    SecPkgCredClass_Ephemeral = 10,
    SecPkgCredClass_PersistedGeneric = 20,
    SecPkgCredClass_PersistedSpecific = 30,
    SecPkgCredClass_Explicit = 40,
}
pub use self::SECPKG_CRED_CLASS::*;
pub type PSECPKG_CRED_CLASS = *mut SECPKG_CRED_CLASS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_CredInfo {
    pub CredClass: SECPKG_CRED_CLASS,
    pub IsPromptingNeeded: ::c_ulong,
}
pub type PSecPkgContext_CredInfo = *mut SecPkgContext_CredInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NegoPackageInfo {
    pub PackageMask: ::c_ulong,
}
pub type PSecPkgContext_NegoPackageInfo = *mut SecPkgContext_NegoPackageInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NegoStatus {
    pub LastStatus: ::c_ulong,
}
pub type PSecPkgContext_NegoStatus = *mut SecPkgContext_NegoStatus;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_Sizes {
    pub cbMaxToken: ::c_ulong,
    pub cbMaxSignature: ::c_ulong,
    pub cbBlockSize: ::c_ulong,
    pub cbSecurityTrailer: ::c_ulong,
}
pub type PSecPkgContext_Sizes = *mut SecPkgContext_Sizes;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_StreamSizes {
    pub cbHeader: ::c_ulong,
    pub cbTrailer: ::c_ulong,
    pub cbMaximumMessage: ::c_ulong,
    pub cBuffers: ::c_ulong,
    pub cbBlockSize: ::c_ulong,
}
pub type PSecPkgContext_StreamSizes = *mut SecPkgContext_StreamSizes;
pub type SecPkgContext_DatagramSizes = SecPkgContext_StreamSizes;
pub type PSecPkgContext_DatagramSizes = PSecPkgContext_StreamSizes;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NamesW {
    pub sUserName: *mut SEC_WCHAR,
}
pub type PSecPkgContext_NamesW = *mut SecPkgContext_NamesW;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum SECPKG_ATTR_LCT_STATUS {
    SecPkgAttrLastClientTokenYes,
    SecPkgAttrLastClientTokenNo,
    SecPkgAttrLastClientTokenMaybe,
}
pub use self::SECPKG_ATTR_LCT_STATUS::*;
pub type PSECPKG_ATTR_LCT_STATUS = *mut SECPKG_ATTR_LCT_STATUS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_LastClientTokenStatus {
    pub LastClientTokenStatus: SECPKG_ATTR_LCT_STATUS,
}
pub type PSecPkgContext_LastClientTokenStatus = *mut SecPkgContext_LastClientTokenStatus;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NamesA {
    pub sUserName: *mut SEC_CHAR,
}
pub type PSecPkgContext_NamesA = *mut SecPkgContext_NamesA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_Lifespan {
    pub tsStart: TimeStamp,
    pub tsExpiry: TimeStamp,
}
pub type PSecPkgContext_Lifespan = *mut SecPkgContext_Lifespan;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_DceInfo {
    pub AuthzSvc: ::c_ulong,
    pub pPac: *mut ::c_void,
}
pub type PSecPkgContext_DceInfo = *mut SecPkgContext_DceInfo;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_KeyInfoA {
    pub sSignatureAlgorithmName: *mut ::SEC_CHAR,
    pub sEncryptAlgorithmName: *mut ::SEC_CHAR,
    pub KeySize: ::c_ulong,
    pub SignatureAlgorithm: ::c_ulong,
    pub EncryptAlgorithm: ::c_ulong,
}
pub type PSecPkgContext_KeyInfoA = *mut SecPkgContext_KeyInfoA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_KeyInfoW {
    pub sSignatureAlgorithmName: *mut ::SEC_WCHAR,
    pub sEncryptAlgorithmName: *mut ::SEC_WCHAR,
    pub KeySize: ::c_ulong,
    pub SignatureAlgorithm: ::c_ulong,
    pub EncryptAlgorithm: ::c_ulong,
}
pub type PSecPkgContext_KeyInfoW = *mut SecPkgContext_KeyInfoW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_AuthorityA {
    pub sAuthorityName: *mut SEC_CHAR,
}
pub type PSecPkgContext_AuthorityA = *mut SecPkgContext_AuthorityA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_AuthorityW {
    pub sAuthorityName: *mut SEC_WCHAR,
}
pub type PSecPkgContext_AuthorityW = *mut SecPkgContext_AuthorityW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_ProtoInfoA {
    pub sProtocolName: *mut SEC_CHAR,
    pub majorVersion: ::c_ulong,
    pub minorVersion: ::c_ulong,
}
pub type PSecPkgContext_ProtoInfoA = *mut SecPkgContext_ProtoInfoA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_ProtoInfoW {
    pub sProtocolName: *mut SEC_WCHAR,
    pub majorVersion: ::c_ulong,
    pub minorVersion: ::c_ulong,
}
pub type PSecPkgContext_ProtoInfoW = *mut SecPkgContext_ProtoInfoW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_PasswordExpiry {
    pub tsPasswordExpires: TimeStamp,
}
pub type PSecPkgContext_PasswordExpiry = *mut SecPkgContext_PasswordExpiry;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_LogoffTime {
    pub tsLogoffTime: TimeStamp,
}
pub type PSecPkgContext_LogoffTime = *mut SecPkgContext_LogoffTime;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_SessionKey {
    pub SessionKeyLength: ::c_ulong,
    pub SessionKey: *mut ::c_uchar,
}
pub type PSecPkgContext_SessionKey = *mut SecPkgContext_SessionKey;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NegoKeys {
    pub KeyType: ::c_ulong,
    pub KeyLength: ::c_ushort,
    pub KeyValue: *mut ::c_uchar,
    pub VerifyKeyType: ::c_ulong,
    pub VerifyKeyLength: ::c_ushort,
    pub VerifyKeyValue: *mut ::c_uchar,
}
pub type PSecPkgContext_NegoKeys = *mut SecPkgContext_NegoKeys;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_PackageInfoW {
    pub PackageInfo: PSecPkgInfoW,
}
pub type PSecPkgContext_PackageInfoW = *mut SecPkgContext_PackageInfoW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_PackageInfoA {
    pub PackageInfo: PSecPkgInfoA,
}
pub type PSecPkgContext_PackageInfoA = *mut SecPkgContext_PackageInfoA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_UserFlags {
    pub UserFlags: ::c_ulong,
}
pub type PSecPkgContext_UserFlags = *mut SecPkgContext_UserFlags;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_Flags {
    pub Flags: ::c_ulong,
}
pub type PSecPkgContext_Flags = *mut SecPkgContext_Flags;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NegotiationInfoA {
    pub PackageInfo: PSecPkgInfoA,
    pub NegotiationState: ::c_ulong,
}
pub type PSecPkgContext_NegotiationInfoA = *mut SecPkgContext_NegotiationInfoA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NegotiationInfoW {
    pub PackageInfo: PSecPkgInfoW,
    pub NegotiationState: ::c_ulong,
}
pub type PSecPkgContext_NegotiationInfoW = *mut SecPkgContext_NegotiationInfoW;
pub const SECPKG_NEGOTIATION_COMPLETE: ::c_ulong = 0;
pub const SECPKG_NEGOTIATION_OPTIMISTIC: ::c_ulong = 1;
pub const SECPKG_NEGOTIATION_IN_PROGRESS: ::c_ulong = 2;
pub const SECPKG_NEGOTIATION_DIRECT: ::c_ulong = 3;
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: ::c_ulong = 4;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NativeNamesW {
    pub sClientName: SEC_WCHAR,
    pub sServerName: SEC_WCHAR,
}
pub type PSecPkgContext_NativeNamesW = *mut SecPkgContext_NativeNamesW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_NativeNamesA {
    pub sClientName: SEC_CHAR,
    pub sServerName: SEC_CHAR,
}
pub type PSecPkgContext_NativeNamesA = *mut SecPkgContext_NativeNamesA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_CredentialNameW {
    pub CredentialType: ::c_ulong,
    pub sCredentialName: *mut SEC_WCHAR,
}
pub type PSecPkgContext_CredentialNameW = *mut SecPkgContext_CredentialNameW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_CredentialNameA {
    pub CredentialType: ::c_ulong,
    pub sCredentialName: *mut SEC_CHAR,
}
pub type PSecPkgContext_CredentialNameA = *mut SecPkgContext_CredentialNameA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_AccessToken {
    pub AccessToken: *mut ::c_void,
}
pub type PSecPkgContext_AccessToken = *mut SecPkgContext_AccessToken;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_TargetInformation {
    pub MarshalledTargetInfoLength: ::c_ulong,
    pub MarshalledTargetInfo: *mut ::c_uchar,
}
pub type PSecPkgContext_TargetInformation = *mut SecPkgContext_TargetInformation;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_AuthzID {
    pub AuthzIDLength: ::c_ulong,
    pub AuthzID: *mut ::c_char,
}
pub type PSecPkgContext_AuthzID = *mut SecPkgContext_AuthzID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_Target {
    pub TargetLength: ::c_ulong,
    pub Target: *mut ::c_char,
}
pub type PSecPkgContext_Target = *mut SecPkgContext_Target;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_ClientSpecifiedTarget {
    pub sTargetName: *mut SEC_WCHAR,
}
pub type PSecPkgContext_ClientSpecifiedTarget = *mut SecPkgContext_ClientSpecifiedTarget;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SecPkgContext_Bindings {
    pub BindingsLength: ::c_ulong,
    pub Bindings: *mut SEC_CHANNEL_BINDINGS,
}
pub type PSecPkgContext_Bindings = *mut SecPkgContext_Bindings;
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    SecApplicationProtocolNegotiationStatus_None,
    SecApplicationProtocolNegotiationStatus_Success,
    SecApplicationProtocolNegotiationStatus_SelectedClientOnly,
}
pub use self::SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS::*;
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS =
    *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS;
pub const MAX_PROTOCOL_ID_SIZE: usize = 0xff;
#[repr(C)] #[derive(Copy)]
pub struct SecPkgContext_ApplicationProtocol {
    pub ProtoNegoStatus: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS,
    pub ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    pub ProtocolIdSize: ::c_uchar,
    pub ProtocolId: [::c_uchar; MAX_PROTOCOL_ID_SIZE],
}
impl Clone for SecPkgContext_ApplicationProtocol {
    fn clone(&self) -> SecPkgContext_ApplicationProtocol { *self }
}
pub type PSecPkgContext_ApplicationProtocol = *mut SecPkgContext_ApplicationProtocol;
pub type SEC_GET_KEY_FN = Option<unsafe extern "system" fn(
    Arg: *mut ::c_void, Principal: *mut ::c_void, KeyVer: ::c_ulong, Key: *mut *mut ::c_void,
    Status: *mut SECURITY_STATUS,
)>;
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: ::c_ulong = 0x00000001;
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: ::c_ulong = 0x00000002;
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: ::c_ulong = 0x00000004;
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_W = Option<unsafe extern "system" fn(
    *mut SEC_WCHAR, *mut SEC_WCHAR, ::c_ulong, *mut ::c_void, *mut ::c_void, SEC_GET_KEY_FN,
    *mut ::c_void, PCredHandle, PTimeStamp,
) -> SECURITY_STATUS>;
pub type ACQUIRE_CREDENTIALS_HANDLE_FN_A = Option<unsafe extern "system" fn(
    *mut SEC_CHAR, *mut SEC_CHAR, ::c_ulong, *mut ::c_void, *mut ::c_void, SEC_GET_KEY_FN,
    *mut ::c_void, PCredHandle, PTimeStamp,
) -> SECURITY_STATUS>;
pub type FREE_CREDENTIALS_HANDLE_FN = Option<unsafe extern "system" fn(
    PCredHandle,
) -> SECURITY_STATUS>;
pub type ADD_CREDENTIALS_FN_W = Option<unsafe extern "system" fn(
    PCredHandle, *mut SEC_WCHAR, *mut SEC_WCHAR, ::c_ulong, *mut ::c_void, SEC_GET_KEY_FN,
    *mut ::c_void, PTimeStamp,
) -> SECURITY_STATUS>;
pub type ADD_CREDENTIALS_FN_A = Option<unsafe extern "system" fn(
    PCredHandle, *mut SEC_CHAR, *mut SEC_CHAR, ::c_ulong, *mut ::c_void, SEC_GET_KEY_FN,
    *mut ::c_void, PTimeStamp,
) -> SECURITY_STATUS>;
pub type CHANGE_PASSWORD_FN_W = Option<unsafe extern "system" fn(
    *mut SEC_WCHAR, *mut SEC_WCHAR, *mut SEC_WCHAR, *mut SEC_WCHAR, *mut SEC_WCHAR, ::BOOLEAN,
    ::c_ulong, PSecBufferDesc,
) -> SECURITY_STATUS>;
pub type CHANGE_PASSWORD_FN_A = Option<unsafe extern "system" fn(
    *mut SEC_CHAR, *mut SEC_CHAR, *mut SEC_CHAR, *mut SEC_CHAR, *mut SEC_CHAR, ::BOOLEAN,
    ::c_ulong, PSecBufferDesc,
) -> SECURITY_STATUS>;
//1844
#[repr(i32)] #[derive(Clone, Copy, Debug)] #[allow(unused_qualifications)]
pub enum SecDelegationType {
    SecFull,
    SecService,
    SecTree,
    SecDirectory,
    SecObject,
}
pub use self::SecDelegationType::*;
pub type PSecDelegationType = *mut SecDelegationType;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_BYTE_VECTOR {
    pub ByteArrayOffset: ::c_ulong,
    pub ByteArrayLength: ::c_ushort,
}
pub type PSEC_WINNT_AUTH_BYTE_VECTOR = *mut SEC_WINNT_AUTH_BYTE_VECTOR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_DATA {
    pub CredType: ::GUID,
    pub CredData: SEC_WINNT_AUTH_BYTE_VECTOR,
}
pub type PSEC_WINNT_AUTH_DATA = *mut SEC_WINNT_AUTH_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_PACKED_CREDENTIALS {
    pub cbHeaderLength: ::c_ushort,
    pub cbStructureLength: ::c_ushort,
    pub AuthData: SEC_WINNT_AUTH_DATA,
}
pub type PSEC_WINNT_AUTH_PACKED_CREDENTIALS = *mut SEC_WINNT_AUTH_PACKED_CREDENTIALS;
DEFINE_GUID!(SEC_WINNT_AUTH_DATA_TYPE_PASSWORD, 0x28bfc32f, 0x10f6, 0x4738,
    0x98, 0xd1, 0x1a, 0xc0, 0x61, 0xdf, 0x71, 0x6a);
DEFINE_GUID!(SEC_WINNT_AUTH_DATA_TYPE_CERT, 0x235f69ad, 0x73fb, 0x4dbc,
    0x82, 0x3, 0x6, 0x29, 0xe7, 0x39, 0x33, 0x9b);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_DATA_PASSWORD {
    pub UnicodePassword: SEC_WINNT_AUTH_BYTE_VECTOR,
}
pub type PSEC_WINNT_AUTH_DATA_PASSWORD = *mut SEC_WINNT_AUTH_DATA_PASSWORD;
DEFINE_GUID!(SEC_WINNT_AUTH_DATA_TYPE_CSP_DATA, 0x68fd9879, 0x79c, 0x4dfe,
    0x82, 0x81, 0x57, 0x8a, 0xad, 0xc1, 0xc1, 0x0);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_CERTIFICATE_DATA {
    pub cbHeaderLength: ::c_ushort,
    pub cbStructureLength: ::c_ushort,
    pub Certificate: SEC_WINNT_AUTH_BYTE_VECTOR,
}
pub type PSEC_WINNT_AUTH_CERTIFICATE_DATA = *mut SEC_WINNT_AUTH_CERTIFICATE_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_CREDUI_CONTEXT_VECTOR {
    pub CredUIContextArrayOffset: ::ULONG,
    pub CredUIContextCount: ::USHORT,
}
pub type PSEC_WINNT_CREDUI_CONTEXT_VECTOR = *mut SEC_WINNT_CREDUI_CONTEXT_VECTOR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_SHORT_VECTOR {
    pub ShortArrayOffset: ::ULONG,
    pub ShortArrayCount: ::USHORT,
}
pub type PSEC_WINNT_AUTH_SHORT_VECTOR = *mut SEC_WINNT_AUTH_SHORT_VECTOR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREDUIWIN_MARSHALED_CONTEXT {
    pub StructureType: ::GUID,
    pub cbHeaderLength: ::USHORT,
    pub LogonId: ::LUID,
    pub MarshaledDataType: ::GUID,
    pub MarshaledDataOffset: ::ULONG,
    pub MarshaledDataLength: ::USHORT,
}
pub type PCREDUIWIN_MARSHALED_CONTEXT = *mut CREDUIWIN_MARSHALED_CONTEXT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_CREDUI_CONTEXT {
    pub cbHeaderLength: ::USHORT,
    pub CredUIContextHandle: ::HANDLE,
    pub UIInfo: ::PCREDUI_INFOW,
    pub dwAuthError: ::ULONG,
    pub pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
    pub TargetName: ::PUNICODE_STRING,
}
pub type PSEC_WINNT_CREDUI_CONTEXT = *mut SEC_WINNT_CREDUI_CONTEXT;

pub type PSEC_WINNT_AUTH_IDENTITY_OPAQUE = ::PVOID;
