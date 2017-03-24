// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definitions to be used with the WinSock service provider
pub const WSPDESCRIPTION_LEN: usize = 255;
STRUCT!{struct WSPDATA {
    wVersion: ::WORD,
    wHighVersion: ::WORD,
    szDescription: [::WCHAR; WSPDESCRIPTION_LEN + 1],
}}
pub type LPWSPDATA = *mut WSPDATA;
STRUCT!{struct WSATHREADID {
    ThreadHandle: ::HANDLE,
    Reserved: ::DWORD_PTR,
}}
pub type LPWSATHREADID = *mut WSATHREADID;
pub type LPNSPV2STARTUP = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID,
    ppvClientSessionArg: *mut ::LPVOID,
) -> ::INT>;
pub type LPNSPV2CLEANUP = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID,
    pvClientSessionArg: ::LPVOID,
) -> ::INT>;
pub type LPNSPV2LOOKUPSERVICEBEGIN = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID,
    lpqsRestrictions: ::LPWSAQUERYSET2W,
    dwControlFlags: ::DWORD,
    lpvClientSessionArg: ::LPVOID,
    lphLookup: ::LPHANDLE,
) -> ::INT>;
pub type LPNSPV2LOOKUPSERVICENEXTEX = Option<unsafe extern "system" fn(
    hAsyncCall: ::HANDLE,
    hLookup: ::HANDLE,
    dwControlFlags: ::DWORD,
    lpdwBufferLength: ::LPDWORD,
    lpqsResults: ::LPWSAQUERYSET2W,
)>;
pub type LPNSPV2LOOKUPSERVICEEND = Option<unsafe extern "system" fn(hLookup: ::HANDLE) -> ::INT>;
pub type LPNSPV2SETSERVICEEX = Option<unsafe extern "system" fn(
    hAsyncCall: ::HANDLE,
    lpProviderId: ::LPGUID,
    lpqsRegInfo: ::LPWSAQUERYSET2W,
    essOperation: ::WSAESETSERVICEOP,
    dwControlFlags: ::DWORD,
    lpvClientSessionArg: ::LPVOID,
)>;
pub type LPNSPV2CLIENTSESSIONRUNDOWN = Option<unsafe extern "system" fn(
    lpProviderId: ::LPGUID,
    pvClientSessionArg: ::LPVOID,
)>;
STRUCT!{struct NSPV2_ROUTINE {
    cbSize: ::DWORD,
    dwMajorVersion: ::DWORD,
    dwMinorVersion: ::DWORD,
    NSPv2Startup: LPNSPV2STARTUP,
    NSPv2Cleanup: LPNSPV2CLEANUP,
    NSPv2LookupServiceBegin: LPNSPV2LOOKUPSERVICEBEGIN,
    NSPv2LookupServiceNextEx: LPNSPV2LOOKUPSERVICENEXTEX,
    NSPv2LookupServiceEnd: LPNSPV2LOOKUPSERVICEEND,
    NSPv2SetServiceEx: LPNSPV2SETSERVICEEX,
    NSPv2ClientSessionRundown: LPNSPV2CLIENTSESSIONRUNDOWN,
}}
pub type PNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type LPNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type PCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
pub type LPCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
ENUM!{enum WSC_PROVIDER_INFO_TYPE {
    ProviderInfoLspCategories,
    ProviderInfoAudit,
}}
