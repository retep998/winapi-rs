// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use _core::ptr;
use ctypes::{c_int, c_long, c_void};
use shared::basetsd::{DWORD_PTR, UINT_PTR};
use shared::guiddef::{CLSID, FMTID, GUID, LPCGUID, REFCLSID, REFFMTID, REFGUID, REFIID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, HKEY, LPARAM, LRESULT, UINT, ULONG, WORD, WPARAM
};
use shared::windef::{
    COLORREF, HACCEL, HBITMAP, HICON, HMENU, HMONITOR, HWND, LPCRECT, POINT, POINTL, RECT, RECTL,
    SIZE
};
use shared::winerror::{E_INVALIDARG, SUCCEEDED};
use shared::wtypes::{PROPERTYKEY, PROPID};
use shared::wtypesbase::CLSCTX_INPROC_SERVER;
use um::cguid::GUID_NULL;
use um::combaseapi::{CoCreateInstance, CoTaskMemFree};
use um::comcat::IEnumGUID;
use um::commctrl::{HIMAGELIST, LPTBBUTTON};
use um::minwinbase::{WIN32_FIND_DATAA, WIN32_FIND_DATAW};
use um::oaidl::{IPropertyBag, VARIANT};
use um::objectarray::IObjectArray;
use um::objidl::{IBindCtx, IDataObject, IPersist, IPersistVtbl};
use um::objidlbase::{IMalloc, IStream};
use um::oleidl::{
    DROPEFFECT_COPY, DROPEFFECT_LINK, DROPEFFECT_MOVE, HOLEMENU, IOleWindow, IOleWindowVtbl,
    LPOLEMENUGROUPWIDTHS
};
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::propkeydef::REFPROPERTYKEY;
use um::propsys::{
    GETPROPERTYSTOREFLAGS, IPropertyChangeArray, IPropertyDescriptionList, IPropertyStore
};
use um::prsht::LPFNADDPROPSHEETPAGE;
use um::servprov::IServiceProvider;
use um::shellapi::{
    SEE_MASK_ASYNCOK, SEE_MASK_FLAG_LOG_USAGE, SEE_MASK_FLAG_NO_UI, SEE_MASK_HOTKEY,
    SEE_MASK_ICON, SEE_MASK_NOASYNC, SEE_MASK_NOZONECHECKS, SEE_MASK_NO_CONSOLE, SEE_MASK_UNICODE
};
use um::shlobj::CSIDL_FLAG_DONT_VERIFY;
use um::shtypes::{
    COMDLG_FILTERSPEC, DEVICE_SCALE_FACTOR, FOLDERTYPEID, KNOWNFOLDERID, PCIDLIST_ABSOLUTE,
    PCIDLIST_ABSOLUTE_ARRAY, PCUIDLIST_RELATIVE, PCUITEMID_CHILD, PCUITEMID_CHILD_ARRAY,
    PIDLIST_ABSOLUTE, PIDLIST_RELATIVE, PITEMID_CHILD, PUITEMID_CHILD, REFFOLDERTYPEID,
    REFKNOWNFOLDERID, REFTASKOWNERID, SHCOLSTATEF, SHCOLUMNID, SHELLDETAILS, STRRET
};
use um::structuredquerycondition::ICondition;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winbase::INFINITE;
use um::wingdi::LOGFONTW;
use um::winnt::{
    CHAR, HANDLE, HRESULT, LARGE_INTEGER, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, PWSTR, PZZWSTR,
    ULONGLONG, WCHAR
};
use um::winuser::{HDWP, MSG};
pub const CMF_NORMAL: UINT = 0x00000000;
pub const CMF_DEFAULTONLY: UINT = 0x00000001;
pub const CMF_VERBSONLY: UINT = 0x00000002;
pub const CMF_EXPLORE: UINT = 0x00000004;
pub const CMF_NOVERBS: UINT = 0x00000008;
pub const CMF_CANRENAME: UINT = 0x00000010;
pub const CMF_NODEFAULT: UINT = 0x00000020;
pub const CMF_ITEMMENU: UINT = 0x00000080;
pub const CMF_EXTENDEDVERBS: UINT = 0x00000100;
pub const CMF_DISABLEDVERBS: UINT = 0x00000200;
pub const CMF_ASYNCVERBSTATE: UINT = 0x00000400;
pub const CMF_OPTIMIZEFORINVOKE: UINT = 0x00000800;
pub const CMF_SYNCCASCADEMENU: UINT = 0x00001000;
pub const CMF_DONOTPICKDEFAULT: UINT = 0x00002000;
pub const CMF_RESERVED: UINT = 0xffff0000;
pub const GCS_VERBA: UINT = 0x00000000;
pub const GCS_HELPTEXTA: UINT = 0x00000001;
pub const GCS_VALIDATEA: UINT = 0x00000002;
pub const GCS_VERBW: UINT = 0x00000004;
pub const GCS_HELPTEXTW: UINT = 0x00000005;
pub const GCS_VALIDATEW: UINT = 0x00000006;
pub const GCS_VERBICONW: UINT = 0x00000014;
pub const GCS_UNICODE: UINT = 0x00000004;
// #define CMDSTR_NEWFOLDERA "NewFolder"
// #define CMDSTR_VIEWLISTA "ViewList"
// #define CMDSTR_VIEWDETAILSA "ViewDetails"
// #define CMDSTR_NEWFOLDERW L"NewFolder"
// #define CMDSTR_VIEWLISTW L"ViewList"
// #define CMDSTR_VIEWDETAILSW L"ViewDetails"
pub const CMIC_MASK_HOTKEY: ULONG = SEE_MASK_HOTKEY;
pub const CMIC_MASK_ICON: ULONG = SEE_MASK_ICON;
pub const CMIC_MASK_FLAG_NO_UI: ULONG = SEE_MASK_FLAG_NO_UI;
pub const CMIC_MASK_UNICODE: ULONG = SEE_MASK_UNICODE;
pub const CMIC_MASK_NO_CONSOLE: ULONG = SEE_MASK_NO_CONSOLE;
// pub const CMIC_MASK_FLAG_SEP_VDM: ULONG = SEE_MASK_FLAG_SEPVDM;
pub const CMIC_MASK_ASYNCOK: ULONG = SEE_MASK_ASYNCOK;
pub const CMIC_MASK_NOASYNC: ULONG = SEE_MASK_NOASYNC;
pub const CMIC_MASK_SHIFT_DOWN: ULONG = 0x10000000;
pub const CMIC_MASK_CONTROL_DOWN: ULONG = 0x40000000;
pub const CMIC_MASK_FLAG_LOG_USAGE: ULONG = SEE_MASK_FLAG_LOG_USAGE;
pub const CMIC_MASK_NOZONECHECKS: ULONG = SEE_MASK_NOZONECHECKS;
pub const CMIC_MASK_PTINVOKE: ULONG = 0x20000000;
STRUCT!{struct CMINVOKECOMMANDINFO {
    cbSize: DWORD,
    fMask: DWORD,
    hwnd: HWND,
    lpVerb: LPCSTR,
    lpParameters: LPCSTR,
    lpDirectory: LPCSTR,
    nShow: c_int,
    dwHotKey: DWORD,
    hIcon: HANDLE,
}}
pub type LPCMINVOKECOMMANDINFO = *mut CMINVOKECOMMANDINFO;
pub type PCCMINVOKECOMMANDINFO = *const CMINVOKECOMMANDINFO;
STRUCT!{struct CMINVOKECOMMANDINFOEX {
    cbSize: DWORD,
    fMask: DWORD,
    hwnd: HWND,
    lpVerb: LPCSTR,
    lpParameters: LPCSTR,
    lpDirectory: LPCSTR,
    nShow: c_int,
    dwHotKey: DWORD,
    hIcon: HANDLE,
    lpTitle: LPCSTR,
    lpVerbW: LPCWSTR,
    lpParametersW: LPCWSTR,
    lpDirectoryW: LPCWSTR,
    lpTitleW: LPCWSTR,
    ptInvoke: POINT,
}}
pub type LPCMINVOKECOMMANDINFOEX = *mut CMINVOKECOMMANDINFOEX;
pub type PCCMINVOKECOMMANDINFOEX = *const CMINVOKECOMMANDINFOEX;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContextMenu,
    0x000214e4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IContextMenu(IContextMenuVtbl): IUnknown(IUnknownVtbl) {
    fn QueryContextMenu(
        hmenu: HMENU,
        indexMenu: UINT,
        idCmdFirst: UINT,
        idCmdLast: UINT,
        uFlags: UINT,
    ) -> HRESULT,
    fn InvokeCommand(
        pici: *mut CMINVOKECOMMANDINFO,
    ) -> HRESULT,
    fn GetCommandString(
        idCmd: UINT_PTR,
        uType: UINT,
        pReserved: *mut UINT,
        pszName: *mut CHAR,
        cchMax: UINT,
    ) -> HRESULT,
}}
pub type LPCONTEXTMENU = *mut IContextMenu;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0001_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContextMenu2,
    0x000214f4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214f4, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IContextMenu2(IContextMenu2Vtbl): IContextMenu(IContextMenuVtbl) {
    fn HandleMenuMsg(
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
}}
pub type LPCONTEXTMENU2 = *mut IContextMenu2;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0002_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContextMenu3,
    0xbcfce0a0, 0xec17, 0x11d0, 0x8d, 0x10, 0x00, 0xa0, 0xc9, 0x0f, 0x27, 0x19}
RIDL!{#[uuid(0xbcfce0a0, 0xec17, 0x11d0, 0x8d, 0x10, 0x00, 0xa0, 0xc9, 0x0f, 0x27, 0x19)]
interface IContextMenu3(IContextMenu3Vtbl): IContextMenu2(IContextMenu2Vtbl) {
    fn HandleMenuMsg2(
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        plResult: *mut LRESULT,
    ) -> HRESULT,
}}
pub type LPCONTEXTMENU3 = *mut IContextMenu3;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0003_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExecuteCommand,
    0x7f9185b0, 0xcb92, 0x43c5, 0x80, 0xa9, 0x92, 0x27, 0x7a, 0x4f, 0x7b, 0x54}
RIDL!{#[uuid(0x7f9185b0, 0xcb92, 0x43c5, 0x80, 0xa9, 0x92, 0x27, 0x7a, 0x4f, 0x7b, 0x54)]
interface IExecuteCommand(IExecuteCommandVtbl): IUnknown(IUnknownVtbl) {
    fn SetKeyState(
        grfKeyState: DWORD,
    ) -> HRESULT,
    fn SetParameters(
        pszParameters: LPCWSTR,
    ) -> HRESULT,
    fn SetPosition(
        pt: POINT,
    ) -> HRESULT,
    fn SetShowWindow(
        nShow: c_int,
    ) -> HRESULT,
    fn SetNoShowUI(
        fNoShowUI: BOOL,
    ) -> HRESULT,
    fn SetDirectory(
        pszDirectory: LPCWSTR,
    ) -> HRESULT,
    fn Execute() -> HRESULT,
}}
DEFINE_GUID!{IID_IPersistFolder,
    0x000214ea, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214ea, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IPersistFolder(IPersistFolderVtbl): IPersist(IPersistVtbl) {
    fn Initialize(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
pub type LPPERSISTFOLDER = *mut IPersistFolder;
pub const IRTIR_TASK_NOT_RUNNING: ULONG = 0;
pub const IRTIR_TASK_RUNNING: ULONG = 1;
pub const IRTIR_TASK_SUSPENDED: ULONG = 2;
pub const IRTIR_TASK_PENDING: ULONG = 3;
pub const IRTIR_TASK_FINISHED: ULONG = 4;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0005_v0_0_s_ifspec;
DEFINE_GUID!{IID_IRunnableTask,
    0x85788d00, 0x6807, 0x11d0, 0xb8, 0x10, 0x00, 0xc0, 0x4f, 0xd7, 0x06, 0xec}
RIDL!{#[uuid(0x85788d00, 0x6807, 0x11d0, 0xb8, 0x10, 0x00, 0xc0, 0x4f, 0xd7, 0x06, 0xec)]
interface IRunnableTask(IRunnableTaskVtbl): IUnknown(IUnknownVtbl) {
    fn Run() -> HRESULT,
    fn Kill(
        bWait: BOOL,
    ) -> HRESULT,
    fn Suspend() -> HRESULT,
    fn Resume() -> HRESULT,
    fn IsRunning() -> ULONG,
}}
pub const TOID_NULL: GUID = GUID_NULL;
pub const ITSAT_DEFAULT_LPARAM: DWORD_PTR = -1isize as usize;
pub const ITSAT_DEFAULT_PRIORITY: DWORD = 0x10000000;
pub const ITSAT_MAX_PRIORITY: DWORD = 0x7fffffff;
pub const ITSAT_MIN_PRIORITY: DWORD = 0x00000000;
pub const ITSSFLAG_COMPLETE_ON_DESTROY: DWORD = 0x0000;
pub const ITSSFLAG_KILL_ON_DESTROY: DWORD = 0x0001;
pub const ITSSFLAG_FLAGS_MASK: DWORD = 0x0003;
pub const ITSS_THREAD_DESTROY_DEFAULT_TIMEOUT: DWORD = (10*1000);
pub const ITSS_THREAD_TERMINATE_TIMEOUT: DWORD = INFINITE;
pub const ITSS_THREAD_TIMEOUT_NO_CHANGE: DWORD = INFINITE - 1;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0006_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0006_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellTaskScheduler,
    0x6ccb7be0, 0x6807, 0x11d0, 0xb8, 0x10, 0x00, 0xc0, 0x4f, 0xd7, 0x06, 0xec}
RIDL!{#[uuid(0x6ccb7be0, 0x6807, 0x11d0, 0xb8, 0x10, 0x00, 0xc0, 0x4f, 0xd7, 0x06, 0xec)]
interface IShellTaskScheduler(IShellTaskSchedulerVtbl): IUnknown(IUnknownVtbl) {
    fn AddTask(
        prt: *mut IRunnableTask,
        rtoid: REFTASKOWNERID,
        lParam: DWORD_PTR,
        dwPriority: DWORD,
    ) -> HRESULT,
    fn RemoveTasks(
        rtoid: REFTASKOWNERID,
        lParam: DWORD_PTR,
        bWaitIfRunning: BOOL,
    ) -> HRESULT,
    fn CountTasks(
        rtoid: REFTASKOWNERID,
    ) -> UINT,
    fn Status(
        dwReleaseStatus: DWORD,
        dwThreadTimeout: DWORD,
    ) -> HRESULT,
}}
// #define SID_ShellTaskScheduler IID_IShellTaskScheduler
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0007_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0007_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPersistFolder2,
    0x1ac3d9f0, 0x175c, 0x11d1, 0x95, 0xbe, 0x00, 0x60, 0x97, 0x97, 0xea, 0x4f}
RIDL!{#[uuid(0x1ac3d9f0, 0x175c, 0x11d1, 0x95, 0xbe, 0x00, 0x60, 0x97, 0x97, 0xea, 0x4f)]
interface IPersistFolder2(IPersistFolder2Vtbl): IPersistFolder(IPersistFolderVtbl) {
    fn GetCurFolder(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
pub const CSIDL_FLAG_PFTI_TRACKTARGET: c_int = CSIDL_FLAG_DONT_VERIFY;
STRUCT!{struct PERSIST_FOLDER_TARGET_INFO {
    pidlTargetFolder: PIDLIST_ABSOLUTE,
    szTargetParsingName: [WCHAR; 260],
    szNetworkProvider: [WCHAR; 260],
    dwAttributes: DWORD,
    csidl: c_int,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0008_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0008_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPersistFolder3,
    0xcef04fdf, 0xfe72, 0x11d2, 0x87, 0xa5, 0x00, 0xc0, 0x4f, 0x68, 0x37, 0xcf}
RIDL!{#[uuid(0xcef04fdf, 0xfe72, 0x11d2, 0x87, 0xa5, 0x00, 0xc0, 0x4f, 0x68, 0x37, 0xcf)]
interface IPersistFolder3(IPersistFolder3Vtbl): IPersistFolder2(IPersistFolder2Vtbl) {
    fn InitializeEx(
        pbc: *mut IBindCtx,
        pidlRoot: PCIDLIST_ABSOLUTE,
        ppfti: *const PERSIST_FOLDER_TARGET_INFO,
    ) -> HRESULT,
    fn GetFolderTargetInfo(
        ppfti: *mut PERSIST_FOLDER_TARGET_INFO,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0009_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0009_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPersistIDList,
    0x1079acfc, 0x29bd, 0x11d3, 0x8e, 0x0d, 0x00, 0xc0, 0x4f, 0x68, 0x37, 0xd5}
RIDL!{#[uuid(0x1079acfc, 0x29bd, 0x11d3, 0x8e, 0x0d, 0x00, 0xc0, 0x4f, 0x68, 0x37, 0xd5)]
interface IPersistIDList(IPersistIDListVtbl): IPersist(IPersistVtbl) {
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0010_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0010_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumIDList,
    0x000214f2, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214f2, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumIDList(IEnumIDListVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut PITEMID_CHILD,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumIDList,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumIDList_RemoteNext_Proxy(
//     IEnumIDList * This,
//     ULONG celt,
//     PITEMID_CHILD *rgelt,
//     ULONG *pceltFetched);
// c_void __RPC_STUB IEnumIDList_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPENUMIDLIST = *mut IEnumIDList;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0011_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0011_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumFullIDList,
    0xd0191542, 0x7954, 0x4908, 0xbc, 0x06, 0xb2, 0x36, 0x0b, 0xbe, 0x45, 0xba}
RIDL!{#[uuid(0xd0191542, 0x7954, 0x4908, 0xbc, 0x06, 0xb2, 0x36, 0x0b, 0xbe, 0x45, 0xba)]
interface IEnumFullIDList(IEnumFullIDListVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut PIDLIST_ABSOLUTE,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumFullIDList,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumFullIDList_RemoteNext_Proxy(
//     IEnumFullIDList * This,
//     ULONG celt,
//     PIDLIST_ABSOLUTE *rgelt,
//     ULONG *pceltFetched);
// c_void __RPC_STUB IEnumFullIDList_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
ENUM!{enum _SHGDNF {
    SHGDN_NORMAL = 0,
    SHGDN_INFOLDER = 0x1,
    SHGDN_FOREDITING = 0x1000,
    SHGDN_FORADDRESSBAR = 0x4000,
    SHGDN_FORPARSING = 0x8000,
}}
pub type SHGDNF = DWORD;
ENUM!{enum _SHCONTF {
    SHCONTF_CHECKING_FOR_CHILDREN = 0x10,
    SHCONTF_FOLDERS = 0x20,
    SHCONTF_NONFOLDERS = 0x40,
    SHCONTF_INCLUDEHIDDEN = 0x80,
    SHCONTF_INIT_ON_FIRST_NEXT = 0x100,
    SHCONTF_NETPRINTERSRCH = 0x200,
    SHCONTF_SHAREABLE = 0x400,
    SHCONTF_STORAGE = 0x800,
    SHCONTF_NAVIGATION_ENUM = 0x1000,
    SHCONTF_FASTITEMS = 0x2000,
    SHCONTF_FLATLIST = 0x4000,
    SHCONTF_ENABLE_ASYNC = 0x8000,
    SHCONTF_INCLUDESUPERHIDDEN = 0x10000,
}}
pub type SHCONTF = DWORD;
pub const SHCIDS_ALLFIELDS: LPARAM = 0x80000000;
pub const SHCIDS_CANONICALONLY: LPARAM = 0x10000000;
pub const SHCIDS_BITMASK: LPARAM = 0xFFFF0000;
pub const SHCIDS_COLUMNMASK: LPARAM = 0x0000FFFF;
pub const SFGAO_CANCOPY: ULONG = DROPEFFECT_COPY;
pub const SFGAO_CANMOVE: ULONG = DROPEFFECT_MOVE;
pub const SFGAO_CANLINK: ULONG = DROPEFFECT_LINK;
pub const SFGAO_STORAGE: ULONG = 0x00000008;
pub const SFGAO_CANRENAME: ULONG = 0x00000010;
pub const SFGAO_CANDELETE: ULONG = 0x00000020;
pub const SFGAO_HASPROPSHEET: ULONG = 0x00000040;
pub const SFGAO_DROPTARGET: ULONG = 0x00000100;
pub const SFGAO_CAPABILITYMASK: ULONG = 0x00000177;
pub const SFGAO_PLACEHOLDER: ULONG = 0x00000800;
pub const SFGAO_SYSTEM: ULONG = 0x00001000;
pub const SFGAO_ENCRYPTED: ULONG = 0x00002000;
pub const SFGAO_ISSLOW: ULONG = 0x00004000;
pub const SFGAO_GHOSTED: ULONG = 0x00008000;
pub const SFGAO_LINK: ULONG = 0x00010000;
pub const SFGAO_SHARE: ULONG = 0x00020000;
pub const SFGAO_READONLY: ULONG = 0x00040000;
pub const SFGAO_HIDDEN: ULONG = 0x00080000;
pub const SFGAO_DISPLAYATTRMASK: ULONG = 0x000FC000;
pub const SFGAO_FILESYSANCESTOR: ULONG = 0x10000000;
pub const SFGAO_FOLDER: ULONG = 0x20000000;
pub const SFGAO_FILESYSTEM: ULONG = 0x40000000;
pub const SFGAO_HASSUBFOLDER: ULONG = 0x80000000;
pub const SFGAO_CONTENTSMASK: ULONG = 0x80000000;
pub const SFGAO_VALIDATE: ULONG = 0x01000000;
pub const SFGAO_REMOVABLE: ULONG = 0x02000000;
pub const SFGAO_COMPRESSED: ULONG = 0x04000000;
pub const SFGAO_BROWSABLE: ULONG = 0x08000000;
pub const SFGAO_NONENUMERATED: ULONG = 0x00100000;
pub const SFGAO_NEWCONTENT: ULONG = 0x00200000;
pub const SFGAO_CANMONIKER: ULONG = 0x00400000;
pub const SFGAO_HASSTORAGE: ULONG = 0x00400000;
pub const SFGAO_STREAM: ULONG = 0x00400000;
pub const SFGAO_STORAGEANCESTOR: ULONG = 0x00800000;
pub const SFGAO_STORAGECAPMASK: ULONG = 0x70C50008;
pub const SFGAO_PKEYSFGAOMASK: ULONG = 0x81044000;
pub type SFGAOF = ULONG;
ENUM!{enum SYNC_TRANSFER_STATUS {
    STS_NONE = 0,
    STS_NEEDSUPLOAD = 0x1,
    STS_NEEDSDOWNLOAD = 0x2,
    STS_TRANSFERRING = 0x4,
    STS_PAUSED = 0x8,
    STS_HASERROR = 0x10,
    STS_FETCHING_METADATA = 0x20,
    STS_USER_REQUESTED_REFRESH = 0x40,
    STS_HASWARNING = 0x80,
    STS_EXCLUDED = 0x100,
    STS_INCOMPLETE = 0x200,
}}
ENUM!{enum STORAGE_PROVIDER_FILE_FLAGS {
    SPFF_NONE = 0,
    SPFF_DOWNLOAD_BY_DEFAULT = 0x1,
    SPFF_CREATED_ON_THIS_DEVICE = 0x2,
}}
ENUM!{enum PLACEHOLDER_STATES {
    PS_NONE = 0,
    PS_MARKED_FOR_OFFLINE_AVAILABILITY = 0x1,
    PS_FULL_PRIMARY_STREAM_AVAILABLE = 0x2,
    PS_CREATE_FILE_ACCESSIBLE = 0x4,
    PS_CLOUDFILE_PLACEHOLDER = 0x8,
    PS_DEFAULT = PS_MARKED_FOR_OFFLINE_AVAILABILITY | PS_FULL_PRIMARY_STREAM_AVAILABLE
        | PS_CREATE_FILE_ACCESSIBLE,
    PS_ALL = PS_MARKED_FOR_OFFLINE_AVAILABILITY | PS_FULL_PRIMARY_STREAM_AVAILABLE
        | PS_CREATE_FILE_ACCESSIBLE | PS_CLOUDFILE_PLACEHOLDER,
}}
// #define CONFLICT_RESOLUTION_CLSID_KEY L"ConflictResolutionCLSID"
ENUM!{enum MERGE_UPDATE_STATUS {
    MUS_COMPLETE = 0,
    MUS_USERINPUTNEEDED = MUS_COMPLETE + 1,
    MUS_FAILED = MUS_USERINPUTNEEDED + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0012_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0012_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileSyncMergeHandler,
    0xd97b5aac, 0xc792, 0x433c, 0x97, 0x5d, 0x35, 0xc4, 0xea, 0xdc, 0x7a, 0x9d}
RIDL!{#[uuid(0xd97b5aac, 0xc792, 0x433c, 0x97, 0x5d, 0x35, 0xc4, 0xea, 0xdc, 0x7a, 0x9d)]
interface IFileSyncMergeHandler(IFileSyncMergeHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn Merge(
        localFilePath: LPCWSTR,
        serverFilePath: LPCWSTR,
        updateStatus: *mut MERGE_UPDATE_STATUS,
    ) -> HRESULT,
    fn ShowResolveConflictUIAsync(
        localFilePath: LPCWSTR,
        monitorToDisplayOn: HMONITOR,
    ) -> HRESULT,
}}
// #define STR_BIND_FORCE_FOLDER_SHORTCUT_RESOLVE L"Force Folder Shortcut Resolve"
// #define STR_AVOID_DRIVE_RESTRICTION_POLICY L"Avoid Drive Restriction Policy"
// #define STR_AVOID_DRIVE_RESTRICTION_POLICY L"Avoid Drive Restriction Policy"
// #define STR_SKIP_BINDING_CLSID L"Skip Binding CLSID"
// #define STR_PARSE_PREFER_FOLDER_BROWSING L"Parse Prefer Folder Browsing"
// #define STR_DONT_PARSE_RELATIVE L"Don't Parse Relative"
// #define STR_PARSE_TRANSLATE_ALIASES L"Parse Translate Aliases"
// #define STR_PARSE_SKIP_NET_CACHE L"Skip Net Resource Cache"
// #define STR_PARSE_SHELL_PROTOCOL_TO_FILE_OBJECTS L"Parse Shell Protocol To File Objects"
// #define STR_TRACK_CLSID L"Track the CLSID"
// #define STR_INTERNAL_NAVIGATE L"Internal Navigation"
// #define STR_PARSE_PROPERTYSTORE L"DelegateNamedProperties"
// #define STR_NO_VALIDATE_FILENAME_CHARS L"NoValidateFilenameChars"
// #define STR_BIND_DELEGATE_CREATE_OBJECT L"Delegate Object Creation"
// #define STR_PARSE_ALLOW_INTERNET_SHELL_FOLDERS L"Allow binding to Internet shell folder handlers and negate STR_PARSE_PREFER_WEB_BROWSING"
// #define STR_PARSE_PREFER_WEB_BROWSING L"Do not bind to Internet shell folder handlers"
// #define STR_PARSE_SHOW_NET_DIAGNOSTICS_UI L"Show network diagnostics UI"
// #define STR_PARSE_DONT_REQUIRE_VALIDATED_URLS L"Do not require validated URLs"
// #define STR_INTERNETFOLDER_PARSE_ONLY_URLMON_BINDABLE L"Validate URL"
pub const BIND_INTERRUPTABLE: DWORD = 0xFFFFFFFF;
// #define STR_BIND_FOLDERS_READ_ONLY L"Folders As Read Only"
// #define STR_BIND_FOLDER_ENUM_MODE L"Folder Enum Mode"
ENUM!{enum FOLDER_ENUM_MODE {
    FEM_VIEWRESULT = 0,
    FEM_NAVIGATION = 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0013_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0013_v0_0_s_ifspec;
DEFINE_GUID!{IID_IObjectWithFolderEnumMode,
    0x6a9d9026, 0x0e6e, 0x464c, 0xb0, 0x00, 0x42, 0xec, 0xc0, 0x7d, 0xe6, 0x73}
RIDL!{#[uuid(0x6a9d9026, 0x0e6e, 0x464c, 0xb0, 0x00, 0x42, 0xec, 0xc0, 0x7d, 0xe6, 0x73)]
interface IObjectWithFolderEnumMode(IObjectWithFolderEnumModeVtbl): IUnknown(IUnknownVtbl) {
    fn SetMode(
        feMode: FOLDER_ENUM_MODE,
    ) -> HRESULT,
    fn GetMode(
        pfeMode: *mut FOLDER_ENUM_MODE,
    ) -> HRESULT,
}}
// #define STR_PARSE_WITH_EXPLICIT_PROGID L"ExplicitProgid"
// #define STR_PARSE_WITH_EXPLICIT_ASSOCAPP L"ExplicitAssociationApp"
// #define STR_PARSE_EXPLICIT_ASSOCIATION_SUCCESSFUL L"ExplicitAssociationSuccessful"
// #define STR_PARSE_AND_CREATE_ITEM L"ParseAndCreateItem"
// #define STR_PROPERTYBAG_PARAM L"SHBindCtxPropertyBag"
// #define STR_ENUM_ITEMS_FLAGS L"SHCONTF"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0014_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0014_v0_0_s_ifspec;
DEFINE_GUID!{IID_IParseAndCreateItem,
    0x67efed0e, 0xe827, 0x4408, 0xb4, 0x93, 0x78, 0xf3, 0x98, 0x2b, 0x68, 0x5c}
RIDL!{#[uuid(0x67efed0e, 0xe827, 0x4408, 0xb4, 0x93, 0x78, 0xf3, 0x98, 0x2b, 0x68, 0x5c)]
interface IParseAndCreateItem(IParseAndCreateItemVtbl): IUnknown(IUnknownVtbl) {
    fn SetItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetItem(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
// #define STR_ITEM_CACHE_CONTEXT L"ItemCacheContext"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0015_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0015_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellFolder,
    0x000214e6, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e6, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellFolder(IShellFolderVtbl): IUnknown(IUnknownVtbl) {
    fn ParseDisplayName(
        hwnd: HWND,
        pbc: *mut IBindCtx,
        pszDisplayName: LPWSTR,
        pchEaten: *mut ULONG,
        ppidl: *mut PIDLIST_RELATIVE,
        pdwAttributes: *mut ULONG,
    ) -> HRESULT,
    fn EnumObjects(
        hwnd: HWND,
        grfFlags: SHCONTF,
        ppenumIDList: *mut *mut IEnumIDList,
    ) -> HRESULT,
    fn BindToObject(
        pidl: PCUIDLIST_RELATIVE,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn BindToStorage(
        pidl: PCUIDLIST_RELATIVE,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CompareIDs(
        lParam: LPARAM,
        pidl1: PCUIDLIST_RELATIVE,
        pidl2: PCUIDLIST_RELATIVE,
    ) -> HRESULT,
    fn CreateViewObject(
        hwndOwner: HWND,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetAttributesOf(
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        rgfInOut: *mut SFGAOF,
    ) -> HRESULT,
    fn GetUIObjectOf(
        hwndOwner: HWND,
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        riid: REFIID,
        rgfReserved: *mut UINT,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetDisplayNameOf(
        pidl: PCUITEMID_CHILD,
        uFlags: SHGDNF,
        pName: *mut STRRET,
    ) -> HRESULT,
    fn SetNameOf(
        hwnd: HWND,
        pidl: PCUITEMID_CHILD,
        pszName: LPCWSTR,
        uFlags: SHGDNF,
        ppidlOut: *mut PITEMID_CHILD,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IShellFolder_RemoteSetNameOf_Proxy(
//     IShellFolder * This,
//     HWND hwnd,
//     PCUITEMID_CHILD pidl,
//     LPCWSTR pszName,
//     SHGDNF uFlags,
//     PITEMID_CHILD *ppidlOut);
// c_void __RPC_STUB IShellFolder_RemoteSetNameOf_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPSHELLFOLDER = *mut IShellFolder;
STRUCT!{struct EXTRASEARCH {
    guidSearch: GUID,
    wszFriendlyName: [WCHAR; 80],
    wszUrl: [WCHAR; 2084],
}}
pub type LPEXTRASEARCH = *mut EXTRASEARCH;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0016_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0016_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumExtraSearch,
    0x0e700be1, 0x9db6, 0x11d1, 0xa1, 0xce, 0x00, 0xc0, 0x4f, 0xd7, 0x5d, 0x13}
RIDL!{#[uuid(0x0e700be1, 0x9db6, 0x11d1, 0xa1, 0xce, 0x00, 0xc0, 0x4f, 0xd7, 0x5d, 0x13)]
interface IEnumExtraSearch(IEnumExtraSearchVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut EXTRASEARCH,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumExtraSearch,
    ) -> HRESULT,
}}
pub type LPENUMEXTRASEARCH = *mut IEnumExtraSearch;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0017_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0017_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellFolder2,
    0x93f2f68c, 0x1d1b, 0x11d3, 0xa3, 0x0e, 0x00, 0xc0, 0x4f, 0x79, 0xab, 0xd1}
RIDL!{#[uuid(0x93f2f68c, 0x1d1b, 0x11d3, 0xa3, 0x0e, 0x00, 0xc0, 0x4f, 0x79, 0xab, 0xd1)]
interface IShellFolder2(IShellFolder2Vtbl): IShellFolder(IShellFolderVtbl) {
    fn GetDefaultSearchGUID(
        pguid: *mut GUID,
    ) -> HRESULT,
    fn EnumSearches(
        ppenum: *mut *mut IEnumExtraSearch,
    ) -> HRESULT,
    fn GetDefaultColumn(
        dwRes: DWORD,
        pSort: *mut ULONG,
        pDisplay: *mut ULONG,
    ) -> HRESULT,
    fn GetDefaultColumnState(
        iColumn: UINT,
        pcsFlags: *mut SHCOLSTATEF,
    ) -> HRESULT,
    fn GetDetailsEx(
        pidl: PCUITEMID_CHILD,
        pscid: *const SHCOLUMNID,
        pv: *mut VARIANT,
    ) -> HRESULT,
    fn GetDetailsOf(
        pidl: PCUITEMID_CHILD,
        iColumn: UINT,
        psd: *mut SHELLDETAILS,
    ) -> HRESULT,
    fn MapColumnToSCID(
        iColumn: UINT,
        pscid: *mut SHCOLUMNID,
    ) -> HRESULT,
}}
ENUM!{enum FOLDERFLAGS {
    FWF_NONE = 0,
    FWF_AUTOARRANGE = 0x1,
    FWF_ABBREVIATEDNAMES = 0x2,
    FWF_SNAPTOGRID = 0x4,
    FWF_OWNERDATA = 0x8,
    FWF_BESTFITWINDOW = 0x10,
    FWF_DESKTOP = 0x20,
    FWF_SINGLESEL = 0x40,
    FWF_NOSUBFOLDERS = 0x80,
    FWF_TRANSPARENT = 0x100,
    FWF_NOCLIENTEDGE = 0x200,
    FWF_NOSCROLL = 0x400,
    FWF_ALIGNLEFT = 0x800,
    FWF_NOICONS = 0x1000,
    FWF_SHOWSELALWAYS = 0x2000,
    FWF_NOVISIBLE = 0x4000,
    FWF_SINGLECLICKACTIVATE = 0x8000,
    FWF_NOWEBVIEW = 0x10000,
    FWF_HIDEFILENAMES = 0x20000,
    FWF_CHECKSELECT = 0x40000,
    FWF_NOENUMREFRESH = 0x80000,
    FWF_NOGROUPING = 0x100000,
    FWF_FULLROWSELECT = 0x200000,
    FWF_NOFILTERS = 0x400000,
    FWF_NOCOLUMNHEADER = 0x800000,
    FWF_NOHEADERINALLVIEWS = 0x1000000,
    FWF_EXTENDEDTILES = 0x2000000,
    FWF_TRICHECKSELECT = 0x4000000,
    FWF_AUTOCHECKSELECT = 0x8000000,
    FWF_NOBROWSERVIEWSTATE = 0x10000000,
    FWF_SUBSETGROUPS = 0x20000000,
    FWF_USESEARCHFOLDER = 0x40000000,
    FWF_ALLOWRTLREADING = 0x80000000,
}}
ENUM!{enum FOLDERVIEWMODE {
    FVM_AUTO = -1i32 as u32,
    FVM_FIRST = 1,
    FVM_ICON = 1,
    FVM_SMALLICON = 2,
    FVM_LIST = 3,
    FVM_DETAILS = 4,
    FVM_THUMBNAIL = 5,
    FVM_TILE = 6,
    FVM_THUMBSTRIP = 7,
    FVM_CONTENT = 8,
    FVM_LAST = 8,
}}
ENUM!{enum FOLDERLOGICALVIEWMODE {
    FLVM_UNSPECIFIED = -1i32 as u32,
    FLVM_FIRST = 1,
    FLVM_DETAILS = 1,
    FLVM_TILES = 2,
    FLVM_ICONS = 3,
    FLVM_LIST = 4,
    FLVM_CONTENT = 5,
    FLVM_LAST = 5,
}}
STRUCT!{struct FOLDERSETTINGS {
    ViewMode: UINT,
    fFlags: UINT,
}}
pub type LPFOLDERSETTINGS = *mut FOLDERSETTINGS;
pub type LPCFOLDERSETTINGS = *const FOLDERSETTINGS;
pub type PFOLDERSETTINGS = *mut FOLDERSETTINGS;
ENUM!{enum _SVSIF {
    SVSI_DESELECT = 0,
    SVSI_SELECT = 0x1,
    SVSI_EDIT = 0x3,
    SVSI_DESELECTOTHERS = 0x4,
    SVSI_ENSUREVISIBLE = 0x8,
    SVSI_FOCUSED = 0x10,
    SVSI_TRANSLATEPT = 0x20,
    SVSI_SELECTIONMARK = 0x40,
    SVSI_POSITIONITEM = 0x80,
    SVSI_CHECK = 0x100,
    SVSI_CHECK2 = 0x200,
    SVSI_KEYBOARDSELECT = 0x401,
    SVSI_NOTAKEFOCUS = 0x40000000,
}}
pub const SVSI_NOSTATECHANGE: UINT = 0x80000000;
pub type SVSIF = UINT;
ENUM!{enum _SVGIO {
    SVGIO_BACKGROUND = 0,
    SVGIO_SELECTION = 0x1,
    SVGIO_ALLVIEW = 0x2,
    SVGIO_CHECKED = 0x3,
    SVGIO_TYPE_MASK = 0xf,
    SVGIO_FLAG_VIEWORDER = 0x80000000,
}}
pub type SVGIO = c_int;
ENUM!{enum SVUIA_STATUS {
    SVUIA_DEACTIVATE = 0,
    SVUIA_ACTIVATE_NOFOCUS = 1,
    SVUIA_ACTIVATE_FOCUS = 2,
    SVUIA_INPLACEACTIVATE = 3,
}}
pub type LPFNSVADDPROPSHEETPAGE = LPFNADDPROPSHEETPAGE;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0018_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0018_v0_0_s_ifspec;
pub type LPSHELLVIEW = *mut IShellView;
DEFINE_GUID!{IID_IShellView,
    0x000214e3, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e3, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellView(IShellViewVtbl): IOleWindow(IOleWindowVtbl) {
    fn TranslateAccelerator(
        pmsg: *mut MSG,
    ) -> HRESULT,
    fn EnableModeless(
        fEnable: BOOL,
    ) -> HRESULT,
    fn UIActivate(
        uState: UINT,
    ) -> HRESULT,
    fn Refresh() -> HRESULT,
    fn CreateViewWindow(
        psvPrevious: *mut IShellView,
        pfs: LPCFOLDERSETTINGS,
        psb: *mut IShellBrowser,
        prcView: *mut RECT,
        phWnd: *mut HWND,
    ) -> HRESULT,
    fn DestroyViewWindow() -> HRESULT,
    fn GetCurrentInfo(
        pfs: LPFOLDERSETTINGS,
    ) -> HRESULT,
    fn AddPropertySheetPages(
        dwReserved: DWORD,
        pfn: LPFNSVADDPROPSHEETPAGE,
        lparam: LPARAM,
    ) -> HRESULT,
    fn SaveViewState() -> HRESULT,
    fn SelectItem(
        pidlItem: PCUITEMID_CHILD,
        uFlags: SVSIF,
    ) -> HRESULT,
    fn GetItemObject(
        uItem: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
pub type SHELLVIEWID = GUID;
pub const SV2GV_CURRENTVIEW: UINT = -1i32 as u32;
pub const SV2GV_DEFAULTVIEW: UINT = -2i32 as u32;
STRUCT!{struct SV2CVW2_PARAMS {
    cbSize: DWORD,
    psvPrev: *mut IShellView,
    pfs: LPCFOLDERSETTINGS,
    psbOwner: *mut IShellBrowser,
    prcView: *mut RECT,
    pvid: *const SHELLVIEWID,
    hwndView: HWND,
}}
pub type LPSV2CVW2_PARAMS = *mut SV2CVW2_PARAMS;
DEFINE_GUID!{IID_IShellView2,
    0x88e39e80, 0x3578, 0x11cf, 0xae, 0x69, 0x08, 0x00, 0x2b, 0x2e, 0x12, 0x62}
RIDL!{#[uuid(0x88e39e80, 0x3578, 0x11cf, 0xae, 0x69, 0x08, 0x00, 0x2b, 0x2e, 0x12, 0x62)]
interface IShellView2(IShellView2Vtbl): IShellView(IShellViewVtbl) {
    fn GetView(
        pvid: *mut SHELLVIEWID,
        uView: ULONG,
    ) -> HRESULT,
    fn CreateViewWindow2(
        lpParams: LPSV2CVW2_PARAMS,
    ) -> HRESULT,
    fn HandleRename(
        pidlNew: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn SelectAndPositionItem(
        pidlItem: PCUITEMID_CHILD,
        uFlags: UINT,
        ppt: *mut POINT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0020_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0020_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderView,
    0xcde725b0, 0xccc9, 0x4519, 0x91, 0x7e, 0x32, 0x5d, 0x72, 0xfa, 0xb4, 0xce}
RIDL!{#[uuid(0xcde725b0, 0xccc9, 0x4519, 0x91, 0x7e, 0x32, 0x5d, 0x72, 0xfa, 0xb4, 0xce)]
interface IFolderView(IFolderViewVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentViewMode(
        pViewMode: *mut UINT,
    ) -> HRESULT,
    fn SetCurrentViewMode(
        ViewMode: UINT,
    ) -> HRESULT,
    fn GetFolder(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn Item(
        iItemIndex: c_int,
        ppidl: *mut PITEMID_CHILD,
    ) -> HRESULT,
    fn ItemCount(
        uFlags: UINT,
        pcItems: *mut c_int,
    ) -> HRESULT,
    fn Items(
        uFlags: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetSelectionMarkedItem(
        piItem: *mut c_int,
    ) -> HRESULT,
    fn GetFocusedItem(
        piItem: *mut c_int,
    ) -> HRESULT,
    fn GetItemPosition(
        pidl: PCUITEMID_CHILD,
        ppt: *mut POINT,
    ) -> HRESULT,
    fn GetSpacing(
        ppt: *mut POINT,
    ) -> HRESULT,
    fn GetDefaultSpacing(
        ppt: *mut POINT,
    ) -> HRESULT,
    fn GetAutoArrange() -> HRESULT,
    fn SelectItem(
        iItem: c_int,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SelectAndPositionItems(
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        apt: *mut POINT,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
// #define SID_SFolderView IID_IFolderView
ENUM!{enum tagSORTDIRECTION {
    SORT_DESCENDING = -1i32 as u32,
    SORT_ASCENDING = 1,
}}
pub type SORTDIRECTION = c_int;
STRUCT!{struct SORTCOLUMN {
    propkey: PROPERTYKEY,
    direction: SORTDIRECTION,
}}
ENUM!{enum FVTEXTTYPE {
    FVST_EMPTYTEXT = 0,
}}
pub type DEPRECATED_HRESULT = HRESULT;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0021_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0021_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderView2,
    0x1af3a467, 0x214f, 0x4298, 0x90, 0x8e, 0x06, 0xb0, 0x3e, 0x0b, 0x39, 0xf9}
RIDL!{#[uuid(0x1af3a467, 0x214f, 0x4298, 0x90, 0x8e, 0x06, 0xb0, 0x3e, 0x0b, 0x39, 0xf9)]
interface IFolderView2(IFolderView2Vtbl): IFolderView(IFolderViewVtbl) {
    fn SetGroupBy(
        key: REFPROPERTYKEY,
        fAscending: BOOL,
    ) -> HRESULT,
    fn GetGroupBy(
        pkey: *mut PROPERTYKEY,
        pfAscending: *mut BOOL,
    ) -> HRESULT,
    fn SetViewProperty(
        pidl: PCUITEMID_CHILD,
        propkey: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
    ) -> DEPRECATED_HRESULT,
    fn GetViewProperty(
        pidl: PCUITEMID_CHILD,
        propkey: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
    ) -> DEPRECATED_HRESULT,
    fn SetTileViewProperties(
        pidl: PCUITEMID_CHILD,
        pszPropList: LPCWSTR,
    ) -> DEPRECATED_HRESULT,
    fn SetExtendedTileViewProperties(
        pidl: PCUITEMID_CHILD,
        pszPropList: LPCWSTR,
    ) -> DEPRECATED_HRESULT,
    fn SetText(
        iType: FVTEXTTYPE,
        pwszText: LPCWSTR,
    ) -> HRESULT,
    fn SetCurrentFolderFlags(
        dwMask: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCurrentFolderFlags(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetSortColumnCount(
        pcColumns: *mut c_int,
    ) -> HRESULT,
    fn SetSortColumns(
        rgSortColumns: *const SORTCOLUMN,
        cColumns: c_int,
    ) -> HRESULT,
    fn GetSortColumns(
        rgSortColumns: *mut SORTCOLUMN,
        cColumns: c_int,
    ) -> HRESULT,
    fn GetItem(
        iItem: c_int,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetVisibleItem(
        iStart: c_int,
        fPrevious: BOOL,
        piItem: *mut c_int,
    ) -> HRESULT,
    fn GetSelectedItem(
        iStart: c_int,
        piItem: *mut c_int,
    ) -> HRESULT,
    fn GetSelection(
        fNoneImpliesFolder: BOOL,
        ppsia: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSelectionState(
        pidl: PCUITEMID_CHILD,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn InvokeVerbOnSelection(
        pszVerb: LPCSTR,
    ) -> HRESULT,
    fn SetViewModeAndIconSize(
        uViewMode: FOLDERVIEWMODE,
        iImageSize: c_int,
    ) -> HRESULT,
    fn GetViewModeAndIconSize(
        puViewMode: *mut FOLDERVIEWMODE,
        piImageSize: *mut c_int,
    ) -> HRESULT,
    fn SetGroupSubsetCount(
        cVisibleRows: UINT,
    ) -> HRESULT,
    fn GetGroupSubsetCount(
        pcVisibleRows: *mut UINT,
    ) -> HRESULT,
    fn SetRedraw(
        fRedrawOn: BOOL,
    ) -> HRESULT,
    fn IsMoveInSameFolder() -> HRESULT,
    fn DoRename() -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IFolderView2_RemoteGetGroupBy_Proxy(
//     IFolderView2 * This,
//     PROPERTYKEY *pkey,
//     BOOL *pfAscending);
// c_void __RPC_STUB IFolderView2_RemoteGetGroupBy_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0022_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0022_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderViewSettings,
    0xae8c987d, 0x8797, 0x4ed3, 0xbe, 0x72, 0x2a, 0x47, 0xdd, 0x93, 0x8d, 0xb0}
RIDL!{#[uuid(0xae8c987d, 0x8797, 0x4ed3, 0xbe, 0x72, 0x2a, 0x47, 0xdd, 0x93, 0x8d, 0xb0)]
interface IFolderViewSettings(IFolderViewSettingsVtbl): IUnknown(IUnknownVtbl) {
    fn GetColumnPropertyList(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetGroupByProperty(
        pkey: *mut PROPERTYKEY,
        pfGroupAscending: *mut BOOL,
    ) -> HRESULT,
    fn GetViewMode(
        plvm: *mut FOLDERLOGICALVIEWMODE,
    ) -> HRESULT,
    fn GetIconSize(
        puIconSize: *mut UINT,
    ) -> HRESULT,
    fn GetFolderFlags(
        pfolderMask: *mut FOLDERFLAGS,
        pfolderFlags: *mut FOLDERFLAGS,
    ) -> HRESULT,
    fn GetSortColumns(
        rgSortColumns: *mut SORTCOLUMN,
        cColumnsIn: UINT,
        pcColumnsOut: *mut UINT,
    ) -> HRESULT,
    fn GetGroupSubsetCount(
        pcVisibleRows: *mut UINT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0023_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0023_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPreviewHandlerVisuals,
    0x196bf9a5, 0xb346, 0x4ef0, 0xaa, 0x1e, 0x5d, 0xcd, 0xb7, 0x67, 0x68, 0xb1}
RIDL!{#[uuid(0x196bf9a5, 0xb346, 0x4ef0, 0xaa, 0x1e, 0x5d, 0xcd, 0xb7, 0x67, 0x68, 0xb1)]
interface IPreviewHandlerVisuals(IPreviewHandlerVisualsVtbl): IUnknown(IUnknownVtbl) {
    fn SetBackgroundColor(
        color: COLORREF,
    ) -> HRESULT,
    fn SetFont(
        plf: *const LOGFONTW,
    ) -> HRESULT,
    fn SetTextColor(
        color: COLORREF,
    ) -> HRESULT,
}}
pub const CDBOSC_SETFOCUS: ULONG = 0x00000000;
pub const CDBOSC_KILLFOCUS: ULONG = 0x00000001;
pub const CDBOSC_SELCHANGE: ULONG = 0x00000002;
pub const CDBOSC_RENAME: ULONG = 0x00000003;
pub const CDBOSC_STATECHANGE: ULONG = 0x00000004;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0024_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0024_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICommDlgBrowser,
    0x000214f1, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214f1, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface ICommDlgBrowser(ICommDlgBrowserVtbl): IUnknown(IUnknownVtbl) {
    fn OnDefaultCommand(
        ppshv: *mut IShellView,
    ) -> HRESULT,
    fn OnStateChange(
        ppshv: *mut IShellView,
        uChange: ULONG,
    ) -> HRESULT,
    fn IncludeObject(
        ppshv: *mut IShellView,
        pidl: PCUITEMID_CHILD,
    ) -> HRESULT,
}}
pub type LPCOMMDLGBROWSER = *mut ICommDlgBrowser;
// #define SID_SExplorerBrowserFrame IID_ICommDlgBrowser
pub const CDB2N_CONTEXTMENU_DONE: DWORD = 0x00000001;
pub const CDB2N_CONTEXTMENU_START: DWORD = 0x00000002;
pub const CDB2GVF_SHOWALLFILES: DWORD = 0x00000001;
pub const CDB2GVF_ISFILESAVE: DWORD = 0x00000002;
pub const CDB2GVF_ALLOWPREVIEWPANE: DWORD = 0x00000004;
pub const CDB2GVF_NOSELECTVERB: DWORD = 0x00000008;
pub const CDB2GVF_NOINCLUDEITEM: DWORD = 0x00000010;
pub const CDB2GVF_ISFOLDERPICKER: DWORD = 0x00000020;
pub const CDB2GVF_ADDSHIELD: DWORD = 0x00000040;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0025_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0025_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICommDlgBrowser2,
    0x10339516, 0x2894, 0x11d2, 0x90, 0x39, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e}
RIDL!{#[uuid(0x10339516, 0x2894, 0x11d2, 0x90, 0x39, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e)]
interface ICommDlgBrowser2(ICommDlgBrowser2Vtbl): ICommDlgBrowser(ICommDlgBrowserVtbl) {
    fn Notify(
        ppshv: *mut IShellView,
        dwNotifyType: DWORD,
    ) -> HRESULT,
    fn GetDefaultMenuText(
        ppshv: *mut IShellView,
        pszText: LPWSTR,
        cchMax: c_int,
    ) -> HRESULT,
    fn GetViewFlags(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPCOMMDLGBROWSER2 = *mut ICommDlgBrowser2;
ENUM!{enum CM_MASK {
    CM_MASK_WIDTH = 0x1,
    CM_MASK_DEFAULTWIDTH = 0x2,
    CM_MASK_IDEALWIDTH = 0x4,
    CM_MASK_NAME = 0x8,
    CM_MASK_STATE = 0x10,
}}
ENUM!{enum CM_STATE {
    CM_STATE_NONE = 0,
    CM_STATE_VISIBLE = 0x1,
    CM_STATE_FIXEDWIDTH = 0x2,
    CM_STATE_NOSORTBYFOLDERNESS = 0x4,
    CM_STATE_ALWAYSVISIBLE = 0x8,
}}
ENUM!{enum CM_ENUM_FLAGS {
    CM_ENUM_ALL = 0x1,
    CM_ENUM_VISIBLE = 0x2,
}}
ENUM!{enum CM_SET_WIDTH_VALUE {
    CM_WIDTH_USEDEFAULT = -1i32 as u32,
    CM_WIDTH_AUTOSIZE = -2i32 as u32,
}}
STRUCT!{struct CM_COLUMNINFO {
    cbSize: DWORD,
    dwMask: DWORD,
    dwState: DWORD,
    uWidth: UINT,
    uDefaultWidth: UINT,
    uIdealWidth: UINT,
    wszName: [WCHAR; 80],
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0026_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0026_v0_0_s_ifspec;
DEFINE_GUID!{IID_IColumnManager,
    0xd8ec27bb, 0x3f3b, 0x4042, 0xb1, 0x0a, 0x4a, 0xcf, 0xd9, 0x24, 0xd4, 0x53}
RIDL!{#[uuid(0xd8ec27bb, 0x3f3b, 0x4042, 0xb1, 0x0a, 0x4a, 0xcf, 0xd9, 0x24, 0xd4, 0x53)]
interface IColumnManager(IColumnManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetColumnInfo(
        propkey: REFPROPERTYKEY,
        pcmci: *const CM_COLUMNINFO,
    ) -> HRESULT,
    fn GetColumnInfo(
        propkey: REFPROPERTYKEY,
        pcmci: *mut CM_COLUMNINFO,
    ) -> HRESULT,
    fn GetColumnCount(
        dwFlags: CM_ENUM_FLAGS,
        puCount: *mut UINT,
    ) -> HRESULT,
    fn GetColumns(
        dwFlags: CM_ENUM_FLAGS,
        rgkeyOrder: *mut PROPERTYKEY,
        cColumns: UINT,
    ) -> HRESULT,
    fn SetColumns(
        rgkeyOrder: *const PROPERTYKEY,
        cVisible: UINT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0027_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0027_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderFilterSite,
    0xc0a651f5, 0xb48b, 0x11d2, 0xb5, 0xed, 0x00, 0x60, 0x97, 0xc6, 0x86, 0xf6}
RIDL!{#[uuid(0xc0a651f5, 0xb48b, 0x11d2, 0xb5, 0xed, 0x00, 0x60, 0x97, 0xc6, 0x86, 0xf6)]
interface IFolderFilterSite(IFolderFilterSiteVtbl): IUnknown(IUnknownVtbl) {
    fn SetFilter(
        punk: *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFolderFilter,
    0x9cc22886, 0xdc8e, 0x11d2, 0xb1, 0xd0, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e}
RIDL!{#[uuid(0x9cc22886, 0xdc8e, 0x11d2, 0xb1, 0xd0, 0x00, 0xc0, 0x4f, 0x8e, 0xeb, 0x3e)]
interface IFolderFilter(IFolderFilterVtbl): IUnknown(IUnknownVtbl) {
    fn ShouldShow(
        psf: *mut IShellFolder,
        pidlFolder: PCIDLIST_ABSOLUTE,
        pidlItem: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn GetEnumFlags(
        psf: *mut IShellFolder,
        pidlFolder: PCIDLIST_ABSOLUTE,
        phwnd: *mut HWND,
        pgrfFlags: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInputObjectSite,
    0xf1db8392, 0x7331, 0x11d0, 0x8c, 0x99, 0x00, 0xa0, 0xc9, 0x2d, 0xbf, 0xe8}
RIDL!{#[uuid(0xf1db8392, 0x7331, 0x11d0, 0x8c, 0x99, 0x00, 0xa0, 0xc9, 0x2d, 0xbf, 0xe8)]
interface IInputObjectSite(IInputObjectSiteVtbl): IUnknown(IUnknownVtbl) {
    fn OnFocusChangeIS(
        punkObj: *mut IUnknown,
        fSetFocus: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInputObject,
    0x68284faa, 0x6a48, 0x11d0, 0x8c, 0x78, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xb4}
RIDL!{#[uuid(0x68284faa, 0x6a48, 0x11d0, 0x8c, 0x78, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xb4)]
interface IInputObject(IInputObjectVtbl): IUnknown(IUnknownVtbl) {
    fn UIActivateIO(
        fActivate: BOOL,
        pMsg: *mut MSG,
    ) -> HRESULT,
    fn HasFocusIO() -> HRESULT,
    fn TranslateAcceleratorIO(
        pMsg: *mut MSG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInputObject2,
    0x6915c085, 0x510b, 0x44cd, 0x94, 0xaf, 0x28, 0xdf, 0xa5, 0x6c, 0xf9, 0x2b}
RIDL!{#[uuid(0x6915c085, 0x510b, 0x44cd, 0x94, 0xaf, 0x28, 0xdf, 0xa5, 0x6c, 0xf9, 0x2b)]
interface IInputObject2(IInputObject2Vtbl): IInputObject(IInputObjectVtbl) {
    fn TranslateAcceleratorGlobal(
        pMsg: *mut MSG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellIcon,
    0x000214e5, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e5, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellIcon(IShellIconVtbl): IUnknown(IUnknownVtbl) {
    fn GetIconOf(
        pidl: PCUITEMID_CHILD,
        flags: UINT,
        pIconIndex: *mut c_int,
    ) -> HRESULT,
}}
pub const SBSP_DEFBROWSER: UINT = 0x0000;
pub const SBSP_SAMEBROWSER: UINT = 0x0001;
pub const SBSP_NEWBROWSER: UINT = 0x0002;
pub const SBSP_DEFMODE: UINT = 0x0000;
pub const SBSP_OPENMODE: UINT = 0x0010;
pub const SBSP_EXPLOREMODE: UINT = 0x0020;
pub const SBSP_HELPMODE: UINT = 0x0040;
pub const SBSP_NOTRANSFERHIST: UINT = 0x0080;
pub const SBSP_ABSOLUTE: UINT = 0x0000;
pub const SBSP_RELATIVE: UINT = 0x1000;
pub const SBSP_PARENT: UINT = 0x2000;
pub const SBSP_NAVIGATEBACK: UINT = 0x4000;
pub const SBSP_NAVIGATEFORWARD: UINT = 0x8000;
pub const SBSP_ALLOW_AUTONAVIGATE: UINT = 0x00010000;
pub const SBSP_KEEPSAMETEMPLATE: UINT = 0x00020000;
pub const SBSP_KEEPWORDWHEELTEXT: UINT = 0x00040000;
pub const SBSP_ACTIVATE_NOFOCUS: UINT = 0x00080000;
pub const SBSP_CREATENOHISTORY: UINT = 0x00100000;
pub const SBSP_PLAYNOSOUND: UINT = 0x00200000;
pub const SBSP_CALLERUNTRUSTED: UINT = 0x00800000;
pub const SBSP_TRUSTFIRSTDOWNLOAD: UINT = 0x01000000;
pub const SBSP_UNTRUSTEDFORDOWNLOAD: UINT = 0x02000000;
pub const SBSP_NOAUTOSELECT: UINT = 0x04000000;
pub const SBSP_WRITENOHISTORY: UINT = 0x08000000;
pub const SBSP_TRUSTEDFORACTIVEX: UINT = 0x10000000;
pub const SBSP_FEEDNAVIGATION: UINT = 0x20000000;
pub const SBSP_REDIRECT: UINT = 0x40000000;
pub const SBSP_INITIATEDBYHLINKFRAME: UINT = 0x80000000;
pub const FCW_STATUS: UINT = 0x0001;
pub const FCW_TOOLBAR: UINT = 0x0002;
pub const FCW_TREE: UINT = 0x0003;
pub const FCW_INTERNETBAR: UINT = 0x0006;
pub const FCW_PROGRESS: UINT = 0x0008;
pub const FCT_MERGE: UINT = 0x0001;
pub const FCT_CONFIGABLE: UINT = 0x0002;
pub const FCT_ADDTOEND: UINT = 0x0004;
pub type LPTBBUTTONSB = LPTBBUTTON;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0033_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0033_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellBrowser,
    0x000214e2, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e2, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellBrowser(IShellBrowserVtbl): IOleWindow(IOleWindowVtbl) {
    fn InsertMenusSB(
        hmenuShared: HMENU,
        lpMenuWidths: LPOLEMENUGROUPWIDTHS,
    ) -> HRESULT,
    fn SetMenuSB(
        hmenuShared: HMENU,
        holemenuRes: HOLEMENU,
        hwndActiveObject: HWND,
    ) -> HRESULT,
    fn RemoveMenusSB(
        hmenuShared: HMENU,
    ) -> HRESULT,
    fn SetStatusTextSB(
        pszStatusText: LPCWSTR,
    ) -> HRESULT,
    fn EnableModelessSB(
        fEnable: BOOL,
    ) -> HRESULT,
    fn TranslateAcceleratorSB(
        pmsg: *mut MSG,
        wID: WORD,
    ) -> HRESULT,
    fn BrowseObject(
        pidl: PCUIDLIST_RELATIVE,
        wFlags: UINT,
    ) -> HRESULT,
    fn GetViewStateStream(
        grfMode: DWORD,
        ppStrm: *mut *mut IStream,
    ) -> HRESULT,
    fn GetControlWindow(
        id: UINT,
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn SendControlMsg(
        id: UINT,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        pret: *mut LRESULT,
    ) -> HRESULT,
    fn QueryActiveShellView(
        ppshv: *mut *mut IShellView,
    ) -> HRESULT,
    fn OnViewWindowActive(
        pshv: *mut IShellView,
    ) -> HRESULT,
    fn SetToolbarItems(
        lpButtons: LPTBBUTTONSB,
        nButtons: UINT,
        uFlags: UINT,
    ) -> HRESULT,
}}
pub type LPSHELLBROWSER = *mut IShellBrowser;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0034_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0034_v0_0_s_ifspec;
DEFINE_GUID!{IID_IProfferService,
    0xcb728b20, 0xf786, 0x11ce, 0x92, 0xad, 0x00, 0xaa, 0x00, 0xa7, 0x4c, 0xd0}
RIDL!{#[uuid(0xcb728b20, 0xf786, 0x11ce, 0x92, 0xad, 0x00, 0xaa, 0x00, 0xa7, 0x4c, 0xd0)]
interface IProfferService(IProfferServiceVtbl): IUnknown(IUnknownVtbl) {
    fn ProfferService(
        guidService: REFGUID,
        psp: *mut IServiceProvider,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn RevokeService(
        dwCookie: DWORD,
    ) -> HRESULT,
}}
// #define SID_SProfferService IID_IProfferService
// #define STR_DONT_RESOLVE_LINK L"Don't Resolve Link"
// #define STR_GET_ASYNC_HANDLER L"GetAsyncHandler"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0035_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0035_v0_0_s_ifspec;
ENUM!{enum SIGDN {
    SIGDN_NORMALDISPLAY = 0,
    SIGDN_PARENTRELATIVEPARSING = 0x80018001,
    SIGDN_DESKTOPABSOLUTEPARSING = 0x80028000,
    SIGDN_PARENTRELATIVEEDITING = 0x80031001,
    SIGDN_DESKTOPABSOLUTEEDITING = 0x8004c000,
    SIGDN_FILESYSPATH = 0x80058000,
    SIGDN_URL = 0x80068000,
    SIGDN_PARENTRELATIVEFORADDRESSBAR = 0x8007c001,
    SIGDN_PARENTRELATIVE = 0x80080001,
    SIGDN_PARENTRELATIVEFORUI = 0x80094001,
}}
ENUM!{enum _SICHINTF {
    SICHINT_DISPLAY = 0,
    SICHINT_ALLFIELDS = 0x80000000,
    SICHINT_CANONICAL = 0x10000000,
    SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL = 0x20000000,
}}
pub type SICHINTF = DWORD;
DEFINE_GUID!{IID_IShellItem,
    0x43826d1e, 0xe718, 0x42ee, 0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe}
RIDL!{#[uuid(0x43826d1e, 0xe718, 0x42ee, 0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe)]
interface IShellItem(IShellItemVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetParent(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetDisplayName(
        sigdnName: SIGDN,
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAttributes(
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> HRESULT,
    fn Compare(
        psi: *mut IShellItem,
        hint: SICHINTF,
        piOrder: *mut c_int,
    ) -> HRESULT,
}}
extern "system" {
    pub fn SHSimpleIDListFromPath(
        pszPath: PCWSTR,
    ) -> PIDLIST_ABSOLUTE;
    pub fn SHCreateItemFromIDList(
        pidl: PCIDLIST_ABSOLUTE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHCreateItemFromParsingName(
        pszPath: PCWSTR,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHCreateItemWithParent(
        pidlParent: PCIDLIST_ABSOLUTE,
        psfParent: *mut IShellFolder,
        pidl: PCUITEMID_CHILD,
        riid: REFIID,
        ppvItem: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHCreateItemFromRelativeName(
        psiParent: *mut IShellItem,
        pszName: PCWSTR,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHCreateItemInKnownFolder(
        kfid: REFKNOWNFOLDERID,
        dwKFFlags: DWORD,
        pszItem: PCWSTR,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetIDListFromObject(
        punk: *mut IUnknown,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetItemFromObject(
        punk: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetPropertyStoreFromIDList(
        pidl: PCIDLIST_ABSOLUTE,
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetPropertyStoreFromParsingName(
        pszPath: PCWSTR,
        pbc: *mut IBindCtx,
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetNameFromIDList(
        pidl: PCIDLIST_ABSOLUTE,
        sigdnName: SIGDN,
        ppszName: *mut PWSTR,
    ) -> HRESULT;
}
ENUM!{enum DATAOBJ_GET_ITEM_FLAGS {
    DOGIF_DEFAULT = 0,
    DOGIF_TRAVERSE_LINK = 0x1,
    DOGIF_NO_HDROP = 0x2,
    DOGIF_NO_URL = 0x4,
    DOGIF_ONLY_IF_ONE = 0x8,
}}
extern "system" {
    pub fn SHGetItemFromDataObject(
        pdtobj: *mut IDataObject,
        dwFlags: DATAOBJ_GET_ITEM_FLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
// #define STR_GPS_HANDLERPROPERTIESONLY L"GPS_HANDLERPROPERTIESONLY"
// #define STR_GPS_FASTPROPERTIESONLY L"GPS_FASTPROPERTIESONLY"
// #define STR_GPS_OPENSLOWITEM L"GPS_OPENSLOWITEM"
// #define STR_GPS_DELAYCREATION L"GPS_DELAYCREATION"
// #define STR_GPS_BESTEFFORT L"GPS_BESTEFFORT"
// #define STR_GPS_NO_OPLOCK L"GPS_NO_OPLOCK"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0036_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0036_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellItem2,
    0x7e9fb0d3, 0x919f, 0x4307, 0xab, 0x2e, 0x9b, 0x18, 0x60, 0x31, 0x0c, 0x93}
RIDL!{#[uuid(0x7e9fb0d3, 0x919f, 0x4307, 0xab, 0x2e, 0x9b, 0x18, 0x60, 0x31, 0x0c, 0x93)]
interface IShellItem2(IShellItem2Vtbl): IShellItem(IShellItemVtbl) {
    fn GetPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyStoreWithCreateObject(
        flags: GETPROPERTYSTOREFLAGS,
        punkCreateObject: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyStoreForKeys(
        rgKeys: *const PROPERTYKEY,
        cKeys: UINT,
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyDescriptionList(
        keyType: REFPROPERTYKEY,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn Update(
        pbc: *mut IBindCtx,
    ) -> HRESULT,
    fn GetProperty(
        key: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT,
    fn GetCLSID(
        key: REFPROPERTYKEY,
        pclsid: *mut CLSID,
    ) -> HRESULT,
    fn GetFileTime(
        key: REFPROPERTYKEY,
        pft: *mut FILETIME,
    ) -> HRESULT,
    fn GetInt32(
        key: REFPROPERTYKEY,
        pi: *mut c_int,
    ) -> HRESULT,
    fn GetString(
        key: REFPROPERTYKEY,
        ppsz: *mut LPWSTR,
    ) -> HRESULT,
    fn GetUInt32(
        key: REFPROPERTYKEY,
        pui: *mut ULONG,
    ) -> HRESULT,
    fn GetUInt64(
        key: REFPROPERTYKEY,
        pull: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetBool(
        key: REFPROPERTYKEY,
        pf: *mut BOOL,
    ) -> HRESULT,
}}
ENUM!{enum _SIIGBF {
    SIIGBF_RESIZETOFIT = 0,
    SIIGBF_BIGGERSIZEOK = 0x1,
    SIIGBF_MEMORYONLY = 0x2,
    SIIGBF_ICONONLY = 0x4,
    SIIGBF_THUMBNAILONLY = 0x8,
    SIIGBF_INCACHEONLY = 0x10,
    SIIGBF_CROPTOSQUARE = 0x20,
    SIIGBF_WIDETHUMBNAILS = 0x40,
    SIIGBF_ICONBACKGROUND = 0x80,
    SIIGBF_SCALEUP = 0x100,
}}
pub type SIIGBF = c_int;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0037_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0037_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellItemImageFactory,
    0xbcc18b79, 0xba16, 0x442f, 0x80, 0xc4, 0x8a, 0x59, 0xc3, 0x0c, 0x46, 0x3b}
RIDL!{#[uuid(0xbcc18b79, 0xba16, 0x442f, 0x80, 0xc4, 0x8a, 0x59, 0xc3, 0x0c, 0x46, 0x3b)]
interface IShellItemImageFactory(IShellItemImageFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetImage(
        size: SIZE,
        flags: SIIGBF,
        phbm: *mut HBITMAP,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0038_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0038_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumShellItems,
    0x70629033, 0xe363, 0x4a28, 0xa5, 0x67, 0x0d, 0xb7, 0x80, 0x06, 0xe6, 0xd7}
RIDL!{#[uuid(0x70629033, 0xe363, 0x4a28, 0xa5, 0x67, 0x0d, 0xb7, 0x80, 0x06, 0xe6, 0xd7)]
interface IEnumShellItems(IEnumShellItemsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IShellItem,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumShellItems,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumShellItems_RemoteNext_Proxy(
//     IEnumShellItems * This,
//     ULONG celt,
//     IShellItem **rgelt,
//     ULONG *pceltFetched);
// c_void __RPC_STUB IEnumShellItems_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type STGTRANSCONFIRMATION = GUID;
pub type LPSTGTRANSCONFIRMATION = *mut GUID;
ENUM!{enum STGOP {
    STGOP_MOVE = 1,
    STGOP_COPY = 2,
    STGOP_SYNC = 3,
    STGOP_REMOVE = 5,
    STGOP_RENAME = 6,
    STGOP_APPLYPROPERTIES = 8,
    STGOP_NEW = 10,
}}
ENUM!{enum _TRANSFER_SOURCE_FLAGS {
    TSF_NORMAL = 0,
    TSF_FAIL_EXIST = 0,
    TSF_RENAME_EXIST = 0x1,
    TSF_OVERWRITE_EXIST = 0x2,
    TSF_ALLOW_DECRYPTION = 0x4,
    TSF_NO_SECURITY = 0x8,
    TSF_COPY_CREATION_TIME = 0x10,
    TSF_COPY_WRITE_TIME = 0x20,
    TSF_USE_FULL_ACCESS = 0x40,
    TSF_DELETE_RECYCLE_IF_POSSIBLE = 0x80,
    TSF_COPY_HARD_LINK = 0x100,
    TSF_COPY_LOCALIZED_NAME = 0x200,
    TSF_MOVE_AS_COPY_DELETE = 0x400,
    TSF_SUSPEND_SHELLEVENTS = 0x800,
}}
pub type TRANSFER_SOURCE_FLAGS = DWORD;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0039_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0039_v0_0_s_ifspec;
ENUM!{enum _TRANSFER_ADVISE_STATE {
    TS_NONE = 0,
    TS_PERFORMING = 0x1,
    TS_PREPARING = 0x2,
    TS_INDETERMINATE = 0x4,
}}
pub type TRANSFER_ADVISE_STATE = DWORD;
DEFINE_GUID!{IID_ITransferAdviseSink,
    0xd594d0d8, 0x8da7, 0x457b, 0xb3, 0xb4, 0xce, 0x5d, 0xba, 0xac, 0x0b, 0x88}
RIDL!{#[uuid(0xd594d0d8, 0x8da7, 0x457b, 0xb3, 0xb4, 0xce, 0x5d, 0xba, 0xac, 0x0b, 0x88)]
interface ITransferAdviseSink(ITransferAdviseSinkVtbl): IUnknown(IUnknownVtbl) {
    fn UpdateProgress(
        ullSizeCurrent: ULONGLONG,
        ullSizeTotal: ULONGLONG,
        nFilesCurrent: c_int,
        nFilesTotal: c_int,
        nFoldersCurrent: c_int,
        nFoldersTotal: c_int,
    ) -> HRESULT,
    fn UpdateTransferState(
        ts: TRANSFER_ADVISE_STATE,
    ) -> HRESULT,
    fn ConfirmOverwrite(
        psiSource: *mut IShellItem,
        psiDestParent: *mut IShellItem,
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn ConfirmEncryptionLoss(
        psiSource: *mut IShellItem,
    ) -> HRESULT,
    fn FileFailure(
        psi: *mut IShellItem,
        pszItem: LPCWSTR,
        hrError: HRESULT,
        pszRename: LPWSTR,
        cchRename: ULONG,
    ) -> HRESULT,
    fn SubStreamFailure(
        psi: *mut IShellItem,
        pszStreamName: LPCWSTR,
        hrError: HRESULT,
    ) -> HRESULT,
    fn PropertyFailure(
        psi: *mut IShellItem,
        pkey: *const PROPERTYKEY,
        hrError: HRESULT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0040_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0040_v0_0_s_ifspec;
DEFINE_GUID!{IID_ITransferSource,
    0x00adb003, 0xbde9, 0x45c6, 0x8e, 0x29, 0xd0, 0x9f, 0x93, 0x53, 0xe1, 0x08}
RIDL!{#[uuid(0x00adb003, 0xbde9, 0x45c6, 0x8e, 0x29, 0xd0, 0x9f, 0x93, 0x53, 0xe1, 0x08)]
interface ITransferSource(ITransferSourceVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        psink: *mut ITransferAdviseSink,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn SetProperties(
        pproparray: *mut IPropertyChangeArray,
    ) -> HRESULT,
    fn OpenItem(
        psi: *mut IShellItem,
        flags: TRANSFER_SOURCE_FLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn MoveItem(
        psi: *mut IShellItem,
        psiParentDst: *mut IShellItem,
        pszNameDst: LPCWSTR,
        flags: TRANSFER_SOURCE_FLAGS,
        ppsiNew: *mut *mut IShellItem,
    ) -> HRESULT,
    fn RecycleItem(
        psiSource: *mut IShellItem,
        psiParentDest: *mut IShellItem,
        flags: TRANSFER_SOURCE_FLAGS,
        ppsiNewDest: *mut *mut IShellItem,
    ) -> HRESULT,
    fn RemoveItem(
        psiSource: *mut IShellItem,
        flags: TRANSFER_SOURCE_FLAGS,
    ) -> HRESULT,
    fn RenameItem(
        psiSource: *mut IShellItem,
        pszNewName: LPCWSTR,
        flags: TRANSFER_SOURCE_FLAGS,
        ppsiNewDest: *mut *mut IShellItem,
    ) -> HRESULT,
    fn LinkItem(
        psiSource: *mut IShellItem,
        psiParentDest: *mut IShellItem,
        pszNewName: LPCWSTR,
        flags: TRANSFER_SOURCE_FLAGS,
        ppsiNewDest: *mut *mut IShellItem,
    ) -> HRESULT,
    fn ApplyPropertiesToItem(
        psiSource: *mut IShellItem,
        ppsiNew: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetDefaultDestinationName(
        psiSource: *mut IShellItem,
        psiParentDest: *mut IShellItem,
        ppszDestinationName: *mut LPWSTR,
    ) -> HRESULT,
    fn EnterFolder(
        psiChildFolderDest: *mut IShellItem,
    ) -> HRESULT,
    fn LeaveFolder(
        psiChildFolderDest: *mut IShellItem,
    ) -> HRESULT,
}}
STRUCT!{struct SHELL_ITEM_RESOURCE {
    guidType: GUID,
    szName: [WCHAR; 260],
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0041_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0041_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumResources,
    0x2dd81fe3, 0xa83c, 0x4da9, 0xa3, 0x30, 0x47, 0x24, 0x9d, 0x34, 0x5b, 0xa1}
RIDL!{#[uuid(0x2dd81fe3, 0xa83c, 0x4da9, 0xa3, 0x30, 0x47, 0x24, 0x9d, 0x34, 0x5b, 0xa1)]
interface IEnumResources(IEnumResourcesVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        psir: *mut SHELL_ITEM_RESOURCE,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenumr: *mut *mut IEnumResources,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellItemResources,
    0xff5693be, 0x2ce0, 0x4d48, 0xb5, 0xc5, 0x40, 0x81, 0x7d, 0x1a, 0xcd, 0xb9}
RIDL!{#[uuid(0xff5693be, 0x2ce0, 0x4d48, 0xb5, 0xc5, 0x40, 0x81, 0x7d, 0x1a, 0xcd, 0xb9)]
interface IShellItemResources(IShellItemResourcesVtbl): IUnknown(IUnknownVtbl) {
    fn GetAttributes(
        pdwAttributes: *mut DWORD,
    ) -> HRESULT,
    fn GetSize(
        pullSize: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetTimes(
        pftCreation: *mut FILETIME,
        pftWrite: *mut FILETIME,
        pftAccess: *mut FILETIME,
    ) -> HRESULT,
    fn SetTimes(
        pftCreation: *const FILETIME,
        pftWrite: *const FILETIME,
        pftAccess: *const FILETIME,
    ) -> HRESULT,
    fn GetResourceDescription(
        pcsir: *const SHELL_ITEM_RESOURCE,
        ppszDescription: *mut LPWSTR,
    ) -> HRESULT,
    fn EnumResources(
        ppenumr: *mut *mut IEnumResources,
    ) -> HRESULT,
    fn SupportsResource(
        pcsir: *const SHELL_ITEM_RESOURCE,
    ) -> HRESULT,
    fn OpenResource(
        pcsir: *const SHELL_ITEM_RESOURCE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateResource(
        pcsir: *const SHELL_ITEM_RESOURCE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn MarkForDelete() -> HRESULT,
}}
DEFINE_GUID!{IID_ITransferDestination,
    0x48addd32, 0x3ca5, 0x4124, 0xab, 0xe3, 0xb5, 0xa7, 0x25, 0x31, 0xb2, 0x07}
RIDL!{#[uuid(0x48addd32, 0x3ca5, 0x4124, 0xab, 0xe3, 0xb5, 0xa7, 0x25, 0x31, 0xb2, 0x07)]
interface ITransferDestination(ITransferDestinationVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        psink: *mut ITransferAdviseSink,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn CreateItem(
        pszName: LPCWSTR,
        dwAttributes: DWORD,
        ullSize: ULONGLONG,
        flags: TRANSFER_SOURCE_FLAGS,
        riidItem: REFIID,
        ppvItem: *mut *mut c_void,
        riidResources: REFIID,
        ppvResources: *mut *mut c_void,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0044_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0044_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileOperationProgressSink,
    0x04b0f1a7, 0x9490, 0x44bc, 0x96, 0xe1, 0x42, 0x96, 0xa3, 0x12, 0x52, 0xe2}
RIDL!{#[uuid(0x04b0f1a7, 0x9490, 0x44bc, 0x96, 0xe1, 0x42, 0x96, 0xa3, 0x12, 0x52, 0xe2)]
interface IFileOperationProgressSink(IFileOperationProgressSinkVtbl): IUnknown(IUnknownVtbl) {
    fn StartOperations() -> HRESULT,
    fn FinishOperations(
        hrResult: HRESULT,
    ) -> HRESULT,
    fn PreRenameItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        pszNewName: LPCWSTR,
    ) -> HRESULT,
    fn PostRenameItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        pszNewName: LPCWSTR,
        hrRename: HRESULT,
        psiNewlyCreated: *mut IShellItem,
    ) -> HRESULT,
    fn PreMoveItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
    ) -> HRESULT,
    fn PostMoveItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
        hrMove: HRESULT,
        psiNewlyCreated: *mut IShellItem,
    ) -> HRESULT,
    fn PreCopyItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
    ) -> HRESULT,
    fn PostCopyItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
        hrCopy: HRESULT,
        psiNewlyCreated: *mut IShellItem,
    ) -> HRESULT,
    fn PreDeleteItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
    ) -> HRESULT,
    fn PostDeleteItem(
        dwFlags: DWORD,
        psiItem: *mut IShellItem,
        hrDelete: HRESULT,
        psiNewlyCreated: *mut IShellItem,
    ) -> HRESULT,
    fn PreNewItem(
        dwFlags: DWORD,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
    ) -> HRESULT,
    fn PostNewItem(
        dwFlags: DWORD,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
        pszTemplateName: LPCWSTR,
        dwFileAttributes: DWORD,
        hrNew: HRESULT,
        psiNewItem: *mut IShellItem,
    ) -> HRESULT,
    fn UpdateProgress(
        iWorkTotal: UINT,
        iWorkSoFar: UINT,
    ) -> HRESULT,
    fn ResetTimer() -> HRESULT,
    fn PauseTimer() -> HRESULT,
    fn ResumeTimer() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0045_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0045_v0_0_s_ifspec;
ENUM!{enum SIATTRIBFLAGS {
    SIATTRIBFLAGS_AND = 0x1,
    SIATTRIBFLAGS_OR = 0x2,
    SIATTRIBFLAGS_APPCOMPAT = 0x3,
    SIATTRIBFLAGS_MASK = 0x3,
    SIATTRIBFLAGS_ALLITEMS = 0x4000,
}}
DEFINE_GUID!{IID_IShellItemArray,
    0xb63ea76d, 0x1f85, 0x456f, 0xa1, 0x9c, 0x48, 0x15, 0x9e, 0xfa, 0x85, 0x8b}
RIDL!{#[uuid(0xb63ea76d, 0x1f85, 0x456f, 0xa1, 0x9c, 0x48, 0x15, 0x9e, 0xfa, 0x85, 0x8b)]
interface IShellItemArray(IShellItemArrayVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppvOut: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyDescriptionList(
        keyType: REFPROPERTYKEY,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetAttributes(
        AttribFlags: SIATTRIBFLAGS,
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> HRESULT,
    fn GetCount(
        pdwNumItems: *mut DWORD,
    ) -> HRESULT,
    fn GetItemAt(
        dwIndex: DWORD,
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn EnumItems(
        ppenumShellItems: *mut *mut IEnumShellItems,
    ) -> HRESULT,
}}
extern "system" {
    pub fn SHCreateShellItemArray(
        pidlParent: PCIDLIST_ABSOLUTE,
        psf: *mut IShellFolder,
        cidl: UINT,
        ppidl: PCUITEMID_CHILD_ARRAY,
        ppsiItemArray: *mut *mut IShellItemArray,
    ) -> HRESULT;
    pub fn SHCreateShellItemArrayFromDataObject(
        pdo: *mut IDataObject,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHCreateShellItemArrayFromIDLists(
        cidl: UINT,
        rgpidl: PCIDLIST_ABSOLUTE_ARRAY,
        ppsiItemArray: *mut *mut IShellItemArray,
    ) -> HRESULT;
    pub fn SHCreateShellItemArrayFromShellItem(
        psi: *mut IShellItem,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0046_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0046_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInitializeWithItem,
    0x7f73be3f, 0xfb79, 0x493c, 0xa6, 0xc7, 0x7e, 0xe1, 0x4e, 0x24, 0x58, 0x41}
RIDL!{#[uuid(0x7f73be3f, 0xfb79, 0x493c, 0xa6, 0xc7, 0x7e, 0xe1, 0x4e, 0x24, 0x58, 0x41)]
interface IInitializeWithItem(IInitializeWithItemVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        psi: *mut IShellItem,
        grfMode: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectWithSelection,
    0x1c9cd5bb, 0x98e9, 0x4491, 0xa6, 0x0f, 0x31, 0xaa, 0xcc, 0x72, 0xb8, 0x3c}
RIDL!{#[uuid(0x1c9cd5bb, 0x98e9, 0x4491, 0xa6, 0x0f, 0x31, 0xaa, 0xcc, 0x72, 0xb8, 0x3c)]
interface IObjectWithSelection(IObjectWithSelectionVtbl): IUnknown(IUnknownVtbl) {
    fn SetSelection(
        psia: *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSelection(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectWithBackReferences,
    0x321a6a6a, 0xd61f, 0x4bf3, 0x97, 0xae, 0x14, 0xbe, 0x29, 0x86, 0xbb, 0x36}
RIDL!{#[uuid(0x321a6a6a, 0xd61f, 0x4bf3, 0x97, 0xae, 0x14, 0xbe, 0x29, 0x86, 0xbb, 0x36)]
interface IObjectWithBackReferences(IObjectWithBackReferencesVtbl): IUnknown(IUnknownVtbl) {
    fn RemoveBackReferences() -> HRESULT,
}}
ENUM!{enum _PROPERTYUI_NAME_FLAGS {
    PUIFNF_DEFAULT = 0,
    PUIFNF_MNEMONIC = 0x1,
}}
pub type PROPERTYUI_NAME_FLAGS = DWORD;
ENUM!{enum _PROPERTYUI_FLAGS {
    PUIF_DEFAULT = 0,
    PUIF_RIGHTALIGN = 0x1,
    PUIF_NOLABELININFOTIP = 0x2,
}}
pub type PROPERTYUI_FLAGS = DWORD;
ENUM!{enum _PROPERTYUI_FORMAT_FLAGS {
    PUIFFDF_DEFAULT = 0,
    PUIFFDF_RIGHTTOLEFT = 0x1,
    PUIFFDF_SHORTFORMAT = 0x2,
    PUIFFDF_NOTIME = 0x4,
    PUIFFDF_FRIENDLYDATE = 0x8,
}}
pub type PROPERTYUI_FORMAT_FLAGS = DWORD;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0049_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0049_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPropertyUI,
    0x757a7d9f, 0x919a, 0x4118, 0x99, 0xd7, 0xdb, 0xb2, 0x08, 0xc8, 0xcc, 0x66}
RIDL!{#[uuid(0x757a7d9f, 0x919a, 0x4118, 0x99, 0xd7, 0xdb, 0xb2, 0x08, 0xc8, 0xcc, 0x66)]
interface IPropertyUI(IPropertyUIVtbl): IUnknown(IUnknownVtbl) {
    fn ParsePropertyName(
        pszName: LPCWSTR,
        pfmtid: *mut FMTID,
        ppid: *mut PROPID,
        pchEaten: *mut ULONG,
    ) -> HRESULT,
    fn GetCannonicalName(
        fmtid: REFFMTID,
        pid: PROPID,
        pwszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT,
    fn GetDisplayName(
        fmtid: REFFMTID,
        pid: PROPID,
        flags: PROPERTYUI_NAME_FLAGS,
        pwszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT,
    fn GetPropertyDescription(
        fmtid: REFFMTID,
        pid: PROPID,
        pwszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT,
    fn GetDefaultWidth(
        fmtid: REFFMTID,
        pid: PROPID,
        pcxChars: *mut ULONG,
    ) -> HRESULT,
    fn GetFlags(
        fmtid: REFFMTID,
        pid: PROPID,
        pflags: *mut PROPERTYUI_FLAGS,
    ) -> HRESULT,
    fn FormatForDisplay(
        fmtid: REFFMTID,
        pid: PROPID,
        ppropvar: *const PROPVARIANT,
        puiff: PROPERTYUI_FORMAT_FLAGS,
        pwszText: LPWSTR,
        cchText: DWORD,
    ) -> HRESULT,
    fn GetHelpInfo(
        fmtid: REFFMTID,
        pid: PROPID,
        pwszHelpFile: LPWSTR,
        cch: DWORD,
        puHelpID: *mut UINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ICategoryProvider,
    0x9af64809, 0x5864, 0x4c26, 0xa7, 0x20, 0xc1, 0xf7, 0x8c, 0x08, 0x6e, 0xe3}
RIDL!{#[uuid(0x9af64809, 0x5864, 0x4c26, 0xa7, 0x20, 0xc1, 0xf7, 0x8c, 0x08, 0x6e, 0xe3)]
interface ICategoryProvider(ICategoryProviderVtbl): IUnknown(IUnknownVtbl) {
    fn CanCategorizeOnSCID(
        pscid: *const SHCOLUMNID,
    ) -> HRESULT,
    fn GetDefaultCategory(
        pguid: *mut GUID,
        pscid: *mut SHCOLUMNID,
    ) -> HRESULT,
    fn GetCategoryForSCID(
        pscid: *const SHCOLUMNID,
        pguid: *mut GUID,
    ) -> HRESULT,
    fn EnumCategories(
        penum: *mut *mut IEnumGUID,
    ) -> HRESULT,
    fn GetCategoryName(
        pguid: *const GUID,
        pszName: LPWSTR,
        cch: UINT,
    ) -> HRESULT,
    fn CreateCategory(
        pguid: *const GUID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum CATEGORYINFO_FLAGS {
    CATINFO_NORMAL = 0,
    CATINFO_COLLAPSED = 0x1,
    CATINFO_HIDDEN = 0x2,
    CATINFO_EXPANDED = 0x4,
    CATINFO_NOHEADER = 0x8,
    CATINFO_NOTCOLLAPSIBLE = 0x10,
    CATINFO_NOHEADERCOUNT = 0x20,
    CATINFO_SUBSETTED = 0x40,
    CATINFO_SEPARATE_IMAGES = 0x80,
    CATINFO_SHOWEMPTY = 0x100,
}}
ENUM!{enum CATSORT_FLAGS {
    CATSORT_DEFAULT = 0,
    CATSORT_NAME = 0x1,
}}
STRUCT!{struct CATEGORY_INFO {
    cif: CATEGORYINFO_FLAGS,
    wszName: [WCHAR; 260],
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0051_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0051_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICategorizer,
    0xa3b14589, 0x9174, 0x49a8, 0x89, 0xa3, 0x06, 0xa1, 0xae, 0x2b, 0x9b, 0xa7}
RIDL!{#[uuid(0xa3b14589, 0x9174, 0x49a8, 0x89, 0xa3, 0x06, 0xa1, 0xae, 0x2b, 0x9b, 0xa7)]
interface ICategorizer(ICategorizerVtbl): IUnknown(IUnknownVtbl) {
    fn GetDescription(
        pszDesc: LPWSTR,
        cch: UINT,
    ) -> HRESULT,
    fn GetCategory(
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        rgCategoryIds: *mut DWORD,
    ) -> HRESULT,
    fn GetCategoryInfo(
        dwCategoryId: DWORD,
        pci: *mut CATEGORY_INFO,
    ) -> HRESULT,
    fn CompareCategory(
        csfFlags: CATSORT_FLAGS,
        dwCategoryId1: DWORD,
        dwCategoryId2: DWORD,
    ) -> HRESULT,
}}
STRUCT!{struct SHDRAGIMAGE {
    sizeDragImage: SIZE,
    ptOffset: POINT,
    hbmpDragImage: HBITMAP,
    crColorKey: COLORREF,
}}
pub type LPSHDRAGIMAGE = *mut SHDRAGIMAGE;
// #define DI_GETDRAGIMAGE TEXT("ShellGetDragImage")
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0052_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0052_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDropTargetHelper,
    0x4657278b, 0x411b, 0x11d2, 0x83, 0x9a, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0}
RIDL!{#[uuid(0x4657278b, 0x411b, 0x11d2, 0x83, 0x9a, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0)]
interface IDropTargetHelper(IDropTargetHelperVtbl): IUnknown(IUnknownVtbl) {
    fn DragEnter(
        hwndTarget: HWND,
        pDataObject: *mut IDataObject,
        ppt: *mut POINT,
        dwEffect: DWORD,
    ) -> HRESULT,
    fn DragLeave() -> HRESULT,
    fn DragOver(
        ppt: *mut POINT,
        dwEffect: DWORD,
    ) -> HRESULT,
    fn Drop(
        pDataObject: *mut IDataObject,
        ppt: *mut POINT,
        dwEffect: DWORD,
    ) -> HRESULT,
    fn Show(
        fShow: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDragSourceHelper,
    0xde5bf786, 0x477a, 0x11d2, 0x83, 0x9d, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0}
RIDL!{#[uuid(0xde5bf786, 0x477a, 0x11d2, 0x83, 0x9d, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0)]
interface IDragSourceHelper(IDragSourceHelperVtbl): IUnknown(IUnknownVtbl) {
    fn InitializeFromBitmap(
        pshdi: LPSHDRAGIMAGE,
        pDataObject: *mut IDataObject,
    ) -> HRESULT,
    fn InitializeFromWindow(
        hwnd: HWND,
        ppt: *mut POINT,
        pDataObject: *mut IDataObject,
    ) -> HRESULT,
}}
ENUM!{enum SLR_FLAGS {
    SLR_NONE = 0,
    SLR_NO_UI = 0x1,
    SLR_ANY_MATCH = 0x2,
    SLR_UPDATE = 0x4,
    SLR_NOUPDATE = 0x8,
    SLR_NOSEARCH = 0x10,
    SLR_NOTRACK = 0x20,
    SLR_NOLINKINFO = 0x40,
    SLR_INVOKE_MSI = 0x80,
    SLR_NO_UI_WITH_MSG_PUMP = 0x101,
    SLR_OFFER_DELETE_WITHOUT_FILE = 0x200,
    SLR_KNOWNFOLDER = 0x400,
    SLR_MACHINE_IN_LOCAL_TARGET = 0x800,
    SLR_UPDATE_MACHINE_AND_SID = 0x1000,
    SLR_NO_OBJECT_ID = 0x2000,
}}
ENUM!{enum SLGP_FLAGS {
    SLGP_SHORTPATH = 0x1,
    SLGP_UNCPRIORITY = 0x2,
    SLGP_RAWPATH = 0x4,
    SLGP_RELATIVEPRIORITY = 0x8,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0054_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0054_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellLinkA,
    0x000214ee, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214ee, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkA(IShellLinkAVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAA,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellLinkW,
    0x000214f9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214f9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkW(IShellLinkWVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPWSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAW,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCWSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCWSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPWSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellLinkDataList,
    0x45e2b4ae, 0xb1c3, 0x11d0, 0xb9, 0x2f, 0x00, 0xa0, 0xc9, 0x03, 0x12, 0xe1}
RIDL!{#[uuid(0x45e2b4ae, 0xb1c3, 0x11d0, 0xb9, 0x2f, 0x00, 0xa0, 0xc9, 0x03, 0x12, 0xe1)]
interface IShellLinkDataList(IShellLinkDataListVtbl): IUnknown(IUnknownVtbl) {
    fn AddDataBlock(
        pDataBlock: *mut c_void,
    ) -> HRESULT,
    fn CopyDataBlock(
        dwSig: DWORD,
        ppDataBlock: *mut *mut c_void,
    ) -> HRESULT,
    fn RemoveDataBlock(
        dwSig: DWORD,
    ) -> HRESULT,
    fn GetFlags(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetFlags(
        dwFlags: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0057_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0057_v0_0_s_ifspec;
DEFINE_GUID!{IID_IResolveShellLink,
    0x5cd52983, 0x9449, 0x11d2, 0x96, 0x3a, 0x00, 0xc0, 0x4f, 0x79, 0xad, 0xf0}
RIDL!{#[uuid(0x5cd52983, 0x9449, 0x11d2, 0x96, 0x3a, 0x00, 0xc0, 0x4f, 0x79, 0xad, 0xf0)]
interface IResolveShellLink(IResolveShellLinkVtbl): IUnknown(IUnknownVtbl) {
    fn ResolveShellLink(
        punkLink: *mut IUnknown,
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0058_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0058_v0_0_s_ifspec;
ENUM!{enum _SPINITF {
    SPINITF_NORMAL = 0,
    SPINITF_MODAL = 0x1,
    SPINITF_NOMINIMIZE = 0x8,
}}
pub type SPINITF = DWORD;
DEFINE_GUID!{IID_IActionProgressDialog,
    0x49ff1172, 0xeadc, 0x446d, 0x92, 0x85, 0x15, 0x64, 0x53, 0xa6, 0x43, 0x1c}
RIDL!{#[uuid(0x49ff1172, 0xeadc, 0x446d, 0x92, 0x85, 0x15, 0x64, 0x53, 0xa6, 0x43, 0x1c)]
interface IActionProgressDialog(IActionProgressDialogVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        flags: SPINITF,
        pszTitle: LPCWSTR,
        pszCancel: LPCWSTR,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
}}
pub const ARCONTENT_AUTORUNINF: DWORD = 0x00000002;
pub const ARCONTENT_AUDIOCD: DWORD = 0x00000004;
pub const ARCONTENT_DVDMOVIE: DWORD = 0x00000008;
pub const ARCONTENT_BLANKCD: DWORD = 0x00000010;
pub const ARCONTENT_BLANKDVD: DWORD = 0x00000020;
pub const ARCONTENT_UNKNOWNCONTENT: DWORD = 0x00000040;
pub const ARCONTENT_AUTOPLAYPIX: DWORD = 0x00000080;
pub const ARCONTENT_AUTOPLAYMUSIC: DWORD = 0x00000100;
pub const ARCONTENT_AUTOPLAYVIDEO: DWORD = 0x00000200;
pub const ARCONTENT_VCD: DWORD = 0x00000400;
pub const ARCONTENT_SVCD: DWORD = 0x00000800;
pub const ARCONTENT_DVDAUDIO: DWORD = 0x00001000;
pub const ARCONTENT_BLANKBD: DWORD = 0x00002000;
pub const ARCONTENT_BLURAY: DWORD = 0x00004000;
pub const ARCONTENT_CAMERASTORAGE: DWORD = 0x00008000;
pub const ARCONTENT_CUSTOMEVENT: DWORD = 0x00010000;
pub const ARCONTENT_NONE: DWORD = 0x00000000;
pub const ARCONTENT_MASK: DWORD = 0x0001FFFE;
pub const ARCONTENT_PHASE_UNKNOWN: DWORD = 0x00000000;
pub const ARCONTENT_PHASE_PRESNIFF: DWORD = 0x10000000;
pub const ARCONTENT_PHASE_SNIFFING: DWORD = 0x20000000;
pub const ARCONTENT_PHASE_FINAL: DWORD = 0x40000000;
pub const ARCONTENT_PHASE_MASK: DWORD = 0x70000000;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0059_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0059_v0_0_s_ifspec;
ENUM!{enum _SPBEGINF {
    SPBEGINF_NORMAL = 0,
    SPBEGINF_AUTOTIME = 0x2,
    SPBEGINF_NOPROGRESSBAR = 0x10,
    SPBEGINF_MARQUEEPROGRESS = 0x20,
    SPBEGINF_NOCANCELBUTTON = 0x40,
}}
pub type SPBEGINF = DWORD;
ENUM!{enum SPACTION {
    SPACTION_NONE = 0,
    SPACTION_MOVING = SPACTION_NONE + 1,
    SPACTION_COPYING = SPACTION_MOVING + 1,
    SPACTION_RECYCLING = SPACTION_COPYING + 1,
    SPACTION_APPLYINGATTRIBS = SPACTION_RECYCLING + 1,
    SPACTION_DOWNLOADING = SPACTION_APPLYINGATTRIBS + 1,
    SPACTION_SEARCHING_INTERNET = SPACTION_DOWNLOADING + 1,
    SPACTION_CALCULATING = SPACTION_SEARCHING_INTERNET + 1,
    SPACTION_UPLOADING = SPACTION_CALCULATING + 1,
    SPACTION_SEARCHING_FILES = SPACTION_UPLOADING + 1,
    SPACTION_DELETING = SPACTION_SEARCHING_FILES + 1,
    SPACTION_RENAMING = SPACTION_DELETING + 1,
    SPACTION_FORMATTING = SPACTION_RENAMING + 1,
    SPACTION_COPY_MOVING = SPACTION_FORMATTING + 1,
}}
ENUM!{enum SPTEXT {
    SPTEXT_ACTIONDESCRIPTION = 1,
    SPTEXT_ACTIONDETAIL = SPTEXT_ACTIONDESCRIPTION + 1,
}}
DEFINE_GUID!{IID_IActionProgress,
    0x49ff1173, 0xeadc, 0x446d, 0x92, 0x85, 0x15, 0x64, 0x53, 0xa6, 0x43, 0x1c}
RIDL!{#[uuid(0x49ff1173, 0xeadc, 0x446d, 0x92, 0x85, 0x15, 0x64, 0x53, 0xa6, 0x43, 0x1c)]
interface IActionProgress(IActionProgressVtbl): IUnknown(IUnknownVtbl) {
    fn Begin(
        action: SPACTION,
        flags: SPBEGINF,
    ) -> HRESULT,
    fn UpdateProgress(
        ulCompleted: ULONGLONG,
        ulTotal: ULONGLONG,
    ) -> HRESULT,
    fn UpdateText(
        sptext: SPTEXT,
        pszText: LPCWSTR,
        fMayCompact: BOOL,
    ) -> HRESULT,
    fn QueryCancel(
        pfCancelled: *mut BOOL,
    ) -> HRESULT,
    fn ResetCancel() -> HRESULT,
    fn End() -> HRESULT,
}}
DEFINE_GUID!{IID_IShellExtInit,
    0x000214e8, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e8, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellExtInit(IShellExtInitVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pidlFolder: PCIDLIST_ABSOLUTE,
        pdtobj: *mut IDataObject,
        hkeyProgID: HKEY,
    ) -> HRESULT,
}}
pub type LPSHELLEXTINIT = *mut IShellExtInit;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0061_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0061_v0_0_s_ifspec;
ENUM!{enum _EXPPS {
    EXPPS_FILETYPES = 0x1,
}}
pub type EXPPS = UINT;
DEFINE_GUID!{IID_IShellPropSheetExt,
    0x000214e9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214e9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellPropSheetExt(IShellPropSheetExtVtbl): IUnknown(IUnknownVtbl) {
    fn AddPages(
        pfnAddPage: LPFNSVADDPROPSHEETPAGE,
        lParam: LPARAM,
    ) -> HRESULT,
    fn ReplacePage(
        uPageID: EXPPS,
        pfnReplaceWith: LPFNSVADDPROPSHEETPAGE,
        lParam: LPARAM,
    ) -> HRESULT,
}}
pub type LPSHELLPROPSHEETEXT = *mut IShellPropSheetExt;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0062_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0062_v0_0_s_ifspec;
DEFINE_GUID!{IID_IRemoteComputer,
    0x000214fe, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x000214fe, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IRemoteComputer(IRemoteComputerVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pszMachine: LPCWSTR,
        bEnumerating: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IQueryContinue,
    0x7307055c, 0xb24a, 0x486b, 0x9f, 0x25, 0x16, 0x3e, 0x59, 0x7a, 0x28, 0xa9}
RIDL!{#[uuid(0x7307055c, 0xb24a, 0x486b, 0x9f, 0x25, 0x16, 0x3e, 0x59, 0x7a, 0x28, 0xa9)]
interface IQueryContinue(IQueryContinueVtbl): IUnknown(IUnknownVtbl) {
    fn QueryContinue() -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectWithCancelEvent,
    0xf279b885, 0x0ae9, 0x4b85, 0xac, 0x06, 0xdd, 0xec, 0xf9, 0x40, 0x89, 0x41}
RIDL!{#[uuid(0xf279b885, 0x0ae9, 0x4b85, 0xac, 0x06, 0xdd, 0xec, 0xf9, 0x40, 0x89, 0x41)]
interface IObjectWithCancelEvent(IObjectWithCancelEventVtbl): IUnknown(IUnknownVtbl) {
    fn GetCancelEvent(
        phEvent: *mut HANDLE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IUserNotification,
    0xba9711ba, 0x5893, 0x4787, 0xa7, 0xe1, 0x41, 0x27, 0x71, 0x51, 0x55, 0x0b}
RIDL!{#[uuid(0xba9711ba, 0x5893, 0x4787, 0xa7, 0xe1, 0x41, 0x27, 0x71, 0x51, 0x55, 0x0b)]
interface IUserNotification(IUserNotificationVtbl): IUnknown(IUnknownVtbl) {
    fn SetBalloonInfo(
        pszTitle: LPCWSTR,
        pszText: LPCWSTR,
        dwInfoFlags: DWORD,
    ) -> HRESULT,
    fn SetBalloonRetry(
        dwShowTime: DWORD,
        dwInterval: DWORD,
        cRetryCount: UINT,
    ) -> HRESULT,
    fn SetIconInfo(
        hIcon: HICON,
        pszToolTip: LPCWSTR,
    ) -> HRESULT,
    fn Show(
        pqc: *mut IQueryContinue,
        dwContinuePollInterval: DWORD,
    ) -> HRESULT,
    fn PlaySound(
        pszSoundName: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IItemNameLimits,
    0x1df0d7f1, 0xb267, 0x4d28, 0x8b, 0x10, 0x12, 0xe2, 0x32, 0x02, 0xa5, 0xc4}
RIDL!{#[uuid(0x1df0d7f1, 0xb267, 0x4d28, 0x8b, 0x10, 0x12, 0xe2, 0x32, 0x02, 0xa5, 0xc4)]
interface IItemNameLimits(IItemNameLimitsVtbl): IUnknown(IUnknownVtbl) {
    fn GetValidCharacters(
        ppwszValidChars: *mut LPWSTR,
        ppwszInvalidChars: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMaxLength(
        pszName: LPCWSTR,
        piMaxNameLen: *mut c_int,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0067_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0067_v0_0_s_ifspec;
DEFINE_GUID!{IID_ISearchFolderItemFactory,
    0xa0ffbc28, 0x5482, 0x4366, 0xbe, 0x27, 0x3e, 0x81, 0xe7, 0x8e, 0x06, 0xc2}
RIDL!{#[uuid(0xa0ffbc28, 0x5482, 0x4366, 0xbe, 0x27, 0x3e, 0x81, 0xe7, 0x8e, 0x06, 0xc2)]
interface ISearchFolderItemFactory(ISearchFolderItemFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn SetDisplayName(
        pszDisplayName: LPCWSTR,
    ) -> HRESULT,
    fn SetFolderTypeID(
        ftid: FOLDERTYPEID,
    ) -> HRESULT,
    fn SetFolderLogicalViewMode(
        flvm: FOLDERLOGICALVIEWMODE,
    ) -> HRESULT,
    fn SetIconSize(
        iIconSize: c_int,
    ) -> HRESULT,
    fn SetVisibleColumns(
        cVisibleColumns: UINT,
        rgKey: *const PROPERTYKEY,
    ) -> HRESULT,
    fn SetSortColumns(
        cSortColumns: UINT,
        rgSortColumns: *mut SORTCOLUMN,
    ) -> HRESULT,
    fn SetGroupColumn(
        keyGroup: REFPROPERTYKEY,
    ) -> HRESULT,
    fn SetStacks(
        cStackKeys: UINT,
        rgStackKeys: *mut PROPERTYKEY,
    ) -> HRESULT,
    fn SetScope(
        psiaScope: *mut IShellItemArray,
    ) -> HRESULT,
    fn SetCondition(
        pCondition: *mut ICondition,
    ) -> HRESULT,
    fn GetShellItem(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
pub const IEI_PRIORITY_MAX: DWORD = ITSAT_MAX_PRIORITY;
pub const IEI_PRIORITY_MIN: DWORD = ITSAT_MIN_PRIORITY;
pub const IEIT_PRIORITY_NORMAL: DWORD = ITSAT_DEFAULT_PRIORITY;
pub const IEIFLAG_ASYNC: DWORD = 0x0001;
pub const IEIFLAG_CACHE: DWORD = 0x0002;
pub const IEIFLAG_ASPECT: DWORD = 0x0004;
pub const IEIFLAG_OFFLINE: DWORD = 0x0008;
pub const IEIFLAG_GLEAM: DWORD = 0x0010;
pub const IEIFLAG_SCREEN: DWORD = 0x0020;
pub const IEIFLAG_ORIGSIZE: DWORD = 0x0040;
pub const IEIFLAG_NOSTAMP: DWORD = 0x0080;
pub const IEIFLAG_NOBORDER: DWORD = 0x0100;
pub const IEIFLAG_QUALITY: DWORD = 0x0200;
pub const IEIFLAG_REFRESH: DWORD = 0x0400;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0068_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0068_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExtractImage,
    0xbb2e617c, 0x0920, 0x11d1, 0x9a, 0x0b, 0x00, 0xc0, 0x4f, 0xc2, 0xd6, 0xc1}
RIDL!{#[uuid(0xbb2e617c, 0x0920, 0x11d1, 0x9a, 0x0b, 0x00, 0xc0, 0x4f, 0xc2, 0xd6, 0xc1)]
interface IExtractImage(IExtractImageVtbl): IUnknown(IUnknownVtbl) {
    fn GetLocation(
        pszPathBuffer: LPWSTR,
        cch: DWORD,
        pdwPriority: *mut DWORD,
        prgSize: *const SIZE,
        dwRecClrDepth: DWORD,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn Extract(
        phBmpThumbnail: *mut HBITMAP,
    ) -> HRESULT,
}}
pub type LPEXTRACTIMAGE = *mut IExtractImage;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0069_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0069_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExtractImage2,
    0x953bb1ee, 0x93b4, 0x11d1, 0x98, 0xa3, 0x00, 0xc0, 0x4f, 0xb6, 0x87, 0xda}
RIDL!{#[uuid(0x953bb1ee, 0x93b4, 0x11d1, 0x98, 0xa3, 0x00, 0xc0, 0x4f, 0xb6, 0x87, 0xda)]
interface IExtractImage2(IExtractImage2Vtbl): IExtractImage(IExtractImageVtbl) {
    fn GetDateStamp(
        pDateStamp: *mut FILETIME,
    ) -> HRESULT,
}}
pub type LPEXTRACTIMAGE2 = *mut IExtractImage2;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0070_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0070_v0_0_s_ifspec;
DEFINE_GUID!{IID_IThumbnailHandlerFactory,
    0xe35b4b2e, 0x00da, 0x4bc1, 0x9f, 0x13, 0x38, 0xbc, 0x11, 0xf5, 0xd4, 0x17}
RIDL!{#[uuid(0xe35b4b2e, 0x00da, 0x4bc1, 0x9f, 0x13, 0x38, 0xbc, 0x11, 0xf5, 0xd4, 0x17)]
interface IThumbnailHandlerFactory(IThumbnailHandlerFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetThumbnailHandler(
        pidlChild: PCUITEMID_CHILD,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IParentAndItem,
    0xb3a4b685, 0xb685, 0x4805, 0x99, 0xd9, 0x5d, 0xea, 0xd2, 0x87, 0x32, 0x36}
RIDL!{#[uuid(0xb3a4b685, 0xb685, 0x4805, 0x99, 0xd9, 0x5d, 0xea, 0xd2, 0x87, 0x32, 0x36)]
interface IParentAndItem(IParentAndItemVtbl): IUnknown(IUnknownVtbl) {
    fn SetParentAndItem(
        pidlParent: PCIDLIST_ABSOLUTE,
        psf: *mut IShellFolder,
        pidlChild: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn GetParentAndItem(
        ppidlParent: *mut PIDLIST_ABSOLUTE,
        ppsf: *mut *mut IShellFolder,
        ppidlChild: *mut PITEMID_CHILD,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IParentAndItem_RemoteGetParentAndItem_Proxy(
//     IParentAndItem * This,
//     PIDLIST_ABSOLUTE *ppidlParent,
//     IShellFolder **ppsf,
//     PITEMID_CHILD *ppidlChild);
// c_void __RPC_STUB IParentAndItem_RemoteGetParentAndItem_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_IDockingWindow,
    0x012dd920, 0x7b26, 0x11d0, 0x8c, 0xa9, 0x00, 0xa0, 0xc9, 0x2d, 0xbf, 0xe8}
RIDL!{#[uuid(0x012dd920, 0x7b26, 0x11d0, 0x8c, 0xa9, 0x00, 0xa0, 0xc9, 0x2d, 0xbf, 0xe8)]
interface IDockingWindow(IDockingWindowVtbl): IOleWindow(IOleWindowVtbl) {
    fn ShowDW(
        fShow: BOOL,
    ) -> HRESULT,
    fn CloseDW(
        dwReserved: DWORD,
    ) -> HRESULT,
    fn ResizeBorderDW(
        prcBorder: LPCRECT,
        punkToolbarSite: *mut IUnknown,
        fReserved: BOOL,
    ) -> HRESULT,
}}
pub const DBIM_MINSIZE: DWORD = 0x0001;
pub const DBIM_MAXSIZE: DWORD = 0x0002;
pub const DBIM_INTEGRAL: DWORD = 0x0004;
pub const DBIM_ACTUAL: DWORD = 0x0008;
pub const DBIM_TITLE: DWORD = 0x0010;
pub const DBIM_MODEFLAGS: DWORD = 0x0020;
pub const DBIM_BKCOLOR: DWORD = 0x0040;
STRUCT!{struct DESKBANDINFO {
    dwMask: DWORD,
    ptMinSize: POINTL,
    ptMaxSize: POINTL,
    ptIntegral: POINTL,
    ptActual: POINTL,
    wszTitle: [WCHAR; 256],
    dwModeFlags: DWORD,
    crBkgnd: COLORREF,
}}
pub const DBIMF_NORMAL: DWORD = 0x0000;
pub const DBIMF_FIXED: DWORD = 0x0001;
pub const DBIMF_FIXEDBMP: DWORD = 0x0004;
pub const DBIMF_VARIABLEHEIGHT: DWORD = 0x0008;
pub const DBIMF_UNDELETEABLE: DWORD = 0x0010;
pub const DBIMF_DEBOSSED: DWORD = 0x0020;
pub const DBIMF_BKCOLOR: DWORD = 0x0040;
pub const DBIMF_USECHEVRON: DWORD = 0x0080;
pub const DBIMF_BREAK: DWORD = 0x0100;
pub const DBIMF_ADDTOFRONT: DWORD = 0x0200;
pub const DBIMF_TOPALIGN: DWORD = 0x0400;
pub const DBIMF_NOGRIPPER: DWORD = 0x0800;
pub const DBIMF_ALWAYSGRIPPER: DWORD = 0x1000;
pub const DBIMF_NOMARGINS: DWORD = 0x2000;
pub const DBIF_VIEWMODE_NORMAL: DWORD = 0x0000;
pub const DBIF_VIEWMODE_VERTICAL: DWORD = 0x0001;
pub const DBIF_VIEWMODE_FLOATING: DWORD = 0x0002;
pub const DBIF_VIEWMODE_TRANSPARENT: DWORD = 0x0004;
ENUM!{enum tagDESKBANDCID {
    DBID_BANDINFOCHANGED = 0,
    DBID_SHOWONLY = 1,
    DBID_MAXIMIZEBAND = 2,
    DBID_PUSHCHEVRON = 3,
    DBID_DELAYINIT = 4,
    DBID_FINISHINIT = 5,
    DBID_SETWINDOWTHEME = 6,
    DBID_PERMITAUTOHIDE	= 7,
}}
pub const DBPC_SELECTFIRST: DWORD = -1i32 as u32;
pub const DBPC_SELECTLAST: DWORD = -2i32 as u32;
// #define CGID_DeskBand IID_IDeskBand
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0073_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0073_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDeskBand,
    0xeb0fe172, 0x1a3a, 0x11d0, 0x89, 0xb3, 0x00, 0xa0, 0xc9, 0x0a, 0x90, 0xac}
RIDL!{#[uuid(0xeb0fe172, 0x1a3a, 0x11d0, 0x89, 0xb3, 0x00, 0xa0, 0xc9, 0x0a, 0x90, 0xac)]
interface IDeskBand(IDeskBandVtbl): IDockingWindow(IDockingWindowVtbl) {
    fn GetBandInfo(
        dwBandID: DWORD,
        dwViewMode: DWORD,
        pdbi: *mut DESKBANDINFO,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0074_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0074_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDeskBandInfo,
    0x77e425fc, 0xcbf9, 0x4307, 0xba, 0x6a, 0xbb, 0x57, 0x27, 0x74, 0x56, 0x61}
RIDL!{#[uuid(0x77e425fc, 0xcbf9, 0x4307, 0xba, 0x6a, 0xbb, 0x57, 0x27, 0x74, 0x56, 0x61)]
interface IDeskBandInfo(IDeskBandInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetDefaultBandWidth(
        dwBandID: DWORD,
        dwViewMode: DWORD,
        pnWidth: *mut c_int,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0075_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0075_v0_0_s_ifspec;
DEFINE_GUID!{IID_ITaskbarList,
    0x56fdf342, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90}
RIDL!{#[uuid(0x56fdf342, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
interface ITaskbarList(ITaskbarListVtbl): IUnknown(IUnknownVtbl) {
    fn HrInit() -> HRESULT,
    fn AddTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn DeleteTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn ActivateTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn SetActiveAlt(
        hwnd: HWND,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITaskbarList2,
    0x602d4995, 0xb13a, 0x429b, 0xa6, 0x6e, 0x19, 0x35, 0xe4, 0x4f, 0x43, 0x17}
RIDL!{#[uuid(0x602d4995, 0xb13a, 0x429b, 0xa6, 0x6e, 0x19, 0x35, 0xe4, 0x4f, 0x43, 0x17)]
interface ITaskbarList2(ITaskbarList2Vtbl): ITaskbarList(ITaskbarListVtbl) {
    fn MarkFullscreenWindow(
        hwnd: HWND,
        fFullscreen: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum THUMBBUTTONFLAGS {
    THBF_ENABLED = 0,
    THBF_DISABLED = 0x1,
    THBF_DISMISSONCLICK = 0x2,
    THBF_NOBACKGROUND = 0x4,
    THBF_HIDDEN = 0x8,
    THBF_NONINTERACTIVE = 0x10,
}}
ENUM!{enum THUMBBUTTONMASK {
    THB_BITMAP = 0x1,
    THB_ICON = 0x2,
    THB_TOOLTIP = 0x4,
    THB_FLAGS = 0x8,
}}
STRUCT!{struct THUMBBUTTON {
    dwMask: THUMBBUTTONMASK,
    iId: UINT,
    iBitmap: UINT,
    hIcon: HICON,
    szTip: [WCHAR; 260],
    dwFlags: THUMBBUTTONFLAGS,
}}
pub type LPTHUMBBUTTON = *mut THUMBBUTTON;
pub const THBN_CLICKED: WORD = 0x1800;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0077_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0077_v0_0_s_ifspec;
ENUM!{enum TBPFLAG {
    TBPF_NOPROGRESS = 0,
    TBPF_INDETERMINATE = 0x1,
    TBPF_NORMAL = 0x2,
    TBPF_ERROR = 0x4,
    TBPF_PAUSED = 0x8,
}}
DEFINE_GUID!{IID_ITaskbarList3,
    0xea1afb91, 0x9e28, 0x4b86, 0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf}
RIDL!{#[uuid(0xea1afb91, 0x9e28, 0x4b86, 0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf)]
interface ITaskbarList3(ITaskbarList3Vtbl): ITaskbarList2(ITaskbarList2Vtbl) {
    fn SetProgressValue(
        hwnd: HWND,
        ullCompleted: ULONGLONG,
        ullTotal: ULONGLONG,
    ) -> HRESULT,
    fn SetProgressState(
        hwnd: HWND,
        tbpFlags: TBPFLAG,
    ) -> HRESULT,
    fn RegisterTab(
        hwndTab: HWND,
        hwndMDI: HWND,
    ) -> HRESULT,
    fn UnregisterTab(
        hwndTab: HWND,
    ) -> HRESULT,
    fn SetTabOrder(
        hwndTab: HWND,
        hwndInsertBefore: HWND,
    ) -> HRESULT,
    fn SetTabActive(
        hwndTab: HWND,
        hwndMDI: HWND,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn ThumbBarAddButtons(
        hwnd: HWND,
        cButtons: UINT,
        pButton: LPTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarUpdateButtons(
        hwnd: HWND,
        cButtons: UINT,
        pButton: LPTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarSetImageList(
        hwnd: HWND,
        himl: HIMAGELIST,
    ) -> HRESULT,
    fn SetOverlayIcon(
        hwnd: HWND,
        hIcon: HICON,
        pszDescription: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailTooltip(
        hwnd: HWND,
        pszTip: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailClip(
        hwnd: HWND,
        prcClip: *mut RECT,
    ) -> HRESULT,
}}
ENUM!{enum STPFLAG {
    STPF_NONE = 0,
    STPF_USEAPPTHUMBNAILALWAYS = 0x1,
    STPF_USEAPPTHUMBNAILWHENACTIVE = 0x2,
    STPF_USEAPPPEEKALWAYS = 0x4,
    STPF_USEAPPPEEKWHENACTIVE = 0x8,
}}
DEFINE_GUID!{IID_ITaskbarList4,
    0xc43dc798, 0x95d1, 0x4bea, 0x90, 0x30, 0xbb, 0x99, 0xe2, 0x98, 0x3a, 0x1a}
RIDL!{#[uuid(0xc43dc798, 0x95d1, 0x4bea, 0x90, 0x30, 0xbb, 0x99, 0xe2, 0x98, 0x3a, 0x1a)]
interface ITaskbarList4(ITaskbarList4Vtbl): ITaskbarList3(ITaskbarList3Vtbl) {
    fn SetTabProperties(
        hwndTab: HWND,
        stpFlags: STPFLAG,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0079_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0079_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExplorerBrowserEvents,
    0x361bbdc7, 0xe6ee, 0x4e13, 0xbe, 0x58, 0x58, 0xe2, 0x24, 0x0c, 0x81, 0x0f}
RIDL!{#[uuid(0x361bbdc7, 0xe6ee, 0x4e13, 0xbe, 0x58, 0x58, 0xe2, 0x24, 0x0c, 0x81, 0x0f)]
interface IExplorerBrowserEvents(IExplorerBrowserEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnNavigationPending(
        pidlFolder: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn OnViewCreated(
        psv: *mut IShellView,
    ) -> HRESULT,
    fn OnNavigationComplete(
        pidlFolder: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn OnNavigationFailed(
        pidlFolder: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
ENUM!{enum EXPLORER_BROWSER_OPTIONS {
    EBO_NONE = 0,
    EBO_NAVIGATEONCE = 0x1,
    EBO_SHOWFRAMES = 0x2,
    EBO_ALWAYSNAVIGATE = 0x4,
    EBO_NOTRAVELLOG = 0x8,
    EBO_NOWRAPPERWINDOW = 0x10,
    EBO_HTMLSHAREPOINTVIEW = 0x20,
    EBO_NOBORDER = 0x40,
    EBO_NOPERSISTVIEWSTATE = 0x80,
}}
ENUM!{enum EXPLORER_BROWSER_FILL_FLAGS {
    EBF_NONE = 0,
    EBF_SELECTFROMDATAOBJECT = 0x100,
    EBF_NODROPTARGET = 0x200,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0080_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0080_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExplorerBrowser,
    0xdfd3b6b5, 0xc10c, 0x4be9, 0x85, 0xf6, 0xa6, 0x69, 0x69, 0xf4, 0x02, 0xf6}
RIDL!{#[uuid(0xdfd3b6b5, 0xc10c, 0x4be9, 0x85, 0xf6, 0xa6, 0x69, 0x69, 0xf4, 0x02, 0xf6)]
interface IExplorerBrowser(IExplorerBrowserVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hwndParent: HWND,
        prc: *const RECT,
        pfs: *const FOLDERSETTINGS,
    ) -> HRESULT,
    fn Destroy() -> HRESULT,
    fn SetRect(
        phdwp: *mut HDWP,
        rcBrowser: RECT,
    ) -> HRESULT,
    fn SetPropertyBag(
        pszPropertyBag: LPCWSTR,
    ) -> HRESULT,
    fn SetEmptyText(
        pszEmptyText: LPCWSTR,
    ) -> HRESULT,
    fn SetFolderSettings(
        pfs: *const FOLDERSETTINGS,
    ) -> HRESULT,
    fn Advise(
        psbe: *mut IExplorerBrowserEvents,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn SetOptions(
        dwFlag: EXPLORER_BROWSER_OPTIONS,
    ) -> HRESULT,
    fn GetOptions(
        pdwFlag: *mut EXPLORER_BROWSER_OPTIONS,
    ) -> HRESULT,
    fn BrowseToIDList(
        pidl: PCUIDLIST_RELATIVE,
        uFlags: UINT,
    ) -> HRESULT,
    fn BrowseToObject(
        punk: *mut IUnknown,
        uFlags: UINT,
    ) -> HRESULT,
    fn FillFromObject(
        punk: *mut IUnknown,
        dwFlags: EXPLORER_BROWSER_FILL_FLAGS,
    ) -> HRESULT,
    fn RemoveAll() -> HRESULT,
    fn GetCurrentView(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0081_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0081_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumObjects,
    0x2c1c7e2e, 0x2d0e, 0x4059, 0x83, 0x1e, 0x1e, 0x6f, 0x82, 0x33, 0x5c, 0x2e}
RIDL!{#[uuid(0x2c1c7e2e, 0x2d0e, 0x4059, 0x83, 0x1e, 0x1e, 0x6f, 0x82, 0x33, 0x5c, 0x2e)]
interface IEnumObjects(IEnumObjectsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        riid: REFIID,
        rgelt: *mut *mut c_void,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumObjects,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumObjects_RemoteNext_Proxy(
//     IEnumObjects * This,
//     ULONG celt,
//     REFIID riid,
//     c_void **rgelt,
//     ULONG *pceltFetched);
// c_void __RPC_STUB IEnumObjects_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
ENUM!{enum _OPPROGDLGF {
    OPPROGDLG_DEFAULT = 0,
    OPPROGDLG_ENABLEPAUSE = 0x80,
    OPPROGDLG_ALLOWUNDO = 0x100,
    OPPROGDLG_DONTDISPLAYSOURCEPATH = 0x200,
    OPPROGDLG_DONTDISPLAYDESTPATH = 0x400,
    OPPROGDLG_NOMULTIDAYESTIMATES = 0x800,
    OPPROGDLG_DONTDISPLAYLOCATIONS = 0x1000,
}}
pub type OPPROGDLGF = DWORD;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0082_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0082_v0_0_s_ifspec;
ENUM!{enum _PDMODE {
    PDM_DEFAULT = 0,
    PDM_RUN = 0x1,
    PDM_PREFLIGHT = 0x2,
    PDM_UNDOING = 0x4,
    PDM_ERRORSBLOCKING = 0x8,
    PDM_INDETERMINATE = 0x10,
}}
pub type PDMODE = DWORD;
ENUM!{enum PDOPSTATUS {
    PDOPS_RUNNING = 1,
    PDOPS_PAUSED = 2,
    PDOPS_CANCELLED = 3,
    PDOPS_STOPPED = 4,
    PDOPS_ERRORS = 5,
}}
DEFINE_GUID!{IID_IOperationsProgressDialog,
    0x0c9fb851, 0xe5c9, 0x43eb, 0xa3, 0x70, 0xf0, 0x67, 0x7b, 0x13, 0x87, 0x4c}
RIDL!{#[uuid(0x0c9fb851, 0xe5c9, 0x43eb, 0xa3, 0x70, 0xf0, 0x67, 0x7b, 0x13, 0x87, 0x4c)]
interface IOperationsProgressDialog(IOperationsProgressDialogVtbl): IUnknown(IUnknownVtbl) {
    fn StartProgressDialog(
        hwndOwner: HWND,
        flags: OPPROGDLGF,
    ) -> HRESULT,
    fn StopProgressDialog() -> HRESULT,
    fn SetOperation(
        action: SPACTION,
    ) -> HRESULT,
    fn SetMode(
        mode: PDMODE,
    ) -> HRESULT,
    fn UpdateProgress(
        ullPointsCurrent: ULONGLONG,
        ullPointsTotal: ULONGLONG,
        ullSizeCurrent: ULONGLONG,
        ullSizeTotal: ULONGLONG,
        ullItemsCurrent: ULONGLONG,
        ullItemsTotal: ULONGLONG,
    ) -> HRESULT,
    fn UpdateLocations(
        psiSource: *mut IShellItem,
        psiTarget: *mut IShellItem,
        psiItem: *mut IShellItem,
    ) -> HRESULT,
    fn ResetTimer() -> HRESULT,
    fn PauseTimer() -> HRESULT,
    fn ResumeTimer() -> HRESULT,
    fn GetMilliseconds(
        pullElapsed: *mut ULONGLONG,
        pullRemaining: *mut ULONGLONG,
    ) -> HRESULT,
    fn GetOperationStatus(
        popstatus: *mut PDOPSTATUS,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IIOCancelInformation,
    0xf5b0bf81, 0x8cb5, 0x4b1b, 0x94, 0x49, 0x1a, 0x15, 0x9e, 0x0c, 0x73, 0x3c}
RIDL!{#[uuid(0xf5b0bf81, 0x8cb5, 0x4b1b, 0x94, 0x49, 0x1a, 0x15, 0x9e, 0x0c, 0x73, 0x3c)]
interface IIOCancelInformation(IIOCancelInformationVtbl): IUnknown(IUnknownVtbl) {
    fn SetCancelInformation(
        dwThreadID: DWORD,
        uMsgCancel: UINT,
    ) -> HRESULT,
    fn GetCancelInformation(
        pdwThreadID: *mut DWORD,
        puMsgCancel: *mut UINT,
    ) -> HRESULT,
}}
pub const FOFX_NOSKIPJUNCTIONS: DWORD = 0x00010000;
pub const FOFX_PREFERHARDLINK: DWORD = 0x00020000;
pub const FOFX_SHOWELEVATIONPROMPT: DWORD = 0x00040000;
pub const FOFX_RECYCLEONDELETE: DWORD = 0x00080000;
pub const FOFX_EARLYFAILURE: DWORD = 0x00100000;
pub const FOFX_PRESERVEFILEEXTENSIONS: DWORD = 0x00200000;
pub const FOFX_KEEPNEWERFILE: DWORD = 0x00400000;
pub const FOFX_NOCOPYHOOKS: DWORD = 0x00800000;
pub const FOFX_NOMINIMIZEBOX: DWORD = 0x01000000;
pub const FOFX_MOVEACLSACROSSVOLUMES: DWORD = 0x02000000;
pub const FOFX_DONTDISPLAYSOURCEPATH: DWORD = 0x04000000;
pub const FOFX_DONTDISPLAYDESTPATH: DWORD = 0x08000000;
pub const FOFX_REQUIREELEVATION: DWORD = 0x10000000;
pub const FOFX_ADDUNDORECORD: DWORD = 0x20000000;
pub const FOFX_COPYASDOWNLOAD: DWORD = 0x40000000;
pub const FOFX_DONTDISPLAYLOCATIONS: DWORD = 0x80000000;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0084_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0084_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileOperation,
    0x947aab5f, 0x0a5c, 0x4c13, 0xb4, 0xd6, 0x4b, 0xf7, 0x83, 0x6f, 0xc9, 0xf8}
RIDL!{#[uuid(0x947aab5f, 0x0a5c, 0x4c13, 0xb4, 0xd6, 0x4b, 0xf7, 0x83, 0x6f, 0xc9, 0xf8)]
interface IFileOperation(IFileOperationVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        pfops: *mut IFileOperationProgressSink,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn SetOperationFlags(
        dwOperationFlags: DWORD,
    ) -> HRESULT,
    fn SetProgressMessage(
        pszMessage: LPCWSTR,
    ) -> HRESULT,
    fn SetProgressDialog(
        popd: *mut IOperationsProgressDialog,
    ) -> HRESULT,
    fn SetProperties(
        pproparray: *mut IPropertyChangeArray,
    ) -> HRESULT,
    fn SetOwnerWindow(
        hwndOwner: HWND,
    ) -> HRESULT,
    fn ApplyPropertiesToItem(
        psiItem: *mut IShellItem,
    ) -> HRESULT,
    fn ApplyPropertiesToItems(
        punkItems: *mut IUnknown,
    ) -> HRESULT,
    fn RenameItem(
        psiItem: *mut IShellItem,
        pszNewName: LPCWSTR,
        pfopsItem: *mut IFileOperationProgressSink,
    ) -> HRESULT,
    fn RenameItems(
        pUnkItems: *mut IUnknown,
        pszNewName: LPCWSTR,
    ) -> HRESULT,
    fn MoveItem(
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszNewName: LPCWSTR,
        pfopsItem: *mut IFileOperationProgressSink,
    ) -> HRESULT,
    fn MoveItems(
        punkItems: *mut IUnknown,
        psiDestinationFolder: *mut IShellItem,
    ) -> HRESULT,
    fn CopyItem(
        psiItem: *mut IShellItem,
        psiDestinationFolder: *mut IShellItem,
        pszCopyName: LPCWSTR,
        pfopsItem: *mut IFileOperationProgressSink,
    ) -> HRESULT,
    fn CopyItems(
        punkItems: *mut IUnknown,
        psiDestinationFolder: *mut IShellItem,
    ) -> HRESULT,
    fn DeleteItem(
        psiItem: *mut IShellItem,
        pfopsItem: *mut IFileOperationProgressSink,
    ) -> HRESULT,
    fn DeleteItems(
        punkItems: *mut IUnknown,
    ) -> HRESULT,
    fn NewItem(
        psiDestinationFolder: *mut IShellItem,
        dwFileAttributes: DWORD,
        pszName: LPCWSTR,
        pszTemplateName: LPCWSTR,
        pfopsItem: *mut IFileOperationProgressSink,
    ) -> HRESULT,
    fn PerformOperations() -> HRESULT,
    fn GetAnyOperationsAborted(
        pfAnyOperationsAborted: *mut BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectProvider,
    0xa6087428, 0x3be3, 0x4d73, 0xb3, 0x08, 0x7c, 0x04, 0xa5, 0x40, 0xbf, 0x1a}
RIDL!{#[uuid(0xa6087428, 0x3be3, 0x4d73, 0xb3, 0x08, 0x7c, 0x04, 0xa5, 0x40, 0xbf, 0x1a)]
interface IObjectProvider(IObjectProviderVtbl): IUnknown(IUnknownVtbl) {
    fn QueryObject(
        guidObject: REFGUID,
        riid: REFIID,
        ppvOut: *mut *mut c_void,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0086_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0086_v0_0_s_ifspec;
DEFINE_GUID!{IID_INamespaceWalkCB,
    0xd92995f8, 0xcf5e, 0x4a76, 0xbf, 0x59, 0xea, 0xd3, 0x9e, 0xa2, 0xb9, 0x7e}
RIDL!{#[uuid(0xd92995f8, 0xcf5e, 0x4a76, 0xbf, 0x59, 0xea, 0xd3, 0x9e, 0xa2, 0xb9, 0x7e)]
interface INamespaceWalkCB(INamespaceWalkCBVtbl): IUnknown(IUnknownVtbl) {
    fn FoundItem(
        psf: *mut IShellFolder,
        pidl: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn EnterFolder(
        psf: *mut IShellFolder,
        pidl: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn LeaveFolder(
        psf: *mut IShellFolder,
        pidl: PCUITEMID_CHILD,
    ) -> HRESULT,
    fn InitializeProgressDialog(
        ppszTitle: *mut LPWSTR,
        ppszCancel: *mut LPWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0087_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0087_v0_0_s_ifspec;
DEFINE_GUID!{IID_INamespaceWalkCB2,
    0x7ac7492b, 0xc38e, 0x438a, 0x87, 0xdb, 0x68, 0x73, 0x78, 0x44, 0xff, 0x70}
RIDL!{#[uuid(0x7ac7492b, 0xc38e, 0x438a, 0x87, 0xdb, 0x68, 0x73, 0x78, 0x44, 0xff, 0x70)]
interface INamespaceWalkCB2(INamespaceWalkCB2Vtbl): INamespaceWalkCB(INamespaceWalkCBVtbl) {
    fn WalkComplete(
        hr: HRESULT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0088_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0088_v0_0_s_ifspec;
ENUM!{enum NAMESPACEWALKFLAG {
    NSWF_DEFAULT = 0,
    NSWF_NONE_IMPLIES_ALL = 0x1,
    NSWF_ONE_IMPLIES_ALL = 0x2,
    NSWF_DONT_TRAVERSE_LINKS = 0x4,
    NSWF_DONT_ACCUMULATE_RESULT = 0x8,
    NSWF_TRAVERSE_STREAM_JUNCTIONS = 0x10,
    NSWF_FILESYSTEM_ONLY = 0x20,
    NSWF_SHOW_PROGRESS = 0x40,
    NSWF_FLAG_VIEWORDER = 0x80,
    NSWF_IGNORE_AUTOPLAY_HIDA = 0x100,
    NSWF_ASYNC = 0x200,
    NSWF_DONT_RESOLVE_LINKS = 0x400,
    NSWF_ACCUMULATE_FOLDERS = 0x800,
    NSWF_DONT_SORT = 0x1000,
    NSWF_USE_TRANSFER_MEDIUM = 0x2000,
    NSWF_DONT_TRAVERSE_STREAM_JUNCTIONS = 0x4000,
    NSWF_ANY_IMPLIES_ALL = 0x8000,
}}
DEFINE_GUID!{IID_INamespaceWalk,
    0x57ced8a7, 0x3f4a, 0x432c, 0x93, 0x50, 0x30, 0xf2, 0x44, 0x83, 0xf7, 0x4f}
RIDL!{#[uuid(0x57ced8a7, 0x3f4a, 0x432c, 0x93, 0x50, 0x30, 0xf2, 0x44, 0x83, 0xf7, 0x4f)]
interface INamespaceWalk(INamespaceWalkVtbl): IUnknown(IUnknownVtbl) {
    fn Walk(
        punkToWalk: *mut IUnknown,
        dwFlags: DWORD,
        cDepth: c_int,
        pnswcb: *mut INamespaceWalkCB,
    ) -> HRESULT,
    fn GetIDArrayResult(
        pcItems: *mut UINT,
        prgpidl: *mut *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
}}
#[inline]
pub unsafe fn FreeIDListArray(ppidls: *mut PIDLIST_RELATIVE, cItems: UINT) {
    for i in 0..(cItems as isize) {
        CoTaskMemFree(ppidls.offset(i) as *mut c_void);
    }
    CoTaskMemFree(ppidls as *mut c_void);
}
#[inline]
pub unsafe fn FreeIDListArrayFull(ppidls: *mut PIDLIST_ABSOLUTE, cItems: UINT) {
    for i in 0..(cItems as isize) {
        CoTaskMemFree(ppidls.offset(i) as *mut c_void);
    }
    CoTaskMemFree(ppidls as *mut c_void);
}
#[inline]
pub unsafe fn FreeIDListArrayChild(ppidls: *mut PITEMID_CHILD, cItems: UINT) {
    for i in 0..(cItems as isize) {
        CoTaskMemFree(ppidls.offset(i) as *mut c_void);
    }
    CoTaskMemFree(ppidls as *mut c_void);
}
STRUCT!{struct BANDSITEINFO {
    dwMask: DWORD,
    dwState: DWORD,
    dwStyle: DWORD,
}}
ENUM!{enum tagBANDSITECID {
    BSID_BANDADDED = 0,
    BSID_BANDREMOVED = BSID_BANDADDED + 1,
}}
pub const BSIM_STATE: DWORD = 0x00000001;
pub const BSIM_STYLE: DWORD = 0x00000002;
pub const BSSF_VISIBLE: DWORD = 0x00000001;
pub const BSSF_NOTITLE: DWORD = 0x00000002;
pub const BSSF_UNDELETEABLE: DWORD = 0x00001000;
pub const BSIS_AUTOGRIPPER: DWORD = 0x00000000;
pub const BSIS_NOGRIPPER: DWORD = 0x00000001;
pub const BSIS_ALWAYSGRIPPER: DWORD = 0x00000002;
pub const BSIS_LEFTALIGN: DWORD = 0x00000004;
pub const BSIS_SINGLECLICK: DWORD = 0x00000008;
pub const BSIS_NOCONTEXTMENU: DWORD = 0x00000010;
pub const BSIS_NODROPTARGET: DWORD = 0x00000020;
pub const BSIS_NOCAPTION: DWORD = 0x00000040;
pub const BSIS_PREFERNOLINEBREAK: DWORD = 0x00000080;
pub const BSIS_LOCKED: DWORD = 0x00000100;
pub const BSIS_PRESERVEORDERDURINGLAYOUT: DWORD = 0x00000200;
pub const BSIS_FIXEDORDER: DWORD = 0x00000400;
// #define SID_SBandSite IID_IBandSite
// #define CGID_BandSite IID_IBandSite
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0089_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0089_v0_0_s_ifspec;
DEFINE_GUID!{IID_IBandSite,
    0x4cf504b0, 0xde96, 0x11d0, 0x8b, 0x3f, 0x00, 0xa0, 0xc9, 0x11, 0xe8, 0xe5}
RIDL!{#[uuid(0x4cf504b0, 0xde96, 0x11d0, 0x8b, 0x3f, 0x00, 0xa0, 0xc9, 0x11, 0xe8, 0xe5)]
interface IBandSite(IBandSiteVtbl): IUnknown(IUnknownVtbl) {
    fn AddBand(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn EnumBands(
        uBand: UINT,
        pdwBandID: *mut DWORD,
    ) -> HRESULT,
    fn QueryBand(
        dwBandID: DWORD,
        ppstb: *mut *mut IDeskBand,
        pdwState: *mut DWORD,
        pszName: LPWSTR,
        cchName: c_int,
    ) -> HRESULT,
    fn SetBandState(
        dwBandID: DWORD,
        dwMask: DWORD,
        dwState: DWORD,
    ) -> HRESULT,
    fn RemoveBand(
        dwBandID: DWORD,
    ) -> HRESULT,
    fn GetBandObject(
        dwBandID: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn SetBandSiteInfo(
        pbsinfo: *const BANDSITEINFO,
    ) -> HRESULT,
    fn GetBandSiteInfo(
        pbsinfo: *mut BANDSITEINFO,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IBandSite_RemoteQueryBand_Proxy(
//     IBandSite * This,
//     DWORD dwBandID,
//     IDeskBand **ppstb,
//     DWORD *pdwState,
//     LPWSTR pszName,
//     c_int cchName);
// c_void __RPC_STUB IBandSite_RemoteQueryBand_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0090_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0090_v0_0_s_ifspec;
DEFINE_GUID!{IID_IModalWindow,
    0xb4db1657, 0x70d7, 0x485e, 0x8e, 0x3e, 0x6f, 0xcb, 0x5a, 0x5c, 0x18, 0x02}
RIDL!{#[uuid(0xb4db1657, 0x70d7, 0x485e, 0x8e, 0x3e, 0x6f, 0xcb, 0x5a, 0x5c, 0x18, 0x02)]
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(
        hwndOwner: HWND,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IModalWindow_RemoteShow_Proxy(
//     IModalWindow * This,
//     HWND hwndOwner);
// c_void __RPC_STUB IModalWindow_RemoteShow_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0091_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0091_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContextMenuSite,
    0x0811aebe, 0x0b87, 0x4c54, 0x9e, 0x72, 0x54, 0x8c, 0xf6, 0x49, 0x01, 0x6b}
RIDL!{#[uuid(0x0811aebe, 0x0b87, 0x4c54, 0x9e, 0x72, 0x54, 0x8c, 0xf6, 0x49, 0x01, 0x6b)]
interface IContextMenuSite(IContextMenuSiteVtbl): IUnknown(IUnknownVtbl) {
    fn DoContextMenuPopup(
        punkContextMenu: *mut IUnknown,
        fFlags: UINT,
        pt: POINT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0092_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0092_v0_0_s_ifspec;
ENUM!{enum tagMENUBANDHANDLERCID {
    MBHANDCID_PIDLSELECT = 0,
}}
DEFINE_GUID!{IID_IMenuBand,
    0x568804cd, 0xcbd7, 0x11d0, 0x98, 0x16, 0x00, 0xc0, 0x4f, 0xd9, 0x19, 0x72}
RIDL!{#[uuid(0x568804cd, 0xcbd7, 0x11d0, 0x98, 0x16, 0x00, 0xc0, 0x4f, 0xd9, 0x19, 0x72)]
interface IMenuBand(IMenuBandVtbl): IUnknown(IUnknownVtbl) {
    fn IsMenuMessage(
        pmsg: *mut MSG,
    ) -> HRESULT,
    fn TranslateMenuMessage(
        pmsg: *mut MSG,
        plRet: *mut LRESULT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IRegTreeItem,
    0xa9521922, 0x0812, 0x4d44, 0x9e, 0xc3, 0x7f, 0xd3, 0x8c, 0x72, 0x6f, 0x3d}
RIDL!{#[uuid(0xa9521922, 0x0812, 0x4d44, 0x9e, 0xc3, 0x7f, 0xd3, 0x8c, 0x72, 0x6f, 0x3d)]
interface IRegTreeItem(IRegTreeItemVtbl): IUnknown(IUnknownVtbl) {
    fn GetCheckState(
        pbCheck: *mut BOOL,
    ) -> HRESULT,
    fn SetCheckState(
        bCheck: BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0094_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0094_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDeskBar,
    0xeb0fe173, 0x1a3a, 0x11d0, 0x89, 0xb3, 0x00, 0xa0, 0xc9, 0x0a, 0x90, 0xac}
RIDL!{#[uuid(0xeb0fe173, 0x1a3a, 0x11d0, 0x89, 0xb3, 0x00, 0xa0, 0xc9, 0x0a, 0x90, 0xac)]
interface IDeskBar(IDeskBarVtbl): IOleWindow(IOleWindowVtbl) {
    fn SetClient(
        punkClient: *mut IUnknown,
    ) -> HRESULT,
    fn GetClient(
        ppunkClient: *mut *mut IUnknown,
    ) -> HRESULT,
    fn OnPosRectChangeDB(
        prc: *mut RECT,
    ) -> HRESULT,
}}
ENUM!{enum tagMENUPOPUPSELECT {
    MPOS_EXECUTE = 0,
    MPOS_FULLCANCEL = MPOS_EXECUTE + 1,
    MPOS_CANCELLEVEL = MPOS_FULLCANCEL + 1,
    MPOS_SELECTLEFT = MPOS_CANCELLEVEL + 1,
    MPOS_SELECTRIGHT = MPOS_SELECTLEFT + 1,
    MPOS_CHILDTRACKING = MPOS_SELECTRIGHT + 1,
}}
ENUM!{enum tagMENUPOPUPPOPUPFLAGS {
    MPPF_SETFOCUS = 0x1,
    MPPF_INITIALSELECT = 0x2,
    MPPF_NOANIMATE = 0x4,
    MPPF_KEYBOARD = 0x10,
    MPPF_REPOSITION = 0x20,
    MPPF_FORCEZORDER = 0x40,
    MPPF_FINALSELECT = 0x80,
    MPPF_TOP = 0x20000000,
    MPPF_LEFT = 0x40000000,
    MPPF_RIGHT = 0x60000000,
    MPPF_BOTTOM = 0x80000000,
    MPPF_POS_MASK = 0xe0000000,
    MPPF_ALIGN_LEFT = 0x2000000,
    MPPF_ALIGN_RIGHT = 0x4000000,
}}
pub type MP_POPUPFLAGS = c_int;
DEFINE_GUID!{IID_IMenuPopup,
    0xd1e7afeb, 0x6a2e, 0x11d0, 0x8c, 0x78, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xb4}
RIDL!{#[uuid(0xd1e7afeb, 0x6a2e, 0x11d0, 0x8c, 0x78, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xb4)]
interface IMenuPopup(IMenuPopupVtbl): IDeskBar(IDeskBarVtbl) {
    fn Popup(
        ppt: *mut POINTL,
        prcExclude: *mut RECTL,
        dwFlags: MP_POPUPFLAGS,
    ) -> HRESULT,
    fn OnSelect(
        dwSelectType: DWORD,
    ) -> HRESULT,
    fn SetSubMenu(
        pmp: *mut IMenuPopup,
        fSet: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum FILE_USAGE_TYPE {
    FUT_PLAYING = 0,
    FUT_EDITING = FUT_PLAYING + 1,
    FUT_GENERIC = FUT_EDITING + 1,
}}
pub const OF_CAP_CANSWITCHTO: DWORD = 0x0001;
pub const OF_CAP_CANCLOSE: DWORD = 0x0002;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0096_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0096_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileIsInUse,
    0x64a1cbf0, 0x3a1a, 0x4461, 0x91, 0x58, 0x37, 0x69, 0x69, 0x69, 0x39, 0x50}
RIDL!{#[uuid(0x64a1cbf0, 0x3a1a, 0x4461, 0x91, 0x58, 0x37, 0x69, 0x69, 0x69, 0x39, 0x50)]
interface IFileIsInUse(IFileIsInUseVtbl): IUnknown(IUnknownVtbl) {
    fn GetAppName(
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetUsage(
        pfut: *mut FILE_USAGE_TYPE,
    ) -> HRESULT,
    fn GetCapabilities(
        pdwCapFlags: *mut DWORD,
    ) -> HRESULT,
    fn GetSwitchToHWND(
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn CloseFile() -> HRESULT,
}}
ENUM!{enum FDE_OVERWRITE_RESPONSE {
    FDEOR_DEFAULT = 0,
    FDEOR_ACCEPT = 1,
    FDEOR_REFUSE = 2,
}}
ENUM!{enum FDE_SHAREVIOLATION_RESPONSE {
    FDESVR_DEFAULT = 0,
    FDESVR_ACCEPT = 1,
    FDESVR_REFUSE = 2,
}}
ENUM!{enum FDAP {
    FDAP_BOTTOM = 0,
    FDAP_TOP = 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0097_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0097_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileDialogEvents,
    0x973510db, 0x7d7f, 0x452b, 0x89, 0x75, 0x74, 0xa8, 0x58, 0x28, 0xd3, 0x54}
RIDL!{#[uuid(0x973510db, 0x7d7f, 0x452b, 0x89, 0x75, 0x74, 0xa8, 0x58, 0x28, 0xd3, 0x54)]
interface IFileDialogEvents(IFileDialogEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnFileOk(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnFolderChanging(
        pfd: *mut IFileDialog,
        psiFolder: *mut IShellItem,
    ) -> HRESULT,
    fn OnFolderChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnSelectionChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnShareViolation(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_SHAREVIOLATION_RESPONSE,
    ) -> HRESULT,
    fn OnTypeChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnOverwrite(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_OVERWRITE_RESPONSE,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0098_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0098_v0_0_s_ifspec;
ENUM!{enum _FILEOPENDIALOGOPTIONS {
    FOS_OVERWRITEPROMPT = 0x2,
    FOS_STRICTFILETYPES = 0x4,
    FOS_NOCHANGEDIR = 0x8,
    FOS_PICKFOLDERS = 0x20,
    FOS_FORCEFILESYSTEM = 0x40,
    FOS_ALLNONSTORAGEITEMS = 0x80,
    FOS_NOVALIDATE = 0x100,
    FOS_ALLOWMULTISELECT = 0x200,
    FOS_PATHMUSTEXIST = 0x800,
    FOS_FILEMUSTEXIST = 0x1000,
    FOS_CREATEPROMPT = 0x2000,
    FOS_SHAREAWARE = 0x4000,
    FOS_NOREADONLYRETURN = 0x8000,
    FOS_NOTESTFILECREATE = 0x10000,
    FOS_HIDEMRUPLACES = 0x20000,
    FOS_HIDEPINNEDPLACES = 0x40000,
    FOS_NODEREFERENCELINKS = 0x100000,
    FOS_OKBUTTONNEEDSINTERACTION = 0x200000,
    FOS_DONTADDTORECENT = 0x2000000,
    FOS_FORCESHOWHIDDEN = 0x10000000,
    FOS_DEFAULTNOMINIMODE = 0x20000000,
    FOS_FORCEPREVIEWPANEON = 0x40000000,
    FOS_SUPPORTSTREAMABLEITEMS = 0x80000000,
}}
pub type FILEOPENDIALOGOPTIONS = DWORD;
DEFINE_GUID!{IID_IFileDialog,
    0x42f85136, 0xdb7e, 0x439c, 0x85, 0xf1, 0xe4, 0x07, 0x5d, 0x13, 0x5f, 0xc8}
RIDL!{#[uuid(0x42f85136, 0xdb7e, 0x439c, 0x85, 0xf1, 0xe4, 0x07, 0x5d, 0x13, 0x5f, 0xc8)]
interface IFileDialog(IFileDialogVtbl): IModalWindow(IModalWindowVtbl) {
    fn SetFileTypes(
        cFileTypes: UINT,
        rgFilterSpec: *const COMDLG_FILTERSPEC,
    ) -> HRESULT,
    fn SetFileTypeIndex(
        iFileType: UINT,
    ) -> HRESULT,
    fn GetFileTypeIndex(
        piFileType: *mut UINT,
    ) -> HRESULT,
    fn Advise(
        pfde: *mut IFileDialogEvents,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn SetOptions(
        fos: FILEOPENDIALOGOPTIONS,
    ) -> HRESULT,
    fn GetOptions(
        pfos: *mut FILEOPENDIALOGOPTIONS,
    ) -> HRESULT,
    fn SetDefaultFolder(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn SetFolder(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetFolder(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetCurrentSelection(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn SetFileName(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetFileName(
        pszName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetTitle(
        pszTitle: LPCWSTR,
    ) -> HRESULT,
    fn SetOkButtonLabel(
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn SetFileNameLabel(
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn GetResult(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn AddPlace(
        psi: *mut IShellItem,
        fdap: FDAP,
    ) -> HRESULT,
    fn SetDefaultExtension(
        pszDefaultExtension: LPCWSTR,
    ) -> HRESULT,
    fn Close(
        hr: HRESULT,
    ) -> HRESULT,
    fn SetClientGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn ClearClientData() -> HRESULT,
    fn SetFilter(
        pFilter: *mut IShellItemFilter,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFileSaveDialog,
    0x84bccd23, 0x5fde, 0x4cdb, 0xae, 0xa4, 0xaf, 0x64, 0xb8, 0x3d, 0x78, 0xab}
RIDL!{#[uuid(0x84bccd23, 0x5fde, 0x4cdb, 0xae, 0xa4, 0xaf, 0x64, 0xb8, 0x3d, 0x78, 0xab)]
interface IFileSaveDialog(IFileSaveDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn SetSaveAsItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn SetProperties(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn SetCollectedProperties(
        pList: *mut IPropertyDescriptionList,
        fAppendDefault: BOOL,
    ) -> HRESULT,
    fn GetProperties(
        ppStore: *mut *mut IPropertyStore,
    ) -> HRESULT,
    fn ApplyProperties(
        psi: *mut IShellItem,
        pStore: *mut IPropertyStore,
        hwnd: HWND,
        pSink: *mut IFileOperationProgressSink,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFileOpenDialog,
    0xd57c7288, 0xd4ad, 0x4768, 0xbe, 0x02, 0x9d, 0x96, 0x95, 0x32, 0xd9, 0x60}
RIDL!{#[uuid(0xd57c7288, 0xd4ad, 0x4768, 0xbe, 0x02, 0x9d, 0x96, 0x95, 0x32, 0xd9, 0x60)]
interface IFileOpenDialog(IFileOpenDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn GetResults(
        ppenum: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSelectedItems(
        ppsai: *mut *mut IShellItemArray,
    ) -> HRESULT,
}}
ENUM!{enum CDCONTROLSTATEF {
    CDCS_INACTIVE = 0,
    CDCS_ENABLED = 0x1,
    CDCS_VISIBLE = 0x2,
    CDCS_ENABLEDVISIBLE = 0x3,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0101_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0101_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileDialogCustomize,
    0xe6fdd21a, 0x163f, 0x4975, 0x9c, 0x8c, 0xa6, 0x9f, 0x1b, 0xa3, 0x70, 0x34}
RIDL!{#[uuid(0xe6fdd21a, 0x163f, 0x4975, 0x9c, 0x8c, 0xa6, 0x9f, 0x1b, 0xa3, 0x70, 0x34)]
interface IFileDialogCustomize(IFileDialogCustomizeVtbl): IUnknown(IUnknownVtbl) {
    fn EnableOpenDropDown(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddMenu(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn AddPushButton(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn AddComboBox(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddRadioButtonList(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddCheckButton(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
        bChecked: BOOL,
    ) -> HRESULT,
    fn AddEditBox(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn AddSeparator(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddText(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn SetControlLabel(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn GetControlState(
        dwIDCtl: DWORD,
        pdwState: *mut CDCONTROLSTATEF,
    ) -> HRESULT,
    fn SetControlState(
        dwIDCtl: DWORD,
        dwState: CDCONTROLSTATEF,
    ) -> HRESULT,
    fn GetEditBoxText(
        dwIDCtl: DWORD,
        ppszText: *mut *mut WCHAR,
    ) -> HRESULT,
    fn SetEditBoxText(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn GetCheckButtonState(
        dwIDCtl: DWORD,
        pbChecked: *mut BOOL,
    ) -> HRESULT,
    fn SetCheckButtonState(
        dwIDCtl: DWORD,
        bChecked: BOOL,
    ) -> HRESULT,
    fn AddControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn RemoveControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn RemoveAllControlItems(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn GetControlItemState(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pdwState: *mut CDCONTROLSTATEF,
    ) -> HRESULT,
    fn SetControlItemState(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        dwState: CDCONTROLSTATEF,
    ) -> HRESULT,
    fn GetSelectedControlItem(
        dwIDCtl: DWORD,
        pdwIDItem: *mut DWORD,
    ) -> HRESULT,
    fn SetSelectedControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn StartVisualGroup(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn EndVisualGroup() -> HRESULT,
    fn MakeProminent(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn SetControlItemText(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
}}
ENUM!{enum ASSOCIATIONLEVEL {
    AL_MACHINE = 0,
    AL_EFFECTIVE = AL_MACHINE + 1,
    AL_USER = AL_EFFECTIVE + 1,
}}
ENUM!{enum ASSOCIATIONTYPE {
    AT_FILEEXTENSION = 0,
    AT_URLPROTOCOL = AT_FILEEXTENSION + 1,
    AT_STARTMENUCLIENT = AT_URLPROTOCOL + 1,
    AT_MIMETYPE = AT_STARTMENUCLIENT + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0102_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0102_v0_0_s_ifspec;
DEFINE_GUID!{IID_IApplicationAssociationRegistration,
    0x4e530b0a, 0xe611, 0x4c77, 0xa3, 0xac, 0x90, 0x31, 0xd0, 0x22, 0x28, 0x1b}
RIDL!{#[uuid(0x4e530b0a, 0xe611, 0x4c77, 0xa3, 0xac, 0x90, 0x31, 0xd0, 0x22, 0x28, 0x1b)]
interface IApplicationAssociationRegistration(IApplicationAssociationRegistrationVtbl):
    IUnknown(IUnknownVtbl)
{
    fn QueryCurrentDefault(
        pszQuery: LPCWSTR,
        atQueryType: ASSOCIATIONTYPE,
        alQueryLevel: ASSOCIATIONLEVEL,
        ppszAssociation: *mut LPWSTR,
    ) -> HRESULT,
    fn QueryAppIsDefault(
        pszQuery: LPCWSTR,
        atQueryType: ASSOCIATIONTYPE,
        alQueryLevel: ASSOCIATIONLEVEL,
        pszAppRegistryName: LPCWSTR,
        pfDefault: *mut BOOL,
    ) -> HRESULT,
    fn QueryAppIsDefaultAll(
        alQueryLevel: ASSOCIATIONLEVEL,
        pszAppRegistryName: LPCWSTR,
        pfDefault: *mut BOOL,
    ) -> HRESULT,
    fn SetAppAsDefault(
        pszAppRegistryName: LPCWSTR,
        pszSet: LPCWSTR,
        atSetType: ASSOCIATIONTYPE,
    ) -> HRESULT,
    fn SetAppAsDefaultAll(
        pszAppRegistryName: LPCWSTR,
    ) -> HRESULT,
    fn ClearUserAssociations() -> HRESULT,
}}
extern "system" {
    pub fn SHCreateAssociationRegistration(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
STRUCT!{#[repr(packed)] struct DELEGATEITEMID {
    cbSize: WORD,
    wOuter: WORD,
    cbInner: WORD,
    rgb: [BYTE; 1],
}}
pub type PCDELEGATEITEMID = *const DELEGATEITEMID;
pub type PDELEGATEITEMID = *mut DELEGATEITEMID;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0103_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0103_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDelegateFolder,
    0xadd8ba80, 0x002b, 0x11d0, 0x8f, 0x0f, 0x00, 0xc0, 0x4f, 0xd7, 0xd0, 0x62}
RIDL!{#[uuid(0xadd8ba80, 0x002b, 0x11d0, 0x8f, 0x0f, 0x00, 0xc0, 0x4f, 0xd7, 0xd0, 0x62)]
interface IDelegateFolder(IDelegateFolderVtbl): IUnknown(IUnknownVtbl) {
    fn SetItemAlloc(
        pmalloc: *mut IMalloc,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0104_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0104_v0_0_s_ifspec;
pub type LPBROWSERFRAMEOPTIONS = *mut IBrowserFrameOptions;
ENUM!{enum _BROWSERFRAMEOPTIONS {
    BFO_NONE = 0,
    BFO_BROWSER_PERSIST_SETTINGS = 0x1,
    BFO_RENAME_FOLDER_OPTIONS_TOINTERNET = 0x2,
    BFO_BOTH_OPTIONS = 0x4,
    BIF_PREFER_INTERNET_SHORTCUT = 0x8,
    BFO_BROWSE_NO_IN_NEW_PROCESS = 0x10,
    BFO_ENABLE_HYPERLINK_TRACKING = 0x20,
    BFO_USE_IE_OFFLINE_SUPPORT = 0x40,
    BFO_SUBSTITUE_INTERNET_START_PAGE = 0x80,
    BFO_USE_IE_LOGOBANDING = 0x100,
    BFO_ADD_IE_TOCAPTIONBAR = 0x200,
    BFO_USE_DIALUP_REF = 0x400,
    BFO_USE_IE_TOOLBAR = 0x800,
    BFO_NO_PARENT_FOLDER_SUPPORT = 0x1000,
    BFO_NO_REOPEN_NEXT_RESTART = 0x2000,
    BFO_GO_HOME_PAGE = 0x4000,
    BFO_PREFER_IEPROCESS = 0x8000,
    BFO_SHOW_NAVIGATION_CANCELLED = 0x10000,
    BFO_USE_IE_STATUSBAR = 0x20000,
    BFO_QUERY_ALL = 0xffffffff,
}}
pub type BROWSERFRAMEOPTIONS = DWORD;
DEFINE_GUID!{IID_IBrowserFrameOptions,
    0x10df43c8, 0x1dbe, 0x11d3, 0x8b, 0x34, 0x00, 0x60, 0x97, 0xdf, 0x5b, 0xd4}
RIDL!{#[uuid(0x10df43c8, 0x1dbe, 0x11d3, 0x8b, 0x34, 0x00, 0x60, 0x97, 0xdf, 0x5b, 0xd4)]
interface IBrowserFrameOptions(IBrowserFrameOptionsVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrameOptions(
        dwMask: BROWSERFRAMEOPTIONS,
        pdwOptions: *mut BROWSERFRAMEOPTIONS,
    ) -> HRESULT,
}}
ENUM!{enum NWMF {
    NWMF_UNLOADING = 0x1,
    NWMF_USERINITED = 0x2,
    NWMF_FIRST = 0x4,
    NWMF_OVERRIDEKEY = 0x8,
    NWMF_SHOWHELP = 0x10,
    NWMF_HTMLDIALOG = 0x20,
    NWMF_FROMDIALOGCHILD = 0x40,
    NWMF_USERREQUESTED = 0x80,
    NWMF_USERALLOWED = 0x100,
    NWMF_FORCEWINDOW = 0x10000,
    NWMF_FORCETAB = 0x20000,
    NWMF_SUGGESTWINDOW = 0x40000,
    NWMF_SUGGESTTAB = 0x80000,
    NWMF_INACTIVETAB = 0x100000,
}}
// #define SID_SNewWindowManager IID_INewWindowManager
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0105_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0105_v0_0_s_ifspec;
DEFINE_GUID!{IID_INewWindowManager,
    0xd2bc4c84, 0x3f72, 0x4a52, 0xa6, 0x04, 0x7b, 0xcb, 0xf3, 0x98, 0x2c, 0xbb}
RIDL!{#[uuid(0xd2bc4c84, 0x3f72, 0x4a52, 0xa6, 0x04, 0x7b, 0xcb, 0xf3, 0x98, 0x2c, 0xbb)]
interface INewWindowManager(INewWindowManagerVtbl): IUnknown(IUnknownVtbl) {
    fn EvaluateNewWindow(
        pszUrl: LPCWSTR,
        pszName: LPCWSTR,
        pszUrlContext: LPCWSTR,
        pszFeatures: LPCWSTR,
        fReplace: BOOL,
        dwFlags: DWORD,
        dwUserActionTime: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum ATTACHMENT_PROMPT {
    ATTACHMENT_PROMPT_NONE = 0,
    ATTACHMENT_PROMPT_SAVE = 0x1,
    ATTACHMENT_PROMPT_EXEC = 0x2,
    ATTACHMENT_PROMPT_EXEC_OR_SAVE = 0x3,
}}
ENUM!{enum ATTACHMENT_ACTION {
    ATTACHMENT_ACTION_CANCEL = 0,
    ATTACHMENT_ACTION_SAVE = 0x1,
    ATTACHMENT_ACTION_EXEC = 0x2,
}}
DEFINE_GUID!{IID_IAttachmentExecute,
    0x73db1241, 0x1e85, 0x4581, 0x8e, 0x4f, 0xa8, 0x1e, 0x1d, 0x0f, 0x8c, 0x57}
RIDL!{#[uuid(0x73db1241, 0x1e85, 0x4581, 0x8e, 0x4f, 0xa8, 0x1e, 0x1d, 0x0f, 0x8c, 0x57)]
interface IAttachmentExecute(IAttachmentExecuteVtbl): IUnknown(IUnknownVtbl) {
    fn SetClientTitle(
        pszTitle: LPCWSTR,
    ) -> HRESULT,
    fn SetClientGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn SetLocalPath(
        pszLocalPath: LPCWSTR,
    ) -> HRESULT,
    fn SetFileName(
        pszFileName: LPCWSTR,
    ) -> HRESULT,
    fn SetSource(
        pszSource: LPCWSTR,
    ) -> HRESULT,
    fn SetReferrer(
        pszReferrer: LPCWSTR,
    ) -> HRESULT,
    fn CheckPolicy() -> HRESULT,
    fn Prompt(
        hwnd: HWND,
        prompt: ATTACHMENT_PROMPT,
        paction: *mut ATTACHMENT_ACTION,
    ) -> HRESULT,
    fn Save() -> HRESULT,
    fn Execute(
        hwnd: HWND,
        pszVerb: LPCWSTR,
        phProcess: *mut HANDLE,
    ) -> HRESULT,
    fn SaveWithUI(
        hwnd: HWND,
    ) -> HRESULT,
    fn ClearClientState() -> HRESULT,
}}
STRUCT!{struct SMDATA {
    dwMask: DWORD,
    dwFlags: DWORD,
    hmenu: HMENU,
    hwnd: HWND,
    uId: UINT,
    uIdParent: UINT,
    uIdAncestor: UINT,
    punk: *mut IUnknown,
    pidlFolder: PIDLIST_ABSOLUTE,
    pidlItem: PUITEMID_CHILD,
    psf: *mut IShellFolder,
    pvUserData: *mut c_void,
}}
pub type LPSMDATA = *mut SMDATA;
pub const SMDM_SHELLFOLDER: DWORD = 0x00000001;
pub const SMDM_HMENU: DWORD = 0x00000002;
pub const SMDM_TOOLBAR: DWORD = 0x00000004;
STRUCT!{struct SMINFO {
    dwMask: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
    iIcon: c_int,
}}
pub type PSMINFO = *mut SMINFO;
STRUCT!{struct SMCSHCHANGENOTIFYSTRUCT {
    lEvent: c_long,
    pidl1: PCIDLIST_ABSOLUTE,
    pidl2: PCIDLIST_ABSOLUTE,
}}
pub type PSMCSHCHANGENOTIFYSTRUCT = *mut SMCSHCHANGENOTIFYSTRUCT;
ENUM!{enum tagSMINFOMASK {
    SMIM_TYPE = 0x1,
    SMIM_FLAGS = 0x2,
    SMIM_ICON = 0x4,
}}
ENUM!{enum tagSMINFOTYPE {
    SMIT_SEPARATOR = 0x1,
    SMIT_STRING = 0x2,
}}
ENUM!{enum tagSMINFOFLAGS {
    SMIF_ICON = 0x1,
    SMIF_ACCELERATOR = 0x2,
    SMIF_DROPTARGET = 0x4,
    SMIF_SUBMENU = 0x8,
    SMIF_CHECKED = 0x20,
    SMIF_DROPCASCADE = 0x40,
    SMIF_HIDDEN = 0x80,
    SMIF_DISABLED = 0x100,
    SMIF_TRACKPOPUP = 0x200,
    SMIF_DEMOTED = 0x400,
    SMIF_ALTSTATE = 0x800,
    SMIF_DRAGNDROP = 0x1000,
    SMIF_NEW = 0x2000,
}}
pub const SMC_INITMENU: UINT = 0x00000001;
pub const SMC_CREATE: UINT = 0x00000002;
pub const SMC_EXITMENU: UINT = 0x00000003;
pub const SMC_GETINFO: UINT = 0x00000005;
pub const SMC_GETSFINFO: UINT = 0x00000006;
pub const SMC_GETOBJECT: UINT = 0x00000007;
pub const SMC_GETSFOBJECT: UINT = 0x00000008;
pub const SMC_SFEXEC: UINT = 0x00000009;
pub const SMC_SFSELECTITEM: UINT = 0x0000000A;
pub const SMC_REFRESH: UINT = 0x00000010;
pub const SMC_DEMOTE: UINT = 0x00000011;
pub const SMC_PROMOTE: UINT = 0x00000012;
pub const SMC_DEFAULTICON: UINT = 0x00000016;
pub const SMC_NEWITEM: UINT = 0x00000017;
pub const SMC_CHEVRONEXPAND: UINT = 0x00000019;
pub const SMC_DISPLAYCHEVRONTIP: UINT = 0x0000002A;
pub const SMC_SETSFOBJECT: UINT = 0x0000002D;
pub const SMC_SHCHANGENOTIFY: UINT = 0x0000002E;
pub const SMC_CHEVRONGETTIP: UINT = 0x0000002F;
pub const SMC_SFDDRESTRICTED: UINT = 0x00000030;
pub const SMC_SFEXEC_MIDDLE: UINT = 0x00000031;
pub const SMC_GETAUTOEXPANDSTATE: UINT = 0x00000041;
pub const SMC_AUTOEXPANDCHANGE: UINT = 0x00000042;
pub const SMC_GETCONTEXTMENUMODIFIER: UINT = 0x00000043;
pub const SMC_GETBKCONTEXTMENU: UINT = 0x00000044;
pub const SMC_OPEN: UINT = 0x00000045;
pub const SMAE_EXPANDED: DWORD = 0x00000001;
pub const SMAE_CONTRACTED: DWORD = 0x00000002;
pub const SMAE_USER: DWORD = 0x00000004;
pub const SMAE_VALID: DWORD = 0x00000007;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0107_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0107_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellMenuCallback,
    0x4ca300a1, 0x9b8d, 0x11d1, 0x8b, 0x22, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0}
RIDL!{#[uuid(0x4ca300a1, 0x9b8d, 0x11d1, 0x8b, 0x22, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0)]
interface IShellMenuCallback(IShellMenuCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn CallbackSM(
        psmd: LPSMDATA,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
}}
pub const SMINIT_DEFAULT: DWORD = 0x00000000;
pub const SMINIT_RESTRICT_DRAGDROP: DWORD = 0x00000002;
pub const SMINIT_TOPLEVEL: DWORD = 0x00000004;
pub const SMINIT_CACHED: DWORD = 0x00000010;
pub const SMINIT_AUTOEXPAND: DWORD = 0x00000100;
pub const SMINIT_AUTOTOOLTIP: DWORD = 0x00000200;
pub const SMINIT_DROPONCONTAINER: DWORD = 0x00000400;
pub const SMINIT_VERTICAL: DWORD = 0x10000000;
pub const SMINIT_HORIZONTAL: DWORD = 0x20000000;
pub const ANCESTORDEFAULT: UINT = -1i32 as u32;
pub const SMSET_TOP: DWORD = 0x10000000;
pub const SMSET_BOTTOM: DWORD = 0x20000000;
pub const SMSET_DONTOWN: DWORD = 0x00000001;
pub const SMINV_REFRESH: DWORD = 0x00000001;
pub const SMINV_ID: DWORD = 0x00000008;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0108_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0108_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellMenu,
    0xee1f7637, 0xe138, 0x11d1, 0x83, 0x79, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0}
RIDL!{#[uuid(0xee1f7637, 0xe138, 0x11d1, 0x83, 0x79, 0x00, 0xc0, 0x4f, 0xd9, 0x18, 0xd0)]
interface IShellMenu(IShellMenuVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        psmc: *mut IShellMenuCallback,
        uId: UINT,
        uIdAncestor: UINT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetMenuInfo(
        ppsmc: *mut *mut IShellMenuCallback,
        puId: *mut UINT,
        puIdAncestor: *mut UINT,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetShellFolder(
        psf: *mut IShellFolder,
        pidlFolder: PCIDLIST_ABSOLUTE,
        hKey: HKEY,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetShellFolder(
        pdwFlags: *mut DWORD,
        ppidl: *mut PIDLIST_ABSOLUTE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn SetMenu(
        hmenu: HMENU,
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetMenu(
        phmenu: *mut HMENU,
        phwnd: *mut HWND,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn InvalidateItem(
        psmd: LPSMDATA,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetState(
        psmd: LPSMDATA,
    ) -> HRESULT,
    fn SetMenuToolbar(
        punk: *mut IUnknown,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum KF_CATEGORY {
    KF_CATEGORY_VIRTUAL = 1,
    KF_CATEGORY_FIXED = 2,
    KF_CATEGORY_COMMON = 3,
    KF_CATEGORY_PERUSER = 4,
}}
ENUM!{enum _KF_DEFINITION_FLAGS {
    KFDF_LOCAL_REDIRECT_ONLY = 0x2,
    KFDF_ROAMABLE = 0x4,
    KFDF_PRECREATE = 0x8,
    KFDF_STREAM = 0x10,
    KFDF_PUBLISHEXPANDEDPATH = 0x20,
    KFDF_NO_REDIRECT_UI = 0x40,
}}
pub type KF_DEFINITION_FLAGS = DWORD;
ENUM!{enum _KF_REDIRECT_FLAGS {
    KF_REDIRECT_USER_EXCLUSIVE = 0x1,
    KF_REDIRECT_COPY_SOURCE_DACL = 0x2,
    KF_REDIRECT_OWNER_USER = 0x4,
    KF_REDIRECT_SET_OWNER_EXPLICIT = 0x8,
    KF_REDIRECT_CHECK_ONLY = 0x10,
    KF_REDIRECT_WITH_UI = 0x20,
    KF_REDIRECT_UNPIN = 0x40,
    KF_REDIRECT_PIN = 0x80,
    KF_REDIRECT_COPY_CONTENTS = 0x200,
    KF_REDIRECT_DEL_SOURCE_CONTENTS = 0x400,
    KF_REDIRECT_EXCLUDE_ALL_KNOWN_SUBFOLDERS = 0x800,
}}
pub type KF_REDIRECT_FLAGS = DWORD;
ENUM!{enum _KF_REDIRECTION_CAPABILITIES {
    KF_REDIRECTION_CAPABILITIES_ALLOW_ALL = 0xff,
    KF_REDIRECTION_CAPABILITIES_REDIRECTABLE = 0x1,
    KF_REDIRECTION_CAPABILITIES_DENY_ALL = 0xfff00,
    KF_REDIRECTION_CAPABILITIES_DENY_POLICY_REDIRECTED = 0x100,
    KF_REDIRECTION_CAPABILITIES_DENY_POLICY = 0x200,
    KF_REDIRECTION_CAPABILITIES_DENY_PERMISSIONS = 0x400,
}}
pub type KF_REDIRECTION_CAPABILITIES = DWORD;
STRUCT!{struct KNOWNFOLDER_DEFINITION {
    category: KF_CATEGORY,
    pszName: LPWSTR,
    pszDescription: LPWSTR,
    fidParent: KNOWNFOLDERID,
    pszRelativePath: LPWSTR,
    pszParsingName: LPWSTR,
    pszTooltip: LPWSTR,
    pszLocalizedName: LPWSTR,
    pszIcon: LPWSTR,
    pszSecurity: LPWSTR,
    dwAttributes: DWORD,
    kfdFlags: KF_DEFINITION_FLAGS,
    ftidType: FOLDERTYPEID,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0109_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0109_v0_0_s_ifspec;
DEFINE_GUID!{IID_IKnownFolder,
    0x3aa7af7e, 0x9b36, 0x420c, 0xa8, 0xe3, 0xf7, 0x7d, 0x46, 0x74, 0xa4, 0x88}
RIDL!{#[uuid(0x3aa7af7e, 0x9b36, 0x420c, 0xa8, 0xe3, 0xf7, 0x7d, 0x46, 0x74, 0xa4, 0x88)]
interface IKnownFolder(IKnownFolderVtbl): IUnknown(IUnknownVtbl) {
    fn GetId(
        pkfid: *mut KNOWNFOLDERID,
    ) -> HRESULT,
    fn GetCategory(
        pCategory: *mut KF_CATEGORY,
    ) -> HRESULT,
    fn GetShellItem(
        dwFlags: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPath(
        dwFlags: DWORD,
        ppszPath: *mut LPWSTR,
    ) -> HRESULT,
    fn SetPath(
        dwFlags: DWORD,
        pszPath: LPCWSTR,
    ) -> HRESULT,
    fn GetIDList(
        dwFlags: DWORD,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetFolderType(
        pftid: *mut FOLDERTYPEID,
    ) -> HRESULT,
    fn GetRedirectionCapabilities(
        pCapabilities: *mut KF_REDIRECTION_CAPABILITIES,
    ) -> HRESULT,
    fn GetFolderDefinition(
        pKFD: *mut KNOWNFOLDER_DEFINITION,
    ) -> HRESULT,
}}
ENUM!{enum FFFP_MODE {
    FFFP_EXACTMATCH = 0,
    FFFP_NEARESTPARENTMATCH = FFFP_EXACTMATCH + 1,
}}
DEFINE_GUID!{IID_IKnownFolderManager,
    0x8be2d872, 0x86aa, 0x4d47, 0xb7, 0x76, 0x32, 0xcc, 0xa4, 0x0c, 0x70, 0x18}
RIDL!{#[uuid(0x8be2d872, 0x86aa, 0x4d47, 0xb7, 0x76, 0x32, 0xcc, 0xa4, 0x0c, 0x70, 0x18)]
interface IKnownFolderManager(IKnownFolderManagerVtbl): IUnknown(IUnknownVtbl) {
    fn FolderIdFromCsidl(
        nCsidl: c_int,
        pfid: *mut KNOWNFOLDERID,
    ) -> HRESULT,
    fn FolderIdToCsidl(
        rfid: REFKNOWNFOLDERID,
        pnCsidl: *mut c_int,
    ) -> HRESULT,
    fn GetFolderIds(
        ppKFId: *mut *mut KNOWNFOLDERID,
        pCount: *mut UINT,
    ) -> HRESULT,
    fn GetFolder(
        rfid: REFKNOWNFOLDERID,
        ppkf: *mut *mut IKnownFolder,
    ) -> HRESULT,
    fn GetFolderByName(
        pszCanonicalName: LPCWSTR,
        ppkf: *mut *mut IKnownFolder,
    ) -> HRESULT,
    fn RegisterFolder(
        rfid: REFKNOWNFOLDERID,
        pKFD: *const KNOWNFOLDER_DEFINITION,
    ) -> HRESULT,
    fn UnregisterFolder(
        rfid: REFKNOWNFOLDERID,
    ) -> HRESULT,
    fn FindFolderFromPath(
        pszPath: LPCWSTR,
        mode: FFFP_MODE,
        ppkf: *mut *mut IKnownFolder,
    ) -> HRESULT,
    fn FindFolderFromIDList(
        pidl: PCIDLIST_ABSOLUTE,
        ppkf: *mut *mut IKnownFolder,
    ) -> HRESULT,
    fn Redirect(
        rfid: REFKNOWNFOLDERID,
        hwnd: HWND,
        flags: KF_REDIRECT_FLAGS,
        pszTargetPath: LPCWSTR,
        cFolders: UINT,
        pExclusion: *const KNOWNFOLDERID,
        ppszError: *mut LPWSTR,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IKnownFolderManager_RemoteRedirect_Proxy(
//     IKnownFolderManager * This,
//     REFKNOWNFOLDERID rfid,
//     HWND hwnd,
//     KF_REDIRECT_FLAGS flags,
//     LPCWSTR pszTargetPath,
//     UINT cFolders,
//     GUID const *pExclusion,
//     LPWSTR *ppszError);
// c_void __RPC_STUB IKnownFolderManager_RemoteRedirect_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
#[inline]
pub unsafe fn FreeKnownFolderDefinitionFields(pKFD: *mut KNOWNFOLDER_DEFINITION) {
    CoTaskMemFree((*pKFD).pszName as *mut c_void);
    CoTaskMemFree((*pKFD).pszDescription as *mut c_void);
    CoTaskMemFree((*pKFD).pszRelativePath as *mut c_void);
    CoTaskMemFree((*pKFD).pszParsingName as *mut c_void);
    CoTaskMemFree((*pKFD).pszTooltip as *mut c_void);
    CoTaskMemFree((*pKFD).pszLocalizedName as *mut c_void);
    CoTaskMemFree((*pKFD).pszIcon as *mut c_void);
    CoTaskMemFree((*pKFD).pszSecurity as *mut c_void);
}
ENUM!{enum SHARE_ROLE {
    SHARE_ROLE_INVALID = -1i32 as u32,
    SHARE_ROLE_READER = 0,
    SHARE_ROLE_CONTRIBUTOR = 1,
    SHARE_ROLE_CO_OWNER = 2,
    SHARE_ROLE_OWNER = 3,
    SHARE_ROLE_CUSTOM = 4,
    SHARE_ROLE_MIXED = 5,
}}
ENUM!{enum DEF_SHARE_ID {
    DEFSHAREID_USERS = 1,
    DEFSHAREID_PUBLIC = 2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0111_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0111_v0_0_s_ifspec;
DEFINE_GUID!{IID_ISharingConfigurationManager,
    0xb4cd448a, 0x9c86, 0x4466, 0x92, 0x01, 0x2e, 0x62, 0x10, 0x5b, 0x87, 0xae}
RIDL!{#[uuid(0xb4cd448a, 0x9c86, 0x4466, 0x92, 0x01, 0x2e, 0x62, 0x10, 0x5b, 0x87, 0xae)]
interface ISharingConfigurationManager(ISharingConfigurationManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CreateShare(
        dsid: DEF_SHARE_ID,
        role: SHARE_ROLE,
    ) -> HRESULT,
    fn DeleteShare(
        dsid: DEF_SHARE_ID,
    ) -> HRESULT,
    fn ShareExists(
        dsid: DEF_SHARE_ID,
    ) -> HRESULT,
    fn GetSharePermissions(
        dsid: DEF_SHARE_ID,
        pRole: *mut SHARE_ROLE,
    ) -> HRESULT,
    fn SharePrinters() -> HRESULT,
    fn StopSharingPrinters() -> HRESULT,
    fn ArePrintersShared() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0112_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0112_v0_0_s_ifspec;
DEFINE_GUID!{IID_IRelatedItem,
    0xa73ce67a, 0x8ab1, 0x44f1, 0x8d, 0x43, 0xd2, 0xfc, 0xbf, 0x6b, 0x1c, 0xd0}
RIDL!{#[uuid(0xa73ce67a, 0x8ab1, 0x44f1, 0x8d, 0x43, 0xd2, 0xfc, 0xbf, 0x6b, 0x1c, 0xd0)]
interface IRelatedItem(IRelatedItemVtbl): IUnknown(IUnknownVtbl) {
    fn GetItemIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetItem(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IIdentityName,
    0x7d903fca, 0xd6f9, 0x4810, 0x83, 0x32, 0x94, 0x6c, 0x01, 0x77, 0xe2, 0x47}
RIDL!{#[uuid(0x7d903fca, 0xd6f9, 0x4810, 0x83, 0x32, 0x94, 0x6c, 0x01, 0x77, 0xe2, 0x47)]
interface IIdentityName(IIdentityNameVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_IDelegateItem,
    0x3c5a1c94, 0xc951, 0x4cb7, 0xbb, 0x6d, 0x3b, 0x93, 0xf3, 0x0c, 0xce, 0x93}
RIDL!{#[uuid(0x3c5a1c94, 0xc951, 0x4cb7, 0xbb, 0x6d, 0x3b, 0x93, 0xf3, 0x0c, 0xce, 0x93)]
interface IDelegateItem(IDelegateItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_ICurrentItem,
    0x240a7174, 0xd653, 0x4a1d, 0xa6, 0xd3, 0xd4, 0x94, 0x3c, 0xfb, 0xfe, 0x3d}
RIDL!{#[uuid(0x240a7174, 0xd653, 0x4a1d, 0xa6, 0xd3, 0xd4, 0x94, 0x3c, 0xfb, 0xfe, 0x3d)]
interface ICurrentItem(ICurrentItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_ITransferMediumItem,
    0x77f295d5, 0x2d6f, 0x4e19, 0xb8, 0xae, 0x32, 0x2f, 0x3e, 0x72, 0x1a, 0xb5}
RIDL!{#[uuid(0x77f295d5, 0x2d6f, 0x4e19, 0xb8, 0xae, 0x32, 0x2f, 0x3e, 0x72, 0x1a, 0xb5)]
interface ITransferMediumItem(ITransferMediumItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_IDisplayItem,
    0xc6fd5997, 0x9f6b, 0x4888, 0x87, 0x03, 0x94, 0xe8, 0x0e, 0x8c, 0xde, 0x3f}
RIDL!{#[uuid(0xc6fd5997, 0x9f6b, 0x4888, 0x87, 0x03, 0x94, 0xe8, 0x0e, 0x8c, 0xde, 0x3f)]
interface IDisplayItem(IDisplayItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_IViewStateIdentityItem,
    0x9d264146, 0xa94f, 0x4195, 0x9f, 0x9f, 0x3b, 0xb1, 0x2c, 0xe0, 0xc9, 0x55}
RIDL!{#[uuid(0x9d264146, 0xa94f, 0x4195, 0x9f, 0x9f, 0x3b, 0xb1, 0x2c, 0xe0, 0xc9, 0x55)]
interface IViewStateIdentityItem(IViewStateIdentityItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{IID_IPreviewItem,
    0x36149969, 0x0a8f, 0x49c8, 0x8b, 0x00, 0x4a, 0xec, 0xb2, 0x02, 0x22, 0xfb}
RIDL!{#[uuid(0x36149969, 0x0a8f, 0x49c8, 0x8b, 0x00, 0x4a, 0xec, 0xb2, 0x02, 0x22, 0xfb)]
interface IPreviewItem(IPreviewItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0120_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0120_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDestinationStreamFactory,
    0x8a87781b, 0x39a7, 0x4a1f, 0xaa, 0xb3, 0xa3, 0x9b, 0x9c, 0x34, 0xa7, 0xd9}
RIDL!{#[uuid(0x8a87781b, 0x39a7, 0x4a1f, 0xaa, 0xb3, 0xa3, 0x9b, 0x9c, 0x34, 0xa7, 0xd9)]
interface IDestinationStreamFactory(IDestinationStreamFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetDestinationStream(
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0121_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0121_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICreateProcessInputs,
    0xf6ef6140, 0xe26f, 0x4d82, 0xba, 0xc4, 0xe9, 0xba, 0x5f, 0xd2, 0x39, 0xa8}
RIDL!{#[uuid(0xf6ef6140, 0xe26f, 0x4d82, 0xba, 0xc4, 0xe9, 0xba, 0x5f, 0xd2, 0x39, 0xa8)]
interface ICreateProcessInputs(ICreateProcessInputsVtbl): IUnknown(IUnknownVtbl) {
    fn GetCreateFlags(
        pdwCreationFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetCreateFlags(
        dwCreationFlags: DWORD,
    ) -> HRESULT,
    fn AddCreateFlags(
        dwCreationFlags: DWORD,
    ) -> HRESULT,
    fn SetHotKey(
        wHotKey: WORD,
    ) -> HRESULT,
    fn AddStartupFlags(
        dwStartupInfoFlags: DWORD,
    ) -> HRESULT,
    fn SetTitle(
        pszTitle: LPCWSTR,
    ) -> HRESULT,
    fn SetEnvironmentVariable(
        pszName: LPCWSTR,
        pszValue: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ICreatingProcess,
    0xc2b937a9, 0x3110, 0x4398, 0x8a, 0x56, 0xf3, 0x4c, 0x63, 0x42, 0xd2, 0x44}
RIDL!{#[uuid(0xc2b937a9, 0x3110, 0x4398, 0x8a, 0x56, 0xf3, 0x4c, 0x63, 0x42, 0xd2, 0x44)]
interface ICreatingProcess(ICreatingProcessVtbl): IUnknown(IUnknownVtbl) {
    fn OnCreating(
        pcpi: *mut ICreateProcessInputs,
    ) -> HRESULT,
}}
// #define SID_ExecuteCreatingProcess __uuidof(ICreatingProcess)
ENUM!{enum _NMCII_FLAGS {
    NMCII_NONE = 0,
    NMCII_ITEMS = 0x1,
    NMCII_FOLDERS = 0x2,
}}
pub type NMCII_FLAGS = c_int;
ENUM!{enum _NMCSAEI_FLAGS {
    NMCSAEI_SELECT = 0,
    NMCSAEI_EDIT = 0x1,
}}
pub type NMCSAEI_FLAGS = c_int;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0123_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0123_v0_0_s_ifspec;
DEFINE_GUID!{IID_INewMenuClient,
    0xdcb07fdc, 0x3bb5, 0x451c, 0x90, 0xbe, 0x96, 0x66, 0x44, 0xfe, 0xd7, 0xb0}
RIDL!{#[uuid(0xdcb07fdc, 0x3bb5, 0x451c, 0x90, 0xbe, 0x96, 0x66, 0x44, 0xfe, 0xd7, 0xb0)]
interface INewMenuClient(INewMenuClientVtbl): IUnknown(IUnknownVtbl) {
    fn IncludeItems(
        pflags: *mut NMCII_FLAGS,
    ) -> HRESULT,
    fn SelectAndEditItem(
        pidlItem: PCIDLIST_ABSOLUTE,
        flags: NMCSAEI_FLAGS,
    ) -> HRESULT,
}}
// #define SID_SNewMenuClient IID_INewMenuClient
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0124_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0124_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInitializeWithBindCtx,
    0x71c0d2bc, 0x726d, 0x45cc, 0xa6, 0xc0, 0x2e, 0x31, 0xc1, 0xdb, 0x21, 0x59}
RIDL!{#[uuid(0x71c0d2bc, 0x726d, 0x45cc, 0xa6, 0xc0, 0x2e, 0x31, 0xc1, 0xdb, 0x21, 0x59)]
interface IInitializeWithBindCtx(IInitializeWithBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pbc: *mut IBindCtx,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IShellItemFilter,
    0x2659b475, 0xeeb8, 0x48b7, 0x8f, 0x07, 0xb3, 0x78, 0x81, 0x0f, 0x48, 0xcf}
RIDL!{#[uuid(0x2659b475, 0xeeb8, 0x48b7, 0x8f, 0x07, 0xb3, 0x78, 0x81, 0x0f, 0x48, 0xcf)]
interface IShellItemFilter(IShellItemFilterVtbl): IUnknown(IUnknownVtbl) {
    fn IncludeItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetEnumFlagsForItem(
        psi: *mut IShellItem,
        pgrfFlags: *mut SHCONTF,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0126_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0126_v0_0_s_ifspec;
ENUM!{enum _NSTCSTYLE {
    NSTCS_HASEXPANDOS = 0x1,
    NSTCS_HASLINES = 0x2,
    NSTCS_SINGLECLICKEXPAND = 0x4,
    NSTCS_FULLROWSELECT = 0x8,
    NSTCS_SPRINGEXPAND = 0x10,
    NSTCS_HORIZONTALSCROLL = 0x20,
    NSTCS_ROOTHASEXPANDO = 0x40,
    NSTCS_SHOWSELECTIONALWAYS = 0x80,
    NSTCS_NOINFOTIP = 0x200,
    NSTCS_EVENHEIGHT = 0x400,
    NSTCS_NOREPLACEOPEN = 0x800,
    NSTCS_DISABLEDRAGDROP = 0x1000,
    NSTCS_NOORDERSTREAM = 0x2000,
    NSTCS_RICHTOOLTIP = 0x4000,
    NSTCS_BORDER = 0x8000,
    NSTCS_NOEDITLABELS = 0x10000,
    NSTCS_TABSTOP = 0x20000,
    NSTCS_FAVORITESMODE = 0x80000,
    NSTCS_AUTOHSCROLL = 0x100000,
    NSTCS_FADEINOUTEXPANDOS = 0x200000,
    NSTCS_EMPTYTEXT = 0x400000,
    NSTCS_CHECKBOXES = 0x800000,
    NSTCS_PARTIALCHECKBOXES = 0x1000000,
    NSTCS_EXCLUSIONCHECKBOXES = 0x2000000,
    NSTCS_DIMMEDCHECKBOXES = 0x4000000,
    NSTCS_NOINDENTCHECKS = 0x8000000,
    NSTCS_ALLOWJUNCTIONS = 0x10000000,
    NSTCS_SHOWTABSBUTTON = 0x20000000,
    NSTCS_SHOWDELETEBUTTON = 0x40000000,
    NSTCS_SHOWREFRESHBUTTON = 0x80000000,
}}
pub type NSTCSTYLE = DWORD;
ENUM!{enum _NSTCROOTSTYLE {
    NSTCRS_VISIBLE = 0,
    NSTCRS_HIDDEN = 0x1,
    NSTCRS_EXPANDED = 0x2,
}}
pub type NSTCROOTSTYLE = DWORD;
ENUM!{enum _NSTCITEMSTATE {
    NSTCIS_NONE = 0,
    NSTCIS_SELECTED = 0x1,
    NSTCIS_EXPANDED = 0x2,
    NSTCIS_BOLD = 0x4,
    NSTCIS_DISABLED = 0x8,
    NSTCIS_SELECTEDNOEXPAND = 0x10,
}}
pub type NSTCITEMSTATE = DWORD;
ENUM!{enum NSTCGNI {
    NSTCGNI_NEXT = 0,
    NSTCGNI_NEXTVISIBLE = 1,
    NSTCGNI_PREV = 2,
    NSTCGNI_PREVVISIBLE = 3,
    NSTCGNI_PARENT = 4,
    NSTCGNI_CHILD = 5,
    NSTCGNI_FIRSTVISIBLE = 6,
    NSTCGNI_LASTVISIBLE = 7,
}}
DEFINE_GUID!{IID_INameSpaceTreeControl,
    0x028212a3, 0xb627, 0x47e9, 0x88, 0x56, 0xc1, 0x42, 0x65, 0x55, 0x4e, 0x4f}
RIDL!{#[uuid(0x028212a3, 0xb627, 0x47e9, 0x88, 0x56, 0xc1, 0x42, 0x65, 0x55, 0x4e, 0x4f)]
interface INameSpaceTreeControl(INameSpaceTreeControlVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hwndParent: HWND,
        prc: *mut RECT,
        nsctsFlags: NSTCSTYLE,
    ) -> HRESULT,
    fn TreeAdvise(
        punk: *mut IUnknown,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn TreeUnadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn AppendRoot(
        psiRoot: *mut IShellItem,
        grfEnumFlags: SHCONTF,
        grfRootStyle: NSTCROOTSTYLE,
        pif: *mut IShellItemFilter,
    ) -> HRESULT,
    fn InsertRoot(
        iIndex: c_int,
        psiRoot: *mut IShellItem,
        grfEnumFlags: SHCONTF,
        grfRootStyle: NSTCROOTSTYLE,
        pif: *mut IShellItemFilter,
    ) -> HRESULT,
    fn RemoveRoot(
        psiRoot: *mut IShellItem,
    ) -> HRESULT,
    fn RemoveAllRoots() -> HRESULT,
    fn GetRootItems(
        ppsiaRootItems: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn SetItemState(
        psi: *mut IShellItem,
        nstcisMask: NSTCITEMSTATE,
        nstcisFlags: NSTCITEMSTATE,
    ) -> HRESULT,
    fn GetItemState(
        psi: *mut IShellItem,
        nstcisMask: NSTCITEMSTATE,
        pnstcisFlags: *mut NSTCITEMSTATE,
    ) -> HRESULT,
    fn GetSelectedItems(
        psiaItems: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn GetItemCustomState(
        psi: *mut IShellItem,
        piStateNumber: *mut c_int,
    ) -> HRESULT,
    fn SetItemCustomState(
        psi: *mut IShellItem,
        iStateNumber: c_int,
    ) -> HRESULT,
    fn EnsureItemVisible(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn SetTheme(
        pszTheme: LPCWSTR,
    ) -> HRESULT,
    fn GetNextItem(
        psi: *mut IShellItem,
        nstcgi: NSTCGNI,
        ppsiNext: *mut *mut IShellItem,
    ) -> HRESULT,
    fn HitTest(
        ppt: *mut POINT,
        ppsiOut: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetItemRect(
        psi: *mut IShellItem,
        prect: *mut RECT,
    ) -> HRESULT,
    fn CollapseAll() -> HRESULT,
}}
// #define SID_SNavigationPane IID_INameSpaceTreeControl
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0127_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0127_v0_0_s_ifspec;
ENUM!{enum NSTCFOLDERCAPABILITIES {
    NSTCFC_NONE = 0,
    NSTCFC_PINNEDITEMFILTERING = 0x1,
    NSTCFC_DELAY_REGISTER_NOTIFY = 0x2,
}}
DEFINE_GUID!{IID_INameSpaceTreeControlFolderCapabilities,
    0xe9701183, 0xe6b3, 0x4ff2, 0x85, 0x68, 0x81, 0x36, 0x15, 0xfe, 0xc7, 0xbe}
RIDL!{#[uuid(0xe9701183, 0xe6b3, 0x4ff2, 0x85, 0x68, 0x81, 0x36, 0x15, 0xfe, 0xc7, 0xbe)]
interface INameSpaceTreeControlFolderCapabilities(INameSpaceTreeControlFolderCapabilitiesVtbl):
    IUnknown(IUnknownVtbl)
{
    fn GetFolderCapabilities(
        nfcMask: NSTCFOLDERCAPABILITIES,
        pnfcValue: *mut NSTCFOLDERCAPABILITIES,
    ) -> HRESULT,
}}
pub const E_PREVIEWHANDLER_DRM_FAIL: HRESULT = 0x86420001;
pub const E_PREVIEWHANDLER_NOAUTH: HRESULT = 0x86420002;
pub const E_PREVIEWHANDLER_NOTFOUND: HRESULT = 0x86420003;
pub const E_PREVIEWHANDLER_CORRUPT: HRESULT = 0x86420004;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0128_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0128_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPreviewHandler,
    0x8895b1c6, 0xb41f, 0x4c1c, 0xa5, 0x62, 0x0d, 0x56, 0x42, 0x50, 0x83, 0x6f}
RIDL!{#[uuid(0x8895b1c6, 0xb41f, 0x4c1c, 0xa5, 0x62, 0x0d, 0x56, 0x42, 0x50, 0x83, 0x6f)]
interface IPreviewHandler(IPreviewHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn SetWindow(
        hwnd: HWND,
        prc: *const RECT,
    ) -> HRESULT,
    fn SetRect(
        prc: *const RECT,
    ) -> HRESULT,
    fn DoPreview() -> HRESULT,
    fn Unload() -> HRESULT,
    fn SetFocus() -> HRESULT,
    fn QueryFocus(
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn TranslateAccelerator(
        pmsg: *mut MSG,
    ) -> HRESULT,
}}
STRUCT!{struct PREVIEWHANDLERFRAMEINFO {
    haccel: HACCEL,
    cAccelEntries: UINT,
}}
DEFINE_GUID!{IID_IPreviewHandlerFrame,
    0xfec87aaf, 0x35f9, 0x447a, 0xad, 0xb7, 0x20, 0x23, 0x44, 0x91, 0x40, 0x1a}
RIDL!{#[uuid(0xfec87aaf, 0x35f9, 0x447a, 0xad, 0xb7, 0x20, 0x23, 0x44, 0x91, 0x40, 0x1a)]
interface IPreviewHandlerFrame(IPreviewHandlerFrameVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindowContext(
        pinfo: *mut PREVIEWHANDLERFRAMEINFO,
    ) -> HRESULT,
    fn TranslateAccelerator(
        pmsg: *mut MSG,
    ) -> HRESULT,
}}
pub type EXPLORERPANE = GUID;
pub type REFEXPLORERPANE = *const EXPLORERPANE;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0130_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0130_v0_0_s_ifspec;
ENUM!{enum _EXPLORERPANESTATE {
    EPS_DONTCARE = 0,
    EPS_DEFAULT_ON = 0x1,
    EPS_DEFAULT_OFF = 0x2,
    EPS_STATEMASK = 0xffff,
    EPS_INITIALSTATE = 0x10000,
    EPS_FORCE = 0x20000,
}}
pub type EXPLORERPANESTATE = DWORD;
DEFINE_GUID!{IID_IExplorerPaneVisibility,
    0xe07010ec, 0xbc17, 0x44c0, 0x97, 0xb0, 0x46, 0xc7, 0xc9, 0x5b, 0x9e, 0xdc}
RIDL!{#[uuid(0xe07010ec, 0xbc17, 0x44c0, 0x97, 0xb0, 0x46, 0xc7, 0xc9, 0x5b, 0x9e, 0xdc)]
interface IExplorerPaneVisibility(IExplorerPaneVisibilityVtbl): IUnknown(IUnknownVtbl) {
    fn GetPaneState(
        ep: REFEXPLORERPANE,
        peps: *mut EXPLORERPANESTATE,
    ) -> HRESULT,
}}
// #define SID_ExplorerPaneVisibility IID_IExplorerPaneVisibility
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0131_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0131_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContextMenuCB,
    0x3409e930, 0x5a39, 0x11d1, 0x83, 0xfa, 0x00, 0xa0, 0xc9, 0x0d, 0xc8, 0x49}
RIDL!{#[uuid(0x3409e930, 0x5a39, 0x11d1, 0x83, 0xfa, 0x00, 0xa0, 0xc9, 0x0d, 0xc8, 0x49)]
interface IContextMenuCB(IContextMenuCBVtbl): IUnknown(IUnknownVtbl) {
    fn CallBack(
        psf: *mut IShellFolder,
        hwndOwner: HWND,
        pdtobj: *mut IDataObject,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0132_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0132_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDefaultExtractIconInit,
    0x41ded17d, 0xd6b3, 0x4261, 0x99, 0x7d, 0x88, 0xc6, 0x0e, 0x4b, 0x1d, 0x58}
RIDL!{#[uuid(0x41ded17d, 0xd6b3, 0x4261, 0x99, 0x7d, 0x88, 0xc6, 0x0e, 0x4b, 0x1d, 0x58)]
interface IDefaultExtractIconInit(IDefaultExtractIconInitVtbl): IUnknown(IUnknownVtbl) {
    fn SetFlags(
        uFlags: UINT,
    ) -> HRESULT,
    fn SetKey(
        hkey: HKEY,
    ) -> HRESULT,
    fn SetNormalIcon(
        pszFile: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetOpenIcon(
        pszFile: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetShortcutIcon(
        pszFile: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetDefaultIcon(
        pszFile: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
}}
extern "system" {
    pub fn SHCreateDefaultExtractIcon(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0133_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0133_v0_0_s_ifspec;
ENUM!{enum _EXPCMDSTATE {
    ECS_ENABLED = 0,
    ECS_DISABLED = 0x1,
    ECS_HIDDEN = 0x2,
    ECS_CHECKBOX = 0x4,
    ECS_CHECKED = 0x8,
    ECS_RADIOCHECK = 0x10,
}}
pub type EXPCMDSTATE = DWORD;
ENUM!{enum _EXPCMDFLAGS {
    ECF_DEFAULT = 0,
    ECF_HASSUBCOMMANDS = 0x1,
    ECF_HASSPLITBUTTON = 0x2,
    ECF_HIDELABEL = 0x4,
    ECF_ISSEPARATOR = 0x8,
    ECF_HASLUASHIELD = 0x10,
    ECF_SEPARATORBEFORE = 0x20,
    ECF_SEPARATORAFTER = 0x40,
    ECF_ISDROPDOWN = 0x80,
    ECF_TOGGLEABLE = 0x100,
    ECF_AUTOMENUICONS = 0x200,
}}
pub type EXPCMDFLAGS = DWORD;
DEFINE_GUID!{IID_IExplorerCommand,
    0xa08ce4d0, 0xfa25, 0x44ab, 0xb5, 0x7c, 0xc7, 0xb1, 0xc3, 0x23, 0xe0, 0xb9}
RIDL!{#[uuid(0xa08ce4d0, 0xfa25, 0x44ab, 0xb5, 0x7c, 0xc7, 0xb1, 0xc3, 0x23, 0xe0, 0xb9)]
interface IExplorerCommand(IExplorerCommandVtbl): IUnknown(IUnknownVtbl) {
    fn GetTitle(
        psiItemArray: *mut IShellItemArray,
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetIcon(
        psiItemArray: *mut IShellItemArray,
        ppszIcon: *mut LPWSTR,
    ) -> HRESULT,
    fn GetToolTip(
        psiItemArray: *mut IShellItemArray,
        ppszInfotip: *mut LPWSTR,
    ) -> HRESULT,
    fn GetCanonicalName(
        pguidCommandName: *mut GUID,
    ) -> HRESULT,
    fn GetState(
        psiItemArray: *mut IShellItemArray,
        fOkToBeSlow: BOOL,
        pCmdState: *mut EXPCMDSTATE,
    ) -> HRESULT,
    fn Invoke(
        psiItemArray: *mut IShellItemArray,
        pbc: *mut IBindCtx,
    ) -> HRESULT,
    fn GetFlags(
        pFlags: *mut EXPCMDFLAGS,
    ) -> HRESULT,
    fn EnumSubCommands(
        ppEnum: *mut *mut IEnumExplorerCommand,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IExplorerCommandState,
    0xbddacb60, 0x7657, 0x47ae, 0x84, 0x45, 0xd2, 0x3e, 0x1a, 0xcf, 0x82, 0xae}
RIDL!{#[uuid(0xbddacb60, 0x7657, 0x47ae, 0x84, 0x45, 0xd2, 0x3e, 0x1a, 0xcf, 0x82, 0xae)]
interface IExplorerCommandState(IExplorerCommandStateVtbl): IUnknown(IUnknownVtbl) {
    fn GetState(
        psiItemArray: *mut IShellItemArray,
        fOkToBeSlow: BOOL,
        pCmdState: *mut EXPCMDSTATE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInitializeCommand,
    0x85075acf, 0x231f, 0x40ea, 0x96, 0x10, 0xd2, 0x6b, 0x7b, 0x58, 0xf6, 0x38}
RIDL!{#[uuid(0x85075acf, 0x231f, 0x40ea, 0x96, 0x10, 0xd2, 0x6b, 0x7b, 0x58, 0xf6, 0x38)]
interface IInitializeCommand(IInitializeCommandVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pszCommandName: LPCWSTR,
        ppb: *mut IPropertyBag,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumExplorerCommand,
    0xa88826f8, 0x186f, 0x4987, 0xaa, 0xde, 0xea, 0x0c, 0xef, 0x8f, 0xbf, 0xe8}
RIDL!{#[uuid(0xa88826f8, 0x186f, 0x4987, 0xaa, 0xde, 0xea, 0x0c, 0xef, 0x8f, 0xbf, 0xe8)]
interface IEnumExplorerCommand(IEnumExplorerCommandVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        pUICommand: *mut *mut IExplorerCommand,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumExplorerCommand,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumExplorerCommand_RemoteNext_Proxy(
//     IEnumExplorerCommand * This,
//     ULONG celt,
//     IExplorerCommand **pUICommand,
//     ULONG *pceltFetched);
// c_void __RPC_STUB IEnumExplorerCommand_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
DEFINE_GUID!{IID_IExplorerCommandProvider,
    0x64961751, 0x0835, 0x43c0, 0x8f, 0xfe, 0xd5, 0x76, 0x86, 0x53, 0x0e, 0x64}
RIDL!{#[uuid(0x64961751, 0x0835, 0x43c0, 0x8f, 0xfe, 0xd5, 0x76, 0x86, 0x53, 0x0e, 0x64)]
interface IExplorerCommandProvider(IExplorerCommandProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetCommands(
        punkSite: *mut IUnknown,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetCommand(
        rguidCommandId: REFGUID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
pub type HTHEME = HANDLE;
ENUM!{enum CPVIEW {
    CPVIEW_CLASSIC = 0,
    CPVIEW_ALLITEMS = CPVIEW_CLASSIC,
    CPVIEW_CATEGORY = 1,
    CPVIEW_HOME = CPVIEW_CATEGORY,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0138_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0138_v0_0_s_ifspec;
DEFINE_GUID!{IID_IOpenControlPanel,
    0xd11ad862, 0x66de, 0x4df4, 0xbf, 0x6c, 0x1f, 0x56, 0x21, 0x99, 0x6a, 0xf1}
RIDL!{#[uuid(0xd11ad862, 0x66de, 0x4df4, 0xbf, 0x6c, 0x1f, 0x56, 0x21, 0x99, 0x6a, 0xf1)]
interface IOpenControlPanel(IOpenControlPanelVtbl): IUnknown(IUnknownVtbl) {
    fn Open(
        pszName: LPCWSTR,
        pszPage: LPCWSTR,
        punkSite: *mut IUnknown,
    ) -> HRESULT,
    fn GetPath(
        pszName: LPCWSTR,
        pszPath: LPWSTR,
        cchPath: UINT,
    ) -> HRESULT,
    fn GetCurrentView(
        pView: *mut CPVIEW,
    ) -> HRESULT,
}}
// #define STR_FILE_SYS_BIND_DATA L"File System Bind Data"
// #define STR_FILE_SYS_BIND_DATA_WIN7_FORMAT L"Win7FileSystemIdList"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0139_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0139_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileSystemBindData,
    0x01e18d10, 0x4d8b, 0x11d2, 0x85, 0x5d, 0x00, 0x60, 0x08, 0x05, 0x93, 0x67}
RIDL!{#[uuid(0x01e18d10, 0x4d8b, 0x11d2, 0x85, 0x5d, 0x00, 0x60, 0x08, 0x05, 0x93, 0x67)]
interface IFileSystemBindData(IFileSystemBindDataVtbl): IUnknown(IUnknownVtbl) {
    fn SetFindData(
        pfd: *const WIN32_FIND_DATAW,
    ) -> HRESULT,
    fn GetFindData(
        pfd: *mut WIN32_FIND_DATAW,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFileSystemBindData2,
    0x3acf075f, 0x71db, 0x4afa, 0x81, 0xf0, 0x3f, 0xc4, 0xfd, 0xf2, 0xa5, 0xb8}
RIDL!{#[uuid(0x3acf075f, 0x71db, 0x4afa, 0x81, 0xf0, 0x3f, 0xc4, 0xfd, 0xf2, 0xa5, 0xb8)]
interface IFileSystemBindData2(IFileSystemBindData2Vtbl):
    IFileSystemBindData(IFileSystemBindDataVtbl)
{
    fn SetFileID(
        liFileID: LARGE_INTEGER,
    ) -> HRESULT,
    fn GetFileID(
        pliFileID: *mut LARGE_INTEGER,
    ) -> HRESULT,
    fn SetJunctionCLSID(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn GetJunctionCLSID(
        pclsid: *mut CLSID,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0141_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0141_v0_0_s_ifspec;
ENUM!{enum KNOWNDESTCATEGORY {
    KDC_FREQUENT = 1,
    KDC_RECENT = KDC_FREQUENT + 1,
}}
DEFINE_GUID!{IID_ICustomDestinationList,
    0x6332debf, 0x87b5, 0x4670, 0x90, 0xc0, 0x5e, 0x57, 0xb4, 0x08, 0xa4, 0x9e}
RIDL!{#[uuid(0x6332debf, 0x87b5, 0x4670, 0x90, 0xc0, 0x5e, 0x57, 0xb4, 0x08, 0xa4, 0x9e)]
interface ICustomDestinationList(ICustomDestinationListVtbl): IUnknown(IUnknownVtbl) {
    fn SetAppID(
        pszAppID: LPCWSTR,
    ) -> HRESULT,
    fn BeginList(
        pcMinSlots: *mut UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn AppendCategory(
        pszCategory: LPCWSTR,
        poa: *mut IObjectArray,
    ) -> HRESULT,
    fn AppendKnownCategory(
        category: KNOWNDESTCATEGORY,
    ) -> HRESULT,
    fn AddUserTasks(
        poa: *mut IObjectArray,
    ) -> HRESULT,
    fn CommitList() -> HRESULT,
    fn GetRemovedDestinations(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn DeleteList(
        pszAppID: LPCWSTR,
    ) -> HRESULT,
    fn AbortList() -> HRESULT,
}}
DEFINE_GUID!{IID_IApplicationDestinations,
    0x12337d35, 0x94c6, 0x48a0, 0xbc, 0xe7, 0x6a, 0x9c, 0x69, 0xd4, 0xd6, 0x00}
RIDL!{#[uuid(0x12337d35, 0x94c6, 0x48a0, 0xbc, 0xe7, 0x6a, 0x9c, 0x69, 0xd4, 0xd6, 0x00)]
interface IApplicationDestinations(IApplicationDestinationsVtbl): IUnknown(IUnknownVtbl) {
    fn SetAppID(
        pszAppID: LPCWSTR,
    ) -> HRESULT,
    fn RemoveDestination(
        punk: *mut IUnknown,
    ) -> HRESULT,
    fn RemoveAllDestinations() -> HRESULT,
}}
ENUM!{enum APPDOCLISTTYPE {
    ADLT_RECENT = 0,
    ADLT_FREQUENT = ADLT_RECENT + 1,
}}
DEFINE_GUID!{IID_IApplicationDocumentLists,
    0x3c594f9f, 0x9f30, 0x47a1, 0x97, 0x9a, 0xc9, 0xe8, 0x3d, 0x3d, 0x0a, 0x06}
RIDL!{#[uuid(0x3c594f9f, 0x9f30, 0x47a1, 0x97, 0x9a, 0xc9, 0xe8, 0x3d, 0x3d, 0x0a, 0x06)]
interface IApplicationDocumentLists(IApplicationDocumentListsVtbl): IUnknown(IUnknownVtbl) {
    fn SetAppID(
        pszAppID: LPCWSTR,
    ) -> HRESULT,
    fn GetList(
        listtype: APPDOCLISTTYPE,
        cItemsDesired: UINT,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectWithAppUserModelID,
    0x36db0196, 0x9665, 0x46d1, 0x9b, 0xa7, 0xd3, 0x70, 0x9e, 0xec, 0xf9, 0xed}
RIDL!{#[uuid(0x36db0196, 0x9665, 0x46d1, 0x9b, 0xa7, 0xd3, 0x70, 0x9e, 0xec, 0xf9, 0xed)]
interface IObjectWithAppUserModelID(IObjectWithAppUserModelIDVtbl): IUnknown(IUnknownVtbl) {
    fn SetAppID(
        pszAppID: LPCWSTR,
    ) -> HRESULT,
    fn GetAppID(
        ppszAppID: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IObjectWithProgID,
    0x71e806fb, 0x8dee, 0x46fc, 0xbf, 0x8c, 0x77, 0x48, 0xa8, 0xa1, 0xae, 0x13}
RIDL!{#[uuid(0x71e806fb, 0x8dee, 0x46fc, 0xbf, 0x8c, 0x77, 0x48, 0xa8, 0xa1, 0xae, 0x13)]
interface IObjectWithProgID(IObjectWithProgIDVtbl): IUnknown(IUnknownVtbl) {
    fn SetProgID(
        pszProgID: LPCWSTR,
    ) -> HRESULT,
    fn GetProgID(
        ppszProgID: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IUpdateIDList,
    0x6589b6d2, 0x5f8d, 0x4b9e, 0xb7, 0xe0, 0x23, 0xcd, 0xd9, 0x71, 0x7d, 0x8c}
RIDL!{#[uuid(0x6589b6d2, 0x5f8d, 0x4b9e, 0xb7, 0xe0, 0x23, 0xcd, 0xd9, 0x71, 0x7d, 0x8c)]
interface IUpdateIDList(IUpdateIDListVtbl): IUnknown(IUnknownVtbl) {
    fn Update(
        pbc: *mut IBindCtx,
        pidlIn: PCUITEMID_CHILD,
        ppidlOut: *mut PITEMID_CHILD,
    ) -> HRESULT,
}}
extern "system" {
    pub fn SetCurrentProcessExplicitAppUserModelID(
        AppID: PCWSTR,
    ) -> HRESULT;
    pub fn GetCurrentProcessExplicitAppUserModelID(
        AppID: *mut PWSTR,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0147_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0147_v0_0_s_ifspec;
ENUM!{enum DESKTOP_SLIDESHOW_OPTIONS {
    DSO_SHUFFLEIMAGES = 0x1,
}}
ENUM!{enum DESKTOP_SLIDESHOW_STATE {
    DSS_ENABLED = 0x1,
    DSS_SLIDESHOW = 0x2,
    DSS_DISABLED_BY_REMOTE_SESSION = 0x4,
}}
ENUM!{enum DESKTOP_SLIDESHOW_DIRECTION {
    DSD_FORWARD = 0,
    DSD_BACKWARD = 1,
}}
ENUM!{enum DESKTOP_WALLPAPER_POSITION {
    DWPOS_CENTER = 0,
    DWPOS_TILE = 1,
    DWPOS_STRETCH = 2,
    DWPOS_FIT = 3,
    DWPOS_FILL = 4,
    DWPOS_SPAN = 5,
}}
DEFINE_GUID!{IID_IDesktopWallpaper,
    0xb92b56a9, 0x8b55, 0x4e14, 0x9a, 0x89, 0x01, 0x99, 0xbb, 0xb6, 0xf9, 0x3b}
RIDL!{#[uuid(0xb92b56a9, 0x8b55, 0x4e14, 0x9a, 0x89, 0x01, 0x99, 0xbb, 0xb6, 0xf9, 0x3b)]
interface IDesktopWallpaper(IDesktopWallpaperVtbl): IUnknown(IUnknownVtbl) {
    fn SetWallpaper(
        monitorID: LPCWSTR,
        wallpaper: LPCWSTR,
    ) -> HRESULT,
    fn GetWallpaper(
        monitorID: LPCWSTR,
        wallpaper: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMonitorDevicePathAt(
        monitorIndex: UINT,
        monitorID: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMonitorDevicePathCount(
        count: *mut UINT,
    ) -> HRESULT,
    fn GetMonitorRECT(
        monitorID: LPCWSTR,
        displayRect: *mut RECT,
    ) -> HRESULT,
    fn SetBackgroundColor(
        color: COLORREF,
    ) -> HRESULT,
    fn GetBackgroundColor(
        color: *mut COLORREF,
    ) -> HRESULT,
    fn SetPosition(
        position: DESKTOP_WALLPAPER_POSITION,
    ) -> HRESULT,
    fn GetPosition(
        position: *mut DESKTOP_WALLPAPER_POSITION,
    ) -> HRESULT,
    fn SetSlideshow(
        items: *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSlideshow(
        items: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn SetSlideshowOptions(
        options: DESKTOP_SLIDESHOW_OPTIONS,
        slideshowTick: UINT,
    ) -> HRESULT,
    fn GetSlideshowOptions(
        options: *mut DESKTOP_SLIDESHOW_OPTIONS,
        slideshowTick: *mut UINT,
    ) -> HRESULT,
    fn AdvanceSlideshow(
        monitorID: LPCWSTR,
        direction: DESKTOP_SLIDESHOW_DIRECTION,
    ) -> HRESULT,
    fn GetStatus(
        state: *mut DESKTOP_SLIDESHOW_STATE,
    ) -> HRESULT,
    fn Enable(
        enable: BOOL,
    ) -> HRESULT,
}}
// #define HOMEGROUP_SECURITY_GROUP_MULTI L"HUG"
// #define HOMEGROUP_SECURITY_GROUP L"HomeUsers"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0148_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0148_v0_0_s_ifspec;
ENUM!{enum HOMEGROUPSHARINGCHOICES {
    HGSC_NONE = 0,
    HGSC_MUSICLIBRARY = 0x1,
    HGSC_PICTURESLIBRARY = 0x2,
    HGSC_VIDEOSLIBRARY = 0x4,
    HGSC_DOCUMENTSLIBRARY = 0x8,
    HGSC_PRINTERS = 0x10,
}}
DEFINE_GUID!{IID_IHomeGroup,
    0x7a3bd1d9, 0x35a9, 0x4fb3, 0xa4, 0x67, 0xf4, 0x8c, 0xac, 0x35, 0xe2, 0xd0}
RIDL!{#[uuid(0x7a3bd1d9, 0x35a9, 0x4fb3, 0xa4, 0x67, 0xf4, 0x8c, 0xac, 0x35, 0xe2, 0xd0)]
interface IHomeGroup(IHomeGroupVtbl): IUnknown(IUnknownVtbl) {
    fn IsMember(
        member: *mut BOOL,
    ) -> HRESULT,
    fn ShowSharingWizard(
        owner: HWND,
        sharingchoices: *mut HOMEGROUPSHARINGCHOICES,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IInitializeWithPropertyStore,
    0xc3e12eb5, 0x7d8d, 0x44f8, 0xb6, 0xdd, 0x0e, 0x77, 0xb3, 0x4d, 0x6d, 0xe4}
RIDL!{#[uuid(0xc3e12eb5, 0x7d8d, 0x44f8, 0xb6, 0xdd, 0x0e, 0x77, 0xb3, 0x4d, 0x6d, 0xe4)]
interface IInitializeWithPropertyStore(IInitializeWithPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pps: *mut IPropertyStore,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IOpenSearchSource,
    0xf0ee7333, 0xe6fc, 0x479b, 0x9f, 0x25, 0xa8, 0x60, 0xc2, 0x34, 0xa3, 0x8e}
RIDL!{#[uuid(0xf0ee7333, 0xe6fc, 0x479b, 0x9f, 0x25, 0xa8, 0x60, 0xc2, 0x34, 0xa3, 0x8e)]
interface IOpenSearchSource(IOpenSearchSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetResults(
        hwnd: HWND,
        pszQuery: LPCWSTR,
        dwStartIndex: DWORD,
        dwCount: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
ENUM!{enum LIBRARYFOLDERFILTER {
    LFF_FORCEFILESYSTEM = 1,
    LFF_STORAGEITEMS = 2,
    LFF_ALLITEMS = 3,
}}
ENUM!{enum LIBRARYOPTIONFLAGS {
    LOF_DEFAULT = 0,
    LOF_PINNEDTONAVPANE = 0x1,
    LOF_MASK_ALL = 0x1,
}}
ENUM!{enum DEFAULTSAVEFOLDERTYPE {
    DSFT_DETECT = 1,
    DSFT_PRIVATE = DSFT_DETECT + 1,
    DSFT_PUBLIC = DSFT_PRIVATE + 1,
}}
ENUM!{enum LIBRARYSAVEFLAGS {
    LSF_FAILIFTHERE = 0,
    LSF_OVERRIDEEXISTING = 0x1,
    LSF_MAKEUNIQUENAME = 0x2,
}}
DEFINE_GUID!{IID_IShellLibrary,
    0x11a66efa, 0x382e, 0x451a, 0x92, 0x34, 0x1e, 0x0e, 0x12, 0xef, 0x30, 0x85}
RIDL!{#[uuid(0x11a66efa, 0x382e, 0x451a, 0x92, 0x34, 0x1e, 0x0e, 0x12, 0xef, 0x30, 0x85)]
interface IShellLibrary(IShellLibraryVtbl): IUnknown(IUnknownVtbl) {
    fn LoadLibraryFromItem(
        psiLibrary: *mut IShellItem,
        grfMode: DWORD,
    ) -> HRESULT,
    fn LoadLibraryFromKnownFolder(
        kfidLibrary: REFKNOWNFOLDERID,
        grfMode: DWORD,
    ) -> HRESULT,
    fn AddFolder(
        psiLocation: *mut IShellItem,
    ) -> HRESULT,
    fn RemoveFolder(
        psiLocation: *mut IShellItem,
    ) -> HRESULT,
    fn GetFolders(
        lff: LIBRARYFOLDERFILTER,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn ResolveFolder(
        psiFolderToResolve: *mut IShellItem,
        dwTimeout: DWORD,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetDefaultSaveFolder(
        dsft: DEFAULTSAVEFOLDERTYPE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn SetDefaultSaveFolder(
        dsft: DEFAULTSAVEFOLDERTYPE,
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetOptions(
        plofOptions: *mut LIBRARYOPTIONFLAGS,
    ) -> HRESULT,
    fn SetOptions(
        lofMask: LIBRARYOPTIONFLAGS,
        lofOptions: LIBRARYOPTIONFLAGS,
    ) -> HRESULT,
    fn GetFolderType(
        pftid: *mut FOLDERTYPEID,
    ) -> HRESULT,
    fn SetFolderType(
        ftid: REFFOLDERTYPEID,
    ) -> HRESULT,
    fn GetIcon(
        ppszIcon: *mut LPWSTR,
    ) -> HRESULT,
    fn SetIcon(
        pszIcon: LPCWSTR,
    ) -> HRESULT,
    fn Commit() -> HRESULT,
    fn Save(
        psiFolderToSaveIn: *mut IShellItem,
        pszLibraryName: LPCWSTR,
        lsf: LIBRARYSAVEFLAGS,
        ppsiSavedTo: *mut *mut IShellItem,
    ) -> HRESULT,
    fn SaveInKnownFolder(
        kfidToSaveIn: REFKNOWNFOLDERID,
        pszLibraryName: LPCWSTR,
        lsf: LIBRARYSAVEFLAGS,
        ppsiSavedTo: *mut *mut IShellItem,
    ) -> HRESULT,
}}
ENUM!{enum DEFAULT_FOLDER_MENU_RESTRICTIONS {
    DFMR_DEFAULT = 0,
    DFMR_NO_STATIC_VERBS = 0x8,
    DFMR_STATIC_VERBS_ONLY = 0x10,
    DFMR_NO_RESOURCE_VERBS = 0x20,
    DFMR_OPTIN_HANDLERS_ONLY = 0x40,
    DFMR_RESOURCE_AND_FOLDER_VERBS_ONLY = 0x80,
    DFMR_USE_SPECIFIED_HANDLERS = 0x100,
    DFMR_USE_SPECIFIED_VERBS = 0x200,
    DFMR_NO_ASYNC_VERBS = 0x400,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0152_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0152_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDefaultFolderMenuInitialize,
    0x7690aa79, 0xf8fc, 0x4615, 0xa3, 0x27, 0x36, 0xf7, 0xd1, 0x8f, 0x5d, 0x91}
RIDL!{#[uuid(0x7690aa79, 0xf8fc, 0x4615, 0xa3, 0x27, 0x36, 0xf7, 0xd1, 0x8f, 0x5d, 0x91)]
interface IDefaultFolderMenuInitialize(IDefaultFolderMenuInitializeVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hwnd: HWND,
        pcmcb: *mut IContextMenuCB,
        pidlFolder: PCIDLIST_ABSOLUTE,
        psf: *mut IShellFolder,
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        punkAssociation: *mut IUnknown,
        cKeys: UINT,
        aKeys: *const HKEY,
    ) -> HRESULT,
    fn SetMenuRestrictions(
        dfmrValues: DEFAULT_FOLDER_MENU_RESTRICTIONS,
    ) -> HRESULT,
    fn GetMenuRestrictions(
        dfmrMask: DEFAULT_FOLDER_MENU_RESTRICTIONS,
        pdfmrValues: *mut DEFAULT_FOLDER_MENU_RESTRICTIONS,
    ) -> HRESULT,
    fn SetHandlerClsid(
        rclsid: REFCLSID,
    ) -> HRESULT,
}}
ENUM!{enum ACTIVATEOPTIONS {
    AO_NONE = 0,
    AO_DESIGNMODE = 0x1,
    AO_NOERRORUI = 0x2,
    AO_NOSPLASHSCREEN = 0x4,
    AO_PRELAUNCH = 0x2000000,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0153_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0153_v0_0_s_ifspec;
DEFINE_GUID!{IID_IApplicationActivationManager,
    0x2e941141, 0x7f97, 0x4756, 0xba, 0x1d, 0x9d, 0xec, 0xde, 0x89, 0x4a, 0x3d}
RIDL!{#[uuid(0x2e941141, 0x7f97, 0x4756, 0xba, 0x1d, 0x9d, 0xec, 0xde, 0x89, 0x4a, 0x3d)]
interface IApplicationActivationManager(IApplicationActivationManagerVtbl):
    IUnknown(IUnknownVtbl)
{
    fn ActivateApplication(
        appUserModelId: LPCWSTR,
        arguments: LPCWSTR,
        options: ACTIVATEOPTIONS,
        processId: *mut DWORD,
    ) -> HRESULT,
    fn ActivateForFile(
        appUserModelId: LPCWSTR,
        itemArray: *mut IShellItemArray,
        verb: LPCWSTR,
        processId: *mut DWORD,
    ) -> HRESULT,
    fn ActivateForProtocol(
        appUserModelId: LPCWSTR,
        itemArray: *mut IShellItemArray,
        processId: *mut DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0154_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0154_v0_0_s_ifspec;
// EXTERN_C const IID LIBID_ShellCoreObjects;
DEFINE_GUID!{CLSID_DesktopWallpaper,
    0xC2CF3110, 0x460E, 0x4fc1, 0xB9, 0xD0, 0x8A, 0x1C, 0x0C, 0x9C, 0xC4, 0xBD}
// class DECLSPEC_UUID("C2CF3110-460E-4fc1-B9D0-8A1C0C9CC4BD")
// DesktopWallpaper;
DEFINE_GUID!{CLSID_ShellDesktop,
    0x00021400, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
// class DECLSPEC_UUID("00021400-0000-0000-C000-000000000046")
// ShellDesktop;
DEFINE_GUID!{CLSID_ShellFSFolder,
    0xF3364BA0, 0x65B9, 0x11CE, 0xA9, 0xBA, 0x00, 0xAA, 0x00, 0x4A, 0xE8, 0x37}
// class DECLSPEC_UUID("F3364BA0-65B9-11CE-A9BA-00AA004AE837")
// ShellFSFolder;
DEFINE_GUID!{CLSID_NetworkPlaces,
    0x208D2C60, 0x3AEA, 0x1069, 0xA2, 0xD7, 0x08, 0x00, 0x2B, 0x30, 0x30, 0x9D}
// class DECLSPEC_UUID("208D2C60-3AEA-1069-A2D7-08002B30309D")
// NetworkPlaces;
DEFINE_GUID!{CLSID_ShellLink,
    0x00021401, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
// class DECLSPEC_UUID("00021401-0000-0000-C000-000000000046")
// ShellLink;
DEFINE_GUID!{CLSID_DriveSizeCategorizer,
    0x94357B53, 0xCA29, 0x4b78, 0x83, 0xAE, 0xE8, 0xFE, 0x74, 0x09, 0x13, 0x4F}
// class DECLSPEC_UUID("94357B53-CA29-4b78-83AE-E8FE7409134F")
// DriveSizeCategorizer;
DEFINE_GUID!{CLSID_DriveTypeCategorizer,
    0xB0A8F3CF, 0x4333, 0x4bab, 0x88, 0x73, 0x1C, 0xCB, 0x1C, 0xAD, 0xA4, 0x8B}
// class DECLSPEC_UUID("B0A8F3CF-4333-4bab-8873-1CCB1CADA48B")
// DriveTypeCategorizer;
DEFINE_GUID!{CLSID_FreeSpaceCategorizer,
    0xB5607793, 0x24AC, 0x44c7, 0x82, 0xE2, 0x83, 0x17, 0x26, 0xAA, 0x6C, 0xB7}
// class DECLSPEC_UUID("B5607793-24AC-44c7-82E2-831726AA6CB7")
// FreeSpaceCategorizer;
DEFINE_GUID!{CLSID_SizeCategorizer,
    0x55d7b852, 0xf6d1, 0x42f2, 0xaa, 0x75, 0x87, 0x28, 0xa1, 0xb2, 0xd2, 0x64}
// class DECLSPEC_UUID("55d7b852-f6d1-42f2-aa75-8728a1b2d264")
// SizeCategorizer;
DEFINE_GUID!{CLSID_PropertiesUI,
    0xd912f8cf, 0x0396, 0x4915, 0x88, 0x4e, 0xfb, 0x42, 0x5d, 0x32, 0x94, 0x3b}
// class DECLSPEC_UUID("d912f8cf-0396-4915-884e-fb425d32943b")
// PropertiesUI;
DEFINE_GUID!{CLSID_UserNotification,
    0x0010890e, 0x8789, 0x413c, 0xad, 0xbc, 0x48, 0xf5, 0xb5, 0x11, 0xb3, 0xaf}
// class DECLSPEC_UUID("0010890e-8789-413c-adbc-48f5b511b3af")
// UserNotification;
DEFINE_GUID!{CLSID_TaskbarList,
    0x56FDF344, 0xFD6D, 0x11d0, 0x95, 0x8A, 0x00, 0x60, 0x97, 0xC9, 0xA0, 0x90}
// class DECLSPEC_UUID("56FDF344-FD6D-11d0-958A-006097C9A090")
// TaskbarList;
DEFINE_GUID!{CLSID_ShellItem,
    0x9ac9fbe1, 0xe0a2, 0x4ad6, 0xb4, 0xee, 0xe2, 0x12, 0x01, 0x3e, 0xa9, 0x17}
// class DECLSPEC_UUID("9ac9fbe1-e0a2-4ad6-b4ee-e212013ea917")
// ShellItem;
DEFINE_GUID!{CLSID_NamespaceWalker,
    0x72eb61e0, 0x8672, 0x4303, 0x91, 0x75, 0xf2, 0xe4, 0xc6, 0x8b, 0x2e, 0x7c}
// class DECLSPEC_UUID("72eb61e0-8672-4303-9175-f2e4c68b2e7c")
// NamespaceWalker;
DEFINE_GUID!{CLSID_FileOperation,
    0x3ad05575, 0x8857, 0x4850, 0x92, 0x77, 0x11, 0xb8, 0x5b, 0xdb, 0x8e, 0x09}
// class DECLSPEC_UUID("3ad05575-8857-4850-9277-11b85bdb8e09")
// FileOperation;
DEFINE_GUID!{CLSID_FileOpenDialog,
    0xDC1C5A9C, 0xE88A, 0x4dde, 0xA5, 0xA1, 0x60, 0xF8, 0x2A, 0x20, 0xAE, 0xF7}
// class DECLSPEC_UUID("DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7")
// FileOpenDialog;
DEFINE_GUID!{CLSID_FileSaveDialog,
    0xC0B4E2F3, 0xBA21, 0x4773, 0x8D, 0xBA, 0x33, 0x5E, 0xC9, 0x46, 0xEB, 0x8B}
// class DECLSPEC_UUID("C0B4E2F3-BA21-4773-8DBA-335EC946EB8B")
// FileSaveDialog;
DEFINE_GUID!{CLSID_KnownFolderManager,
    0x4df0c730, 0xdf9d, 0x4ae3, 0x91, 0x53, 0xaa, 0x6b, 0x82, 0xe9, 0x79, 0x5a}
// class DECLSPEC_UUID("4df0c730-df9d-4ae3-9153-aa6b82e9795a")
// KnownFolderManager;
DEFINE_GUID!{CLSID_SharingConfigurationManager,
    0x49F371E1, 0x8C5C, 0x4d9c, 0x9A, 0x3B, 0x54, 0xA6, 0x82, 0x7F, 0x51, 0x3C}
// class DECLSPEC_UUID("49F371E1-8C5C-4d9c-9A3B-54A6827F513C")
// SharingConfigurationManager;
DEFINE_GUID!{CLSID_NetworkConnections,
    0x7007ACC7, 0x3202, 0x11D1, 0xAA, 0xD2, 0x00, 0x80, 0x5F, 0xC1, 0x27, 0x0E}
// class DECLSPEC_UUID("7007ACC7-3202-11D1-AAD2-00805FC1270E")
// NetworkConnections;
DEFINE_GUID!{CLSID_ScheduledTasks,
    0xD6277990, 0x4C6A, 0x11CF, 0x8D, 0x87, 0x00, 0xAA, 0x00, 0x60, 0xF5, 0xBF}
// class DECLSPEC_UUID("D6277990-4C6A-11CF-8D87-00AA0060F5BF")
// ScheduledTasks;
DEFINE_GUID!{CLSID_ApplicationAssociationRegistration,
    0x591209c7, 0x767b, 0x42b2, 0x9f, 0xba, 0x44, 0xee, 0x46, 0x15, 0xf2, 0xc7}
// class DECLSPEC_UUID("591209c7-767b-42b2-9fba-44ee4615f2c7")
// ApplicationAssociationRegistration;
DEFINE_GUID!{CLSID_SearchFolderItemFactory,
    0x14010e02, 0xbbbd, 0x41f0, 0x88, 0xe3, 0xed, 0xa3, 0x71, 0x21, 0x65, 0x84}
// class DECLSPEC_UUID("14010e02-bbbd-41f0-88e3-eda371216584")
// SearchFolderItemFactory;
DEFINE_GUID!{CLSID_OpenControlPanel,
    0x06622D85, 0x6856, 0x4460, 0x8D, 0xE1, 0xA8, 0x19, 0x21, 0xB4, 0x1C, 0x4B}
// class DECLSPEC_UUID("06622D85-6856-4460-8DE1-A81921B41C4B")
// OpenControlPanel;
DEFINE_GUID!{CLSID_MailRecipient,
    0x9E56BE60, 0xC50F, 0x11CF, 0x9A, 0x2C, 0x00, 0xA0, 0xC9, 0x0A, 0x90, 0xCE}
// class DECLSPEC_UUID("9E56BE60-C50F-11CF-9A2C-00A0C90A90CE")
// MailRecipient;
DEFINE_GUID!{CLSID_NetworkExplorerFolder,
    0xF02C1A0D, 0xBE21, 0x4350, 0x88, 0xB0, 0x73, 0x67, 0xFC, 0x96, 0xEF, 0x3C}
// class DECLSPEC_UUID("F02C1A0D-BE21-4350-88B0-7367FC96EF3C")
// NetworkExplorerFolder;
DEFINE_GUID!{CLSID_DestinationList,
    0x77f10cf0, 0x3db5, 0x4966, 0xb5, 0x20, 0xb7, 0xc5, 0x4f, 0xd3, 0x5e, 0xd6}
// class DECLSPEC_UUID("77f10cf0-3db5-4966-b520-b7c54fd35ed6")
// DestinationList;
DEFINE_GUID!{CLSID_ApplicationDestinations,
    0x86c14003, 0x4d6b, 0x4ef3, 0xa7, 0xb4, 0x05, 0x06, 0x66, 0x3b, 0x2e, 0x68}
// class DECLSPEC_UUID("86c14003-4d6b-4ef3-a7b4-0506663b2e68")
// ApplicationDestinations;
DEFINE_GUID!{CLSID_ApplicationDocumentLists,
    0x86bec222, 0x30f2, 0x47e0, 0x9f, 0x25, 0x60, 0xd1, 0x1c, 0xd7, 0x5c, 0x28}
// class DECLSPEC_UUID("86bec222-30f2-47e0-9f25-60d11cd75c28")
// ApplicationDocumentLists;
DEFINE_GUID!{CLSID_HomeGroup,
    0xDE77BA04, 0x3C92, 0x4d11, 0xA1, 0xA5, 0x42, 0x35, 0x2A, 0x53, 0xE0, 0xE3}
// class DECLSPEC_UUID("DE77BA04-3C92-4d11-A1A5-42352A53E0E3")
// HomeGroup;
DEFINE_GUID!{CLSID_ShellLibrary,
    0xd9b3211d, 0xe57f, 0x4426, 0xaa, 0xef, 0x30, 0xa8, 0x06, 0xad, 0xd3, 0x97}
// class DECLSPEC_UUID("d9b3211d-e57f-4426-aaef-30a806add397")
// ShellLibrary;
DEFINE_GUID!{CLSID_AppStartupLink,
    0x273eb5e7, 0x88b0, 0x4843, 0xbf, 0xef, 0xe2, 0xc8, 0x1d, 0x43, 0xaa, 0xe5}
// class DECLSPEC_UUID("273eb5e7-88b0-4843-bfef-e2c81d43aae5")
// AppStartupLink;
DEFINE_GUID!{CLSID_EnumerableObjectCollection,
    0x2d3468c1, 0x36a7, 0x43b6, 0xac, 0x24, 0xd3, 0xf0, 0x2f, 0xd9, 0x60, 0x7a}
// class DECLSPEC_UUID("2d3468c1-36a7-43b6-ac24-d3f02fd9607a")
// EnumerableObjectCollection;
DEFINE_GUID!{CLSID_FrameworkInputPane,
    0xD5120AA3, 0x46BA, 0x44C5, 0x82, 0x2D, 0xCA, 0x80, 0x92, 0xC1, 0xFC, 0x72}
// class DECLSPEC_UUID("D5120AA3-46BA-44C5-822D-CA8092C1FC72")
// FrameworkInputPane;
DEFINE_GUID!{CLSID_DefFolderMenu,
    0xc63382be, 0x7933, 0x48d0, 0x9a, 0xc8, 0x85, 0xfb, 0x46, 0xbe, 0x2f, 0xdd}
// class DECLSPEC_UUID("c63382be-7933-48d0-9ac8-85fb46be2fdd")
// DefFolderMenu;
DEFINE_GUID!{CLSID_AppVisibility,
    0x7E5FE3D9, 0x985F, 0x4908, 0x91, 0xF9, 0xEE, 0x19, 0xF9, 0xFD, 0x15, 0x14}
// class DECLSPEC_UUID("7E5FE3D9-985F-4908-91F9-EE19F9FD1514")
// AppVisibility;
DEFINE_GUID!{CLSID_AppShellVerbHandler,
    0x4ED3A719, 0xCEA8, 0x4BD9, 0x91, 0x0D, 0xE2, 0x52, 0xF9, 0x97, 0xAF, 0xC2}
// class DECLSPEC_UUID("4ED3A719-CEA8-4BD9-910D-E252F997AFC2")
// AppShellVerbHandler;
DEFINE_GUID!{CLSID_ExecuteUnknown,
    0xe44e9428, 0xbdbc, 0x4987, 0xa0, 0x99, 0x40, 0xdc, 0x8f, 0xd2, 0x55, 0xe7}
// class DECLSPEC_UUID("e44e9428-bdbc-4987-a099-40dc8fd255e7")
// ExecuteUnknown;
DEFINE_GUID!{CLSID_PackageDebugSettings,
    0xB1AEC16F, 0x2383, 0x4852, 0xB0, 0xE9, 0x8F, 0x0B, 0x1D, 0xC6, 0x6B, 0x4D}
// class DECLSPEC_UUID("B1AEC16F-2383-4852-B0E9-8F0B1DC66B4D")
// PackageDebugSettings;
DEFINE_GUID!{CLSID_SuspensionDependencyManager,
    0x6B273FC5, 0x61FD, 0x4918, 0x95, 0xA2, 0xC3, 0xB5, 0xE9, 0xD7, 0xF5, 0x81}
// class DECLSPEC_UUID("6B273FC5-61FD-4918-95A2-C3B5E9D7F581")
// SuspensionDependencyManager;
DEFINE_GUID!{CLSID_ApplicationActivationManager,
    0x45BA127D, 0x10A8, 0x46EA, 0x8A, 0xB7, 0x56, 0xEA, 0x90, 0x78, 0x94, 0x3C}
// class DECLSPEC_UUID("45BA127D-10A8-46EA-8AB7-56EA9078943C")
// ApplicationActivationManager;
DEFINE_GUID!{CLSID_ApplicationDesignModeSettings,
    0x958a6fb5, 0xdcb2, 0x4faf, 0xaa, 0xfd, 0x7f, 0xb0, 0x54, 0xad, 0x1a, 0x3b}
// class DECLSPEC_UUID("958a6fb5-dcb2-4faf-aafd-7fb054ad1a3b")
// ApplicationDesignModeSettings;
extern "system" {
    pub fn SHGetTemporaryPropertyForItem(
        psi: *mut IShellItem,
        propkey: REFPROPERTYKEY,
        ppropvar: *mut PROPVARIANT,
    ) -> HRESULT;
    pub fn SHSetTemporaryPropertyForItem(
        psi: *mut IShellItem,
        propkey: REFPROPERTYKEY,
        propvar: REFPROPVARIANT,
    ) -> HRESULT;
}
ENUM!{enum LIBRARYMANAGEDIALOGOPTIONS {
    LMD_DEFAULT = 0,
    LMD_ALLOWUNINDEXABLENETWORKLOCATIONS = 0x1,
}}
extern "system" {
    pub fn SHShowManageLibraryUI(
        psiLibrary: *mut IShellItem,
        hwndOwner: HWND,
        pszTitle: LPCWSTR,
        pszInstruction: LPCWSTR,
        lmdOptions: LIBRARYMANAGEDIALOGOPTIONS,
    ) -> HRESULT;
    pub fn SHResolveLibrary(
        psiLibrary: *mut IShellItem,
    ) -> HRESULT;
}
#[inline]
pub unsafe fn SHCreateLibrary(riid: REFIID, ppv: *mut *mut c_void) -> HRESULT {
    CoCreateInstance(&CLSID_ShellLibrary as *const _, ptr::null_mut(), CLSCTX_INPROC_SERVER, riid,
        ppv)
}
#[inline]
pub unsafe fn SHLoadLibraryFromItem(
    psiLibrary: *mut IShellItem,
    grfMode: DWORD,
    riid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    *ppv = ptr::null_mut();
    let mut plib: *mut IShellLibrary = ptr::null_mut();
    let mut hr = SHCreateLibrary(&IID_IShellLibrary as *const _,
        &mut plib as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        hr = (*plib).LoadLibraryFromItem(psiLibrary, grfMode);
        if SUCCEEDED(hr) {
            hr = (*plib).QueryInterface(riid, ppv);
        }
        (*plib).Release();
    }
    hr
}
#[inline]
pub unsafe fn SHLoadLibraryFromKnownFolder(
    kfidLibrary: REFKNOWNFOLDERID,
    grfMode: DWORD,
    riid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    *ppv = ptr::null_mut();
    let mut plib: *mut IShellLibrary = ptr::null_mut();
    let mut hr = SHCreateLibrary(&IID_IShellLibrary as *const _,
        &mut plib as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        hr = (*plib).LoadLibraryFromKnownFolder(kfidLibrary, grfMode);
        if SUCCEEDED(hr) {
            hr = (*plib).QueryInterface(riid, ppv);
        }
        (*plib).Release();
    }
    hr
}
#[inline]
pub unsafe fn SHLoadLibraryFromParsingName(
    pszParsingName: PCWSTR,
    grfMode: DWORD,
    riid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    *ppv = ptr::null_mut();
    let mut psiLibrary: *mut IShellItem = ptr::null_mut();
    let mut hr = SHCreateItemFromParsingName(pszParsingName, ptr::null_mut(),
        &IID_IShellItem as *const _, &mut psiLibrary as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        hr = SHLoadLibraryFromItem(psiLibrary, grfMode, riid, ppv);
        (*psiLibrary).Release();
    }
    hr
}
#[inline]
pub unsafe fn SHAddFolderPathToLibrary(plib: *mut IShellLibrary, pszFolderPath: PCWSTR) -> HRESULT {
    let mut psiFolder: *mut IShellItem = ptr::null_mut();
    let mut hr = SHCreateItemFromParsingName(pszFolderPath, ptr::null_mut(),
        &IID_IShellItem as *const _, &mut psiFolder as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        hr = (*plib).AddFolder(psiFolder);
        (*psiFolder).Release();
    }
    hr
}
#[inline]
pub unsafe fn SHRemoveFolderPathFromLibrary(plib: *mut IShellLibrary, pszFolderPath: PCWSTR) -> HRESULT {
    let pidlFolder = SHSimpleIDListFromPath(pszFolderPath);
    if pidlFolder.is_null() { return E_INVALIDARG }
    let mut psiFolder: *mut IShellItem = ptr::null_mut();
    let mut hr = SHCreateItemFromIDList(pidlFolder, &IID_IShellItem as *const _,
        &mut psiFolder as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        hr = (*plib).RemoveFolder(psiFolder);
        (*psiFolder).Release();
    }
    CoTaskMemFree(pidlFolder as *mut c_void);
    hr
}
#[inline]
pub unsafe fn SHSaveLibraryInFolderPath(
    plib: *mut IShellLibrary,
    pszFolderPath: PCWSTR,
    pszLibraryName: PCWSTR,
    lsf: LIBRARYSAVEFLAGS,
    ppszSavedToPath: *mut PWSTR,
) -> HRESULT {
    if !ppszSavedToPath.is_null() {
        *ppszSavedToPath = ptr::null_mut()
    }

    let mut psiFolder: *mut IShellItem = ptr::null_mut();
    let mut hr = SHCreateItemFromParsingName(pszFolderPath, ptr::null_mut(),
        &IID_IShellItem as *const _, &mut psiFolder as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        let mut psiSavedTo: *mut IShellItem = ptr::null_mut();
        hr = (*plib).Save(psiFolder, pszLibraryName, lsf, &mut psiSavedTo as *mut *mut _);
        if SUCCEEDED(hr) {
            if !ppszSavedToPath.is_null() {
                hr = (*psiSavedTo).GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING, ppszSavedToPath);
            }
            (*psiSavedTo).Release();
        }
        (*psiFolder).Release();
    }
    hr
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0155_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0155_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAssocHandlerInvoker,
    0x92218cab, 0xecaa, 0x4335, 0x81, 0x33, 0x80, 0x7f, 0xd2, 0x34, 0xc2, 0xee}
RIDL!{#[uuid(0x92218cab, 0xecaa, 0x4335, 0x81, 0x33, 0x80, 0x7f, 0xd2, 0x34, 0xc2, 0xee)]
interface IAssocHandlerInvoker(IAssocHandlerInvokerVtbl): IUnknown(IUnknownVtbl) {
    fn SupportsSelection() -> HRESULT,
    fn Invoke() -> HRESULT,
}}
DEFINE_GUID!{IID_IAssocHandler,
    0xf04061ac, 0x1659, 0x4a3f, 0xa9, 0x54, 0x77, 0x5a, 0xa5, 0x7f, 0xc0, 0x83}
RIDL!{#[uuid(0xf04061ac, 0x1659, 0x4a3f, 0xa9, 0x54, 0x77, 0x5a, 0xa5, 0x7f, 0xc0, 0x83)]
interface IAssocHandler(IAssocHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn GetName(
        ppsz: *mut LPWSTR,
    ) -> HRESULT,
    fn GetUIName(
        ppsz: *mut LPWSTR,
    ) -> HRESULT,
    fn GetIconLocation(
        ppszPath: *mut LPWSTR,
        pIndex: *mut c_int,
    ) -> HRESULT,
    fn IsRecommended() -> HRESULT,
    fn MakeDefault(
        pszDescription: LPCWSTR,
    ) -> HRESULT,
    fn Invoke(
        pdo: *mut IDataObject,
    ) -> HRESULT,
    fn CreateInvoker(
        pdo: *mut IDataObject,
        ppInvoker: *mut *mut IAssocHandlerInvoker,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumAssocHandlers,
    0x973810ae, 0x9599, 0x4b88, 0x9e, 0x4d, 0x6e, 0xe9, 0x8c, 0x95, 0x52, 0xda}
RIDL!{#[uuid(0x973810ae, 0x9599, 0x4b88, 0x9e, 0x4d, 0x6e, 0xe9, 0x8c, 0x95, 0x52, 0xda)]
interface IEnumAssocHandlers(IEnumAssocHandlersVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IAssocHandler,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}}
ENUM!{enum ASSOC_FILTER {
    ASSOC_FILTER_NONE = 0,
    ASSOC_FILTER_RECOMMENDED = 0x1,
}}
extern "system" {
    pub fn SHAssocEnumHandlers(
        pszExtra: PCWSTR,
        afFilter: ASSOC_FILTER,
        ppEnumHandler: *mut *mut IEnumAssocHandlers,
    ) -> HRESULT;
    pub fn SHAssocEnumHandlersForProtocolByApplication(
        protocol: PCWSTR,
        riid: REFIID,
        enumHandlers: *mut *mut c_void,
    ) -> HRESULT;
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0158_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0158_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDataObjectProvider,
    0x3d25f6d6, 0x4b2a, 0x433c, 0x91, 0x84, 0x7c, 0x33, 0xad, 0x35, 0xd0, 0x01}
RIDL!{#[uuid(0x3d25f6d6, 0x4b2a, 0x433c, 0x91, 0x84, 0x7c, 0x33, 0xad, 0x35, 0xd0, 0x01)]
interface IDataObjectProvider(IDataObjectProviderVtbl): IUnknown(IUnknownVtbl) {
    fn GetDataObject(
        dataObject: *mut *mut IDataObject,
    ) -> HRESULT,
    fn SetDataObject(
        dataObject: *mut IDataObject,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDataTransferManagerInterop,
    0x3a3dcd6c, 0x3eab, 0x43dc, 0xbc, 0xde, 0x45, 0x67, 0x1c, 0xe8, 0x00, 0xc8}
RIDL!{#[uuid(0x3a3dcd6c, 0x3eab, 0x43dc, 0xbc, 0xde, 0x45, 0x67, 0x1c, 0xe8, 0x00, 0xc8)]
interface IDataTransferManagerInterop(IDataTransferManagerInteropVtbl): IUnknown(IUnknownVtbl) {
    fn GetForWindow(
        appWindow: HWND,
        riid: REFIID,
        dataTransferManager: *mut *mut c_void,
    ) -> HRESULT,
    fn ShowShareUIForWindow(
        appWindow: HWND,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0160_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0160_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFrameworkInputPaneHandler,
    0x226c537b, 0x1e76, 0x4d9e, 0xa7, 0x60, 0x33, 0xdb, 0x29, 0x92, 0x2f, 0x18}
RIDL!{#[uuid(0x226c537b, 0x1e76, 0x4d9e, 0xa7, 0x60, 0x33, 0xdb, 0x29, 0x92, 0x2f, 0x18)]
interface IFrameworkInputPaneHandler(IFrameworkInputPaneHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn Showing(
        prcInputPaneScreenLocation: *mut RECT,
        fEnsureFocusedElementInView: BOOL,
    ) -> HRESULT,
    fn Hiding(
        fEnsureFocusedElementInView: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFrameworkInputPane,
    0x5752238b, 0x24f0, 0x495a, 0x82, 0xf1, 0x2f, 0xd5, 0x93, 0x05, 0x67, 0x96}
RIDL!{#[uuid(0x5752238b, 0x24f0, 0x495a, 0x82, 0xf1, 0x2f, 0xd5, 0x93, 0x05, 0x67, 0x96)]
interface IFrameworkInputPane(IFrameworkInputPaneVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        pWindow: *mut IUnknown,
        pHandler: *mut IFrameworkInputPaneHandler,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn AdviseWithHWND(
        hwnd: HWND,
        pHandler: *mut IFrameworkInputPaneHandler,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn Location(
        prcInputPaneScreenLocation: *mut RECT,
    ) -> HRESULT,
}}
// #define PROP_CONTRACT_DELEGATE L"ContractDelegate"
// #[inline]
// pub unsafe fn SetContractDelegateWindow(hwndSource: HWND, hwndDelegate: HWND) {
//     if !hwndDelegate.is_null() {
//         SetPropW(hwndSource, PROP_CONTRACT_DELEGATE, hwndDelegate as HANDLE);
//     } else {
//         RemovePropW(hwndSource, PROP_CONTRACT_DELEGATE);
//     }
// }
ENUM!{enum MONITOR_APP_VISIBILITY {
    MAV_UNKNOWN = 0,
    MAV_NO_APP_VISIBLE = 1,
    MAV_APP_VISIBLE = 2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0162_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0162_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAppVisibilityEvents,
    0x6584ce6b, 0x7d82, 0x49c2, 0x89, 0xc9, 0xc6, 0xbc, 0x02, 0xba, 0x8c, 0x38}
RIDL!{#[uuid(0x6584ce6b, 0x7d82, 0x49c2, 0x89, 0xc9, 0xc6, 0xbc, 0x02, 0xba, 0x8c, 0x38)]
interface IAppVisibilityEvents(IAppVisibilityEventsVtbl): IUnknown(IUnknownVtbl) {
    fn AppVisibilityOnMonitorChanged(
        hMonitor: HMONITOR,
        previousMode: MONITOR_APP_VISIBILITY,
        currentMode: MONITOR_APP_VISIBILITY,
    ) -> HRESULT,
    fn LauncherVisibilityChange(
        currentVisibleState: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IAppVisibility,
    0x2246ea2d, 0xcaea, 0x4444, 0xa3, 0xc4, 0x6d, 0xe8, 0x27, 0xe4, 0x43, 0x13}
RIDL!{#[uuid(0x2246ea2d, 0xcaea, 0x4444, 0xa3, 0xc4, 0x6d, 0xe8, 0x27, 0xe4, 0x43, 0x13)]
interface IAppVisibility(IAppVisibilityVtbl): IUnknown(IUnknownVtbl) {
    fn GetAppVisibilityOnMonitor(
        hMonitor: HMONITOR,
        pMode: *mut MONITOR_APP_VISIBILITY,
    ) -> HRESULT,
    fn IsLauncherVisible(
        pfVisible: *mut BOOL,
    ) -> HRESULT,
    fn Advise(
        pCallback: *mut IAppVisibilityEvents,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum PACKAGE_EXECUTION_STATE {
    PES_UNKNOWN = 0,
    PES_RUNNING = 1,
    PES_SUSPENDING = 2,
    PES_SUSPENDED = 3,
    PES_TERMINATED = 4,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0164_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0164_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPackageExecutionStateChangeNotification,
    0x1bb12a62, 0x2ad8, 0x432b, 0x8c, 0xcf, 0x0c, 0x2c, 0x52, 0xaf, 0xcd, 0x5b}
RIDL!{#[uuid(0x1bb12a62, 0x2ad8, 0x432b, 0x8c, 0xcf, 0x0c, 0x2c, 0x52, 0xaf, 0xcd, 0x5b)]
interface IPackageExecutionStateChangeNotification(IPackageExecutionStateChangeNotificationVtbl):
    IUnknown(IUnknownVtbl)
{
    fn OnStateChanged(
        pszPackageFullName: LPCWSTR,
        pesNewState: PACKAGE_EXECUTION_STATE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPackageDebugSettings,
    0xf27c3930, 0x8029, 0x4ad1, 0x94, 0xe3, 0x3d, 0xba, 0x41, 0x78, 0x10, 0xc1}
RIDL!{#[uuid(0xf27c3930, 0x8029, 0x4ad1, 0x94, 0xe3, 0x3d, 0xba, 0x41, 0x78, 0x10, 0xc1)]
interface IPackageDebugSettings(IPackageDebugSettingsVtbl): IUnknown(IUnknownVtbl) {
    fn EnableDebugging(
        packageFullName: LPCWSTR,
        debuggerCommandLine: LPCWSTR,
        environment: PZZWSTR,
    ) -> HRESULT,
    fn DisableDebugging(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn Suspend(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn Resume(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn TerminateAllProcesses(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn SetTargetSessionId(
        sessionId: ULONG,
    ) -> HRESULT,
    fn EnumerateBackgroundTasks(
        packageFullName: LPCWSTR,
        taskCount: *mut ULONG,
        taskIds: *mut LPCGUID,
        taskNames: *mut *mut LPCWSTR,
    ) -> HRESULT,
    fn ActivateBackgroundTask(
        taskId: LPCGUID,
    ) -> HRESULT,
    fn StartServicing(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn StopServicing(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn StartSessionRedirection(
        packageFullName: LPCWSTR,
        sessionId: ULONG,
    ) -> HRESULT,
    fn StopSessionRedirection(
        packageFullName: LPCWSTR,
    ) -> HRESULT,
    fn GetPackageExecutionState(
        packageFullName: LPCWSTR,
        packageExecutionState: *mut PACKAGE_EXECUTION_STATE,
    ) -> HRESULT,
    fn RegisterForPackageStateChanges(
        packageFullName: LPCWSTR,
        pPackageExecutionStateChangeNotification: *mut IPackageExecutionStateChangeNotification,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterForPackageStateChanges(
        dwCookie: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPackageDebugSettings2,
    0x6e3194bb, 0xab82, 0x4d22, 0x93, 0xf5, 0xfa, 0xbd, 0xa4, 0x0e, 0x7b, 0x16}
RIDL!{#[uuid(0x6e3194bb, 0xab82, 0x4d22, 0x93, 0xf5, 0xfa, 0xbd, 0xa4, 0x0e, 0x7b, 0x16)]
interface IPackageDebugSettings2(IPackageDebugSettings2Vtbl):
    IPackageDebugSettings(IPackageDebugSettingsVtbl)
{
    fn EnumerateApps(
        packageFullName: LPCWSTR,
        appCount: *mut ULONG,
        appUserModelIds: *mut *mut LPWSTR,
        appDisplayNames: *mut *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ISuspensionDependencyManager,
    0x52b83a42, 0x2543, 0x416a, 0x81, 0xd9, 0xc0, 0xde, 0x79, 0x69, 0xc8, 0xb3}
RIDL!{#[uuid(0x52b83a42, 0x2543, 0x416a, 0x81, 0xd9, 0xc0, 0xde, 0x79, 0x69, 0xc8, 0xb3)]
interface ISuspensionDependencyManager(ISuspensionDependencyManagerVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterAsChild(
        processHandle: HANDLE,
    ) -> HRESULT,
    fn GroupChildWithParent(
        childProcessHandle: HANDLE,
    ) -> HRESULT,
    fn UngroupChildFromParent(
        childProcessHandle: HANDLE,
    ) -> HRESULT,
}}
ENUM!{enum AHE_TYPE {
    AHE_DESKTOP = 0,
    AHE_IMMERSIVE = 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0168_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0168_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExecuteCommandApplicationHostEnvironment,
    0x18b21aa9, 0xe184, 0x4ff0, 0x9f, 0x5e, 0xf8, 0x82, 0xd0, 0x37, 0x71, 0xb3}
RIDL!{#[uuid(0x18b21aa9, 0xe184, 0x4ff0, 0x9f, 0x5e, 0xf8, 0x82, 0xd0, 0x37, 0x71, 0xb3)]
interface IExecuteCommandApplicationHostEnvironment(IExecuteCommandApplicationHostEnvironmentVtbl):
    IUnknown(IUnknownVtbl)
{
    fn GetValue(
        pahe: *mut AHE_TYPE,
    ) -> HRESULT,
}}
ENUM!{enum EC_HOST_UI_MODE {
    ECHUIM_DESKTOP = 0,
    ECHUIM_IMMERSIVE = ECHUIM_DESKTOP + 1,
    ECHUIM_SYSTEM_LAUNCHER = ECHUIM_IMMERSIVE + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0169_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0169_v0_0_s_ifspec;
DEFINE_GUID!{IID_IExecuteCommandHost,
    0x4b6832a2, 0x5f04, 0x4c9d, 0xb8, 0x9d, 0x72, 0x7a, 0x15, 0xd1, 0x03, 0xe7}
RIDL!{#[uuid(0x4b6832a2, 0x5f04, 0x4c9d, 0xb8, 0x9d, 0x72, 0x7a, 0x15, 0xd1, 0x03, 0xe7)]
interface IExecuteCommandHost(IExecuteCommandHostVtbl): IUnknown(IUnknownVtbl) {
    fn GetUIMode(
        pUIMode: *mut EC_HOST_UI_MODE,
    ) -> HRESULT,
}}
// #define SID_ExecuteCommandHost IID_IExecuteCommandHost
ENUM!{enum APPLICATION_VIEW_STATE {
    AVS_FULLSCREEN_LANDSCAPE = 0,
    AVS_FILLED = AVS_FULLSCREEN_LANDSCAPE + 1,
    AVS_SNAPPED = AVS_FILLED + 1,
    AVS_FULLSCREEN_PORTRAIT = AVS_SNAPPED + 1,
}}
ENUM!{enum EDGE_GESTURE_KIND {
    EGK_TOUCH = 0,
    EGK_KEYBOARD = EGK_TOUCH + 1,
    EGK_MOUSE = EGK_KEYBOARD + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0170_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0170_v0_0_s_ifspec;
DEFINE_GUID!{IID_IApplicationDesignModeSettings,
    0x2a3dee9a, 0xe31d, 0x46d6, 0x85, 0x08, 0xbc, 0xc5, 0x97, 0xdb, 0x35, 0x57}
RIDL!{#[uuid(0x2a3dee9a, 0xe31d, 0x46d6, 0x85, 0x08, 0xbc, 0xc5, 0x97, 0xdb, 0x35, 0x57)]
interface IApplicationDesignModeSettings(IApplicationDesignModeSettingsVtbl):
    IUnknown(IUnknownVtbl)
{
    fn SetNativeDisplaySize(
        nativeDisplaySizePixels: SIZE,
    ) -> HRESULT,
    fn SetScaleFactor(
        scaleFactor: DEVICE_SCALE_FACTOR,
    ) -> HRESULT,
    fn SetApplicationViewState(
        viewState: APPLICATION_VIEW_STATE,
    ) -> HRESULT,
    fn ComputeApplicationSize(
        applicationSizePixels: *mut SIZE,
    ) -> HRESULT,
    fn IsApplicationViewStateSupported(
        viewState: APPLICATION_VIEW_STATE,
        nativeDisplaySizePixels: SIZE,
        scaleFactor: DEVICE_SCALE_FACTOR,
        supported: *mut BOOL,
    ) -> HRESULT,
    fn TriggerEdgeGesture(
        edgeGestureKind: EDGE_GESTURE_KIND,
    ) -> HRESULT,
}}
ENUM!{enum NATIVE_DISPLAY_ORIENTATION {
    NDO_LANDSCAPE = 0,
    NDO_PORTRAIT = NDO_LANDSCAPE + 1,
}}
ENUM!{enum APPLICATION_VIEW_ORIENTATION {
    AVO_LANDSCAPE = 0,
    AVO_PORTRAIT = AVO_LANDSCAPE + 1,
}}
ENUM!{enum ADJACENT_DISPLAY_EDGES {
    ADE_NONE = 0,
    ADE_LEFT = 0x1,
    ADE_RIGHT = 0x2,
}}
ENUM!{enum APPLICATION_VIEW_MIN_WIDTH {
    AVMW_DEFAULT = 0,
    AVMW_320 = 1,
    AVMW_500 = 2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0171_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0171_v0_0_s_ifspec;
DEFINE_GUID!{IID_IApplicationDesignModeSettings2,
    0x490514e1, 0x675a, 0x4d6e, 0xa5, 0x8d, 0xe5, 0x49, 0x01, 0xb4, 0xca, 0x2f}
RIDL!{#[uuid(0x490514e1, 0x675a, 0x4d6e, 0xa5, 0x8d, 0xe5, 0x49, 0x01, 0xb4, 0xca, 0x2f)]
interface IApplicationDesignModeSettings2(IApplicationDesignModeSettings2Vtbl):
    IApplicationDesignModeSettings(IApplicationDesignModeSettingsVtbl)
{
    fn SetNativeDisplayOrientation(
        nativeDisplayOrientation: NATIVE_DISPLAY_ORIENTATION,
    ) -> HRESULT,
    fn SetApplicationViewOrientation(
        viewOrientation: APPLICATION_VIEW_ORIENTATION,
    ) -> HRESULT,
    fn SetAdjacentDisplayEdges(
        adjacentDisplayEdges: ADJACENT_DISPLAY_EDGES,
    ) -> HRESULT,
    fn SetIsOnLockScreen(
        isOnLockScreen: BOOL,
    ) -> HRESULT,
    fn SetApplicationViewMinWidth(
        viewMinWidth: APPLICATION_VIEW_MIN_WIDTH,
    ) -> HRESULT,
    fn GetApplicationSizeBounds(
        minApplicationSizePixels: *mut SIZE,
        maxApplicationSizePixels: *mut SIZE,
    ) -> HRESULT,
    fn GetApplicationViewOrientation(
        applicationSizePixels: SIZE,
        viewOrientation: *mut APPLICATION_VIEW_ORIENTATION,
    ) -> HRESULT,
}}
DEFINE_GUID!{SID_URLExecutionContext,
    0xFB5F8EBC, 0xBBB6, 0x4D10, 0xA4, 0x61, 0x77, 0x72, 0x91, 0xA0, 0x90, 0x30}
// #define STR_TAB_REUSE_IDENTIFIER L"Tab Reuse Identifier"
// #define STR_REFERRER_IDENTIFIER L"Referrer Identifier"
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0172_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0172_v0_0_s_ifspec;
DEFINE_GUID!{IID_ILaunchTargetMonitor,
    0x266fbc7e, 0x490d, 0x46ed, 0xa9, 0x6b, 0x22, 0x74, 0xdb, 0x25, 0x20, 0x03}
RIDL!{#[uuid(0x266fbc7e, 0x490d, 0x46ed, 0xa9, 0x6b, 0x22, 0x74, 0xdb, 0x25, 0x20, 0x03)]
interface ILaunchTargetMonitor(ILaunchTargetMonitorVtbl): IUnknown(IUnknownVtbl) {
    fn GetMonitor(
        monitor: *mut HMONITOR,
    ) -> HRESULT,
}}
// #define SID_LaunchTargetMonitor __uuidof((struct __declspec(uuid("8D547FA1-CC45-40C8-B7C1-D48C183F13F3")) LaunchTargetMonitor*)0)
ENUM!{enum APPLICATION_VIEW_SIZE_PREFERENCE {
    AVSP_DEFAULT = 0,
    AVSP_USE_LESS = 1,
    AVSP_USE_HALF = 2,
    AVSP_USE_MORE = 3,
    AVSP_USE_MINIMUM = 4,
    AVSP_USE_NONE = 5,
    AVSP_CUSTOM = 6,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0173_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0173_v0_0_s_ifspec;
DEFINE_GUID!{IID_ILaunchSourceViewSizePreference,
    0xe5aa01f7, 0x1fb8, 0x4830, 0x87, 0x20, 0x4e, 0x67, 0x34, 0xcb, 0xd5, 0xf3}
RIDL!{#[uuid(0xe5aa01f7, 0x1fb8, 0x4830, 0x87, 0x20, 0x4e, 0x67, 0x34, 0xcb, 0xd5, 0xf3)]
interface ILaunchSourceViewSizePreference(ILaunchSourceViewSizePreferenceVtbl):
    IUnknown(IUnknownVtbl)
{
    fn GetSourceViewToPosition(
        hwnd: *mut HWND,
    ) -> HRESULT,
    fn GetSourceViewSizePreference(
        sourceSizeAfterLaunch: *mut APPLICATION_VIEW_SIZE_PREFERENCE,
    ) -> HRESULT,
}}
DEFINE_GUID!{SID_LaunchSourceViewSizePreference,
    0x80605492, 0x67d9, 0x414f, 0xaf, 0x89, 0xa1, 0xcd, 0xf1, 0x24, 0x2b, 0xc1}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0174_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0174_v0_0_s_ifspec;
DEFINE_GUID!{IID_ILaunchTargetViewSizePreference,
    0x2f0666c6, 0x12f7, 0x4360, 0xb5, 0x11, 0xa3, 0x94, 0xa0, 0x55, 0x37, 0x25}
RIDL!{#[uuid(0x2f0666c6, 0x12f7, 0x4360, 0xb5, 0x11, 0xa3, 0x94, 0xa0, 0x55, 0x37, 0x25)]
interface ILaunchTargetViewSizePreference(ILaunchTargetViewSizePreferenceVtbl):
    IUnknown(IUnknownVtbl)
{
    fn GetTargetViewSizePreference(
        targetSizeOnLaunch: *mut APPLICATION_VIEW_SIZE_PREFERENCE,
    ) -> HRESULT,
}}
DEFINE_GUID!{SID_LaunchTargetViewSizePreference,
    0x26db2472, 0xb7b7, 0x406b, 0x97, 0x2, 0x73, 0xa, 0x4e, 0x20, 0xd3, 0xbf}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0175_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0175_v0_0_s_ifspec;
DEFINE_GUID!{IID_ILaunchSourceAppUserModelId,
    0x989191ac, 0x28ff, 0x4cf0, 0x95, 0x84, 0xe0, 0xd0, 0x78, 0xbc, 0x23, 0x96}
RIDL!{#[uuid(0x989191ac, 0x28ff, 0x4cf0, 0x95, 0x84, 0xe0, 0xd0, 0x78, 0xbc, 0x23, 0x96)]
interface ILaunchSourceAppUserModelId(ILaunchSourceAppUserModelIdVtbl): IUnknown(IUnknownVtbl) {
    fn GetAppUserModelId(
        launchingApp: *mut LPWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{SID_LaunchSourceAppUserModelId,
    0x2ce78010, 0x74db, 0x48bc, 0x9c, 0x6a, 0x10, 0xf3, 0x72, 0x49, 0x57, 0x23}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0176_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0176_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInitializeWithWindow,
    0x3e68d4bd, 0x7135, 0x4d10, 0x80, 0x18, 0x9f, 0xb6, 0xd9, 0xf3, 0x3f, 0xa1}
RIDL!{#[uuid(0x3e68d4bd, 0x7135, 0x4d10, 0x80, 0x18, 0x9f, 0xb6, 0xd9, 0xf3, 0x3f, 0xa1)]
interface IInitializeWithWindow(IInitializeWithWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hwnd: HWND,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IHandlerInfo,
    0x997706ef, 0xf880, 0x453b, 0x81, 0x18, 0x39, 0xe1, 0xa2, 0xd2, 0x65, 0x5a}
RIDL!{#[uuid(0x997706ef, 0xf880, 0x453b, 0x81, 0x18, 0x39, 0xe1, 0xa2, 0xd2, 0x65, 0x5a)]
interface IHandlerInfo(IHandlerInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetApplicationDisplayName(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn GetApplicationPublisher(
        value: *mut LPWSTR,
    ) -> HRESULT,
    fn GetApplicationIconReference(
        value: *mut LPWSTR,
    ) -> HRESULT,
}}
// #define SID_HandlerInfo IID_IHandlerInfo
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0178_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0178_v0_0_s_ifspec;
DEFINE_GUID!{IID_IHandlerActivationHost,
    0x35094a87, 0x8bb1, 0x4237, 0x96, 0xc6, 0xc4, 0x17, 0xee, 0xbd, 0xb0, 0x78}
RIDL!{#[uuid(0x35094a87, 0x8bb1, 0x4237, 0x96, 0xc6, 0xc4, 0x17, 0xee, 0xbd, 0xb0, 0x78)]
interface IHandlerActivationHost(IHandlerActivationHostVtbl): IUnknown(IUnknownVtbl) {
    fn BeforeCoCreateInstance(
        clsidHandler: REFCLSID,
        itemsBeingActivated: *mut IShellItemArray,
        handlerInfo: *mut IHandlerInfo,
    ) -> HRESULT,
    fn BeforeCreateProcess(
        applicationPath: LPCWSTR,
        commandLine: LPCWSTR,
        handlerInfo: *mut IHandlerInfo,
    ) -> HRESULT,
}}
// #define SID_SHandlerActivationHost IID_IHandlerActivationHost
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0179_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0179_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAppActivationUIInfo,
    0xabad189d, 0x9fa3, 0x4278, 0xb3, 0xca, 0x8c, 0xa4, 0x48, 0xa8, 0x8d, 0xcb}
RIDL!{#[uuid(0xabad189d, 0x9fa3, 0x4278, 0xb3, 0xca, 0x8c, 0xa4, 0x48, 0xa8, 0x8d, 0xcb)]
interface IAppActivationUIInfo(IAppActivationUIInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetMonitor(
        value: *mut HMONITOR,
    ) -> HRESULT,
    fn GetInvokePoint(
        value: *mut POINT,
    ) -> HRESULT,
    fn GetShowCommand(
        value: *mut c_int,
    ) -> HRESULT,
    fn GetShowUI(
        value: *mut BOOL,
    ) -> HRESULT,
    fn GetKeyState(
        value: *mut DWORD,
    ) -> HRESULT,
}}
// #define SID_AppActivationUIInfo IID_IAppActivationUIInfo
DEFINE_GUID!{SID_ShellExecuteNamedPropertyStore,
    0xeb84ada2, 0x00ff, 0x4992, 0x83, 0x24, 0xed, 0x5c, 0xe0, 0x61, 0xcb, 0x29}
ENUM!{enum FLYOUT_PLACEMENT {
    FP_DEFAULT = 0,
    FP_ABOVE = FP_DEFAULT + 1,
    FP_BELOW = FP_ABOVE + 1,
    FP_LEFT = FP_BELOW + 1,
    FP_RIGHT = FP_LEFT + 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0180_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0180_v0_0_s_ifspec;
DEFINE_GUID!{IID_IContactManagerInterop,
    0x99eacba7, 0xe073, 0x43b6, 0xa8, 0x96, 0x55, 0xaf, 0xe4, 0x8a, 0x08, 0x33}
RIDL!{#[uuid(0x99eacba7, 0xe073, 0x43b6, 0xa8, 0x96, 0x55, 0xaf, 0xe4, 0x8a, 0x08, 0x33)]
interface IContactManagerInterop(IContactManagerInteropVtbl): IUnknown(IUnknownVtbl) {
    fn ShowContactCardForWindow(
        appWindow: HWND,
        contact: *mut IUnknown,
        selection: *const RECT,
        preferredPlacement: FLYOUT_PLACEMENT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0181_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_core_0000_0181_v0_0_s_ifspec;
// unsigned long __RPC_USER BSTR_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// c_void __RPC_USER BSTR_UserFree( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER HACCEL_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HACCEL * );
// unsigned char * __RPC_USER HACCEL_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HACCEL * );
// unsigned char * __RPC_USER HACCEL_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HACCEL * );
// c_void __RPC_USER HACCEL_UserFree( __RPC__in unsigned long *, __RPC__in HACCEL * );
// unsigned long __RPC_USER HBITMAP_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER HBITMAP_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * );
// c_void __RPC_USER HBITMAP_UserFree( __RPC__in unsigned long *, __RPC__in HBITMAP * );
// unsigned long __RPC_USER HGLOBAL_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER HGLOBAL_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER HGLOBAL_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HGLOBAL * );
// c_void __RPC_USER HGLOBAL_UserFree( __RPC__in unsigned long *, __RPC__in HGLOBAL * );
// unsigned long __RPC_USER HICON_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HICON * );
// unsigned char * __RPC_USER HICON_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * );
// unsigned char * __RPC_USER HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * );
// c_void __RPC_USER HICON_UserFree( __RPC__in unsigned long *, __RPC__in HICON * );
// unsigned long __RPC_USER HMENU_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HMENU * );
// unsigned char * __RPC_USER HMENU_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * );
// unsigned char * __RPC_USER HMENU_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * );
// c_void __RPC_USER HMENU_UserFree( __RPC__in unsigned long *, __RPC__in HMENU * );
// unsigned long __RPC_USER HMONITOR_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HMONITOR * );
// unsigned char * __RPC_USER HMONITOR_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * );
// unsigned char * __RPC_USER HMONITOR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * );
// c_void __RPC_USER HMONITOR_UserFree( __RPC__in unsigned long *, __RPC__in HMONITOR * );
// unsigned long __RPC_USER HWND_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in HWND * );
// unsigned char * __RPC_USER HWND_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// c_void __RPC_USER HWND_UserFree( __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// c_void __RPC_USER LPSAFEARRAY_UserFree( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// unsigned long __RPC_USER PCIDLIST_ABSOLUTE_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PCIDLIST_ABSOLUTE_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PCIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * );
// c_void __RPC_USER PCIDLIST_ABSOLUTE_UserFree( __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned long __RPC_USER PCUIDLIST_RELATIVE_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PCUIDLIST_RELATIVE_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PCUIDLIST_RELATIVE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCUIDLIST_RELATIVE * );
// c_void __RPC_USER PCUIDLIST_RELATIVE_UserFree( __RPC__in unsigned long *, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned long __RPC_USER PCUITEMID_CHILD_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PCUITEMID_CHILD * );
// unsigned char * __RPC_USER PCUITEMID_CHILD_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCUITEMID_CHILD * );
// unsigned char * __RPC_USER PCUITEMID_CHILD_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCUITEMID_CHILD * );
// c_void __RPC_USER PCUITEMID_CHILD_UserFree( __RPC__in unsigned long *, __RPC__in PCUITEMID_CHILD * );
// unsigned long __RPC_USER PIDLIST_ABSOLUTE_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PIDLIST_ABSOLUTE_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_ABSOLUTE * );
// c_void __RPC_USER PIDLIST_ABSOLUTE_UserFree( __RPC__in unsigned long *, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned long __RPC_USER PIDLIST_RELATIVE_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PIDLIST_RELATIVE_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PIDLIST_RELATIVE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_RELATIVE * );
// c_void __RPC_USER PIDLIST_RELATIVE_UserFree( __RPC__in unsigned long *, __RPC__in PIDLIST_RELATIVE * );
// unsigned long __RPC_USER PITEMID_CHILD_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in PITEMID_CHILD * );
// unsigned char * __RPC_USER PITEMID_CHILD_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PITEMID_CHILD * );
// unsigned char * __RPC_USER PITEMID_CHILD_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PITEMID_CHILD * );
// c_void __RPC_USER PITEMID_CHILD_UserFree( __RPC__in unsigned long *, __RPC__in PITEMID_CHILD * );
// unsigned long __RPC_USER VARIANT_UserSize( __RPC__in unsigned long *, unsigned long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// c_void __RPC_USER VARIANT_UserFree( __RPC__in unsigned long *, __RPC__in VARIANT * );
// unsigned long __RPC_USER BSTR_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// c_void __RPC_USER BSTR_UserFree64( __RPC__in unsigned long *, __RPC__in BSTR * );
// unsigned long __RPC_USER HACCEL_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HACCEL * );
// unsigned char * __RPC_USER HACCEL_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HACCEL * );
// unsigned char * __RPC_USER HACCEL_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HACCEL * );
// c_void __RPC_USER HACCEL_UserFree64( __RPC__in unsigned long *, __RPC__in HACCEL * );
// unsigned long __RPC_USER HBITMAP_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER HBITMAP_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * );
// unsigned char * __RPC_USER HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * );
// c_void __RPC_USER HBITMAP_UserFree64( __RPC__in unsigned long *, __RPC__in HBITMAP * );
// unsigned long __RPC_USER HGLOBAL_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER HGLOBAL_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER HGLOBAL_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HGLOBAL * );
// c_void __RPC_USER HGLOBAL_UserFree64( __RPC__in unsigned long *, __RPC__in HGLOBAL * );
// unsigned long __RPC_USER HICON_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HICON * );
// unsigned char * __RPC_USER HICON_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * );
// unsigned char * __RPC_USER HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * );
// c_void __RPC_USER HICON_UserFree64( __RPC__in unsigned long *, __RPC__in HICON * );
// unsigned long __RPC_USER HMENU_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HMENU * );
// unsigned char * __RPC_USER HMENU_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * );
// unsigned char * __RPC_USER HMENU_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * );
// c_void __RPC_USER HMENU_UserFree64( __RPC__in unsigned long *, __RPC__in HMENU * );
// unsigned long __RPC_USER HMONITOR_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HMONITOR * );
// unsigned char * __RPC_USER HMONITOR_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * );
// unsigned char * __RPC_USER HMONITOR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * );
// c_void __RPC_USER HMONITOR_UserFree64( __RPC__in unsigned long *, __RPC__in HMONITOR * );
// unsigned long __RPC_USER HWND_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in HWND * );
// unsigned char * __RPC_USER HWND_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// c_void __RPC_USER HWND_UserFree64( __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long __RPC_USER LPSAFEARRAY_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * );
// unsigned char * __RPC_USER LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * );
// c_void __RPC_USER LPSAFEARRAY_UserFree64( __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * );
// unsigned long __RPC_USER PCIDLIST_ABSOLUTE_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PCIDLIST_ABSOLUTE_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PCIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCIDLIST_ABSOLUTE * );
// c_void __RPC_USER PCIDLIST_ABSOLUTE_UserFree64( __RPC__in unsigned long *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned long __RPC_USER PCUIDLIST_RELATIVE_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PCUIDLIST_RELATIVE_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PCUIDLIST_RELATIVE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCUIDLIST_RELATIVE * );
// c_void __RPC_USER PCUIDLIST_RELATIVE_UserFree64( __RPC__in unsigned long *, __RPC__in PCUIDLIST_RELATIVE * );
// unsigned long __RPC_USER PCUITEMID_CHILD_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PCUITEMID_CHILD * );
// unsigned char * __RPC_USER PCUITEMID_CHILD_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PCUITEMID_CHILD * );
// unsigned char * __RPC_USER PCUITEMID_CHILD_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PCUITEMID_CHILD * );
// c_void __RPC_USER PCUITEMID_CHILD_UserFree64( __RPC__in unsigned long *, __RPC__in PCUITEMID_CHILD * );
// unsigned long __RPC_USER PIDLIST_ABSOLUTE_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PIDLIST_ABSOLUTE_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned char * __RPC_USER PIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_ABSOLUTE * );
// c_void __RPC_USER PIDLIST_ABSOLUTE_UserFree64( __RPC__in unsigned long *, __RPC__in PIDLIST_ABSOLUTE * );
// unsigned long __RPC_USER PIDLIST_RELATIVE_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PIDLIST_RELATIVE_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PIDLIST_RELATIVE * );
// unsigned char * __RPC_USER PIDLIST_RELATIVE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PIDLIST_RELATIVE * );
// c_void __RPC_USER PIDLIST_RELATIVE_UserFree64( __RPC__in unsigned long *, __RPC__in PIDLIST_RELATIVE * );
// unsigned long __RPC_USER PITEMID_CHILD_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in PITEMID_CHILD * );
// unsigned char * __RPC_USER PITEMID_CHILD_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in PITEMID_CHILD * );
// unsigned char * __RPC_USER PITEMID_CHILD_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out PITEMID_CHILD * );
// c_void __RPC_USER PITEMID_CHILD_UserFree64( __RPC__in unsigned long *, __RPC__in PITEMID_CHILD * );
// unsigned long __RPC_USER VARIANT_UserSize64( __RPC__in unsigned long *, unsigned long, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserMarshal64( __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// c_void __RPC_USER VARIANT_UserFree64( __RPC__in unsigned long *, __RPC__in VARIANT * );
// HRESULT STDMETHODCALLTYPE IEnumIDList_Next_Proxy(
//     IEnumIDList * This,
//     _In_ ULONG celt,
//     _Out_writes_to_(celt, *pceltFetched) PITEMID_CHILD *rgelt,
//     _Out_opt_ _Deref_out_range_(0, celt) ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumIDList_Next_Stub(
//     IEnumIDList * This,
//     ULONG celt,
//     PITEMID_CHILD *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumFullIDList_Next_Proxy(
//     IEnumFullIDList * This,
//     _In_ ULONG celt,
//     _Out_writes_to_(celt, *pceltFetched) PIDLIST_ABSOLUTE *rgelt,
//     _Out_opt_ _Deref_out_range_(0, celt) ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumFullIDList_Next_Stub(
//     IEnumFullIDList * This,
//     ULONG celt,
//     PIDLIST_ABSOLUTE *rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IShellFolder_SetNameOf_Proxy(
//     IShellFolder * This,
//     _In_opt_ HWND hwnd,
//     _In_ PCUITEMID_CHILD pidl,
//     _In_ LPCWSTR pszName,
//     _In_ SHGDNF uFlags,
//     _Outptr_opt_ PITEMID_CHILD *ppidlOut);
// HRESULT STDMETHODCALLTYPE IShellFolder_SetNameOf_Stub(
//     IShellFolder * This,
//     HWND hwnd,
//     PCUITEMID_CHILD pidl,
//     LPCWSTR pszName,
//     SHGDNF uFlags,
//     PITEMID_CHILD *ppidlOut);
// HRESULT STDMETHODCALLTYPE IFolderView2_GetGroupBy_Proxy(
//     IFolderView2 * This,
//     _Out_ PROPERTYKEY *pkey,
//     _Out_opt_ BOOL *pfAscending);
// HRESULT STDMETHODCALLTYPE IFolderView2_GetGroupBy_Stub(
//     IFolderView2 * This,
//     PROPERTYKEY *pkey,
//     BOOL *pfAscending);
// HRESULT STDMETHODCALLTYPE IEnumShellItems_Next_Proxy(
//     IEnumShellItems * This,
//     _In_ ULONG celt,
//     _Out_writes_to_(celt, *pceltFetched) IShellItem **rgelt,
//     _Out_opt_ _Deref_out_range_(0, celt) ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumShellItems_Next_Stub(
//     IEnumShellItems * This,
//     ULONG celt,
//     IShellItem **rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IParentAndItem_GetParentAndItem_Proxy(
//     IParentAndItem * This,
//     _Outptr_opt_ PIDLIST_ABSOLUTE *ppidlParent,
//     _Outptr_opt_ IShellFolder **ppsf,
//     _Outptr_opt_ PITEMID_CHILD *ppidlChild);
// HRESULT STDMETHODCALLTYPE IParentAndItem_GetParentAndItem_Stub(
//     IParentAndItem * This,
//     PIDLIST_ABSOLUTE *ppidlParent,
//     IShellFolder **ppsf,
//     PITEMID_CHILD *ppidlChild);
// HRESULT STDMETHODCALLTYPE IEnumObjects_Next_Proxy(
//     IEnumObjects * This,
//     _In_ ULONG celt,
//     _In_ REFIID riid,
//     _Out_writes_to_(celt, *pceltFetched) c_void **rgelt,
//     _Out_opt_ _Deref_out_range_(0, celt) ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumObjects_Next_Stub(
//     IEnumObjects * This,
//     ULONG celt,
//     REFIID riid,
//     c_void **rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IBandSite_QueryBand_Proxy(
//     IBandSite * This,
//     _In_ DWORD dwBandID,
//     _Outptr_opt_ IDeskBand **ppstb,
//     _Out_opt_ DWORD *pdwState,
//     _Out_writes_opt_(cchName) LPWSTR pszName,
//     _In_ c_int cchName);
// HRESULT STDMETHODCALLTYPE IBandSite_QueryBand_Stub(
//     IBandSite * This,
//     DWORD dwBandID,
//     IDeskBand **ppstb,
//     DWORD *pdwState,
//     LPWSTR pszName,
//     c_int cchName);
// HRESULT STDMETHODCALLTYPE IModalWindow_Show_Proxy(
//     IModalWindow * This,
//     _In_opt_ HWND hwndOwner);
// HRESULT STDMETHODCALLTYPE IModalWindow_Show_Stub(
//     IModalWindow * This,
//     HWND hwndOwner);
// HRESULT STDMETHODCALLTYPE IKnownFolderManager_Redirect_Proxy(
//     IKnownFolderManager * This,
//     _In_ REFKNOWNFOLDERID rfid,
//     _In_opt_ HWND hwnd,
//     _In_ KF_REDIRECT_FLAGS flags,
//     _In_opt_ LPCWSTR pszTargetPath,
//     _In_ UINT cFolders,
//     _In_reads_opt_(cFolders) KNOWNFOLDERID const *pExclusion,
//     _Outptr_opt_result_maybenull_ LPWSTR *ppszError);
// HRESULT STDMETHODCALLTYPE IKnownFolderManager_Redirect_Stub(
//     IKnownFolderManager * This,
//     REFKNOWNFOLDERID rfid,
//     HWND hwnd,
//     KF_REDIRECT_FLAGS flags,
//     LPCWSTR pszTargetPath,
//     UINT cFolders,
//     GUID const *pExclusion,
//     LPWSTR *ppszError);
// HRESULT STDMETHODCALLTYPE IEnumExplorerCommand_Next_Proxy(
//     IEnumExplorerCommand * This,
//     _In_ ULONG celt,
//     _Out_writes_to_(celt, *pceltFetched) IExplorerCommand **pUICommand,
//     _Out_opt_ _Deref_out_range_(0, celt) ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumExplorerCommand_Next_Stub(
//     IEnumExplorerCommand * This,
//     ULONG celt,
//     IExplorerCommand **pUICommand,
//     ULONG *pceltFetched);
