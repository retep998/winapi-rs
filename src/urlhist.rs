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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STATURL {
    pub cbSize: ::DWORD,
    pub pwcsUrl: ::LPWSTR,
    pub pwcsTitle: ::LPWSTR,
    pub ftLastVisited: ::FILETIME,
    pub ftLastUpdated: ::FILETIME,
    pub ftExpires: ::FILETIME,
    pub dwFlags: ::DWORD,
}
pub type LPSTATURL = *mut STATURL;
DEFINE_GUID!(IID_IEnumSTATURL, 0x3C374A42, 0xBAE4, 0x11CF,
    0xBF, 0x7D, 0x00, 0xAA, 0x00, 0x69, 0x46, 0xEE);
RIDL!{interface IEnumSTATURL(IEnumSTATURLVtbl): IUnknown(IUnknownVtbl) {
    fn Next(&mut self, celt: ::ULONG, rgelt: LPSTATURL, pceltFetched: *mut ::ULONG) -> ::HRESULT,
    fn Skip(&mut self, celt: ::ULONG) -> ::HRESULT,
    fn Reset(&mut self) -> ::HRESULT,
    fn Clone(&mut self, ppenum: *mut *mut ::IEnumSTATURL) -> ::HRESULT,
    fn SetFilter(&mut self, poszFilter: ::LPCOLESTR, dwFlags: ::DWORD) -> ::HRESULT
}}
pub type LPURLHISTORYSTG = *mut IUrlHistoryStg;
DEFINE_GUID!(IID_IUrlHistoryStg, 0x3C374A41, 0xBAE4, 0x11CF,
    0xBF, 0x7D, 0x00, 0xAA, 0x00, 0x69, 0x46, 0xEE);
RIDL!{interface IUrlHistoryStg(IUrlHistoryStgVtbl): IUnknown(IUnknownVtbl) {
    fn AddUrl(&mut self, pocsUrl: ::LPCOLESTR) -> ::HRESULT,
    fn DeleteUrl(&mut self, pocsUrl: ::LPCOLESTR, dwFlags: ::DWORD) -> ::HRESULT,
    fn QueryUrl(
        &mut self, pocsUrl: ::LPCOLESTR, dwFlags: ::DWORD, lpSTATURL: LPSTATURL
    ) -> ::HRESULT,
    fn BindToObject(
        &mut self, pocsUrl: ::LPCOLESTR, riid: ::REFIID, ppvOut: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn EnumUrls(&mut self, ppEnum: *mut *mut ::IEnumSTATURL) -> ::HRESULT
}}
pub type LPURLHISTORYSTG2 = *mut IUrlHistoryStg2;
DEFINE_GUID!(IID_IUrlHistoryStg2, 0xAFA0DC11, 0xC313, 0x11d0,
    0x83, 0x1A, 0x00, 0xC0, 0x4F, 0xD5, 0xAE, 0x38);
RIDL!{interface IUrlHistoryStg2(IUrlHistoryStg2Vtbl): IUrlHistoryStg(IUrlHistoryStgVtbl) {
    fn AddUrlAndNotify(
        &mut self, pocsUrl: ::LPCOLESTR, pocsTitle: ::LPCOLESTR, dwFlags: ::DWORD,
        fWriteHistory: ::BOOL, poctNotify: *mut ::IOleCommandTarget, punkISFolder: *mut ::IUnknown
    ) -> ::HRESULT,
    fn ClearHistory(&mut self) -> ::HRESULT
}}
pub type LPURLHISTORYNOTIFY = *mut IUrlHistoryNotify;
RIDL!{interface IUrlHistoryNotify(IUrlHistoryNotifyVtbl):
    IOleCommandTarget(IOleCommandTargetVtbl) {}}
