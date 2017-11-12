// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use ctypes::{c_int, c_uchar, c_ulong, c_void};
use shared::basetsd::{SIZE_T, UINT64};
use shared::guiddef::{CLSID, IID, LPCLSID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, HGLOBAL, HTASK, ULONG, WORD};
use shared::rpcndr::byte;
use shared::windef::{HBITMAP, HDC, HENHMETAFILE, HICON, HWND};
use shared::wtypes::{
    CLIPFORMAT, HMETAFILEPICT, wireHBITMAP, wireHENHMETAFILE, wireHGLOBAL, wireHMETAFILEPICT,
    wireHPALETTE,
};
use shared::wtypesbase::{BYTE_BLOB, LPCOLESTR, LPOLESTR, OLECHAR};
use um::objidlbase::{COSERVERINFO, IEnumString, IStream, STATSTG};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LARGE_INTEGER, LCID, LONG, LPCWSTR, ULARGE_INTEGER};
pub type LPMALLOCSPY = *mut IMallocSpy;
RIDL!(
#[uuid(0x0000001d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMallocSpy(IMallocSpyVtbl): IUnknown(IUnknownVtbl) {
    fn PreAlloc(
        cbRequest: SIZE_T,
    ) -> SIZE_T,
    fn PostAlloc(
        pActual: *mut c_void,
    ) -> *mut c_void,
    fn PreFree(
        pRequest: *mut c_void,
        fSpyed: BOOL,
    ) -> *mut c_void,
    fn PostFree(
        fSpyed: BOOL,
    ) -> c_void,
    fn PreRealloc(
        pRequest: *mut c_void,
        cbRequest: SIZE_T,
        ppNewRequest: *mut *mut c_void,
        fSpyed: BOOL,
    ) -> SIZE_T,
    fn PostRealloc(
        pActual: *mut c_void,
        fSpyed: BOOL,
    ) -> *mut c_void,
    fn PreGetSize(
        pRequest: *mut c_void,
        fSpyed: BOOL,
    ) -> *mut c_void,
    fn PostGetSize(
        cbActual: SIZE_T,
        fSpyed: BOOL,
    ) -> SIZE_T,
    fn PreDidAlloc(
        pRequest: *mut c_void,
        fSpyed: BOOL,
    ) -> *mut c_void,
    fn PostDidAlloc(
        pRequest: *mut c_void,
        fSpyed: BOOL,
        fActual: c_int,
    ) -> c_int,
    fn PreHeapMinimize() -> (),
    fn PostHeapMinimize() -> (),
});
pub type LPBC = *mut IBindCtx;
pub type LPBINDCTX = *mut IBindCtx;
STRUCT!{struct BIND_OPTS {
    cbStruct: DWORD,
    grfFlags: DWORD,
    grfMode: DWORD,
    dwTickCountDeadline: DWORD,
}}
pub type LPBIND_OPTS = *mut BIND_OPTS;
STRUCT!{struct BIND_OPTS2 {
    cbStruct: DWORD,
    grfFlags: DWORD,
    grfMode: DWORD,
    dwTickCountDeadline: DWORD,
    dwTrackFlags: DWORD,
    dwClassContext: DWORD,
    locale: LCID,
    pServerInfo: *mut COSERVERINFO,
}}
pub type LPBIND_OPTS2 = *mut BIND_OPTS2;
STRUCT!{struct BIND_OPTS3 {
    cbStruct: DWORD,
    grfFlags: DWORD,
    grfMode: DWORD,
    dwTickCountDeadline: DWORD,
    dwTrackFlags: DWORD,
    dwClassContext: DWORD,
    locale: LCID,
    pServerInfo: *mut COSERVERINFO,
    hwnd: HWND,
}}
pub type LPBIND_OPTS3 = *mut BIND_OPTS3;
ENUM!{enum BIND_FLAGS {
    BIND_MAYBOTHERUSER = 1,
    BIND_JUSTTESTEXISTENCE = 2,
}}
RIDL!(
#[uuid(0x0000000e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn RevokeObjectBound(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn ReleaseBoundObjects() -> HRESULT,
    fn SetBindOptions(
        pbindopts: *mut BIND_OPTS,
    ) -> HRESULT,
    fn GetBindOptions(
        pbindopts: *mut BIND_OPTS,
    ) -> HRESULT,
    fn GetRunningObjectTable(
        pprot: *mut *mut IRunningObjectTable,
    ) -> HRESULT,
    fn RegisterObjectParam(
        pszKey: LPOLESTR,
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn GetObjectParam(
        pszKey: LPOLESTR,
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn EnumObjectParam(
        ppenum: *mut *mut IEnumString,
    ) -> HRESULT,
    fn RevokeObjectParam(
        pszKey: LPOLESTR,
    ) -> HRESULT,
});
pub type LPENUMMONIKER = *mut IEnumMoniker;
RIDL!(
#[uuid(0x00000102, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumMoniker(IEnumMonikerVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IMoniker,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumMoniker,
    ) -> HRESULT,
});
pub type LPRUNNABLEOBJECT = *mut IRunnableObject;
RIDL!(
#[uuid(0x00000126, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRunnableObject(IRunnableObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetRunningClass(
        lpClsid: LPCLSID,
    ) -> HRESULT,
    fn Run(
        pbc: LPBINDCTX,
    ) -> HRESULT,
    fn IsRunning() -> BOOL,
    fn LockRunning(
        fLock: BOOL,
        fLastUnlockCloses: BOOL,
    ) -> HRESULT,
    fn SetContainedObject(
        fContained: BOOL,
    ) -> HRESULT,
});
pub type LPRUNNINGOBJECTTABLE = *mut IRunningObjectTable;
RIDL!(
#[uuid(0x00000010, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        grfFlags: DWORD,
        punkObject: *mut IUnknown,
        pmkObjectName: *mut IMoniker,
        pdwRegister: *mut DWORD,
    ) -> HRESULT,
    fn Revoke(
        dwRegister: DWORD,
    ) -> HRESULT,
    fn IsRunning(
        pmkObjectName: *mut IMoniker,
    ) -> HRESULT,
    fn GetObject(
        pmkObjectName: *mut IMoniker,
        ppunkObject: *mut *mut IUnknown,
    ) -> HRESULT,
    fn NoteChangeTime(
        dwRegister: DWORD,
        pfiletime: *mut FILETIME,
    ) -> HRESULT,
    fn GetTimeOfLastChange(
        pmkObjectName: *mut IMoniker,
        pfiletime: *mut FILETIME,
    ) -> HRESULT,
    fn EnumRunning(
        ppenumMoniker: *mut *mut IEnumMoniker,
    ) -> HRESULT,
}
);
pub type LPPERSIST = *mut IPersist;
RIDL!(
#[uuid(0x0000010c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID(
        pClassID: *mut CLSID,
    ) -> HRESULT,
}
);
pub type LPPERSISTSTREAM = *mut IPersistStream;
RIDL!(
#[uuid(0x00000109, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistStream(IPersistStreamVtbl): IPersist(IPersistVtbl) {
    fn IsDirty() -> HRESULT,
    fn Load(
        pStm: *mut IStream,
    ) -> HRESULT,
    fn Save(
        pStm: *mut IStream,
        fClearDirty: BOOL,
    ) -> HRESULT,
    fn GetSizeMax(
        pcbSize: *mut ULARGE_INTEGER,
    ) -> HRESULT,
}
);
pub type LPMONIKER = *mut IMoniker;
ENUM!{enum MKSYS {
    MKSYS_NONE = 0,
    MKSYS_GENERICCOMPOSITE = 1,
    MKSYS_FILEMONIKER = 2,
    MKSYS_ANTIMONIKER = 3,
    MKSYS_ITEMMONIKER = 4,
    MKSYS_POINTERMONIKER = 5,
    MKSYS_CLASSMONIKER = 7,
    MKSYS_OBJREFMONIKER = 8,
    MKSYS_SESSIONMONIKER = 9,
    MKSYS_LUAMONIKER = 10,
}}
ENUM!{enum MKRREDUCE {
    MKRREDUCE_ONE = 3 << 16,
    MKRREDUCE_TOUSER = 2 << 16,
    MKRREDUCE_THROUGHUSER = 1 << 16,
    MKRREDUCE_ALL = 0,
}}
RIDL!(
#[uuid(0x0000000f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMoniker(IMonikerVtbl): IPersistStream(IPersistStreamVtbl) {
    fn BindToObject(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        riidResult: REFIID,
        ppvResult: *mut *mut c_void,
    ) -> HRESULT,
    fn BindToStorage(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        riid: REFIID,
        ppvObj: *mut *mut c_void,
    ) -> HRESULT,
    fn Reduce(
        pbc: *mut IBindCtx,
        dwReduceHowFar: DWORD,
        ppmkToLeft: *mut *mut IMoniker,
        ppmkReduced: *mut *mut IMoniker,
    ) -> HRESULT,
    fn ComposeWith(
        pmkRight: *mut IMoniker,
        fOnlyIfNotGeneric: BOOL,
        ppmkComposite: *mut *mut IMoniker,
    ) -> HRESULT,
    fn Enum(
        fForward: BOOL,
        ppenumMoniker: *mut *mut IEnumMoniker,
    ) -> HRESULT,
    fn IsEqual(
        pmkOtherMoniker: *mut IMoniker,
    ) -> HRESULT,
    fn Hash(
        pdwHash: *mut DWORD,
    ) -> HRESULT,
    fn IsRunning(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pmkNewlyRunning: *mut IMoniker,
    ) -> HRESULT,
    fn GetTimeOfLastChange(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pFileTime: *mut FILETIME,
    ) -> HRESULT,
    fn Inverse(
        ppmk: *mut *mut IMoniker,
    ) -> HRESULT,
    fn CommonPrefixWith(
        pmkOther: *mut IMoniker,
        ppmkPrefix: *mut *mut IMoniker,
    ) -> HRESULT,
    fn RelativePathTo(
        pmkOther: *mut IMoniker,
        ppmkRelPath: *mut *mut IMoniker,
    ) -> HRESULT,
    fn GetDisplayName(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        ppszDisplayName: *mut LPOLESTR,
    ) -> HRESULT,
    fn ParseDisplayName(
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pszDisplayName: LPOLESTR,
        pchEaten: *mut ULONG,
        ppmkOut: *mut *mut IMoniker,
    ) -> HRESULT,
    fn IsSystemMoniker(
        pdwMksys: *mut DWORD,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0xf29f6bc0, 0x5021, 0x11ce, 0xaa, 0x15, 0x00, 0x00, 0x69, 0x01, 0x29, 0x3f)]
interface IROTData(IROTDataVtbl): IUnknown(IUnknownVtbl) {
    fn GetComparisonData(
        pbData: *mut byte,
        cbMax: ULONG,
        pcbData: *mut ULONG,
    ) -> HRESULT,
}
);
pub type LPENUMSTATSTG = *mut IEnumSTATSTG;
RIDL!(
#[uuid(0x0000000d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATSTG(IEnumSTATSTGVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATSTG,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATSTG,
    ) -> HRESULT,
}
);
pub type LPSTORAGE = *mut IStorage;
STRUCT!{struct RemSNB {
    ulCntStr: ULONG,
    ulCntChar: ULONG,
    rgString: [OLECHAR; 1],
}}
pub type wireSNB = *mut RemSNB;
pub type SNB = *mut LPOLESTR;
RIDL!(
#[uuid(0x0000000b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IStorage(IStorageVtbl): IUnknown(IUnknownVtbl) {
    fn CreateStream(
        pwcsName: *const OLECHAR,
        grfMode: DWORD,
        reserved1: DWORD,
        reserved2: DWORD,
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
    fn OpenStream(
        pwcsName: *const OLECHAR,
        reserved1: *mut c_void,
        grfMode: DWORD,
        reserved2: DWORD,
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
    fn CreateStorage(
        pwcsName: *const OLECHAR,
        grfMode: DWORD,
        reserved1: DWORD,
        reserved2: DWORD,
        ppstg: *mut *mut IStorage,
    ) -> HRESULT,
    fn OpenStorage(
        pwcsName: *const OLECHAR,
        pstgPriority: *mut IStorage,
        grfMode: DWORD,
        snbExclude: SNB,
        reserved: DWORD,
        ppstg: *mut *mut IStorage,
    ) -> HRESULT,
    fn CopyTo(
        ciidExclude: DWORD,
        rgiidExclude: *const IID,
        snbExclude: SNB,
        ppstg: *mut IStorage,
    ) -> HRESULT,
    fn MoveElementTo(
        pwcsName: *const OLECHAR,
        pstgDest: *mut IStorage,
        pwcsNewName: *const OLECHAR,
        grfMode: DWORD,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn EnumElements(
        reserved1: DWORD,
        reserved2: *mut c_void,
        reserved3: DWORD,
        ppenum: *mut *mut IEnumSTATSTG,
    ) -> HRESULT,
    fn DestroyElement(
        pwcsName: *const OLECHAR,
    ) -> HRESULT,
    fn RenameElement(
        pwcsOldName: *const OLECHAR,
        pwcsNewName: *const OLECHAR,
    ) -> HRESULT,
    fn SetElementTimes(
        pwcsName: *const OLECHAR,
        pctime: *const FILETIME,
        patime: *const FILETIME,
        pmtime: *const FILETIME,
    ) -> HRESULT,
    fn SetClass(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn SetStateBits(
        grfStateBits: DWORD,
        grfMask: DWORD,
    ) -> HRESULT,
    fn Stat(
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
}
);
pub type LPPERSISTFILE = *mut IPersistFile;
RIDL!(
#[uuid(0x0000010b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistFile(IPersistFileVtbl): IPersist(IPersistVtbl) {
    fn IsDirty() -> HRESULT,
    fn Load(
        pszFileName: LPCOLESTR,
        dwMode: DWORD,
    ) -> HRESULT,
    fn Save(
        pszFileName: LPCOLESTR,
        fRemember: BOOL,
    ) -> HRESULT,
    fn SaveCompleted(
        pszFileName: LPCOLESTR,
    ) -> HRESULT,
    fn GetCurFile(
        ppszFileName: *mut LPOLESTR,
    ) -> HRESULT,
}
);
pub type LPPERSISTSTORAGE = *mut IPersistStorage;
RIDL!(
#[uuid(0x0000010a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistStorage(IPersistStorageVtbl): IPersist(IPersistVtbl) {
    fn IsDirty() -> HRESULT,
    fn InitNew(
        pStg: *mut IStorage,
    ) -> HRESULT,
    fn Load(
        pStg: *mut IStorage,
    ) -> HRESULT,
    fn Save(
        pStgSave: *mut IStorage,
        fSameAsLoad: BOOL,
    ) -> HRESULT,
    fn SaveCompleted(
        pStgNew: *mut IStorage,
    ) -> HRESULT,
    fn HandsOffStorage() -> HRESULT,
}
);
pub type LPLOCKBYTES = *mut ILockBytes;
RIDL!(
#[uuid(0x0000000a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ILockBytes(ILockBytesVtbl): IUnknown(IUnknownVtbl) {
    fn ReadAt(
        ulOffset: ULARGE_INTEGER,
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn WriteAt(
        ulOffset: ULARGE_INTEGER,
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn Flush() -> HRESULT,
    fn SetSize(
        cb: ULARGE_INTEGER,
    ) -> HRESULT,
    fn LockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn UnlockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn Stat(
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
}
);
pub type LPENUMFORMATETC = *mut IEnumFORMATETC;
STRUCT!{struct DVTARGETDEVICE {
    tdSize: DWORD,
    tdDriverNameOffset: WORD,
    tdDeviceNameOffset: WORD,
    tdPortNameOffset: WORD,
    tdExtDevmodeOffset: WORD,
    tdData: [BYTE; 1],
}}
pub type LPCLIPFORMAT = *mut CLIPFORMAT;
STRUCT!{struct FORMATETC {
    cfFormat: CLIPFORMAT,
    ptd: *mut DVTARGETDEVICE,
    dwAspect: DWORD,
    lindex: LONG,
    tymed: DWORD,
}}
pub type LPFORMATETC = *mut FORMATETC;
RIDL!(
#[uuid(0x00000103, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumFORMATETC(IEnumFORMATETCVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut FORMATETC,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumFORMATETC,
    ) -> HRESULT,
}
);
pub type LPENUMSTATDATA = *mut IEnumSTATDATA;
ENUM!{enum ADVF {
    ADVF_NODATA = 1,
    ADVF_PRIMEFIRST = 2,
    ADVF_ONLYONCE = 4,
    ADVF_DATAONSTOP = 64,
    ADVFCACHE_NOHANDLER = 8,
    ADVFCACHE_FORCEBUILTIN = 16,
    ADVFCACHE_ONSAVE = 32,
}}
STRUCT!{struct STATDATA {
    formatetc: FORMATETC,
    advf: DWORD,
    pAdvSink: *mut IAdviseSink,
    dwConnection: DWORD,
}}
pub type LPSTATDATA = *mut STATDATA;
RIDL!(
#[uuid(0x00000105, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumSTATDATA(IEnumSTATDATAVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut STATDATA,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
}
);
pub type LPROOTSTORAGE = *mut IRootStorage;
RIDL!(
#[uuid(0x00000012, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRootStorage(IRootStorageVtbl): IUnknown(IUnknownVtbl) {
    fn SwitchToFile(
        pszFile: LPOLESTR,
    ) -> HRESULT,
}
);
pub type LPADVISESINK = *mut IAdviseSink;
ENUM!{enum TYMED {
    TYMED_HGLOBAL = 1,
    TYMED_FILE = 2,
    TYMED_ISTREAM = 4,
    TYMED_ISTORAGE = 8,
    TYMED_GDI = 16,
    TYMED_MFPICT = 32,
    TYMED_ENHMF = 64,
    TYMED_NULL = 0,
}}
STRUCT!{struct RemSTGMEDIUM {
    tymed: DWORD,
    dwHandleType: DWORD,
    pData: ULONG,
    pUnkForRelease: ULONG,
    cbData: ULONG,
    data: [byte; 1],
}}
UNION!{union uSTGMEDIUM_u {
    [usize; 1],
    hBitmap hBitmap_mut: HBITMAP,
    hMetaFilePict hMetaFilePict_mut: HMETAFILEPICT,
    hEnhMetaFile hEnhMetaFile_mut: HENHMETAFILE,
    hGlobal hGlobal_mut: HGLOBAL,
    lpszFileName lpszFileName_mut: LPOLESTR,
    pstm pstm_mut: *mut IStream,
    pstg pstg_mut: *mut IStorage,
}}
STRUCT!{struct uSTGMEDIUM {
    tymed: DWORD,
    u: uSTGMEDIUM_u,
    pUnkForRelease: *mut IUnknown,
}}
UNION!{union GDI_OBJECT_u {
    [usize; 1],
    hBitmap hBitmap_mut: wireHBITMAP,
    hPalette hPalette_mut: wireHPALETTE,
    hGeneric hGeneric_mut: wireHGLOBAL,
}}
STRUCT!{struct GDI_OBJECT {
    ObjectType: DWORD,
    u: GDI_OBJECT_u,
}}
UNION!{union userSTGMEDIUM_u {
    [usize; 1],
    hMetaFilePict hMetaFilePict_mut: wireHMETAFILEPICT,
    hHEnhMetaFile hHEnhMetaFile_mut: wireHENHMETAFILE,
    hGdiHandle hGdiHandle_mut: *mut GDI_OBJECT,
    hGlobal hGlobal_mut: wireHGLOBAL,
    lpszFileName lpszFileName_mut: LPOLESTR,
    pstm pstm_mut: *mut BYTE_BLOB,
    pstg pstg_mut: *mut BYTE_BLOB,
}}
STRUCT!{struct userSTGMEDIUM {
    tymed: DWORD,
    u: userSTGMEDIUM_u,
    pUnkForRelease: *mut IUnknown,
}}
pub type wireSTGMEDIUM = *mut userSTGMEDIUM;
pub type STGMEDIUM = uSTGMEDIUM;
pub type wireASYNC_STGMEDIUM = *mut userSTGMEDIUM;
pub type ASYNC_STGMEDIUM = STGMEDIUM;
pub type LPSTGMEDIUM = *mut STGMEDIUM;
STRUCT!{struct userFLAG_STGMEDIUM {
    ContextFlags: LONG,
    fPassOwnership: LONG,
    Stgmed: userSTGMEDIUM,
}}
pub type wireFLAG_STGMEDIUM = *mut userFLAG_STGMEDIUM;
STRUCT!{struct FLAG_STGMEDIUM {
    ContextFlags: LONG,
    fPassOwnership: LONG,
    Stgmed: STGMEDIUM,
}}
RIDL!(
#[uuid(0x0000010f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAdviseSink(IAdviseSinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnDataChange(
        pFormatetc: *mut FORMATETC,
        pStgmed: *mut STGMEDIUM,
    ) -> (),
    fn OnViewChange(
        dwAspect: DWORD,
        lindex: LONG,
    ) -> (),
    fn OnRename(
        pmk: *mut IMoniker,
    ) -> (),
    fn OnSave() -> (),
    fn OnClose() -> (),
}
);
RIDL!(
#[uuid(0x00000150, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIAdviseSink(AsyncIAdviseSinkVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_OnDataChange(
        pFormatetc: *mut FORMATETC,
        pStgmed: *mut STGMEDIUM,
    ) -> (),
    fn Finish_OnDataChange() -> (),
    fn Begin_OnViewChange(
        dwAspect: DWORD,
        lindex: LONG,
    ) -> (),
    fn Finish_OnViewChange() -> (),
    fn Begin_OnRename(
        pmk: *mut IMoniker,
    ) -> (),
    fn Finish_OnRename() -> (),
    fn Begin_OnSave() -> (),
    fn Finish_OnSave() -> (),
    fn Begin_OnClose() -> (),
    fn Finish_OnClose() -> (),
}
);
pub type LPADVISESINK2 = *mut IAdviseSink2;
RIDL!(
#[uuid(0x00000125, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAdviseSink2(IAdviseSink2Vtbl): IAdviseSink(IAdviseSinkVtbl) {
    fn OnLinkSrcChange(
        pmk: *mut IMoniker,
    ) -> (),
}
);
RIDL!(
#[uuid(0x00000151, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIAdviseSink2(AsyncIAdviseSink2Vtbl): AsyncIAdviseSink(AsyncIAdviseSinkVtbl) {
    fn Begin_OnLinkSrcChange(
        pmk: *mut IMoniker,
    ) -> (),
    fn Finish_OnLinkSrcChange() -> (),
}
);
pub type LPDATAOBJECT = *mut IDataObject;
ENUM!{enum DATADIR {
    DATADIR_GET = 1,
    DATADIR_SET = 2,
}}
RIDL!(
#[uuid(0x0000010e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDataObject(IDataObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetData(
        pformatetcIn: *mut FORMATETC,
        pmedium: *mut STGMEDIUM,
    ) -> HRESULT,
    fn GetDataHere(
        pformatetc: *mut FORMATETC,
        pmedium: *mut STGMEDIUM,
    ) -> HRESULT,
    fn QueryGetData(
        pformatetc: *mut FORMATETC,
    ) -> HRESULT,
    fn GetCanonicalFormatEtc(
        pformatetcIn: *mut FORMATETC,
        pformatetc: *mut FORMATETC,
    ) -> HRESULT,
    fn SetData(
        pformatetc: *mut FORMATETC,
        pmedium: *mut STGMEDIUM,
        fRelease: BOOL,
    ) -> HRESULT,
    fn EnumFormatEtc(
        dwDirection: DWORD,
        ppenumFormatEtc: *mut *mut IEnumFORMATETC,
    ) -> HRESULT,
    fn DAdvise(
        pformatetc: *mut FORMATETC,
        advf: DWORD,
        pAdvSink: *mut IAdviseSink,
        pdwConnection: *mut DWORD,
    ) -> HRESULT,
    fn DUnadvise(
        dwConnection: DWORD,
    ) -> HRESULT,
    fn EnumDAdvise(
        ppenumAdvise: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
}
);
pub type LPDATAADVISEHOLDER = *mut IDataAdviseHolder;
RIDL!(
#[uuid(0x00000110, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDataAdviseHolder(IDataAdviseHolderVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        pDataObject: *mut IDataObject,
        pFetc: *mut FORMATETC,
        advf: DWORD,
        pAdvise: *mut IAdviseSink,
        pdwConnection: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwConnection: DWORD,
    ) -> HRESULT,
    fn EnumAdvise(
        ppenumAdvise: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
    fn SendOnDataChange(
        pDataObject: *mut IDataObject,
        advf: DWORD,
        dwReserved: DWORD,
    ) -> HRESULT,
}
);
pub type LPMESSAGEFILTER = *mut IMessageFilter;
ENUM!{enum CALLTYPE {
    CALLTYPE_TOPLEVEL = 1,
    CALLTYPE_NESTED = 2,
    CALLTYPE_ASYNC = 3,
    CALLTYPE_TOPLEVEL_CALLPENDING = 4,
    CALLTYPE_ASYNC_CALLPENDING = 5,
}}
ENUM!{enum SERVERCALL {
    SERVERCALL_ISHANDLED = 0,
    SERVERCALL_REJECTED = 1,
    SERVERCALL_RETRYLATER = 2,
}}
ENUM!{enum PENDINGTYPE {
    PENDINGTYPE_TOPLEVEL = 1,
    PENDINGTYPE_NESTED = 2,
}}
ENUM!{enum PENDINGMSG {
    PENDINGMSG_CANCELCALL = 0,
    PENDINGMSG_WAITNOPROCESS = 1,
    PENDINGMSG_WAITDEFPROCESS = 2,
}}
STRUCT!{struct INTERFACEINFO {
    pUnk: *mut IUnknown,
    iid: IID,
    wMethod: WORD,
}}
pub type LPINTERFACEINFO = *mut INTERFACEINFO;
RIDL!(
#[uuid(0x00000016, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMessageFilter(IMessageFilterVtbl): IUnknown(IUnknownVtbl) {
    fn HandleInComingCall(
        dwCallType: DWORD,
        htaskCaller: HTASK,
        dwTickCount: DWORD,
        lpInterfaceInfo: LPINTERFACEINFO,
    ) -> DWORD,
    fn RetryRejectedCall(
        htaskCallee: HTASK,
        dwTickCount: DWORD,
        dwRejectType: DWORD,
    ) -> DWORD,
    fn MessagePending(
        htaskCallee: HTASK,
        dwTickCount: DWORD,
        dwPendingType: DWORD,
    ) -> DWORD,
}
);
RIDL!(
#[uuid(0x00000140, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IClassActivator(IClassActivatorVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassObject(
        rclsid: REFCLSID,
        dwClassContext: DWORD,
        locale: LCID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x99caf010, 0x415e, 0x11cf, 0x88, 0x14, 0x00, 0xaa, 0x00, 0xb5, 0x69, 0xf5)]
interface IFillLockBytes(IFillLockBytesVtbl): IUnknown(IUnknownVtbl) {
    fn FillAppend(
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn FillAt(
        ulOffset: ULARGE_INTEGER,
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
    fn SetFillSize(
        ulSize: ULARGE_INTEGER,
    ) -> HRESULT,
    fn Terminate(
        bCanceled: BOOL,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0xa9d758a0, 0x4617, 0x11cf, 0x95, 0xfc, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
interface IProgressNotify(IProgressNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn OnProgress(
        dwProgressCurrent: DWORD,
        dwProgressMaximum: DWORD,
        fAccurate: BOOL,
        fOwner: BOOL,
    ) -> HRESULT,
}
);
STRUCT!{struct StorageLayout {
    LayoutType: DWORD,
    pwcsElementName: *mut OLECHAR,
    cOffset: LARGE_INTEGER,
    cBytes: LARGE_INTEGER,
}}
RIDL!(
#[uuid(0x0e6d4d90, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
interface ILayoutStorage(ILayoutStorageVtbl): IUnknown(IUnknownVtbl) {
    fn LayoutScript(
        pStorageLayout: *mut StorageLayout,
        nEntries: DWORD,
        glfInterleavedFlag: DWORD,
    ) -> HRESULT,
    fn BeginMonitor() -> HRESULT,
    fn EndMonitor() -> HRESULT,
    fn ReLayoutDocfile(
        pwcsNewDfName: *mut OLECHAR,
    ) -> HRESULT,
    fn ReLayoutDocfileOnILockBytes(
        pILockBytes: *mut ILockBytes,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x30f3d47a, 0x6447, 0x11d1, 0x8e, 0x3c, 0x00, 0xc0, 0x4f, 0xb9, 0x38, 0x6d)]
interface IBlockingLock(IBlockingLockVtbl): IUnknown(IUnknownVtbl) {
    fn Lock(
        dwTimeout: DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
}
);
RIDL!(
#[uuid(0xbc0bf6ae, 0x8878, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4)]
interface ITimeAndNoticeControl(ITimeAndNoticeControlVtbl): IUnknown(IUnknownVtbl) {
    fn SuppressChanges(
        res1: DWORD,
        res2: DWORD,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x8d19c834, 0x8879, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4)]
interface IOplockStorage(IOplockStorageVtbl): IUnknown(IUnknownVtbl) {
    fn CreateStorageEx(
        pwcsName: LPCWSTR,
        grfMode: DWORD,
        stgfmt: DWORD,
        grfAttrs: DWORD,
        riid: REFIID,
        ppstgOpen: *mut *mut c_void,
    ) -> HRESULT,
    fn OpenStorageEx(
        pwcsName: LPCWSTR,
        grfMode: DWORD,
        stgfmt: DWORD,
        grfAttrs: DWORD,
        riid: REFIID,
        ppstgOpen: *mut *mut c_void,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x0e6d4d92, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
interface IDirectWriterLock(IDirectWriterLockVtbl): IUnknown(IUnknownVtbl) {
    fn WaitForWriteAccess(
        dwTimeout: DWORD,
    ) -> HRESULT,
    fn ReleaseWriteAccess() -> HRESULT,
    fn HaveWriteAccess() -> HRESULT,
}
);
RIDL!(
#[uuid(0x00000026, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUrlMon(IUrlMonVtbl): IUnknown(IUnknownVtbl) {
    fn AsyncGetClassBits(
        rclsid: REFCLSID,
        pszTYPE: LPCWSTR,
        pszExt: LPCWSTR,
        dwFileVersionMS: DWORD,
        dwFileVersionLS: DWORD,
        pszCodeBase: LPCWSTR,
        pbc: *mut IBindCtx,
        dwClassContext: DWORD,
        riid: REFIID,
        flags: DWORD,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x00000145, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IForegroundTransfer(IForegroundTransferVtbl): IUnknown(IUnknownVtbl) {
    fn AllowForegroundTransfer(
        lpvReserved: *mut c_void,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x969dc708, 0x5c76, 0x11d1, 0x8d, 0x86, 0x00, 0x00, 0xf8, 0x04, 0xb0, 0x57)]
interface IThumbnailExtractor(IThumbnailExtractorVtbl): IUnknown(IUnknownVtbl) {
    fn ExtractThumbnail(
        pStg: *mut IStorage,
        ulLength: ULONG,
        ulHeight: ULONG,
        pulOutputLength: *mut ULONG,
        pulOutputHeight: *mut ULONG,
        phOutputBitmap: *mut HBITMAP,
    ) -> HRESULT,
    fn OnFileUpdated(
        pStg: *mut IStorage,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x947990de, 0xcc28, 0x11d2, 0xa0, 0xf7, 0x00, 0x80, 0x5f, 0x85, 0x8f, 0xb1)]
interface IDummyHICONIncluder(IDummyHICONIncluderVtbl): IUnknown(IUnknownVtbl) {
    fn Dummy(
        h1: HICON,
        h2: HDC,
    ) -> HRESULT,
}
);
ENUM!{enum ApplicationType {
    ServerApplication = 0,
    LibraryApplication = ServerApplication + 1,
}}
ENUM!{enum ShutdownType {
    IdleShutdown = 0,
    ForcedShutdown = IdleShutdown + 1,
}}
RIDL!(
#[uuid(0x000001d5, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IProcessLock(IProcessLockVtbl): IUnknown(IUnknownVtbl) {
    fn AddRefOnProcess() -> ULONG,
    fn ReleaseRefOnProcess() -> ULONG,
}
);
RIDL!(
#[uuid(0x000001d4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ISurrogateService(ISurrogateServiceVtbl): IUnknown(IUnknownVtbl) {
    fn Init(
        rguidProcessID: REFGUID,
        pProcessLock: *mut IProcessLock,
        pfApplicationAware: *mut BOOL,
    ) -> HRESULT,
    fn ApplicationLaunch(
        rguidApplID: REFGUID,
        appType: ApplicationType,
    ) -> HRESULT,
    fn ApplicationFree(
        rguidApplID: REFGUID,
    ) -> HRESULT,
    fn CatalogRefresh(
        ulReserved: ULONG,
    ) -> HRESULT,
    fn ProcessShutdown(
        shutdownType: ShutdownType,
    ) -> HRESULT,
}
);
pub type LPINITIALIZESPY = *mut IInitializeSpy;
RIDL!(
#[uuid(0x00000034, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IInitializeSpy(IInitializeSpyVtbl): IUnknown(IUnknownVtbl) {
    fn PreInitialize(
        dwCoInit: DWORD,
        dwCurThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PostInitialize(
        hrCoInit: HRESULT,
        dwCoInit: DWORD,
        dwCurThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PreUninitialize(
        dwCurThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PostUninitialize(
        dwNewThreadAptRefs: DWORD,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0xa2f05a09, 0x27a2, 0x42b5, 0xbc, 0x0e, 0xac, 0x16, 0x3e, 0xf4, 0x9d, 0x9b)]
interface IApartmentShutdown(IApartmentShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn OnUninitialize(
        ui64ApartmentIdentifier: UINT64,
    ) -> (),
}
);
extern "system" {
    pub fn CLIPFORMAT_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pCF: *mut CLIPFORMAT,
    ) -> c_ulong;
    pub fn CLIPFORMAT_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pCF: *mut CLIPFORMAT,
    ) -> *mut c_uchar;
    pub fn CLIPFORMAT_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pCF: *mut CLIPFORMAT,
    ) -> *mut c_uchar;
    pub fn CLIPFORMAT_UserFree(
        pFlags: *mut c_ulong,
        pCF: *mut CLIPFORMAT,
    );
    pub fn HBITMAP_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phBmp: *mut HBITMAP,
    ) -> c_ulong;
    pub fn HBITMAP_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phBmp: *mut HBITMAP,
    ) -> *mut c_uchar;
    pub fn HBITMAP_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phBmp: *mut HBITMAP,
    ) -> *mut c_uchar;
    pub fn HBITMAP_UserFree(
        pFlags: *mut c_ulong,
        phBmp: *mut HBITMAP,
    );
    pub fn HDC_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phdc: *mut HDC,
    ) -> c_ulong;
    pub fn HDC_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phdc: *mut HDC,
    ) -> *mut c_uchar;
    pub fn HDC_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phdc: *mut HDC,
    ) -> *mut c_uchar;
    pub fn HDC_UserFree(
        pFlags: *mut c_ulong,
        phdc: *mut HDC,
    );
    pub fn HICON_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phIcon: *mut HICON,
    ) -> c_ulong;
    pub fn HICON_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phIcon: *mut HICON,
    ) -> *mut c_uchar;
    pub fn HICON_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phIcon: *mut HICON,
    ) -> *mut c_uchar;
    pub fn HICON_UserFree(
        pFlags: *mut c_ulong,
        phIcon: *mut HICON,
    );
    pub fn SNB_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pSnb: *mut SNB,
    ) -> c_ulong;
    pub fn SNB_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pSnb: *mut SNB,
    ) -> *mut c_uchar;
    pub fn SNB_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pSnb: *mut SNB,
    ) -> *mut c_uchar;
    pub fn SNB_UserFree(
        pFlags: *mut c_ulong,
        pSnb: *mut SNB,
    );
    pub fn STGMEDIUM_UserSize(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pstgm: *mut STGMEDIUM,
    ) -> c_ulong;
    pub fn STGMEDIUM_UserMarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pstgm: *mut STGMEDIUM,
    ) -> *mut c_uchar;
    pub fn STGMEDIUM_UserUnmarshal(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pstgm: *mut STGMEDIUM,
    ) -> *mut c_uchar;
    pub fn STGMEDIUM_UserFree(
        pFlags: *mut c_ulong,
        pstgm: *mut STGMEDIUM,
    );
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    pub fn CLIPFORMAT_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pCF: *mut CLIPFORMAT,
    ) -> c_ulong;
    pub fn CLIPFORMAT_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pCF: *mut CLIPFORMAT,
    ) -> *mut c_uchar;
    pub fn CLIPFORMAT_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pCF: *mut CLIPFORMAT,
    ) -> *mut c_uchar;
    pub fn CLIPFORMAT_UserFree64(
        pFlags: *mut c_ulong,
        pCF: *mut CLIPFORMAT,
    );
    pub fn HBITMAP_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phBmp: *mut HBITMAP,
    ) -> c_ulong;
    pub fn HBITMAP_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phBmp: *mut HBITMAP,
    ) -> *mut c_uchar;
    pub fn HBITMAP_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phBmp: *mut HBITMAP,
    ) -> *mut c_uchar;
    pub fn HBITMAP_UserFree64(
        pFlags: *mut c_ulong,
        phBmp: *mut HBITMAP,
    );
    pub fn HDC_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phdc: *mut HDC,
    ) -> c_ulong;
    pub fn HDC_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phdc: *mut HDC,
    ) -> *mut c_uchar;
    pub fn HDC_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phdc: *mut HDC,
    ) -> *mut c_uchar;
    pub fn HDC_UserFree64(
        pFlags: *mut c_ulong,
        phdc: *mut HDC,
    );
    pub fn HICON_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        phIcon: *mut HICON,
    ) -> c_ulong;
    pub fn HICON_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phIcon: *mut HICON,
    ) -> *mut c_uchar;
    pub fn HICON_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        phIcon: *mut HICON,
    ) -> *mut c_uchar;
    pub fn HICON_UserFree64(
        pFlags: *mut c_ulong,
        phIcon: *mut HICON,
    );
    pub fn SNB_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pSnb: *mut SNB,
    ) -> c_ulong;
    pub fn SNB_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pSnb: *mut SNB,
    ) -> *mut c_uchar;
    pub fn SNB_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pSnb: *mut SNB,
    ) -> *mut c_uchar;
    pub fn SNB_UserFree64(
        pFlags: *mut c_ulong,
        pSnb: *mut SNB,
    );
    pub fn STGMEDIUM_UserSize64(
        pFlags: *mut c_ulong,
        Offset: c_ulong,
        pstgm: *mut STGMEDIUM,
    ) -> c_ulong;
    pub fn STGMEDIUM_UserMarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pstgm: *mut STGMEDIUM,
    ) -> *mut c_uchar;
    pub fn STGMEDIUM_UserUnmarshal64(
        pFlags: *mut c_ulong,
        pBuffer: *mut c_uchar,
        pstgm: *mut STGMEDIUM,
    ) -> *mut c_uchar;
    pub fn STGMEDIUM_UserFree64(
        pFlags: *mut c_ulong,
        pstgm: *mut STGMEDIUM,
    );
}
