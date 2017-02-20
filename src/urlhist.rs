// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Url History Interfaces
pub const STATURL_QUERYFLAG_ISCACHED: ::DWORD = 0x00010000;
pub const STATURL_QUERYFLAG_NOURL: ::DWORD = 0x00020000;
pub const STATURL_QUERYFLAG_NOTITLE: ::DWORD = 0x00040000;
pub const STATURL_QUERYFLAG_TOPLEVEL: ::DWORD = 0x00080000;
pub const STATURLFLAG_ISCACHED: ::DWORD = 0x00000001;
pub const STATURLFLAG_ISTOPLEVEL: ::DWORD = 0x00000002;
ENUM!{enum ADDURL_FLAG {
    ADDURL_FIRST = 0,
    ADDURL_ADDTOHISTORYANDCACHE = 0,
    ADDURL_ADDTOCACHE = 1,
    ADDURL_Max = 2147483647,
}}
pub type LPENUMSTATURL = *mut IEnumSTATURL;
STRUCT!{struct STATURL {
    cbSize: ::DWORD,
    pwcsUrl: ::LPWSTR,
    pwcsTitle: ::LPWSTR,
    ftLastVisited: ::FILETIME,
    ftLastUpdated: ::FILETIME,
    ftExpires: ::FILETIME,
    dwFlags: ::DWORD,
}}
pub type LPSTATURL = *mut STATURL;
RIDL!{interface IEnumSTATURL(IEnumSTATURLVtbl): IUnknown(IUnknownVtbl) {
    fn Next(celt: ::ULONG, rgelt: LPSTATURL, pceltFetched: *mut ::ULONG) -> ::HRESULT,
    fn Skip(celt: ::ULONG) -> ::HRESULT,
    fn Reset() -> ::HRESULT,
    fn Clone(ppenum: *mut *mut ::IEnumSTATURL) -> ::HRESULT,
    fn SetFilter(poszFilter: ::LPCOLESTR, dwFlags: ::DWORD) -> ::HRESULT
}}
pub type LPURLHISTORYSTG = *mut IUrlHistoryStg;
RIDL!{interface IUrlHistoryStg(IUrlHistoryStgVtbl): IUnknown(IUnknownVtbl) {
    fn AddUrl(pocsUrl: ::LPCOLESTR) -> ::HRESULT,
    fn DeleteUrl(pocsUrl: ::LPCOLESTR, dwFlags: ::DWORD) -> ::HRESULT,
    fn QueryUrl(
        pocsUrl: ::LPCOLESTR, dwFlags: ::DWORD, lpSTATURL: LPSTATURL
    ) -> ::HRESULT,
    fn BindToObject(
        pocsUrl: ::LPCOLESTR, riid: ::REFIID, ppvOut: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn EnumUrls(ppEnum: *mut *mut ::IEnumSTATURL) -> ::HRESULT
}}
pub type LPURLHISTORYSTG2 = *mut IUrlHistoryStg2;
RIDL!{interface IUrlHistoryStg2(IUrlHistoryStg2Vtbl): IUrlHistoryStg(IUrlHistoryStgVtbl) {
    fn AddUrlAndNotify(
        pocsUrl: ::LPCOLESTR, pocsTitle: ::LPCOLESTR, dwFlags: ::DWORD,
        fWriteHistory: ::BOOL, poctNotify: *mut ::IOleCommandTarget, punkISFolder: *mut ::IUnknown
    ) -> ::HRESULT,
    fn ClearHistory() -> ::HRESULT
}}
pub type LPURLHISTORYNOTIFY = *mut IUrlHistoryNotify;
RIDL!{interface IUrlHistoryNotify(IUrlHistoryNotifyVtbl):
    IOleCommandTarget(IOleCommandTargetVtbl) {}}
