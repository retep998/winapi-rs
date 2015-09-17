// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OLECMD {
    pub cmdID: ::ULONG,
    pub cmdf: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OLECMDTEXT {
    pub cmdtextf: ::DWORD,
    pub cwActual: ::ULONG,
    pub cwBuf: ::ULONG,
    pub rgwz: [::wchar_t; 0],
}
RIDL!{interface IOleCommandTarget(IOleCommandTargetVtbl): IUnknown(IUnknownVtbl) {
    fn QueryStatus(
        &mut self, pguidCmdGroup: *const ::GUID, cCmds: ::ULONG, prgCmds: *mut OLECMD,
        pCmdText: *mut OLECMDTEXT
    ) -> ::HRESULT,
    fn Exec(
        &mut self, pguidCmdGroup: *const :: GUID, nCmdID: ::DWORD, nCmdexecopt: ::DWORD,
        pvaIn: *mut ::VARIANT, pvaOut: *mut ::VARIANT
    ) -> ::HRESULT
}}
