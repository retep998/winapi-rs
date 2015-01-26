// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to ole32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn CoAllowUnmarshalerCLSID(clsid: REFCLSID) -> HRESULT;
    pub fn CoCreateInstance(
        rclsid: REFCLSID, pUnkOuter: LPUNKNOWN, dwClsContext: DWORD, riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoDecodeProxy(
        dwClientPid: DWORD, ui64ProxyAddress: UINT64, pServerInformation: PServerInformation,
    ) -> HRESULT;
    pub fn CoDecrementMTAUsage(Cookie: CO_MTA_USAGE_COOKIE) -> HRESULT;
    pub fn CoGetApartmentType(
        pAptType: *mut APTTYPE, pAptQualifier: *mut APTTYPEQUALIFIER,
    ) -> HRESULT;
    pub fn CoGetCallerTID(lpdwTID: LPDWORD) -> HRESULT;
    pub fn CoGetClassObject(
        rclsid: REFCLSID, dwClsContext: DWORD, pvReserved: LPVOID, riid: REFIID, ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoGetContextToken(pToken: *mut ULONG_PTR) -> HRESULT;
    pub fn CoGetCurrentLogicalThreadId(pguid: *mut GUID) -> HRESULT;
    pub fn CoGetCurrentProcess() -> DWORD;
    pub fn CoGetDefaultContext(aptType: APTTYPE, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
    pub fn CoGetMalloc(dwMemContext: DWORD, ppMalloc: *mut LPMALLOC) -> HRESULT;
    pub fn CoGetObjectContext(riid: REFIID, ppv: *mut LPVOID) -> HRESULT;
    pub fn CoIncrementMTAUsage(pCookie: *mut CO_MTA_USAGE_COOKIE) -> HRESULT;
    pub fn CoInitializeEx(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;
    pub fn CoRegisterClassObject(
        rclsid: REFCLSID, pUnk: LPUNKNOWN, dwClsContext: DWORD, flags: DWORD,
        lpdwRegister: LPDWORD,
    ) -> HRESULT;
    pub fn CoRevokeClassObject() -> HRESULT;
    pub fn CoTaskMemAlloc(cb: SIZE_T) -> LPVOID;
    pub fn CoTaskMemFree(pv: LPVOID);
    pub fn CoTaskMemRealloc(pv: LPVOID, cb: SIZE_T) -> LPVOID;
    pub fn CoUninitialize();
    pub fn CreateStreamOnHGlobal(
        hGlobal: HGLOBAL, fDeleteOnRelease: BOOL, ppstm: *mut LPSTREAM,
    ) -> HRESULT;
    pub fn GetHGlobalFromStream(pstm: LPSTREAM, phglobal: *mut HGLOBAL) -> HRESULT;
}
