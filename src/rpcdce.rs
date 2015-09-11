// Copyright © 2015, Brian Vincent
// Licensed under the MIT License <LICENSE.md>
// This module contains the DCE RPC runtime APIs.
pub type RPC_CSTR = *mut ::c_uchar;
pub type RPC_WSTR = *mut ::wchar_t;
pub type RPC_CWSTR = *const ::wchar_t;

pub type RPC_BINDING_HANDLE = ::I_RPC_HANDLE;
pub type handle_t = RPC_BINDING_HANDLE;
pub type rpc_binding_handle_t = RPC_BINDING_HANDLE;

pub type UUID = ::GUID;
pub type uuid_t = UUID;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_VECTOR {
    pub Count: ::c_ulong,
    pub BindingH: [RPC_BINDING_HANDLE; 1],
}
pub type rpc_binding_vector_t = RPC_BINDING_VECTOR;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct UUID_VECTOR {
    pub Count: ::c_ulong,
    pub Uuid: [*mut UUID; 1],
}
pub type uuid_vector_t = UUID_VECTOR;

pub type RPC_IF_HANDLE = *mut ::c_void;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_IF_ID {
    pub Uuid: UUID,
    pub VersMajor: ::c_ushort,
    pub VersMinor: ::c_ushort,
}

pub const RPC_C_BINDING_INFINITE_TIMEOUT: ::DWORD = 10;
pub const RPC_C_BINDING_MIN_TIMEOUT: ::DWORD = 0;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: ::DWORD = 5;
pub const RPC_C_BINDING_MAX_TIMEOUT: ::DWORD = 9;

pub const RPC_C_CANCEL_INFINITE_TIMEOUT: ::c_int = -1;

pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: ::DWORD = 1234;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: ::DWORD = 10;

pub const RPC_C_BIND_TO_ALL_NICS: ::DWORD = 1;
pub const RPC_C_USE_INTERNET_PORT: ::DWORD = 0x1;
pub const RPC_C_USE_INTRANET_PORT: ::DWORD = 0x2;
pub const RPC_C_DONT_FAIL: ::DWORD = 0x4;
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: ::DWORD = 0x8;

pub const RPC_C_MQ_TEMPORARY: ::DWORD = 0x0000;
pub const RPC_C_MQ_PERMANENT: ::DWORD = 0x0001;
pub const RPC_C_MQ_CLEAR_ON_OPEN: ::DWORD = 0x0002;
pub const RPC_C_MQ_USE_EXISTING_SECURITY: ::DWORD = 0x0004;
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: ::DWORD = 0x0000;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: ::DWORD = 0x0008;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: ::DWORD = 0x0010;

pub const RPC_C_OPT_MQ_DELIVERY: ::DWORD = 1;
pub const RPC_C_OPT_MQ_PRIORITY: ::DWORD = 2;
pub const RPC_C_OPT_MQ_JOURNAL: ::DWORD = 3;
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: ::DWORD = 4;
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: ::DWORD = 5;
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: ::DWORD = 6;
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: ::DWORD = 7;
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: ::DWORD = 8;
pub const RPC_C_OPT_BINDING_NONCAUSAL: ::DWORD = 9;
pub const RPC_C_OPT_SECURITY_CALLBACK: ::DWORD = 10;
pub const RPC_C_OPT_UNIQUE_BINDING: ::DWORD = 11;
pub const RPC_C_OPT_CALL_TIMEOUT: ::DWORD = 12;
pub const RPC_C_OPT_DONT_LINGER: ::DWORD = 13;
pub const RPC_C_OPT_TRUST_PEER: ::DWORD = 14;
pub const RPC_C_OPT_ASYNC_BLOCK: ::DWORD = 15;
pub const RPC_C_OPT_OPTIMIZE_TIME: ::DWORD = 16;
pub const RPC_C_OPT_MAX_OPTIONS: ::DWORD = 17;

pub const RPC_C_MQ_EXPRESS: ::DWORD = 0;
pub const RPC_C_MQ_RECOVERABLE: ::DWORD = 1;

pub const RPC_C_MQ_JOURNAL_NONE: ::DWORD = 0;
pub const RPC_C_MQ_JOURNAL_DEADLETTER: ::DWORD = 1;
pub const RPC_C_MQ_JOURNAL_ALWAYS: ::DWORD = 2;

pub const RPC_C_FULL_CERT_CHAIN: ::DWORD = 0x0001;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: ::c_uint,
    pub Protseq: [*mut ::c_uchar; 1],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: ::c_uint,
    pub Protseq: [*mut ::c_ushort; 1],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_POLICY {
    pub Length: ::c_uint,
    pub EndpointFlags: ::c_ulong,
    pub NICFlags: ::c_ulong,
}
pub type PRPC_POLICY = *mut RPC_POLICY;

pub type RPC_OBJECT_INQ_FN = Option<unsafe extern "system" fn(
    ObjectUuid: *mut UUID, TypeUuid: *mut UUID, Status: *mut ::RPC_STATUS,
)>;
pub type RPC_IF_CALLBACK_FN = Option<unsafe extern "system" fn(
    InterfaceUuid: RPC_IF_HANDLE, Context: *mut ::c_void,
) -> ::RPC_STATUS>;
pub type RPC_SECURITY_CALLBACK_FN = Option<unsafe extern "system" fn(Context: *mut ::c_void)>;

pub type RPC_MGR_EPV = ::c_void;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_STATS_VECTOR {
    pub Count: ::c_uint,
    pub Stats: [::c_ulong; 1],
}

pub const RPC_C_STATS_CALLS_IN: ::c_ulong = 0;
pub const RPC_C_STATS_CALLS_OUT: ::c_ulong = 1;
pub const RPC_C_STATS_PKTS_IN: ::c_ulong = 2;
pub const RPC_C_STATS_PKTS_OUT: ::c_ulong = 3;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_IF_ID_VECTOR {
    pub Count: ::c_ulong,
    pub IfId: [*mut RPC_IF_ID; 1],
}

pub type RPC_AUTH_IDENTITY_HANDLE = *mut ::c_void;
pub type RPC_AUTHZ_HANDLE = *mut ::c_void;

pub const RPC_C_AUTHN_LEVEL_DEFAULT: ::DWORD = 0;
pub const RPC_C_AUTHN_LEVEL_NONE: ::DWORD = 1;
pub const RPC_C_AUTHN_LEVEL_CONNECT: ::DWORD = 2;
pub const RPC_C_AUTHN_LEVEL_CALL: ::DWORD = 3;
pub const RPC_C_AUTHN_LEVEL_PKT: ::DWORD = 4;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: ::DWORD = 5;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: ::DWORD = 6;

pub const RPC_C_IMP_LEVEL_DEFAULT: ::DWORD = 0;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: ::DWORD = 1;
pub const RPC_C_IMP_LEVEL_IDENTIFY: ::DWORD = 2;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: ::DWORD = 3;
pub const RPC_C_IMP_LEVEL_DELEGATE: ::DWORD = 4;

pub const RPC_C_QOS_IDENTITY_STATIC: ::DWORD = 0;
pub const RPC_C_QOS_IDENTITY_DYNAMIC: ::DWORD = 1;

pub const RPC_C_QOS_CAPABILITIES_DEFAULT: ::DWORD = 0x0;
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: ::DWORD = 0x1;
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: ::DWORD = 0x2;
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: ::DWORD = 0x4;
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: ::DWORD = 0x8;
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: ::DWORD = 0x10;
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: ::DWORD = 0x20;

pub const RPC_C_PROTECT_LEVEL_DEFAULT: ::DWORD = RPC_C_AUTHN_LEVEL_DEFAULT;
pub const RPC_C_PROTECT_LEVEL_NONE: ::DWORD = RPC_C_AUTHN_LEVEL_NONE;
pub const RPC_C_PROTECT_LEVEL_CONNECT: ::DWORD = RPC_C_AUTHN_LEVEL_CONNECT;
pub const RPC_C_PROTECT_LEVEL_CALL: ::DWORD = RPC_C_AUTHN_LEVEL_CALL;
pub const RPC_C_PROTECT_LEVEL_PKT: ::DWORD = RPC_C_AUTHN_LEVEL_PKT;
pub const RPC_C_PROTECT_LEVEL_PKT_INTEGRITY: ::DWORD = RPC_C_AUTHN_LEVEL_PKT_INTEGRITY;
pub const RPC_C_PROTECT_LEVEL_PKT_PRIVACY: ::DWORD = RPC_C_AUTHN_LEVEL_PKT_PRIVACY;

pub const RPC_C_AUTHN_NONE: ::DWORD = 0;
pub const RPC_C_AUTHN_DCE_PRIVATE: ::DWORD = 1;
pub const RPC_C_AUTHN_DCE_PUBLIC: ::DWORD = 2;
pub const RPC_C_AUTHN_DEC_PUBLIC: ::DWORD = 4;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: ::DWORD = 9;
pub const RPC_C_AUTHN_WINNT: ::DWORD = 10;
pub const RPC_C_AUTHN_GSS_SCHANNEL: ::DWORD = 14;
pub const RPC_C_AUTHN_GSS_KERBEROS: ::DWORD = 16;
pub const RPC_C_AUTHN_DPA: ::DWORD = 17;
pub const RPC_C_AUTHN_MSN: ::DWORD = 18;
pub const RPC_C_AUTHN_DIGEST: ::DWORD = 21;
pub const RPC_C_AUTHN_KERNEL: ::DWORD = 20;
pub const RPC_C_AUTHN_NEGO_EXTENDER: ::DWORD = 30;
pub const RPC_C_AUTHN_PKU2U: ::DWORD = 31;
pub const RPC_C_AUTHN_LIVE_SSP: ::DWORD = 32;
pub const RPC_C_AUTHN_LIVEXP_SSP: ::DWORD = 35;
pub const RPC_C_AUTHN_MSONLINE: ::DWORD = 82;
pub const RPC_C_AUTHN_MQ: ::DWORD = 100;
pub const RPC_C_AUTHN_DEFAULT: ::DWORD = 0xFFFFFFFF;

pub const RPC_C_NO_CREDENTIALS: ::DWORD = 0xFFFFFFFF;

pub const RPC_C_SECURITY_QOS_VERSION: ::DWORD = 1;
pub const RPC_C_SECURITY_QOS_VERSION_1: ::DWORD = 1;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
}
pub type PRPC_SECURITY_QOS = *mut RPC_SECURITY_QOS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut ::c_ushort,
    pub UserLength: ::c_ulong,
    pub Domain: *mut ::c_ushort,
    pub DomainLength: ::c_ulong,
    pub Password: *mut ::c_ushort,
    pub PasswordLength: ::c_ulong,
    pub Flags: ::c_ulong,
}
pub type PSEC_WINNT_AUTH_IDENTITY_W = *mut SEC_WINNT_AUTH_IDENTITY_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut ::c_uchar,
    pub UserLength: ::c_ulong,
    pub Domain: *mut ::c_uchar,
    pub DomainLength: ::c_ulong,
    pub Password: *mut ::c_uchar,
    pub PasswordLength: ::c_ulong,
    pub Flags: ::c_ulong,
}
pub type PSEC_WINNT_AUTH_IDENTITY_A = *mut SEC_WINNT_AUTH_IDENTITY_A;

pub const RPC_C_AUTHN_INFO_TYPE_HTTP: ::c_ulong = 1;
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: ::c_ulong = 1;
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: ::c_ulong = 2;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: ::c_ulong = 0x00000001;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: ::c_ulong = 0x00000002;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: ::c_ulong = 0x00000004;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: ::c_ulong = 0x00000008;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: ::c_ulong = 0x00000010;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: ::c_ulong = 0x00010000;
pub const RPC_C_HTTP_FLAG_USE_SSL: ::c_ulong = 1;
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: ::c_ulong = 2;
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: ::c_ulong = 8;
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: ::c_ulong = 16;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_ushort,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_uchar,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_ushort,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: ::c_ulong,
    pub ProxyAuthnSchemes: *mut ::c_ulong,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_uchar,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: ::c_ulong,
    pub ProxyAuthnSchemes: *mut ::c_ulong,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_ushort,
    pub ProxyCredentials: *mut RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: ::c_ulong,
    pub ProxyAuthnSchemes: *mut ::c_ulong,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: ::c_ulong,
    pub AuthenticationTarget: ::c_ulong,
    pub NumberOfAuthnSchemes: ::c_ulong,
    pub AuthnSchemes: *mut ::c_ulong,
    pub ServerCertificateSubject: *mut ::c_uchar,
    pub ProxyCredentials: *mut RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: ::c_ulong,
    pub ProxyAuthnSchemes: *mut ::c_ulong,
}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V2_W_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V2_W_union,
}
pub type PRPC_SECURITY_QOS_V2_W = *mut RPC_SECURITY_QOS_V2_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V2_A_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V2_A_union,
}
pub type PRPC_SECURITY_QOS_V2_A = *mut RPC_SECURITY_QOS_V2_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V3_W_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V3_W_union,
    pub Sid: *mut ::c_void,
}
pub type PRPC_SECURITY_QOS_V3_W = *mut RPC_SECURITY_QOS_V3_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V3_A_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V3_A_union,
    pub Sid: *mut ::c_void,
}
pub type PRPC_SECURITY_QOS_V3_A = *mut RPC_SECURITY_QOS_V3_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V4_W_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V4_W_union,
    pub Sid: *mut ::c_void,
    pub EffectiveOnly: ::c_uint,
}
pub type PRPC_SECURITY_QOS_V4_W = *mut RPC_SECURITY_QOS_V4_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V4_A_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V4_A_union,
    pub Sid: *mut ::c_void,
    pub EffectiveOnly: ::c_uint,
}
pub type PRPC_SECURITY_QOS_V4_A = *mut RPC_SECURITY_QOS_V4_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V5_W_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V5_W_union,
    pub Sid: *mut ::c_void,
    pub EffectiveOnly: ::c_uint,
    pub ServerSecurityDescriptor: *mut ::c_void,
}
pub type PRPC_SECURITY_QOS_V5_W = *mut RPC_SECURITY_QOS_V5_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V5_A_union {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: ::c_ulong,
    pub Capabilities: ::c_ulong,
    pub IdentityTracking: ::c_ulong,
    pub ImpersonationType: ::c_ulong,
    pub AdditionalSecurityInfoType: ::c_ulong,
    pub u: RPC_SECURITY_QOS_V5_A_union,
    pub Sid: *mut ::c_void,
    pub EffectiveOnly: ::c_uint,
    pub ServerSecurityDescriptor: *mut ::c_void,
}
pub type PRPC_SECURITY_QOS_V5_A = *mut RPC_SECURITY_QOS_V5_A;

pub const RPC_PROTSEQ_TCP: ::c_ulong = 0x1;
pub const RPC_PROTSEQ_NMP: ::c_ulong = 0x2;
pub const RPC_PROTSEQ_LRPC: ::c_ulong = 0x3;
pub const RPC_PROTSEQ_HTTP: ::c_ulong = 0x4;

pub const RPC_BHT_OBJECT_UUID_VALID: ::c_ulong = 0x1;

pub const RPC_BHO_NONCAUSAL: ::c_ulong = 0x1;
pub const RPC_BHO_DONTLINGER: ::c_ulong = 0x2;
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: ::c_ulong = 0x4;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W_union {
    pub Reserved: *mut ::c_ushort,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: ::c_ulong,
    pub Flags: ::c_ulong,
    pub ProtocolSequence: ::c_ulong,
    pub NetworkAddress: *mut ::c_ushort,
    pub StringEndpoint: *mut ::c_ushort,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_union,
    pub ObjectUuid: UUID,
}
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_W = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A_union {
    pub Reserved: *mut ::c_uchar,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: ::c_ulong,
    pub Flags: ::c_ulong,
    pub ProtocolSequence: ::c_ulong,
    pub NetworkAddress: *mut ::c_uchar,
    pub StringEndpoint: *mut ::c_uchar,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_union,
    pub ObjectUuid: UUID,
}
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_A = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: ::c_ulong,
    pub ServerPrincName: *mut ::c_ushort,
    pub AuthnLevel: ::c_ulong,
    pub AuthnSvc: ::c_ulong,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
pub type PRPC_BINDING_HANDLE_SECURITY_V1_W = *mut RPC_BINDING_HANDLE_SECURITY_V1_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: ::c_ulong,
    pub ServerPrincName: *mut ::c_uchar,
    pub AuthnLevel: ::c_ulong,
    pub AuthnSvc: ::c_ulong,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
pub type PRPC_BINDING_HANDLE_SECURITY_V1_A = *mut RPC_BINDING_HANDLE_SECURITY_V1_A;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: ::c_ulong,
    pub Flags: ::c_ulong,
    pub ComTimeout: ::c_ulong,
    pub CallTimeout: ::c_ulong,
}
pub type PRPC_BINDING_HANDLE_OPTIONS_V1 = *mut RPC_BINDING_HANDLE_OPTIONS_V1;

ENUM!{enum RPC_HTTP_REDIRECTOR_STAGE {
    RPCHTTP_RS_REDIRECT = 1,
    RPCHTTP_RS_ACCESS_1,
    RPCHTTP_RS_SESSION,
    RPCHTTP_RS_ACCESS_2,
    RPCHTTP_RS_INTERFACE,
}}

pub type RPC_NEW_HTTP_PROXY_CHANNEL = Option<unsafe extern "system" fn(
    RedirectorStage: RPC_HTTP_REDIRECTOR_STAGE, ServerName: RPC_WSTR, ServerPort: RPC_WSTR,
    RemoteUser: RPC_WSTR, AuthType: RPC_WSTR, ResourceUuid: *mut ::c_void,
    SessionId: *mut ::c_void, Interface: *mut ::c_void, Reserved: *mut ::c_void, Flags: ::c_ulong,
    NewServerName: *mut RPC_WSTR, NewServerPort: *mut RPC_WSTR,
) -> ::RPC_STATUS>;
pub type RPC_HTTP_PROXY_FREE_STRING = Option<unsafe extern "system" fn(String: RPC_WSTR)>;

pub const RPC_C_AUTHZ_NONE: ::DWORD = 0;
pub const RPC_C_AUTHZ_NAME: ::DWORD = 1;
pub const RPC_C_AUTHZ_DCE: ::DWORD = 2;
pub const RPC_C_AUTHZ_DEFAULT: ::DWORD = 0xffffffff;

pub type RPC_AUTH_KEY_RETRIEVAL_FN = Option<unsafe extern "system" fn(
    Arg: *mut ::c_void, ServerPrincName: RPC_WSTR, KeyVer: ::c_ulong, Key: *mut *mut ::c_void,
    Status: *mut ::RPC_STATUS,
)>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut ::c_uchar,
    pub ComputerName: *mut ::c_uchar,
    pub Privilege: ::c_ushort,
    pub AuthFlags: ::c_ulong,
}
pub type PRPC_CLIENT_INFORMATION1 = *mut RPC_CLIENT_INFORMATION1;

pub type RPC_EP_INQ_HANDLE = *mut ::I_RPC_HANDLE;

pub const  RPC_C_EP_ALL_ELTS: ::c_ulong = 0;
pub const  RPC_C_EP_MATCH_BY_IF: ::c_ulong = 1;
pub const  RPC_C_EP_MATCH_BY_OBJ: ::c_ulong = 2;
pub const  RPC_C_EP_MATCH_BY_BOTH: ::c_ulong = 3;

pub const  RPC_C_VERS_ALL: ::c_ulong = 1;
pub const  RPC_C_VERS_COMPATIBLE: ::c_ulong = 2;
pub const  RPC_C_VERS_EXACT: ::c_ulong = 3;
pub const  RPC_C_VERS_MAJOR_ONLY: ::c_ulong = 4;
pub const  RPC_C_VERS_UPTO: ::c_ulong = 5;

pub type RPC_MGMT_AUTHORIZATION_FN = Option<unsafe extern "system" fn(
    ClientBinding: RPC_BINDING_HANDLE, RequestedMgmtOperation: ::c_ulong,
    Status: *mut ::RPC_STATUS,
) -> ::c_int>;

pub const RPC_C_MGMT_INQ_IF_IDS: ::c_ulong = 0;
pub const RPC_C_MGMT_INQ_PRINC_NAME: ::c_ulong = 1;
pub const RPC_C_MGMT_INQ_STATS: ::c_ulong = 2;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: ::c_ulong = 3;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: ::c_ulong = 4;

pub const RPC_IF_AUTOLISTEN: ::c_uint = 0x0001;
pub const RPC_IF_OLE: ::c_uint = 0x0002;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: ::c_uint = 0x0004;
pub const RPC_IF_ALLOW_SECURE_ONLY: ::c_uint = 0x0008;
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: ::c_uint = 0x0010;
pub const RPC_IF_ALLOW_LOCAL_ONLY: ::c_uint = 0x0020;
pub const RPC_IF_SEC_NO_CACHE: ::c_uint = 0x0040;
pub const RPC_IF_SEC_CACHE_PER_PROC: ::c_uint = 0x0080;
pub const RPC_IF_ASYNC_CALLBACK: ::c_uint = 0x0100;

pub const RPC_FW_IF_FLAG_DCOM: ::c_uint = 0x0001;

pub type RPC_INTERFACE_GROUP = *mut ::c_void;
pub type PRPC_INTERFACE_GROUP = *mut *mut ::c_void;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: ::c_ulong,
    pub ProtSeq: RPC_WSTR,
    pub Endpoint: RPC_WSTR,
    pub SecurityDescriptor: *mut ::c_void,
    pub Backlog: ::c_ulong,
}
pub type PRPC_ENDPOINT_TEMPLATEW = *mut RPC_ENDPOINT_TEMPLATEW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: ::c_ulong,
    pub ProtSeq: RPC_CSTR,
    pub Endpoint: RPC_CSTR,
    pub SecurityDescriptor: *mut ::c_void,
    pub Backlog: ::c_ulong,
}
pub type PRPC_ENDPOINT_TEMPLATEA = *mut RPC_ENDPOINT_TEMPLATEA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: ::c_ulong,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut UUID,
    pub MgrEpv: *mut RPC_MGR_EPV,
    pub Flags: ::c_uint,
    pub MaxCalls: ::c_uint,
    pub MaxRpcSize: ::c_uint,
    pub IfCallback: *mut RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_CSTR,
    pub SecurityDescriptor: *mut ::c_void,
}
pub type PRPC_INTERFACE_TEMPLATEA = *mut RPC_INTERFACE_TEMPLATEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: ::c_ulong,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut UUID,
    pub MgrEpv: *mut RPC_MGR_EPV,
    pub Flags: ::c_uint,
    pub MaxCalls: ::c_uint,
    pub MaxRpcSize: ::c_uint,
    pub IfCallback: *mut RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_WSTR,
    pub SecurityDescriptor: *mut ::c_void,
}
pub type PRPC_INTERFACE_TEMPLATEW = *mut RPC_INTERFACE_TEMPLATEW;

pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = Option<unsafe extern "system" fn(
    IfGroup: RPC_INTERFACE_GROUP, IdleCallbackContext: *mut ::c_void, IsGroupIdle: ::c_ulong,
)>;
