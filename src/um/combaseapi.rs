// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Base Component Object Model defintions.
use ctypes::c_void;
use shared::basetsd::{SIZE_T, UINT64, ULONG_PTR};
use shared::guiddef::{GUID, LPCLSID, REFCLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, HGLOBAL, LPDWORD, LPVOID};
use shared::wtypesbase::{
    CLSCTX, CLSCTX_INPROC_HANDLER, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, CLSCTX_REMOTE_SERVER,
    LPCOLESTR,
};
use um::objidl::SOLE_AUTHENTICATION_SERVICE;
use um::objidlbase::{APTTYPE, APTTYPEQUALIFIER, LPMALLOC, LPSTREAM};
use um::unknwnbase::LPUNKNOWN;
use um::winnt::{HRESULT, LONG, PSECURITY_DESCRIPTOR, VOID};
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
pub const CLSCTX_ALL: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
pub const CLSCTX_SERVER: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER
    | CLSCTX_REMOTE_SERVER;
ENUM!{enum REGCLS {
    REGCLS_SINGLEUSE = 0,
    REGCLS_MULTIPLEUSE = 1,
    REGCLS_MULTI_SEPARATE = 2,
    REGCLS_SUSPENDED = 4,
    REGCLS_SURROGATE = 8,
    REGCLS_AGILE = 0x10,
}}
ENUM!{enum COINITBASE {
    COINITBASE_MULTITHREADED = 0x0,
}}
extern "system" {
    pub fn CoGetMalloc(
        dwMemContext: DWORD,
        ppMalloc: *mut LPMALLOC,
    ) -> HRESULT;
    pub fn CreateStreamOnHGlobal(
        hGlobal: HGLOBAL,
        fDeleteOnRelease: BOOL,
        ppstm: *mut LPSTREAM,
    ) -> HRESULT;
    pub fn GetHGlobalFromStream(
        pstm: LPSTREAM,
        phglobal: *mut HGLOBAL,
    ) -> HRESULT;
    pub fn CoUninitialize() -> VOID;
    pub fn CoGetCurrentProcess() -> DWORD;
    pub fn CoInitializeEx(
        pvReserved: LPVOID,
        dwCoInit: DWORD,
    ) -> HRESULT;
    pub fn CoGetCallerTID(
        lpdwTID: LPDWORD,
    ) -> HRESULT;
    pub fn CoGetCurrentLogicalThreadId(
        pguid: *mut GUID,
    ) -> HRESULT;
    pub fn CoGetContextToken(
        pToken: *mut ULONG_PTR,
    ) -> HRESULT;
    pub fn CoGetDefaultContext(
        aptType: APTTYPE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CoGetApartmentType(
        pAptType: *mut APTTYPE,
        pAptQualifier: *mut APTTYPEQUALIFIER,
    ) -> HRESULT;
}
STRUCT!{struct ServerInformation {
    dwServerPid: DWORD,
    dwServerTid: DWORD,
    ui64ServerAddress: UINT64,
}}
pub type PServerInformation = *mut ServerInformation;
extern "system" {
    pub fn CoDecodeProxy(
        dwClientPid: DWORD,
        ui64ProxyAddress: UINT64,
        pServerInformation: PServerInformation,
    ) -> HRESULT;
}
DECLARE_HANDLE!(CO_MTA_USAGE_COOKIE, CO_MTA_USAGE_COOKIE__);
extern "system" {
    pub fn CoIncrementMTAUsage(
        pCookie: *mut CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoDecrementMTAUsage(
        Cookie: CO_MTA_USAGE_COOKIE,
    ) -> HRESULT;
    pub fn CoAllowUnmarshalerCLSID(
        clsid: REFCLSID,
    ) -> HRESULT;
    pub fn CoGetObjectContext(
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoGetClassObject(
        rclsid: REFCLSID,
        dwClsContext: DWORD,
        pvReserved: LPVOID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn CoRegisterClassObject(
        rclsid: REFCLSID,
        pUnk: LPUNKNOWN,
        dwClsContext: DWORD,
        flags: DWORD,
        lpdwRegister: LPDWORD,
    ) -> HRESULT;
    pub fn CoRevokeClassObject(
        dwRegister: DWORD,
    ) -> HRESULT;
}
    // pub fn CoResumeClassObjects() -> HRESULT;
    // pub fn CoSuspendClassObjects() -> HRESULT;

    // pub fn CoAddRefServerProcess();
    // pub fn CoReleaseServerProcess();
    // pub fn CoGetPSClsid();
    // pub fn CoRegisterPSClsid();
    // pub fn CoRegisterSurrogate();
    // pub fn CoGetMarshalSizeMax();
    // pub fn CoMarshalInterface();
    // pub fn CoUnmarshalInterface();
    // pub fn CoMarshalHresult();
    // pub fn CoUnmarshalHresult();
    // pub fn CoReleaseMarshalData();
    // pub fn CoDisconnectObject();
    // pub fn CoLockObjectExternal();
    // pub fn CoGetStandardMarshal();
    // pub fn CoGetStdMarshalEx();
ENUM!{enum STDMSHLFLAGS {
    SMEXF_SERVER = 0x01,
    SMEXF_HANDLER = 0x02,
}}
    // pub fn CoIsHandlerConnected();
    // pub fn CoMarshalInterThreadInterfaceInStream();
    // pub fn CoGetInterfaceAndReleaseStream();
    // pub fn CoCreateFreeThreadedMarshaler();
    // pub fn CoFreeUnusedLibraries();
    // pub fn CoFreeUnusedLibrariesEx();
    // pub fn CoDisconnectContext();
extern "system" {
    pub fn CoInitializeSecurity(
        pSecDesc: PSECURITY_DESCRIPTOR,
        cAuthSvc: LONG,
        asAuthSvc: *mut SOLE_AUTHENTICATION_SERVICE,
        pReserved1: *mut c_void,
        dwAuthnLevel: DWORD,
        dwImpLevel: DWORD,
        pAuthList: *mut c_void,
        dwCapabilities: DWORD,
        pReserved3: *mut c_void,
    ) -> HRESULT;
}
    // pub fn CoGetCallContext();
    // pub fn CoQueryProxyBlanket();
    // pub fn CoSetProxyBlanket();
    // pub fn CoCopyProxy();
    // pub fn CoQueryClientBlanket();
    // pub fn CoImpersonateClient();
    // pub fn CoRevertToSelf();
    // pub fn CoQueryAuthenticationServices();
    // pub fn CoSwitchCallContext();
// #define COM_RIGHTS_EXECUTE 1
// #define COM_RIGHTS_EXECUTE_LOCAL 2
// #define COM_RIGHTS_EXECUTE_REMOTE 4
// #define COM_RIGHTS_ACTIVATE_LOCAL 8
// #define COM_RIGHTS_ACTIVATE_REMOTE 16
extern "system" {
    pub fn CoCreateInstance(
        rclsid: REFCLSID,
        pUnkOuter: LPUNKNOWN,
        dwClsContext: DWORD,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
}
    // pub fn CoCreateInstanceEx();
    // pub fn CoRegisterActivationFilter();
    // pub fn CoCreateInstanceFromApp();
    // pub fn CoGetCancelObject();
    // pub fn CoSetCancelObject();
    // pub fn CoCancelCall();
    // pub fn CoTestCancel();
    // pub fn CoEnableCallCancellation();
    // pub fn CoDisableCallCancellation();
    // pub fn StringFromCLSID();
    // pub fn CLSIDFromString();
    // pub fn StringFromIID();
    // pub fn IIDFromString();
    // pub fn ProgIDFromCLSID();
extern "system" {
    pub fn CLSIDFromProgID(
        lpszProgID: LPCOLESTR,
        lpclsid: LPCLSID,
    ) -> HRESULT;
}
    // pub fn StringFromGUID2();
    // pub fn CoCreateGuid();
    // pub fn PropVariantCopy();
    // pub fn PropVariantClear();
    // pub fn FreePropVariantArray();
    // pub fn CoWaitForMultipleHandles();
ENUM!{enum COWAIT_FLAGS {
    COWAIT_DEFAULT = 0,
    COWAIT_WAITALL = 1,
    COWAIT_ALERTABLE = 2,
    COWAIT_INPUTAVAILABLE = 4,
    COWAIT_DISPATCH_CALLS = 8,
    COWAIT_DISPATCH_WINDOW_MESSAGES = 0x10,
}}
ENUM!{enum CWMO_FLAGS {
    CWMO_DEFAULT = 0,
    CWMO_DISPATCH_CALLS = 1,
    CWMO_DISPATCH_WINDOW_MESSAGES = 2,
}}
    // pub fn CoWaitForMultipleObjects();
// #define CWMO_MAX_HANDLES 56
    // pub fn CoGetTreatAsClass();
    // pub fn CoInvalidateRemoteMachineBindings();
ENUM!{enum AgileReferenceOptions {
    AGILEREFERENCE_DEFAULT = 0,
    AGILEREFERENCE_DELAYEDMARSHAL = 1,
}}
    // pub fn RoGetAgileReference();
// typedef HRESULT (STDAPICALLTYPE * LPFNGETCLASSOBJECT) (REFCLSID, REFIID, LPVOID *);
// typedef HRESULT (STDAPICALLTYPE * LPFNCANUNLOADNOW)(void);
extern "system" {
    pub fn DllGetClassObject(
        rclsid: REFCLSID,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    pub fn DllCanUnloadNow() -> HRESULT;
    pub fn CoTaskMemAlloc(
        cb: SIZE_T,
    ) -> LPVOID;
    pub fn CoTaskMemRealloc(
        pv: LPVOID,
        cb: SIZE_T,
    ) -> LPVOID;
    pub fn CoTaskMemFree(
        pv: LPVOID,
    ) -> VOID;
}
