// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::UINT_PTR;
use shared::guiddef::{CLSID, GUID, IID, REFIID};
use shared::minwindef::{
    BOOL, DWORD, FLOAT, HRGN, INT, LPARAM, LPVOID, LRESULT, UINT, ULONG, USHORT, WPARAM
};
use shared::windef::{
    HACCEL, HDC, HFONT, HPALETTE, HWND, LPCRECT, LPRECT, LPRECTL, LPSIZEL, POINT, POINTL, SIZE,
    SIZEL
};
use shared::wtypes::{BSTR, CLIPFORMAT, CY, VARTYPE};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR, SHORT};
use um::oaidl::{DISPID, IDispatch, IDispatchVtbl, IErrorLog, IPropertyBag, ITypeInfo, VARIANT};
use um::objidl::{DVTARGETDEVICE, IAdviseSink, IAdviseSinkVtbl, IPersist, IPersistVtbl};
use um::objidlbase::LPSTREAM;
use um::oleidl::{
    IDropTarget, IOleClientSite, IOleInPlaceObject, IOleInPlaceObjectVtbl, IOleInPlaceSite,
    IOleInPlaceSiteVtbl, IViewObject2, IViewObject2Vtbl
};
use um::servprov::IServiceProvider;
use um::unknwnbase::{IClassFactory, IClassFactoryVtbl, IUnknown, IUnknownVtbl};
use um::urlmon::IBindHost;
use um::wingdi::TEXTMETRICW;
use um::winnt::{HRESULT, LCID, LONG, PVOID, ULARGE_INTEGER};
use um::winuser::{MSG};
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0000_v0_0_s_ifspec;
ENUM!{enum UASFLAGS {
    UAS_NORMAL = 0x00,
    UAS_BLOCKED = 0x01,
    UAS_NOPARENTENABLE = 0x02,
    UAS_MASK = 0x03,
}}
ENUM!{enum READYSTATE {
    READYSTATE_UNINITIALIZED = 0,
    READYSTATE_LOADING = 1,
    READYSTATE_LOADED = 2,
    READYSTATE_INTERACTIVE = 3,
    READYSTATE_COMPLETE = 4,
}}
// extern RPC_IF_HANDLE IOleControlTypes_v1_0_c_ifspec;
// extern RPC_IF_HANDLE IOleControlTypes_v1_0_s_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0001_v0_0_s_ifspec;
pub type PENUMCONNECTIONS = *mut IEnumConnections;
pub type LPENUMCONNECTIONS = *mut IEnumConnections;
STRUCT!{struct CONNECTDATA {
    pUnk: *mut IUnknown,
    dwCookie: DWORD,
}}
pub type PCONNECTDATA = *mut CONNECTDATA;
pub type LPCONNECTDATA = *mut CONNECTDATA;
DEFINE_GUID!{IID_IEnumConnections,
    0xb196b287, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b287, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IEnumConnections(IEnumConnectionsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cConnections: ULONG,
        rgcd: LPCONNECTDATA,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cConnections: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumConnections,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumConnections_RemoteNext_Proxy(
//     IEnumConnections * This,
//     ULONG cConnections,
//     LPCONNECTDATA rgcd,
//     ULONG *pcFetched);
// void __RPC_STUB IEnumConnections_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type PCONNECTIONPOINT = *mut IConnectionPoint;
pub type LPCONNECTIONPOINT = *mut IConnectionPoint;
DEFINE_GUID!{IID_IConnectionPoint,
    0xb196b286, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b286, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IConnectionPoint(IConnectionPointVtbl): IUnknown(IUnknownVtbl) {
    fn GetConnectionInterface(
        pIID: *mut IID,
    ) -> HRESULT,
    fn GetConnectionPointContainer(
        ppCPC: *mut *mut IConnectionPointContainer,
    ) -> HRESULT,
    fn Advise(
        pUnkSink: *mut IUnknown,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn EnumConnections(
        ppEnum: *mut *mut IEnumConnections,
    ) -> HRESULT,
}}
pub type PENUMCONNECTIONPOINTS = *mut IEnumConnectionPoints;
pub type LPENUMCONNECTIONPOINTS = *mut IEnumConnectionPoints;
DEFINE_GUID!{IID_IEnumConnectionPoints,
    0xb196b285, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b285, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IEnumConnectionPoints(IEnumConnectionPointsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cConnections: ULONG,
        ppCP: *mut LPCONNECTIONPOINT,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cConnections: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumConnectionPoints,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumConnectionPoints_RemoteNext_Proxy(
//     IEnumConnectionPoints * This,
//     ULONG cConnections,
//     LPCONNECTIONPOINT *ppCP,
//     ULONG *pcFetched);
// void __RPC_STUB IEnumConnectionPoints_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type PCONNECTIONPOINTCONTAINER = *mut IConnectionPointContainer;
pub type LPCONNECTIONPOINTCONTAINER = *mut IConnectionPointContainer;
DEFINE_GUID!{IID_IConnectionPointContainer,
    0xb196b284, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b284, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IConnectionPointContainer(IConnectionPointContainerVtbl): IUnknown(IUnknownVtbl) {
    fn EnumConnectionPoints(
        ppEnum: *mut *mut IEnumConnectionPoints,
    ) -> HRESULT,
    fn FindConnectionPoint(
        riid: REFIID,
        ppCP: *mut *mut IConnectionPoint,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0005_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0005_v0_0_s_ifspec;
pub type LPCLASSFACTORY2 = *mut IClassFactory2;
STRUCT!{struct LICINFO {
    cbLicInfo: LONG,
    fRuntimeKeyAvail: BOOL,
    fLicVerified: BOOL,
}}
pub type LPLICINFO = *mut LICINFO;
DEFINE_GUID!{IID_IClassFactory2,
    0xb196b28f, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b28f, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IClassFactory2(IClassFactory2Vtbl): IClassFactory(IClassFactoryVtbl) {
    fn GetLicInfo(
        pLicInfo: *mut LICINFO,
    ) -> HRESULT,
    fn RequestLicKey(
        dwReserved: DWORD,
        pBstrKey: *mut BSTR,
    ) -> HRESULT,
    fn CreateInstanceLic(
        pUnkOuter: *mut IUnknown,
        pUnkReserved: *mut IUnknown,
        riid: REFIID,
        bstrKey: BSTR,
        ppvObj: *mut PVOID,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IClassFactory2_RemoteCreateInstanceLic_Proxy(
//     IClassFactory2 * This,
//     REFIID riid,
//     BSTR bstrKey,
//     IUnknown **ppvObj);
// void __RPC_STUB IClassFactory2_RemoteCreateInstanceLic_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPPROVIDECLASSINFO = *mut IProvideClassInfo;
DEFINE_GUID!{IID_IProvideClassInfo,
    0xb196b283, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b283, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IProvideClassInfo(IProvideClassInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassInfo(
        ppTI: *mut *mut ITypeInfo,
    ) -> HRESULT,
}}
pub type LPPROVIDECLASSINFO2 = *mut IProvideClassInfo2;
ENUM!{enum GUIDKIND {
    GUIDKIND_DEFAULT_SOURCE_DISP_IID = 1,
}}
DEFINE_GUID!{IID_IProvideClassInfo2,
    0xa6bc3ac0, 0xdbaa, 0x11ce, 0x9d, 0xe3, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51}
RIDL!{#[uuid(0xa6bc3ac0, 0xdbaa, 0x11ce, 0x9d, 0xe3, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51)]
interface IProvideClassInfo2(IProvideClassInfo2Vtbl): IProvideClassInfo(IProvideClassInfoVtbl) {
    fn GetGUID(
        dwGuidKind: DWORD,
        pGUID: *mut GUID,
    ) -> HRESULT,
}}
pub const MULTICLASSINFO_GETTYPEINFO: DWORD = 0x00000001;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: DWORD = 0x00000002;
pub const MULTICLASSINFO_GETIIDPRIMARY: DWORD = 0x00000004;
pub const MULTICLASSINFO_GETIIDSOURCE: DWORD = 0x00000008;
pub const TIFLAGS_EXTENDDISPATCHONLY: DWORD = 0x00000001;
pub type LPPROVIDEMULTIPLECLASSINFO = *mut IProvideMultipleClassInfo;
DEFINE_GUID!{IID_IProvideMultipleClassInfo,
    0xa7aba9c1, 0x8983, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64}
RIDL!{#[uuid(0xa7aba9c1, 0x8983, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64)]
interface IProvideMultipleClassInfo(IProvideMultipleClassInfoVtbl):
  IProvideClassInfo2(IProvideClassInfo2Vtbl)
{
    fn GetMultiTypeInfoCount(
        pcti: *mut ULONG,
    ) -> HRESULT,
    fn GetInfoOfIndex(
        iti: ULONG,
        dwFlags: DWORD,
        pptiCoClass: *mut *mut ITypeInfo,
        pdwTIFlags: *mut DWORD,
        pcdispidReserved: *mut ULONG,
        piidPrimary: *mut IID,
        piidSource: *mut IID,
    ) -> HRESULT,
}}
pub type LPOLECONTROL = *mut IOleControl;
STRUCT!{struct CONTROLINFO {
    cb: ULONG,
    hAccel: HACCEL,
    cAccel: USHORT,
    dwFlags: DWORD,
}}
pub type LPCONTROLINFO = *mut CONTROLINFO;
ENUM!{enum CTRLINFO {
    CTRLINFO_EATS_RETURN = 1,
    CTRLINFO_EATS_ESCAPE = 2,
}}
DEFINE_GUID!{IID_IOleControl,
    0xb196b288, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b288, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IOleControl(IOleControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetControlInfo(
        pCI: *mut CONTROLINFO,
    ) -> HRESULT,
    fn OnMnemonic(
        pMsg: *mut MSG,
    ) -> HRESULT,
    fn OnAmbientPropertyChange(
        dispID: DISPID,
    ) -> HRESULT,
    fn FreezeEvents(
        bFreeze: BOOL,
    ) -> HRESULT,
}}
pub type LPOLECONTROLSITE = *mut IOleControlSite;
STRUCT!{struct POINTF {
    x: FLOAT,
    y: FLOAT,
}}
pub type LPPOINTF = *mut POINTF;
ENUM!{enum XFORMCOORDS {
    XFORMCOORDS_POSITION = 0x1,
    XFORMCOORDS_SIZE = 0x2,
    XFORMCOORDS_HIMETRICTOCONTAINER = 0x4,
    XFORMCOORDS_CONTAINERTOHIMETRIC = 0x8,
    XFORMCOORDS_EVENTCOMPAT = 0x10,
}}
DEFINE_GUID!{IID_IOleControlSite,
    0xb196b289, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b289, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IOleControlSite(IOleControlSiteVtbl): IUnknown(IUnknownVtbl) {
    fn OnControlInfoChanged() -> HRESULT,
    fn LockInPlaceActive(
        fLock: BOOL,
    ) -> HRESULT,
    fn GetExtendedControl(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn TransformCoords(
        pPtlHimetric: *mut POINTL,
        pPtfContainer: *mut POINTF,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn TranslateAccelerator(
        pMsg: *mut MSG,
        grfModifiers: DWORD,
    ) -> HRESULT,
    fn OnFocus(
        fGotFocus: BOOL,
    ) -> HRESULT,
    fn ShowPropertyFrame() -> HRESULT,
}}
pub type LPPROPERTYPAGE = *mut IPropertyPage;
STRUCT!{struct PROPPAGEINFO {
    cb: ULONG,
    pszTitle: LPOLESTR,
    size: SIZE,
    pszDocString: LPOLESTR,
    pszHelpFile: LPOLESTR,
    dwHelpContext: DWORD,
}}
pub type LPPROPPAGEINFO = *mut PROPPAGEINFO;
DEFINE_GUID!{IID_IPropertyPage,
    0xb196b28d, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b28d, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IPropertyPage(IPropertyPageVtbl): IUnknown(IUnknownVtbl) {
    fn SetPageSite(
        pPageSite: *mut IPropertyPageSite,
    ) -> HRESULT,
    fn Activate(
        hWndParent: HWND,
        pRect: LPCRECT,
        bModal: BOOL,
    ) -> HRESULT,
    fn Deactivate() -> HRESULT,
    fn GetPageInfo(
        pPageInfo: *mut PROPPAGEINFO,
    ) -> HRESULT,
    fn SetObjects(
        cObjects: ULONG,
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Show(
        nCmdShow: UINT,
    ) -> HRESULT,
    fn Move(
        pRect: LPCRECT,
    ) -> HRESULT,
    fn IsPageDirty() -> HRESULT,
    fn Apply() -> HRESULT,
    fn Help(
        pszHelpDir: LPCOLESTR,
    ) -> HRESULT,
    fn TranslateAccelerator(
        pMsg: *mut MSG,
    ) -> HRESULT,
}}
pub type LPPROPERTYPAGE2 = *mut IPropertyPage2;
DEFINE_GUID!{IID_IPropertyPage2,
    0x01e44665, 0x24ac, 0x101b, 0x84, 0xed, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13}
RIDL!{#[uuid(0x01e44665, 0x24ac, 0x101b, 0x84, 0xed, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13)]
interface IPropertyPage2(IPropertyPage2Vtbl): IPropertyPage(IPropertyPageVtbl) {
    fn EditProperty(
        dispID: DISPID,
    ) -> HRESULT,
}}
pub type LPPROPERTYPAGESITE = *mut IPropertyPageSite;
ENUM!{enum PROPPAGESTATUS {
    PROPPAGESTATUS_DIRTY = 0x01,
    PROPPAGESTATUS_VALIDATE = 0x02,
    PROPPAGESTATUS_CLEAN = 0x04,
}}
DEFINE_GUID!{IID_IPropertyPageSite,
    0xb196b28c, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b28c, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IPropertyPageSite(IPropertyPageSiteVtbl): IUnknown(IUnknownVtbl) {
    fn OnStatusChange(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetLocaleID(
        pLocaleID: *mut LCID,
    ) -> HRESULT,
    fn GetPageContainer(
        ppUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn TranslateAccelerator(
        pMsg: *mut MSG,
    ) -> HRESULT,
}}
pub type LPPROPERTYNOTIFYSINK = *mut IPropertyNotifySink;
DEFINE_GUID!{IID_IPropertyNotifySink,
    0x9bfbbc02, 0xeff1, 0x101a, 0x84, 0xed, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0x9bfbbc02, 0xeff1, 0x101a, 0x84, 0xed, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface IPropertyNotifySink(IPropertyNotifySinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnChanged(
        dispID: DISPID,
    ) -> HRESULT,
    fn OnRequestEdit(
        dispID: DISPID,
    ) -> HRESULT,
}}
pub type LPSPECIFYPROPERTYPAGES = *mut ISpecifyPropertyPages;
STRUCT!{struct CAUUID {
    cElems: ULONG,
    pElems: *mut GUID,
}}
pub type LPCAUUID = *mut CAUUID;
DEFINE_GUID!{IID_ISpecifyPropertyPages,
    0xb196b28b, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07}
RIDL!{#[uuid(0xb196b28b, 0xbab4, 0x101a, 0xb6, 0x9c, 0x00, 0xaa, 0x00, 0x34, 0x1d, 0x07)]
interface ISpecifyPropertyPages(ISpecifyPropertyPagesVtbl): IUnknown(IUnknownVtbl) {
    fn GetPages(
        pPages: *mut CAUUID,
    ) -> HRESULT,
}}
pub type LPPERSISTMEMORY = *mut IPersistMemory;
DEFINE_GUID!{IID_IPersistMemory,
    0xbd1ae5e0, 0xa6ae, 0x11ce, 0xbd, 0x37, 0x50, 0x42, 0x00, 0xc1, 0x00, 0x00}
RIDL!{#[uuid(0xbd1ae5e0, 0xa6ae, 0x11ce, 0xbd, 0x37, 0x50, 0x42, 0x00, 0xc1, 0x00, 0x00)]
interface IPersistMemory(IPersistMemoryVtbl): IPersist(IPersistVtbl) {
    fn IsDirty() -> HRESULT,
    fn Load(
        pMem: LPVOID,
        cbSize: ULONG,
    ) -> HRESULT,
    fn Save(
        pMem: LPVOID,
        fClearDirty: BOOL,
        cbSize: ULONG,
    ) -> HRESULT,
    fn GetSizeMax(
        pCbSize: *mut ULONG,
    ) -> HRESULT,
    fn InitNew() -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IPersistMemory_RemoteLoad_Proxy(
//     IPersistMemory * This,
//     BYTE *pMem,
//     ULONG cbSize);
// void __RPC_STUB IPersistMemory_RemoteLoad_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// HRESULT STDMETHODCALLTYPE IPersistMemory_RemoteSave_Proxy(
//     IPersistMemory * This,
//     BYTE *pMem,
//     BOOL fClearDirty,
//     ULONG cbSize);
// void __RPC_STUB IPersistMemory_RemoteSave_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPPERSISTSTREAMINIT = *mut IPersistStreamInit;
DEFINE_GUID!{IID_IPersistStreamInit,
    0x7fd52380, 0x4e07, 0x101b, 0xae, 0x2d, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13}
RIDL!{#[uuid(0x7fd52380, 0x4e07, 0x101b, 0xae, 0x2d, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13)]
interface IPersistStreamInit(IPersistStreamInitVtbl): IPersist(IPersistVtbl) {
    fn IsDirty() -> HRESULT,
    fn Load(
        pStm: LPSTREAM,
    ) -> HRESULT,
    fn Save(
        pStm: LPSTREAM,
        fClearDirty: BOOL,
    ) -> HRESULT,
    fn GetSizeMax(
        pCbSize: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn InitNew() -> HRESULT,
}}
pub type LPPERSISTPROPERTYBAG = *mut IPersistPropertyBag;
DEFINE_GUID!{IID_IPersistPropertyBag,
    0x37d84f60, 0x42cb, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51}
RIDL!{#[uuid(0x37d84f60, 0x42cb, 0x11ce, 0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51)]
interface IPersistPropertyBag(IPersistPropertyBagVtbl): IPersist(IPersistVtbl) {
    fn InitNew() -> HRESULT,
    fn Load(
        pPropBag: *mut IPropertyBag,
        pErrorLog: *mut IErrorLog,
    ) -> HRESULT,
    fn Save(
        pPropBag: *mut IPropertyBag,
        fClearDirty: BOOL,
        fSaveAllProperties: BOOL,
    ) -> HRESULT,
}}
pub type LPSIMPLEFRAMESITE = *mut ISimpleFrameSite;
DEFINE_GUID!{IID_ISimpleFrameSite,
    0x742b0e01, 0x14e6, 0x101b, 0x91, 0x4e, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab}
RIDL!{#[uuid(0x742b0e01, 0x14e6, 0x101b, 0x91, 0x4e, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab)]
interface ISimpleFrameSite(ISimpleFrameSiteVtbl): IUnknown(IUnknownVtbl) {
    fn PreMessageFilter(
        hWnd: HWND,
        msg: UINT,
        wp: WPARAM,
        lp: LPARAM,
        plResult: *mut LRESULT,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn PostMessageFilter(
        hWnd: HWND,
        msg: UINT,
        wp: WPARAM,
        lp: LPARAM,
        plResult: *mut LRESULT,
        dwCookie: DWORD,
    ) -> HRESULT,
}}
pub type LPFONT = *mut IFont;
pub type TEXTMETRICOLE = TEXTMETRICW;
pub type LPTEXTMETRICOLE = *mut TEXTMETRICOLE;
DEFINE_GUID!{IID_IFont,
    0xbef6e002, 0xa874, 0x101a, 0x8b, 0xba, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab}
RIDL!{#[uuid(0xbef6e002, 0xa874, 0x101a, 0x8b, 0xba, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab)]
interface IFont(IFontVtbl): IUnknown(IUnknownVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        name: BSTR,
    ) -> HRESULT,
    fn get_Size(
        pSize: *mut CY,
    ) -> HRESULT,
    fn put_Size(
        size: CY,
    ) -> HRESULT,
    fn get_Bold(
        pBold: *mut BOOL,
    ) -> HRESULT,
    fn put_Bold(
        bold: BOOL,
    ) -> HRESULT,
    fn get_Italic(
        pItalic: *mut BOOL,
    ) -> HRESULT,
    fn put_Italic(
        italic: BOOL,
    ) -> HRESULT,
    fn get_Underline(
        pUnderline: *mut BOOL,
    ) -> HRESULT,
    fn put_Underline(
        underline: BOOL,
    ) -> HRESULT,
    fn get_Strikethrough(
        pStrikethrough: *mut BOOL,
    ) -> HRESULT,
    fn put_Strikethrough(
        strikethrough: BOOL,
    ) -> HRESULT,
    fn get_Weight(
        pWeight: *mut SHORT,
    ) -> HRESULT,
    fn put_Weight(
        weight: SHORT,
    ) -> HRESULT,
    fn get_Charset(
        pCharset: *mut SHORT,
    ) -> HRESULT,
    fn put_Charset(
        charset: SHORT,
    ) -> HRESULT,
    fn get_hFont(
        phFont: *mut HFONT,
    ) -> HRESULT,
    fn Clone(
        ppFont: *mut *mut IFont,
    ) -> HRESULT,
    fn IsEqual(
        pFontOther: *mut IFont,
    ) -> HRESULT,
    fn SetRatio(
        cyLogical: LONG,
        cyHimetric: LONG,
    ) -> HRESULT,
    fn QueryTextMetrics(
        pTM: *mut TEXTMETRICOLE,
    ) -> HRESULT,
    fn AddRefHfont(
        hFont: HFONT,
    ) -> HRESULT,
    fn ReleaseHfont(
        hFont: HFONT,
    ) -> HRESULT,
    fn SetHdc(
        hDC: HDC,
    ) -> HRESULT,
}}
pub type LPPICTURE = *mut IPicture;
ENUM!{enum PICTUREATTRIBUTES {
    PICTURE_SCALABLE = 0x1,
    PICTURE_TRANSPARENT = 0x2,
}}
// [uuid(66504313-BE0F-101A-8BBB-00AA00300CAB), public]
pub type OLE_HANDLE = UINT;
// [uuid(66504306-BE0F-101A-8BBB-00AA00300CAB), hidden]
pub type OLE_XPOS_HIMETRIC = LONG;
// [uuid(66504307-BE0F-101A-8BBB-00AA00300CAB), hidden]
pub type OLE_YPOS_HIMETRIC = LONG;
// [uuid(66504308-BE0F-101A-8BBB-00AA00300CAB), hidden]
pub type OLE_XSIZE_HIMETRIC = LONG;
// [uuid(66504309-BE0F-101A-8BBB-00AA00300CAB), hidden]
pub type OLE_YSIZE_HIMETRIC = LONG;
DEFINE_GUID!{IID_IPicture,
    0x7bf80980, 0xbf32, 0x101a, 0x8b, 0xbb, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab}
RIDL!{#[uuid(0x7bf80980, 0xbf32, 0x101a, 0x8b, 0xbb, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab)]
interface IPicture(IPictureVtbl): IUnknown(IUnknownVtbl) {
    fn get_Handle(
        pHandle: *mut OLE_HANDLE,
    ) -> HRESULT,
    fn get_hPal(
        phPal: *mut OLE_HANDLE,
    ) -> HRESULT,
    fn get_Type(
        pType: *mut SHORT,
    ) -> HRESULT,
    fn get_Width(
        pWidth: *mut OLE_XSIZE_HIMETRIC,
    ) -> HRESULT,
    fn get_Height(
        pHeight: *mut OLE_YSIZE_HIMETRIC,
    ) -> HRESULT,
    fn Render(
        hDC: HDC,
        x: LONG,
        y: LONG,
        cx: LONG,
        cy: LONG,
        xSrc: OLE_XPOS_HIMETRIC,
        ySrc: OLE_YPOS_HIMETRIC,
        cxSrc: OLE_XSIZE_HIMETRIC,
        cySrc: OLE_YSIZE_HIMETRIC,
        pRcWBounds: LPCRECT,
    ) -> HRESULT,
    fn set_hPal(
        hPal: OLE_HANDLE,
    ) -> HRESULT,
    fn get_CurDC(
        phDC: *mut HDC,
    ) -> HRESULT,
    fn SelectPicture(
        hDCIn: HDC,
        phDCOut: *mut HDC,
        phBmpOut: *mut OLE_HANDLE,
    ) -> HRESULT,
    fn get_KeepOriginalFormat(
        pKeep: *mut BOOL,
    ) -> HRESULT,
    fn put_KeepOriginalFormat(
        keep: BOOL,
    ) -> HRESULT,
    fn PictureChanged() -> HRESULT,
    fn SaveAsFile(
        pStream: LPSTREAM,
        fSaveMemCopy: BOOL,
        pCbSize: *mut LONG,
    ) -> HRESULT,
    fn get_Attributes(
        pDwAttr: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPPICTURE2 = *mut IPicture2;
pub type HHANDLE = UINT_PTR;
DEFINE_GUID!{IID_IPicture2,
    0xf5185dd8, 0x2012, 0x4b0b, 0xaa, 0xd9, 0xf0, 0x52, 0xc6, 0xbd, 0x48, 0x2b}
RIDL!{#[uuid(0xf5185dd8, 0x2012, 0x4b0b, 0xaa, 0xd9, 0xf0, 0x52, 0xc6, 0xbd, 0x48, 0x2b)]
interface IPicture2(IPicture2Vtbl): IUnknown(IUnknownVtbl) {
    fn get_Handle(
        pHandle: *mut HHANDLE,
    ) -> HRESULT,
    fn get_hPal(
        phPal: *mut HHANDLE,
    ) -> HRESULT,
    fn get_Type(
        pType: *mut SHORT,
    ) -> HRESULT,
    fn get_Width(
        pWidth: *mut OLE_XSIZE_HIMETRIC,
    ) -> HRESULT,
    fn get_Height(
        pHeight: *mut OLE_YSIZE_HIMETRIC,
    ) -> HRESULT,
    fn Render(
        hDC: HDC,
        x: LONG,
        y: LONG,
        cx: LONG,
        cy: LONG,
        xSrc: OLE_XPOS_HIMETRIC,
        ySrc: OLE_YPOS_HIMETRIC,
        cxSrc: OLE_XSIZE_HIMETRIC,
        cySrc: OLE_YSIZE_HIMETRIC,
        pRcWBounds: LPCRECT,
    ) -> HRESULT,
    fn set_hPal(
        hPal: HHANDLE,
    ) -> HRESULT,
    fn get_CurDC(
        phDC: *mut HDC,
    ) -> HRESULT,
    fn SelectPicture(
        hDCIn: HDC,
        phDCOut: *mut HDC,
        phBmpOut: *mut HHANDLE,
    ) -> HRESULT,
    fn get_KeepOriginalFormat(
        pKeep: *mut BOOL,
    ) -> HRESULT,
    fn put_KeepOriginalFormat(
        keep: BOOL,
    ) -> HRESULT,
    fn PictureChanged() -> HRESULT,
    fn SaveAsFile(
        pStream: LPSTREAM,
        fSaveMemCopy: BOOL,
        pCbSize: *mut LONG,
    ) -> HRESULT,
    fn get_Attributes(
        pDwAttr: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPFONTEVENTS = *mut IFontEventsDisp;
DEFINE_GUID!{IID_IFontEventsDisp,
    0x4ef6100a, 0xaf88, 0x11d0, 0x98, 0x46, 0x00, 0xc0, 0x4f, 0xc2, 0x99, 0x93}
RIDL!{#[uuid(0x4ef6100a, 0xaf88, 0x11d0, 0x98, 0x46, 0x00, 0xc0, 0x4f, 0xc2, 0x99, 0x93)]
interface IFontEventsDisp(IFontEventsDispVtbl): IDispatch(IDispatchVtbl) {
}}
pub type LPFONTDISP = *mut IFontDisp;
DEFINE_GUID!{IID_IFontDisp,
    0xbef6e003, 0xa874, 0x101a, 0x8b, 0xba, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab}
RIDL!{#[uuid(0xbef6e003, 0xa874, 0x101a, 0x8b, 0xba, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab)]
interface IFontDisp(IFontDispVtbl): IDispatch(IDispatchVtbl) {
}}
pub type LPPICTUREDISP = *mut IPictureDisp;
DEFINE_GUID!{IID_IPictureDisp,
    0x7bf80981, 0xbf32, 0x101a, 0x8b, 0xbb, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab}
RIDL!{#[uuid(0x7bf80981, 0xbf32, 0x101a, 0x8b, 0xbb, 0x00, 0xaa, 0x00, 0x30, 0x0c, 0xab)]
interface IPictureDisp(IPictureDispVtbl): IDispatch(IDispatchVtbl) {
}}
pub type LPOLEINPLACEOBJECTWINDOWLESS = *mut IOleInPlaceObjectWindowless;
DEFINE_GUID!{IID_IOleInPlaceObjectWindowless,
    0x1c2056cc, 0x5ef4, 0x101b, 0x8b, 0xc8, 0x00, 0xaa, 0x00, 0x3e, 0x3b, 0x29}
RIDL!{#[uuid(0x1c2056cc, 0x5ef4, 0x101b, 0x8b, 0xc8, 0x00, 0xaa, 0x00, 0x3e, 0x3b, 0x29)]
interface IOleInPlaceObjectWindowless(IOleInPlaceObjectWindowlessVtbl):
  IOleInPlaceObject(IOleInPlaceObjectVtbl)
{
    fn OnWindowMessage(
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        plResult: *mut LRESULT,
    ) -> HRESULT,
    fn GetDropTarget(
        ppDropTarget: *mut *mut IDropTarget,
    ) -> HRESULT,
}}
pub type LPOLEINPLACESITEEX = *mut IOleInPlaceSiteEx;
ENUM!{enum ACTIVATEFLAGS {
    ACTIVATE_WINDOWLESS = 1,
}}
DEFINE_GUID!{IID_IOleInPlaceSiteEx,
    0x9c2cad80, 0x3424, 0x11cf, 0xb6, 0x70, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8}
RIDL!{#[uuid(0x9c2cad80, 0x3424, 0x11cf, 0xb6, 0x70, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8)]
interface IOleInPlaceSiteEx(IOleInPlaceSiteExVtbl): IOleInPlaceSite(IOleInPlaceSiteVtbl) {
    fn OnInPlaceActivateEx(
        pfNoRedraw: *mut BOOL,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn OnInPlaceDeactivateEx(
        fNoRedraw: BOOL,
    ) -> HRESULT,
    fn RequestUIActivate() -> HRESULT,
}}
pub type LPOLEINPLACESITEWINDOWLESS = *mut IOleInPlaceSiteWindowless;
ENUM!{enum OLEDCFLAGS {
    OLEDC_NODRAW = 0x01,
    OLEDC_PAINTBKGND = 0x02,
    OLEDC_OFFSCREEN = 0x04,
}}
DEFINE_GUID!{IID_IOleInPlaceSiteWindowless,
    0x922eada0, 0x3424, 0x11cf, 0xb6, 0x70, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8}
RIDL!{#[uuid(0x922eada0, 0x3424, 0x11cf, 0xb6, 0x70, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8)]
interface IOleInPlaceSiteWindowless(IOleInPlaceSiteWindowlessVtbl):
  IOleInPlaceSiteEx(IOleInPlaceSiteExVtbl)
{
    fn CanWindowlessActivate() -> HRESULT,
    fn GetCapture() -> HRESULT,
    fn SetCapture(
        fCapture: BOOL,
    ) -> HRESULT,
    fn GetFocus() -> HRESULT,
    fn SetFocus(
        fFocus: BOOL,
    ) -> HRESULT,
    fn GetDC(
        pRect: LPCRECT,
        grfFlags: DWORD,
        phDC: *mut HDC,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn InvalidateRect(
        pRect: LPCRECT,
        fErase: BOOL,
    ) -> HRESULT,
    fn InvalidateRgn(
        hRGN: HRGN,
        fErase: BOOL,
    ) -> HRESULT,
    fn ScrollRect(
        dx: INT,
        dy: INT,
        pRectScroll: LPCRECT,
        pRectClip: LPCRECT,
    ) -> HRESULT,
    fn AdjustRect(
        prc: LPRECT,
    ) -> HRESULT,
    fn OnDefWindowMessage(
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        plResult: *mut LRESULT,
    ) -> HRESULT,
}}
pub type LPVIEWOBJECTEX = *mut IViewObjectEx;
ENUM!{enum VIEWSTATUS {
    VIEWSTATUS_OPAQUE = 1,
    VIEWSTATUS_SOLIDBKGND = 2,
    VIEWSTATUS_DVASPECTOPAQUE = 4,
    VIEWSTATUS_DVASPECTTRANSPARENT = 8,
    VIEWSTATUS_SURFACE = 16,
    VIEWSTATUS_3DSURFACE = 32,
}}
ENUM!{enum HITRESULT {
    HITRESULT_OUTSIDE = 0,
    HITRESULT_TRANSPARENT = 1,
    HITRESULT_CLOSE = 2,
    HITRESULT_HIT = 3,
}}
ENUM!{enum DVASPECT2 {
    DVASPECT_OPAQUE = 16,
    DVASPECT_TRANSPARENT = 32,
}}
STRUCT!{struct DVEXTENTINFO {
    cb: ULONG,
    dwExtentMode: DWORD,
    sizelProposed: SIZEL,
}}
ENUM!{enum DVEXTENTMODE {
    DVEXTENT_CONTENT,
    DVEXTENT_INTEGRAL,
}}
ENUM!{enum DVASPECTINFOFLAG {
    DVASPECTINFOFLAG_CANOPTIMIZE = 1,
}}
STRUCT!{struct DVASPECTINFO {
    cb: ULONG,
    dwFlags: DWORD,
}}
DEFINE_GUID!{IID_IViewObjectEx,
    0x3af24292, 0x0c96, 0x11ce, 0xa0, 0xcf, 0x00, 0xaa, 0x00, 0x60, 0x0a, 0xb8}
RIDL!{#[uuid(0x3af24292, 0x0c96, 0x11ce, 0xa0, 0xcf, 0x00, 0xaa, 0x00, 0x60, 0x0a, 0xb8)]
interface IViewObjectEx(IViewObjectExVtbl): IViewObject2(IViewObject2Vtbl) {
    fn GetRect(
        dwAspect: DWORD,
        pRect: LPRECTL,
    ) -> HRESULT,
    fn GetViewStatus(
        pdwStatus: *mut DWORD,
    ) -> HRESULT,
    fn QueryHitPoint(
        dwAspect: DWORD,
        pRectBounds: LPCRECT,
        ptlLoc: POINT,
        lCloseHint: LONG,
        pHitResult: *mut DWORD,
    ) -> HRESULT,
    fn QueryHitRect(
        dwAspect: DWORD,
        pRectBounds: LPCRECT,
        pRectLoc: LPCRECT,
        lCloseHint: LONG,
        pHitResult: *mut DWORD,
    ) -> HRESULT,
    fn GetNaturalExtent(
        dwAspect: DWORD,
        lindex: LONG,
        ptd: *mut DVTARGETDEVICE,
        hicTargetDev: HDC,
        pExtentInfo: *mut DVEXTENTINFO,
        pSizel: LPSIZEL,
    ) -> HRESULT,
}}
pub type LPOLEUNDOUNIT = *mut IOleUndoUnit;
DEFINE_GUID!{IID_IOleUndoUnit,
    0x894ad3b0, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01}
RIDL!{#[uuid(0x894ad3b0, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01)]
interface IOleUndoUnit(IOleUndoUnitVtbl): IUnknown(IUnknownVtbl) {
    fn Do(
        pUndoManager: *mut IOleUndoManager,
    ) -> HRESULT,
    fn GetDescription(
        pBstr: *mut BSTR,
    ) -> HRESULT,
    fn GetUnitType(
        pClsid: *mut CLSID,
        plID: *mut LONG,
    ) -> HRESULT,
    fn OnNextAdd() -> HRESULT,
}}
pub type LPOLEPARENTUNDOUNIT = *mut IOleParentUndoUnit;
DEFINE_GUID!{IID_IOleParentUndoUnit,
    0xa1faf330, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01}
RIDL!{#[uuid(0xa1faf330, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01)]
interface IOleParentUndoUnit(IOleParentUndoUnitVtbl): IOleUndoUnit(IOleUndoUnitVtbl) {
    fn Open(
        pPUU: *mut IOleParentUndoUnit,
    ) -> HRESULT,
    fn Close(
        pPUU: *mut IOleParentUndoUnit,
        fCommit: BOOL,
    ) -> HRESULT,
    fn Add(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn FindUnit(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn GetParentState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPENUMOLEUNDOUNITS = *mut IEnumOleUndoUnits;
DEFINE_GUID!{IID_IEnumOleUndoUnits,
    0xb3e7c340, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01}
RIDL!{#[uuid(0xb3e7c340, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01)]
interface IEnumOleUndoUnits(IEnumOleUndoUnitsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cElt: ULONG,
        rgElt: *mut *mut IOleUndoUnit,
        pcEltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cElt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumOleUndoUnits,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IEnumOleUndoUnits_RemoteNext_Proxy(
//     IEnumOleUndoUnits * This,
//     ULONG cElt,
//     IOleUndoUnit **rgElt,
//     ULONG *pcEltFetched);
// void __RPC_STUB IEnumOleUndoUnits_RemoteNext_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
pub type LPOLEUNDOMANAGER = *mut IOleUndoManager;
pub use self::IID_IOleUndoManager as SID_SOleUndoManager;
DEFINE_GUID!{IID_IOleUndoManager,
    0xd001f200, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01}
RIDL!{#[uuid(0xd001f200, 0xef97, 0x11ce, 0x9b, 0xc9, 0x00, 0xaa, 0x00, 0x60, 0x8e, 0x01)]
interface IOleUndoManager(IOleUndoManagerVtbl): IUnknown(IUnknownVtbl) {
    fn Open(
        pPUU: *mut IOleParentUndoUnit,
    ) -> HRESULT,
    fn Close(
        pPUU: *mut IOleParentUndoUnit,
        fCommit: BOOL,
    ) -> HRESULT,
    fn Add(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn GetOpenParentState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
    fn DiscardFrom(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn UndoTo(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn RedoTo(
        pUU: *mut IOleUndoUnit,
    ) -> HRESULT,
    fn EnumUndoable(
        ppEnum: *mut *mut IEnumOleUndoUnits,
    ) -> HRESULT,
    fn EnumRedoable(
        ppEnum: *mut *mut IEnumOleUndoUnits,
    ) -> HRESULT,
    fn GetLastUndoDescription(
        pBstr: *mut BSTR,
    ) -> HRESULT,
    fn GetLastRedoDescription(
        pBstr: *mut BSTR,
    ) -> HRESULT,
    fn Enable(
        fEnable: BOOL,
    ) -> HRESULT,
}}
pub type LPPOINTERINACTIVE = *mut IPointerInactive;
ENUM!{enum POINTERINACTIVE {
    POINTERINACTIVE_ACTIVATEONENTRY = 1,
    POINTERINACTIVE_DEACTIVATEONLEAVE = 2,
    POINTERINACTIVE_ACTIVATEONDRAG = 4,
}}
DEFINE_GUID!{IID_IPointerInactive,
    0x55980ba0, 0x35aa, 0x11cf, 0xb6, 0x71, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8}
RIDL!{#[uuid(0x55980ba0, 0x35aa, 0x11cf, 0xb6, 0x71, 0x00, 0xaa, 0x00, 0x4c, 0xd6, 0xd8)]
interface IPointerInactive(IPointerInactiveVtbl): IUnknown(IUnknownVtbl) {
    fn GetActivationPolicy(
        pdwPolicy: *mut DWORD,
    ) -> HRESULT,
    fn OnInactiveMouseMove(
        pRectBounds: LPCRECT,
        x: LONG,
        y: LONG,
        grfKeyState: DWORD,
    ) -> HRESULT,
    fn OnInactiveSetCursor(
        pRectBounds: LPCRECT,
        x: LONG,
        y: LONG,
        dwMouseMsg: DWORD,
        fSetAlways: BOOL,
    ) -> HRESULT,
}}
pub type LPOBJECTWITHSITE = *mut IObjectWithSite;
DEFINE_GUID!{IID_IObjectWithSite,
    0xfc4801a3, 0x2ba9, 0x11cf, 0xa2, 0x29, 0x00, 0xaa, 0x00, 0x3d, 0x73, 0x52}
RIDL!{#[uuid(0xfc4801a3, 0x2ba9, 0x11cf, 0xa2, 0x29, 0x00, 0xaa, 0x00, 0x3d, 0x73, 0x52)]
interface IObjectWithSite(IObjectWithSiteVtbl): IUnknown(IUnknownVtbl) {
    fn SetSite(
        pUnkSite: *mut IUnknown,
    ) -> HRESULT,
    fn GetSite(
        riid: REFIID,
        ppvSite: *mut *mut c_void,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0036_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0036_v0_0_s_ifspec;
pub type LPPERPROPERTYBROWSING = *mut IPerPropertyBrowsing;
STRUCT!{struct CALPOLESTR {
    cElems: ULONG,
    pElems: *mut LPOLESTR,
}}
pub type LPCALPOLESTR = *mut CALPOLESTR;
STRUCT!{struct CADWORD {
    cElems: ULONG,
    pElems: *mut DWORD,
}}
pub type LPCADWORD = *mut CADWORD;
DEFINE_GUID!{IID_IPerPropertyBrowsing,
    0x376bd3aa, 0x3845, 0x101b, 0x84, 0xed, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13}
RIDL!{#[uuid(0x376bd3aa, 0x3845, 0x101b, 0x84, 0xed, 0x08, 0x00, 0x2b, 0x2e, 0xc7, 0x13)]
interface IPerPropertyBrowsing(IPerPropertyBrowsingVtbl): IUnknown(IUnknownVtbl) {
    fn GetDisplayString(
        dispID: DISPID,
        pBstr: *mut BSTR,
    ) -> HRESULT,
    fn MapPropertyToPage(
        dispID: DISPID,
        pClsid: *mut CLSID,
    ) -> HRESULT,
    fn GetPredefinedStrings(
        dispID: DISPID,
        pCaStringsOut: *mut CALPOLESTR,
        pCaCookiesOut: *mut CADWORD,
    ) -> HRESULT,
    fn GetPredefinedValue(
        dispID: DISPID,
        dwCookie: DWORD,
        pVarOut: *mut VARIANT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0037_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0037_v0_0_s_ifspec;
pub type LPPROPERTYBAG2 = *mut IPropertyBag2;
ENUM!{enum PROPBAG2_TYPE {
    PROPBAG2_TYPE_UNDEFINED = 0,
    PROPBAG2_TYPE_DATA = 1,
    PROPBAG2_TYPE_URL = 2,
    PROPBAG2_TYPE_OBJECT = 3,
    PROPBAG2_TYPE_STREAM = 4,
    PROPBAG2_TYPE_STORAGE = 5,
    PROPBAG2_TYPE_MONIKER = 6,
}}
STRUCT!{struct PROPBAG2 {
    dwType: DWORD,
    vt: VARTYPE,
    cfType: CLIPFORMAT,
    dwHint: DWORD,
    pstrName: LPOLESTR,
    clsid: CLSID,
}}
DEFINE_GUID!{IID_IPropertyBag2,
    0x22f55882, 0x280b, 0x11d0, 0xa8, 0xa9, 0x00, 0xa0, 0xc9, 0x0c, 0x20, 0x04}
RIDL!{#[uuid(0x22f55882, 0x280b, 0x11d0, 0xa8, 0xa9, 0x00, 0xa0, 0xc9, 0x0c, 0x20, 0x04)]
interface IPropertyBag2(IPropertyBag2Vtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        cProperties: ULONG,
        pPropBag: *mut PROPBAG2,
        pErrLog: *mut IErrorLog,
        pvarValue: *mut VARIANT,
        phrError: *mut HRESULT,
    ) -> HRESULT,
    fn Write(
        cProperties: ULONG,
        pPropBag: *mut PROPBAG2,
        pvarValue: *mut VARIANT,
    ) -> HRESULT,
    fn CountProperties(
        pcProperties: *mut ULONG,
    ) -> HRESULT,
    fn GetPropertyInfo(
        iProperty: ULONG,
        cProperties: ULONG,
        pPropBag: *mut PROPBAG2,
        pcProperties: *mut ULONG,
    ) -> HRESULT,
    fn LoadObject(
        pstrName: LPCOLESTR,
        dwHint: DWORD,
        pUnkObject: *mut IUnknown,
        pErrLog: *mut IErrorLog,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0038_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0038_v0_0_s_ifspec;
pub type LPPERSISTPROPERTYBAG2 = *mut IPersistPropertyBag2;
DEFINE_GUID!{IID_IPersistPropertyBag2,
    0x22f55881, 0x280b, 0x11d0, 0xa8, 0xa9, 0x00, 0xa0, 0xc9, 0x0c, 0x20, 0x04}
RIDL!{#[uuid(0x22f55881, 0x280b, 0x11d0, 0xa8, 0xa9, 0x00, 0xa0, 0xc9, 0x0c, 0x20, 0x04)]
interface IPersistPropertyBag2(IPersistPropertyBag2Vtbl): IPersist(IPersistVtbl) {
    fn InitNew() -> HRESULT,
    fn Load(
        pPropBag: *mut IPropertyBag2,
        pErrLog: *mut IErrorLog,
    ) -> HRESULT,
    fn Save(
        pPropBag: *mut IPropertyBag2,
        fClearDirty: BOOL,
        fSaveAllProperties: BOOL,
    ) -> HRESULT,
    fn IsDirty() -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0039_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0039_v0_0_s_ifspec;
pub type LPADVISESINKEX = *mut IAdviseSinkEx;
DEFINE_GUID!{IID_IAdviseSinkEx,
    0x3af24290, 0x0c96, 0x11ce, 0xa0, 0xcf, 0x00, 0xaa, 0x00, 0x60, 0x0a, 0xb8}
RIDL!{#[uuid(0x3af24290, 0x0c96, 0x11ce, 0xa0, 0xcf, 0x00, 0xaa, 0x00, 0x60, 0x0a, 0xb8)]
interface IAdviseSinkEx(IAdviseSinkExVtbl): IAdviseSink(IAdviseSinkVtbl) {
    fn OnViewStatusChange(
        dwViewStatus: DWORD,
    ) -> (),
}}
// HRESULT STDMETHODCALLTYPE IAdviseSinkEx_RemoteOnViewStatusChange_Proxy(
//     IAdviseSinkEx * This,
//     DWORD dwViewStatus);
// void __RPC_STUB IAdviseSinkEx_RemoteOnViewStatusChange_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0040_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0040_v0_0_s_ifspec;
pub type LPQUICKACTIVATE = *mut IQuickActivate;
ENUM!{enum QACONTAINERFLAGS {
    QACONTAINER_SHOWHATCHING = 0x0001,
    QACONTAINER_SHOWGRABHANDLES = 0x0002,
    QACONTAINER_USERMODE = 0x0004,
    QACONTAINER_DISPLAYASDEFAULT = 0x0008,
    QACONTAINER_UIDEAD = 0x0010,
    QACONTAINER_AUTOCLIP = 0x0020,
    QACONTAINER_MESSAGEREFLECT = 0x0040,
    QACONTAINER_SUPPORTSMNEMONICS = 0x0080,
}}
// [uuid(66504301-BE0F-101A-8BBB-00AA00300CAB), public]
pub type OLE_COLOR = DWORD;
STRUCT!{struct QACONTAINER {
    cbSize: ULONG,
    pClientSite: *mut IOleClientSite,
    pAdviseSink: *mut IAdviseSinkEx,
    pPropertyNotifySink: *mut IPropertyNotifySink,
    pUnkEventSink: *mut IUnknown,
    dwAmbientFlags: DWORD,
    colorFore: OLE_COLOR,
    colorBack: OLE_COLOR,
    pFont: *mut IFont,
    pUndoMgr: *mut IOleUndoManager,
    dwAppearance: DWORD,
    lcid: LONG,
    hpal: HPALETTE,
    pBindHost: *mut IBindHost,
    pOleControlSite: *mut IOleControlSite,
    pServiceProvider: *mut IServiceProvider,
}}
STRUCT!{struct QACONTROL {
    cbSize: ULONG,
    dwMiscStatus: DWORD,
    dwViewStatus: DWORD,
    dwEventCookie: DWORD,
    dwPropNotifyCookie: DWORD,
    dwPointerActivationPolicy: DWORD,
}}
DEFINE_GUID!{IID_IQuickActivate,
    0xcf51ed10, 0x62fe, 0x11cf, 0xbf, 0x86, 0x00, 0xa0, 0xc9, 0x03, 0x48, 0x36}
RIDL!{#[uuid(0xcf51ed10, 0x62fe, 0x11cf, 0xbf, 0x86, 0x00, 0xa0, 0xc9, 0x03, 0x48, 0x36)]
interface IQuickActivate(IQuickActivateVtbl): IUnknown(IUnknownVtbl) {
    fn QuickActivate(
        pQaContainer: *mut QACONTAINER,
        pQaControl: *mut QACONTROL,
    ) -> HRESULT,
    fn SetContentExtent(
        pSizel: LPSIZEL,
    ) -> HRESULT,
    fn GetContentExtent(
        pSizel: LPSIZEL,
    ) -> HRESULT,
}}
// HRESULT STDMETHODCALLTYPE IQuickActivate_RemoteQuickActivate_Proxy(
//     IQuickActivate * This,
//     QACONTAINER *pQaContainer,
//     QACONTROL *pQaControl);
// void __RPC_STUB IQuickActivate_RemoteQuickActivate_Stub(
//     IRpcStubBuffer *This,
//     IRpcChannelBuffer *_pRpcChannelBuffer,
//     PRPC_MESSAGE _pRpcMessage,
//     DWORD *_pdwStubPhase);
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0041_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocidl_0000_0041_v0_0_s_ifspec;
// unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * );
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
// unsigned long             __RPC_USER  HFONT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HFONT * );
// unsigned char * __RPC_USER  HFONT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HFONT * );
// unsigned char * __RPC_USER  HFONT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HFONT * );
// void                      __RPC_USER  HFONT_UserFree(     __RPC__in unsigned long *, __RPC__in HFONT * );
// unsigned long             __RPC_USER  HPALETTE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HPALETTE * );
// unsigned char * __RPC_USER  HPALETTE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HPALETTE * );
// unsigned char * __RPC_USER  HPALETTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HPALETTE * );
// void                      __RPC_USER  HPALETTE_UserFree(     __RPC__in unsigned long *, __RPC__in HPALETTE * );
// unsigned long             __RPC_USER  HRGN_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HRGN * );
// unsigned char * __RPC_USER  HRGN_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HRGN * );
// unsigned char * __RPC_USER  HRGN_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HRGN * );
// void                      __RPC_USER  HRGN_UserFree(     __RPC__in unsigned long *, __RPC__in HRGN * );
// unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * );
// unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * );
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
// unsigned long             __RPC_USER  HFONT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HFONT * );
// unsigned char * __RPC_USER  HFONT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HFONT * );
// unsigned char * __RPC_USER  HFONT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HFONT * );
// void                      __RPC_USER  HFONT_UserFree64(     __RPC__in unsigned long *, __RPC__in HFONT * );
// unsigned long             __RPC_USER  HPALETTE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HPALETTE * );
// unsigned char * __RPC_USER  HPALETTE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HPALETTE * );
// unsigned char * __RPC_USER  HPALETTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HPALETTE * );
// void                      __RPC_USER  HPALETTE_UserFree64(     __RPC__in unsigned long *, __RPC__in HPALETTE * );
// unsigned long             __RPC_USER  HRGN_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HRGN * );
// unsigned char * __RPC_USER  HRGN_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HRGN * );
// unsigned char * __RPC_USER  HRGN_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HRGN * );
// void                      __RPC_USER  HRGN_UserFree64(     __RPC__in unsigned long *, __RPC__in HRGN * );
// unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * );
// unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * );
// void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * );
// unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * );
// HRESULT STDMETHODCALLTYPE IEnumConnections_Next_Proxy(
//     IEnumConnections * This,
//     ULONG cConnections,
//     LPCONNECTDATA rgcd,
//     ULONG *pcFetched);
// HRESULT STDMETHODCALLTYPE IEnumConnections_Next_Stub(
//     IEnumConnections * This,
//     ULONG cConnections,
//     LPCONNECTDATA rgcd,
//     ULONG *pcFetched);
// HRESULT STDMETHODCALLTYPE IEnumConnectionPoints_Next_Proxy(
//     IEnumConnectionPoints * This,
//     ULONG cConnections,
//     LPCONNECTIONPOINT *ppCP,
//     ULONG *pcFetched);
// HRESULT STDMETHODCALLTYPE IEnumConnectionPoints_Next_Stub(
//     IEnumConnectionPoints * This,
//     ULONG cConnections,
//     LPCONNECTIONPOINT *ppCP,
//     ULONG *pcFetched);
// HRESULT STDMETHODCALLTYPE IClassFactory2_CreateInstanceLic_Proxy(
//     IClassFactory2 * This,
//     IUnknown *pUnkOuter,
//     IUnknown *pUnkReserved,
//     REFIID riid,
//     BSTR bstrKey,
//     PVOID *ppvObj);
// HRESULT STDMETHODCALLTYPE IClassFactory2_CreateInstanceLic_Stub(
//     IClassFactory2 * This,
//     REFIID riid,
//     BSTR bstrKey,
//     IUnknown **ppvObj);
// HRESULT STDMETHODCALLTYPE IPersistMemory_Load_Proxy(
//     IPersistMemory * This,
//     LPVOID pMem,
//     ULONG cbSize);
// HRESULT STDMETHODCALLTYPE IPersistMemory_Load_Stub(
//     IPersistMemory * This,
//     BYTE *pMem,
//     ULONG cbSize);
// HRESULT STDMETHODCALLTYPE IPersistMemory_Save_Proxy(
//     IPersistMemory * This,
//     LPVOID pMem,
//     BOOL fClearDirty,
//     ULONG cbSize);
// HRESULT STDMETHODCALLTYPE IPersistMemory_Save_Stub(
//     IPersistMemory * This,
//     BYTE *pMem,
//     BOOL fClearDirty,
//     ULONG cbSize);
// HRESULT STDMETHODCALLTYPE IEnumOleUndoUnits_Next_Proxy(
//     IEnumOleUndoUnits * This,
//     ULONG cElt,
//     IOleUndoUnit **rgElt,
//     ULONG *pcEltFetched);
// HRESULT STDMETHODCALLTYPE IEnumOleUndoUnits_Next_Stub(
//     IEnumOleUndoUnits * This,
//     ULONG cElt,
//     IOleUndoUnit **rgElt,
//     ULONG *pcEltFetched);
// void STDMETHODCALLTYPE IAdviseSinkEx_OnViewStatusChange_Proxy(
//     IAdviseSinkEx * This,
//     DWORD dwViewStatus);
// HRESULT STDMETHODCALLTYPE IAdviseSinkEx_OnViewStatusChange_Stub(
//     IAdviseSinkEx * This,
//     DWORD dwViewStatus);
// HRESULT STDMETHODCALLTYPE IQuickActivate_QuickActivate_Proxy(
//     IQuickActivate * This,
//     QACONTAINER *pQaContainer,
//     QACONTROL *pQaControl);
// HRESULT STDMETHODCALLTYPE IQuickActivate_QuickActivate_Stub(
//     IQuickActivate * This,
//     QACONTAINER *pQaContainer,
//     QACONTROL *pQaControl);
