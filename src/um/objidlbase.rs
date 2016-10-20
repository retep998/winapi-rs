// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::basetsd::SIZE_T;
use shared::guiddef::CLSID;
use shared::minwindef::{ DWORD, FILETIME, ULONG };
use shared::wtypesbase::{ LPOLESTR };
use um::unknwnbase::{ IUnknown, IUnknownVtbl };
use um::winnt::{ HRESULT, LARGE_INTEGER, ULARGE_INTEGER };
RIDL!{interface IMalloc(IMallocVtbl): IUnknown(IUnknownVtbl) {
    fn Alloc(&mut self, cb: SIZE_T) -> *mut c_void,
    fn Realloc(&mut self, pv: *mut c_void, cb: SIZE_T) -> *mut c_void,
    fn Free(&mut self, pv: *mut c_void) -> (),
    fn GetSize(&mut self, pv: *mut c_void) -> SIZE_T,
    fn DidAlloc(&mut self, pv: *mut c_void) -> c_int,
    fn HeapMinimize(&mut self) -> ()
}}
pub type LPMALLOC = *mut IMalloc;
STRUCT!{struct STATSTG {
    pwcsName: LPOLESTR,
    type_: DWORD,
    cbSize: ULARGE_INTEGER,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    grfMode: DWORD,
    grfLocksSupported: DWORD,
    clsid: CLSID,
    grfStateBits: DWORD,
    reserved: DWORD,
}}
//1945
pub type IEnumString = IUnknown; // TODO
//2075
RIDL!(
interface ISequentialStream(ISequentialStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Read(&mut self, pv: *mut c_void, cb: ULONG, pcbRead: *mut ULONG) -> HRESULT,
    fn Write(&mut self, pv: *const c_void, cb: ULONG, pcbWritten: *mut ULONG) -> HRESULT
}
);
ENUM!{enum STGTY {
    STGTY_STORAGE = 1,
    STGTY_STREAM = 2,
    STGTY_LOCKBYTES = 3,
    STGTY_PROPERTY = 4,
}}
ENUM!{enum STREAM_SEEK {
    STREAM_SEEK_SET = 0,
    STREAM_SEEK_CUR = 1,
    STREAM_SEEK_END = 2,
}}
ENUM!{enum LOCKTYPE {
    LOCK_WRITE = 1,
    LOCK_EXCLUSIVE = 2,
    LOCK_ONLYONCE = 4,
}}
//2255
RIDL!(
interface IStream(IStreamVtbl): ISequentialStream(ISequentialStreamVtbl) {
    fn Seek(
        &mut self, dlibMove: LARGE_INTEGER, dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER
    ) -> HRESULT,
    fn SetSize(&mut self, libNewSize: ULARGE_INTEGER) -> HRESULT,
    fn CopyTo(
        &mut self, pstm: *mut IStream, cb: ULARGE_INTEGER, pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER
    ) -> HRESULT,
    fn Commit(&mut self, grfCommitFlags: DWORD) -> HRESULT,
    fn Revert(&mut self) -> HRESULT,
    fn LockRegion(
        &mut self, libOffset: ULARGE_INTEGER, cb: ULARGE_INTEGER, dwLockType: DWORD
    ) -> HRESULT,
    fn UnlockRegion(
        &mut self, libOffset: ULARGE_INTEGER, cb: ULARGE_INTEGER, dwLockType: DWORD
    ) -> HRESULT,
    fn Stat(&mut self, pstatstg: *mut STATSTG, grfStatFlag: DWORD) -> HRESULT,
    fn Clone(&mut self, ppstm: *mut *mut IStream) -> HRESULT
}
);
pub type LPSTREAM = *mut IStream;
ENUM!{enum APTTYPEQUALIFIER {
    APTTYPEQUALIFIER_NONE = 0,
    APTTYPEQUALIFIER_IMPLICIT_MTA = 1,
    APTTYPEQUALIFIER_NA_ON_MTA = 2,
    APTTYPEQUALIFIER_NA_ON_STA = 3,
    APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA = 4,
    APTTYPEQUALIFIER_NA_ON_MAINSTA = 5,
    APTTYPEQUALIFIER_APPLICATION_STA= 6,
}}
ENUM!{enum APTTYPE {
    APTTYPE_CURRENT = -1i32 as u32,
    APTTYPE_STA = 0,
    APTTYPE_MTA = 1,
    APTTYPE_NA = 2,
    APTTYPE_MAINSTA = 3,
}}
