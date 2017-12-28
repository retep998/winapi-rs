// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::ULONG_PTR;
use shared::guiddef::{CLSID, REFCLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, HGLOBAL, LPVOID, UINT, ULONG, WORD};
use shared::windef::{HACCEL, HDC, HMENU, HWND, LPCRECT, LPCRECTL, LPRECT, LPSIZEL, POINTL, RECT, SIZE, SIZEL};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use um::objidl::{DVTARGETDEVICE, FORMATETC, IAdviseSink, IBindCtx, IDataObject, IEnumSTATDATA, IMoniker, LPDATAOBJECT, STGMEDIUM};
use um::objidlbase::IEnumUnknown;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::LOGPALETTE;
use um::winnt::{HRESULT, LONG, LPCWSTR};
use um::winuser::LPMSG;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0000_v0_0_s_ifspec;
pub type LPOLEADVISEHOLDER = *mut IOleAdviseHolder;
DEFINE_GUID!{IID_IOleAdviseHolder,
    0x00000111, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000111, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleAdviseHolder(IOleAdviseHolderVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        pAdvise: *mut IAdviseSink,
        pdwConnection: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwConnection: DWORD,
    ) -> HRESULT,
    fn EnumAdvise(
        ppenumAdvise: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
    fn SendOnRename(
        pmk: *mut IMoniker,
    ) -> HRESULT,
    fn SendOnSave() -> HRESULT,
    fn SendOnClose() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0001_v0_0_s_ifspec;
pub type LPOLECACHE = *mut IOleCache;
DEFINE_GUID!{IID_IOleCache,
    0x0000011e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000011e, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleCache(IOleCacheVtbl): IUnknown(IUnknownVtbl) {
    fn Cache(
        pformatetc: *mut FORMATETC,
        advf: DWORD,
        pdwConnection: *mut DWORD,
    ) -> HRESULT,
    fn Uncache(
        dwConnection: DWORD,
    ) -> HRESULT,
    fn EnumCache(
        ppenumSTATDATA: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
    fn InitCache(
        pDataObject: *mut IDataObject,
    ) -> HRESULT,
    fn SetData(
        pformatetc: *mut FORMATETC,
        pmedium: *mut STGMEDIUM,
        fRelease: BOOL,
    ) -> HRESULT,
}}
pub type LPOLECACHE2 = *mut IOleCache2;
pub const UPDFCACHE_NODATACACHE: DWORD = 0x00000001;
pub const UPDFCACHE_ONSAVECACHE: DWORD = 0x00000002;
pub const UPDFCACHE_ONSTOPCACHE: DWORD = 0x00000004;
pub const UPDFCACHE_NORMALCACHE: DWORD = 0x00000008;
pub const UPDFCACHE_IFBLANK: DWORD = 0x00000010;
pub const UPDFCACHE_ONLYIFBLANK: DWORD = 0x80000000;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: DWORD = UPDFCACHE_IFBLANK | UPDFCACHE_ONSAVECACHE;
pub const UPDFCACHE_ALL: DWORD = !UPDFCACHE_ONLYIFBLANK;
pub const UPDFCACHE_ALLBUTNODATACACHE: DWORD = UPDFCACHE_ALL & !UPDFCACHE_NODATACACHE;
ENUM!{enum DISCARDCACHE {
    DISCARDCACHE_SAVEIFDIRTY = 0,
    DISCARDCACHE_NOSAVE = 1,
}}
DEFINE_GUID!{IID_IOleCache2,
    0x00000128, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000128, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleCache2(IOleCache2Vtbl): IOleCache(IOleCacheVtbl) {
    fn UpdateCache(
        pDataObject: LPDATAOBJECT,
        grfUpdf: DWORD,
        pReserved: LPVOID,
    ) -> HRESULT,
    fn DiscardCache(
        dwDiscardOptions: DWORD,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IOleCache2_RemoteUpdateCache_Proxy(
//     IOleCache2 * This,
//     LPDATAOBJECT pDataObject,
//     DWORD grfUpdf,
//     LONG_PTR pReserved);
// void __RPC_STUB IOleCache2_RemoteUpdateCache_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0003_v0_0_s_ifspec;
pub type LPOLECACHECONTROL = *mut IOleCacheControl;
DEFINE_GUID!{IID_IOleCacheControl,
    0x00000129, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000129, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleCacheControl(IOleCacheControlVtbl): IUnknown(IUnknownVtbl) {
    fn OnRun(
        pDataObject: LPDATAOBJECT,
    ) -> HRESULT,
    fn OnStop() -> HRESULT,
}}
pub type LPPARSEDISPLAYNAME = *mut IParseDisplayName;
DEFINE_GUID!{IID_IParseDisplayName,
    0x0000011a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000011a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IParseDisplayName(IParseDisplayNameVtbl): IUnknown(IUnknownVtbl) {
    fn ParseDisplayName(
        pbc: *mut IBindCtx,
        pszDisplayName: LPOLESTR,
        pchEaten: *mut ULONG,
        ppmkOut: *mut *mut IMoniker,
    ) -> HRESULT,
}}
pub type LPOLECONTAINER = *mut IOleContainer;
DEFINE_GUID!{IID_IOleContainer,
    0x0000011b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000011b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleContainer(IOleContainerVtbl): IParseDisplayName(IParseDisplayNameVtbl) {
    fn EnumObjects(
        grfFlags: DWORD,
        ppenum: *mut *mut IEnumUnknown,
    ) -> HRESULT,
    fn LockContainer(
        fLock: BOOL,
    ) -> HRESULT,
}}
pub type LPOLECLIENTSITE = *mut IOleClientSite;
DEFINE_GUID!{IID_IOleClientSite,
    0x00000118, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000118, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleClientSite(IOleClientSiteVtbl): IUnknown(IUnknownVtbl) {
    fn SaveObject() -> HRESULT,
    fn GetMoniker(
        dwAssign: DWORD,
        dwWhichMoniker: DWORD,
        ppmk: *mut *mut IMoniker,
    ) -> HRESULT,
    fn GetContainer(
        ppContainer: *mut *mut IOleContainer,
    ) -> HRESULT,
    fn ShowObject() -> HRESULT,
    fn OnShowWindow(
        fShow: BOOL,
    ) -> HRESULT,
    fn RequestNewObjectLayout() -> HRESULT,
}}
pub type LPOLEOBJECT = *mut IOleObject;
ENUM!{enum OLEGETMONIKER {
    OLEGETMONIKER_ONLYIFTHERE = 1,
    OLEGETMONIKER_FORCEASSIGN = 2,
    OLEGETMONIKER_UNASSIGN = 3,
    OLEGETMONIKER_TEMPFORUSER = 4,
}}
ENUM!{enum OLEWHICHMK {
    OLEWHICHMK_CONTAINER = 1,
    OLEWHICHMK_OBJREL = 2,
    OLEWHICHMK_OBJFULL = 3,
}}
ENUM!{enum USERCLASSTYPE {
    USERCLASSTYPE_FULL = 1,
    USERCLASSTYPE_SHORT = 2,
    USERCLASSTYPE_APPNAME = 3,
}}
ENUM!{enum OLEMISC {
    OLEMISC_RECOMPOSEONRESIZE = 0x00000001,
    OLEMISC_ONLYICONIC = 0x00000002,
    OLEMISC_INSERTNOTREPLACE = 0x00000004,
    OLEMISC_STATIC = 0x00000008,
    OLEMISC_CANTLINKINSIDE = 0x00000010,
    OLEMISC_CANLINKBYOLE1 = 0x00000020,
    OLEMISC_ISLINKOBJECT = 0x00000040,
    OLEMISC_INSIDEOUT = 0x00000080,
    OLEMISC_ACTIVATEWHENVISIBLE = 0x00000100,
    OLEMISC_RENDERINGISDEVICEINDEPENDENT = 0x00000200,
    OLEMISC_INVISIBLEATRUNTIME = 0x00000400,
    OLEMISC_ALWAYSRUN = 0x00000800,
    OLEMISC_ACTSLIKEBUTTON = 0x00001000,
    OLEMISC_ACTSLIKELABEL = 0x00002000,
    OLEMISC_NOUIACTIVATE = 0x00004000,
    OLEMISC_ALIGNABLE = 0x00008000,
    OLEMISC_SIMPLEFRAME = 0x00010000,
    OLEMISC_SETCLIENTSITEFIRST = 0x00020000,
    OLEMISC_IMEMODE = 0x00040000,
    OLEMISC_IGNOREACTIVATEWHENVISIBLE = 0x00080000,
    OLEMISC_WANTSTOMENUMERGE = 0x00100000,
    OLEMISC_SUPPORTSMULTILEVELUNDO = 0x00200000,
}}
ENUM!{enum OLECLOSE {
    OLECLOSE_SAVEIFDIRTY = 0,
    OLECLOSE_NOSAVE = 1,
    OLECLOSE_PROMPTSAVE = 2,
}}
DEFINE_GUID!{IID_IOleObject,
    0x00000112, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000112, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleObject(IOleObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetClientSite(
        pClientSite: *mut IOleClientSite,
    ) -> HRESULT,
    fn GetClientSite(
        ppClientSite: *mut *mut IOleClientSite,
    ) -> HRESULT,
    fn SetHostNames(
        szContainerApp: LPCOLESTR,
        szContainerObj: LPCOLESTR,
    ) -> HRESULT,
    fn Close(
        dwSaveOption: DWORD,
    ) -> HRESULT,
    fn SetMoniker(
        dwWhichMoniker: DWORD,
        pmk: *mut IMoniker,
    ) -> HRESULT,
    fn GetMoniker(
        dwAssign: DWORD,
        dwWhichMoniker: DWORD,
        ppmk: *mut *mut IMoniker,
    ) -> HRESULT,
    fn InitFromData(
        pDataObject: *mut IDataObject,
        fCreation: BOOL,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn GetClipboardData(
        dwReserved: DWORD,
        ppDataObject: *mut *mut IDataObject,
    ) -> HRESULT,
    fn DoVerb(
        iVerb: LONG,
        lpmsg: LPMSG,
        pActiveSite: *mut IOleClientSite,
        lindex: LONG,
        hwndParent: HWND,
        lprcPosRect: LPCRECT,
    ) -> HRESULT,
    fn EnumVerbs(
        ppEnumOleVerb: *mut *mut IEnumOLEVERB,
    ) -> HRESULT,
    fn Update() -> HRESULT,
    fn IsUpToDate() -> HRESULT,
    fn GetUserClassID(
        pClsid: *mut CLSID,
    ) -> HRESULT,
    fn GetUserType(
        dwFormOfType: DWORD,
        pszUserType: *mut LPOLESTR,
    ) -> HRESULT,
    fn SetExtent(
        dwDrawAspect: DWORD,
        psizel: *mut SIZEL,
    ) -> HRESULT,
    fn GetExtent(
        dwDrawAspect: DWORD,
        psizel: *mut SIZEL,
    ) -> HRESULT,
    fn Advise(
        pAdvSink: *mut IAdviseSink,
        pdwConnection: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwConnection: DWORD,
    ) -> HRESULT,
    fn EnumAdvise(
        ppenumAdvise: *mut *mut IEnumSTATDATA,
    ) -> HRESULT,
    fn GetMiscStatus(
        dwAspect: DWORD,
        pdwStatus: *mut DWORD,
    ) -> HRESULT,
    fn SetColorScheme(
        pLogpal: *mut LOGPALETTE,
    ) -> HRESULT,
}}
ENUM!{enum OLERENDER {
    OLERENDER_NONE = 0,
    OLERENDER_DRAW = 1,
    OLERENDER_FORMAT = 2,
    OLERENDER_ASIS = 3,
}}
pub type LPOLERENDER = *mut OLERENDER;
STRUCT!{struct OBJECTDESCRIPTOR {
    cbSize: ULONG,
    clsid: CLSID,
    dwDrawAspect: DWORD,
    sizel: SIZEL,
    pointl: POINTL,
    dwStatus: DWORD,
    dwFullUserTypeName: DWORD,
    dwSrcOfCopy: DWORD,
}}
pub type POBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
pub type LPOBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
pub type LINKSRCDESCRIPTOR = OBJECTDESCRIPTOR;
pub type PLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
pub type LPLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
// extern RPC_IF_HANDLE IOLETypes_v0_0_c_ifspec;
// extern RPC_IF_HANDLE IOLETypes_v0_0_s_ifspec;
pub type LPOLEWINDOW = *mut IOleWindow;
DEFINE_GUID!{IID_IOleWindow,
    0x00000114, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000114, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleWindow(IOleWindowVtbl): IUnknown(IUnknownVtbl) {
    fn GetWindow(
        phwnd: *mut HWND,
    ) -> HRESULT,
    fn ContextSensitiveHelp(
        fEnterMode: BOOL,
    ) -> HRESULT,
}}
pub type LPOLELINK = *mut IOleLink;
ENUM!{enum OLEUPDATE {
    OLEUPDATE_ALWAYS = 1,
    OLEUPDATE_ONCALL = 3,
}}
pub type LPOLEUPDATE = *mut OLEUPDATE;
pub type POLEUPDATE = *mut OLEUPDATE;
ENUM!{enum OLELINKBIND {
    OLELINKBIND_EVENIFCLASSDIFF = 1,
}}
DEFINE_GUID!{IID_IOleLink,
    0x0000011d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000011d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleLink(IOleLinkVtbl): IUnknown(IUnknownVtbl) {
    fn SetUpdateOptions(
        dwUpdateOpt: DWORD,
    ) -> HRESULT,
    fn GetUpdateOptions(
        pdwUpdateOpt: *mut DWORD,
    ) -> HRESULT,
    fn SetSourceMoniker(
        pmk: *mut IMoniker,
        rclsid: REFCLSID,
    ) -> HRESULT,
    fn GetSourceMoniker(
        ppmk: *mut *mut IMoniker,
    ) -> HRESULT,
    fn SetSourceDisplayName(
        pszStatusText: LPCOLESTR,
    ) -> HRESULT,
    fn GetSourceDisplayName(
        ppszDisplayName: *mut LPOLESTR,
    ) -> HRESULT,
    fn BindToSource(
        bindflags: DWORD,
        pbc: *mut IBindCtx,
    ) -> HRESULT,
    fn BindIfRunning() -> HRESULT,
    fn GetBoundSource(
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn UnbindSource() -> HRESULT,
    fn Update(
        pbc: *mut IBindCtx,
    ) -> HRESULT,
}}
pub type LPOLEITEMCONTAINER = *mut IOleItemContainer;
ENUM!{enum BINDSPEED {
    BINDSPEED_INDEFINITE = 1,
    BINDSPEED_MODERATE = 2,
    BINDSPEED_IMMEDIATE = 3,
}}
ENUM!{enum OLECONTF {
    OLECONTF_EMBEDDINGS = 1,
    OLECONTF_LINKS = 2,
    OLECONTF_OTHERS = 4,
    OLECONTF_ONLYUSER = 8,
    OLECONTF_ONLYIFRUNNING = 16,
}}
DEFINE_GUID!{IID_IOleItemContainer,
    0x0000011c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000011c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleItemContainer(IOleItemContainerVtbl): IOleContainer(IOleContainerVtbl) {
    fn GetObject(
        pszItem: LPOLESTR,
        dwSpeedNeeded: DWORD,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn GetObjectStorage(
        pszItem: LPOLESTR,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppvStorage: *mut *mut c_void,
    ) -> HRESULT,
    fn IsRunning(
        pszItem: LPOLESTR,
    ) -> HRESULT,
}}
pub type LPOLEINPLACEUIWINDOW = *mut IOleInPlaceUIWindow;
pub type BORDERWIDTHS = RECT;
pub type LPBORDERWIDTHS = LPRECT;
pub type LPCBORDERWIDTHS = LPCRECT;
DEFINE_GUID!{IID_IOleInPlaceUIWindow,
    0x00000115, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000115, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleInPlaceUIWindow(IOleInPlaceUIWindowVtbl): IOleWindow(IOleWindowVtbl) {
    fn GetBorder(
        lprectBorder: LPRECT,
    ) -> HRESULT,
    fn RequestBorderSpace(
        pborderwidths: LPCBORDERWIDTHS,
    ) -> HRESULT,
    fn SetBorderSpace(
        pborderwidths: LPCBORDERWIDTHS,
    ) -> HRESULT,
    fn SetActiveObject(
        pActiveObject: *mut IOleInPlaceActiveObject,
        pszObjName: LPCOLESTR,
    ) -> HRESULT,
}}
pub type LPOLEINPLACEACTIVEOBJECT = *mut IOleInPlaceActiveObject;
DEFINE_GUID!{IID_IOleInPlaceActiveObject,
    0x00000117, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000117, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleInPlaceActiveObject(IOleInPlaceActiveObjectVtbl): IOleWindow(IOleWindowVtbl) {
    fn TranslateAccelerator(
        lpmsg: LPMSG,
    ) -> HRESULT,
    fn OnFrameWindowActivate(
        fActivate: BOOL,
    ) -> HRESULT,
    fn OnDocWindowActivate(
        fActivate: BOOL,
    ) -> HRESULT,
    fn ResizeBorder(
        prcBorder: LPCRECT,
        pUIWindow: *mut IOleInPlaceUIWindow,
        fFrameWindow: BOOL,
    ) -> HRESULT,
    fn EnableModeless(
        fEnable: BOOL,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_RemoteTranslateAccelerator_Proxy(
//     IOleInPlaceActiveObject * This);
// void __RPC_STUB IOleInPlaceActiveObject_RemoteTranslateAccelerator_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_RemoteResizeBorder_Proxy(
//     IOleInPlaceActiveObject * This,
//     LPCRECT prcBorder,
//     REFIID riid,
//     IOleInPlaceUIWindow *pUIWindow,
//     BOOL fFrameWindow);
// void __RPC_STUB IOleInPlaceActiveObject_RemoteResizeBorder_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPOLEINPLACEFRAME = *mut IOleInPlaceFrame;
STRUCT!{struct OLEINPLACEFRAMEINFO {
    cb: UINT,
    fMDIApp: BOOL,
    hwndFrame: HWND,
    haccel: HACCEL,
    cAccelEntries: UINT,
}}
pub type LPOLEINPLACEFRAMEINFO = *mut OLEINPLACEFRAMEINFO;
STRUCT!{struct OLEMENUGROUPWIDTHS {
    width: [LONG; 6],
}}
pub type LPOLEMENUGROUPWIDTHS = *mut OLEMENUGROUPWIDTHS;
pub type HOLEMENU = HGLOBAL;
DEFINE_GUID!{IID_IOleInPlaceFrame,
    0x00000116, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000116, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleInPlaceFrame(IOleInPlaceFrameVtbl): IOleInPlaceUIWindow(IOleInPlaceUIWindowVtbl) {
    fn InsertMenus(
        hmenuShared: HMENU,
        lpMenuWidths: LPOLEMENUGROUPWIDTHS,
    ) -> HRESULT,
    fn SetMenu(
        hmenuShared: HMENU,
        holemenu: HOLEMENU,
        hwndActiveObject: HWND,
    ) -> HRESULT,
    fn RemoveMenus(
        hmenuShared: HMENU,
    ) -> HRESULT,
    fn SetStatusText(
        pszStatusText: LPCOLESTR,
    ) -> HRESULT,
    fn EnableModeless(
        fEnable: BOOL,
    ) -> HRESULT,
    fn TranslateAccelerator(
        lpmsg: LPMSG,
        wID: WORD,
    ) -> HRESULT,
}}
pub type LPOLEINPLACEOBJECT = *mut IOleInPlaceObject;
DEFINE_GUID!{IID_IOleInPlaceObject,
    0x00000113, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000113, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleInPlaceObject(IOleInPlaceObjectVtbl): IOleWindow(IOleWindowVtbl) {
    fn InPlaceDeactivate() -> HRESULT,
    fn UIDeactivate() -> HRESULT,
    fn SetObjectRects(
        lprcPosRect: LPCRECT,
        lprcClipRect: LPCRECT,
    ) -> HRESULT,
    fn ReactivateAndUndo() -> HRESULT,
}}
pub type LPOLEINPLACESITE = *mut IOleInPlaceSite;
DEFINE_GUID!{IID_IOleInPlaceSite,
    0x00000119, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000119, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IOleInPlaceSite(IOleInPlaceSiteVtbl): IOleWindow(IOleWindowVtbl) {
    fn CanInPlaceActivate() -> HRESULT,
    fn OnInPlaceActivate() -> HRESULT,
    fn OnUIActivate() -> HRESULT,
    fn GetWindowContext(
        ppFrame: *mut *mut IOleInPlaceFrame,
        ppDoc: *mut *mut IOleInPlaceUIWindow,
        lprcPosRect: LPRECT,
        lprcClipRect: LPRECT,
        lpFrameInfo: LPOLEINPLACEFRAMEINFO,
    ) -> HRESULT,
    fn Scroll(
        scrollExtant: SIZE,
    ) -> HRESULT,
    fn OnUIDeactivate(
        fUndoable: BOOL,
    ) -> HRESULT,
    fn OnInPlaceDeactivate() -> HRESULT,
    fn DiscardUndoState() -> HRESULT,
    fn DeactivateAndUndo() -> HRESULT,
    fn OnPosRectChange(
        lprcPosRect: LPCRECT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IContinue,
    0x0000012a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000012a, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IContinue(IContinueVtbl): IUnknown(IUnknownVtbl) {
    fn FContinue() -> HRESULT,
}}
pub type LPVIEWOBJECT = *mut IViewObject;
DEFINE_GUID!{IID_IViewObject,
    0x0000010d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000010d, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IViewObject(IViewObjectVtbl): IUnknown(IUnknownVtbl) {
    fn Draw(
        dwDrawAspect: DWORD,
        lindex: LONG,
        pvAspect: *mut c_void,
        ptd: *mut DVTARGETDEVICE,
        hdcTargetDev: HDC,
        hdcDraw: HDC,
        lprcBounds: LPCRECTL,
        lprcWBounds: LPCRECTL,
        pfnContinue: fn(ULONG_PTR) -> BOOL,
        dwContinue: ULONG_PTR,
    ) -> HRESULT,
    fn GetColorSet(
        dwDrawAspect: DWORD,
        lindex: LONG,
        pvAspect: *mut c_void,
        ptd: *mut DVTARGETDEVICE,
        hicTargetDev: HDC,
        ppColorSet: *mut *mut LOGPALETTE,
    ) -> HRESULT,
    fn Freeze(
        dwDrawAspect: DWORD,
        lindex: LONG,
        pvAspect: *mut c_void,
        pdwFreeze: *mut DWORD,
    ) -> HRESULT,
    fn Unfreeze(
        dwFreeze: DWORD,
    ) -> HRESULT,
    fn SetAdvise(
        aspects: DWORD,
        advf: DWORD,
        pAdvSink: *mut IAdviseSink,
    ) -> HRESULT,
    fn GetAdvise(
        pAspects: *mut DWORD,
        pAdvf: *mut DWORD,
        ppAdvSink: *mut *mut IAdviseSink,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IViewObject_RemoteDraw_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DVTARGETDEVICE *ptd,
//     HDC hdcTargetDev,
//     HDC hdcDraw,
//     LPCRECTL lprcBounds,
//     LPCRECTL lprcWBounds,
//     IContinue *pContinue);
// void __RPC_STUB IViewObject_RemoteDraw_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IViewObject_RemoteGetColorSet_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DVTARGETDEVICE *ptd,
//     ULONG_PTR hicTargetDev,
//     LOGPALETTE **ppColorSet);
// void __RPC_STUB IViewObject_RemoteGetColorSet_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IViewObject_RemoteFreeze_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DWORD *pdwFreeze);
// void __RPC_STUB IViewObject_RemoteFreeze_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IViewObject_RemoteGetAdvise_Proxy(
//     IViewObject * This,
//     DWORD *pAspects,
//     DWORD *pAdvf,
//     IAdviseSink **ppAdvSink);
// void __RPC_STUB IViewObject_RemoteGetAdvise_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPVIEWOBJECT2 = *mut IViewObject2;
DEFINE_GUID!{IID_IViewObject2,
    0x00000127, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000127, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IViewObject2(IViewObject2Vtbl): IViewObject(IViewObjectVtbl) {
    fn GetExtent(
        dwDrawAspect: DWORD,
        lindex: LONG,
        ptd: *mut DVTARGETDEVICE,
        lpsizel: LPSIZEL,
    ) -> HRESULT,
}}
pub type LPDROPSOURCE = *mut IDropSource;
DEFINE_GUID!{IID_IDropSource,
    0x00000121, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000121, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDropSource(IDropSourceVtbl): IUnknown(IUnknownVtbl) {
    fn QueryContinueDrag(
        fEscapePressed: BOOL,
        grfKeyState: DWORD,
    ) -> HRESULT,
    fn GiveFeedback(
        dwEffect: DWORD,
    ) -> HRESULT,
}}
pub type LPDROPTARGET = *mut IDropTarget;
pub const MK_ALT: DWORD = 0x0020;
pub const DROPEFFECT_NONE: DWORD = 0;
pub const DROPEFFECT_COPY: DWORD = 1;
pub const DROPEFFECT_MOVE: DWORD = 2;
pub const DROPEFFECT_LINK: DWORD = 4;
pub const DROPEFFECT_SCROLL: DWORD = 0x80000000;
pub const DD_DEFSCROLLINSET: DWORD = 11;
pub const DD_DEFSCROLLDELAY: DWORD = 50;
pub const DD_DEFSCROLLINTERVAL: DWORD = 50;
pub const DD_DEFDRAGDELAY: DWORD = 200;
pub const DD_DEFDRAGMINDIST: DWORD = 2;
DEFINE_GUID!{IID_IDropTarget,
    0x00000122, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000122, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDropTarget(IDropTargetVtbl): IUnknown(IUnknownVtbl) {
    fn DragEnter(
        pDataObj: *mut IDataObject,
        grfKeyState: DWORD,
        pt: POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn DragOver(
        grfKeyState: DWORD,
        pt: POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn DragLeave() -> HRESULT,
    fn Drop(
        pDataObj: *mut IDataObject,
        grfKeyState: DWORD,
        pt: POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDropSourceNotify,
    0x0000012b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x0000012b, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDropSourceNotify(IDropSourceNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn DragEnterTarget(
        hwndTarget: HWND,
    ) -> HRESULT,
    fn DragLeaveTarget() -> HRESULT,
}}
DEFINE_GUID!{IID_IEnterpriseDropTarget,
    0x390e3878, 0xfd55, 0x4e18, 0x81, 0x9d, 0x46, 0x82, 0x08, 0x1c, 0x0c, 0xfd}
RIDL!{#[uuid(0x390e3878, 0xfd55, 0x4e18, 0x81, 0x9d, 0x46, 0x82, 0x08, 0x1c, 0x0c, 0xfd)]
interface IEnterpriseDropTarget(IEnterpriseDropTargetVtbl): IUnknown(IUnknownVtbl) {
    fn SetDropSourceEnterpriseId(
        identity: LPCWSTR,
    ) -> HRESULT,
    fn IsEvaluatingEdpPolicy(
        value: *mut BOOL,
    ) -> HRESULT,
}}
// #define CFSTR_ENTERPRISE_ID (TEXT(\"EnterpriseDataProtectionId\"))
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0024_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0024_v0_0_s_ifspec;
pub type LPENUMOLEVERB = *mut IEnumOLEVERB;
STRUCT!{struct OLEVERB {
    lVerb: LONG,
    lpszVerbName: LPOLESTR,
    fuFlags: DWORD,
    grfAttribs: DWORD,
}}
pub type LPOLEVERB = *mut OLEVERB;
ENUM!{enum OLEVERBATTRIB {
    OLEVERBATTRIB_NEVERDIRTIES = 1,
    OLEVERBATTRIB_ONCONTAINERMENU = 2,
}}
DEFINE_GUID!{IID_IEnumOLEVERB,
    0x00000104, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
RIDL!{#[uuid(0x00000104, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumOLEVERB(IEnumOLEVERBVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: LPOLEVERB,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumOLEVERB,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumOLEVERB_RemoteNext_Proxy(
//     IEnumOLEVERB * This,
//     ULONG celt,
//     LPOLEVERB rgelt,
//     ULONG *pceltFetched);
// void __RPC_STUB IEnumOLEVERB_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0025_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_oleidl_0000_0025_v0_0_s_ifspec;
// unsigned long             __RPC_USER  CLIPFORMAT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * );
// void                      __RPC_USER  CLIPFORMAT_UserFree(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * );
// unsigned long             __RPC_USER  HACCEL_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HACCEL * );
// unsigned char * __RPC_USER  HACCEL_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HACCEL * );
// unsigned char * __RPC_USER  HACCEL_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HACCEL * );
// void                      __RPC_USER  HACCEL_UserFree(     __RPC__in unsigned long *, __RPC__in HACCEL * );
// unsigned long             __RPC_USER  HDC_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * );
// void                      __RPC_USER  HDC_UserFree(     __RPC__in unsigned long *, __RPC__in HDC * );
// unsigned long             __RPC_USER  HGLOBAL_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER  HGLOBAL_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER  HGLOBAL_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HGLOBAL * );
// void                      __RPC_USER  HGLOBAL_UserFree(     __RPC__in unsigned long *, __RPC__in HGLOBAL * );
// unsigned long             __RPC_USER  HMENU_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * );
// unsigned char * __RPC_USER  HMENU_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * );
// unsigned char * __RPC_USER  HMENU_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * );
// void                      __RPC_USER  HMENU_UserFree(     __RPC__in unsigned long *, __RPC__in HMENU * );
// unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long             __RPC_USER  STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * );
// void                      __RPC_USER  STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * );
// unsigned long             __RPC_USER  CLIPFORMAT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * );
// unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * );
// void                      __RPC_USER  CLIPFORMAT_UserFree64(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * );
// unsigned long             __RPC_USER  HACCEL_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HACCEL * );
// unsigned char * __RPC_USER  HACCEL_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HACCEL * );
// unsigned char * __RPC_USER  HACCEL_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HACCEL * );
// void                      __RPC_USER  HACCEL_UserFree64(     __RPC__in unsigned long *, __RPC__in HACCEL * );
// unsigned long             __RPC_USER  HDC_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * );
// unsigned char * __RPC_USER  HDC_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * );
// void                      __RPC_USER  HDC_UserFree64(     __RPC__in unsigned long *, __RPC__in HDC * );
// unsigned long             __RPC_USER  HGLOBAL_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER  HGLOBAL_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HGLOBAL * );
// unsigned char * __RPC_USER  HGLOBAL_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HGLOBAL * );
// void                      __RPC_USER  HGLOBAL_UserFree64(     __RPC__in unsigned long *, __RPC__in HGLOBAL * );
// unsigned long             __RPC_USER  HMENU_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * );
// unsigned char * __RPC_USER  HMENU_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * );
// unsigned char * __RPC_USER  HMENU_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * );
// void                      __RPC_USER  HMENU_UserFree64(     __RPC__in unsigned long *, __RPC__in HMENU * );
// unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long             __RPC_USER  STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * );
// unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * );
// void                      __RPC_USER  STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * );
// HRESULT STDMETHODCALLTYPE IOleCache2_UpdateCache_Proxy(
//     IOleCache2 * This,
//     LPDATAOBJECT pDataObject,
//     DWORD grfUpdf,
//     LPVOID pReserved);
// HRESULT STDMETHODCALLTYPE IOleCache2_UpdateCache_Stub(
//     IOleCache2 * This,
//     LPDATAOBJECT pDataObject,
//     DWORD grfUpdf,
//     LONG_PTR pReserved);
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_TranslateAccelerator_Proxy(
//     IOleInPlaceActiveObject * This,
//     LPMSG lpmsg);
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_TranslateAccelerator_Stub(
//     IOleInPlaceActiveObject * This);
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_ResizeBorder_Proxy(
//     IOleInPlaceActiveObject * This,
//     LPCRECT prcBorder,
//     IOleInPlaceUIWindow *pUIWindow,
//     BOOL fFrameWindow);
// HRESULT STDMETHODCALLTYPE IOleInPlaceActiveObject_ResizeBorder_Stub(
//     IOleInPlaceActiveObject * This,
//     LPCRECT prcBorder,
//     REFIID riid,
//     IOleInPlaceUIWindow *pUIWindow,
//     BOOL fFrameWindow);
// HRESULT STDMETHODCALLTYPE IViewObject_Draw_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     void *pvAspect,
//     DVTARGETDEVICE *ptd,
//     HDC hdcTargetDev,
//     HDC hdcDraw,
//     LPCRECTL lprcBounds,
//     LPCRECTL lprcWBounds,
//     BOOL ( STDMETHODCALLTYPE *pfnContinue )(
//         ULONG_PTR dwContinue),
//     ULONG_PTR dwContinue);
// HRESULT STDMETHODCALLTYPE IViewObject_Draw_Stub(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DVTARGETDEVICE *ptd,
//     HDC hdcTargetDev,
//     HDC hdcDraw,
//     LPCRECTL lprcBounds,
//     LPCRECTL lprcWBounds,
//     IContinue *pContinue);
// HRESULT STDMETHODCALLTYPE IViewObject_GetColorSet_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     void *pvAspect,
//     DVTARGETDEVICE *ptd,
//     HDC hicTargetDev,
//     LOGPALETTE **ppColorSet);
// HRESULT STDMETHODCALLTYPE IViewObject_GetColorSet_Stub(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DVTARGETDEVICE *ptd,
//     ULONG_PTR hicTargetDev,
//     LOGPALETTE **ppColorSet);
// HRESULT STDMETHODCALLTYPE IViewObject_Freeze_Proxy(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     void *pvAspect,
//     DWORD *pdwFreeze);
// HRESULT STDMETHODCALLTYPE IViewObject_Freeze_Stub(
//     IViewObject * This,
//     DWORD dwDrawAspect,
//     LONG lindex,
//     ULONG_PTR pvAspect,
//     DWORD *pdwFreeze);
// HRESULT STDMETHODCALLTYPE IViewObject_GetAdvise_Proxy(
//     IViewObject * This,
//     DWORD *pAspects,
//     DWORD *pAdvf,
//     IAdviseSink **ppAdvSink);
// HRESULT STDMETHODCALLTYPE IViewObject_GetAdvise_Stub(
//     IViewObject * This,
//     DWORD *pAspects,
//     DWORD *pAdvf,
//     IAdviseSink **ppAdvSink);
// HRESULT STDMETHODCALLTYPE IEnumOLEVERB_Next_Proxy(
//     IEnumOLEVERB * This,
//     ULONG celt,
//     LPOLEVERB rgelt,
//     ULONG *pceltFetched);
// HRESULT STDMETHODCALLTYPE IEnumOLEVERB_Next_Stub(
//     IEnumOLEVERB * This,
//     ULONG celt,
//     LPOLEVERB rgelt,
//     ULONG *pceltFetched);
