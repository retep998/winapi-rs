// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
//8402
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BIND_OPTS {
    pub cbStruct: ::DWORD,
    pub grfFlags: ::DWORD,
    pub grfMode: ::DWORD,
    pub dwTickCountDeadline: ::DWORD,
}
pub type LPBIND_OPTS = *mut BIND_OPTS;
//8479
RIDL!(
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(&mut self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn RevokeObjectBound(&mut self, punk: *mut ::IUnknown) -> ::HRESULT,
    fn ReleaseBoundObjects(&mut self) -> ::HRESULT,
    fn SetBindOptions(&mut self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetBindOptions(&mut self, pbindopts: *mut BIND_OPTS) -> ::HRESULT,
    fn GetRunningObjectTable(&mut self, pprot: *mut *mut IRunningObjectTable) -> ::HRESULT,
    fn RegisterObjectParam(&mut self, pszKey: ::LPOLESTR, punk: *mut ::IUnknown) -> ::HRESULT,
    fn GetObjectParam(&mut self, pszKey: ::LPOLESTR, ppunk: *mut *mut ::IUnknown) -> ::HRESULT,
    fn EnumObjectParam(&mut self, ppenum: *mut *mut ::IEnumString) -> ::HRESULT,
    fn RevokeObjectParam(&mut self, pszKey: ::LPOLESTR) -> ::HRESULT
}
);
//8681
pub type IEnumMoniker = ::IUnknown; // TODO
//8958
RIDL!(
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        &mut self, grfFlags: ::DWORD, punkObject: *mut ::IUnknown, pmkObjectName: *mut IMoniker,
        pdwRegister: *mut ::DWORD
    ) -> ::HRESULT,
    fn Revoke(&mut self, dwRegister: ::DWORD) -> ::HRESULT,
    fn IsRunning(&mut self, pmkObjectName: *mut IMoniker) -> ::HRESULT,
    fn GetObject(
        &mut self, pmkObjectName: *mut IMoniker, ppunkObject: *mut *mut ::IUnknown
    ) -> ::HRESULT,
    fn NoteChangeTime(&mut self, dwRegister: ::DWORD, pfiletime: *mut ::FILETIME) -> ::HRESULT,
    fn GetTimeOfLastChange(
        &mut self, pmkObjectName: *mut IMoniker, pfiletime: *mut ::FILETIME
    ) -> ::HRESULT,
    fn EnumRunning(&mut self, ppenumMoniker: *mut *mut IEnumMoniker) -> ::HRESULT
}
);
//9350
pub type IMoniker = ::IUnknown; // TODO
