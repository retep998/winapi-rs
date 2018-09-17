// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{CLSID, FMTID};
use shared::minwindef::{DWORD, FILETIME, PULONG, WORD};
use shared::wtypesbase::{LPOLESTR, ULONG};
use shared::wtypes::{PROPID, VARTYPE};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
UNION!{union PROPSPEC_u {
    [ULONG; 1],
    propid propid_mut: PROPID,
    lpwstr lpwstr_mut: LPOLESTR,
}}
STRUCT!{struct PROPSPEC {
    ulKind: ULONG,
    u: PROPSPEC_u,
}}
pub type REFPROPSPEC = *const PROPSPEC;
STRUCT!{struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: [u8; 16],
}}
pub type REFPROPVARIANT = *const PROPVARIANT;
STRUCT!{struct STATPROPSETSTG {
    fmtid: FMTID,
    clsid: CLSID,
    grfFlags: DWORD,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    dwOSVersion: DWORD,
}}
STRUCT!{struct STATPROPSTG {
    lpwstrName: LPOLESTR,
    propid: PROPID,
    vt: VARTYPE,
}}
DEFINE_GUID!{IID_IEnumSTATPROPSTG,
    0x00000139, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000139, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATPROPSTG(IEnumSTATPROPSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        ppIWiaItem: *mut STATPROPSTG,
        pceltFetched: PULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATPROPSTG,
    ) -> HRESULT,
}}
