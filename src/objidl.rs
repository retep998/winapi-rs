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
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(&self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn RevokeObjectBound(&self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn ReleaseBoundObjects(&self) -> ::HRESULT,
    fn SetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetRunningObjectTable(&self, pprot: *mut *mut IRunningObjectTable) -> ::HRESULT,
    fn RegisterObjectParam(&self, pszKey: ::LPOLESTR, punk: *mut ::IUnknown) -> ::HRESULT,
    fn GetObjectParam(&self, pszKey: ::LPOLESTR, ppunk: *mut *mut ::IUnknown) -> ::HRESULT,
    fn EnumObjectParam(&self, ppenum: *mut *mut ::IEnumString) -> ::HRESULT,
    fn RevokeObjectParam(&self, pszKey: ::LPOLESTR) -> ::HRESULT
}
);
//8681
pub type IEnumMoniker = ::IUnknown; // TODO
//8958
RIDL!(
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        &self, grfFlags: ::DWORD, punkObject: *mut ::IUnknown, pmkObjectName: *mut IMoniker,
        pdwRegister: *mut ::DWORD
    ) -> ::HRESULT,
    fn Revoke(&self, dwRegister: ::DWORD) -> ::HRESULT,
    fn IsRunning(&self, pmkObjectName: *mut IMoniker) -> ::HRESULT,
    fn GetObject(
        &self, pmkObjectName: *mut IMoniker, ppunkObject: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn NoteChangeTime(&self, dwRegister: ::DWORD, pfiletime: *mut ::FILETIME) -> ::HRESULT,
    fn GetTimeOfLastChange(
        &self, pmkObjectName: *mut IMoniker, pfiletime: *mut ::FILETIME
    ) -> ::HRESULT,
    fn EnumRunning(&self, ppenumMoniker: *mut *mut IEnumMoniker) -> ::HRESULT
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
interface IApartmentShutdown(IApartmentShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn OnUninitialize(&self, ui64ApartmentIdentifier: ::UINT64) -> ::VOID
}
);

RIDL!(
interface IMarshal(IMarshalVtbl): IUnknown(IUnknownVtbl) {
    fn GetUnmarshalClass(
        &self, riid: ::REFIID, pv: *const ::VOID, dwDestContext: ::DWORD,
        pvDestContext: *const ::VOID, mshlflags: ::DWORD, pCid: *mut ::CLSID
    ) -> ::HRESULT,
    fn GetMarshalSizeMax(
        &self, riid: ::REFIID, pv: *const ::VOID, dwDestContext: ::DWORD,
        pvDestContext: *const ::VOID, mshlflags: ::DWORD, pSize: *mut ::DWORD
    ) -> ::HRESULT,
    fn MarshalInterface(
        &self, pStm: *const ::IStream, riid: ::REFIID, pv: *const ::VOID,
        dwDestContext: ::DWORD, pvDestContext: *const ::VOID,
        mshlflags: ::DWORD
    ) -> ::HRESULT,
    fn UnmarshalInterface(
        &self, pStm: *const ::IStream, riid: ::REFIID, ppv: *mut *mut ::VOID
    ) -> ::HRESULT,
    fn ReleaseMarshalData(&self, pStm: *const ::IStream) -> ::HRESULT,
    fn DisconnectObject(&self, dwReserved: ::DWORD) -> ::HRESULT
}
);
