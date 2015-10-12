// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! definitions to be used with the WinSock 2 DLL and WinSock 2 applications.
//!
//! This header file corresponds to version 2.2.x of the WinSock API specification.
pub const WINSOCK_VERSION: ::WORD = 2 | (2 << 8);
pub type u_char = ::c_uchar;
pub type u_short = ::c_ushort;
pub type u_int = ::c_uint;
pub type u_long = ::c_ulong;
pub type u_int64 = ::__uint64;
pub type SOCKET = ::UINT_PTR;
pub type GROUP = ::c_uint;
pub const FD_SETSIZE: usize = 64;
pub const FD_MAX_EVENTS: usize = 10;
#[repr(C)] #[derive(Copy)]
pub struct fd_set {
    pub fd_count: u_int,
    pub fd_array: [SOCKET; FD_SETSIZE],
}
impl Clone for fd_set { fn clone(&self) -> fd_set { *self } }
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct timeval {
    pub tv_sec: ::c_long,
    pub tv_usec: ::c_long,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct hostent {
    pub h_name: *mut ::c_char,
    pub h_aliases: *mut *mut ::c_char,
    pub h_addrtype: ::c_short,
    pub h_length: ::c_short,
    pub h_addr_list: *mut *mut ::c_char,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct netent {
    pub n_name: *mut ::c_char,
    pub n_aliases: *mut *mut ::c_char,
    pub n_addrtype: ::c_short,
    pub n_net: u_long,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct servent {
    pub s_name: *mut ::c_char,
    pub s_aliases: *mut *mut ::c_char,
    #[cfg(target_arch="x86")]
    pub s_port: ::c_short,
    #[cfg(target_arch="x86")]
    pub s_proto: *mut ::c_char,
    #[cfg(target_arch="x86_64")]
    pub s_proto: *mut ::c_char,
    #[cfg(target_arch="x86_64")]
    pub s_port: ::c_short,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct protoent {
    pub p_name: *mut ::c_char,
    pub p_aliases: *mut *mut ::c_char,
    pub p_proto: ::c_short,
}
pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;
#[repr(C)] #[derive(Copy)]
pub struct WSADATA {
    pub wVersion: ::WORD,
    pub wHighVersion: ::WORD,
    #[cfg(target_arch="x86")]
    pub szDescription: [::c_char; WSADESCRIPTION_LEN + 1],
    #[cfg(target_arch="x86")]
    pub szSystemStatus: [::c_char; WSASYS_STATUS_LEN + 1],
    pub iMaxSockets: ::c_ushort,
    pub iMaxUdpDg: ::c_ushort,
    pub lpVendorInfo: *mut ::c_char,
    #[cfg(target_arch="x86_64")]
    pub szDescription: [::c_char; WSADESCRIPTION_LEN + 1],
    #[cfg(target_arch="x86_64")]
    pub szSystemStatus: [::c_char; WSASYS_STATUS_LEN + 1],
}
impl Clone for WSADATA { fn clone(&self) -> WSADATA { *self } }
pub type LPWSADATA = *mut WSADATA;
//391
pub const INVALID_SOCKET: SOCKET = !0;
pub const SOCKET_ERROR: ::c_int = -1;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct sockproto {
    pub sp_family: u_short,
    pub sp_protocol: u_short,
}
pub const PF_UNSPEC: ::c_int = ::AF_UNSPEC;
pub const PF_UNIX: ::c_int = ::AF_UNIX;
pub const PF_INET: ::c_int = ::AF_INET;
pub const PF_IMPLINK: ::c_int = ::AF_IMPLINK;
pub const PF_PUP: ::c_int = ::AF_PUP;
pub const PF_CHAOS: ::c_int = ::AF_CHAOS;
pub const PF_NS: ::c_int = ::AF_NS;
pub const PF_IPX: ::c_int = ::AF_IPX;
pub const PF_ISO: ::c_int = ::AF_ISO;
pub const PF_OSI: ::c_int = ::AF_OSI;
pub const PF_ECMA: ::c_int = ::AF_ECMA;
pub const PF_DATAKIT: ::c_int = ::AF_DATAKIT;
pub const PF_CCITT: ::c_int = ::AF_CCITT;
pub const PF_SNA: ::c_int = ::AF_SNA;
pub const PF_DECnet: ::c_int = ::AF_DECnet;
pub const PF_DLI: ::c_int = ::AF_DLI;
pub const PF_LAT: ::c_int = ::AF_LAT;
pub const PF_HYLINK: ::c_int = ::AF_HYLINK;
pub const PF_APPLETALK: ::c_int = ::AF_APPLETALK;
pub const PF_VOICEVIEW: ::c_int = ::AF_VOICEVIEW;
pub const PF_FIREFOX: ::c_int = ::AF_FIREFOX;
pub const PF_UNKNOWN1: ::c_int = ::AF_UNKNOWN1;
pub const PF_BAN: ::c_int = ::AF_BAN;
pub const PF_ATM: ::c_int = ::AF_ATM;
pub const PF_INET6: ::c_int = ::AF_INET6;
pub const PF_BTH: ::c_int = ::AF_BTH;
pub const PF_MAX: ::c_int = ::AF_MAX;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct linger {
    pub l_onoff: u_short,
    pub l_linger: u_short,
}
pub const SOMAXCONN: ::c_int = 0x7fffffff;
pub type WSAEVENT = ::HANDLE;
pub type LPWSAEVENT = ::LPHANDLE;
pub type WSAOVERLAPPED = ::OVERLAPPED;
pub type LPWSAOVERLAPPED = *mut ::OVERLAPPED;
pub const WSA_IO_PENDING: ::DWORD = ::ERROR_IO_PENDING;
pub const WSA_IO_INCOMPLETE: ::DWORD = ::ERROR_IO_INCOMPLETE;
pub const WSA_INVALID_HANDLE: ::DWORD = ::ERROR_INVALID_HANDLE;
pub const WSA_INVALID_PARAMETER: ::DWORD = ::ERROR_INVALID_PARAMETER;
pub const WSA_NOT_ENOUGH_MEMORY: ::DWORD = ::ERROR_NOT_ENOUGH_MEMORY;
pub const WSA_OPERATION_ABORTED: ::DWORD = ::ERROR_OPERATION_ABORTED;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct QOS {
    pub SendingFlowspec: ::FLOWSPEC,
    pub FLOWSPEC: ::FLOWSPEC,
    pub ProviderSpecific: ::WSABUF,
}
pub type LPQOS = *mut QOS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANETWORKEVENTS {
    pub lNetworkEvents: ::c_long,
    pub iErrorCode: [::c_int; FD_MAX_EVENTS],
}
pub type LPWSANETWORKEVENTS = *mut WSANETWORKEVENTS;
pub const MAX_PROTOCOL_CHAIN: usize = 7;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: ::c_int,
    pub ChainEntries: [::DWORD; MAX_PROTOCOL_CHAIN],
}
pub type LPWSAPROTOCOLCHAIN = *mut WSAPROTOCOLCHAIN;
pub const WSAPROTOCOL_LEN: usize = 255;
#[repr(C)] #[derive(Copy)]
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: ::DWORD,
    pub dwServiceFlags2: ::DWORD,
    pub dwServiceFlags3: ::DWORD,
    pub dwServiceFlags4: ::DWORD,
    pub dwServiceFlags5: ::DWORD,
    pub ProviderId: ::GUID,
    pub dwCatalogEntryId: ::DWORD,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: ::c_int,
    pub iAddressFamily: ::c_int,
    pub iMaxSockAddr: ::c_int,
    pub iMinSockAddr: ::c_int,
    pub iSocketType: ::c_int,
    pub iProtocol: ::c_int,
    pub iProtocolMaxOffset: ::c_int,
    pub iNetworkByteOrder: ::c_int,
    pub iSecurityScheme: ::c_int,
    pub dwMessageSize: ::DWORD,
    pub dwProviderReserved: ::DWORD,
    pub szProtocol: [::CHAR; WSAPROTOCOL_LEN + 1],
}
impl Clone for WSAPROTOCOL_INFOA { fn clone(&self) -> WSAPROTOCOL_INFOA { *self } }
pub type LPWSAPROTOCOL_INFOA = *mut WSAPROTOCOL_INFOA;
#[repr(C)] #[derive(Copy)]
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: ::DWORD,
    pub dwServiceFlags2: ::DWORD,
    pub dwServiceFlags3: ::DWORD,
    pub dwServiceFlags4: ::DWORD,
    pub dwServiceFlags5: ::DWORD,
    pub ProviderId: ::GUID,
    pub dwCatalogEntryId: ::DWORD,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: ::c_int,
    pub iAddressFamily: ::c_int,
    pub iMaxSockAddr: ::c_int,
    pub iMinSockAddr: ::c_int,
    pub iSocketType: ::c_int,
    pub iProtocol: ::c_int,
    pub iProtocolMaxOffset: ::c_int,
    pub iNetworkByteOrder: ::c_int,
    pub iSecurityScheme: ::c_int,
    pub dwMessageSize: ::DWORD,
    pub dwProviderReserved: ::DWORD,
    pub szProtocol: [::WCHAR; WSAPROTOCOL_LEN + 1],
}
impl Clone for WSAPROTOCOL_INFOW { fn clone(&self) -> WSAPROTOCOL_INFOW { *self } }
pub type LPWSAPROTOCOL_INFOW = *mut WSAPROTOCOL_INFOW;
pub type LPCONDITIONPROC = Option<unsafe extern "system" fn(
    lpCallerId: ::LPWSABUF, lpCallerData: ::LPWSABUF, lpSQOS: LPQOS, lpGQOS: LPQOS,
    lpCalleeId: ::LPWSABUF, lpCalleeData: ::LPWSABUF, g: *mut GROUP, dwCallbackData: ::DWORD,
) -> ::c_int>;
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(
    dwError: ::DWORD, cbTransferred: ::DWORD, lpOverlapped: LPWSAOVERLAPPED, dwFlags: ::DWORD,
)>;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum WSACOMPLETIONTYPE {
    NSP_NOTIFY_IMMEDIATELY = 0,
    NSP_NOTIFY_HWND,
    NSP_NOTIFY_EVENT,
    NSP_NOTIFY_PORT,
    NSP_NOTIFY_APC,
}
pub type PWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
pub type LPWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSACOMPLETION_WindowMessage {
    pub hWnd: ::HWND,
    pub uMsg: ::UINT,
    pub context: ::WPARAM,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSACOMPLETION_Event {
    pub lpOverlapped: LPWSAOVERLAPPED,
}
#[repr(C)] #[derive(Copy)]
pub struct WSACOMPLETION_Apc {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE
}
impl Clone for WSACOMPLETION_Apc { fn clone(&self) -> WSACOMPLETION_Apc { *self } }
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSACOMPLETION_Port {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub hPort: ::HANDLE,
    pub Key: ::ULONG_PTR,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    #[cfg(target_arch="x86")]
    pub Parameters: [u8; 12],
    #[cfg(target_arch="x86_64")]
    pub Parameters: [u8; 24],
}
UNION!(WSACOMPLETION, Parameters, WindowMessage, WindowMessage_mut, WSACOMPLETION_WindowMessage);
UNION!(WSACOMPLETION, Parameters, Event, Event_mut, WSACOMPLETION_Event);
UNION!(WSACOMPLETION, Parameters, Apc, Apc_mut, WSACOMPLETION_Apc);
UNION!(WSACOMPLETION, Parameters, Port, Port_mut, WSACOMPLETION_Port);
pub type PWSACOMPLETION = *mut WSACOMPLETION;
pub type LPWSACOMPLETION = *mut WSACOMPLETION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct AFPROTOCOLS {
    pub iAddressFamily: ::INT,
    pub iProtocol: ::INT,
}
pub type PAFPROTOCOLS = *mut AFPROTOCOLS;
pub type LPAFPROTOCOLS = *mut AFPROTOCOLS;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum WSAECOMPARATOR {
    COMP_EQUAL = 0,
    COMP_NOTLESS,
}
pub type PWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type LPWSAECOMPARATOR = *mut WSAECOMPARATOR;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAVERSION {
    pub dwVersion: ::DWORD,
    pub ecHow: WSAECOMPARATOR,
}
pub type PWSAVERSION = *mut WSAVERSION;
pub type LPWSAVERSION = *mut WSAVERSION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAQUERYSETA {
    pub dwSize: ::DWORD,
    pub lpszServiceInstanceName: ::LPSTR,
    pub lpServiceClassId: ::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: ::LPSTR,
    pub dwNameSpace: ::DWORD,
    pub lpNSProviderId: ::LPGUID,
    pub lpszContext: ::LPSTR,
    pub dwNumberOfProtocols: ::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: ::LPSTR,
    pub dwNumberOfCsAddrs: ::DWORD,
    pub lpcsaBuffer: ::LPCSADDR_INFO,
    pub dwOutputFlags: ::DWORD,
    pub lpBlob: ::LPBLOB,
}
pub type PWSAQUERYSETA = *mut WSAQUERYSETA;
pub type LPWSAQUERYSETA = *mut WSAQUERYSETA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAQUERYSETW {
    pub dwSize: ::DWORD,
    pub lpszServiceInstanceName: ::LPWSTR,
    pub lpServiceClassId: ::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: ::LPWSTR,
    pub dwNameSpace: ::DWORD,
    pub lpNSProviderId: ::LPGUID,
    pub lpszContext: ::LPWSTR,
    pub dwNumberOfProtocols: ::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: ::LPWSTR,
    pub dwNumberOfCsAddrs: ::DWORD,
    pub lpcsaBuffer: ::LPCSADDR_INFO,
    pub dwOutputFlags: ::DWORD,
    pub lpBlob: ::LPBLOB,
}
pub type PWSAQUERYSETW = *mut WSAQUERYSETW;
pub type LPWSAQUERYSETW = *mut WSAQUERYSETW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAQUERYSET2A {
    pub dwSize: ::DWORD,
    pub lpszServiceInstanceName: ::LPSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: ::LPSTR,
    pub dwNameSpace: ::DWORD,
    pub lpNSProviderId: ::LPGUID,
    pub lpszContext: ::LPSTR,
    pub dwNumberOfProtocols: ::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: ::LPSTR,
    pub dwNumberOfCsAddrs: ::DWORD,
    pub lpcsaBuffer: ::LPCSADDR_INFO,
    pub dwOutputFlags: ::DWORD,
    pub lpBlob: ::LPBLOB,
}
pub type PWSAQUERYSET2A = *mut WSAQUERYSET2A;
pub type LPWSAQUERYSET2A = *mut WSAQUERYSET2A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAQUERYSET2W {
    pub dwSize: ::DWORD,
    pub lpszServiceInstanceName: ::LPWSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: ::LPWSTR,
    pub dwNameSpace: ::DWORD,
    pub lpNSProviderId: ::LPGUID,
    pub lpszContext: ::LPWSTR,
    pub dwNumberOfProtocols: ::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: ::LPWSTR,
    pub dwNumberOfCsAddrs: ::DWORD,
    pub lpcsaBuffer: ::LPCSADDR_INFO,
    pub dwOutputFlags: ::DWORD,
    pub lpBlob: ::LPBLOB,
}
pub type PWSAQUERYSET2W = *mut WSAQUERYSET2W;
pub type LPWSAQUERYSET2W = *mut WSAQUERYSET2W;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum WSAESETSERVICEOP {
    RNRSERVICE_REGISTER = 0,
    RNRSERVICE_DEREGISTER,
    RNRSERVICE_DELETE,
}
pub type PWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
pub type LPWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANSCLASSINFOA {
    pub lpszName: ::LPSTR,
    pub dwNameSpace: ::DWORD,
    pub dwValueType: ::DWORD,
    pub dwValueSize: ::DWORD,
    pub lpValue: ::LPVOID,
}
pub type PWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
pub type LPWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANSCLASSINFOW {
    pub lpszName: ::LPWSTR,
    pub dwNameSpace: ::DWORD,
    pub dwValueType: ::DWORD,
    pub dwValueSize: ::DWORD,
    pub lpValue: ::LPVOID,
}
pub type PWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
pub type LPWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: ::LPGUID,
    pub lpszServiceClassName: ::LPSTR,
    pub dwCount: ::DWORD,
    pub lpClassInfos: LPWSANSCLASSINFOA,
}
pub type PWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
pub type LPWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: ::LPGUID,
    pub lpszServiceClassName: ::LPWSTR,
    pub dwCount: ::DWORD,
    pub lpClassInfos: LPWSANSCLASSINFOW,
}
pub type PWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
pub type LPWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: ::GUID,
    pub dwNameSpace: ::DWORD,
    pub fActive: ::BOOL,
    pub dwVersion: ::DWORD,
    pub lpszIdentifier: ::LPSTR,
}
pub type PWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
pub type LPWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: ::GUID,
    pub dwNameSpace: ::DWORD,
    pub fActive: ::BOOL,
    pub dwVersion: ::DWORD,
    pub lpszIdentifier: ::LPWSTR,
}
pub type PWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
pub type LPWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: ::GUID,
    pub dwNameSpace: ::DWORD,
    pub fActive: ::BOOL,
    pub dwVersion: ::DWORD,
    pub lpszIdentifier: ::LPSTR,
    pub ProviderSpecific: ::BLOB,
}
pub type PWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
pub type LPWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: ::GUID,
    pub dwNameSpace: ::DWORD,
    pub fActive: ::BOOL,
    pub dwVersion: ::DWORD,
    pub lpszIdentifier: ::LPWSTR,
    pub ProviderSpecific: ::BLOB,
}
pub type PWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub type LPWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub const POLLRDNORM: ::SHORT = 0x0100;
pub const POLLRDBAND: ::SHORT = 0x0200;
pub const POLLIN: ::SHORT = POLLRDNORM | POLLRDBAND;
pub const POLLPRI: ::SHORT = 0x0400;
pub const POLLWRNORM: ::SHORT = 0x0010;
pub const POLLOUT: ::SHORT = POLLWRNORM;
pub const POLLWRBAND: ::SHORT = 0x0020;
pub const POLLERR: ::SHORT = 0x0001;
pub const POLLHUP: ::SHORT = 0x0002;
pub const POLLNVAL: ::SHORT = 0x0004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSAPOLLFD {
    pub fd: ::SOCKET,
    pub events: ::SHORT,
    pub revents: ::SHORT,
}
pub type PWSAPOLLFD = *mut WSAPOLLFD;
pub type LPWSAPOLLFD = *mut WSAPOLLFD;
