// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! HTTP API specification
pub const HTTP_INITIALIZE_SERVER: ::ULONG = 0x00000001;
pub const HTTP_INITIALIZE_CONFIG: ::ULONG = 0x00000002;
pub const HTTP_DEMAND_CBT: ::ULONG = 0x00000004;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVER_PROPERTY {
    HttpServerAuthenticationProperty,
    HttpServerLoggingProperty,
    HttpServerQosProperty,
    HttpServerTimeoutsProperty,
    HttpServerQueueLengthProperty,
    HttpServerStateProperty,
    HttpServer503VerbosityProperty,
    HttpServerBindingProperty,
    HttpServerExtendedAuthenticationProperty,
    HttpServerListenEndpointProperty,
    HttpServerChannelBindProperty,
    HttpServerProtectionLevelProperty,
}
pub type PHTTP_SERVER_PROPERTY = *mut HTTP_SERVER_PROPERTY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_PROPERTY_FLAGS {
    pub BitFields: ::ULONG,
}
BITFIELD!(HTTP_PROPERTY_FLAGS BitFields: ::ULONG [
    Present set_Present[0..1],
]);
pub type PHTTP_PROPERTY_FLAGS = *mut HTTP_PROPERTY_FLAGS;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_ENABLED_STATE {
    HttpEnabledStateActive,
    HttpEnabledStateInactive,
}
pub type PHTTP_ENABLED_STATE = *mut HTTP_ENABLED_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
pub type PHTTP_STATE_INFO = *mut HTTP_STATE_INFO;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_503_RESPONSE_VERBOSITY {
    Http503ResponseVerbosityBasic,
    Http503ResponseVerbosityLimited,
    Http503ResponseVerbosityFull,
}
pub type PHTTP_503_RESPONSE_VERBOSITY = *mut HTTP_503_RESPONSE_VERBOSITY;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_QOS_SETTING_TYPE {
    HttpQosSettingTypeBandwidth,
    HttpQosSettingTypeConnectionLimit,
    HttpQosSettingTypeFlowRate,
}
pub type PHTTP_QOS_SETTING_TYPE = *mut HTTP_QOS_SETTING_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: ::PVOID,
}
pub type PHTTP_QOS_SETTING_INFO = *mut HTTP_QOS_SETTING_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: ::ULONG,
}
pub type PHTTP_CONNECTION_LIMIT_INFO = *mut HTTP_CONNECTION_LIMIT_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: ::ULONG,
}
pub type PHTTP_BANDWIDTH_LIMIT_INFO = *mut HTTP_BANDWIDTH_LIMIT_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: ::ULONG,
    pub MaxPeakBandwidth: ::ULONG,
    pub BurstSize: ::ULONG,
}
pub type PHTTP_FLOWRATE_INFO = *mut HTTP_FLOWRATE_INFO;
pub const HTTP_MIN_ALLOWED_BANDWIDTH_THROTTLING_RATE: ::ULONG = 1024;
pub const HTTP_LIMIT_INFINITE: ::ULONG = !0;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    IdleConnectionTimeout = 0,
    HeaderWaitTimeout,
}
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_KEY = *mut HTTP_SERVICE_CONFIG_TIMEOUT_KEY;
pub type HTTP_SERVICE_CONFIG_TIMEOUT_PARAM = ::USHORT;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_PARAM = *mut ::USHORT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_TIMEOUT_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_SET = *mut HTTP_SERVICE_CONFIG_TIMEOUT_SET;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: ::USHORT,
    pub DrainEntityBody: ::USHORT,
    pub RequestQueue: ::USHORT,
    pub IdleConnection: ::USHORT,
    pub HeaderWait: ::USHORT,
    pub MinSendRate: ::ULONG,
}
pub type PHTTP_TIMEOUT_LIMIT_INFO = *mut HTTP_TIMEOUT_LIMIT_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: ::BOOLEAN,
}
pub type PHTTP_LISTEN_ENDPOINT_INFO = *mut HTTP_LISTEN_ENDPOINT_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: ::USHORT,
    pub DomainName: ::PWSTR,
    pub RealmLength: ::USHORT,
    pub Realm: ::PWSTR,
}
pub type PHTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: ::USHORT,
    pub Realm: ::PWSTR,
}
pub type PHTTP_SERVER_AUTHENTICATION_BASIC_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS;
pub const HTTP_AUTH_ENABLE_BASIC: ::ULONG = 0x00000001;
pub const HTTP_AUTH_ENABLE_DIGEST: ::ULONG = 0x00000002;
pub const HTTP_AUTH_ENABLE_NTLM: ::ULONG = 0x00000004;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: ::ULONG = 0x00000008;
pub const HTTP_AUTH_ENABLE_KERBEROS: ::ULONG = 0x00000010;
pub const HTTP_AUTH_ENABLE_ALL: ::ULONG =  HTTP_AUTH_ENABLE_BASIC | HTTP_AUTH_ENABLE_DIGEST |
    HTTP_AUTH_ENABLE_NTLM | HTTP_AUTH_ENABLE_NEGOTIATE | HTTP_AUTH_ENABLE_KERBEROS;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: ::UCHAR = 0x01;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: ::UCHAR = 0x02;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVER_AUTHENTICATION_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub AuthSchemes: ::ULONG,
    pub ReceiveMutualAuth: ::BOOLEAN,
    pub ReceiveContextHandle: ::BOOLEAN,
    pub DisableNTLMCredentialCaching: ::BOOLEAN,
    pub ExFlags: ::UCHAR,
    pub DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    pub BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}
pub type PHTTP_SERVER_AUTHENTICATION_INFO = *mut HTTP_SERVER_AUTHENTICATION_INFO;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVICE_BINDING_TYPE {
    HttpServiceBindingTypeNone = 0,
    HttpServiceBindingTypeW,
    HttpServiceBindingTypeA,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
pub type PHTTP_SERVICE_BINDING_BASE = *mut HTTP_SERVICE_BINDING_BASE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: ::PCHAR,
    pub BufferSize: ::ULONG,
}
pub type PHTTP_SERVICE_BINDING_A = *mut HTTP_SERVICE_BINDING_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: ::PWCHAR,
    pub BufferSize: ::ULONG,
}
pub type PHTTP_SERVICE_BINDING_W = *mut HTTP_SERVICE_BINDING_W;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_AUTHENTICATION_HARDENING_LEVELS {
    HttpAuthenticationHardeningLegacy = 0,
    HttpAuthenticationHardeningMedium,
    HttpAuthenticationHardeningStrict,
}
pub const HTTP_CHANNEL_BIND_PROXY: ::ULONG = 0x1;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: ::ULONG = 0x20;
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: ::ULONG = 0x2;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: ::ULONG = 0x4;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: ::ULONG = 0x8;
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: ::ULONG = 0x10;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: ::ULONG,
    pub ServiceNames: *mut PHTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: ::ULONG,
}
pub type PHTTP_CHANNEL_BIND_INFO = *mut HTTP_CHANNEL_BIND_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: PHTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: ::PUCHAR,
    pub ChannelTokenSize: ::ULONG,
    pub Flags: ::ULONG,
}
pub type PHTTP_REQUEST_CHANNEL_BIND_STATUS = *mut HTTP_REQUEST_CHANNEL_BIND_STATUS;
pub const HTTP_LOG_FIELD_DATE: ::ULONG = 0x00000001;
pub const HTTP_LOG_FIELD_TIME: ::ULONG = 0x00000002;
pub const HTTP_LOG_FIELD_CLIENT_IP: ::ULONG = 0x00000004;
pub const HTTP_LOG_FIELD_USER_NAME: ::ULONG = 0x00000008;
pub const HTTP_LOG_FIELD_SITE_NAME: ::ULONG = 0x00000010;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: ::ULONG = 0x00000020;
pub const HTTP_LOG_FIELD_SERVER_IP: ::ULONG = 0x00000040;
pub const HTTP_LOG_FIELD_METHOD: ::ULONG = 0x00000080;
pub const HTTP_LOG_FIELD_URI_STEM: ::ULONG = 0x00000100;
pub const HTTP_LOG_FIELD_URI_QUERY: ::ULONG = 0x00000200;
pub const HTTP_LOG_FIELD_STATUS: ::ULONG = 0x00000400;
pub const HTTP_LOG_FIELD_WIN32_STATUS: ::ULONG = 0x00000800;
pub const HTTP_LOG_FIELD_BYTES_SENT: ::ULONG = 0x00001000;
pub const HTTP_LOG_FIELD_BYTES_RECV: ::ULONG = 0x00002000;
pub const HTTP_LOG_FIELD_TIME_TAKEN: ::ULONG = 0x00004000;
pub const HTTP_LOG_FIELD_SERVER_PORT: ::ULONG = 0x00008000;
pub const HTTP_LOG_FIELD_USER_AGENT: ::ULONG = 0x00010000;
pub const HTTP_LOG_FIELD_COOKIE: ::ULONG = 0x00020000;
pub const HTTP_LOG_FIELD_REFERER: ::ULONG = 0x00040000;
pub const HTTP_LOG_FIELD_VERSION: ::ULONG = 0x00080000;
pub const HTTP_LOG_FIELD_HOST: ::ULONG = 0x00100000;
pub const HTTP_LOG_FIELD_SUB_STATUS: ::ULONG = 0x00200000;
pub const HTTP_LOG_FIELD_CLIENT_PORT: ::ULONG = 0x00400000;
pub const HTTP_LOG_FIELD_URI: ::ULONG = 0x00800000;
pub const HTTP_LOG_FIELD_SITE_ID: ::ULONG = 0x01000000;
pub const HTTP_LOG_FIELD_REASON: ::ULONG = 0x02000000;
pub const HTTP_LOG_FIELD_QUEUE_NAME: ::ULONG = 0x04000000;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_LOGGING_TYPE {
    HttpLoggingTypeW3C,
    HttpLoggingTypeIIS,
    HttpLoggingTypeNCSA,
    HttpLoggingTypeRaw,
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_LOGGING_ROLLOVER_TYPE {
    HttpLoggingRolloverSize,
    HttpLoggingRolloverDaily,
    HttpLoggingRolloverWeekly,
    HttpLoggingRolloverMonthly,
    HttpLoggingRolloverHourly,
}
pub const HTTP_MIN_ALLOWED_LOG_FILE_ROLLOVER_SIZE: ::ULONG = (1 * 1024 * 1024) as ::ULONG;
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: ::ULONG = 0x00000001;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: ::ULONG = 0x00000002;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: ::ULONG = 0x00000004;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: ::ULONG = 0x00000008;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: ::ULONG,
    pub SoftwareName: ::PCWSTR,
    pub SoftwareNameLength: ::USHORT,
    pub DirectoryNameLength: ::USHORT,
    pub DirectoryName: ::PCWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: ::ULONG,
    pub pExtFields: ::PVOID,
    pub NumOfExtFields: ::USHORT,
    pub MaxRecordSize: ::USHORT,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: ::ULONG,
    pub pSecurityDescriptor: ::PSECURITY_DESCRIPTOR,
}
pub type PHTTP_LOGGING_INFO = *mut HTTP_LOGGING_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: ::HANDLE,
}
pub type PHTTP_BINDING_INFO = *mut HTTP_BINDING_INFO;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_PROTECTION_LEVEL_TYPE {
    HttpProtectionLevelUnrestricted,
    HttpProtectionLevelEdgeRestricted,
    HttpProtectionLevelRestricted,
}
pub type PHTTP_PROTECTION_LEVEL_TYPE = *mut HTTP_PROTECTION_LEVEL_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
pub type PHTTP_PROTECTION_LEVEL_INFO = *mut HTTP_PROTECTION_LEVEL_INFO;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: ::ULONG = 0x00000001;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: ::ULONG = 0x00000002;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: ::ULONG = 0x00000001;
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: ::ULONG = 0x00000002;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: ::ULONG = 0x00000001;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: ::ULONG = 0x00000001;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: ::ULONG = 0x00000002;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: ::ULONG = 0x00000004;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: ::ULONG = 0x00000008;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: ::ULONG = 0x00000020;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: ::ULONG = 0x00000040;
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: ::ULONG = 0x00000001;
pub type HTTP_OPAQUE_ID = ::ULONGLONG;
pub type PHTTP_OPAQUE_ID = *mut ::ULONGLONG;
pub type HTTP_REQUEST_ID = HTTP_OPAQUE_ID;
pub type PHTTP_REQUEST_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_CONNECTION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_RAW_CONNECTION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_RAW_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_URL_GROUP_ID = HTTP_OPAQUE_ID;
pub type PHTTP_URL_GROUP_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_SERVER_SESSION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_SERVER_SESSION_ID = *mut HTTP_OPAQUE_ID;
pub const HTTP_BYTE_RANGE_TO_EOF: ::ULONGLONG = !0;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: ::ULARGE_INTEGER,
    pub Length: ::ULARGE_INTEGER,
}
pub type PHTTP_BYTE_RANGE = *mut HTTP_BYTE_RANGE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_VERSION {
    pub MajorVersion: ::USHORT,
    pub MinorVersion: ::USHORT,
}
pub type PHTTP_VERSION = *mut HTTP_VERSION;
pub const HTTP_VERSION_UNKNOWN: HTTP_VERSION = HTTP_VERSION { MajorVersion: 0, MinorVersion: 0 };
pub const HTTP_VERSION_0_9: HTTP_VERSION = HTTP_VERSION { MajorVersion: 0, MinorVersion: 9 };
pub const HTTP_VERSION_1_0: HTTP_VERSION = HTTP_VERSION { MajorVersion: 1, MinorVersion: 0 };
pub const HTTP_VERSION_1_1: HTTP_VERSION = HTTP_VERSION { MajorVersion: 1, MinorVersion: 1 };
#[inline] #[allow(dead_code)]
pub fn HTTP_SET_VERSION(mut version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) {
    version.MajorVersion = major;
    version.MinorVersion = minor;
}
#[inline] #[allow(dead_code)]
pub fn HTTP_EQUAL_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.MajorVersion == major && version.MinorVersion == minor
}
#[inline] #[allow(dead_code)]
pub fn HTTP_GREATER_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.MajorVersion > major || (version.MajorVersion == major && version.MinorVersion > minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTP_LESS_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.MajorVersion < major || (version.MajorVersion == major && version.MinorVersion < minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTP_NOT_EQUAL_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    !HTTP_EQUAL_VERSION(version, major, minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTP_GREATER_EQUAL_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    !HTTP_LESS_VERSION(version, major, minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTP_LESS_EQUAL_VERSION(version: HTTP_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    !HTTP_GREATER_VERSION(version, major, minor)
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_VERB {
    HttpVerbUnparsed,
    HttpVerbUnknown,
    HttpVerbInvalid,
    HttpVerbOPTIONS,
    HttpVerbGET,
    HttpVerbHEAD,
    HttpVerbPOST,
    HttpVerbPUT,
    HttpVerbDELETE,
    HttpVerbTRACE,
    HttpVerbCONNECT,
    HttpVerbTRACK,
    HttpVerbMOVE,
    HttpVerbCOPY,
    HttpVerbPROPFIND,
    HttpVerbPROPPATCH,
    HttpVerbMKCOL,
    HttpVerbLOCK,
    HttpVerbUNLOCK,
    HttpVerbSEARCH,
    HttpVerbMaximum,
}
pub type PHTTP_VERB = *mut HTTP_VERB;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_HEADER_ID {
    HttpHeaderCacheControl = 0,
    HttpHeaderConnection = 1,
    HttpHeaderDate = 2,
    HttpHeaderKeepAlive = 3,
    HttpHeaderPragma = 4,
    HttpHeaderTrailer = 5,
    HttpHeaderTransferEncoding = 6,
    HttpHeaderUpgrade = 7,
    HttpHeaderVia = 8,
    HttpHeaderWarning= 9,
    HttpHeaderAllow= 10,
    HttpHeaderContentLength= 11,
    HttpHeaderContentType= 12,
    HttpHeaderContentEncoding= 13,
    HttpHeaderContentLanguage= 14,
    HttpHeaderContentLocation= 15,
    HttpHeaderContentMd5= 16,
    HttpHeaderContentRange= 17,
    HttpHeaderExpires= 18,
    HttpHeaderLastModified= 19,
    HttpHeaderAccept= 20,
    HttpHeaderAcceptCharset= 21,
    HttpHeaderAcceptEncoding= 22,
    HttpHeaderAcceptLanguage= 23,
    HttpHeaderAuthorization= 24,
    HttpHeaderCookie= 25,
    HttpHeaderExpect= 26,
    HttpHeaderFrom= 27,
    HttpHeaderHost= 28,
    HttpHeaderIfMatch= 29,
    HttpHeaderIfModifiedSince= 30,
    HttpHeaderIfNoneMatch= 31,
    HttpHeaderIfRange= 32,
    HttpHeaderIfUnmodifiedSince= 33,
    HttpHeaderMaxForwards= 34,
    HttpHeaderProxyAuthorization= 35,
    HttpHeaderReferer= 36,
    HttpHeaderRange= 37,
    HttpHeaderTe= 38,
    HttpHeaderTranslate= 39,
    HttpHeaderUserAgent= 40,
    HttpHeaderRequestMaximum= 41,
    // HttpHeaderAcceptRanges= 20,  >>> FIXME: rust doesn't allow duplicate enum value
    // HttpHeaderAge= 21,
    // HttpHeaderEtag= 22,
    // HttpHeaderLocation= 23,
    // HttpHeaderProxyAuthenticate= 24,
    // HttpHeaderRetryAfter= 25,
    // HttpHeaderServer= 26,
    // HttpHeaderSetCookie= 27,
    // HttpHeaderVary= 28,
    // HttpHeaderWwwAuthenticate= 29,
    // HttpHeaderResponseMaximum= 30,
    // HttpHeaderMaximum= 41,
}
pub type PHTTP_HEADER_ID = *mut HTTP_HEADER_ID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: ::USHORT,
    pub pRawValue: ::PCSTR,
}
pub type PHTTP_KNOWN_HEADER = *mut HTTP_KNOWN_HEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: ::USHORT,
    pub RawValueLength: ::USHORT,
    pub pName: ::PCSTR,
    pub pRawValue: ::PCSTR,
}
pub type PHTTP_UNKNOWN_HEADER = *mut HTTP_UNKNOWN_HEADER;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_LOG_DATA_TYPE {
    HttpLogDataTypeFields = 0,
    DUMMY, // rustc --explain E0083
}
pub type PHTTP_LOG_DATA_TYPE = *mut HTTP_LOG_DATA_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
pub type PHTTP_LOG_DATA = *mut HTTP_LOG_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_LOG_FIELDS_DATA {
    pub Base: HTTP_LOG_DATA,
    pub UserNameLength: ::USHORT,
    pub UriStemLength: ::USHORT,
    pub ClientIpLength: ::USHORT,
    pub ServerNameLength: ::USHORT,
    pub ServiceNameLength: ::USHORT,
    pub ServerIpLength: ::USHORT,
    pub MethodLength: ::USHORT,
    pub UriQueryLength: ::USHORT,
    pub HostLength: ::USHORT,
    pub UserAgentLength: ::USHORT,
    pub CookieLength: ::USHORT,
    pub ReferrerLength: ::USHORT,
    pub UserName: ::PWCHAR,
    pub UriStem: ::PWCHAR,
    pub ClientIp: ::PCHAR,
    pub ServerName: ::PCHAR,
    pub ServiceName: ::PCHAR,
    pub ServerIp: ::PCHAR,
    pub Method: ::PCHAR,
    pub UriQuery: ::PCHAR,
    pub Host: ::PCHAR,
    pub UserAgent: ::PCHAR,
    pub Cookie: ::PCHAR,
    pub Referrer: ::PCHAR,
    pub ServerPort: ::USHORT,
    pub ProtocolStatus: ::USHORT,
    pub Win32Status: ::ULONG,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: ::USHORT,
}
pub type PHTTP_LOG_FIELDS_DATA = *mut HTTP_LOG_FIELDS_DATA;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_DATA_CHUNK_TYPE {
    HttpDataChunkFromMemory,
    HttpDataChunkFromFileHandle,
    HttpDataChunkFromFragmentCache,
    HttpDataChunkFromFragmentCacheEx,
    HttpDataChunkMaximum,
}
pub type PHTTP_DATA_CHUNK_TYPE = *mut HTTP_DATA_CHUNK_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_DATA_CHUNK_FromMemory {
    pub pBuffer: ::PVOID,
    pub BufferLength: ::ULONG,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_DATA_CHUNK_FromFileHandle {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: ::HANDLE,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_DATA_CHUNK_FromFragmentCache {
    pub FragmentNameLength: ::USHORT,
    pub pFragmentName: ::PCWSTR,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_DATA_CHUNK_FromFragmentCacheEx {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: ::PCWSTR,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub FromFileHandle: HTTP_DATA_CHUNK_FromFileHandle,
}
UNION!(HTTP_DATA_CHUNK, FromFileHandle, FromMemory, FromMemory_mut, HTTP_DATA_CHUNK_FromMemory);
UNION!(
    HTTP_DATA_CHUNK, FromFileHandle, FromFragmentCache, FromFragmentCache_mut,
    HTTP_DATA_CHUNK_FromFragmentCache
);
UNION!(
    HTTP_DATA_CHUNK, FromFileHandle, FromFragmentCacheEx, FromFragmentCacheEx_mut,
    HTTP_DATA_CHUNK_FromFragmentCacheEx
);
pub type PHTTP_DATA_CHUNK = *mut HTTP_DATA_CHUNK;
#[repr(C)] #[derive(Copy)]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: ::USHORT,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: ::USHORT,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; HTTP_HEADER_ID::HttpHeaderRequestMaximum as usize],
}
impl Clone for HTTP_REQUEST_HEADERS { fn clone(&self) -> HTTP_REQUEST_HEADERS { *self } }
pub type PHTTP_REQUEST_HEADERS = *mut HTTP_REQUEST_HEADERS;
#[repr(C)] #[derive(Copy)]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: ::USHORT,
    pub pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    pub TrailerCount: ::USHORT,
    pub pTrailers: PHTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
    //                                    ^- FIXME: need to be HttpHeaderResponseMaximum
}
impl Clone for HTTP_RESPONSE_HEADERS { fn clone(&self) -> HTTP_RESPONSE_HEADERS { *self } }
pub type PHTTP_RESPONSE_HEADERS = *mut HTTP_RESPONSE_HEADERS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: ::PSOCKADDR,
    pub pLocalAddress: ::PSOCKADDR,
}
pub type PHTTP_TRANSPORT_ADDRESS = *mut HTTP_TRANSPORT_ADDRESS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: ::USHORT,
    pub HostLength: ::USHORT,
    pub AbsPathLength: ::USHORT,
    pub QueryStringLength: ::USHORT,
    pub pFullUrl: ::PCWSTR,
    pub pHost: ::PCWSTR,
    pub pAbsPath: ::PCWSTR,
    pub pQueryString: ::PCWSTR,
}
pub type PHTTP_COOKED_URL = *mut HTTP_COOKED_URL;
pub type HTTP_URL_CONTEXT = ::ULONGLONG;
pub const HTTP_URL_FLAG_REMOVE_ALL: ::ULONG = 0x00000001;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_AUTH_STATUS {
    HttpAuthStatusSuccess,
    HttpAuthStatusNotAuthenticated,
    HttpAuthStatusFailure,
}
pub type PHTTP_AUTH_STATUS = *mut HTTP_AUTH_STATUS;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_REQUEST_AUTH_TYPE {
    HttpRequestAuthTypeNone = 0,
    HttpRequestAuthTypeBasic,
    HttpRequestAuthTypeDigest,
    HttpRequestAuthTypeNTLM,
    HttpRequestAuthTypeNegotiate,
    HttpRequestAuthTypeKerberos,
}
pub type PHTTP_REQUEST_AUTH_TYPE = *mut HTTP_REQUEST_AUTH_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: ::ULONG,
    pub CertEncodedSize: ::ULONG,
    pub pCertEncoded: ::PUCHAR,
    pub Token: ::HANDLE,
    pub CertDeniedByMapper: ::BOOLEAN,
}
pub type PHTTP_SSL_CLIENT_CERT_INFO = *mut HTTP_SSL_CLIENT_CERT_INFO;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: ::ULONG = 0x1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: ::USHORT,
    pub ConnectionKeySize: ::USHORT,
    pub ServerCertIssuerSize: ::ULONG,
    pub ServerCertSubjectSize: ::ULONG,
    pub pServerCertIssuer: ::PCSTR,
    pub pServerCertSubject: ::PCSTR,
    pub pClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: ::ULONG,
}
pub type PHTTP_SSL_INFO = *mut HTTP_SSL_INFO;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_REQUEST_INFO_TYPE {
    HttpRequestInfoTypeAuth,
    HttpRequestInfoTypeChannelBind,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: ::ULONG,
    pub pInfo: ::PVOID,
}
pub type PHTTP_REQUEST_INFO = *mut HTTP_REQUEST_INFO;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: ::ULONG = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: ::SECURITY_STATUS,
    pub Flags: ::ULONG,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: ::HANDLE,
    pub ContextAttributes: ::ULONG,
    pub PackedContextLength: ::ULONG,
    pub PackedContextType: ::ULONG,
    pub PackedContext: ::PVOID,
    pub MutualAuthDataLength: ::ULONG,
    pub pMutualAuthData: ::PCHAR,
    pub PackageNameLength: ::USHORT,
    pub pPackageName: ::PWSTR,
}
pub type PHTTP_REQUEST_AUTH_INFO = *mut HTTP_REQUEST_AUTH_INFO;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_REQUEST_V1 {
    pub Flags: ::ULONG,
    pub ConnectionId: HTTP_CONNECTION_ID,
    pub RequestId: HTTP_REQUEST_ID,
    pub UrlContext: HTTP_URL_CONTEXT,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: ::USHORT,
    pub RawUrlLength: ::USHORT,
    pub pUnknownVerb: ::PCSTR,
    pub pRawUrl: ::PCSTR,
    pub CookedUrl: HTTP_COOKED_URL,
    pub Address: HTTP_TRANSPORT_ADDRESS,
    pub Headers: HTTP_REQUEST_HEADERS,
    pub BytesReceived: ::ULONGLONG,
    pub EntityChunkCount: ::USHORT,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
    pub RawConnectionId: HTTP_RAW_CONNECTION_ID,
    pub pSslInfo: PHTTP_SSL_INFO,
}
pub type PHTTP_REQUEST_V1 = *mut HTTP_REQUEST_V1;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_REQUEST_V2 {
    pub Base: HTTP_REQUEST_V1,
    pub RequestInfoCount: ::USHORT,
    pub pRequestInfo: PHTTP_REQUEST_INFO,
}
pub type PHTTP_REQUEST_V2 = *mut HTTP_REQUEST_V2;
pub type HTTP_REQUEST = HTTP_REQUEST_V2;
pub type PHTTP_REQUEST = *mut HTTP_REQUEST;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: ::ULONG = 0x00000001;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: ::ULONG = 0x00000002;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: ::ULONG,
    pub Version: HTTP_VERSION,
    pub StatusCode: ::USHORT,
    pub ReasonLength: ::USHORT,
    pub pReason: ::PCSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: ::USHORT,
    pub pEntityChunks: PHTTP_DATA_CHUNK,
}
pub type PHTTP_RESPONSE_V1 = *mut HTTP_RESPONSE_V1;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: ::ULONG = 0x00000001;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_RESPONSE_INFO_TYPE {
    HttpResponseInfoTypeMultipleKnownHeaders,
    HttpResponseInfoTypeAuthenticationProperty,
    HttpResponseInfoTypeQoSProperty,
    HttpResponseInfoTypeChannelBind,
}
pub type PHTTP_RESPONSE_INFO_TYPE = *mut HTTP_RESPONSE_INFO_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: ::ULONG,
    pub pInfo: ::PVOID,
}
pub type PHTTP_RESPONSE_INFO = *mut HTTP_RESPONSE_INFO;
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: ::ULONG = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: ::ULONG,
    pub KnownHeaderCount: ::USHORT,
    pub KnownHeaders: PHTTP_KNOWN_HEADER,
}
pub type PHTTP_MULTIPLE_KNOWN_HEADERS = *mut HTTP_MULTIPLE_KNOWN_HEADERS;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_RESPONSE_V2 {
    pub Base: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: ::USHORT,
    pub pResponseInfo: PHTTP_RESPONSE_INFO,
}
pub type PHTTP_RESPONSE_V2 = *mut HTTP_RESPONSE_V2;
pub type HTTP_RESPONSE = HTTP_RESPONSE_V2;
pub type PHTTP_RESPONSE = *mut HTTP_RESPONSE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: ::USHORT,
    pub HttpApiMinorVersion: ::USHORT,
}
pub type PHTTPAPI_VERSION = *mut HTTPAPI_VERSION;
pub const HTTPAPI_VERSION_2: HTTPAPI_VERSION = HTTPAPI_VERSION {
    HttpApiMajorVersion: 2, HttpApiMinorVersion: 0,
};
pub const HTTPAPI_VERSION_1: HTTPAPI_VERSION = HTTPAPI_VERSION {
    HttpApiMajorVersion: 1, HttpApiMinorVersion: 0,
};
#[inline] #[allow(dead_code)]
pub fn HTTPAPI_EQUAL_VERSION(version: HTTPAPI_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.HttpApiMajorVersion == major && version.HttpApiMinorVersion == minor
}
#[inline] #[allow(dead_code)]
pub fn HTTPAPI_GREATER_VERSION(version: HTTPAPI_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.HttpApiMajorVersion > major ||
    (version.HttpApiMajorVersion == major && version.HttpApiMinorVersion > minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTPAPI_LESS_VERSION(version: HTTPAPI_VERSION, major: ::USHORT, minor: ::USHORT) -> bool {
    version.HttpApiMajorVersion < major ||
    (version.HttpApiMajorVersion == major && version.HttpApiMinorVersion < minor)
}
#[inline] #[allow(dead_code)]
pub fn HTTPAPI_VERSION_GREATER_OR_EQUAL(
    version: HTTPAPI_VERSION, major: ::USHORT, minor: ::USHORT
) -> bool {
    !HTTPAPI_LESS_VERSION(version, major, minor)
}
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_CACHE_POLICY_TYPE {
    HttpCachePolicyNocache,
    HttpCachePolicyUserInvalidates,
    HttpCachePolicyTimeToLive,
    HttpCachePolicyMaximum,
}
pub type PHTTP_CACHE_POLICY_TYPE = *mut HTTP_CACHE_POLICY_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: ::ULONG,
}
pub type PHTTP_CACHE_POLICY = *mut HTTP_CACHE_POLICY;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVICE_CONFIG_ID {
    HttpServiceConfigIPListenList,
    HttpServiceConfigSSLCertInfo,
    HttpServiceConfigUrlAclInfo,
    HttpServiceConfigTimeout,
    HttpServiceConfigCache,
    HttpServiceConfigSslSniCertInfo,
    HttpServiceConfigSslCcsCertInfo,
    HttpServiceConfigMax,
}
pub type PHTTP_SERVICE_CONFIG_ID = *mut HTTP_SERVICE_CONFIG_ID;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVICE_CONFIG_QUERY_TYPE {
    HttpServiceConfigQueryExact,
    HttpServiceConfigQueryNext,
    HttpServiceConfigQueryMax,
}
pub type PHTTP_SERVICE_CONFIG_QUERY_TYPE = *mut HTTP_SERVICE_CONFIG_QUERY_TYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: ::PSOCKADDR,
}
pub type PHTTP_SERVICE_CONFIG_SSL_KEY = *mut HTTP_SERVICE_CONFIG_SSL_KEY;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: ::SOCKADDR_STORAGE,
    pub Host: ::PWSTR,
}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_KEY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_KEY;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: ::SOCKADDR_STORAGE,
}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_KEY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_KEY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: ::ULONG,
    pub pSslHash: ::PVOID,
    pub AppId: ::GUID,
    pub pSslCertStoreName: ::PWSTR,
    pub DefaultCertCheckMode: ::DWORD,
    pub DefaultRevocationFreshnessTime: ::DWORD,
    pub DefaultRevocationUrlRetrievalTimeout: ::DWORD,
    pub pDefaultSslCtlIdentifier: ::PWSTR,
    pub pDefaultSslCtlStoreName: ::PWSTR,
    pub DefaultFlags: ::DWORD,
}
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM = *mut HTTP_SERVICE_CONFIG_SSL_PARAM;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: ::DWORD = 0x00000001;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: ::DWORD = 0x00000002;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_SSL_SET = *mut HTTP_SERVICE_CONFIG_SSL_SET;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: ::DWORD,
}
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_QUERY;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: ::DWORD,
}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: ::DWORD,
}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: ::USHORT,
    pub pAddress: ::PSOCKADDR,
}
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_PARAM = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM;
#[repr(C)] #[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: ::ULONG,
    pub AddrList: [::SOCKADDR_STORAGE; ::ANYSIZE_ARRAY],
}
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_QUERY = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: ::PWSTR,
}
pub type PHTTP_SERVICE_CONFIG_URLACL_KEY = *mut HTTP_SERVICE_CONFIG_URLACL_KEY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: ::PWSTR,
}
pub type PHTTP_SERVICE_CONFIG_URLACL_PARAM = *mut HTTP_SERVICE_CONFIG_URLACL_PARAM;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_URLACL_SET = *mut HTTP_SERVICE_CONFIG_URLACL_SET;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: ::DWORD,
}
pub type PHTTP_SERVICE_CONFIG_URLACL_QUERY = *mut HTTP_SERVICE_CONFIG_URLACL_QUERY;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HTTP_SERVICE_CONFIG_CACHE_KEY {
    MaxCacheResponseSize = 0,
    CacheRangeChunkSize,
}
pub type PHTTP_SERVICE_CONFIG_CACHE_KEY = *mut HTTP_SERVICE_CONFIG_CACHE_KEY;
pub type HTTP_SERVICE_CONFIG_CACHE_PARAM = ::ULONG;
pub type PHTTP_SERVICE_CONFIG_CACHE_PARAM = *mut ::ULONG;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_CACHE_PARAM,
}
pub type PHTTP_SERVICE_CONFIG_CACHE_SET = *mut HTTP_SERVICE_CONFIG_CACHE_SET;
