// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use ctypes::{wchar_t};
use shared::guiddef::{GUID};
use shared::minwindef::{DWORD, ULONG};
use um::oaidl::{VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT};

STRUCT!{struct OLECMD {
    cmdID: ULONG,
    cmdf: DWORD,
}}
STRUCT!{struct OLECMDTEXT {
    cmdtextf: DWORD,
    cwActual: ULONG,
    cwBuf: ULONG,
    rgwz: [wchar_t; 0],
}}
RIDL!{interface IOleCommandTarget(IOleCommandTargetVtbl): IUnknown(IUnknownVtbl) {
    fn QueryStatus(
        pguidCmdGroup: *const GUID, cCmds: ULONG, prgCmds: *mut OLECMD,
        pCmdText: *mut OLECMDTEXT
    ) -> HRESULT,
    fn Exec(
        pguidCmdGroup: *const  GUID, nCmdID: DWORD, nCmdexecopt: DWORD,
        pvaIn: *mut VARIANT, pvaOut: *mut VARIANT
    ) -> HRESULT
}}
