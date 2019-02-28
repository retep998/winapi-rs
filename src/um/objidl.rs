// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::basetsd::{SIZE_T, UINT64};
use shared::guiddef::{CLSID, IID, LPCLSID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, HGLOBAL, HTASK, ULONG, WORD};
use shared::rpcndr::byte;
use shared::windef::{HBITMAP, HDC, HENHMETAFILE, HICON, HWND};
use shared::wtypes::{
    CLIPFORMAT, HMETAFILEPICT, wireHBITMAP, wireHENHMETAFILE, wireHGLOBAL, wireHMETAFILEPICT,
    wireHPALETTE
};
use shared::wtypesbase::{BYTE_BLOB, LPCOLESTR, LPOLESTR, OLECHAR};
pub use um::objidlbase::*;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LARGE_INTEGER, LCID, LONG, LPCWSTR, ULARGE_INTEGER};
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0053_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0053_v0_0_s_ifspec;
pub type LPMALLOCSPY = *mut IMallocSpy;
DEFINE_GUID!{IID_IMallocSpy,
    0x0000001d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000001d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
    ) -> (),
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0054_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0054_v0_0_s_ifspec;
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
DEFINE_GUID!{IID_IBindCtx,
    0x0000000e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000000e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IBindCtx_RemoteSetBindOptions_Proxy(
//     IBindCtx * This,
//     BIND_OPTS2 *pbindopts);
// void __RPC_STUB IBindCtx_RemoteSetBindOptions_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IBindCtx_RemoteGetBindOptions_Proxy(
//     IBindCtx * This,
//     BIND_OPTS2 *pbindopts);
// void __RPC_STUB IBindCtx_RemoteGetBindOptions_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPENUMMONIKER = *mut IEnumMoniker;
DEFINE_GUID!{IID_IEnumMoniker,
    0x00000102, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000102, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IEnumMoniker_RemoteNext_Proxy(
//     IEnumMoniker * This,
//     ULONG celt,
//     IMoniker **rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumMoniker_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0056_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0056_v0_0_s_ifspec;
pub type LPRUNNABLEOBJECT = *mut IRunnableObject;
DEFINE_GUID!{IID_IRunnableObject,
    0x00000126, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000126, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IRunnableObject_RemoteIsRunning_Proxy(
//     IRunnableObject * This);
// void __RPC_STUB IRunnableObject_RemoteIsRunning_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPRUNNINGOBJECTTABLE = *mut IRunningObjectTable;
DEFINE_GUID!{IID_IRunningObjectTable,
    0x00000010, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000010, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0058_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0058_v0_0_s_ifspec;
pub type LPPERSIST = *mut IPersist;
DEFINE_GUID!{IID_IPersist,
    0x0000010c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID(
        pClassID: *mut CLSID,
    ) -> HRESULT,
}}
pub type LPPERSISTSTREAM = *mut IPersistStream;
DEFINE_GUID!{IID_IPersistStream,
    0x00000109, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000109, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
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
    MKRREDUCE_ONE = 3<<16,
    MKRREDUCE_TOUSER = 2<<16,
    MKRREDUCE_THROUGHUSER = 1<<16,
    MKRREDUCE_ALL = 0,
}}
DEFINE_GUID!{IID_IMoniker,
    0x0000000f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000000f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IMoniker_RemoteBindToObject_Proxy(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riidResult,
//     IUnknown **ppvResult);
// void __RPC_STUB IMoniker_RemoteBindToObject_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IMoniker_RemoteBindToStorage_Proxy(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riid,
//     IUnknown **ppvObj);
// void __RPC_STUB IMoniker_RemoteBindToStorage_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0061_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0061_v0_0_s_ifspec;
DEFINE_GUID!{IID_IROTData,
    0xf29f6bc0, 0x5021, 0x11ce, 0xaa, 0x15, 0x00, 0x00, 0x69, 0x01, 0x29, 0x3f}
RIDL!{#[uuid(0xf29f6bc0, 0x5021, 0x11ce, 0xaa, 0x15, 0x00, 0x00, 0x69, 0x01, 0x29, 0x3f)]
interface IROTData(IROTDataVtbl): IUnknown(IUnknownVtbl) {
    fn GetComparisonData(
        pbData: *mut byte,
        cbMax: ULONG,
        pcbData: *mut ULONG,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0062_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0062_v0_0_s_ifspec;
pub type LPENUMSTATSTG = *mut IEnumSTATSTG;
DEFINE_GUID!{IID_IEnumSTATSTG,
    0x0000000d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000000d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IEnumSTATSTG_RemoteNext_Proxy(
//     IEnumSTATSTG * This,
//     ULONG celt,
//     STATSTG *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumSTATSTG_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPSTORAGE = *mut IStorage;
STRUCT!{struct RemSNB {
    ulCntStr: ULONG,
    ulCntChar: ULONG,
    rgString: [OLECHAR; 1],
}}
pub type wireSNB = *mut RemSNB;
pub type SNB = *mut LPOLESTR;
DEFINE_GUID!{IID_IStorage,
    0x0000000b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000000b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
        pstgDest: *mut IStorage,
    ) -> HRESULT,
    fn MoveElementTo(
        pwcsName: *const OLECHAR,
        pstgDest: *mut IStorage,
        pwcsNewName: *const OLECHAR,
        grfFlags: DWORD,
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
}}
// HRESULT STDMETHODCALLTYPE IStorage_RemoteOpenStream_Proxy(
//     IStorage * This,
//     const OLECHAR *pwcsName,
//     ULONG cbReserved1,
//     byte *reserved1,
//     DWORD grfMode,
//     DWORD reserved2,
//     IStream **ppstm);
// void __RPC_STUB IStorage_RemoteOpenStream_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IStorage_RemoteCopyTo_Proxy(
//     IStorage * This,
//     DWORD ciidExclude,
//     const IID *rgiidExclude,
//     SNB snbExclude,
//     IStorage *pstgDest);
// void __RPC_STUB IStorage_RemoteCopyTo_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IStorage_RemoteEnumElements_Proxy(
//     IStorage * This,
//     DWORD reserved1,
//     ULONG cbReserved2,
//     byte *reserved2,
//     DWORD reserved3,
//     IEnumSTATSTG **ppenum);
// void __RPC_STUB IStorage_RemoteEnumElements_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0064_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0064_v0_0_s_ifspec;
pub type LPPERSISTFILE = *mut IPersistFile;
DEFINE_GUID!{IID_IPersistFile,
    0x0000010b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
pub type LPPERSISTSTORAGE = *mut IPersistStorage;
DEFINE_GUID!{IID_IPersistStorage,
    0x0000010a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0066_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0066_v0_0_s_ifspec;
pub type LPLOCKBYTES = *mut ILockBytes;
DEFINE_GUID!{IID_ILockBytes,
    0x0000000a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000000a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT __stdcall ILockBytes_RemoteReadAt_Proxy(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     byte *pv,
//     ULONG cb,
//     ULONG *pcbRead);
// void __RPC_STUB ILockBytes_RemoteReadAt_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE ILockBytes_RemoteWriteAt_Proxy(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// void __RPC_STUB ILockBytes_RemoteWriteAt_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
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
DEFINE_GUID!{IID_IEnumFORMATETC,
    0x00000103, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000103, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IEnumFORMATETC_RemoteNext_Proxy(
//     IEnumFORMATETC * This,
//     ULONG celt,
//     FORMATETC *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumFORMATETC_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
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
DEFINE_GUID!{IID_IEnumSTATDATA,
    0x00000105, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000105, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IEnumSTATDATA_RemoteNext_Proxy(
//     IEnumSTATDATA * This,
//     ULONG celt,
//     STATDATA *rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumSTATDATA_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPROOTSTORAGE = *mut IRootStorage;
DEFINE_GUID!{IID_IRootStorage,
    0x00000012, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000012, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRootStorage(IRootStorageVtbl): IUnknown(IUnknownVtbl) {
    fn SwitchToFile(
        pszFile: LPOLESTR,
    ) -> HRESULT,
}}
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
    [u32; 1] [u64; 1],
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
pub const OBJ_PEN: DWORD = 1;
pub const OBJ_BRUSH: DWORD = 2;
pub const OBJ_DC: DWORD = 3;
pub const OBJ_METADC: DWORD = 4;
pub const OBJ_PAL: DWORD = 5;
pub const OBJ_FONT: DWORD = 6;
pub const OBJ_BITMAP: DWORD = 7;
pub const OBJ_REGION: DWORD = 8;
pub const OBJ_METAFILE: DWORD = 9;
pub const OBJ_MEMDC: DWORD = 10;
pub const OBJ_EXTPEN: DWORD = 11;
pub const OBJ_ENHMETADC: DWORD = 12;
pub const OBJ_ENHMETAFILE: DWORD = 13;
UNION!{union GDI_OBJECT_u {
    [usize; 1],
    hBitmap hBitmap_mut: wireHBITMAP,
    hPalette hPalette_mut: wireHPALETTE,
    wireHGLOBAL wireHGLOBAL_mut: wireHGLOBAL,
}}
STRUCT!{struct GDI_OBJECT {
    ObjectType: DWORD,
    u: GDI_OBJECT_u,
}}
UNION!{union userSTGMEDIUM_u_u {
    [usize; 1],
    hMetaFilePict hMetaFilePict_mut: wireHMETAFILEPICT,
    hHEnhMetaFile hHEnhMetaFile_mut: wireHENHMETAFILE,
    hGdiHandle hGdiHandle_mut: *mut GDI_OBJECT,
    hGlobal hGlobal_mut: wireHGLOBAL,
    lpszFileName lpszFileName_mut: LPOLESTR,
    pstm pstm_mut: *mut BYTE_BLOB,
    pstg pstg_mut: *mut BYTE_BLOB,
}}
STRUCT!{struct userSTGMEDIUM_u {
    tymed: DWORD,
    u: userSTGMEDIUM_u_u,
}}
STRUCT!{struct userSTGMEDIUM {
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
DEFINE_GUID!{IID_IAdviseSink,
    0x0000010f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010f, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnDataChange_Proxy(
//     IAdviseSink * This,
//     FORMATETC *pFormatetc,
//     ASYNC_STGMEDIUM *pStgmed);
// void __RPC_STUB IAdviseSink_RemoteOnDataChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnViewChange_Proxy(
//     IAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// void __RPC_STUB IAdviseSink_RemoteOnViewChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnRename_Proxy(
//     IAdviseSink * This,
//     IMoniker *pmk);
// void __RPC_STUB IAdviseSink_RemoteOnRename_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnSave_Proxy(
//     IAdviseSink * This);
// void __RPC_STUB IAdviseSink_RemoteOnSave_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnClose_Proxy(
//     IAdviseSink * This);
// void __RPC_STUB IAdviseSink_RemoteOnClose_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_AsyncIAdviseSink,
    0x00000150, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000150, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnDataChange_Proxy(
//     AsyncIAdviseSink * This,
//     FORMATETC *pFormatetc,
//     ASYNC_STGMEDIUM *pStgmed);
// void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnDataChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnDataChange_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnDataChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnViewChange_Proxy(
//     AsyncIAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnViewChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnViewChange_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnViewChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnRename_Proxy(
//     AsyncIAdviseSink * This,
//     IMoniker *pmk);
// void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnRename_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnRename_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnRename_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnSave_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnSave_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnSave_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnSave_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnClose_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnClose_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnClose_Proxy(
//     AsyncIAdviseSink * This);
// void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnClose_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0071_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0071_v0_0_s_ifspec;
pub type LPADVISESINK2 = *mut IAdviseSink2;
DEFINE_GUID!{IID_IAdviseSink2,
    0x00000125, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000125, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IAdviseSink2(IAdviseSink2Vtbl): IAdviseSink(IAdviseSinkVtbl) {
    fn OnLinkSrcChange(
        pmk: *mut IMoniker,
    ) -> (),
}}
// HRESULT STDMETHODCALLTYPE IAdviseSink2_RemoteOnLinkSrcChange_Proxy(
//     IAdviseSink2 * This,
//     IMoniker *pmk);
// void __RPC_STUB IAdviseSink2_RemoteOnLinkSrcChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_AsyncIAdviseSink2,
    0x00000151, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000151, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIAdviseSink2(AsyncIAdviseSink2Vtbl): AsyncIAdviseSink(AsyncIAdviseSinkVtbl) {
    fn Begin_OnLinkSrcChange(
        pmk: *mut IMoniker,
    ) -> (),
    fn Finish_OnLinkSrcChange() -> (),
}}
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_RemoteOnLinkSrcChange_Proxy(
//     AsyncIAdviseSink2 * This,
//     IMoniker *pmk);
// void __RPC_STUB AsyncIAdviseSink2_Begin_RemoteOnLinkSrcChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_RemoteOnLinkSrcChange_Proxy(
//     AsyncIAdviseSink2 * This);
// void __RPC_STUB AsyncIAdviseSink2_Finish_RemoteOnLinkSrcChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0072_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0072_v0_0_s_ifspec;
pub type LPDATAOBJECT = *mut IDataObject;
ENUM!{enum DATADIR {
    DATADIR_GET = 1,
    DATADIR_SET = 2,
}}
DEFINE_GUID!{IID_IDataObject,
    0x0000010e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
        pformatectIn: *mut FORMATETC,
        pformatetcOut: *mut FORMATETC,
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
}}
// HRESULT STDMETHODCALLTYPE IDataObject_RemoteGetData_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetcIn,
//     STGMEDIUM *pRemoteMedium);
// void __RPC_STUB IDataObject_RemoteGetData_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IDataObject_RemoteGetDataHere_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     STGMEDIUM *pRemoteMedium);
// void __RPC_STUB IDataObject_RemoteGetDataHere_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IDataObject_RemoteSetData_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     FLAG_STGMEDIUM *pmedium,
//     BOOL fRelease);
// void __RPC_STUB IDataObject_RemoteSetData_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0073_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0073_v0_0_s_ifspec;
pub type LPDATAADVISEHOLDER = *mut IDataAdviseHolder;
DEFINE_GUID!{IID_IDataAdviseHolder,
    0x00000110, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000110, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
        dwReserved: DWORD,
        advf: DWORD,
    ) -> HRESULT,
}}
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
DEFINE_GUID!{IID_IMessageFilter,
    0x00000016, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000016, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// extern const FMTID FMTID_SummaryInformation;
// extern const FMTID FMTID_DocSummaryInformation;
// extern const FMTID FMTID_UserDefinedProperties;
// extern const FMTID FMTID_DiscardableInformation;
// extern const FMTID FMTID_ImageSummaryInformation;
// extern const FMTID FMTID_AudioSummaryInformation;
// extern const FMTID FMTID_VideoSummaryInformation;
// extern const FMTID FMTID_MediaFileSummaryInformation;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0075_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0075_v0_0_s_ifspec;
DEFINE_GUID!{IID_IClassActivator,
    0x00000140, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000140, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IClassActivator(IClassActivatorVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassObject(
        rclsid: REFCLSID,
        dwClassContext: DWORD,
        locale: LCID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0076_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0076_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFillLockBytes,
    0x99caf010, 0x415e, 0x11cf, 0x88, 0x14, 0x00, 0xaa, 0x00, 0xb5, 0x69, 0xf5}
RIDL!{#[uuid(0x99caf010, 0x415e, 0x11cf, 0x88, 0x14, 0x00, 0xaa, 0x00, 0xb5, 0x69, 0xf5)]
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
}}
// HRESULT __stdcall IFillLockBytes_RemoteFillAppend_Proxy(
//     IFillLockBytes * This,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// void __RPC_STUB IFillLockBytes_RemoteFillAppend_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT __stdcall IFillLockBytes_RemoteFillAt_Proxy(
//     IFillLockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// void __RPC_STUB IFillLockBytes_RemoteFillAt_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0077_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0077_v0_0_s_ifspec;
DEFINE_GUID!{IID_IProgressNotify,
    0xa9d758a0, 0x4617, 0x11cf, 0x95, 0xfc, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4}
RIDL!{#[uuid(0xa9d758a0, 0x4617, 0x11cf, 0x95, 0xfc, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
interface IProgressNotify(IProgressNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn OnProgress(
        dwProgressCurrent: DWORD,
        dwProgressMaximum: DWORD,
        fAccurate: BOOL,
        fOwner: BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0078_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0078_v0_0_s_ifspec;
STRUCT!{struct StorageLayout {
    LayoutType: DWORD,
    pwcsElementName: *mut OLECHAR,
    cOffset: LARGE_INTEGER,
    cBytes: LARGE_INTEGER,
}}
DEFINE_GUID!{IID_ILayoutStorage,
    0x0e6d4d90, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4}
RIDL!{#[uuid(0x0e6d4d90, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0079_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0079_v0_0_s_ifspec;
DEFINE_GUID!{IID_IBlockingLock,
    0x30f3d47a, 0x6447, 0x11d1, 0x8e, 0x3c, 0x00, 0xc0, 0x4f, 0xb9, 0x38, 0x6d}
RIDL!{#[uuid(0x30f3d47a, 0x6447, 0x11d1, 0x8e, 0x3c, 0x00, 0xc0, 0x4f, 0xb9, 0x38, 0x6d)]
interface IBlockingLock(IBlockingLockVtbl): IUnknown(IUnknownVtbl) {
    fn Lock(
        dwTimeout: DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
}}
DEFINE_GUID!{IID_ITimeAndNoticeControl,
    0xbc0bf6ae, 0x8878, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4}
RIDL!{#[uuid(0xbc0bf6ae, 0x8878, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4)]
interface ITimeAndNoticeControl(ITimeAndNoticeControlVtbl): IUnknown(IUnknownVtbl) {
    fn SuppressChanges(
        res1: DWORD,
        res2: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IOplockStorage,
    0x8d19c834, 0x8879, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4}
RIDL!{#[uuid(0x8d19c834, 0x8879, 0x11d1, 0x83, 0xe9, 0x00, 0xc0, 0x4f, 0xc2, 0xc6, 0xd4)]
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0082_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0082_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDirectWriterLock,
    0x0e6d4d92, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4}
RIDL!{#[uuid(0x0e6d4d92, 0x6738, 0x11cf, 0x96, 0x08, 0x00, 0xaa, 0x00, 0x68, 0x0d, 0xb4)]
interface IDirectWriterLock(IDirectWriterLockVtbl): IUnknown(IUnknownVtbl) {
    fn WaitForWriteAccess(
        dwTimeout: DWORD,
    ) -> HRESULT,
    fn ReleaseWriteAccess() -> HRESULT,
    fn HaveWriteAccess() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0083_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0083_v0_0_s_ifspec;
DEFINE_GUID!{IID_IUrlMon,
    0x00000026, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000026, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
DEFINE_GUID!{IID_IForegroundTransfer,
    0x00000145, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000145, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IForegroundTransfer(IForegroundTransferVtbl): IUnknown(IUnknownVtbl) {
    fn AllowForegroundTransfer(
        lpvReserved: *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IThumbnailExtractor,
    0x969dc708, 0x5c76, 0x11d1, 0x8d, 0x86, 0x00, 0x00, 0xf8, 0x04, 0xb0, 0x57}
RIDL!{#[uuid(0x969dc708, 0x5c76, 0x11d1, 0x8d, 0x86, 0x00, 0x00, 0xf8, 0x04, 0xb0, 0x57)]
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
}}
DEFINE_GUID!{IID_IDummyHICONIncluder,
    0x947990de, 0xcc28, 0x11d2, 0xa0, 0xf7, 0x00, 0x80, 0x5f, 0x85, 0x8f, 0xb1}
RIDL!{#[uuid(0x947990de, 0xcc28, 0x11d2, 0xa0, 0xf7, 0x00, 0x80, 0x5f, 0x85, 0x8f, 0xb1)]
interface IDummyHICONIncluder(IDummyHICONIncluderVtbl): IUnknown(IUnknownVtbl) {
    fn Dummy(
        h1: HICON,
        h2: HDC,
    ) -> HRESULT,
}}
ENUM!{enum ApplicationType {
    ServerApplication,
    LibraryApplication,
}}
ENUM!{enum ShutdownType {
    IdleShutdown,
    ForcedShutdown,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0087_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0087_v0_0_s_ifspec;
DEFINE_GUID!{IID_IProcessLock,
    0x000001d5, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000001d5, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IProcessLock(IProcessLockVtbl): IUnknown(IUnknownVtbl) {
    fn AddRefOnProcess() -> ULONG,
    fn ReleaseRefOnProcess() -> ULONG,
}}
DEFINE_GUID!{IID_ISurrogateService,
    0x000001d4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000001d4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
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
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0089_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0089_v0_0_s_ifspec;
pub type LPINITIALIZESPY = *mut IInitializeSpy;
DEFINE_GUID!{IID_IInitializeSpy,
    0x00000034, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000034, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IInitializeSpy(IInitializeSpyVtbl): IUnknown(IUnknownVtbl) {
    fn PreInitialize(
        dwCoInit: DWORD,
        dwCurThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PostInitialize(
        hrCoInit: HRESULT,
        dwCoInit: DWORD,
        dwNewThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PreUninitialize(
        dwCurThreadAptRefs: DWORD,
    ) -> HRESULT,
    fn PostUninitialize(
        dwNewThreadAptRefs: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0090_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0090_v0_0_s_ifspec;
DEFINE_GUID!{IID_IApartmentShutdown,
    0xa2f05a09, 0x27a2, 0x42b5, 0xbc, 0x0e, 0xac, 0x16, 0x3e, 0xf4, 0x9d, 0x9b}
RIDL!{#[uuid(0xa2f05a09, 0x27a2, 0x42b5, 0xbc, 0x0e, 0xac, 0x16, 0x3e, 0xf4, 0x9d, 0x9b)]
interface IApartmentShutdown(IApartmentShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn OnUninitialize(
        ui64ApartmentIdentifier: UINT64,
    ) -> (),
}}
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0091_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0091_v0_0_s_ifspec;
// unsigned long             __RPC_USER  ASYNC_STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in ASYNC_STGMEDIUM * );
// unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in ASYNC_STGMEDIUM * );
// unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out ASYNC_STGMEDIUM * );
// void                      __RPC_USER  ASYNC_STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in ASYNC_STGMEDIUM * );
// unsigned long             __RPC_USER  CLIPFORMAT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * );
// void                      __RPC_USER  CLIPFORMAT_UserFree(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * );
// unsigned long             __RPC_USER  FLAG_STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in FLAG_STGMEDIUM * );
// unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in FLAG_STGMEDIUM * );
// unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out FLAG_STGMEDIUM * );
// void                      __RPC_USER  FLAG_STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in FLAG_STGMEDIUM * );
// unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * );
// unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * );
// void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * );
// unsigned long             __RPC_USER  HDC_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * );
// void                      __RPC_USER  HDC_UserFree(     __RPC__in unsigned long *, __RPC__in HDC * );
// unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * );
// unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * );
// unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * );
// void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * );
// unsigned long             __RPC_USER  SNB_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in SNB * );
// unsigned char * __RPC_USER  SNB_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in SNB * );
// unsigned char * __RPC_USER  SNB_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out SNB * );
// void                      __RPC_USER  SNB_UserFree(     __RPC__in unsigned long *, __RPC__in SNB * );
// unsigned long             __RPC_USER  STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * );
// void                      __RPC_USER  STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * );
// unsigned long             __RPC_USER  ASYNC_STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in ASYNC_STGMEDIUM * );
// unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in ASYNC_STGMEDIUM * );
// unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out ASYNC_STGMEDIUM * );
// void                      __RPC_USER  ASYNC_STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in ASYNC_STGMEDIUM * );
// unsigned long             __RPC_USER  CLIPFORMAT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * );
// void                      __RPC_USER  CLIPFORMAT_UserFree64(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * );
// unsigned long             __RPC_USER  FLAG_STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in FLAG_STGMEDIUM * );
// unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in FLAG_STGMEDIUM * );
// unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out FLAG_STGMEDIUM * );
// void                      __RPC_USER  FLAG_STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in FLAG_STGMEDIUM * );
// unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * );
// unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * );
// void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * );
// unsigned long             __RPC_USER  HDC_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * );
// void                      __RPC_USER  HDC_UserFree64(     __RPC__in unsigned long *, __RPC__in HDC * );
// unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * );
// unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * );
// unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * );
// void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * );
// unsigned long             __RPC_USER  SNB_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in SNB * );
// unsigned char * __RPC_USER  SNB_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in SNB * );
// unsigned char * __RPC_USER  SNB_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out SNB * );
// void                      __RPC_USER  SNB_UserFree64(     __RPC__in unsigned long *, __RPC__in SNB * );
// unsigned long             __RPC_USER  STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * );
// void                      __RPC_USER  STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * );
// HRESULT STDMETHODCALLTYPE IEnumUnknown_Next_Proxy(
//     IEnumUnknown * This,
//     ULONG celt,
//     IUnknown **rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumUnknown_Next_Stub(
//     IEnumUnknown * This,
//     ULONG celt,
//     IUnknown **rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumString_Next_Proxy(
//     IEnumString * This,
//     ULONG celt,
//     LPOLESTR *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumString_Next_Stub(
//     IEnumString * This,
//     ULONG celt,
//     LPOLESTR *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE ISequentialStream_Read_Proxy(
//     ISequentialStream * This,
//     void *pv,
//     ULONG cb,
//     ULONG *pcbRead);
// HRESULT STDMETHODCALLTYPE ISequentialStream_Read_Stub(
//     ISequentialStream * This,
//     byte *pv,
//     ULONG cb,
//     ULONG *pcbRead);
// HRESULT STDMETHODCALLTYPE ISequentialStream_Write_Proxy(
//     ISequentialStream * This,
//     const void *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT STDMETHODCALLTYPE ISequentialStream_Write_Stub(
//     ISequentialStream * This,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT STDMETHODCALLTYPE IStream_Seek_Proxy(
//     IStream * This,
//     LARGE_INTEGER dlibMove,
//     DWORD dwOrigin,
//     ULARGE_INTEGER *plibNewPosition);
// HRESULT STDMETHODCALLTYPE IStream_Seek_Stub(
//     IStream * This,
//     LARGE_INTEGER dlibMove,
//     DWORD dwOrigin,
//     ULARGE_INTEGER *plibNewPosition);
// HRESULT STDMETHODCALLTYPE IStream_CopyTo_Proxy(
//     IStream * This,
//     IStream *pstm,
//     ULARGE_INTEGER cb,
//     ULARGE_INTEGER *pcbRead,
//     ULARGE_INTEGER *pcbWritten);
// HRESULT STDMETHODCALLTYPE IStream_CopyTo_Stub(
//     IStream * This,
//     IStream *pstm,
//     ULARGE_INTEGER cb,
//     ULARGE_INTEGER *pcbRead,
//     ULARGE_INTEGER *pcbWritten);
// HRESULT STDMETHODCALLTYPE IBindCtx_SetBindOptions_Proxy(
//     IBindCtx * This,
//     BIND_OPTS *pbindopts);
// HRESULT STDMETHODCALLTYPE IBindCtx_SetBindOptions_Stub(
//     IBindCtx * This,
//     BIND_OPTS2 *pbindopts);
// HRESULT STDMETHODCALLTYPE IBindCtx_GetBindOptions_Proxy(
//     IBindCtx * This,
//     BIND_OPTS *pbindopts);
// HRESULT STDMETHODCALLTYPE IBindCtx_GetBindOptions_Stub(
//     IBindCtx * This,
//     BIND_OPTS2 *pbindopts);
// HRESULT STDMETHODCALLTYPE IEnumMoniker_Next_Proxy(
//     IEnumMoniker * This,
//     ULONG celt,
//     IMoniker **rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumMoniker_Next_Stub(
//     IEnumMoniker * This,
//     ULONG celt,
//     IMoniker **rgelt,
//     ULONG *pceltFetched);
// BOOL STDMETHODCALLTYPE IRunnableObject_IsRunning_Proxy(
//     IRunnableObject * This);
// HRESULT STDMETHODCALLTYPE IRunnableObject_IsRunning_Stub(
//     IRunnableObject * This);
// HRESULT STDMETHODCALLTYPE IMoniker_BindToObject_Proxy(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riidResult,
//     void **ppvResult);
// HRESULT STDMETHODCALLTYPE IMoniker_BindToObject_Stub(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riidResult,
//     IUnknown **ppvResult);
// HRESULT STDMETHODCALLTYPE IMoniker_BindToStorage_Proxy(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riid,
//     void **ppvObj);
// HRESULT STDMETHODCALLTYPE IMoniker_BindToStorage_Stub(
//     IMoniker * This,
//     IBindCtx *pbc,
//     IMoniker *pmkToLeft,
//     REFIID riid,
//     IUnknown **ppvObj);
// HRESULT STDMETHODCALLTYPE IEnumSTATSTG_Next_Proxy(
//     IEnumSTATSTG * This,
//     ULONG celt,
//     STATSTG *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATSTG_Next_Stub(
//     IEnumSTATSTG * This,
//     ULONG celt,
//     STATSTG *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IStorage_OpenStream_Proxy(
//     IStorage * This,
//     const OLECHAR *pwcsName,
//     void *reserved1,
//     DWORD grfMode,
//     DWORD reserved2,
//     IStream **ppstm);
// HRESULT STDMETHODCALLTYPE IStorage_OpenStream_Stub(
//     IStorage * This,
//     const OLECHAR *pwcsName,
//     ULONG cbReserved1,
//     byte *reserved1,
//     DWORD grfMode,
//     DWORD reserved2,
//     IStream **ppstm);
// HRESULT STDMETHODCALLTYPE IStorage_CopyTo_Proxy(
//     IStorage * This,
//     DWORD ciidExclude,
//     const IID *rgiidExclude,
//     SNB snbExclude,
//     IStorage *pstgDest);
// HRESULT STDMETHODCALLTYPE IStorage_CopyTo_Stub(
//     IStorage * This,
//     DWORD ciidExclude,
//     const IID *rgiidExclude,
//     SNB snbExclude,
//     IStorage *pstgDest);
// HRESULT STDMETHODCALLTYPE IStorage_EnumElements_Proxy(
//     IStorage * This,
//     DWORD reserved1,
//     void *reserved2,
//     DWORD reserved3,
//     IEnumSTATSTG **ppenum);
// HRESULT STDMETHODCALLTYPE IStorage_EnumElements_Stub(
//     IStorage * This,
//     DWORD reserved1,
//     ULONG cbReserved2,
//     byte *reserved2,
//     DWORD reserved3,
//     IEnumSTATSTG **ppenum);
// HRESULT STDMETHODCALLTYPE ILockBytes_ReadAt_Proxy(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     void *pv,
//     ULONG cb,
//     ULONG *pcbRead);
// HRESULT __stdcall ILockBytes_ReadAt_Stub(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     byte *pv,
//     ULONG cb,
//     ULONG *pcbRead);
// HRESULT STDMETHODCALLTYPE ILockBytes_WriteAt_Proxy(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const void *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT STDMETHODCALLTYPE ILockBytes_WriteAt_Stub(
//     ILockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT STDMETHODCALLTYPE IEnumFORMATETC_Next_Proxy(
//     IEnumFORMATETC * This,
//     ULONG celt,
//     FORMATETC *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumFORMATETC_Next_Stub(
//     IEnumFORMATETC * This,
//     ULONG celt,
//     FORMATETC *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATDATA_Next_Proxy(
//     IEnumSTATDATA * This,
//     ULONG celt,
//     STATDATA *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumSTATDATA_Next_Stub(
//     IEnumSTATDATA * This,
//     ULONG celt,
//     STATDATA *rgelt,
//     ULONG *pceltFetched);
// void STDMETHODCALLTYPE IAdviseSink_OnDataChange_Proxy(
//     IAdviseSink * This,
//     FORMATETC *pFormatetc,
//     STGMEDIUM *pStgmed);
// HRESULT STDMETHODCALLTYPE IAdviseSink_OnDataChange_Stub(
//     IAdviseSink * This,
//     FORMATETC *pFormatetc,
//     ASYNC_STGMEDIUM *pStgmed);
// void STDMETHODCALLTYPE IAdviseSink_OnViewChange_Proxy(
//     IAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// HRESULT STDMETHODCALLTYPE IAdviseSink_OnViewChange_Stub(
//     IAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// void STDMETHODCALLTYPE IAdviseSink_OnRename_Proxy(
//     IAdviseSink * This,
//     IMoniker *pmk);
// HRESULT STDMETHODCALLTYPE IAdviseSink_OnRename_Stub(
//     IAdviseSink * This,
//     IMoniker *pmk);
// void STDMETHODCALLTYPE IAdviseSink_OnSave_Proxy(
//     IAdviseSink * This);
// HRESULT STDMETHODCALLTYPE IAdviseSink_OnSave_Stub(
//     IAdviseSink * This);
// void STDMETHODCALLTYPE IAdviseSink_OnClose_Proxy(
//     IAdviseSink * This);
// HRESULT STDMETHODCALLTYPE IAdviseSink_OnClose_Stub(
//     IAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnDataChange_Proxy(
//     AsyncIAdviseSink * This,
//     FORMATETC *pFormatetc,
//     STGMEDIUM *pStgmed);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnDataChange_Stub(
//     AsyncIAdviseSink * This,
//     FORMATETC *pFormatetc,
//     ASYNC_STGMEDIUM *pStgmed);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnDataChange_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnDataChange_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnViewChange_Proxy(
//     AsyncIAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnViewChange_Stub(
//     AsyncIAdviseSink * This,
//     DWORD dwAspect,
//     LONG lindex);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnViewChange_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnViewChange_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnRename_Proxy(
//     AsyncIAdviseSink * This,
//     IMoniker *pmk);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnRename_Stub(
//     AsyncIAdviseSink * This,
//     IMoniker *pmk);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnRename_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnRename_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnSave_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnSave_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnSave_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnSave_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnClose_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnClose_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnClose_Proxy(
//     AsyncIAdviseSink * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnClose_Stub(
//     AsyncIAdviseSink * This);
// void STDMETHODCALLTYPE IAdviseSink2_OnLinkSrcChange_Proxy(
//     IAdviseSink2 * This,
//     IMoniker *pmk);
// HRESULT STDMETHODCALLTYPE IAdviseSink2_OnLinkSrcChange_Stub(
//     IAdviseSink2 * This,
//     IMoniker *pmk);
// void STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_OnLinkSrcChange_Proxy(
//     AsyncIAdviseSink2 * This,
//     IMoniker *pmk);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_OnLinkSrcChange_Stub(
//     AsyncIAdviseSink2 * This,
//     IMoniker *pmk);
// void STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_OnLinkSrcChange_Proxy(
//     AsyncIAdviseSink2 * This);
// HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_OnLinkSrcChange_Stub(
//     AsyncIAdviseSink2 * This);
// HRESULT STDMETHODCALLTYPE IDataObject_GetData_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetcIn,
//     STGMEDIUM *pmedium);
// HRESULT STDMETHODCALLTYPE IDataObject_GetData_Stub(
//     IDataObject * This,
//     FORMATETC *pformatetcIn,
//     STGMEDIUM *pRemoteMedium);
// HRESULT STDMETHODCALLTYPE IDataObject_GetDataHere_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     STGMEDIUM *pmedium);
// HRESULT STDMETHODCALLTYPE IDataObject_GetDataHere_Stub(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     STGMEDIUM *pRemoteMedium);
// HRESULT STDMETHODCALLTYPE IDataObject_SetData_Proxy(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     STGMEDIUM *pmedium,
//     BOOL fRelease);
// HRESULT STDMETHODCALLTYPE IDataObject_SetData_Stub(
//     IDataObject * This,
//     FORMATETC *pformatetc,
//     FLAG_STGMEDIUM *pmedium,
//     BOOL fRelease);
// HRESULT STDMETHODCALLTYPE IFillLockBytes_FillAppend_Proxy(
//     IFillLockBytes * This,
//     const void *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT __stdcall IFillLockBytes_FillAppend_Stub(
//     IFillLockBytes * This,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT STDMETHODCALLTYPE IFillLockBytes_FillAt_Proxy(
//     IFillLockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const void *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
// HRESULT __stdcall IFillLockBytes_FillAt_Stub(
//     IFillLockBytes * This,
//     ULARGE_INTEGER ulOffset,
//     const byte *pv,
//     ULONG cb,
//     ULONG *pcbWritten);
