// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{GUID};
use shared::wtypes::{BSTR};
use um::oaidl::{IDispatch, IDispatchVtbl, LPDISPATCH, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{HRESULT};
pub type LONG_PTR = i64;
RIDL!{#[uuid(0x56a868b9, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IAMCollection(IAMCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        plCount: *mut i32,
    ) -> HRESULT,
    fn Item(
        lItem: i32,
        ppUnk: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get__NewEnum(
        ppUnk: *mut LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b1, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaControl(IMediaControlVtbl): IDispatch(IDispatchVtbl) {
    fn Run(
    ) -> HRESULT,
    fn Pause(
    ) -> HRESULT,
    fn Stop(
    ) -> HRESULT,
    fn GetState(
        msTimeout: i32,
        pfs: *mut i32,
    ) -> HRESULT,
    fn RenderFile(
        strFilename: BSTR,
    ) -> HRESULT,
    fn AddSourceFilter(
        strFilename: BSTR,
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_FilterCollection(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_RegFilterCollection(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn StopWhenReady(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b6, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaEvent(IMediaEventVtbl): IDispatch(IDispatchVtbl) {
    fn GetEventHandle(
        hEvent: *mut LONG_PTR,
    ) -> HRESULT,
    fn GetEvent(
        lEventCode: *mut i32,
        lParam1: *mut LONG_PTR,
        lParam2: *mut LONG_PTR,
        msTimeout: i32,
    ) -> HRESULT,
    fn WaitForCompletion(
        msTimeout: i32,
        pEvCode: *mut i32,
    ) -> HRESULT,
    fn CancelDefaultHandling(
        lEvCode: i32,
    ) -> HRESULT,
    fn RestoreDefaultHandling(
        lEvCode: i32,
    ) -> HRESULT,
    fn FreeEventParams(
        lEvCode: i32,
        lParam1: LONG_PTR,
        lParam2: LONG_PTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868c0, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaEventEx(IMediaEventExVtbl): IMediaEvent(IMediaEventVtbl) {
    fn SetNotifyWindow(
        hwnd: LONG_PTR,
        lMsg: i32,
        lInstanceData: LONG_PTR,
    ) -> HRESULT,
    fn SetNotifyFlags(
        lNoNotifyFlags: i32,
    ) -> HRESULT,
    fn GetNotifyFlags(
        lplNoNotifyFlags: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b2, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaPosition(IMediaPositionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Duration(
        plength: *mut f64,
    ) -> HRESULT,
    fn put_CurrentPosition(
        pllTime: f64,
    ) -> HRESULT,
    fn get_CurrentPosition(
        pllTime: *mut f64,
    ) -> HRESULT,
    fn get_StopTime(
        pllTime: *mut f64,
    ) -> HRESULT,
    fn put_StopTime(
        pllTime: f64,
    ) -> HRESULT,
    fn get_PrerollTime(
        pllTime: *mut f64,
    ) -> HRESULT,
    fn put_PrerollTime(
        pllTime: f64,
    ) -> HRESULT,
    fn put_Rate(
        pdRate: f64,
    ) -> HRESULT,
    fn get_Rate(
        pdRate: *mut f64,
    ) -> HRESULT,
    fn CanSeekForward(
        pCanSeekForward: *mut i32,
    ) -> HRESULT,
    fn CanSeekBackward(
        pCanSeekBackward: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b3, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IBasicAudio(IBasicAudioVtbl): IDispatch(IDispatchVtbl) {
    fn put_Volume(
        plVolume: i32,
    ) -> HRESULT,
    fn get_Volume(
        plVolume: *mut i32,
    ) -> HRESULT,
    fn put_Balance(
        plBalance: i32,
    ) -> HRESULT,
    fn get_Balance(
        plBalance: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b4, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IVideoWindow(IVideoWindowVtbl): IDispatch(IDispatchVtbl) {
    fn put_Caption(
        strCaption: BSTR,
    ) -> HRESULT,
    fn get_Caption(
        strCaption: *mut BSTR,
    ) -> HRESULT,
    fn put_WindowStyle(
        WindowStyle: i32,
    ) -> HRESULT,
    fn get_WindowStyle(
        WindowStyle: *mut i32,
    ) -> HRESULT,
    fn put_WindowStyleEx(
        WindowStyleEx: i32,
    ) -> HRESULT,
    fn get_WindowStyleEx(
        WindowStyleEx: *mut i32,
    ) -> HRESULT,
    fn put_AutoShow(
        AutoShow: i32,
    ) -> HRESULT,
    fn get_AutoShow(
        AutoShow: *mut i32,
    ) -> HRESULT,
    fn put_WindowState(
        WindowState: i32,
    ) -> HRESULT,
    fn get_WindowState(
        WindowState: *mut i32,
    ) -> HRESULT,
    fn put_BackgroundPalette(
        pBackgroundPalette: i32,
    ) -> HRESULT,
    fn get_BackgroundPalette(
        pBackgroundPalette: *mut i32,
    ) -> HRESULT,
    fn put_Visible(
        pVisible: i32,
    ) -> HRESULT,
    fn get_Visible(
        pVisible: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pLeft: i32,
    ) -> HRESULT,
    fn get_Left(
        pLeft: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pWidth: i32,
    ) -> HRESULT,
    fn get_Width(
        pWidth: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pTop: i32,
    ) -> HRESULT,
    fn get_Top(
        pTop: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pHeight: i32,
    ) -> HRESULT,
    fn get_Height(
        pHeight: *mut i32,
    ) -> HRESULT,
    fn put_Owner(
        Owner: LONG_PTR,
    ) -> HRESULT,
    fn get_Owner(
        Owner: *mut LONG_PTR,
    ) -> HRESULT,
    fn put_MessageDrain(
        Drain: LONG_PTR,
    ) -> HRESULT,
    fn get_MessageDrain(
        Drain: *mut LONG_PTR,
    ) -> HRESULT,
    fn get_BorderColor(
        Color: *mut i32,
    ) -> HRESULT,
    fn put_BorderColor(
        Color: i32,
    ) -> HRESULT,
    fn get_FullScreenMode(
        FullScreenMode: *mut i32,
    ) -> HRESULT,
    fn put_FullScreenMode(
        FullScreenMode: i32,
    ) -> HRESULT,
    fn SetWindowForeground(
        Focus: i32,
    ) -> HRESULT,
    fn NotifyOwnerMessage(
        hwnd: LONG_PTR,
        uMsg: i32,
        wParam: LONG_PTR,
        lParam: LONG_PTR,
    ) -> HRESULT,
    fn SetWindowPosition(
        Left: i32,
        Top: i32,
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn GetWindowPosition(
        pLeft: *mut i32,
        pTop: *mut i32,
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn GetMinIdealImageSize(
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn GetMaxIdealImageSize(
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn GetRestorePosition(
        pLeft: *mut i32,
        pTop: *mut i32,
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn HideCursor(
        HideCursor: i32,
    ) -> HRESULT,
    fn IsCursorHidden(
        CursorHidden: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b5, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IBasicVideo(IBasicVideoVtbl): IDispatch(IDispatchVtbl) {
    fn get_AvgTimePerFrame(
        pAvgTimePerFrame: *mut f64,
    ) -> HRESULT,
    fn get_BitRate(
        pBitRate: *mut i32,
    ) -> HRESULT,
    fn get_BitErrorRate(
        pBitErrorRate: *mut i32,
    ) -> HRESULT,
    fn get_VideoWidth(
        pVideoWidth: *mut i32,
    ) -> HRESULT,
    fn get_VideoHeight(
        pVideoHeight: *mut i32,
    ) -> HRESULT,
    fn put_SourceLeft(
        pSourceLeft: i32,
    ) -> HRESULT,
    fn get_SourceLeft(
        pSourceLeft: *mut i32,
    ) -> HRESULT,
    fn put_SourceWidth(
        pSourceWidth: i32,
    ) -> HRESULT,
    fn get_SourceWidth(
        pSourceWidth: *mut i32,
    ) -> HRESULT,
    fn put_SourceTop(
        pSourceTop: i32,
    ) -> HRESULT,
    fn get_SourceTop(
        pSourceTop: *mut i32,
    ) -> HRESULT,
    fn put_SourceHeight(
        pSourceHeight: i32,
    ) -> HRESULT,
    fn get_SourceHeight(
        pSourceHeight: *mut i32,
    ) -> HRESULT,
    fn put_DestinationLeft(
        pDestinationLeft: i32,
    ) -> HRESULT,
    fn get_DestinationLeft(
        pDestinationLeft: *mut i32,
    ) -> HRESULT,
    fn put_DestinationWidth(
        pDestinationWidth: i32,
    ) -> HRESULT,
    fn get_DestinationWidth(
        pDestinationWidth: *mut i32,
    ) -> HRESULT,
    fn put_DestinationTop(
        pDestinationTop: i32,
    ) -> HRESULT,
    fn get_DestinationTop(
        pDestinationTop: *mut i32,
    ) -> HRESULT,
    fn put_DestinationHeight(
        pDestinationHeight: i32,
    ) -> HRESULT,
    fn get_DestinationHeight(
        pDestinationHeight: *mut i32,
    ) -> HRESULT,
    fn SetSourcePosition(
        Left: i32,
        Top: i32,
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn GetSourcePosition(
        pLeft: *mut i32,
        pTop: *mut i32,
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn SetDefaultSourcePosition(
    ) -> HRESULT,
    fn SetDestinationPosition(
        Left: i32,
        Top: i32,
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn GetDestinationPosition(
        pLeft: *mut i32,
        pTop: *mut i32,
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn SetDefaultDestinationPosition(
    ) -> HRESULT,
    fn GetVideoSize(
        pWidth: *mut i32,
        pHeight: *mut i32,
    ) -> HRESULT,
    fn GetVideoPaletteEntries(
        StartIndex: i32,
        Entries: i32,
        pRetrieved: *mut i32,
        pPalette: *mut i32,
    ) -> HRESULT,
    fn GetCurrentImage(
        pBufferSize: *mut i32,
        pDIBImage: *mut i32,
    ) -> HRESULT,
    fn IsUsingDefaultSource(
    ) -> HRESULT,
    fn IsUsingDefaultDestination(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x329bb360, 0xf6ea, 0x11d1, 0x90, 0x38, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x98)]
interface IBasicVideo2(IBasicVideo2Vtbl): IBasicVideo(IBasicVideoVtbl) {
    fn GetPreferredAspectRatio(
        plAspectX: *mut i32,
        plAspectY: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b8, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IDeferredCommand(IDeferredCommandVtbl): IUnknown(IUnknownVtbl) {
    fn Cancel(
    ) -> HRESULT,
    fn Confidence(
        pConfidence: *mut i32,
    ) -> HRESULT,
    fn Postpone(
        newtime: f64,
    ) -> HRESULT,
    fn GetHResult(
        phrResult: *mut HRESULT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868b7, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IQueueCommand(IQueueCommandVtbl): IUnknown(IUnknownVtbl) {
    fn InvokeAtStreamTime(
        pCmd: *mut *mut IDeferredCommand,
        time: f64,
        iid: *const GUID,
        dispidMethod: i32,
        wFlags: i16,
        cArgs: i32,
        pDispParams: *const VARIANT,
        pvarResult: *mut VARIANT,
        puArgErr: *mut i16,
    ) -> HRESULT,
    fn InvokeAtPresentationTime(
        pCmd: *mut *mut IDeferredCommand,
        time: f64,
        iid: *const GUID,
        dispidMethod: i32,
        wFlags: i16,
        cArgs: i32,
        pDispParams: *const VARIANT,
        pvarResult: *mut VARIANT,
        puArgErr: *mut i16,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe436ebb3, 0x524f, 0x11ce, 0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
class FilgraphManager; }
RIDL!{#[uuid(0x56a868ba, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IFilterInfo(IFilterInfoVtbl): IDispatch(IDispatchVtbl) {
    fn FindPin(
        strPinID: BSTR,
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
    fn get_VendorInfo(
        strVendorInfo: *mut BSTR,
    ) -> HRESULT,
    fn get_Filter(
        ppUnk: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Pins(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_IsFileSource(
        pbIsSource: *mut i32,
    ) -> HRESULT,
    fn get_Filename(
        pstrFilename: *mut BSTR,
    ) -> HRESULT,
    fn put_Filename(
        pstrFilename: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868bb, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IRegFilterInfo(IRegFilterInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
    fn Filter(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868bc, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaTypeInfo(IMediaTypeInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        strType: *mut BSTR,
    ) -> HRESULT,
    fn get_Subtype(
        strType: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868bd, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IPinInfo(IPinInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_Pin(
        ppUnk: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_ConnectedTo(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_ConnectionMediaType(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_FilterInfo(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Name(
        ppUnk: *mut BSTR,
    ) -> HRESULT,
    fn get_Direction(
        ppDirection: *mut i32,
    ) -> HRESULT,
    fn get_PinID(
        strPinID: *mut BSTR,
    ) -> HRESULT,
    fn get_MediaTypes(
        ppUnk: *mut LPDISPATCH,
    ) -> HRESULT,
    fn Connect(
        pPin: LPUNKNOWN,
    ) -> HRESULT,
    fn ConnectDirect(
        pPin: LPUNKNOWN,
    ) -> HRESULT,
    fn ConnectWithType(
        pPin: LPUNKNOWN,
        pMediaType: LPDISPATCH,
    ) -> HRESULT,
    fn Disconnect(
    ) -> HRESULT,
    fn Render(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbc9bcf80, 0xdcd2, 0x11d2, 0xab, 0xf6, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IAMStats(IAMStatsVtbl): IDispatch(IDispatchVtbl) {
    fn Reset(
    ) -> HRESULT,
    fn get_Count(
        plCount: *mut i32,
    ) -> HRESULT,
    fn GetValueByIndex(
        lIndex: i32,
        szName: *mut BSTR,
        lCount: *mut i32,
        dLast: *mut f64,
        dAverage: *mut f64,
        dStdDev: *mut f64,
        dMin: *mut f64,
        dMax: *mut f64,
    ) -> HRESULT,
    fn GetValueByName(
        szName: BSTR,
        lIndex: *mut i32,
        lCount: *mut i32,
        dLast: *mut f64,
        dAverage: *mut f64,
        dStdDev: *mut f64,
        dMin: *mut f64,
        dMax: *mut f64,
    ) -> HRESULT,
    fn GetIndex(
        szName: BSTR,
        lCreate: i32,
        plIndex: *mut i32,
    ) -> HRESULT,
    fn AddValue(
        lIndex: i32,
        dValue: f64,
    ) -> HRESULT,
}}
