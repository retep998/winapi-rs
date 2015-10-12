// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Definitions to be used with the WinSock service provider
pub const WSPDESCRIPTION_LEN: usize = 255;
#[repr(C)] #[derive(Copy)]
pub struct WSPDATA {
    pub wVersion: ::WORD,
    pub wHighVersion: ::WORD,
    pub szDescription: [::WCHAR; WSPDESCRIPTION_LEN + 1],
}
impl Clone for WSPDATA { fn clone(&self) -> WSPDATA { *self } }
pub type LPWSPDATA = *mut WSPDATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WSATHREADID {
    pub ThreadHandle: ::HANDLE,
    pub Reserved: ::DWORD_PTR,
}
pub type LPWSATHREADID = *mut WSATHREADID;
pub type LPNSPV2STARTUP = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID, ppvClientSessionArg: *mut ::LPVOID,
) -> ::INT>;
pub type LPNSPV2CLEANUP = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID, pvClientSessionArg: ::LPVOID,
) -> ::INT>;
pub type LPNSPV2LOOKUPSERVICEBEGIN = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID, lpqsRestrictions: ::LPWSAQUERYSET2W, dwControlFlags: ::DWORD,
    lpvClientSessionArg: ::LPVOID, lphLookup: ::LPHANDLE,
) -> ::INT>;
pub type LPNSPV2LOOKUPSERVICENEXTEX = Option<unsafe extern "system" fn(
    hAsyncCall: ::HANDLE, hLookup: ::HANDLE, dwControlFlags: ::DWORD, lpdwBufferLength: ::LPDWORD,
    lpqsResults: ::LPWSAQUERYSET2W
)>;
pub type LPNSPV2LOOKUPSERVICEEND = Option<unsafe extern "system" fn(hLookup: ::HANDLE) -> ::INT>;
pub type LPNSPV2SETSERVICEEX = Option<unsafe extern "system" fn(
    hAsyncCall: ::HANDLE, lpProviderId: ::LPGUID, lpqsRegInfo: ::LPWSAQUERYSET2W,
    essOperation: ::WSAESETSERVICEOP, dwControlFlags: ::DWORD, lpvClientSessionArg: ::LPVOID,
)>;
pub type LPNSPV2CLIENTSESSIONRUNDOWN = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID, pvClientSessionArg: ::LPVOID,
)>;
#[repr(C)] #[derive(Copy)]
pub struct NSPV2_ROUTINE {
    pub cbSize: ::DWORD,
    pub dwMajorVersion: ::DWORD,
    pub dwMinorVersion: ::DWORD,
    pub NSPv2Startup: LPNSPV2STARTUP,
    pub NSPv2Cleanup: LPNSPV2CLEANUP,
    pub NSPv2LookupServiceBegin: LPNSPV2LOOKUPSERVICEBEGIN,
    pub NSPv2LookupServiceNextEx: LPNSPV2LOOKUPSERVICENEXTEX,
    pub NSPv2LookupServiceEnd: LPNSPV2LOOKUPSERVICEEND,
    pub NSPv2SetServiceEx: LPNSPV2SETSERVICEEX,
    pub NSPv2ClientSessionRundown: LPNSPV2CLIENTSESSIONRUNDOWN,
}
impl Clone for NSPV2_ROUTINE { fn clone(&self) -> NSPV2_ROUTINE { *self } }
pub type PNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type LPNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type PCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
pub type LPCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum WSC_PROVIDER_INFO_TYPE {
    ProviderInfoLspCategories,
    ProviderInfoAudit,
}
