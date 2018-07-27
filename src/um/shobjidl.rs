// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use _core::ptr;
use ctypes::{c_char, c_int, c_void};
use shared::guiddef::{GUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, LPARAM, LPDWORD, LRESULT, UINT, ULONG, WPARAM};
use shared::windef::{COLORREF, HBITMAP, HDC, HICON, HMONITOR, HWND, POINT, RECT};
use shared::winerror::{E_INVALIDARG, SUCCEEDED};
use shared::wtypes::BSTR;
use um::combaseapi::CoTaskMemFree;
use um::commctrl::HIMAGELIST;
use um::minwinbase::LPOVERLAPPED;
use um::msxml::IXMLDOMDocument;
use um::oaidl::VARIANT;
use um::objidl::{IDataObject, IStorage};
use um::objidlbase::{IStream, IStreamVtbl};
use um::propsys::IPropertyStore;
use um::prsht::HPROPSHEETPAGE;
use um::shobjidl_core::{
    FOLDERFLAGS, FOLDERVIEWMODE, ICommDlgBrowser2, ICommDlgBrowser2Vtbl, IContextMenu, IDeskBand, IDeskBandVtbl,
    IDragSourceHelper, IDragSourceHelperVtbl, IEnumIDList, IFileDialog, IFileDialogCustomize, IFileDialogVtbl, IFileOperation,
    IFileOperationProgressSink, IID_IShellItem, INameSpaceTreeControl, INameSpaceTreeControlVtbl, IQueryContinue, IRelatedItem, IRelatedItemVtbl,
    IShellBrowser, IShellItem, IShellItemArray, IShellLibrary, IShellView, IShellView2, IShellView2Vtbl,
    NSTCITEMSTATE, NSTCSTYLE, SHCreateItemFromIDList, SHELLVIEWID, SHSimpleIDListFromPath,
    SIGDN_DESKTOPABSOLUTEPARSING
};
use um::shtypes::{PCIDLIST_ABSOLUTE, PCUIDLIST_RELATIVE, PITEMID_CHILD};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::LOGFONTW;
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, PCWSTR, PWSTR};
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_IQueryCodePage,
    0xc7b236ce, 0xee80, 0x11d0, 0x98, 0x5f, 0x00, 0x60, 0x08, 0x05, 0x93, 0x82}
RIDL!{#[uuid(0xc7b236ce, 0xee80, 0x11d0, 0x98, 0x5f, 0x00, 0x60, 0x08, 0x05, 0x93, 0x82)]
interface IQueryCodePage(IQueryCodePageVtbl): IUnknown(IUnknownVtbl) {
    fn GetCodePage(
        puiCodePage: *mut UINT,
    ) -> HRESULT,
    fn SetCodePage(
        uiCodePage: UINT,
    ) -> HRESULT,
}}
ENUM!{enum SYNC_ENGINE_STATE_FLAGS {
    SESF_NONE = 0,
    SESF_SERVICE_QUOTA_NEARING_LIMIT = 0x1,
    SESF_SERVICE_QUOTA_EXCEEDED_LIMIT = 0x2,
    SESF_AUTHENTICATION_ERROR = 0x4,
    SESF_PAUSED_DUE_TO_METERED_NETWORK = 0x8,
    SESF_PAUSED_DUE_TO_DISK_SPACE_FULL = 0x10,
    SESF_PAUSED_DUE_TO_CLIENT_POLICY = 0x20,
    SESF_PAUSED_DUE_TO_SERVICE_POLICY = 0x40,
    SESF_SERVICE_UNAVAILABLE = 0x80,
    SESF_PAUSED_DUE_TO_USER_REQUEST = 0x100,
    SESF_ALL_FLAGS = SESF_NONE | SESF_SERVICE_QUOTA_NEARING_LIMIT
        | SESF_SERVICE_QUOTA_EXCEEDED_LIMIT | SESF_AUTHENTICATION_ERROR
        | SESF_PAUSED_DUE_TO_METERED_NETWORK | SESF_PAUSED_DUE_TO_DISK_SPACE_FULL
        | SESF_PAUSED_DUE_TO_CLIENT_POLICY | SESF_PAUSED_DUE_TO_SERVICE_POLICY
        | SESF_SERVICE_UNAVAILABLE | SESF_PAUSED_DUE_TO_USER_REQUEST,
}}
pub type LPVIEWSETTINGS = *mut c_char;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0001_v0_0_s_ifspec;
ENUM!{enum FOLDERVIEWOPTIONS {
    FVO_DEFAULT = 0,
    FVO_VISTALAYOUT = 0x1,
    FVO_CUSTOMPOSITION = 0x2,
    FVO_CUSTOMORDERING = 0x4,
    FVO_SUPPORTHYPERLINKS = 0x8,
    FVO_NOANIMATIONS = 0x10,
    FVO_NOSCROLLTIPS = 0x20,
}}
DEFINE_GUID!{IID_IFolderViewOptions,
    0x3cc974d2, 0xb302, 0x4d36, 0xad, 0x3e, 0x06, 0xd9, 0x3f, 0x69, 0x5d, 0x3f}
RIDL!{#[uuid(0x3cc974d2, 0xb302, 0x4d36, 0xad, 0x3e, 0x06, 0xd9, 0x3f, 0x69, 0x5d, 0x3f)]
interface IFolderViewOptions(IFolderViewOptionsVtbl): IUnknown(IUnknownVtbl) {
    fn SetFolderViewOptions(
        fvoMask: FOLDERVIEWOPTIONS,
        fvoFlags: FOLDERVIEWOPTIONS,
    ) -> HRESULT,
    fn GetFolderViewOptions(
        pfvoFlags: *mut FOLDERVIEWOPTIONS,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0002_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0002_v0_0_s_ifspec;
ENUM!{enum _SV3CVW3_FLAGS {
    SV3CVW3_DEFAULT = 0,
    SV3CVW3_NONINTERACTIVE = 0x1,
    SV3CVW3_FORCEVIEWMODE = 0x2,
    SV3CVW3_FORCEFOLDERFLAGS = 0x4,
}}
pub type SV3CVW3_FLAGS = DWORD;
DEFINE_GUID!{IID_IShellView3,
    0xec39fa88, 0xf8af, 0x41c5, 0x84, 0x21, 0x38, 0xbe, 0xd2, 0x8f, 0x46, 0x73}
RIDL!{#[uuid(0xec39fa88, 0xf8af, 0x41c5, 0x84, 0x21, 0x38, 0xbe, 0xd2, 0x8f, 0x46, 0x73)]
interface IShellView3(IShellView3Vtbl): IShellView2(IShellView2Vtbl) {
    fn CreateViewWindow3(
        psbOwner: *mut IShellBrowser,
        psvPrev: *mut IShellView,
        dwViewFlags: SV3CVW3_FLAGS,
        dwMask: FOLDERFLAGS,
        dwFlags: FOLDERFLAGS,
        fvMode: FOLDERVIEWMODE,
        pvid: *const SHELLVIEWID,
        prcView: *const RECT,
        phwndView: *mut HWND,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0003_v0_0_s_ifspec;
DEFINE_GUID!{IID_ISearchBoxInfo,
    0x6af6e03f, 0xd664, 0x4ef4, 0x96, 0x26, 0xf7, 0xe0, 0xed, 0x36, 0x75, 0x5e}
RIDL!{#[uuid(0x6af6e03f, 0xd664, 0x4ef4, 0x96, 0x26, 0xf7, 0xe0, 0xed, 0x36, 0x75, 0x5e)]
interface ISearchBoxInfo(ISearchBoxInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetCondition(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetText(
        ppsz: *mut LPWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0004_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0004_v0_0_s_ifspec;
ENUM!{enum VPWATERMARKFLAGS {
    VPWF_DEFAULT = 0,
    VPWF_ALPHABLEND = 0x1,
}}
ENUM!{enum VPCOLORFLAGS {
    VPCF_TEXT = 1,
    VPCF_BACKGROUND = 2,
    VPCF_SORTCOLUMN = 3,
    VPCF_SUBTEXT = 4,
    VPCF_TEXTBACKGROUND = 5,
}}
DEFINE_GUID!{IID_IVisualProperties,
    0xe693cf68, 0xd967, 0x4112, 0x87, 0x63, 0x99, 0x17, 0x2a, 0xee, 0x5e, 0x5a}
RIDL!{#[uuid(0xe693cf68, 0xd967, 0x4112, 0x87, 0x63, 0x99, 0x17, 0x2a, 0xee, 0x5e, 0x5a)]
interface IVisualProperties(IVisualPropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn SetWatermark(
        hbmp: HBITMAP,
        vpwf: VPWATERMARKFLAGS,
    ) -> HRESULT,
    fn SetColor(
        vpcf: VPCOLORFLAGS,
        cr: COLORREF,
    ) -> HRESULT,
    fn GetColor(
        vpcf: VPCOLORFLAGS,
        pcr: *mut COLORREF,
    ) -> HRESULT,
    fn SetItemHeight(
        cyItemInPixels: c_int,
    ) -> HRESULT,
    fn GetItemHeight(
        cyItemInPixels: *mut c_int,
    ) -> HRESULT,
    fn SetFont(
        plf: *const LOGFONTW,
        bRedraw: BOOL,
    ) -> HRESULT,
    fn GetFont(
        plf: *mut LOGFONTW,
    ) -> HRESULT,
    fn SetTheme(
        pszSubAppName: LPCWSTR,
        pszSubIdList: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0005_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICommDlgBrowser3,
    0xc8ad25a1, 0x3294, 0x41ee, 0x81, 0x65, 0x71, 0x17, 0x4b, 0xd0, 0x1c, 0x57}
RIDL!{#[uuid(0xc8ad25a1, 0x3294, 0x41ee, 0x81, 0x65, 0x71, 0x17, 0x4b, 0xd0, 0x1c, 0x57)]
interface ICommDlgBrowser3(ICommDlgBrowser3Vtbl): ICommDlgBrowser2(ICommDlgBrowser2Vtbl) {
    fn OnColumnClicked(
        ppshv: *mut IShellView,
        iColumn: c_int,
    ) -> HRESULT,
    fn GetCurrentFilter(
        pszFileSpec: LPWSTR,
        cchFileSpec: c_int,
    ) -> HRESULT,
    fn OnPreViewCreated(
        ppshv: *mut IShellView,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0006_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0006_v0_0_s_ifspec;
DEFINE_GUID!{IID_IUserAccountChangeCallback,
    0xa561e69a, 0xb4b8, 0x4113, 0x91, 0xa5, 0x64, 0xc6, 0xbc, 0xca, 0x34, 0x30}
RIDL!{#[uuid(0xa561e69a, 0xb4b8, 0x4113, 0x91, 0xa5, 0x64, 0xc6, 0xbc, 0xca, 0x34, 0x30)]
interface IUserAccountChangeCallback(IUserAccountChangeCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnPictureChange(
        pszUserName: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0007_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0007_v0_0_s_ifspec;
DEFINE_GUID!{IID_IStreamAsync,
    0xfe0b6665, 0xe0ca, 0x49b9, 0xa1, 0x78, 0x2b, 0x5c, 0xb4, 0x8d, 0x92, 0xa5}
RIDL!{#[uuid(0xfe0b6665, 0xe0ca, 0x49b9, 0xa1, 0x78, 0x2b, 0x5c, 0xb4, 0x8d, 0x92, 0xa5)]
interface IStreamAsync(IStreamAsyncVtbl): IStream(IStreamVtbl) {
    fn ReadAsync(
        pv: *mut c_void,
        cb: DWORD,
        pcbRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> HRESULT,
    fn WriteAsync(
        lpBuffer: *const c_void,
        cb: DWORD,
        pcbWritten: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> HRESULT,
    fn OverlappedResult(
        lpOverlapped: LPOVERLAPPED,
        lpNumberOfBytesTransferred: LPDWORD,
        bWait: BOOL,
    ) -> HRESULT,
    fn CancelIo() -> HRESULT,
}}
DEFINE_GUID!{IID_IStreamUnbufferedInfo,
    0x8a68fdda, 0x1fdc, 0x4c20, 0x8c, 0xeb, 0x41, 0x66, 0x43, 0xb5, 0xa6, 0x25}
RIDL!{#[uuid(0x8a68fdda, 0x1fdc, 0x4c20, 0x8c, 0xeb, 0x41, 0x66, 0x43, 0xb5, 0xa6, 0x25)]
interface IStreamUnbufferedInfo(IStreamUnbufferedInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetSectorSize(
        pcbSectorSize: *mut ULONG,
    ) -> HRESULT,
}}
extern "system" {
    pub fn SHRemovePersonalPropertyValues(
        psia: *mut IShellItemArray,
    ) -> HRESULT;
    pub fn SHAddDefaultPropertiesByExt(
        pszExt: PCWSTR,
        pPropStore: *mut IPropertyStore,
    ) -> HRESULT;
    pub fn SHCreateDefaultPropertiesOp(
        psi: *mut IShellItem,
        ppFileOp: *mut *mut IFileOperation,
    ) -> HRESULT;
    pub fn SHSetDefaultProperties(
        hwnd: HWND,
        psi: *mut IShellItem,
        dwFileOpFlags: DWORD,
        pfops: *mut IFileOperationProgressSink,
    ) -> HRESULT;
}
ENUM!{enum DSH_FLAGS {
    DSH_ALLOWDROPDESCRIPTIONTEXT = 0x1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0009_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0009_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDragSourceHelper2,
    0x83e07d0d, 0x0c5f, 0x4163, 0xbf, 0x1a, 0x60, 0xb2, 0x74, 0x05, 0x1e, 0x40}
RIDL!{#[uuid(0x83e07d0d, 0x0c5f, 0x4163, 0xbf, 0x1a, 0x60, 0xb2, 0x74, 0x05, 0x1e, 0x40)]
interface IDragSourceHelper2(IDragSourceHelper2Vtbl): IDragSourceHelper(IDragSourceHelperVtbl) {
    fn SetFlags(
        dwFlags: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0010_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0010_v0_0_s_ifspec;
DEFINE_GUID!{IID_IHWEventHandler,
    0xc1fb73d0, 0xec3a, 0x4ba2, 0xb5, 0x12, 0x8c, 0xdb, 0x91, 0x87, 0xb6, 0xd1}
RIDL!{#[uuid(0xc1fb73d0, 0xec3a, 0x4ba2, 0xb5, 0x12, 0x8c, 0xdb, 0x91, 0x87, 0xb6, 0xd1)]
interface IHWEventHandler(IHWEventHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pszParams: LPCWSTR,
    ) -> HRESULT,
    fn HandleEvent(
        pszDeviceID: LPCWSTR,
        pszAltDeviceID: LPCWSTR,
        pszEventType: LPCWSTR,
    ) -> HRESULT,
    fn HandleEventWithContent(
        pszDeviceID: LPCWSTR,
        pszAltDeviceID: LPCWSTR,
        pszEventType: LPCWSTR,
        pszContentTypeHandler: LPCWSTR,
        pdataobject: *mut IDataObject,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IHWEventHandler2,
    0xcfcc809f, 0x295d, 0x42e8, 0x9f, 0xfc, 0x42, 0x4b, 0x33, 0xc4, 0x87, 0xe6}
RIDL!{#[uuid(0xcfcc809f, 0x295d, 0x42e8, 0x9f, 0xfc, 0x42, 0x4b, 0x33, 0xc4, 0x87, 0xe6)]
interface IHWEventHandler2(IHWEventHandler2Vtbl): IHWEventHandler(IHWEventHandlerVtbl) {
    fn HandleEventWithHWND(
        pszDeviceID: LPCWSTR,
        pszAltDeviceID: LPCWSTR,
        pszEventType: LPCWSTR,
        hwndOwner: HWND,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IQueryCancelAutoPlay,
    0xddefe873, 0x6997, 0x4e68, 0xbe, 0x26, 0x39, 0xb6, 0x33, 0xad, 0xbe, 0x12}
RIDL!{#[uuid(0xddefe873, 0x6997, 0x4e68, 0xbe, 0x26, 0x39, 0xb6, 0x33, 0xad, 0xbe, 0x12)]
interface IQueryCancelAutoPlay(IQueryCancelAutoPlayVtbl): IUnknown(IUnknownVtbl) {
    fn AllowAutoPlay(
        pszPath: LPCWSTR,
        dwContentType: DWORD,
        pszLabel: LPCWSTR,
        dwSerialNumber: DWORD,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0013_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0013_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDynamicHWHandler,
    0xdc2601d7, 0x059e, 0x42fc, 0xa0, 0x9d, 0x2a, 0xfd, 0x21, 0xb6, 0xd5, 0xf7}
RIDL!{#[uuid(0xdc2601d7, 0x059e, 0x42fc, 0xa0, 0x9d, 0x2a, 0xfd, 0x21, 0xb6, 0xd5, 0xf7)]
interface IDynamicHWHandler(IDynamicHWHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn GetDynamicInfo(
        pszDeviceID: LPCWSTR,
        dwContentType: DWORD,
        ppszAction: *mut LPWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0014_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0014_v0_0_s_ifspec;
DEFINE_GUID!{IID_IUserNotificationCallback,
    0x19108294, 0x0441, 0x4aff, 0x80, 0x13, 0xfa, 0x0a, 0x73, 0x0b, 0x0b, 0xea}
RIDL!{#[uuid(0x19108294, 0x0441, 0x4aff, 0x80, 0x13, 0xfa, 0x0a, 0x73, 0x0b, 0x0b, 0xea)]
interface IUserNotificationCallback(IUserNotificationCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn OnBalloonUserClick(
        pt: *mut POINT,
    ) -> HRESULT,
    fn OnLeftClick(
        pt: *mut POINT,
    ) -> HRESULT,
    fn OnContextMenu(
        pt: *mut POINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IUserNotification2,
    0x215913cc, 0x57eb, 0x4fab, 0xab, 0x5a, 0xe5, 0xfa, 0x7b, 0xea, 0x2a, 0x6c}
RIDL!{#[uuid(0x215913cc, 0x57eb, 0x4fab, 0xab, 0x5a, 0xe5, 0xfa, 0x7b, 0xea, 0x2a, 0x6c)]
interface IUserNotification2(IUserNotification2Vtbl): IUnknown(IUnknownVtbl) {
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
        pSink: *mut IUserNotificationCallback,
    ) -> HRESULT,
    fn PlaySound(
        pszSoundName: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0016_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0016_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDeskBand2,
    0x79d16de4, 0xabee, 0x4021, 0x8d, 0x9d, 0x91, 0x69, 0xb2, 0x61, 0xd6, 0x57}
RIDL!{#[uuid(0x79d16de4, 0xabee, 0x4021, 0x8d, 0x9d, 0x91, 0x69, 0xb2, 0x61, 0xd6, 0x57)]
interface IDeskBand2(IDeskBand2Vtbl): IDeskBand(IDeskBandVtbl) {
    fn CanRenderComposited(
        pfCanRenderComposited: *mut BOOL,
    ) -> HRESULT,
    fn SetCompositionState(
        fCompositionEnabled: BOOL,
    ) -> HRESULT,
    fn GetCompositionState(
        pfCompositionEnabled: *mut BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0017_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0017_v0_0_s_ifspec;
DEFINE_GUID!{IID_IStartMenuPinnedList,
    0x4cd19ada, 0x25a5, 0x4a32, 0xb3, 0xb7, 0x34, 0x7b, 0xee, 0x5b, 0xe3, 0x6b}
RIDL!{#[uuid(0x4cd19ada, 0x25a5, 0x4a32, 0xb3, 0xb7, 0x34, 0x7b, 0xee, 0x5b, 0xe3, 0x6b)]
interface IStartMenuPinnedList(IStartMenuPinnedListVtbl): IUnknown(IUnknownVtbl) {
    fn RemoveFromList(
        pitem: *mut IShellItem,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ICDBurn,
    0x3d73a659, 0xe5d0, 0x4d42, 0xaf, 0xc0, 0x51, 0x21, 0xba, 0x42, 0x5c, 0x8d}
RIDL!{#[uuid(0x3d73a659, 0xe5d0, 0x4d42, 0xaf, 0xc0, 0x51, 0x21, 0xba, 0x42, 0x5c, 0x8d)]
interface ICDBurn(ICDBurnVtbl): IUnknown(IUnknownVtbl) {
    fn GetRecorderDriveLetter(
        pszDrive: LPWSTR,
        cch: UINT,
    ) -> HRESULT,
    fn Burn(
        hwnd: HWND,
    ) -> HRESULT,
    fn HasRecordableDrive(
        pfHasRecorder: *mut BOOL,
    ) -> HRESULT,
}}
pub const IDD_WIZEXTN_FIRST: DWORD = 0x5000;
pub const IDD_WIZEXTN_LAST: DWORD = 0x5100;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0019_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0019_v0_0_s_ifspec;
DEFINE_GUID!{IID_IWizardSite,
    0x88960f5b, 0x422f, 0x4e7b, 0x80, 0x13, 0x73, 0x41, 0x53, 0x81, 0xc3, 0xc3}
RIDL!{#[uuid(0x88960f5b, 0x422f, 0x4e7b, 0x80, 0x13, 0x73, 0x41, 0x53, 0x81, 0xc3, 0xc3)]
interface IWizardSite(IWizardSiteVtbl): IUnknown(IUnknownVtbl) {
    fn GetPreviousPage(
        phpage: *mut HPROPSHEETPAGE,
    ) -> HRESULT,
    fn GetNextPage(
        phpage: *mut HPROPSHEETPAGE,
    ) -> HRESULT,
    fn GetCancelledPage(
        phpage: *mut HPROPSHEETPAGE,
    ) -> HRESULT,
}}
pub use self::IID_IWizardSite as SID_WizardSite;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0020_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0020_v0_0_s_ifspec;
DEFINE_GUID!{IID_IWizardExtension,
    0xc02ea696, 0x86cc, 0x491e, 0x9b, 0x23, 0x74, 0x39, 0x4a, 0x04, 0x44, 0xa8}
RIDL!{#[uuid(0xc02ea696, 0x86cc, 0x491e, 0x9b, 0x23, 0x74, 0x39, 0x4a, 0x04, 0x44, 0xa8)]
interface IWizardExtension(IWizardExtensionVtbl): IUnknown(IUnknownVtbl) {
    fn AddPages(
        aPages: *mut HPROPSHEETPAGE,
        cPages: UINT,
        pnPagesAdded: *mut UINT,
    ) -> HRESULT,
    fn GetFirstPage(
        phpage: *mut HPROPSHEETPAGE,
    ) -> HRESULT,
    fn GetLastPage(
        phpage: *mut HPROPSHEETPAGE,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IWebWizardExtension,
    0x0e6b3f66, 0x98d1, 0x48c0, 0xa2, 0x22, 0xfb, 0xde, 0x74, 0xe2, 0xfb, 0xc5}
RIDL!{#[uuid(0x0e6b3f66, 0x98d1, 0x48c0, 0xa2, 0x22, 0xfb, 0xde, 0x74, 0xe2, 0xfb, 0xc5)]
interface IWebWizardExtension(IWebWizardExtensionVtbl): IWizardExtension(IWizardExtensionVtbl) {
    fn SetInitialURL(
        pszURL: LPCWSTR,
    ) -> HRESULT,
    fn SetErrorURL(
        pszErrorURL: LPCWSTR,
    ) -> HRESULT,
}}
pub use self::IID_IWebWizardExtension as SID_WebWizardHost;
pub const SHPWHF_NORECOMPRESS: DWORD = 0x00000001;
pub const SHPWHF_NONETPLACECREATE: DWORD = 0x00000002;
pub const SHPWHF_NOFILESELECTOR: DWORD = 0x00000004;
pub const SHPWHF_USEMRU: DWORD = 0x00000008;
pub const SHPWHF_ANYLOCATION: DWORD = 0x00000100;
pub const SHPWHF_VALIDATEVIAWEBFOLDERS: DWORD = 0x00010000;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0022_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0022_v0_0_s_ifspec;
DEFINE_GUID!{IID_IPublishingWizard,
    0xaa9198bb, 0xccec, 0x472d, 0xbe, 0xed, 0x19, 0xa4, 0xf6, 0x73, 0x3f, 0x7a}
RIDL!{#[uuid(0xaa9198bb, 0xccec, 0x472d, 0xbe, 0xed, 0x19, 0xa4, 0xf6, 0x73, 0x3f, 0x7a)]
interface IPublishingWizard(IPublishingWizardVtbl): IWizardExtension(IWizardExtensionVtbl) {
    fn Initialize(
        pdo: *mut IDataObject,
        dwOptions: DWORD,
        pszServiceScope: LPCWSTR,
    ) -> HRESULT,
    fn GetTransferManifest(
        phrFromTransfer: *mut HRESULT,
        pdocManifest: *mut *mut IXMLDOMDocument,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0023_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0023_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderViewHost,
    0x1ea58f02, 0xd55a, 0x411d, 0xb0, 0x9e, 0x9e, 0x65, 0xac, 0x21, 0x60, 0x5b}
RIDL!{#[uuid(0x1ea58f02, 0xd55a, 0x411d, 0xb0, 0x9e, 0x9e, 0x65, 0xac, 0x21, 0x60, 0x5b)]
interface IFolderViewHost(IFolderViewHostVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hwndParent: HWND,
        pdo: *mut IDataObject,
        prc: *mut RECT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0024_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0024_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAccessibleObject,
    0x95a391c5, 0x9ed4, 0x4c28, 0x84, 0x01, 0xab, 0x9e, 0x06, 0x71, 0x9e, 0x11}
RIDL!{#[uuid(0x95a391c5, 0x9ed4, 0x4c28, 0x84, 0x01, 0xab, 0x9e, 0x06, 0x71, 0x9e, 0x11)]
interface IAccessibleObject(IAccessibleObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetAccessibleName(
        pszName: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0025_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0025_v0_0_s_ifspec;
DEFINE_GUID!{IID_IResultsFolder,
    0x96e5ae6d, 0x6ae1, 0x4b1c, 0x90, 0x0c, 0xc6, 0x48, 0x0e, 0xaa, 0x88, 0x28}
RIDL!{#[uuid(0x96e5ae6d, 0x6ae1, 0x4b1c, 0x90, 0x0c, 0xc6, 0x48, 0x0e, 0xaa, 0x88, 0x28)]
interface IResultsFolder(IResultsFolderVtbl): IUnknown(IUnknownVtbl) {
    fn AddItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn AddIDList(
        pidl: PCIDLIST_ABSOLUTE,
        ppidlAdded: *mut PITEMID_CHILD,
    ) -> HRESULT,
    fn RemoveItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn RemoveIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn RemoveAll() -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IResultsFolder_RemoteAddIDList_Proxy(
//     IResultsFolder * This,
//     PCIDLIST_ABSOLUTE pidl,
//     PITEMID_CHILD *ppidlAdded);
// c_void __RPC_STUB IResultsFolder_RemoteAddIDList_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub const ACDD_VISIBLE: DWORD = 0x0001;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0026_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0026_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAutoCompleteDropDown,
    0x3cd141f4, 0x3c6a, 0x11d2, 0xbc, 0xaa, 0x00, 0xc0, 0x4f, 0xd9, 0x29, 0xdb}
RIDL!{#[uuid(0x3cd141f4, 0x3c6a, 0x11d2, 0xbc, 0xaa, 0x00, 0xc0, 0x4f, 0xd9, 0x29, 0xdb)]
interface IAutoCompleteDropDown(IAutoCompleteDropDownVtbl): IUnknown(IUnknownVtbl) {
    fn GetDropDownStatus(
        pdwFlags: *mut DWORD,
        ppwszString: *mut LPWSTR,
    ) -> HRESULT,
    fn ResetEnumerator() -> HRESULT,
}}
// #define PROPSTR_EXTENSIONCOMPLETIONSTATE L"ExtensionCompletionState"
ENUM!{enum tagCDBURNINGEXTENSIONRET {
    CDBE_RET_DEFAULT = 0,
    CDBE_RET_DONTRUNOTHEREXTS = 0x1,
    CDBE_RET_STOPWIZARD = 0x2,
}}
pub use self::IID_ICDBurnExt as SID_CDWizardHost;
ENUM!{enum _CDBE_ACTIONS {
    CDBE_TYPE_MUSIC = 0x1,
    CDBE_TYPE_DATA = 0x2,
    CDBE_TYPE_ALL = 0xffffffff,
}}
pub type CDBE_ACTIONS = DWORD;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0027_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0027_v0_0_s_ifspec;
DEFINE_GUID!{IID_ICDBurnExt,
    0x2271dcca, 0x74fc, 0x4414, 0x8f, 0xb7, 0xc5, 0x6b, 0x05, 0xac, 0xe2, 0xd7}
RIDL!{#[uuid(0x2271dcca, 0x74fc, 0x4414, 0x8f, 0xb7, 0xc5, 0x6b, 0x05, 0xac, 0xe2, 0xd7)]
interface ICDBurnExt(ICDBurnExtVtbl): IUnknown(IUnknownVtbl) {
    fn GetSupportedActionTypes(
        pdwActions: *mut CDBE_ACTIONS,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0028_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0028_v0_0_s_ifspec;
DEFINE_GUID!{IID_IEnumReadyCallback,
    0x61e00d45, 0x8fff, 0x4e60, 0x92, 0x4e, 0x65, 0x37, 0xb6, 0x16, 0x12, 0xdd}
RIDL!{#[uuid(0x61e00d45, 0x8fff, 0x4e60, 0x92, 0x4e, 0x65, 0x37, 0xb6, 0x16, 0x12, 0xdd)]
interface IEnumReadyCallback(IEnumReadyCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn EnumReady() -> HRESULT,
}}
DEFINE_GUID!{IID_IEnumerableView,
    0x8c8bf236, 0x1aec, 0x495f, 0x98, 0x94, 0x91, 0xd5, 0x7c, 0x3c, 0x68, 0x6f}
RIDL!{#[uuid(0x8c8bf236, 0x1aec, 0x495f, 0x98, 0x94, 0x91, 0xd5, 0x7c, 0x3c, 0x68, 0x6f)]
interface IEnumerableView(IEnumerableViewVtbl): IUnknown(IUnknownVtbl) {
    fn SetEnumReadyCallback(
        percb: *mut IEnumReadyCallback,
    ) -> HRESULT,
    fn CreateEnumIDListFromContents(
        pidlFolder: PCIDLIST_ABSOLUTE,
        dwEnumFlags: DWORD,
        ppEnumIDList: *mut *mut IEnumIDList,
    ) -> HRESULT,
}}
pub use self::IID_IEnumerableView as SID_EnumerableView;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0030_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0030_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInsertItem,
    0xd2b57227, 0x3d23, 0x4b95, 0x93, 0xc0, 0x49, 0x2b, 0xd4, 0x54, 0xc3, 0x56}
RIDL!{#[uuid(0xd2b57227, 0x3d23, 0x4b95, 0x93, 0xc0, 0x49, 0x2b, 0xd4, 0x54, 0xc3, 0x56)]
interface IInsertItem(IInsertItemVtbl): IUnknown(IUnknownVtbl) {
    fn InsertItem(
        pidl: PCUIDLIST_RELATIVE,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0031_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0031_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFolderBandPriv,
    0x47c01f95, 0xe185, 0x412c, 0xb5, 0xc5, 0x4f, 0x27, 0xdf, 0x96, 0x5a, 0xea}
RIDL!{#[uuid(0x47c01f95, 0xe185, 0x412c, 0xb5, 0xc5, 0x4f, 0x27, 0xdf, 0x96, 0x5a, 0xea)]
interface IFolderBandPriv(IFolderBandPrivVtbl): IUnknown(IUnknownVtbl) {
    fn SetCascade(
        fCascade: BOOL,
    ) -> HRESULT,
    fn SetAccelerators(
        fAccelerators: BOOL,
    ) -> HRESULT,
    fn SetNoIcons(
        fNoIcons: BOOL,
    ) -> HRESULT,
    fn SetNoText(
        fNoText: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IImageRecompress,
    0x505f1513, 0x6b3e, 0x4892, 0xa2, 0x72, 0x59, 0xf8, 0x88, 0x9a, 0x4d, 0x3e}
RIDL!{#[uuid(0x505f1513, 0x6b3e, 0x4892, 0xa2, 0x72, 0x59, 0xf8, 0x88, 0x9a, 0x4d, 0x3e)]
interface IImageRecompress(IImageRecompressVtbl): IUnknown(IUnknownVtbl) {
    fn RecompressImage(
        psi: *mut IShellItem,
        cx: c_int,
        cy: c_int,
        iQuality: c_int,
        pstg: *mut IStorage,
        ppstrmOut: *mut *mut IStream,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0033_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0033_v0_0_s_ifspec;
DEFINE_GUID!{IID_IFileDialogControlEvents,
    0x36116642, 0xd713, 0x4b97, 0x9b, 0x83, 0x74, 0x84, 0xa9, 0xd0, 0x04, 0x33}
RIDL!{#[uuid(0x36116642, 0xd713, 0x4b97, 0x9b, 0x83, 0x74, 0x84, 0xa9, 0xd0, 0x04, 0x33)]
interface IFileDialogControlEvents(IFileDialogControlEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnItemSelected(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn OnButtonClicked(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn OnCheckButtonToggled(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
        bChecked: BOOL,
    ) -> HRESULT,
    fn OnControlActivating(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IFileDialog2,
    0x61744fc7, 0x85b5, 0x4791, 0xa9, 0xb0, 0x27, 0x22, 0x76, 0x30, 0x9b, 0x13}
RIDL!{#[uuid(0x61744fc7, 0x85b5, 0x4791, 0xa9, 0xb0, 0x27, 0x22, 0x76, 0x30, 0x9b, 0x13)]
interface IFileDialog2(IFileDialog2Vtbl): IFileDialog(IFileDialogVtbl) {
    fn SetCancelButtonLabel(
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn SetNavigationRoot(
        psi: *mut IShellItem,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IApplicationAssociationRegistrationUI,
    0x1f76a169, 0xf994, 0x40ac, 0x8f, 0xc8, 0x09, 0x59, 0xe8, 0x87, 0x47, 0x10}
RIDL!{#[uuid(0x1f76a169, 0xf994, 0x40ac, 0x8f, 0xc8, 0x09, 0x59, 0xe8, 0x87, 0x47, 0x10)]
interface IApplicationAssociationRegistrationUI(IApplicationAssociationRegistrationUIVtbl): IUnknown(IUnknownVtbl) {
    fn LaunchAdvancedAssociationUI(
        pszAppRegistryName: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0036_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0036_v0_0_s_ifspec;
DEFINE_GUID!{IID_IShellRunDll,
    0xfce4bde0, 0x4b68, 0x4b80, 0x8e, 0x9c, 0x74, 0x26, 0x31, 0x5a, 0x73, 0x88}
RIDL!{#[uuid(0xfce4bde0, 0x4b68, 0x4b80, 0x8e, 0x9c, 0x74, 0x26, 0x31, 0x5a, 0x73, 0x88)]
interface IShellRunDll(IShellRunDllVtbl): IUnknown(IUnknownVtbl) {
    fn Run(
        pszArgs: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IPreviousVersionsInfo,
    0x76e54780, 0xad74, 0x48e3, 0xa6, 0x95, 0x3b, 0xa9, 0xa0, 0xaf, 0xf1, 0x0d}
RIDL!{#[uuid(0x76e54780, 0xad74, 0x48e3, 0xa6, 0x95, 0x3b, 0xa9, 0xa0, 0xaf, 0xf1, 0x0d)]
interface IPreviousVersionsInfo(IPreviousVersionsInfoVtbl): IUnknown(IUnknownVtbl) {
    fn AreSnapshotsAvailable(
        pszPath: LPCWSTR,
        fOkToBeSlow: BOOL,
        pfAvailable: *mut BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0038_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0038_v0_0_s_ifspec;
DEFINE_GUID!{IID_IUseToBrowseItem,
    0x05edda5c, 0x98a3, 0x4717, 0x8a, 0xdb, 0xc5, 0xe7, 0xda, 0x99, 0x1e, 0xb1}
RIDL!{#[uuid(0x05edda5c, 0x98a3, 0x4717, 0x8a, 0xdb, 0xc5, 0xe7, 0xda, 0x99, 0x1e, 0xb1)]
interface IUseToBrowseItem(IUseToBrowseItemVtbl): IRelatedItem(IRelatedItemVtbl) {
}}
DEFINE_GUID!{SID_SCommandBarState,
    0xB99EAA5C, 0x3850, 0x4400, 0xBC, 0x33, 0x2C, 0xE5, 0x34, 0x04, 0x8B, 0xF8}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0039_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0039_v0_0_s_ifspec;
ENUM!{enum NSTCSTYLE2 {
    NSTCS2_DEFAULT = 0,
    NSTCS2_INTERRUPTNOTIFICATIONS = 0x1,
    NSTCS2_SHOWNULLSPACEMENU = 0x2,
    NSTCS2_DISPLAYPADDING = 0x4,
    NSTCS2_DISPLAYPINNEDONLY = 0x8,
    NTSCS2_NOSINGLETONAUTOEXPAND = 0x10,
    NTSCS2_NEVERINSERTNONENUMERATED = 0x20,
}}
DEFINE_GUID!{IID_INameSpaceTreeControl2,
    0x7cc7aed8, 0x290e, 0x49bc, 0x89, 0x45, 0xc1, 0x40, 0x1c, 0xc9, 0x30, 0x6c}
RIDL!{#[uuid(0x7cc7aed8, 0x290e, 0x49bc, 0x89, 0x45, 0xc1, 0x40, 0x1c, 0xc9, 0x30, 0x6c)]
interface INameSpaceTreeControl2(INameSpaceTreeControl2Vtbl): INameSpaceTreeControl(INameSpaceTreeControlVtbl) {
    fn SetControlStyle(
        nstcsMask: NSTCSTYLE,
        nstcsStyle: NSTCSTYLE,
    ) -> HRESULT,
    fn GetControlStyle(
        nstcsMask: NSTCSTYLE,
        pnstcsStyle: *mut NSTCSTYLE,
    ) -> HRESULT,
    fn SetControlStyle2(
        nstcsMask: NSTCSTYLE2,
        nstcsStyle: NSTCSTYLE2,
    ) -> HRESULT,
    fn GetControlStyle2(
        nstcsMask: NSTCSTYLE2,
        pnstcsStyle: *mut NSTCSTYLE2,
    ) -> HRESULT,
}}
pub const NSTCS2_ALLMASK: NSTCSTYLE2 = (NSTCS2_INTERRUPTNOTIFICATIONS | NSTCS2_SHOWNULLSPACEMENU | NSTCS2_DISPLAYPADDING);
#[inline]
pub fn ISLBUTTON(x: NSTCECLICKTYPE) -> bool {
    NSTCECT_LBUTTON == (x & NSTCECT_BUTTON)
}
#[inline]
pub fn ISMBUTTON(x: NSTCECLICKTYPE) -> bool {
    NSTCECT_MBUTTON == (x & NSTCECT_BUTTON)
}
#[inline]
pub fn ISRBUTTON(x: NSTCECLICKTYPE) -> bool {
    NSTCECT_RBUTTON == (x & NSTCECT_BUTTON)
}
#[inline]
pub fn ISDBLCLICK(x: NSTCECLICKTYPE) -> bool {
    NSTCECT_DBLCLICK == (x & NSTCECT_DBLCLICK)
}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0040_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0040_v0_0_s_ifspec;
ENUM!{enum _NSTCEHITTEST {
    NSTCEHT_NOWHERE = 0x1,
    NSTCEHT_ONITEMICON = 0x2,
    NSTCEHT_ONITEMLABEL = 0x4,
    NSTCEHT_ONITEMINDENT = 0x8,
    NSTCEHT_ONITEMBUTTON = 0x10,
    NSTCEHT_ONITEMRIGHT = 0x20,
    NSTCEHT_ONITEMSTATEICON = 0x40,
    NSTCEHT_ONITEM = 0x46,
    NSTCEHT_ONITEMTABBUTTON = 0x1000,
}}
pub type NSTCEHITTEST = DWORD;
ENUM!{enum _NSTCECLICKTYPE {
    NSTCECT_LBUTTON = 0x1,
    NSTCECT_MBUTTON = 0x2,
    NSTCECT_RBUTTON = 0x3,
    NSTCECT_BUTTON = 0x3,
    NSTCECT_DBLCLICK = 0x4,
}}
pub type NSTCECLICKTYPE = DWORD;
DEFINE_GUID!{IID_INameSpaceTreeControlEvents,
    0x93d77985, 0xb3d8, 0x4484, 0x83, 0x18, 0x67, 0x2c, 0xdd, 0xa0, 0x02, 0xce}
RIDL!{#[uuid(0x93d77985, 0xb3d8, 0x4484, 0x83, 0x18, 0x67, 0x2c, 0xdd, 0xa0, 0x02, 0xce)]
interface INameSpaceTreeControlEvents(INameSpaceTreeControlEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnItemClick(
        psi: *mut IShellItem,
        nstceHitTest: NSTCEHITTEST,
        nstceClickType: NSTCECLICKTYPE,
    ) -> HRESULT,
    fn OnPropertyItemCommit(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnItemStateChanging(
        psi: *mut IShellItem,
        nstcisMask: NSTCITEMSTATE,
        nstcisState: NSTCITEMSTATE,
    ) -> HRESULT,
    fn OnItemStateChanged(
        psi: *mut IShellItem,
        nstcisMask: NSTCITEMSTATE,
        nstcisState: NSTCITEMSTATE,
    ) -> HRESULT,
    fn OnSelectionChanged(
        psiaSelection: *mut IShellItemArray,
    ) -> HRESULT,
    fn OnKeyboardInput(
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> HRESULT,
    fn OnBeforeExpand(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnAfterExpand(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnBeginLabelEdit(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnEndLabelEdit(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnGetToolTip(
        psi: *mut IShellItem,
        pszTip: LPWSTR,
        cchTip: c_int,
    ) -> HRESULT,
    fn OnBeforeItemDelete(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnItemAdded(
        psi: *mut IShellItem,
        fIsRoot: BOOL,
    ) -> HRESULT,
    fn OnItemDeleted(
        psi: *mut IShellItem,
        fIsRoot: BOOL,
    ) -> HRESULT,
    fn OnBeforeContextMenu(
        psi: *mut IShellItem,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn OnAfterContextMenu(
        psi: *mut IShellItem,
        pcmIn: *mut IContextMenu,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn OnBeforeStateImageChange(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnGetDefaultIconIndex(
        psi: *mut IShellItem,
        piDefaultIcon: *mut c_int,
        piOpenIcon: *mut c_int,
    ) -> HRESULT,
}}
pub const NSTCDHPOS_ONTOP: c_int = -1;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0041_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0041_v0_0_s_ifspec;
DEFINE_GUID!{IID_INameSpaceTreeControlDropHandler,
    0xf9c665d6, 0xc2f2, 0x4c19, 0xbf, 0x33, 0x83, 0x22, 0xd7, 0x35, 0x2f, 0x51}
RIDL!{#[uuid(0xf9c665d6, 0xc2f2, 0x4c19, 0xbf, 0x33, 0x83, 0x22, 0xd7, 0x35, 0x2f, 0x51)]
interface INameSpaceTreeControlDropHandler(INameSpaceTreeControlDropHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn OnDragEnter(
        psiOver: *mut IShellItem,
        psiaData: *mut IShellItemArray,
        fOutsideSource: BOOL,
        grfKeyState: DWORD,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn OnDragOver(
        psiOver: *mut IShellItem,
        psiaData: *mut IShellItemArray,
        grfKeyState: DWORD,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn OnDragPosition(
        psiOver: *mut IShellItem,
        psiaData: *mut IShellItemArray,
        iNewPosition: c_int,
        iOldPosition: c_int,
    ) -> HRESULT,
    fn OnDrop(
        psiOver: *mut IShellItem,
        psiaData: *mut IShellItemArray,
        iPosition: c_int,
        grfKeyState: DWORD,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn OnDropPosition(
        psiOver: *mut IShellItem,
        psiaData: *mut IShellItemArray,
        iNewPosition: c_int,
        iOldPosition: c_int,
    ) -> HRESULT,
    fn OnDragLeave(
        psiOver: *mut IShellItem,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_INameSpaceTreeAccessible,
    0x71f312de, 0x43ed, 0x4190, 0x84, 0x77, 0xe9, 0x53, 0x6b, 0x82, 0x35, 0x0b}
RIDL!{#[uuid(0x71f312de, 0x43ed, 0x4190, 0x84, 0x77, 0xe9, 0x53, 0x6b, 0x82, 0x35, 0x0b)]
interface INameSpaceTreeAccessible(INameSpaceTreeAccessibleVtbl): IUnknown(IUnknownVtbl) {
    fn OnGetDefaultAccessibilityAction(
        psi: *mut IShellItem,
        pbstrDefaultAction: *mut BSTR,
    ) -> HRESULT,
    fn OnDoDefaultAccessibilityAction(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn OnGetAccessibilityRole(
        psi: *mut IShellItem,
        pvarRole: *mut VARIANT,
    ) -> HRESULT,
}}
STRUCT!{struct NSTCCUSTOMDRAW {
    psi: *mut IShellItem,
    uItemState: UINT,
    nstcis: NSTCITEMSTATE,
    pszText: LPCWSTR,
    iImage: c_int,
    himl: HIMAGELIST,
    iLevel: c_int,
    iIndent: c_int,
}}
DEFINE_GUID!{IID_INameSpaceTreeControlCustomDraw,
    0x2d3ba758, 0x33ee, 0x42d5, 0xbb, 0x7b, 0x5f, 0x34, 0x31, 0xd8, 0x6c, 0x78}
RIDL!{#[uuid(0x2d3ba758, 0x33ee, 0x42d5, 0xbb, 0x7b, 0x5f, 0x34, 0x31, 0xd8, 0x6c, 0x78)]
interface INameSpaceTreeControlCustomDraw(INameSpaceTreeControlCustomDrawVtbl): IUnknown(IUnknownVtbl) {
    fn PrePaint(
        hdc: HDC,
        prc: *mut RECT,
        plres: *mut LRESULT,
    ) -> HRESULT,
    fn PostPaint(
        hdc: HDC,
        prc: *mut RECT,
    ) -> HRESULT,
    fn ItemPrePaint(
        hdc: HDC,
        prc: *mut RECT,
        pnstccdItem: *mut NSTCCUSTOMDRAW,
        pclrText: *mut COLORREF,
        pclrTextBk: *mut COLORREF,
        plres: *mut LRESULT,
    ) -> HRESULT,
    fn ItemPostPaint(
        hdc: HDC,
        prc: *mut RECT,
        pnstccdItem: *mut NSTCCUSTOMDRAW,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0044_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0044_v0_0_s_ifspec;
DEFINE_GUID!{IID_ITrayDeskBand,
    0x6d67e846, 0x5b9c, 0x4db8, 0x9c, 0xbc, 0xdd, 0xe1, 0x2f, 0x42, 0x54, 0xf1}
RIDL!{#[uuid(0x6d67e846, 0x5b9c, 0x4db8, 0x9c, 0xbc, 0xdd, 0xe1, 0x2f, 0x42, 0x54, 0xf1)]
interface ITrayDeskBand(ITrayDeskBandVtbl): IUnknown(IUnknownVtbl) {
    fn ShowDeskBand(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn HideDeskBand(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn IsDeskBandShown(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn DeskBandRegistrationChanged() -> HRESULT,
}}
DEFINE_GUID!{IID_IBandHost,
    0xb9075c7c, 0xd48e, 0x403f, 0xab, 0x99, 0xd6, 0xc7, 0x7a, 0x10, 0x84, 0xac}
RIDL!{#[uuid(0xb9075c7c, 0xd48e, 0x403f, 0xab, 0x99, 0xd6, 0xc7, 0x7a, 0x10, 0x84, 0xac)]
interface IBandHost(IBandHostVtbl): IUnknown(IUnknownVtbl) {
    fn CreateBand(
        rclsidBand: REFCLSID,
        fAvailable: BOOL,
        fVisible: BOOL,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn SetBandAvailability(
        rclsidBand: REFCLSID,
        fAvailable: BOOL,
    ) -> HRESULT,
    fn DestroyBand(
        rclsidBand: REFCLSID,
    ) -> HRESULT,
}}
pub use self::IID_IBandHost as SID_SBandHost;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0048_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0048_v0_0_s_ifspec;
DEFINE_GUID!{IID_IInitializeNetworkFolder,
    0x6e0f9881, 0x42a8, 0x4f2a, 0x97, 0xf8, 0x8a, 0xf4, 0xe0, 0x26, 0xd9, 0x2d}
RIDL!{#[uuid(0x6e0f9881, 0x42a8, 0x4f2a, 0x97, 0xf8, 0x8a, 0xf4, 0xe0, 0x26, 0xd9, 0x2d)]
interface IInitializeNetworkFolder(IInitializeNetworkFolderVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pidl: PCIDLIST_ABSOLUTE,
        pidlTarget: PCIDLIST_ABSOLUTE,
        uDisplayType: UINT,
        pszResName: LPCWSTR,
        pszProvider: LPCWSTR,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IComputerInfoChangeNotify,
    0x0df60d92, 0x6818, 0x46d6, 0xb3, 0x58, 0xd6, 0x61, 0x70, 0xdd, 0xe4, 0x66}
RIDL!{#[uuid(0x0df60d92, 0x6818, 0x46d6, 0xb3, 0x58, 0xd6, 0x61, 0x70, 0xdd, 0xe4, 0x66)]
interface IComputerInfoChangeNotify(IComputerInfoChangeNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn ComputerInfoChanged() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0050_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0050_v0_0_s_ifspec;
DEFINE_GUID!{IID_IDesktopGadget,
    0xc1646bc4, 0xf298, 0x4f91, 0xa2, 0x04, 0xeb, 0x2d, 0xd1, 0x70, 0x9d, 0x1a}
RIDL!{#[uuid(0xc1646bc4, 0xf298, 0x4f91, 0xa2, 0x04, 0xeb, 0x2d, 0xd1, 0x70, 0x9d, 0x1a)]
interface IDesktopGadget(IDesktopGadgetVtbl): IUnknown(IUnknownVtbl) {
    fn RunGadget(
        gadgetPath: LPCWSTR,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0051_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0051_v0_0_s_ifspec;
DEFINE_GUID!{IID_IVirtualDesktopManager,
    0xa5cd92ff, 0x29be, 0x454c, 0x8d, 0x04, 0xd8, 0x28, 0x79, 0xfb, 0x3f, 0x1b}
RIDL!{#[uuid(0xa5cd92ff, 0x29be, 0x454c, 0x8d, 0x04, 0xd8, 0x28, 0x79, 0xfb, 0x3f, 0x1b)]
interface IVirtualDesktopManager(IVirtualDesktopManagerVtbl): IUnknown(IUnknownVtbl) {
    fn IsWindowOnCurrentVirtualDesktop(
        topLevelWindow: HWND,
        onCurrentDesktop: *mut BOOL,
    ) -> HRESULT,
    fn GetWindowDesktopId(
        topLevelWindow: HWND,
        desktopId: *mut GUID,
    ) -> HRESULT,
    fn MoveWindowToDesktop(
        topLevelWindow: HWND,
        desktopId: REFGUID,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0052_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0052_v0_0_s_ifspec;
pub use self::CLSID_PublishingWizard as SID_PublishingWizard;
// EXTERN_C const IID LIBID_ShellObjects;
DEFINE_GUID!{CLSID_QueryCancelAutoPlay,
    0x331F1768, 0x05A9, 0x4ddd, 0xB8, 0x6E, 0xDA, 0xE3, 0x4D, 0xDC, 0x99, 0x8A}
// class DECLSPEC_UUID("331F1768-05A9-4ddd-B86E-DAE34DDC998A")
// QueryCancelAutoPlay;
DEFINE_GUID!{CLSID_TimeCategorizer,
    0x3bb4118f, 0xddfd, 0x4d30, 0xa3, 0x48, 0x9f, 0xb5, 0xd6, 0xbf, 0x1a, 0xfe}
// class DECLSPEC_UUID("3bb4118f-ddfd-4d30-a348-9fb5d6bf1afe")
// TimeCategorizer;
DEFINE_GUID!{CLSID_AlphabeticalCategorizer,
    0x3c2654c6, 0x7372, 0x4f6b, 0xb3, 0x10, 0x55, 0xd6, 0x12, 0x8f, 0x49, 0xd2}
// class DECLSPEC_UUID("3c2654c6-7372-4f6b-b310-55d6128f49d2")
// AlphabeticalCategorizer;
DEFINE_GUID!{CLSID_MergedCategorizer,
    0x8e827c11, 0x33e7, 0x4bc1, 0xb2, 0x42, 0x8c, 0xd9, 0xa1, 0xc2, 0xb3, 0x04}
// class DECLSPEC_UUID("8e827c11-33e7-4bc1-b242-8cd9a1c2b304")
// MergedCategorizer;
DEFINE_GUID!{CLSID_ImageProperties,
    0x7ab770c7, 0x0e23, 0x4d7a, 0x8a, 0xa2, 0x19, 0xbf, 0xad, 0x47, 0x98, 0x29}
// class DECLSPEC_UUID("7ab770c7-0e23-4d7a-8aa2-19bfad479829")
// ImageProperties;
DEFINE_GUID!{CLSID_CDBurn,
    0xfbeb8a05, 0xbeee, 0x4442, 0x80, 0x4e, 0x40, 0x9d, 0x6c, 0x45, 0x15, 0xe9}
// class DECLSPEC_UUID("fbeb8a05-beee-4442-804e-409d6c4515e9")
// CDBurn;
DEFINE_GUID!{CLSID_StartMenuPin,
    0xa2a9545d, 0xa0c2, 0x42b4, 0x97, 0x08, 0xa0, 0xb2, 0xba, 0xdd, 0x77, 0xc8}
// class DECLSPEC_UUID("a2a9545d-a0c2-42b4-9708-a0b2badd77c8")
// StartMenuPin;
DEFINE_GUID!{CLSID_WebWizardHost,
    0xc827f149, 0x55c1, 0x4d28, 0x93, 0x5e, 0x57, 0xe4, 0x7c, 0xae, 0xd9, 0x73}
// class DECLSPEC_UUID("c827f149-55c1-4d28-935e-57e47caed973")
// WebWizardHost;
DEFINE_GUID!{CLSID_PublishDropTarget,
    0xCC6EEFFB, 0x43F6, 0x46c5, 0x96, 0x19, 0x51, 0xD5, 0x71, 0x96, 0x7F, 0x7D}
// class DECLSPEC_UUID("CC6EEFFB-43F6-46c5-9619-51D571967F7D")
// PublishDropTarget;
DEFINE_GUID!{CLSID_PublishingWizard,
    0x6b33163c, 0x76a5, 0x4b6c, 0xbf, 0x21, 0x45, 0xde, 0x9c, 0xd5, 0x03, 0xa1}
// class DECLSPEC_UUID("6b33163c-76a5-4b6c-bf21-45de9cd503a1")
// PublishingWizard;
DEFINE_GUID!{CLSID_InternetPrintOrdering,
    0xadd36aa8, 0x751a, 0x4579, 0xa2, 0x66, 0xd6, 0x6f, 0x52, 0x02, 0xcc, 0xbb}
// class DECLSPEC_UUID("add36aa8-751a-4579-a266-d66f5202ccbb")
// InternetPrintOrdering;
DEFINE_GUID!{CLSID_FolderViewHost,
    0x20b1cb23, 0x6968, 0x4eb9, 0xb7, 0xd4, 0xa6, 0x6d, 0x00, 0xd0, 0x7c, 0xee}
// class DECLSPEC_UUID("20b1cb23-6968-4eb9-b7d4-a66d00d07cee")
// FolderViewHost;
DEFINE_GUID!{CLSID_ExplorerBrowser,
    0x71f96385, 0xddd6, 0x48d3, 0xa0, 0xc1, 0xae, 0x06, 0xe8, 0xb0, 0x55, 0xfb}
// class DECLSPEC_UUID("71f96385-ddd6-48d3-a0c1-ae06e8b055fb")
// ExplorerBrowser;
DEFINE_GUID!{CLSID_ImageRecompress,
    0x6e33091c, 0xd2f8, 0x4740, 0xb5, 0x5e, 0x2e, 0x11, 0xd1, 0x47, 0x7a, 0x2c}
// class DECLSPEC_UUID("6e33091c-d2f8-4740-b55e-2e11d1477a2c")
// ImageRecompress;
DEFINE_GUID!{CLSID_TrayBandSiteService,
    0xF60AD0A0, 0xE5E1, 0x45cb, 0xB5, 0x1A, 0xE1, 0x5B, 0x9F, 0x8B, 0x29, 0x34}
// class DECLSPEC_UUID("F60AD0A0-E5E1-45cb-B51A-E15B9F8B2934")
// TrayBandSiteService;
DEFINE_GUID!{CLSID_TrayDeskBand,
    0xE6442437, 0x6C68, 0x4f52, 0x94, 0xDD, 0x2C, 0xFE, 0xD2, 0x67, 0xEF, 0xB9}
// class DECLSPEC_UUID("E6442437-6C68-4f52-94DD-2CFED267EFB9")
// TrayDeskBand;
DEFINE_GUID!{CLSID_AttachmentServices,
    0x4125dd96, 0xe03a, 0x4103, 0x8f, 0x70, 0xe0, 0x59, 0x7d, 0x80, 0x3b, 0x9c}
// class DECLSPEC_UUID("4125dd96-e03a-4103-8f70-e0597d803b9c")
// AttachmentServices;
DEFINE_GUID!{CLSID_DocPropShellExtension,
    0x883373C3, 0xBF89, 0x11D1, 0xBE, 0x35, 0x08, 0x00, 0x36, 0xB1, 0x1A, 0x03}
// class DECLSPEC_UUID("883373C3-BF89-11D1-BE35-080036B11A03")
// DocPropShellExtension;
DEFINE_GUID!{CLSID_FSCopyHandler,
    0xD197380A, 0x0A79, 0x4dc8, 0xA0, 0x33, 0xED, 0x88, 0x2C, 0x2F, 0xA1, 0x4B}
// class DECLSPEC_UUID("D197380A-0A79-4dc8-A033-ED882C2FA14B")
// FSCopyHandler;
DEFINE_GUID!{CLSID_PreviousVersions,
    0x596AB062, 0xB4D2, 0x4215, 0x9F, 0x74, 0xE9, 0x10, 0x9B, 0x0A, 0x81, 0x53}
// class DECLSPEC_UUID("596AB062-B4D2-4215-9F74-E9109B0A8153")
// PreviousVersions;
DEFINE_GUID!{CLSID_NamespaceTreeControl,
    0xAE054212, 0x3535, 0x4430, 0x83, 0xED, 0xD5, 0x01, 0xAA, 0x66, 0x80, 0xE6}
// class DECLSPEC_UUID("AE054212-3535-4430-83ED-D501AA6680E6")
// NamespaceTreeControl;
DEFINE_GUID!{CLSID_IENamespaceTreeControl,
    0xACE52D03, 0xE5CD, 0x4b20, 0x82, 0xFF, 0xE7, 0x1B, 0x11, 0xBE, 0xAE, 0x1D}
// class DECLSPEC_UUID("ACE52D03-E5CD-4b20-82FF-E71B11BEAE1D")
// IENamespaceTreeControl;
DEFINE_GUID!{CLSID_ApplicationAssociationRegistrationUI,
    0x1968106d, 0xf3b5, 0x44cf, 0x89, 0x0e, 0x11, 0x6f, 0xcb, 0x9e, 0xce, 0xf1}
// class DECLSPEC_UUID("1968106d-f3b5-44cf-890e-116fcb9ecef1")
// ApplicationAssociationRegistrationUI;
DEFINE_GUID!{CLSID_DesktopGadget,
    0x924ccc1b, 0x6562, 0x4c85, 0x86, 0x57, 0xd1, 0x77, 0x92, 0x52, 0x22, 0xb6}
// class DECLSPEC_UUID("924ccc1b-6562-4c85-8657-d177925222b6")
// DesktopGadget;
DEFINE_GUID!{CLSID_AccessibilityDockingService,
    0x29CE1D46, 0xB481, 0x4AA0, 0xA0, 0x8A, 0xD3, 0xEB, 0xC8, 0xAC, 0xA4, 0x02}
// class DECLSPEC_UUID("29CE1D46-B481-4AA0-A08A-D3EBC8ACA402")
// AccessibilityDockingService;
DEFINE_GUID!{CLSID_ExecuteFolder,
    0x11dbb47c, 0xa525, 0x400b, 0x9e, 0x80, 0xa5, 0x46, 0x15, 0xa0, 0x90, 0xc0}
// class DECLSPEC_UUID("11dbb47c-a525-400b-9e80-a54615a090c0")
// ExecuteFolder;
DEFINE_GUID!{CLSID_VirtualDesktopManager,
    0xaa509086, 0x5ca9, 0x4c25, 0x8f, 0x95, 0x58, 0x9d, 0x3c, 0x07, 0xb4, 0x8a}
// class DECLSPEC_UUID("aa509086-5ca9-4c25-8f95-589d3c07b48a")
// VirtualDesktopManager;
pub unsafe fn SHResolveFolderPathInLibrary(
    plib: *mut IShellLibrary,
    pszFolderPath: PCWSTR,
    dwTimeout: DWORD,
    ppszResolvedPath: *mut PWSTR,
) -> HRESULT {
    *ppszResolvedPath = ptr::null_mut();
    let pidlFolder = SHSimpleIDListFromPath(pszFolderPath);
    if pidlFolder.is_null() { return E_INVALIDARG }
    let mut psiFolder: *mut IShellItem = ptr::null_mut();
    let mut hr = SHCreateItemFromIDList(pidlFolder, &IID_IShellItem as *const _,
        &mut psiFolder as *mut _ as *mut *mut c_void);
    if SUCCEEDED(hr) {
        let mut psiResolved: *mut IShellItem = ptr::null_mut();
        hr = (*plib).ResolveFolder(psiFolder, dwTimeout, &IID_IShellItem as *const _,
            &mut psiResolved as *mut _ as *mut *mut c_void);
        if SUCCEEDED(hr) {
            hr = (*psiResolved).GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING, ppszResolvedPath);
            (*psiFolder).Release();
        }
        (*psiFolder).Release();
    }
    CoTaskMemFree(pidlFolder as *mut c_void);
    hr
}
ENUM!{enum UNDOCK_REASON {
    UR_RESOLUTION_CHANGE = 0,
    UR_MONITOR_DISCONNECT = 1,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0053_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0053_v0_0_s_ifspec;
DEFINE_GUID!{IID_IAccessibilityDockingServiceCallback,
    0x157733fd, 0xa592, 0x42e5, 0xb5, 0x94, 0x24, 0x84, 0x68, 0xc5, 0xa8, 0x1b}
RIDL!{#[uuid(0x157733fd, 0xa592, 0x42e5, 0xb5, 0x94, 0x24, 0x84, 0x68, 0xc5, 0xa8, 0x1b)]
interface IAccessibilityDockingServiceCallback(IAccessibilityDockingServiceCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Undocked(
        undockReason: UNDOCK_REASON,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IAccessibilityDockingService,
    0x8849dc22, 0xcedf, 0x4c95, 0x99, 0x8d, 0x05, 0x14, 0x19, 0xdd, 0x3f, 0x76}
RIDL!{#[uuid(0x8849dc22, 0xcedf, 0x4c95, 0x99, 0x8d, 0x05, 0x14, 0x19, 0xdd, 0x3f, 0x76)]
interface IAccessibilityDockingService(IAccessibilityDockingServiceVtbl): IUnknown(IUnknownVtbl) {
    fn GetAvailableSize(
        hMonitor: HMONITOR,
        pcxFixed: *mut UINT,
        pcyMax: *mut UINT,
    ) -> HRESULT,
    fn DockWindow(
        hwnd: HWND,
        hMonitor: HMONITOR,
        cyRequested: UINT,
        pCallback: *mut IAccessibilityDockingServiceCallback,
    ) -> HRESULT,
    fn UndockWindow(
        hwnd: HWND,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0055_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_shobjidl_0000_0055_v0_0_s_ifspec;
// unsigned c_long __RPC_USER HBITMAP_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HBITMAP * );
// unsigned c_char * __RPC_USER HBITMAP_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HBITMAP * );
// unsigned c_char * __RPC_USER HBITMAP_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HBITMAP * );
// c_void __RPC_USER HBITMAP_UserFree( __RPC__in unsigned c_long *, __RPC__in HBITMAP * );
// unsigned c_long __RPC_USER HICON_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HICON * );
// unsigned c_char * __RPC_USER HICON_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HICON * );
// unsigned c_char * __RPC_USER HICON_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HICON * );
// c_void __RPC_USER HICON_UserFree( __RPC__in unsigned c_long *, __RPC__in HICON * );
// unsigned c_long __RPC_USER HMONITOR_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HMONITOR * );
// unsigned c_char * __RPC_USER HMONITOR_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HMONITOR * );
// unsigned c_char * __RPC_USER HMONITOR_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HMONITOR * );
// c_void __RPC_USER HMONITOR_UserFree( __RPC__in unsigned c_long *, __RPC__in HMONITOR * );
// unsigned c_long __RPC_USER HWND_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HWND * );
// unsigned c_char * __RPC_USER HWND_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HWND * );
// unsigned c_char * __RPC_USER HWND_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HWND * );
// c_void __RPC_USER HWND_UserFree( __RPC__in unsigned c_long *, __RPC__in HWND * );
// unsigned c_long __RPC_USER PCIDLIST_ABSOLUTE_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_char * __RPC_USER PCIDLIST_ABSOLUTE_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_char * __RPC_USER PCIDLIST_ABSOLUTE_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out PCIDLIST_ABSOLUTE * );
// c_void __RPC_USER PCIDLIST_ABSOLUTE_UserFree( __RPC__in unsigned c_long *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_long __RPC_USER PITEMID_CHILD_UserSize( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in PITEMID_CHILD * );
// unsigned c_char * __RPC_USER PITEMID_CHILD_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in PITEMID_CHILD * );
// unsigned c_char * __RPC_USER PITEMID_CHILD_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out PITEMID_CHILD * );
// c_void __RPC_USER PITEMID_CHILD_UserFree( __RPC__in unsigned c_long *, __RPC__in PITEMID_CHILD * );
// unsigned c_long __RPC_USER HBITMAP_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HBITMAP * );
// unsigned c_char * __RPC_USER HBITMAP_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HBITMAP * );
// unsigned c_char * __RPC_USER HBITMAP_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HBITMAP * );
// c_void __RPC_USER HBITMAP_UserFree64( __RPC__in unsigned c_long *, __RPC__in HBITMAP * );
// unsigned c_long __RPC_USER HICON_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HICON * );
// unsigned c_char * __RPC_USER HICON_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HICON * );
// unsigned c_char * __RPC_USER HICON_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HICON * );
// c_void __RPC_USER HICON_UserFree64( __RPC__in unsigned c_long *, __RPC__in HICON * );
// unsigned c_long __RPC_USER HMONITOR_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HMONITOR * );
// unsigned c_char * __RPC_USER HMONITOR_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HMONITOR * );
// unsigned c_char * __RPC_USER HMONITOR_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HMONITOR * );
// c_void __RPC_USER HMONITOR_UserFree64( __RPC__in unsigned c_long *, __RPC__in HMONITOR * );
// unsigned c_long __RPC_USER HWND_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in HWND * );
// unsigned c_char * __RPC_USER HWND_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in HWND * );
// unsigned c_char * __RPC_USER HWND_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out HWND * );
// c_void __RPC_USER HWND_UserFree64( __RPC__in unsigned c_long *, __RPC__in HWND * );
// unsigned c_long __RPC_USER PCIDLIST_ABSOLUTE_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_char * __RPC_USER PCIDLIST_ABSOLUTE_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_char * __RPC_USER PCIDLIST_ABSOLUTE_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out PCIDLIST_ABSOLUTE * );
// c_void __RPC_USER PCIDLIST_ABSOLUTE_UserFree64( __RPC__in unsigned c_long *, __RPC__in PCIDLIST_ABSOLUTE * );
// unsigned c_long __RPC_USER PITEMID_CHILD_UserSize64( __RPC__in unsigned c_long *, unsigned c_long, __RPC__in PITEMID_CHILD * );
// unsigned c_char * __RPC_USER PITEMID_CHILD_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned c_char *, __RPC__in PITEMID_CHILD * );
// unsigned c_char * __RPC_USER PITEMID_CHILD_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned c_char *, __RPC__out PITEMID_CHILD * );
// c_void __RPC_USER PITEMID_CHILD_UserFree64( __RPC__in unsigned c_long *, __RPC__in PITEMID_CHILD * );
// HRESULT STDMETHODCALLTYPE IResultsFolder_AddIDList_Proxy(
//     IResultsFolder * This,
//     PCIDLIST_ABSOLUTE pidl,
//     PITEMID_CHILD *ppidlAdded);
// HRESULT STDMETHODCALLTYPE IResultsFolder_AddIDList_Stub(
//     IResultsFolder * This,
//     PCIDLIST_ABSOLUTE pidl,
//     PITEMID_CHILD *ppidlAdded);
// }
