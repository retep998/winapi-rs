// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Component object model defintions
use ctypes::c_void;
use shared::guiddef::{CLSID, LPCLSID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, FILETIME, HINSTANCE, LPVOID, LPWORD, ULONG, WORD};
use shared::windef::HWND;
use shared::wtypes::{QUERYCONTEXT, uCLSSPEC};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR, OLECHAR};
use um::combaseapi::COINITBASE_MULTITHREADED;
use um::objidl::{
    BIND_OPTS, IBindCtx, IFillLockBytes, IInitializeSpy, ILockBytes, IStorage, LPBC,
    LPDATAADVISEHOLDER, LPMALLOCSPY, LPMESSAGEFILTER, LPMONIKER, LPRUNNINGOBJECTTABLE,
};
use um::objidlbase::{COSERVERINFO, IChannelHook, IMalloc, MULTI_QI};
use um::unknwnbase::{IUnknown, LPUNKNOWN};
use um::urlmon::IBindStatusCallback;
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, PSECURITY_DESCRIPTOR, ULARGE_INTEGER};
ENUM!{enum COINIT {
    COINIT_APARTMENTTHREADED = 0x2,
    COINIT_MULTITHREADED = COINITBASE_MULTITHREADED,
    COINIT_DISABLE_OLE1DDE = 0x4,
    COINIT_SPEED_OVER_MEMORY = 0x8,
}}
pub const MARSHALINTERFACE_MIN: DWORD = 500;
pub const ASYNC_MODE_COMPATIBILITY: DWORD = 0x00000001;
pub const ASYNC_MODE_DEFAULT: DWORD = 0x00000000;
pub const STGTY_REPEAT: DWORD = 0x00000100;
pub const STG_TOEND: DWORD = 0xFFFFFFFF;
pub const STG_LAYOUT_SEQUENTIAL: DWORD = 0x00000000;
pub const STG_LAYOUT_INTERLEAVED: DWORD = 0x00000001;
extern "system" {
    pub fn CoBuildVersion() -> DWORD;
    pub fn CoInitialize(
        pvReserved: LPVOID,
    ) -> HRESULT;
    pub fn CoRegisterMallocSpy(
        pMallocSpy: LPMALLOCSPY,
    ) -> HRESULT;
    pub fn CoRevokeMallocSpy() -> HRESULT;
    pub fn CoCreateStandardMalloc(
        memctx: DWORD,
        ppMalloc: *mut *mut IMalloc,
    ) -> HRESULT;
    pub fn CoRegisterInitializeSpy(
        pSpy: *mut IInitializeSpy,
        puliCookie: *mut ULARGE_INTEGER,
    ) -> HRESULT;
    pub fn CoRevokeInitializeSpy(
        uliCookie: ULARGE_INTEGER,
    ) -> HRESULT;
}
ENUM!{enum COMSD {
    SD_LAUNCHPERMISSIONS = 0,
    SD_ACCESSPERMISSIONS = 1,
    SD_LAUNCHRESTRICTIONS = 2,
    SD_ACCESSRESTRICTIONS = 3,
}}
extern "system" {
    pub fn CoGetSystemSecurityPermissions(
        comSDType: COMSD,
        ppSD: *mut PSECURITY_DESCRIPTOR,
    ) -> HRESULT;
    pub fn CoLoadLibrary(
        lpszLibName: LPOLESTR,
        bAutoFree: BOOL,
    ) -> HINSTANCE;
    pub fn CoFreeLibrary(
        hInst: HINSTANCE,
    );
    pub fn CoFreeAllLibraries();
    pub fn CoGetInstanceFromFile(
        pServerInfo: *mut COSERVERINFO,
        pClsid: *mut CLSID,
        punkOuter: *mut IUnknown,
        dwClsCtx: DWORD,
        grfMode: DWORD,
        pwszName: *mut OLECHAR,
        dwCount: DWORD,
        pResults: *mut MULTI_QI,
    ) -> HRESULT;
    pub fn CoGetInstanceFromIStorage(
        pServerInfo: *mut COSERVERINFO,
        pClsid: *mut CLSID,
        punkOuter: *mut IUnknown,
        dwClsCtx: DWORD,
        pstg: *mut IStorage,
        dwCount: DWORD,
        pResults: *mut MULTI_QI,
    ) -> HRESULT;
    pub fn CoAllowSetForegroundWindow(
        pUnk: *mut IUnknown,
        lpvReserved: LPVOID,
    ) -> HRESULT;
    pub fn DcomChannelSetHResult(
        pvReserved: LPVOID,
        pulReserved: *mut ULONG,
        appsHR: HRESULT,
    ) -> HRESULT;
    pub fn CoIsOle1Class(
        rclsid: REFCLSID,
    ) -> BOOL;
    pub fn CLSIDFromProgIDEx(
        lpszProgID: LPCOLESTR,
        lpclsid: LPCLSID,
    ) -> HRESULT;
    pub fn CoFileTimeToDosDateTime(
        lpFileTime: *mut FILETIME,
        lpDosDate: LPWORD,
        lpDosTime: LPWORD,
    ) -> BOOL;
    pub fn CoDosDateTimeToFileTime(
        nDosDate: WORD,
        nDosTime: WORD,
        lpFileTime: *mut FILETIME,
    ) -> BOOL;
    pub fn CoFileTimeNow(
        lpFileTime: *mut FILETIME,
    ) -> HRESULT;
    pub fn CoRegisterMessageFilter(
        lpMessageFilter: LPMESSAGEFILTER,
        lplpMessageFilter: *mut LPMESSAGEFILTER,
    ) -> HRESULT;
    pub fn CoRegisterChannelHook(
        ExtensionUuid: REFGUID,
        pChannelHook: *mut IChannelHook,
    ) -> HRESULT;
    pub fn CoTreatAsClass(
        clsidOld: REFCLSID,
        clsidNew: REFCLSID,
    ) -> HRESULT;
    pub fn CreateDataAdviseHolder(
        ppDAHolder: *mut LPDATAADVISEHOLDER,
    ) -> HRESULT;
    pub fn CreateDataCache(
        pUnkOuter: LPUNKNOWN,
        rclsid: REFCLSID,
        iid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn StgOpenAsyncDocfileOnIFillLockBytes(
        pflb: *mut IFillLockBytes,
        grfMode: DWORD,
        asyncFlags: DWORD,
        ppstgOpen: *mut *mut IStorage,
    ) -> HRESULT;
    pub fn StgGetIFillLockBytesOnILockBytes(
        pilb: *mut ILockBytes,
        ppflb: *mut IFillLockBytes,
    ) -> HRESULT;
    pub fn StgGetIFillLockBytesOnFile(
        pwcsName: *const OLECHAR,
        ppflb: *mut IFillLockBytes,
    ) -> HRESULT;
    pub fn StgOpenLayoutDocfile(
        pwcsDfName: *const OLECHAR,
        grfMode: DWORD,
        reserved: DWORD,
        ppstgOpen: *mut *mut IStorage,
    ) -> HRESULT;
    pub fn CoInstall(
        pbc: *mut IBindCtx,
        dwFlags: DWORD,
        pClassSpec: *mut uCLSSPEC,
        pQuery: *mut QUERYCONTEXT,
        pszCodeBase: LPWSTR,
    ) -> HRESULT;
    pub fn BindMoniker(
        pmk: LPMONIKER,
        grfOpt: DWORD,
        iidResult: REFIID,
        ppvResult: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoGetObject(
        pszName: LPCWSTR,
        pBindOptions: *mut BIND_OPTS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn MkParseDisplayName(
        pbc: LPBC,
        szUserName: LPCOLESTR,
        pchEaten: *mut ULONG,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn MonikerRelativePathTo(
        pmkSrc: LPMONIKER,
        pmkDest: LPMONIKER,
        ppmkRelPath: *mut LPMONIKER,
        dwReserved: BOOL,
    ) -> HRESULT;
    pub fn MonikerCommonPrefixWith(
        pmkThis: LPMONIKER,
        pmkOther: LPMONIKER,
        ppmkCommon: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateBindCtx(
        reserved: DWORD,
        ppbc: *mut LPBC,
    ) -> HRESULT;
    pub fn CreateGenericComposite(
        pmkFirst: LPMONIKER,
        pmkRest: LPMONIKER,
        ppmkComposite: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn GetClassFile(
        szFilename: LPCOLESTR,
        pclsid: *mut CLSID,
    ) -> HRESULT;
    pub fn CreateClassMoniker(
        rclsid: REFCLSID,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateFileMoniker(
        lpszPathName: LPCOLESTR,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateItemMoniker(
        lpszDelim: LPCOLESTR,
        lpszItem: LPCOLESTR,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateAntiMoniker(
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreatePointerMoniker(
        punk: LPUNKNOWN,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn CreateObjrefMoniker(
        punk: LPUNKNOWN,
        ppmk: *mut LPMONIKER,
    ) -> HRESULT;
    pub fn GetRunningObjectTable(
        reserved: DWORD,
        pprot: *mut LPRUNNINGOBJECTTABLE,
    ) -> HRESULT;
    pub fn CreateStdProgressIndicator(
        hwndParent: HWND,
        pszTitle: LPCOLESTR,
        pIbscCaller: *mut IBindStatusCallback,
        ppIbsc: *mut *mut IBindStatusCallback,
    ) -> HRESULT;
}
