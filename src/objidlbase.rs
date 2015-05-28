// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! this ALWAYS GENERATED file contains the definitions for the interfaces
RIDL!(
interface IMalloc(IMallocVtbl): IUnknown(IUnknownVtbl) {
    fn Alloc(&mut self, cb: ::SIZE_T) -> *mut ::c_void,
    fn Realloc(&mut self, pv: *mut ::c_void, cb: ::SIZE_T) -> *mut ::c_void,
    fn Free(&mut self, pv: *mut ::c_void) -> (),
    fn GetSize(&mut self, pv: *mut ::c_void) -> ::SIZE_T,
    fn DidAlloc(&mut self, pv: *mut ::c_void) -> ::c_int,
    fn HeapMinimize(&mut self) -> ()
}
);
pub type LPMALLOC = *mut IMalloc;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STATSTG {
    pub pwcsName: ::LPOLESTR,
    pub type_: ::DWORD,
    pub cbSize: ::ULARGE_INTEGER,
    pub mtime: ::FILETIME,
    pub ctime: ::FILETIME,
    pub atime: ::FILETIME,
    pub grfMode: ::DWORD,
    pub grfLocksSupported: ::DWORD,
    pub clsid: ::CLSID,
    pub grfStateBits: ::DWORD,
    pub reserved: ::DWORD,
}
//1945
pub type IEnumString = ::IUnknown; // TODO
//2075
RIDL!(
interface ISequentialStream(ISequentialStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Read(&mut self, pv: *mut ::c_void, cb: ::ULONG, pcbRead: *mut ::ULONG) -> ::HRESULT,
    fn Write(&mut self, pv: *const ::c_void, cb: ::ULONG, pcbWritten: *mut ::ULONG) -> ::HRESULT
}
);
//2255
RIDL!(
interface IStream(IStreamVtbl): ISequentialStream(ISequentialStreamVtbl) {
    fn Seek(
        &mut self, dlibMove: ::LARGE_INTEGER, dwOrigin: ::DWORD,
        plibNewPosition: *mut ::ULARGE_INTEGER
    ) -> ::HRESULT,
    fn SetSize(&mut self, libNewSize: ::ULARGE_INTEGER) -> ::HRESULT,
    fn CopyTo(
        &mut self, pstm: *mut IStream, cb: ::ULARGE_INTEGER, pcbRead: *mut ::ULARGE_INTEGER,
        pcbWritten: *mut ::ULARGE_INTEGER
    ) -> ::HRESULT,
    fn Commit(&mut self, grfCommitFlags: ::DWORD) -> ::HRESULT,
    fn Revert(&mut self) -> ::HRESULT,
    fn LockRegion(
        &mut self, libOffset: ::ULARGE_INTEGER, cb: ::ULARGE_INTEGER, dwLockType: ::DWORD
    ) -> ::HRESULT,
    fn UnlockRegion(
        &mut self, libOffset: ::ULARGE_INTEGER, cb: ::ULARGE_INTEGER, dwLockType: ::DWORD
    ) -> ::HRESULT,
    fn Stat(&mut self, pstatstg: *mut STATSTG, grfStatFlag: ::DWORD) -> ::HRESULT,
    fn Clone(&mut self, ppstm: *mut *mut IStream) -> ::HRESULT
}
);
pub type LPSTREAM = *mut IStream;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum APTTYPEQUALIFIER {
    APTTYPEQUALIFIER_NONE = 0,
    APTTYPEQUALIFIER_IMPLICIT_MTA = 1,
    APTTYPEQUALIFIER_NA_ON_MTA = 2,
    APTTYPEQUALIFIER_NA_ON_STA = 3,
    APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA = 4,
    APTTYPEQUALIFIER_NA_ON_MAINSTA = 5,
    APTTYPEQUALIFIER_APPLICATION_STA= 6,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub enum APTTYPE {
    APTTYPE_CURRENT = -1,
    APTTYPE_STA = 0,
    APTTYPE_MTA = 1,
    APTTYPE_NA = 2,
    APTTYPE_MAINSTA = 3,
}
