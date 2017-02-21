// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
//8402
STRUCT!{struct BIND_OPTS {
    cbStruct: ::DWORD,
    grfFlags: ::DWORD,
    grfMode: ::DWORD,
    dwTickCountDeadline: ::DWORD,
}}
pub type LPBIND_OPTS = *mut BIND_OPTS;
//8479
RIDL!(
#[uuid(0x0000000e, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(punk: *mut ::IUnknown) -> ::HRESULT,
    fn RevokeObjectBound(punk: *mut ::IUnknown) -> ::HRESULT,
    fn ReleaseBoundObjects() -> ::HRESULT,
    fn SetBindOptions(pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetBindOptions(pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetRunningObjectTable(pprot: *mut *mut IRunningObjectTable) -> ::HRESULT,
    fn RegisterObjectParam(pszKey: ::LPOLESTR, punk: *mut ::IUnknown) -> ::HRESULT,
    fn GetObjectParam(pszKey: ::LPOLESTR, ppunk: *mut *mut ::IUnknown) -> ::HRESULT,
    fn EnumObjectParam(ppenum: *mut *mut ::IEnumString) -> ::HRESULT,
    fn RevokeObjectParam(pszKey: ::LPOLESTR) -> ::HRESULT
}
);
//8681
pub type IEnumMoniker = ::IUnknown; // TODO
//8958
RIDL!(
#[uuid(0x00000010, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        grfFlags: ::DWORD, punkObject: *mut ::IUnknown, pmkObjectName: *mut IMoniker,
        pdwRegister: *mut ::DWORD
    ) -> ::HRESULT,
    fn Revoke(dwRegister: ::DWORD) -> ::HRESULT,
    fn IsRunning(pmkObjectName: *mut IMoniker) -> ::HRESULT,
    fn GetObject(
        pmkObjectName: *mut IMoniker, ppunkObject: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn NoteChangeTime(dwRegister: ::DWORD, pfiletime: *mut ::FILETIME) -> ::HRESULT,
    fn GetTimeOfLastChange(
        pmkObjectName: *mut IMoniker, pfiletime: *mut ::FILETIME
    ) -> ::HRESULT,
    fn EnumRunning(ppenumMoniker: *mut *mut IEnumMoniker) -> ::HRESULT
}
);
//9350
pub type IMoniker = ::IUnknown; // TODO
pub type EOLE_AUTHENTICATION_CAPABILITIES = ::DWORD;
pub const EOAC_NONE: ::DWORD = 0;
pub const EOAC_MUTUAL_AUTH: ::DWORD = 0x1;
pub const EOAC_STATIC_CLOAKING: ::DWORD = 0x20;
pub const EOAC_DYNAMIC_CLOAKING: ::DWORD = 0x40;
pub const EOAC_ANY_AUTHORITY: ::DWORD = 0x80;
pub const EOAC_MAKE_FULLSIC: ::DWORD = 0x100;
pub const EOAC_DEFAULT: ::DWORD = 0x800;
pub const EOAC_SECURE_REFS: ::DWORD = 0x2;
pub const EOAC_ACCESS_CONTROL: ::DWORD = 0x4;
pub const EOAC_APPID: ::DWORD = 0x8;
pub const EOAC_DYNAMIC: ::DWORD = 0x10;
pub const EOAC_REQUIRE_FULLSIC: ::DWORD = 0x200;
pub const EOAC_AUTO_IMPERSONATE: ::DWORD = 0x400;
pub const EOAC_NO_CUSTOM_MARSHAL: ::DWORD = 0x2000;
pub const EOAC_DISABLE_AAA: ::DWORD = 0x1000;
STRUCT!{struct SOLE_AUTHENTICATION_SERVICE {
    dwAuthnSvc: ::DWORD,
    dwAuthzSvc: ::DWORD,
    pPrincipalName: *mut ::OLECHAR,
    hr: ::HRESULT,
}}

RIDL!(
#[uuid(0xA2F05A09, 0x27A2, 0x42B5, 0xBC, 0x0E, 0xAC, 0x16, 0x3E, 0xF4, 0x9D, 0x9B)]
interface IApartmentShutdown(IApartmentShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn OnUninitialize(ui64ApartmentIdentifier: ::UINT64) -> ::VOID
}
);

RIDL!(
#[uuid(0x00000003, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMarshal(IMarshalVtbl): IUnknown(IUnknownVtbl) {
    fn GetUnmarshalClass(
        riid: ::REFIID, pv: *const ::VOID, dwDestContext: ::DWORD,
        pvDestContext: *const ::VOID, mshlflags: ::DWORD, pCid: *mut ::CLSID
    ) -> ::HRESULT,
    fn GetMarshalSizeMax(
        riid: ::REFIID, pv: *const ::VOID, dwDestContext: ::DWORD,
        pvDestContext: *const ::VOID, mshlflags: ::DWORD, pSize: *mut ::DWORD
    ) -> ::HRESULT,
    fn MarshalInterface(
        pStm: *const ::IStream, riid: ::REFIID, pv: *const ::VOID,
        dwDestContext: ::DWORD, pvDestContext: *const ::VOID,
        mshlflags: ::DWORD
    ) -> ::HRESULT,
    fn UnmarshalInterface(
        pStm: *const ::IStream, riid: ::REFIID, ppv: *mut *mut ::VOID
    ) -> ::HRESULT,
    fn ReleaseMarshalData(pStm: *const ::IStream) -> ::HRESULT,
    fn DisconnectObject(dwReserved: ::DWORD) -> ::HRESULT
}
);
