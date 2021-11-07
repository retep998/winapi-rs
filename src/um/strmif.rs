// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// TODO: Missing DVD things
use ctypes::{c_double, c_float, c_int, c_long, c_void};
use shared::basetsd::{DWORD_PTR, LONG_PTR};
use shared::guiddef::{CLSID, GUID, LPGUID, REFCLSID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, HKEY, LPDWORD, LPVOID, UINT, ULONG, WORD};
use shared::windef::{COLORREF, HDC, HMONITOR, HWND, LPRECT, RECT, SIZE};
use shared::wtypesbase::{LPCOLESTR, LPOLESTR};
use um::ddraw::{DDCOLORKEY, LPDDCOLORKEY, LPDDPIXELFORMAT, LPDIRECTDRAW7, LPDIRECTDRAWSURFACE7};
use um::oaidl::{IErrorLog, IPropertyBag, IPropertyBagVtbl, VARIANT};
use um::objidl::{IBindCtx, IEnumMoniker, IMoniker, IPersist, IPersistVtbl};
use um::objidlbase::{IStream};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{LPBITMAPINFOHEADER, PALETTEENTRY, RGNDATA};
use um::winnt::{
    DWORDLONG, HANDLE, HRESULT, LARGE_INTEGER, LCID, LONG, LONGLONG, LPCWSTR, LPWSTR, PVOID,
    ULONGLONG, WCHAR
};
RIDL!{#[uuid(0x29840822, 0x5b84, 0x11d0, 0xbd, 0x3b, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface ICreateDevEnum(ICreateDevEnumVtbl): IUnknown(IUnknownVtbl) {
    fn CreateClassEnumerator(
        clsidDeviceClass: *const REFCLSID,
        ppenumMoniker: *mut *mut IEnumMoniker,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
STRUCT!{struct AM_MEDIA_TYPE {
    majortype: GUID,
    subtype: GUID,
    bFixedSizeSamples: BOOL,
    bTemporalCompression: BOOL,
    lSampleSize: ULONG,
    formattype: GUID,
    punk: *mut IUnknown,
    cbFormat: ULONG,
    pbFormat: *mut u8,
}}
ENUM!{enum PIN_DIRECTION {
    PINDIR_INPUT = 0,
    PINDIR_OUTPUT = 1,
}}
pub const MAX_PIN_NAME: usize = 128;
pub const MAX_FILTER_NAME: usize = 128;
pub type REFERENCE_TIME = LONGLONG;
pub type REFTIME = c_double;
pub type HSEMAPHORE = DWORD_PTR;
pub type HEVENT = DWORD_PTR;
STRUCT!{struct ALLOCATOR_PROPERTIES {
    cBuffers: c_long,
    cbBuffer: c_long,
    cbAlign: c_long,
    cbPrefix: c_long,
}}
STRUCT!{struct PIN_INFO {
    pFilter: *mut IBaseFilter,
    dir: PIN_DIRECTION,
    achName: [WCHAR; MAX_PIN_NAME],
}}
RIDL!{#[uuid(0x56a86891, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IPin(IPinVtbl): IUnknown(IUnknownVtbl) {
    fn Connect(
        pReceivePin: *const IPin,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn ReceiveConnection(
        pConnector: *const IPin,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn Disconnect(
    ) -> HRESULT,
    fn ConnectedTo(
        pPin: *mut *mut IPin,
    ) -> HRESULT,
    fn ConnectionMediaType(
        pmt: *mut AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn QueryPinInfo(
        pInfo: *mut PIN_INFO,
    ) -> HRESULT,
    fn QueryDirection(
        pPinDir: *mut PIN_DIRECTION,
    ) -> HRESULT,
    fn QueryId(
        Id: *mut LPWSTR,
    ) -> HRESULT,
    fn QueryAccept(
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn EnumMediaTypes(
        ppenum: *mut *mut IEnumMediaTypes,
    ) -> HRESULT,
    fn QueryInternalConnections(
        apPin: *mut *mut IPin,
        nPin: *mut ULONG,
    ) -> HRESULT,
    fn EndOfStream(
    ) -> HRESULT,
    fn BeginFlush(
    ) -> HRESULT,
    fn EndFlush(
    ) -> HRESULT,
    fn NewSegment(
        tStart: REFERENCE_TIME,
        tStop: REFERENCE_TIME,
        dRate: c_double ,
    ) -> HRESULT,
}}
pub type PPIN = *mut IPin;
RIDL!{#[uuid(0x56a86892, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IEnumPins(IEnumPinsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cPins: ULONG,
        ppPins: *mut *mut IPin,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cPins: ULONG,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumPins,
    ) -> HRESULT,
}}
pub type PENUMPINS = *mut IEnumPins;
RIDL!{#[uuid(0x89c31040, 0x846b, 0x11ce, 0x97, 0xd3, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a)]
interface IEnumMediaTypes(IEnumMediaTypesVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cMediaTypes: ULONG,
        ppMediaTypes: *mut *mut AM_MEDIA_TYPE,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cMediaTypes: ULONG,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumMediaTypes,
    ) -> HRESULT,
}}
pub type PENUMMEDIATYPES = *mut IEnumMediaTypes;
RIDL!{#[uuid(0x56a8689f, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IFilterGraph(IFilterGraphVtbl): IUnknown(IUnknownVtbl) {
    fn AddFilter(
        pFilter: *const IBaseFilter,
        pName: LPCWSTR,
    ) -> HRESULT,
    fn RemoveFilter(
        pFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn EnumFilters(
        ppenum: *mut *mut IEnumFilters,
    ) -> HRESULT,
    fn FindFilterByName(
        pName: LPCWSTR,
        ppFilter: *mut *mut IBaseFilter,
    ) -> HRESULT,
    fn ConnectDirect(
        ppinOut: *const IPin,
        ppinIn: *const IPin,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn Reconnect(
        pPin: *const IPin,
    ) -> HRESULT,
    fn Disconnect(
        pPin: *const IPin,
    ) -> HRESULT,
    fn SetDefaultSyncSource(
    ) -> HRESULT,
}}
pub type PFILTERGRAPH = *mut IFilterGraph;
RIDL!{#[uuid(0x56a86893, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IEnumFilters(IEnumFiltersVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cFilters: ULONG,
        ppFilter: *mut *mut IBaseFilter,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cFilters: ULONG,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumFilters,
    ) -> HRESULT,
}}
pub type PENUMFILTER = *mut IEnumFilters;
ENUM!{enum FILTER_STATE {
    State_Stopped = 0,
    State_Paused = 1,
    State_Running = 2,
}}
RIDL!{#[uuid(0x56a86899, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaFilter(IMediaFilterVtbl): IPersist(IPersistVtbl) {
    fn Stop(
    ) -> HRESULT,
    fn Pause(
    ) -> HRESULT,
    fn Run(
        tStart: REFERENCE_TIME,
    ) -> HRESULT,
    fn GetState(
        dwMilliSecsTimeout: DWORD,
        State: *mut FILTER_STATE,
    ) -> HRESULT,
    fn SetSyncSource(
        pClock: *const IReferenceClock,
    ) -> HRESULT,
    fn GetSyncSource(
        pClock: *mut *mut IReferenceClock,
    ) -> HRESULT,
}}
pub type PMEDIAFILTER = *mut IMediaFilter;
STRUCT!{struct FILTER_INFO {
    achName: [WCHAR; MAX_FILTER_NAME],
    pGraph: *mut IFilterGraph,
}}
RIDL!{#[uuid(0x56a86895, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IBaseFilter(IBaseFilterVtbl): IMediaFilter(IMediaFilterVtbl) {
    fn EnumPins(
        ppenum: *mut *mut IEnumPins,
    ) -> HRESULT,
    fn FindPin(
        Id: LPCWSTR,
        ppPin: *mut *mut IPin,
    ) -> HRESULT,
    fn QueryFilterInfo(
        pInfo: *mut FILTER_INFO,
    ) -> HRESULT,
    fn JoinFilterGraph(
        pGraph: *const IFilterGraph,
        pName: LPCWSTR,
    ) -> HRESULT,
    fn QueryVendorInfo(
        pVendorInfo: *mut LPWSTR,
    ) -> HRESULT,
}}
pub type PFILTER = *mut IBaseFilter;
RIDL!{#[uuid(0x56a86897, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IReferenceClock(IReferenceClockVtbl): IUnknown(IUnknownVtbl) {
    fn GetTime(
        pTime: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn AdviseTime(
        baseTime: REFERENCE_TIME,
        streamTime: REFERENCE_TIME,
        hEvent: HEVENT,
        pdwAdviseCookie: *mut DWORD_PTR,
    ) -> HRESULT,
    fn AdvisePeriodic(
        startTime: REFERENCE_TIME,
        periodTime: REFERENCE_TIME,
        hSemaphore: HSEMAPHORE,
        pdwAdviseCookie: *mut DWORD_PTR,
    ) -> HRESULT,
    fn Unadvise(
        dwAdviseCookie: DWORD_PTR,
    ) -> HRESULT,
}}
pub type PREFERENCECLOCK = *mut IReferenceClock;
RIDL!{#[uuid(0xebec459c, 0x2eca, 0x4d42, 0xa8, 0xaf, 0x30, 0xdf, 0x55, 0x76, 0x14, 0xb8)]
interface IReferenceClockTimerControl(IReferenceClockTimerControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetDefaultTimerResolution(
        timerResolution: REFERENCE_TIME,
    ) -> HRESULT,
    fn GetDefaultTimerResolution(
        pTimerResolution: *mut REFERENCE_TIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x36b73885, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface IReferenceClock2(IReferenceClock2Vtbl): IReferenceClock(IReferenceClockVtbl) {
}}
pub type PREFERENCECLOCK2 = *mut IReferenceClock2;
RIDL!{#[uuid(0x56a8689a, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaSample(IMediaSampleVtbl): IUnknown(IUnknownVtbl) {
    fn GetPointer(
        ppBuffer: *mut *mut BYTE,
    ) -> HRESULT,
    fn GetSize(
    ) -> c_long,
    fn GetTime(
        pTimeStart: *mut REFERENCE_TIME,
        pTimeEnd: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn SetTime(
        pTimeStart: *const REFERENCE_TIME,
        pTimeEnd: *const REFERENCE_TIME,
    ) -> HRESULT,
    fn IsSyncPoint(
    ) -> HRESULT,
    fn SetSyncPoint(
        bIsSyncPoint: BOOL,
    ) -> HRESULT,
    fn IsPreroll(
    ) -> HRESULT,
    fn SetPreroll(
        bIsPreroll: BOOL,
    ) -> HRESULT,
    fn GetActualDataLength(
    ) -> c_long,
    fn SetActualDataLength(
        __MIDL__IMediaSample0000: c_long,
    ) -> HRESULT,
    fn GetMediaType(
        ppMediaType: *mut *mut AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn SetMediaType(
        pMediaType: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn IsDiscontinuity(
    ) -> HRESULT,
    fn SetDiscontinuity(
        bDiscontinuity: BOOL,
    ) -> HRESULT,
    fn GetMediaTime(
        pTimeStart: *mut LONGLONG,
        pTimeEnd: *mut LONGLONG,
    ) -> HRESULT,
    fn SetMediaTime(
        pTimeStart: *const LONGLONG,
        pTimeEnd: *const LONGLONG,
    ) -> HRESULT,
}}
pub type PMEDIASAMPLE = IMediaSample;
ENUM!{enum AM_SAMPLE_PROPERTY_FLAGS {
    AM_SAMPLE_SPLICEPOINT = 1,
    AM_SAMPLE_PREROLL = 2,
    AM_SAMPLE_DATADISCONTINUITY = 4,
    AM_SAMPLE_TYPECHANGED = 8,
    AM_SAMPLE_TIMEVALID = 16,
    AM_SAMPLE_TIMEDISCONTINUITY = 64,
    AM_SAMPLE_FLUSH_ON_PAUSE = 128,
    AM_SAMPLE_STOPVALID = 256,
    AM_SAMPLE_ENDOFSTREAM = 512,
    AM_STREAM_MEDIA = 0,
    AM_STREAM_CONTROL = 1,
}}
STRUCT!{struct AM_SAMPLE2_PROPERTIES {
    cbData: DWORD,
    dwTypeSpecificFlags: DWORD,
    dwSampleFlags: DWORD,
    lActual: LONG,
    tStart: REFERENCE_TIME,
    tStop: REFERENCE_TIME,
    dwStreamId: DWORD,
    pMediaType: *mut AM_MEDIA_TYPE,
    pbBuffer: *mut BYTE,
    cbBuffer: LONG,
}}
RIDL!{#[uuid(0x36b73884, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface IMediaSample2(IMediaSample2Vtbl): IMediaSample(IMediaSampleVtbl) {
    fn GetProperties(
        cbProperties: DWORD,
        pbProperties: *mut BYTE,
    ) -> HRESULT,
    fn SetProperties(
        cbProperties: DWORD,
        pbProperties: *const BYTE,
    ) -> HRESULT,
}}
pub type PMEDIASAMPLE2 = *mut IMediaSample;
RIDL!{#[uuid(0x68961e68, 0x832b, 0x41ea, 0xbc, 0x91, 0x63, 0x59, 0x3f, 0x3e, 0x70, 0xe3)]
interface IMediaSample2Config(IMediaSample2ConfigVtbl): IUnknown(IUnknownVtbl) {
    fn GetSurface(
        ppDirect3DSurface9: *mut *mut IUnknown,
    ) -> HRESULT,
}}
pub const AM_GBF_PREVFRAMESKIPPED: DWORD = 1;
pub const AM_GBF_NOTASYNCPOINT: DWORD = 2;
pub const AM_GBF_NOWAIT: DWORD = 4;
pub const AM_GBF_NODDSURFACECLOCK: DWORD = 8;
RIDL!{#[uuid(0x56a8689c, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMemAllocator(IMemAllocatorVtbl): IUnknown(IUnknownVtbl) {
    fn SetProperties(
        pRequest: *const ALLOCATOR_PROPERTIES,
        pActual: *mut ALLOCATOR_PROPERTIES,
    ) -> HRESULT,
    fn GetProperties(
        pProps: *mut ALLOCATOR_PROPERTIES,
    ) -> HRESULT,
    fn Commit(
    ) -> HRESULT,
    fn Decommit(
    ) -> HRESULT,
    fn GetBuffer(
        ppBuffer: *mut *mut IMediaSample,
        pStartTime: *const REFERENCE_TIME,
        pEndTime: *const REFERENCE_TIME,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ReleaseBuffer(
        pBuffer: *const IMediaSample,
    ) -> HRESULT,
}}
pub type PMEMALLOCATOR = *mut IMemAllocator;
RIDL!{#[uuid(0x379a0cf0, 0xc1de, 0x11d2, 0xab, 0xf5, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IMemAllocatorCallbackTemp(IMemAllocatorCallbackTempVtbl):
    IMemAllocator(IMemAllocatorVtbl) {
    fn SetNotify(
        pNotify: *const IMemAllocatorNotifyCallbackTemp,
    ) -> HRESULT,
    fn GetFreeCount(
        plBuffersFree: *mut LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x92980b30, 0xc1de, 0x11d2, 0xab, 0xf5, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IMemAllocatorNotifyCallbackTemp(IMemAllocatorNotifyCallbackTempVtbl):
    IUnknown(IUnknownVtbl) {
    fn NotifyRelease(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a8689d, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMemInputPin(IMemInputPinVtbl): IUnknown(IUnknownVtbl) {
    fn GetAllocator(
        ppAllocator: *mut *mut IMemAllocator,
    ) -> HRESULT,
    fn NotifyAllocator(
        pAllocator: *const IMemAllocator,
        bReadOnly: BOOL,
    ) -> HRESULT,
    fn GetAllocatorRequirements(
        pProps: *mut ALLOCATOR_PROPERTIES,
    ) -> HRESULT,
    fn Receive(
        pSample: *const IMediaSample,
    ) -> HRESULT,
    fn ReceiveMultiple(
        pSamples: *const *const IMediaSample,
        nSamples: c_long,
        nSamplesProcessed: *mut c_long,
    ) -> HRESULT,
    fn ReceiveCanBlock(
    ) -> HRESULT,
}}
pub type PMEMINPUTPIN = *mut IMemInputPin;
RIDL!{#[uuid(0xa3d8cec0, 0x7e5a, 0x11cf, 0xbb, 0xc5, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x20)]
interface IAMovieSetup(IAMovieSetupVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
    ) -> HRESULT,
    fn Unregister(
    ) -> HRESULT,
}}
pub type PAMOVIESETUP = *mut IAMovieSetup;
ENUM!{enum AM_SEEKING_SEEKING_FLAGS {
    AM_SEEKING_NoPositioning = 0,
    AM_SEEKING_AbsolutePositioning = 1,
    AM_SEEKING_RelativePositioning = 2,
    AM_SEEKING_IncrementalPositioning = 3,
    AM_SEEKING_PositioningBitsMask = 3,
    AM_SEEKING_SeekToKeyFrame = 4,
    AM_SEEKING_ReturnTime = 8,
    AM_SEEKING_Segment = 16,
    AM_SEEKING_NoFlush = 32,
}}
ENUM!{enum AM_SEEKING_SEEKING_CAPABILITIES {
    AM_SEEKING_CanSeekAbsolute = 1,
    AM_SEEKING_CanSeekForwards = 2,
    AM_SEEKING_CanSeekBackwards = 4,
    AM_SEEKING_CanGetCurrentPos = 8,
    AM_SEEKING_CanGetStopPos = 16,
    AM_SEEKING_CanGetDuration = 32,
    AM_SEEKING_CanPlayBackwards = 64,
    AM_SEEKING_CanDoSegments = 128,
    AM_SEEKING_Source = 256,
}}
RIDL!{#[uuid(0x36b73880, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface IMediaSeeking(IMediaSeekingVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        pCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn CheckCapabilities(
        pCapabilities: *mut DWORD,
    ) -> HRESULT,
    fn IsFormatSupported(
        pFormat: *const GUID,
    ) -> HRESULT,
    fn QueryPreferredFormat(
        pFormat: *mut GUID,
    ) -> HRESULT,
    fn GetTimeFormat(
        pFormat: *mut GUID,
    ) -> HRESULT,
    fn IsUsingTimeFormat(
        pFormat: *const GUID,
    ) -> HRESULT,
    fn SetTimeFormat(
        pFormat: *const GUID,
    ) -> HRESULT,
    fn GetDuration(
        pDuration: *mut LONGLONG,
    ) -> HRESULT,
    fn GetStopPosition(
        pStop: *mut LONGLONG,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pCurrent: *mut LONGLONG,
    ) -> HRESULT,
    fn ConvertTimeFormat(
        pTarget: *mut LONGLONG,
        pTargetFormat: *const GUID,
        Source: LONGLONG,
        pSourceFormat: *const GUID,
    ) -> HRESULT,
    fn SetPositions(
        pCurrent: *mut LONGLONG,
        dwCurrentFlags: DWORD,
        pStop: *mut LONGLONG,
        dwStopFlags: DWORD,
    ) -> HRESULT,
    fn GetPositions(
        pCurrent: *mut LONGLONG,
        pStop: *mut LONGLONG,
    ) -> HRESULT,
    fn GetAvailable(
        pEarliest: *mut LONGLONG,
        pLatest: *mut LONGLONG,
    ) -> HRESULT,
    fn SetRate(
        dRate: c_double,
    ) -> HRESULT,
    fn GetRate(
        pdRate: *mut c_double,
    ) -> HRESULT,
    fn GetPreroll(
        pllPreroll: *mut LONGLONG,
    ) -> HRESULT,
}}
pub type PMEDIASEEKING = *mut IMediaSeeking;
ENUM!{enum AM_MEDIAEVENT_FLAGS {
    AM_MEDIAEVENT_NONOTIFY = 1,
}}
STRUCT!{struct CodecAPIEventData {
    guid: GUID,
    dataLength: DWORD,
    reserved: [DWORD; 3],
}}
RIDL!{#[uuid(0x901db4c7, 0x31ce, 0x41a2, 0x85, 0xdc, 0x8f, 0xa0, 0xbf, 0x41, 0xb8, 0xda)]
interface ICodecAPI(ICodecAPIVtbl): IUnknown(IUnknownVtbl) {
    fn IsSupported(
        Api: *const GUID,
    ) -> HRESULT,
    fn IsModifiable(
        Api: *const GUID,
    ) -> HRESULT,
    fn GetParameterRange(
        Api: *const GUID,
        ValueMin: *mut VARIANT,
        ValueMax: *mut VARIANT,
        SteppingDelta: *mut VARIANT,
    ) -> HRESULT,
    fn GetParameterValues(
        Api: *const GUID,
        Values: *mut *mut VARIANT,
        ValuesCount: *mut ULONG,
    ) -> HRESULT,
    fn GetDefaultValue(
        Api: *const GUID,
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn GetValue(
        Api: *const GUID,
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn SetValue(
        Api: *const GUID,
        Value: *const VARIANT,
    ) -> HRESULT,
    fn RegisterForEvent(
        Api: *const GUID,
        userData: LONG_PTR,
    ) -> HRESULT,
    fn UnregisterForEvent(
        Api: *const GUID,
    ) -> HRESULT,
    fn SetAllDefaults(
    ) -> HRESULT,
    fn SetValueWithNotify(
        Api: *const GUID,
        Value: *const VARIANT,
        ChangedParam: *mut *mut GUID,
        ChangedParamCount: *mut ULONG,
    ) -> HRESULT,
    fn SetAllDefaultsWithNotify(
        ChangedParam: *mut *mut GUID,
        ChangedParamCount: *mut ULONG,
    ) -> HRESULT,
    fn GetAllSettings(
        __MIDL__ICodecAPI0000: *const IStream,
    ) -> HRESULT,
    fn SetAllSettings(
        __MIDL__ICodecAPI0001: *const IStream,
    ) -> HRESULT,
    fn SetAllSettingsWithNotify(
        __MIDL__ICodecAPI0002: *mut IStream,
        ChangedParam: *mut *mut GUID,
        ChangedParamCount: *mut ULONG,
    ) -> HRESULT,
}}
STRUCT!{struct REGFILTER {
    clsid: CLSID,
    Name: LPWSTR,
}}
RIDL!{#[uuid(0x56a868a4, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IEnumRegFilters(IEnumRegFiltersVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cFilters: ULONG,
        apRegFilter: *mut *mut REGFILTER,
        pcFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cFilters: ULONG,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumRegFilters,
    ) -> HRESULT,
}}
pub type PENUMREGFILTERS = *mut IEnumRegFilters;
pub const MERIT_PREFERRED: DWORD = 0x800000;
pub const MERIT_NORMAL: DWORD = 0x600000;
pub const MERIT_UNLIKELY: DWORD = 0x400000;
pub const MERIT_DO_NOT_USE: DWORD = 0x200000;
pub const MERIT_SW_COMPRESSOR: DWORD = 0x100000;
pub const MERIT_HW_COMPRESSOR: DWORD = 0x100050;
RIDL!{#[uuid(0x56a868a3, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IFilterMapper(IFilterMapperVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterFilter(
        clsid: CLSID,
        Name: LPCWSTR,
        dwMerit: DWORD,
    ) -> HRESULT,
    fn RegisterFilterInstance(
        clsid: CLSID,
        Name: LPCWSTR,
        MRId: *mut CLSID,
    ) -> HRESULT,
    fn RegisterPin(
        Filter: CLSID,
        Name: LPCWSTR,
        bRendered: BOOL,
        bOutput: BOOL,
        bZero: BOOL,
        bMany: BOOL,
        ConnectsToFilter: CLSID,
        ConnectsToPin: LPCWSTR,
    ) -> HRESULT,
    fn RegisterPinType(
        clsFilter: CLSID,
        strName: LPCWSTR,
        clsMajorType: CLSID,
        clsSubType: CLSID,
    ) -> HRESULT,
    fn UnregisterFilter(
        Filter: CLSID,
    ) -> HRESULT,
    fn UnregisterFilterInstance(
        MRId: CLSID,
    ) -> HRESULT,
    fn UnregisterPin(
        Filter: CLSID,
        Name: LPCWSTR,
    ) -> HRESULT,
    fn EnumMatchingFilters(
        ppenum: *mut *mut IEnumRegFilters,
        dwMerit: DWORD,
        bInputNeeded: BOOL,
        clsInMaj: CLSID,
        clsInSub: CLSID,
        bRender: BOOL,
        bOututNeeded: BOOL,
        clsOutMaj: CLSID,
        clsOutSub: CLSID,
    ) -> HRESULT,
}}
STRUCT!{struct REGPINTYPES {
    clsMajorType: *mut CLSID,
    clsMinorType: *mut CLSID,
}}
STRUCT!{struct REGFILTERPINS {
    strName: LPWSTR,
    bRendered: BOOL,
    bOutput: BOOL,
    bZero: BOOL,
    bMany: BOOL,
    clsConnectsToFilter: *mut CLSID,
    strConnectsToPin: *mut WCHAR,
    nMediaTypes: UINT,
    lpMediaType: *mut REGPINTYPES,
}}
STRUCT!{struct REGPINMEDIUM {
    clsMedium: CLSID,
    dw1: DWORD,
    dw2: DWORD,
}}
ENUM!{enum __MIDL___MIDL_itf_strmif_0001_0022_0001 {
    REG_PINFLAG_B_ZERO = 1,
    REG_PINFLAG_B_RENDERER = 2,
    REG_PINFLAG_B_MANY = 4,
    REG_PINFLAG_B_OUTPUT = 8,
}}
STRUCT!{struct REGFILTERPINS2 {
    dwFlags: DWORD,
    cInstances: UINT,
    nMediaTypes: UINT,
    lpMediaType: *mut REGPINTYPES,
    nMediums: UINT,
    lpMedium: *mut REGPINMEDIUM,
    clsPinCategory: *mut CLSID,
}}
STRUCT!{struct REGFILTER2_s1 {
    cPins: ULONG,
    rgPins: *mut REGFILTERPINS,
}}
STRUCT!{struct REGFILTER2_s2 {
    cPins2: ULONG,
    rgPins2: *mut REGFILTERPINS2,
}}
UNION!{union REGFILTER2_u {
    [u64; 2],
    u1 u1_mut: REGFILTER2_s1,
    u2 u2_mut: REGFILTER2_s2,
}}
STRUCT!{struct REGFILTER2 {
    dwVersion: DWORD,
    dwMerit: DWORD,
    u: REGFILTER2_u,
}}
RIDL!{#[uuid(0xb79bb0b0, 0x33c1, 0x11d1, 0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IFilterMapper2(IFilterMapper2Vtbl): IUnknown(IUnknownVtbl) {
    fn CreateCategory(
        clsidCategory: REFCLSID,
        dwCategoryMerit: DWORD,
        Description: LPCWSTR,
    ) -> HRESULT,
    fn UnregisterFilter(
        pclsidCategory: *const CLSID,
        szInstance: LPCWSTR,
        Filter: REFCLSID,
    ) -> HRESULT,
    fn RegisterFilter(
        clsidFilter: REFCLSID,
        Name: LPCWSTR,
        ppMoniker: *mut *mut IMoniker,
        pclsidCategory: *const CLSID,
        szInstance: LPCOLESTR,
        prf2: *const REGFILTER2,
    ) -> HRESULT,
    fn EnumMatchingFilters(
        ppenum: *mut *mut IEnumMoniker,
        dwFlags: DWORD,
        bExactMatch: BOOL,
        dwMerit: DWORD,
        bInputNeeded: BOOL,
        cInputTypes: DWORD,
        pInputTypes: *mut GUID,
        pMedIn: *const REGPINMEDIUM,
        pPinCategoryIn: *const CLSID,
        bRender: BOOL,
        bOutputNeeded: BOOL,
        cOutputTypes: DWORD,
        pOutputTypes: *mut GUID,
        pMedOut: *const REGPINMEDIUM,
        pPinCategoryOut: *const CLSID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb79bb0b1, 0x33c1, 0x11d1, 0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IFilterMapper3(IFilterMapper3Vtbl): IFilterMapper2(IFilterMapper2Vtbl) {
    fn GetICreateDevEnum(
        ppenum: *mut *mut ICreateDevEnum,
    ) -> HRESULT,
}}
ENUM!{enum QualityMessageType {
    Famine = 0,
    Flood = 1,
}}
STRUCT!{struct Quality {
    type_: QualityMessageType,
    Proportion: c_long,
    Late: REFERENCE_TIME,
    TimeStamp: REFERENCE_TIME,
}}
pub type PQUALITYCONTROL = *mut IQualityControl;
RIDL!{#[uuid(0x56a868a5, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IQualityControl(IQualityControlVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(
        pSelf: *const IBaseFilter,
        q: Quality,
    ) -> HRESULT,
    fn SetSink(
        piqc: *const IQualityControl,
    ) -> HRESULT,
}}
ENUM!{enum __MIDL___MIDL_itf_strmif_0001_0025_0001 {
    CK_NOCOLORKEY = 0,
    CK_INDEX = 1,
    CK_RGB = 2,
}}
STRUCT!{struct COLORKEY {
    KeyType: u32,
    PaletteIndex: u32,
    LowColorValue: u32,
    HighColorValue: u32,
}}
ENUM!{enum __MIDL___MIDL_itf_strmif_0001_0025_0002 {
    ADVISE_NONE = 0,
    ADVISE_CLIPPING = 1,
    ADVISE_PALETTE = 2,
    ADVISE_COLORKEY = 4,
    ADVISE_POSITION = 8,
    ADVISE_DISPLAY_CHANGE = 16,
}}
pub const ADVISE_ALL: DWORD = ADVISE_CLIPPING | ADVISE_PALETTE | ADVISE_COLORKEY | ADVISE_POSITION;
pub const ADVISE_ALL2: DWORD = ADVISE_ALL | ADVISE_DISPLAY_CHANGE;
RIDL!{#[uuid(0x56a868a0, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IOverlayNotify(IOverlayNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn OnPaletteChange(
        dwColors: DWORD,
        pPalette: *const PALETTEENTRY,
    ) -> HRESULT,
    fn OnClipChange(
        pSourceRect: *const RECT,
        pDestinationRect: *const RECT,
        pRgnData: *const RGNDATA,
    ) -> HRESULT,
    fn OnColorKeyChange(
        pColorKey: *const COLORKEY,
    ) -> HRESULT,
    fn OnPositionChange(
        pSourceRect: *const RECT,
        pDestinationRect: *const RECT,
    ) -> HRESULT,
}}
pub type POVERLAYNOTIFY = *mut IOverlayNotify;
RIDL!{#[uuid(0x680efa10, 0xd535, 0x11d1, 0x87, 0xc8, 0x00, 0xa0, 0xc9, 0x22, 0x31, 0x96)]
interface IOverlayNotify2(IOverlayNotify2Vtbl): IOverlayNotify(IOverlayNotifyVtbl) {
    fn OnDisplayChange(
        hMonitor: HMONITOR,
    ) -> HRESULT,
}}
pub type POVERLAYNOTIFY2 = *mut IOverlayNotify2;
RIDL!{#[uuid(0x56a868a1, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IOverlay(IOverlayVtbl): IUnknown(IUnknownVtbl) {
    fn GetPalette(
        pdwColors: *mut DWORD,
        ppPalette: *mut *mut PALETTEENTRY,
    ) -> HRESULT,
    fn SetPalette(
        dwColors: DWORD,
        pPalette: *const PALETTEENTRY,
    ) -> HRESULT,
    fn GetDefaultColorKey(
        pColorKey: *mut COLORKEY,
    ) -> HRESULT,
    fn GetColorKey(
        pColorKey: *mut COLORKEY,
    ) -> HRESULT,
    fn SetColorKey(
        pColorKey: *mut COLORKEY,
    ) -> HRESULT,
    fn GetWindowHandle(
        pHwnd: *mut HWND,
    ) -> HRESULT,
    fn GetClipList(
        pSourceRect: *mut RECT,
        pDestinationRect: *mut RECT,
        ppRgnData: *mut *mut RGNDATA,
    ) -> HRESULT,
    fn GetVideoPosition(
        pSourceRect: *mut RECT,
        pDestinationRect: *mut RECT,
    ) -> HRESULT,
    fn Advise(
        pOverlayNotify: *const IOverlayNotify,
        dwInterests: DWORD,
    ) -> HRESULT,
    fn Unadvise(
    ) -> HRESULT,
}}
pub type POVERLAY = *mut IOverlay;
RIDL!{#[uuid(0x56a868a2, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IMediaEventSink(IMediaEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn Notify(
        EventCode: c_long,
        EventParam1: LONG_PTR,
        EventParam2: LONG_PTR,
    ) -> HRESULT,
}}
pub type PMEDIAEVENTSINK = *mut IMediaEventSink;
RIDL!{#[uuid(0x56a868a6, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IFileSourceFilter(IFileSourceFilterVtbl): IUnknown(IUnknownVtbl) {
    fn Load(
        pszFileName: LPCOLESTR,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetCurFile(
        ppszFileName: *mut LPOLESTR,
        pmt: *mut AM_MEDIA_TYPE,
    ) -> HRESULT,
}}
pub type PFILTERFILESOURCE = *mut IFileSourceFilter;
RIDL!{#[uuid(0xa2104830, 0x7c70, 0x11cf, 0x8b, 0xce, 0x00, 0xaa, 0x00, 0xa3, 0xf1, 0xa6)]
interface IFileSinkFilter(IFileSinkFilterVtbl): IUnknown(IUnknownVtbl) {
    fn SetFileName(
        pszFileName: LPCOLESTR,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetCurFile(
        ppszFileName: *mut LPOLESTR,
        pmt: *mut AM_MEDIA_TYPE,
    ) -> HRESULT,
}}
pub type PFILTERFILESINK = *mut IFileSinkFilter;
RIDL!{#[uuid(0x00855b90, 0xce1b, 0x11d0, 0xbd, 0x4f, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IFileSinkFilter2(IFileSinkFilter2Vtbl): IFileSinkFilter(IFileSinkFilterVtbl) {
    fn SetMode(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetMode(
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
}}
pub type PFILESINKFILTER2 = *mut IFileSinkFilter2;
ENUM!{enum AM_FILESINK_FLAGS {
    AM_FILE_OVERWRITE = 1,
}}
RIDL!{#[uuid(0x56a868a9, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IGraphBuilder(IGraphBuilderVtbl): IFilterGraph(IFilterGraphVtbl) {
    fn Connect(
        ppinOut: *const IPin,
        ppinIn: *const IPin,
    ) -> HRESULT,
    fn Render(
        ppinOut: *const IPin,
    ) -> HRESULT,
    fn RenderFile(
        lpcwstrFile: LPCWSTR,
        lpcwstrPlayList: LPCWSTR,
    ) -> HRESULT,
    fn AddSourceFilter(
        lpcwstrFileName: LPCWSTR,
        lpcwstrFilterName: LPCWSTR,
        ppFilter: *mut *mut IBaseFilter,
    ) -> HRESULT,
    fn SetLogFile(
        hFile: DWORD_PTR,
    ) -> HRESULT,
    fn Abort(
    ) -> HRESULT,
    fn ShouldOperationContinue(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbf87b6e0, 0x8c27, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface ICaptureGraphBuilder(ICaptureGraphBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn SetFiltergraph(
        pfg: *const IGraphBuilder,
    ) -> HRESULT,
    fn GetFiltergraph(
        ppfg: *mut *mut IGraphBuilder,
    ) -> HRESULT,
    fn SetOutputFileName(
        pType: *const GUID,
        lpstrFile: LPCOLESTR,
        ppf: *mut *mut IBaseFilter,
        ppSink: *mut *mut IFileSinkFilter,
    ) -> HRESULT,
    fn RemoteFindInterface(
        pCategory: *const GUID,
        pf: *const IBaseFilter,
        riid: REFIID,
        ppint: *mut *mut c_void,
    ) -> HRESULT,
    fn RenderStream(
        pCategory: *const GUID,
        pSource: *const IUnknown,
        pfCompressor: *const IBaseFilter,
        pfRenderer: *const IBaseFilter,
    ) -> HRESULT,
    fn ControlStream(
        pCategory: *const GUID,
        pFilter: *const IBaseFilter,
        pstart: *const REFERENCE_TIME,
        pStop: *const REFERENCE_TIME,
        wStartCookie: WORD,
        wStopCookie: WORD,
    ) -> HRESULT,
    fn AllocCapFile(
        lpstr: LPCOLESTR,
        dwlSize: DWORDLONG,
    ) -> HRESULT,
    fn CopyCaptureFile(
        lpwstrOld: LPOLESTR,
        lpwstrNew: LPOLESTR,
        fAllowEscAbort: c_int,
        pCallback: *const IAMCopyCaptureFileProgress,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x670d1d20, 0xa068, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface IAMCopyCaptureFileProgress(IAMCopyCaptureFileProgressVtbl): IUnknown(IUnknownVtbl) {
    fn Progress(
        iProgress: c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x93e5a4e0, 0x2d50, 0x11d2, 0xab, 0xfa, 0x00, 0xa0, 0xc9, 0xc6, 0xe3, 0x8d)]
interface ICaptureGraphBuilder2(ICaptureGraphBuilder2Vtbl): IUnknown(IUnknownVtbl) {
    fn SetFiltergraph(
        pfg: *const IGraphBuilder,
    ) -> HRESULT,
    fn GetFiltergraph(
        ppfg: *mut *mut IGraphBuilder,
    ) -> HRESULT,
    fn SetOutputFileName(
        pType: *const GUID,
        lpstrFile: LPCOLESTR,
        ppf: *mut *mut IBaseFilter,
        ppSink: *mut *mut IFileSinkFilter,
    ) -> HRESULT,
    fn RemoteFindInterface(
        pCategory: *const GUID,
        pType: *const GUID,
        pf: *const IBaseFilter,
        riid: REFIID,
        ppint: *mut *mut c_void,
    ) -> HRESULT,
    fn RenderStream(
        pCategory: *const GUID,
        pType: *const GUID,
        pSource: *const IUnknown,
        pfCompressor: *const IBaseFilter,
        pfRenderer: *const IBaseFilter,
    ) -> HRESULT,
    fn ControlStream(
        pCategory: *const GUID,
        pType: *const GUID,
        pFilter: *const IBaseFilter,
        pstart: *const REFERENCE_TIME,
        pStop: *const REFERENCE_TIME,
        wStartCookie: WORD,
        wStopCookie: WORD,
    ) -> HRESULT,
    fn AllocCapFile(
        lpstr: LPCOLESTR,
        dwlSize: DWORDLONG,
    ) -> HRESULT,
    fn CopyCaptureFile(
        lpwstrOld: LPOLESTR,
        lpwstrNew: LPOLESTR,
        fAllowEscAbort: c_int,
        pCallback: *const IAMCopyCaptureFileProgress,
    ) -> HRESULT,
    fn FindPin(
        pSource: *const IUnknown,
        pindir: PIN_DIRECTION,
        pCategory: *const GUID,
        pType: *const GUID,
        fUnconnected: BOOL,
        num: c_int,
        ppPin: *mut *mut IPin,
    ) -> HRESULT,
}}
ENUM!{enum AM_RENDEREXFLAGS {
    AM_RENDEREX_RENDERTOEXISTINGRENDERERS = 1,
}}
RIDL!{#[uuid(0x36b73882, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface IFilterGraph2(IFilterGraph2Vtbl): IGraphBuilder(IGraphBuilderVtbl) {
    fn AddSourceFilterForMoniker(
        pMoniker: *const IMoniker,
        pCtx: *const IBindCtx,
        lpcwstrFilterName: LPCWSTR,
        ppFilter: *mut *mut IBaseFilter,
    ) -> HRESULT,
    fn ReconnectEx(
        pPin: *const IPin,
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn RenderEx(
        ppinOut: *const IPin,
        dwFlags: DWORD,
        pvContext: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaaf38154, 0xb80b, 0x422f, 0x91, 0xe6, 0xb6, 0x64, 0x67, 0x50, 0x9a, 0x07)]
interface IFilterGraph3(IFilterGraph3Vtbl): IFilterGraph2(IFilterGraph2Vtbl) {
    fn SetSyncSourceEx(
        pClockForMostOfFilterGraph: *const IReferenceClock,
        pClockForFilter: *const IReferenceClock,
        pFilter: *const IBaseFilter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868bf, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IStreamBuilder(IStreamBuilderVtbl): IUnknown(IUnknownVtbl) {
    fn Render(
        ppinOut: *const IPin,
        pGraph: *const IGraphBuilder,
    ) -> HRESULT,
    fn Backout(
        ppinOut: *const IPin,
        pGraph: *const IGraphBuilder,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868aa, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IAsyncReader(IAsyncReaderVtbl): IUnknown(IUnknownVtbl) {
    fn RequestAllocator(
        pPreferred: *const IMemAllocator,
        pProps: *const ALLOCATOR_PROPERTIES,
        ppActual: *mut *mut IMemAllocator,
    ) -> HRESULT,
    fn Request(
        pSample: *const IMediaSample,
        dwUser: DWORD_PTR,
    ) -> HRESULT,
    fn WaitForNext(
        dwTimeout: DWORD,
        ppSample: *mut *mut IMediaSample,
        pdwUser: *mut DWORD_PTR,
    ) -> HRESULT,
    fn SyncReadAligned(
        pSample: *const IMediaSample,
    ) -> HRESULT,
    fn SyncRead(
        llPosition: LONGLONG,
        lLength: LONG,
        pBuffer: *mut BYTE,
    ) -> HRESULT,
    fn Length(
        pTotal: *mut LONGLONG,
        pAvailable: *mut LONGLONG,
    ) -> HRESULT,
    fn BeginFlush(
    ) -> HRESULT,
    fn EndFlush(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868ab, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IGraphVersion(IGraphVersionVtbl): IUnknown(IUnknownVtbl) {
    fn QueryVersion(
        pVersion: *mut LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868ad, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IResourceConsumer(IResourceConsumerVtbl): IUnknown(IUnknownVtbl) {
    fn AcquireResource(
        idResource: LONG,
    ) -> HRESULT,
    fn ReleaseResource(
        idResource: LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868ac, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IResourceManager(IResourceManagerVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        pName: LPCWSTR,
        cResource: LONG,
        plToken: *mut LONG,
    ) -> HRESULT,
    fn RegisterGroup(
        pName: LPCWSTR,
        cResource: LONG,
        palTokens: *const LONG,
        plToken: *mut LONG,
    ) -> HRESULT,
    fn RequestResource(
        idResource: LONG,
        pFocusObject: *const IUnknown,
        pConsumer: *const IResourceConsumer,
    ) -> HRESULT,
    fn NotifyAcquire(
        idResource: LONG,
        pConsumer: *const IResourceConsumer,
        hr: HRESULT,
    ) -> HRESULT,
    fn NotifyRelease(
        idResource: LONG,
        pConsumer: *const IResourceConsumer,
        bStillWant: BOOL,
    ) -> HRESULT,
    fn CancelRequest(
        idResource: LONG,
        pConsumer: *const IResourceConsumer,
    ) -> HRESULT,
    fn SetFocus(
        pFocusObject: *const IUnknown,
    ) -> HRESULT,
    fn ReleaseFocus(
        pFocusObject: *const IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868af, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IDistributorNotify(IDistributorNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn Stop(
    ) -> HRESULT,
    fn Pause(
    ) -> HRESULT,
    fn Run(
        tStart: i64,
    ) -> HRESULT,
    fn SetSyncSource(
        pClock: *const IReferenceClock,
    ) -> HRESULT,
    fn NotifyGraphChange(
    ) -> HRESULT,
}}
ENUM!{enum AM_STREAM_INFO_FLAGS {
    AM_STREAM_INFO_START_DEFINED = 1,
    AM_STREAM_INFO_STOP_DEFINED = 2,
    AM_STREAM_INFO_DISCARDING = 4,
    AM_STREAM_INFO_STOP_SEND_EXTRA = 16,
}}
STRUCT!{struct AM_STREAM_INFO {
    tStart: REFERENCE_TIME,
    tStop: REFERENCE_TIME,
    dwStartCookie: DWORD,
    dwStopCookie: DWORD,
    dwFlags: DWORD,
}}
RIDL!{#[uuid(0x36b73881, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface IAMStreamControl(IAMStreamControlVtbl): IUnknown(IUnknownVtbl) {
    fn StartAt(
        ptStart: *const REFERENCE_TIME,
        dwCookie: DWORD,
    ) -> HRESULT,
    fn StopAt(
        ptStop: *const REFERENCE_TIME,
        bSendExtra: BOOL,
        dwCookie: DWORD,
    ) -> HRESULT,
    fn GetInfo(
        pInfo: *mut AM_STREAM_INFO,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x36b73883, 0xc2c8, 0x11cf, 0x8b, 0x46, 0x00, 0x80, 0x5f, 0x6c, 0xef, 0x60)]
interface ISeekingPassThru(ISeekingPassThruVtbl): IUnknown(IUnknownVtbl) {
    fn Init(
        bSupportRendering: BOOL,
        pPin: *const IPin,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc6e13340, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMStreamConfig(IAMStreamConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetFormat(
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetFormat(
        ppmt: *mut *mut AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn GetNumberOfCapabilities(
        piCount: *mut c_int,
        piSize: *mut c_int,
    ) -> HRESULT,
    fn GetStreamCaps(
        iIndex: c_int,
        ppmt: *mut *mut AM_MEDIA_TYPE,
        pSCC: *mut BYTE,
    ) -> HRESULT,
}}
ENUM!{enum InterleavingMode {
    INTERLEAVE_NONE = 0,
    INTERLEAVE_CAPTURE = 1,
    INTERLEAVE_FULL = 2,
    INTERLEAVE_NONE_BUFFERED = 3,
}}
RIDL!{#[uuid(0xbee3d220, 0x157b, 0x11d0, 0xbd, 0x23, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IConfigInterleaving(IConfigInterleavingVtbl): IUnknown(IUnknownVtbl) {
    fn put_Mode(
        mode: InterleavingMode,
    ) -> HRESULT,
    fn get_Mode(
        pMode: *mut InterleavingMode,
    ) -> HRESULT,
    fn put_Interleaving(
        prtInterleave: *const REFERENCE_TIME,
        prtPreroll: *const REFERENCE_TIME,
    ) -> HRESULT,
    fn get_Interleaving(
        prtInterleave: *mut REFERENCE_TIME,
        prtPreroll: *mut REFERENCE_TIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5acd6aa0, 0xf482, 0x11ce, 0x8b, 0x67, 0x00, 0xaa, 0x00, 0xa3, 0xf1, 0xa6)]
interface IConfigAviMux(IConfigAviMuxVtbl): IUnknown(IUnknownVtbl) {
    fn SetMasterStream(
        IStream: LONG,
    ) -> HRESULT,
    fn GetMasterStream(
        pStream: *mut LONG,
    ) -> HRESULT,
    fn SetOutputCompatibilityIndex(
        fOldIndex: BOOL,
    ) -> HRESULT,
    fn GetOutputCompatibilityIndex(
        pfOldIndex: *mut BOOL,
    ) -> HRESULT,
}}
ENUM!{enum CompressionCaps {
    CompressionCaps_CanQuality = 1,
    CompressionCaps_CanCrunch = 2,
    CompressionCaps_CanKeyFrame = 4,
    CompressionCaps_CanBFrame = 8,
    CompressionCaps_CanWindow = 16,
}}
RIDL!{#[uuid(0xc6e13343, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMVideoCompression(IAMVideoCompressionVtbl): IUnknown(IUnknownVtbl) {
    fn put_KeyFrameRate(
        KeyFrameRate: c_long,
    ) -> HRESULT,
    fn get_KeyFrameRate(
        pKeyFrameRate: *mut c_long,
    ) -> HRESULT,
    fn put_PFramesPerKeyFrame(
        PFramesPerKeyFrame: c_long,
    ) -> HRESULT,
    fn get_PFramesPerKeyFrame(
        pPFramesPerKeyFrame: *mut c_long,
    ) -> HRESULT,
    fn put_Quality(
        Quality: c_double,
    ) -> HRESULT,
    fn get_Quality(
        pQuality: *mut c_double,
    ) -> HRESULT,
    fn put_WindowSize(
        WindowSize: DWORDLONG,
    ) -> HRESULT,
    fn get_WindowSize(
        pWindowSize: *mut DWORDLONG,
    ) -> HRESULT,
    fn GetInfo(
        pszVersion: LPWSTR,
        pcbVersion: *mut c_int,
        pszDescription: LPWSTR,
        pcbDescription: *mut c_int,
        pDefaultKeyFrameRate: *mut c_long,
        pDefaultPFramesPerKey: *mut c_long,
        pDefaultQuality: *mut c_double,
        pCapabilities: *mut c_long,
    ) -> HRESULT,
    fn OverrideKeyFrame(
        FrameNumber: c_long,
    ) -> HRESULT,
    fn OverrideFrameSize(
        FrameNumber: c_long,
        Size: c_long,
    ) -> HRESULT,
}}
ENUM!{enum VfwCaptureDialogs {
    VfwCaptureDialog_Source = 1,
    VfwCaptureDialog_Format = 2,
    VfwCaptureDialog_Display = 4,
}} 
ENUM!{enum VfwCompressDialogs {
    VfwCompressDialog_Config = 1,
    VfwCompressDialog_About = 2,
    VfwCompressDialog_QueryConfig = 4,
    VfwCompressDialog_QueryAbout = 8,
}}
RIDL!{#[uuid(0xd8d715a0, 0x6e5e, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface IAMVfwCaptureDialogs(IAMVfwCaptureDialogsVtbl): IUnknown(IUnknownVtbl) {
    fn HasDialog(
        iDialog: c_int,
    ) -> HRESULT,
    fn ShowDialog(
        iDialog: c_int,
        hwnd: HWND,
    ) -> HRESULT,
    fn SendDriverMessage(
        iDialog: c_int,
        uMsg: c_int,
        dw1: c_long,
        dw2: c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd8d715a3, 0x6e5e, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface IAMVfwCompressDialogs(IAMVfwCompressDialogsVtbl): IUnknown(IUnknownVtbl) {
    fn ShowDialog(
        iDialog: c_int,
        hwnd: HWND,
    ) -> HRESULT,
    fn GetState(
        pState: LPVOID,
        pcbState: *mut c_int,
    ) -> HRESULT,
    fn SetState(
        pState: LPVOID,
        cbState: c_int,
    ) -> HRESULT,
    fn SendDriverMessage(
        uMsg: c_int,
        dw1: c_long,
        dw2: c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc6e13344, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMDroppedFrames(IAMDroppedFramesVtbl): IUnknown(IUnknownVtbl) {
    fn GetNumDropped(
        plDropped: *mut c_long,
    ) -> HRESULT,
    fn GetNumNotDropped(
        plNotDropped: *mut c_long,
    ) -> HRESULT,
    fn GetDroppedInfo(
        lSize: c_long,
        plArray: *mut c_long,
        plNumCopied: *mut c_long,
    ) -> HRESULT,
    fn GetAverageFrameSize(
        plAverageSize: *mut c_long,
    ) -> HRESULT,
}}
pub const AMF_AUTOMATICGAIN: c_double = -1.0;
RIDL!{#[uuid(0x54c39221, 0x8380, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface IAMAudioInputMixer(IAMAudioInputMixerVtbl): IUnknown(IUnknownVtbl) {
    fn put_Enable(
        fEnable: BOOL,
    ) -> HRESULT,
    fn get_Enable(
        pfEnable: *mut BOOL,
    ) -> HRESULT,
    fn put_Mono(
        fMono: BOOL,
    ) -> HRESULT,
    fn get_Mono(
        pfMono: *mut BOOL,
    ) -> HRESULT,
    fn put_MixLevel(
        Level: c_double,
    ) -> HRESULT,
    fn get_MixLevel(
        pLevel: *mut c_double,
    ) -> HRESULT,
    fn put_Pan(
        Pan: c_double,
    ) -> HRESULT,
    fn get_Pan(
        pPan: *mut c_double,
    ) -> HRESULT,
    fn put_Loudness(
        fLoudness: BOOL,
    ) -> HRESULT,
    fn get_Loudness(
        pfLoudness: *mut BOOL,
    ) -> HRESULT,
    fn put_Treble(
        Treble: c_double,
    ) -> HRESULT,
    fn get_Treble(
        pTreble: *mut c_double,
    ) -> HRESULT,
    fn get_TrebleRange(
        pRange: *mut c_double,
    ) -> HRESULT,
    fn put_Bass(
        Bass: c_double,
    ) -> HRESULT,
    fn get_Bass(
        pBass: *mut c_double,
    ) -> HRESULT,
    fn get_BassRange(
        pRange: *mut c_double,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56ed71a0, 0xaf5f, 0x11d0, 0xb3, 0xf0, 0x00, 0xaa, 0x00, 0x37, 0x61, 0xc5)]
interface IAMBufferNegotiation(IAMBufferNegotiationVtbl): IUnknown(IUnknownVtbl) {
    fn SuggestAllocatorProperties(
        pprop: *const ALLOCATOR_PROPERTIES,
    ) -> HRESULT,
    fn GetAllocatorProperties(
        pprop: *mut ALLOCATOR_PROPERTIES,
    ) -> HRESULT,
}}
ENUM!{enum AnalogVideoStandard {
    AnalogVideo_None = 0,
    AnalogVideo_NTSC_M = 1,
    AnalogVideo_NTSC_M_J = 2,
    AnalogVideo_NTSC_433 = 4,
    AnalogVideo_PAL_B = 16,
    AnalogVideo_PAL_D = 32,
    AnalogVideo_PAL_G = 64,
    AnalogVideo_PAL_H = 128,
    AnalogVideo_PAL_I = 256,
    AnalogVideo_PAL_M = 512,
    AnalogVideo_PAL_N = 1024,
    AnalogVideo_PAL_60 = 2048,
    AnalogVideo_SECAM_B = 4096,
    AnalogVideo_SECAM_D = 8192,
    AnalogVideo_SECAM_G = 16384,
    AnalogVideo_SECAM_H = 32768,
    AnalogVideo_SECAM_K = 65536,
    AnalogVideo_SECAM_K1 = 131072,
    AnalogVideo_SECAM_L = 262144,
    AnalogVideo_SECAM_L1 = 524288,
    AnalogVideo_PAL_N_COMBO = 1048576,
    AnalogVideoMask_MCE_NTSC = 1052167,
    AnalogVideoMask_MCE_PAL = 496,
    AnalogVideoMask_MCE_SECAM = 1044480,
}}
ENUM!{enum TunerInputType {
    TunerInputCable = 0,
    TunerInputAntenna = 1,
}}
pub const AnalogVideo_NTSC_Mask: DWORD = 0x00000007;
pub const AnalogVideo_PAL_Mask: DWORD = 0x00100FF0;
pub const AnalogVideo_SECAM_Mask: DWORD = 0x000FF000;
ENUM!{enum VideoCopyProtectionType {
    VideoCopyProtectionMacrovisionBasic = 0,
    VideoCopyProtectionMacrovisionCBI = 1,
}}
ENUM!{enum PhysicalConnectorType {
    PhysConn_Video_Tuner = 1,
    PhysConn_Video_Composite = 2,
    PhysConn_Video_SVideo = 3,
    PhysConn_Video_RGB = 4,
    PhysConn_Video_YRYBY = 5,
    PhysConn_Video_SerialDigital = 6,
    PhysConn_Video_ParallelDigital = 7,
    PhysConn_Video_SCSI = 8,
    PhysConn_Video_AUX = 9,
    PhysConn_Video_1394 = 10,
    PhysConn_Video_USB = 11,
    PhysConn_Video_VideoDecoder = 12,
    PhysConn_Video_VideoEncoder = 13,
    PhysConn_Video_SCART = 14,
    PhysConn_Video_Black = 15,
    PhysConn_Audio_Tuner = 4096,
    PhysConn_Audio_Line = 4097,
    PhysConn_Audio_Mic = 4098,
    PhysConn_Audio_AESDigital = 4099,
    PhysConn_Audio_SPDIFDigital = 4100,
    PhysConn_Audio_SCSI = 4101,
    PhysConn_Audio_AUX = 4102,
    PhysConn_Audio_1394 = 4103,
    PhysConn_Audio_USB = 4104,
    PhysConn_Audio_AudioDecoder = 4105,
}}
RIDL!{#[uuid(0xc6e13350, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMAnalogVideoDecoder(IAMAnalogVideoDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn get_AvailableTVFormats(
        lAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn put_TVFormat(
        lAnalogVideoStandard: c_long,
    ) -> HRESULT,
    fn get_TVFormat(
        plAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn get_HorizontalLocked(
        plLocked: *mut c_long,
    ) -> HRESULT,
    fn put_VCRHorizontalLocking(
        lVCRHorizontalLocking: c_long,
    ) -> HRESULT,
    fn get_VCRHorizontalLocking(
        plVCRHorizontalLocking: *mut c_long,
    ) -> HRESULT,
    fn get_NumberOfLines(
        plNumberOfLines: *mut c_long,
    ) -> HRESULT,
    fn put_OutputEnable(
        lOutputEnable: c_long,
    ) -> HRESULT,
    fn get_OutputEnable(
        plOutputEnable: *mut c_long,
    ) -> HRESULT,
}}
ENUM!{enum VideoProcAmpProperty {
    VideoProcAmp_Brightness = 0,
    VideoProcAmp_Contrast = 1,
    VideoProcAmp_Hue = 2,
    VideoProcAmp_Saturation = 3,
    VideoProcAmp_Sharpness = 4,
    VideoProcAmp_Gamma = 5,
    VideoProcAmp_ColorEnable = 6,
    VideoProcAmp_WhiteBalance = 7,
    VideoProcAmp_BacklightCompensation = 8,
    VideoProcAmp_Gain = 9,
}}
ENUM!{enum VideoProcAmpFlags {
    VideoProcAmp_Flags_Auto = 1,
    VideoProcAmp_Flags_Manual = 2,
}}
RIDL!{#[uuid(0xc6e13360, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMVideoProcAmp(IAMVideoProcAmpVtbl): IUnknown(IUnknownVtbl) {
    fn GetRange(
        Property: c_long,
        pMin: *mut c_long,
        pMax: *mut c_long,
        pSteppingDelta: *mut c_long,
        pDefault: *mut c_long,
        pCapsFlags: *mut c_long,
    ) -> HRESULT,
    fn Set(
        Property: c_long,
        lValue: c_long,
        Flags: c_long,
    ) -> HRESULT,
    fn Get(
        Property: c_long,
        lValue: *mut c_long,
        Flags: *mut c_long,
    ) -> HRESULT,
}}
ENUM!{enum CameraControlProperty {
    CameraControl_Pan = 0,
    CameraControl_Tilt = 1,
    CameraControl_Roll = 2,
    CameraControl_Zoom = 3,
    CameraControl_Exposure = 4,
    CameraControl_Iris = 5,
    CameraControl_Focus = 6,
}}
ENUM!{enum CameraControlFlags {
    CameraControl_Flags_Auto = 1,
    CameraControl_Flags_Manual = 2,
}}
RIDL!{#[uuid(0xc6e13370, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMCameraControl(IAMCameraControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetRange(
        Property: c_long,
        pMin: *mut c_long,
        pMax: *mut c_long,
        pSteppingDelta: *mut c_long,
        pDefault: *mut c_long,
        pCapsFlags: *mut c_long,
    ) -> HRESULT,
    fn Set(
        Property: c_long,
        lValue: c_long,
        Flags: c_long,
    ) -> HRESULT,
    fn Get(
        Property: c_long,
        lValue: *mut c_long,
        Flags: *mut c_long,
    ) -> HRESULT,
}}
ENUM!{enum VideoControlFlags {
    VideoControlFlag_FlipHorizontal = 1,
    VideoControlFlag_FlipVertical = 2,
    VideoControlFlag_ExternalTriggerEnable = 4,
    VideoControlFlag_Trigger = 8,
}}
RIDL!{#[uuid(0x6a2e0670, 0x28e4, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMVideoControl(IAMVideoControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(
        pPin: *const IPin,
        pCapsFlags: *mut c_long,
    ) -> HRESULT,
    fn SetMode(
        pPin: *const IPin,
        mode: c_long,
    ) -> HRESULT,
    fn GetMode(
        pPin: *const IPin,
        mode: *mut c_long,
    ) -> HRESULT,
    fn GetCurrentActualFrameRate(
        pPin: *const IPin,
        ActualFrameRate: *mut LONGLONG,
    ) -> HRESULT,
    fn GetMaxAvailableFrameRate(
        pPin: *const IPin,
        iIndex: c_long,
        Dimensions: SIZE,
        MaxAvailableFrameRate: *mut LONGLONG,
    ) -> HRESULT,
    fn GetFrameRateList(
        pPin: *const IPin,
        iIndex: c_long,
        Dimensions: SIZE,
        ListSize: *mut c_long,
        FrameRates: *mut *mut LONGLONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc6e13380, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMCrossbar(IAMCrossbarVtbl): IUnknown(IUnknownVtbl) {
    fn get_PinCounts(
        OutputPinCount: *mut c_long,
        InputPinCount: *mut c_long,
    ) -> HRESULT,
    fn CanRoute(
        OutputPinIndex: c_long,
        InputPinIndex: c_long,
    ) -> HRESULT,
    fn Route(
        OutputPinIndex: c_long,
        InputPinIndex: c_long,
    ) -> HRESULT,
    fn get_IsRoutedTo(
        OutputPinIndex: c_long,
        InputPinIndex: *mut c_long,
    ) -> HRESULT,
    fn get_CrossbarPinInfo(
        IsInputPin: c_long,
        PinIndex: c_long,
        PinIndexRelated: *mut c_long,
        PhysicalType: *mut c_long,
    ) -> HRESULT,
}}
ENUM!{enum AMTunerSubChannel {
    AMTUNER_SUBCHAN_NO_TUNE = 0xfffffffe,
    AMTUNER_SUBCHAN_DEFAULT = 0xffffffff,
}}
ENUM!{enum AMTunerSignalStrength {
    AMTUNER_HASNOSIGNALSTRENGTH = 0xffffffff,
    AMTUNER_NOSIGNAL = 0,
    AMTUNER_SIGNALPRESENT = 1,
}}
ENUM!{enum AMTunerModeType {
    AMTUNER_MODE_DEFAULT = 0,
    AMTUNER_MODE_TV = 1,
    AMTUNER_MODE_FM_RADIO = 2,
    AMTUNER_MODE_AM_RADIO = 4,
    AMTUNER_MODE_DSS = 8,
}}
ENUM!{enum AMTunerEventType {
    AMTUNER_EVENT_CHANGED = 1,
}}
RIDL!{#[uuid(0x211a8761, 0x03ac, 0x11d1, 0x8d, 0x13, 0x00, 0xaa, 0x00, 0xbd, 0x83, 0x39)]
interface IAMTuner(IAMTunerVtbl): IUnknown(IUnknownVtbl) {
    fn put_Channel(
        lChannel: c_long,
        lVideoSubChannel: c_long,
        lAudioSubChannel: c_long,
    ) -> HRESULT,
    fn get_Channel(
        plChannel: *mut c_long,
        plVideoSubChannel: *mut c_long,
        plAudioSubChannel: *mut c_long,
    ) -> HRESULT,
    fn ChannelMinMax(
        lChannelMin: *mut c_long,
        lChannelMax: *mut c_long,
    ) -> HRESULT,
    fn put_CountryCode(
        lCountryCode: c_long,
    ) -> HRESULT,
    fn get_CountryCode(
        plCountryCode: *mut c_long,
    ) -> HRESULT,
    fn put_TuningSpace(
        lTuningSpace: c_long,
    ) -> HRESULT,
    fn get_TuningSpace(
        plTuningSpace: *mut c_long,
    ) -> HRESULT,
    fn Logon(
        hCurrentUser: *const HANDLE,
    ) -> HRESULT,
    fn Logout(
    ) -> HRESULT,
    fn SignalPresent(
        plSignalStrength: *mut c_long,
    ) -> HRESULT,
    fn put_Mode(
        lMode: AMTunerModeType,
    ) -> HRESULT,
    fn get_Mode(
        plMode: *mut AMTunerModeType,
    ) -> HRESULT,
    fn GetAvailableModes(
        plModes: *mut c_long,
    ) -> HRESULT,
    fn RegisterNotificationCallBack(
        pNotify: *const IAMTunerNotification,
        lEvents: c_long,
    ) -> HRESULT,
    fn UnRegisterNotificationCallBack(
        pNotify: *const IAMTunerNotification,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x211a8760, 0x03ac, 0x11d1, 0x8d, 0x13, 0x00, 0xaa, 0x00, 0xbd, 0x83, 0x39)]
interface IAMTunerNotification(IAMTunerNotificationVtbl): IUnknown(IUnknownVtbl) {
    fn OnEvent(
        Event: AMTunerEventType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x211a8766, 0x03ac, 0x11d1, 0x8d, 0x13, 0x00, 0xaa, 0x00, 0xbd, 0x83, 0x39)]
interface IAMTVTuner(IAMTVTunerVtbl): IAMTuner(IAMTunerVtbl) {
    fn get_AvailableTVFormats(
        lAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn get_TVFormat(
        plAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn AutoTune(
        lChannel: c_long,
        plFoundSignal: *mut c_long,
    ) -> HRESULT,
    fn StoreAutoTune(
    ) -> HRESULT,
    fn get_NumInputConnections(
        plNumInputConnections: *mut c_long,
    ) -> HRESULT,
    fn put_InputType(
        lIndex: c_long,
        InputType: TunerInputType,
    ) -> HRESULT,
    fn get_InputType(
        lIndex: c_long,
        pInputType: *mut TunerInputType,
    ) -> HRESULT,
    fn put_ConnectInput(
        lIndex: c_long,
    ) -> HRESULT,
    fn get_ConnectInput(
        plIndex: *mut c_long,
    ) -> HRESULT,
    fn get_VideoFrequency(
        lFreq: *mut c_long,
    ) -> HRESULT,
    fn get_AudioFrequency(
        lFreq: *mut c_long,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x211a8765, 0x03ac, 0x11d1, 0x8d, 0x13, 0x00, 0xaa, 0x00, 0xbd, 0x83, 0x39)]
interface IBPCSatelliteTuner(IBPCSatelliteTunerVtbl): IAMTuner(IAMTunerVtbl) {
    fn get_DefaultSubChannelTypes(
        plDefaultVideoType: *mut c_long,
        plDefaultAudioType: *mut c_long,
    ) -> HRESULT,
    fn put_DefaultSubChannelTypes(
        lDefaultVideoType: c_long,
        lDefaultAudioType: c_long,
    ) -> HRESULT,
    fn IsTapingPermitted(
    ) -> HRESULT,
}}
ENUM!{enum TVAudioMode {
    AMTVAUDIO_MODE_MONO = 1,
    AMTVAUDIO_MODE_STEREO = 2,
    AMTVAUDIO_MODE_LANG_A = 16,
    AMTVAUDIO_MODE_LANG_B = 32,
    AMTVAUDIO_MODE_LANG_C = 64,
    AMTVAUDIO_PRESET_STEREO = 512,
    AMTVAUDIO_PRESET_LANG_A = 4096,
    AMTVAUDIO_PRESET_LANG_B = 8192,
    AMTVAUDIO_PRESET_LANG_C = 16384,
}}
ENUM!{enum AMTVAudioEventType {
    AMTVAUDIO_EVENT_CHANGED = 1,
}}
RIDL!{#[uuid(0x83ec1c30, 0x23d1, 0x11d1, 0x99, 0xe6, 0x00, 0xa0, 0xc9, 0x56, 0x02, 0x66)]
interface IAMTVAudio(IAMTVAudioVtbl): IUnknown(IUnknownVtbl) {
    fn GetHardwareSupportedTVAudioModes(
        plModes: *mut c_long,
    ) -> HRESULT,
    fn GetAvailableTVAudioModes(
        plModes: *mut c_long,
    ) -> HRESULT,
    fn get_TVAudioMode(
        plMode: *mut c_long,
    ) -> HRESULT,
    fn put_TVAudioMode(
        lMode: c_long,
    ) -> HRESULT,
    fn RegisterNotificationCallBack(
        pNotify: *const IAMTunerNotification,
        lEvents: c_long,
    ) -> HRESULT,
    fn UnRegisterNotificationCallBack(
        pNotify: *mut IAMTunerNotification,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x83ec1c33, 0x23d1, 0x11d1, 0x99, 0xe6, 0x00, 0xa0, 0xc9, 0x56, 0x02, 0x66)]
interface IAMTVAudioNotification(IAMTVAudioNotificationVtbl): IUnknown(IUnknownVtbl) {
    fn OnEvent(
        Event: AMTVAudioEventType,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc6e133b0, 0x30ac, 0x11d0, 0xa1, 0x8c, 0x00, 0xa0, 0xc9, 0x11, 0x89, 0x56)]
interface IAMAnalogVideoEncoder(IAMAnalogVideoEncoderVtbl): IUnknown(IUnknownVtbl) {
    fn get_AvailableTVFormats(
        lAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn put_TVFormat(
        lAnalogVideoStandard: c_long,
    ) -> HRESULT,
    fn get_TVFormat(
        plAnalogVideoStandard: *mut c_long,
    ) -> HRESULT,
    fn put_CopyProtection(
        lVideoCopyProtection: c_long,
    ) -> HRESULT,
    fn get_CopyProtection(
        lVideoCopyProtection: *mut c_long,
    ) -> HRESULT,
    fn put_CCEnable(
        lCCEnable: c_long,
    ) -> HRESULT,
    fn get_CCEnable(
        lCCEnable: *mut c_long,
    ) -> HRESULT,
}}
ENUM!{enum AMPROPERTY_PIN {
    AMPROPERTY_PIN_CATEGORY = 0,
    AMPROPERTY_PIN_MEDIUM = 1,
}}
RIDL!{#[uuid(0x31efac30, 0x515c, 0x11d0, 0xa9, 0xaa, 0x00, 0xaa, 0x00, 0x61, 0xbe, 0x93)]
interface IKsPropertySet(IKsPropertySetVtbl): IUnknown(IUnknownVtbl) {
    fn RemoteSet(
        guidPropSet: REFGUID,
        dwPropID: DWORD,
        pInstanceData: LPVOID,
        cbInstanceData: DWORD,
        pPropData: LPVOID,
        cbPropData: DWORD,
    ) -> HRESULT,
    fn RemoteGet(
        guidPropSet: REFGUID,
        dwPropID: DWORD,
        pInstanceData: LPVOID,
        cbInstanceData: DWORD,
        pPropData: LPVOID,
        cbPropData: u32,
        pcbReturned: *mut DWORD,
    ) -> HRESULT,
    fn QuerySupported(
        guidPropSet: REFGUID,
        dwPropID: DWORD,
        pTypeSupport: *mut DWORD,
    ) -> HRESULT,
}}
pub type LPMEDIAPROPERTYBAG = *mut IMediaPropertyBag;
RIDL!{#[uuid(0x6025a880, 0xc0d5, 0x11d0, 0xbd, 0x4e, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IMediaPropertyBag(IMediaPropertyBagVtbl): IPropertyBag(IPropertyBagVtbl) {
    fn EnumProperty(
        iProperty: ULONG,
        pvarPropertyName: *mut VARIANT,
        pvarPropertyValue: *mut VARIANT,
    ) -> HRESULT,
}}
pub type LPPERSISTMEDIAPROPERTYBAG = *mut IPersistMediaPropertyBag;
RIDL!{#[uuid(0x5738e040, 0xb67f, 0x11d0, 0xbd, 0x4d, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IPersistMediaPropertyBag(IPersistMediaPropertyBagVtbl): IPersist(IPersistVtbl) {
    fn InitNew(
    ) -> HRESULT,
    fn Load(
        pPropBag: *const IMediaPropertyBag,
        pErrorLog: *const IErrorLog,
    ) -> HRESULT,
    fn Save(
        pPropBag: *const IMediaPropertyBag,
        fClearDirty: BOOL,
        fSaveAllProperties: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf938c991, 0x3029, 0x11cf, 0x8c, 0x44, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMPhysicalPinInfo(IAMPhysicalPinInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetPhysicalType(
        pType: *mut c_long,
        ppszType: *mut LPOLESTR,
    ) -> HRESULT,
}}
pub type PAMPHYSICALPININFO = *mut IAMPhysicalPinInfo;
RIDL!{#[uuid(0xb5730a90, 0x1a2c, 0x11cf, 0x8c, 0x23, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMExtDevice(IAMExtDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapability(
        Capability: c_long,
        pValue: *mut c_long,
        pdblValue: *mut c_double,
    ) -> HRESULT,
    fn get_ExternalDeviceID(
        ppszData: *mut LPOLESTR,
    ) -> HRESULT,
    fn get_ExternalDeviceVersion(
        ppszData: *mut LPOLESTR,
    ) -> HRESULT,
    fn put_DevicePower(
        PowerMode: c_long,
    ) -> HRESULT,
    fn get_DevicePower(
        pPowerMode: *mut c_long,
    ) -> HRESULT,
    fn Calibrate(
        hEvent: HEVENT,
        mode: c_long,
        pStatus: *mut c_long,
    ) -> HRESULT,
    fn put_DevicePort(
        DevicePort: c_long,
    ) -> HRESULT,
    fn get_DevicePort(
        pDevicePort: *mut c_long,
    ) -> HRESULT,
}}
pub type PEXTDEVICE = *mut IAMExtDevice;
RIDL!{#[uuid(0xa03cd5f0, 0x3045, 0x11cf, 0x8c, 0x44, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMExtTransport(IAMExtTransportVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapability(
        Capability: c_long,
        pValue: *mut c_long,
        pdblValue: *mut c_double,
    ) -> HRESULT,
    fn put_MediaState(
        State: c_long,
    ) -> HRESULT,
    fn get_MediaState(
        pState: *mut c_long,
    ) -> HRESULT,
    fn put_LocalControl(
        State: c_long,
    ) -> HRESULT,
    fn get_LocalControl(
        pState: *mut c_long,
    ) -> HRESULT,
    fn GetStatus(
        StatusItem: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn GetTransportBasicParameters(
        Param: c_long,
        pValue: *mut c_long,
        ppszData: *mut LPOLESTR,
    ) -> HRESULT,
    fn SetTransportBasicParameters(
        Param: c_long,
        Value: c_long,
        pszData: LPCOLESTR,
    ) -> HRESULT,
    fn GetTransportVideoParameters(
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetTransportVideoParameters(
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
    fn GetTransportAudioParameters(
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetTransportAudioParameters(
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
    fn put_Mode(
        mode: c_long,
    ) -> HRESULT,
    fn get_Mode(
        pMode: *mut c_long,
    ) -> HRESULT,
    fn put_Rate(
        dblRate: c_double,
    ) -> HRESULT,
    fn get_Rate(
        pdblRate: *mut f64,
    ) -> HRESULT,
    fn GetChase(
        pEnabled: *mut c_long,
        pOffset: *mut c_long,
        phEvent: *mut HEVENT,
    ) -> HRESULT,
    fn SetChase(
        Enable: c_long,
        Offset: c_long,
        hEvent: HEVENT,
    ) -> HRESULT,
    fn GetBump(
        pSpeed: *mut c_long,
        pDuration: *mut c_long,
    ) -> HRESULT,
    fn SetBump(
        Speed: c_long,
        Duration: c_long,
    ) -> HRESULT,
    fn get_AntiClogControl(
        pEnabled: *mut c_long,
    ) -> HRESULT,
    fn put_AntiClogControl(
        Enable: c_long,
    ) -> HRESULT,
    fn GetEditPropertySet(
        EditID: c_long,
        pState: *mut c_long,
    ) -> HRESULT,
    fn SetEditPropertySet(
        pEditID: *mut c_long,
        State: c_long,
    ) -> HRESULT,
    fn GetEditProperty(
        EditID: c_long,
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetEditProperty(
        EditID: c_long,
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
    fn get_EditStart(
        pValue: *mut c_long,
    ) -> HRESULT,
    fn put_EditStart(
        Value: c_long,
    ) -> HRESULT,
}}
pub type PIAMEXTTRANSPORT = *mut IAMExtTransport;
STRUCT!{struct TIMECODE_s1 {
    wFrameRate: WORD,
    wFrameFract: WORD,
    dwFrames: DWORD,
}}
UNION!{union TIMECODE {
    [u64; 1],
    s1 u1_mut: TIMECODE_s1,
    qw qw_mut: DWORDLONG,
}}
STRUCT!{struct TIMECODE_SAMPLE {
    qwTick: LONGLONG,
    timecode: TIMECODE,
    dwUser: DWORD,
    dwFlags: DWORD,
}}
pub type PTIMECODE = *mut TIMECODE;
pub type PTIMECODE_SAMPLE = *mut TIMECODE_SAMPLE;
RIDL!{#[uuid(0x9b496ce1, 0x811b, 0x11cf, 0x8c, 0x77, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMTimecodeReader(IAMTimecodeReaderVtbl): IUnknown(IUnknownVtbl) {
    fn GetTCRMode(
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetTCRMode(
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
    fn put_VITCLine(
        Line: c_long,
    ) -> HRESULT,
    fn get_VITCLine(
        pLine: *mut c_long,
    ) -> HRESULT,
    fn GetTimecode(
        pTimecodeSample: PTIMECODE_SAMPLE,
    ) -> HRESULT,
}}
pub type PIAMTIMECODEREADER = *mut IAMTimecodeReader;
RIDL!{#[uuid(0x9b496ce0, 0x811b, 0x11cf, 0x8c, 0x77, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMTimecodeGenerator(IAMTimecodeGeneratorVtbl): IUnknown(IUnknownVtbl) {
    fn GetTCGMode(
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetTCGMode(
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
    fn put_VITCLine(
        Line: c_long,
    ) -> HRESULT,
    fn get_VITCLine(
        pLine: *mut c_long,
    ) -> HRESULT,
    fn SetTimecode(
        pTimecodeSample: PTIMECODE_SAMPLE,
    ) -> HRESULT,
    fn GetTimecode(
        pTimecodeSample: PTIMECODE_SAMPLE,
    ) -> HRESULT,
}}
pub type PIAMTIMECODEGENERATOR = *mut IAMTimecodeGenerator;
RIDL!{#[uuid(0x9b496ce2, 0x811b, 0x11cf, 0x8c, 0x77, 0x00, 0xaa, 0x00, 0x6b, 0x68, 0x14)]
interface IAMTimecodeDisplay(IAMTimecodeDisplayVtbl): IUnknown(IUnknownVtbl) {
    fn GetTCDisplayEnable(
        pState: *mut c_long,
    ) -> HRESULT,
    fn SetTCDisplayEnable(
        State: c_long,
    ) -> HRESULT,
    fn GetTCDisplay(
        Param: c_long,
        pValue: *mut c_long,
    ) -> HRESULT,
    fn SetTCDisplay(
        Param: c_long,
        Value: c_long,
    ) -> HRESULT,
}}
pub type PIAMTIMECODEDISPLAY = *mut IAMTimecodeDisplay;
RIDL!{#[uuid(0xc6545bf0, 0xe76b, 0x11d0, 0xbd, 0x52, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IAMDevMemoryAllocator(IAMDevMemoryAllocatorVtbl): IUnknown(IUnknownVtbl) {
    fn GetInfo(
        pdwcbTotalFree: *mut DWORD,
        pdwcbLargestFree: *mut DWORD,
        pdwcbTotalMemory: *mut DWORD,
        pdwcbMinimumChunk: *mut DWORD,
    ) -> HRESULT,
    fn CheckMemory(
        pBuffer: *const BYTE,
    ) -> HRESULT,
    fn Alloc(
        ppBuffer: *mut *mut BYTE,
        pdwcbBuffer: *mut DWORD,
    ) -> HRESULT,
    fn Free(
        pBuffer: *const BYTE,
    ) -> HRESULT,
    fn GetDevMemoryObject(
        ppUnkInnner: *mut *mut IUnknown,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
}}
pub type PAMDEVMEMORYALLOCATOR = *mut IAMDevMemoryAllocator;
RIDL!{#[uuid(0xc6545bf1, 0xe76b, 0x11d0, 0xbd, 0x52, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86)]
interface IAMDevMemoryControl(IAMDevMemoryControlVtbl): IUnknown(IUnknownVtbl) {
    fn QueryWriteSync(
    ) -> HRESULT,
    fn WriteSync(
    ) -> HRESULT,
    fn GetDevId(
        pdwDevId: *mut DWORD,
    ) -> HRESULT,
}}
pub type PAMDEVMEMORYCONTROL = *mut IAMDevMemoryControl;
ENUM!{enum AMSTREAMSELECTINFOFLAGS {
    AMSTREAMSELECTINFO_ENABLED = 1,
    AMSTREAMSELECTINFO_EXCLUSIVE = 2,
}}
ENUM!{enum AMSTREAMSELECTENABLEFLAGS {
    AMSTREAMSELECTENABLE_ENABLE = 1,
    AMSTREAMSELECTENABLE_ENABLEALL = 2,
}}
RIDL!{#[uuid(0xc1960960, 0x17f5, 0x11d1, 0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IAMStreamSelect(IAMStreamSelectVtbl): IUnknown(IUnknownVtbl) {
    fn Count(
        pcStreams: *mut DWORD,
    ) -> HRESULT,
    fn Info(
        lIndex: c_long,
        ppmt: *mut *mut AM_MEDIA_TYPE,
        pdwFlags: *mut DWORD,
        plcid: *mut LCID,
        pdwGroup: *mut DWORD,
        ppszName: *mut LPWSTR,
        ppObject: *mut *mut IUnknown,
        ppunk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Enable(
        lIndex: c_long,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
pub type PAMSTREAMSELECT = *mut IAMStreamSelect;
ENUM!{enum AMRESCTL_RESERVEFLAGS {
    AMRESCTL_RESERVEFLAGS_RESERVE = 0,
    AMRESCTL_RESERVEFLAGS_UNRESERVE = 1,
}}
RIDL!{#[uuid(0x8389d2d0, 0x77d7, 0x11d1, 0xab, 0xe6, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IAMResourceControl(IAMResourceControlVtbl): IUnknown(IUnknownVtbl) {
    fn Reserve(
        dwFlags: DWORD,
        pvReserved: PVOID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4d5466b0, 0xa49c, 0x11d1, 0xab, 0xe8, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IAMClockAdjust(IAMClockAdjustVtbl): IUnknown(IUnknownVtbl) {
    fn SetClockDelta(
        rtDelta: REFERENCE_TIME,
    ) -> HRESULT,
}}
ENUM!{enum AM_FILTER_MISC_FLAGS {
    AM_FILTER_MISC_FLAGS_IS_RENDERER = 1,
    AM_FILTER_MISC_FLAGS_IS_SOURCE = 2,
}}
RIDL!{#[uuid(0x2dd74950, 0xa890, 0x11d1, 0xab, 0xe8, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IAMFilterMiscFlags(IAMFilterMiscFlagsVtbl): IUnknown(IUnknownVtbl) {
    fn GetMiscFlags(
    ) -> ULONG,
}}
RIDL!{#[uuid(0x48efb120, 0xab49, 0x11d2, 0xae, 0xd2, 0x00, 0xa0, 0xc9, 0x95, 0xe8, 0xd5)]
interface IDrawVideoImage(IDrawVideoImageVtbl): IUnknown(IUnknownVtbl) {
    fn DrawVideoImageBegin(
    ) -> HRESULT,
    fn DrawVideoImageEnd(
    ) -> HRESULT,
    fn DrawVideoImageDraw(
        hdc: HDC,
        lprcSrc: LPRECT,
        lprcDst: LPRECT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2e5ea3e0, 0xe924, 0x11d2, 0xb6, 0xda, 0x00, 0xa0, 0xc9, 0x95, 0xe8, 0xdf)]
interface IDecimateVideoImage(IDecimateVideoImageVtbl): IUnknown(IUnknownVtbl) {
    fn SetDecimationImageSize(
        lWidth: c_long,
        lHeight: c_long,
    ) -> HRESULT,
    fn ResetDecimationImageSize(
    ) -> HRESULT,
}}
ENUM!{enum DECIMATION_USAGE {
    DECIMATION_LEGACY = 0,
    DECIMATION_USE_DECODER_ONLY = 1,
    DECIMATION_USE_VIDEOPORT_ONLY = 2,
    DECIMATION_USE_OVERLAY_ONLY = 3,
    DECIMATION_DEFAULT = 4,
}}
RIDL!{#[uuid(0x60d32930, 0x13da, 0x11d3, 0x9e, 0xc6, 0xc4, 0xfc, 0xae, 0xf5, 0xc7, 0xbe)]
interface IAMVideoDecimationProperties(IAMVideoDecimationPropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn QueryDecimationUsage(
        lpUsage: *mut DECIMATION_USAGE,
    ) -> HRESULT,
    fn SetDecimationUsage(
        Usage: DECIMATION_USAGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe46a9787, 0x2b71, 0x444d, 0xa4, 0xb5, 0x1f, 0xab, 0x7b, 0x70, 0x8d, 0x6a)]
interface IVideoFrameStep(IVideoFrameStepVtbl): IUnknown(IUnknownVtbl) {
    fn Step(
        dwFrames: DWORD,
        pStepObject: *mut IUnknown,
    ) -> HRESULT,
    fn CanStep(
        bMultiple: c_long,
        pStepObject: *mut IUnknown,
    ) -> HRESULT,
    fn CancelStep(
    ) -> HRESULT,
}}
ENUM!{enum AM_PUSHSOURCE_FLAGS {
    AM_PUSHSOURCECAPS_INTERNAL_RM = 1,
    AM_PUSHSOURCECAPS_NOT_LIVE = 2,
    AM_PUSHSOURCECAPS_PRIVATE_CLOCK = 4,
    AM_PUSHSOURCEREQS_USE_STREAM_CLOCK = 65536,
    AM_PUSHSOURCEREQS_USE_CLOCK_CHAIN = 131072,
}}
RIDL!{#[uuid(0x62ea93ba, 0xec62, 0x11d2, 0xb7, 0x70, 0x00, 0xc0, 0x4f, 0xb6, 0xbd, 0x3d)]
interface IAMLatency(IAMLatencyVtbl): IUnknown(IUnknownVtbl) {
    fn GetLatency(
        prtLatency: *const REFERENCE_TIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf185fe76, 0xe64e, 0x11d2, 0xb7, 0x6e, 0x00, 0xc0, 0x4f, 0xb6, 0xbd, 0x3d)]
interface IAMPushSource(IAMPushSourceVtbl): IAMLatency(IAMLatencyVtbl) {
    fn GetPushSourceFlags(
        pFlags: *mut ULONG,
    ) -> HRESULT,
    fn SetPushSourceFlags(
        Flags: ULONGLONG,
    ) -> HRESULT,
    fn SetStreamOffset(
        rtOffset: REFERENCE_TIME,
    ) -> HRESULT,
    fn GetStreamOffset(
        prtOffset: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn GetMaxStreamOffset(
        prtMaxOffset: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn SetMaxStreamOffset(
        rtMaxOffset: REFERENCE_TIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf90a6130, 0xb658, 0x11d2, 0xae, 0x49, 0x00, 0x00, 0xf8, 0x75, 0x4b, 0x99)]
interface IAMDeviceRemoval(IAMDeviceRemovalVtbl): IUnknown(IUnknownVtbl) {
    fn DeviceInfo(
        pclsidInterfaceClass: *mut CLSID,
        pwszSymbolicLink: *mut LPWSTR,
    ) -> HRESULT,
    fn Reassociate(
    ) -> HRESULT,
    fn Disassociate(
    ) -> HRESULT,
}}
STRUCT!{struct DVINFO {
    dwDVAAuxSrc: DWORD,
    dwDVAAuxCtl: DWORD,
    dwDVAAuxSrc1: DWORD,
    dwDVAAuxCtl1: DWORD,
    dwDVVAuxSrc: DWORD,
    dwDVVAuxCtl: DWORD,
    dwDVReserved: [DWORD; 2],
}}
pub type PDVINFO = *mut DVINFO;
ENUM!{enum DVENCODERRESOLUTION {
    DVENCODERRESOLUTION_720x480 = 2012,
    DVENCODERRESOLUTION_360x240 = 2013,
    DVENCODERRESOLUTION_180x120 = 2014,
    DVENCODERRESOLUTION_88x60 = 2015,
}}
ENUM!{enum DVENCODERVIDEOFORMAT {
    DVENCODERVIDEOFORMAT_NTSC = 2000,
    DVENCODERVIDEOFORMAT_PAL = 2001,
}}
ENUM!{enum DVENCODERFORMAT {
    DVENCODERFORMAT_DVSD = 2007,
    DVENCODERFORMAT_DVHD = 2008,
    DVENCODERFORMAT_DVSL = 2009,
}}
RIDL!{#[uuid(0xd18e17a0, 0xaacb, 0x11d0, 0xaf, 0xb0, 0x00, 0xaa, 0x00, 0xb6, 0x7a, 0x42)]
interface IDVEnc(IDVEncVtbl): IUnknown(IUnknownVtbl) {
    fn get_IFormatResolution(
        VideoFormat: *mut c_int,
        DVFormat: *mut c_int,
        Resolution: *mut c_int,
        fDVInfo: BYTE,
        sDVInfo: *mut DVINFO,
    ) -> HRESULT,
    fn put_IFormatResolution(
        VideoFormat: c_int,
        DVFormat: c_int,
        Resolution: c_int,
        fDVInfo: BYTE,
        sDVInfo: *const DVINFO,
    ) -> HRESULT,
}}
ENUM!{enum DVDECODERRESOLUTION {
    DVDECODERRESOLUTION_720x480 = 1000,
    DVDECODERRESOLUTION_360x240 = 1001,
    DVDECODERRESOLUTION_180x120 = 1002,
    DVDECODERRESOLUTION_88x60 = 1003,
}}
ENUM!{enum DVRESOLUTION {
    DVRESOLUTION_FULL = 1000,
    DVRESOLUTION_HALF = 1001,
    DVRESOLUTION_QUARTER = 1002,
    DVRESOLUTION_DC = 1003,
}}
RIDL!{#[uuid(0xb8e8bd60, 0x0bfe, 0x11d0, 0xaf, 0x91, 0x00, 0xaa, 0x00, 0xb6, 0x7a, 0x42)]
interface IIPDVDec(IIPDVDecVtbl): IUnknown(IUnknownVtbl) {
    fn get_IPDisplay(
        displayPix: *mut c_int,
    ) -> HRESULT,
    fn put_IPDisplay(
        displayPix: c_int,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x58473a19, 0x2bc8, 0x4663, 0x80, 0x12, 0x25, 0xf8, 0x1b, 0xab, 0xdd, 0xd1)]
interface IDVRGB219(IDVRGB219Vtbl): IUnknown(IUnknownVtbl) {
    fn SetRGB219(
        bState: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x92a3a302, 0xda7c, 0x4a1f, 0xba, 0x7e, 0x18, 0x02, 0xbb, 0x5d, 0x2d, 0x02)]
interface IDVSplitter(IDVSplitterVtbl): IUnknown(IUnknownVtbl) {
    fn DiscardAlternateVideoFrames(
        nDiscard: c_int,
    ) -> HRESULT,
}}
ENUM!{enum AM_AUDIO_RENDERER__PARAM {
    AM_AUDREND_STAT_PARAM_BREAK_COUNT = 1,
    AM_AUDREND_STAT_PARAM_SLAVE_MODE = 2,
    AM_AUDREND_STAT_PARAM_SILENCE_DUR = 3,
    AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR = 4,
    AM_AUDREND_STAT_PARAM_DISCONTINUITIES = 5,
    AM_AUDREND_STAT_PARAM_SLAVE_RATE = 6,
    AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR = 7,
    AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR = 8,
    AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR = 9,
    AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR = 10,
    AM_AUDREND_STAT_PARAM_BUFFERFULLNESS = 11,
    AM_AUDREND_STAT_PARAM_JITTER = 12,
}}
RIDL!{#[uuid(0x22320cb2, 0xd41a, 0x11d2, 0xbf, 0x7c, 0xd7, 0xcb, 0x9d, 0xf0, 0xbf, 0x93)]
interface IAMAudioRendererStats(IAMAudioRendererStatsVtbl): IUnknown(IUnknownVtbl) {
    fn GetStatParam(
        dwParam: DWORD,
        pdwParam1: *mut DWORD,
        pdwParam2: *mut DWORD,
    ) -> HRESULT,
}}
ENUM!{enum _AM_INTF_SEARCH_FLAGS {
    AM_INTF_SEARCH_INPUT_PIN = 1,
    AM_INTF_SEARCH_OUTPUT_PIN = 2,
    AM_INTF_SEARCH_FILTER = 4,
}}
RIDL!{#[uuid(0x632105fa, 0x072e, 0x11d3, 0x8a, 0xf9, 0x00, 0xc0, 0x4f, 0xb6, 0xbd, 0x3d)]
interface IAMGraphStreams(IAMGraphStreamsVtbl): IUnknown(IUnknownVtbl) {
    fn FindUpstreamInterface(
        pPin: *const IPin,
        riid: REFIID,
        ppvInterface: *mut *mut c_void,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SyncUsingStreamOffset(
        bUseStreamOffset: BOOL,
    ) -> HRESULT,
    fn SetMaxGraphLatency(
        rtMaxGraphLatency: REFERENCE_TIME,
    ) -> HRESULT,
}}
ENUM!{enum AMOVERLAYFX {
    AMOVERFX_NOFX = 0,
    AMOVERFX_MIRRORLEFTRIGHT = 2,
    AMOVERFX_MIRRORUPDOWN = 4,
    AMOVERFX_DEINTERLACE = 8,
}}
RIDL!{#[uuid(0x62fae250, 0x7e65, 0x4460, 0xbf, 0xc9, 0x63, 0x98, 0xb3, 0x22, 0x07, 0x3c)]
interface IAMOverlayFX(IAMOverlayFXVtbl): IUnknown(IUnknownVtbl) {
    fn QueryOverlayFXCaps(
        lpdwOverlayFXCaps: *mut DWORD,
    ) -> HRESULT,
    fn SetOverlayFX(
        dwOverlayFX: DWORD,
    ) -> HRESULT,
    fn GetOverlayFX(
        lpdwOverlayFX: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8e1c39a1, 0xde53, 0x11cf, 0xaa, 0x63, 0x00, 0x80, 0xc7, 0x44, 0x52, 0x8d)]
interface IAMOpenProgress(IAMOpenProgressVtbl): IUnknown(IUnknownVtbl) {
    fn QueryProgress(
        pllTotal: *mut LONGLONG,
        pllCurrent: *mut LONGLONG,
    ) -> HRESULT,
    fn AbortOperation(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x436eee9c, 0x264f, 0x4242, 0x90, 0xe1, 0x4e, 0x33, 0x0c, 0x10, 0x75, 0x12)]
interface IMpeg2Demultiplexer(IMpeg2DemultiplexerVtbl): IUnknown(IUnknownVtbl) {
    fn CreateOutputPin(
        pMediaType: *const AM_MEDIA_TYPE,
        pszPinName: LPWSTR,
        ppIPin: *mut *mut IPin,
    ) -> HRESULT,
    fn SetOutputPinMediaType(
        pszPinName: LPWSTR,
        pMediaType: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn DeleteOutputPin(
        pszPinName: LPWSTR,
    ) -> HRESULT,
}}
pub const MPEG2_PROGRAM_STREAM_MAP: DWORD = 0;
pub const MPEG2_PROGRAM_ELEMENTARY_STREAM: DWORD = 1;
pub const MPEG2_PROGRAM_DIRECTORY_PES_PACKET: DWORD = 2;
pub const MPEG2_PROGRAM_PACK_HEADER: DWORD = 3;
pub const MPEG2_PROGRAM_PES_STREAM: DWORD = 4;
pub const MPEG2_PROGRAM_SYSTEM_HEADER: DWORD = 5;
pub const SUBSTREAM_FILTER_VAL_NONE: DWORD = 0x10000000;
STRUCT!{struct STREAM_ID_MAP {
    stream_id: ULONG,
    dwMediaSampleContent: DWORD,
    ulSubstreamFilterValue: ULONG,
    iDataOffset: c_int,
}}
RIDL!{#[uuid(0x945c1566, 0x6202, 0x46fc, 0x96, 0xc7, 0xd8, 0x7f, 0x28, 0x9c, 0x65, 0x34)]
interface IEnumStreamIdMap(IEnumStreamIdMapVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cRequest: ULONG,
        pStreamIdMap: *mut STREAM_ID_MAP,
        pcReceived: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cRecords: ULONG,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppIEnumStreamIdMap: *mut *mut IEnumStreamIdMap,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd0e04c47, 0x25b8, 0x4369, 0x92, 0x5a, 0x36, 0x2a, 0x01, 0xd9, 0x54, 0x44)]
interface IMPEG2StreamIdMap(IMPEG2StreamIdMapVtbl): IUnknown(IUnknownVtbl) {
    fn MapStreamId(
        ulStreamId: ULONG,
        MediaSampleContent: DWORD,
        ulSubstreamFilterValue: ULONG,
        iDataOffset: c_int,
    ) -> HRESULT,
    fn UnmapStreamId(
        culStreamId: ULONG,
        pulStreamId: *const ULONG,
    ) -> HRESULT,
    fn EnumStreamIdMap(
        ppIEnumStreamIdMap: *mut *mut IEnumStreamIdMap,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7b3a2f01, 0x0751, 0x48dd, 0xb5, 0x56, 0x00, 0x47, 0x85, 0x17, 0x1c, 0x54)]
interface IRegisterServiceProvider(IRegisterServiceProviderVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterService(
        guidService: REFGUID,
        punkObject: *const IUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9fd52741, 0x176d, 0x4b36, 0x8f, 0x51, 0xca, 0x8f, 0x93, 0x32, 0x23, 0xbe)]
interface IAMClockSlave(IAMClockSlaveVtbl): IUnknown(IUnknownVtbl) {
    fn SetErrorTolerance(
        dwTolerance: DWORD,
    ) -> HRESULT,
    fn GetErrorTolerance(
        pdwTolerance: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4995f511, 0x9ddb, 0x4f12, 0xbd, 0x3b, 0xf0, 0x46, 0x11, 0x80, 0x7b, 0x79)]
interface IAMGraphBuilderCallback(IAMGraphBuilderCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn SelectedFilter(
        pMon: *const IMoniker,
    ) -> HRESULT,
    fn CreatedFilter(
        pFil: *const IBaseFilter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56a868fd, 0x0ad4, 0x11ce, 0xb0, 0xa3, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70)]
interface IAMFilterGraphCallback(IAMFilterGraphCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn UnableToRender(
        pPin: *mut IPin,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa8809222, 0x07bb, 0x48ea, 0x95, 0x1c, 0x33, 0x15, 0x81, 0x00, 0x62, 0x5b)]
interface IGetCapabilitiesKey(IGetCapabilitiesKeyVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilitiesKey(
        pHKey: *mut HKEY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x70423839, 0x6acc, 0x4b23, 0xb0, 0x79, 0x21, 0xdb, 0xf0, 0x81, 0x56, 0xa5)]
interface IEncoderAPI(IEncoderAPIVtbl): IUnknown(IUnknownVtbl) {
    fn IsSupported(
        Api: *const GUID,
    ) -> HRESULT,
    fn IsAvailable(
        Api: *const GUID,
    ) -> HRESULT,
    fn GetParameterRange(
        Api: *const GUID,
        ValueMin: *mut VARIANT,
        ValueMax: *mut VARIANT,
        SteppingDelta: *mut VARIANT,
    ) -> HRESULT,
    fn GetParameterValues(
        Api: *const GUID,
        Values: *mut *mut VARIANT,
        ValuesCount: *mut ULONG,
    ) -> HRESULT,
    fn GetDefaultValue(
        Api: *const GUID,
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn GetValue(
        Api: *const GUID,
        Value: *mut VARIANT,
    ) -> HRESULT,
    fn SetValue(
        Api: *const GUID,
        Value: *const VARIANT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x02997c3b, 0x8e1b, 0x460e, 0x92, 0x70, 0x54, 0x5e, 0x0d, 0xe9, 0x56, 0x3e)]
interface IVideoEncoder(IVideoEncoderVtbl): IEncoderAPI(IEncoderAPIVtbl) {
}}
ENUM!{enum VIDEOENCODER_BITRATE_MODE {
    ConstantBitRate = 0,
    VariableBitRateAverage = 1,
    VariableBitRatePeak = 2,
}}
pub const AM_GETDECODERCAP_QUERY_VMR_SUPPORT: DWORD = 0x00000001;
pub const VMR_NOTSUPPORTED: DWORD = 0x00000000;
pub const VMR_SUPPORTED: DWORD = 0x00000001;
pub const AM_QUERY_DECODER_VMR_SUPPORT: DWORD = 0x00000001;
pub const AM_QUERY_DECODER_DXVA_1_SUPPORT: DWORD = 0x00000002;
pub const AM_QUERY_DECODER_DVD_SUPPORT: DWORD = 0x00000003;
pub const AM_QUERY_DECODER_ATSC_SD_SUPPORT: DWORD = 0x00000004;
pub const AM_QUERY_DECODER_ATSC_HD_SUPPORT: DWORD = 0x00000005;
pub const AM_GETDECODERCAP_QUERY_VMR9_SUPPORT: DWORD = 0x00000006;
pub const AM_GETDECODERCAP_QUERY_EVR_SUPPORT: DWORD = 0x00000007;
pub const DECODER_CAP_NOTSUPPORTED: DWORD = 0x00000000;
pub const DECODER_CAP_SUPPORTED: DWORD = 0x00000001;
RIDL!{#[uuid(0xc0dff467, 0xd499, 0x4986, 0x97, 0x2b, 0xe1, 0xd9, 0x09, 0x0f, 0xa9, 0x41)]
interface IAMDecoderCaps(IAMDecoderCapsVtbl): IUnknown(IUnknownVtbl) {
    fn GetDecoderCaps(
        dwCapIndex: DWORD,
        lpdwCap: *mut DWORD,
    ) -> HRESULT,
}}
STRUCT!{struct AMCOPPSignature {
    Signature: [BYTE; 256],
}}
STRUCT!{struct AMCOPPCommand {
    macKDI: GUID,
    guidCommandID: GUID,
    dwSequence: DWORD,
    cbSizeData: DWORD,
    CommandData: [BYTE; 4056],
}}
pub type LPAMCOPPCommand = *mut AMCOPPCommand;
STRUCT!{struct AMCOPPStatusInput {
    rApp: GUID,
    guidStatusRequestID: GUID,
    dwSequence: DWORD,
    cbSizeData: DWORD,
    StatusData: [BYTE; 4056],
}}
pub type LPAMCOPPStatusInput = *mut AMCOPPStatusInput;
STRUCT!{struct AMCOPPStatusOutput {
    macKDI: GUID,
    cbSizeData: DWORD,
    COPPStatus: [BYTE; 4076],
}}
pub type LPAMCOPPStatusOutput = *mut AMCOPPStatusOutput;
RIDL!{#[uuid(0x6feded3e, 0x0ff1, 0x4901, 0xa2, 0xf1, 0x43, 0xf7, 0x01, 0x2c, 0x85, 0x15)]
interface IAMCertifiedOutputProtection(IAMCertifiedOutputProtectionVtbl): IUnknown(IUnknownVtbl) {
    fn KeyExchange(
        pRandom: *mut GUID,
        VarLenCertGH: *mut *mut BYTE,
        pdwLengthCertGH: *mut DWORD,
    ) -> HRESULT,
    fn SessionSequenceStart(
        pSig: *const AMCOPPSignature,
    ) -> HRESULT,
    fn ProtectionCommand(
        cmd: *const AMCOPPCommand,
    ) -> HRESULT,
    fn ProtectionStatus(
        pStatusInput: *const AMCOPPStatusInput,
        pStatusOutput: *mut AMCOPPStatusOutput,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcf7b26fc, 0x9a00, 0x485b, 0x81, 0x47, 0x3e, 0x78, 0x9d, 0x5e, 0x8f, 0x67)]
interface IAMAsyncReaderTimestampScaling(IAMAsyncReaderTimestampScalingVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetTimestampMode(
        pfRaw: *mut BOOL,
    ) -> HRESULT,
    fn SetTimestampMode(
        fRaw: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0e26a181, 0xf40c, 0x4635, 0x87, 0x86, 0x97, 0x62, 0x84, 0xb5, 0x29, 0x81)]
interface IAMPluginControl(IAMPluginControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetPreferredClsid(
        subtype: REFGUID,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn GetPreferredClsidByIndex(
        index: DWORD,
        subtype: *mut GUID,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetPreferredClsid(
        subtype: REFGUID,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn IsDisabled(
        clsid: REFCLSID,
    ) -> HRESULT,
    fn GetDisabledByIndex(
        index: DWORD,
        clsid: *mut CLSID,
    ) -> HRESULT,
    fn SetDisabled(
        clsid: REFCLSID,
        disabled: BOOL,
    ) -> HRESULT,
    fn IsLegacyDisabled(
        dllName: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4a9a62d3, 0x27d4, 0x403d, 0x91, 0xe9, 0x89, 0xf5, 0x40, 0xe5, 0x55, 0x34)]
interface IPinConnection(IPinConnectionVtbl): IUnknown(IUnknownVtbl) {
    fn DynamicQueryAccept(
        pmt: *const AM_MEDIA_TYPE,
    ) -> HRESULT,
    fn NotifyEndOfStream(
        hNotifyEvent: HANDLE,
    ) -> HRESULT,
    fn IsEndPin(
    ) -> HRESULT,
    fn DynamicDisconnect(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc56e9858, 0xdbf3, 0x4f6b, 0x81, 0x19, 0x38, 0x4a, 0xf2, 0x06, 0x0d, 0xeb)]
interface IPinFlowControl(IPinFlowControlVtbl): IUnknown(IUnknownVtbl) {
    fn Block(
        dwBlockFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
}}
ENUM!{enum AM_PIN_FLOW_CONTROL_BLOCK_FLAGS {
    AM_PIN_FLOW_CONTROL_BLOCK = 1,
}}
ENUM!{enum AM_GRAPH_CONFIG_RECONNECT_FLAGS {
    AM_GRAPH_CONFIG_RECONNECT_DIRECTCONNECT = 1,
    AM_GRAPH_CONFIG_RECONNECT_CACHE_REMOVED_FILTERS = 2,
    AM_GRAPH_CONFIG_RECONNECT_USE_ONLY_CACHED_FILTERS = 4,
}}
ENUM!{enum REM_FILTER_FLAGS {
    REMFILTERF_LEAVECONNECTED = 1,
}}
ENUM!{enum AM_FILTER_FLAGS {
    AM_FILTER_FLAGS_REMOVABLE = 1,
}}
RIDL!{#[uuid(0x03a1eb8e, 0x32bf, 0x4245, 0x85, 0x02, 0x11, 0x4d, 0x08, 0xa9, 0xcb, 0x88)]
interface IGraphConfig(IGraphConfigVtbl): IUnknown(IUnknownVtbl) {
    fn Reconnect(
        pOutputPin: *const IPin,
        pInputPin: *const IPin,
        pmtFirstConnection: *const AM_MEDIA_TYPE,
        pUsingFilter: *const IBaseFilter,
        hAbortEvent: HANDLE,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Reconfigure(
        pCallback: *const IGraphConfigCallback,
        pvContext: PVOID,
        dwFlags: DWORD,
        hAbortEvent: HANDLE,
    ) -> HRESULT,
    fn AddFilterToCache(
        pFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn EnumCacheFilter(
        pEnum: *mut *mut IEnumFilters,
    ) -> HRESULT,
    fn RemoveFilterFromCache(
        pFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn GetStartTime(
        prtStart: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn PushThroughData(
        pOutputPin: *const IPin,
        pConnection: *const IPinConnection,
        hEventAbort: HANDLE,
    ) -> HRESULT,
    fn SetFilterFlags(
        pFilter: *const IBaseFilter,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetFilterFlags(
        pFilter: *const IBaseFilter,
        pdwFlags: *mut DWORD,
    ) -> HRESULT,
    fn RemoveFilterEx(
        pFilter: *const IBaseFilter,
        Flags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xade0fd60, 0xd19d, 0x11d2, 0xab, 0xf6, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75)]
interface IGraphConfigCallback(IGraphConfigCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Reconfigure(
        pvContext: PVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcfbdcf6, 0x0dc2, 0x45f5, 0x9a, 0xb2, 0x7c, 0x33, 0x0e, 0xa0, 0x9c, 0x29)]
interface IFilterChain(IFilterChainVtbl): IUnknown(IUnknownVtbl) {
    fn StartChain(
        pStartFilter: *const IBaseFilter,
        pEndFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn PauseChain(
        pStartFilter: *const IBaseFilter,
        pEndFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn StopChain(
        pStartFilter: *const IBaseFilter,
        pEndFilter: *const IBaseFilter,
    ) -> HRESULT,
    fn RemoveChain(
        pStartFilter: *const IBaseFilter,
        pEndFilter: *const IBaseFilter,
    ) -> HRESULT,
}}
ENUM!{enum VMRPresentationFlags {
    VMRSample_SyncPoint = 1,
    VMRSample_Preroll = 2,
    VMRSample_Discontinuity = 4,
    VMRSample_TimeValid = 8,
    VMRSample_SrcDstRectsValid = 16,
}}
STRUCT!{struct VMRPRESENTATIONINFO {
    dwFlags: DWORD,
    lpSurf: LPDIRECTDRAWSURFACE7,
    rtStart: REFERENCE_TIME,
    rtEnd: REFERENCE_TIME,
    szAspectRatio: SIZE,
    rcSrc: RECT,
    rcDst: RECT,
    dwTypeSpecificFlags: DWORD,
    dwInterlaceFlags: DWORD,
}}
RIDL!{#[uuid(0xce704fe7, 0xe71e, 0x41fb, 0xba, 0xa2, 0xc4, 0x40, 0x3e, 0x11, 0x82, 0xf5)]
interface IVMRImagePresenter(IVMRImagePresenterVtbl): IUnknown(IUnknownVtbl) {
    fn StartPresenting(
        dwUserID: DWORD_PTR,
    ) -> HRESULT,
    fn StopPresenting(
        dwUserID: DWORD_PTR,
    ) -> HRESULT,
    fn PresentImage(
        dwUserID: DWORD_PTR,
        lpPresInfo: *const VMRPRESENTATIONINFO,
    ) -> HRESULT,
}}
ENUM!{enum VMRSurfaceAllocationFlags {
    AMAP_PIXELFORMAT_VALID = 1,
    AMAP_3D_TARGET = 2,
    AMAP_ALLOW_SYSMEM = 4,
    AMAP_FORCE_SYSMEM = 8,
    AMAP_DIRECTED_FLIP = 16,
    AMAP_DXVA_TARGET = 32,
}}
STRUCT!{struct VMRALLOCATIONINFO {
    dwFlags: DWORD,
    lpHdr: LPBITMAPINFOHEADER,
    lpPixFmt: LPDDPIXELFORMAT,
    szAspectRatio: SIZE,
    dwMinBuffers: DWORD,
    dwMaxBuffers: DWORD,
    dwInterlaceFlags: DWORD,
    szNativeSize: SIZE,
}}
RIDL!{#[uuid(0x31ce832e, 0x4484, 0x458b, 0x8c, 0xca, 0xf4, 0xd7, 0xe3, 0xdb, 0x0b, 0x52)]
interface IVMRSurfaceAllocator(IVMRSurfaceAllocatorVtbl): IUnknown(IUnknownVtbl) {
    fn AllocateSurface(
        dwUserID: DWORD_PTR,
        lpAllocInfo: *const VMRALLOCATIONINFO,
        lpdwActualBuffers: *mut DWORD,
        lplpSurface: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn FreeSurface(
        dwID: DWORD_PTR,
    ) -> HRESULT,
    fn PrepareSurface(
        dwUserID: DWORD_PTR,
        lpSurface: LPDIRECTDRAWSURFACE7,
        dwSurfaceFlags: DWORD,
    ) -> HRESULT,
    fn AdviseNotify(
        lpIVMRSurfAllocNotify: *const IVMRSurfaceAllocatorNotify,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaada05a8, 0x5a4e, 0x4729, 0xaf, 0x0b, 0xce, 0xa2, 0x7a, 0xed, 0x51, 0xe2)]
interface IVMRSurfaceAllocatorNotify(IVMRSurfaceAllocatorNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn AdviseSurfaceAllocator(
        dwUserID: DWORD_PTR,
        lpIVRMSurfaceAllocator: *const IVMRSurfaceAllocator,
    ) -> HRESULT,
    fn SetDDrawDevice(
        lpDDrawDevice: LPDIRECTDRAWSURFACE7,
        hMonitor: HMONITOR,
    ) -> HRESULT,
    fn ChangeDDrawDevice(
        lpDDrawDevice: LPDIRECTDRAWSURFACE7,
        hMonitor: HMONITOR,
    ) -> HRESULT,
    fn RestoreDDrawSurfaces(
    ) -> HRESULT,
    fn NotifyEvent(
        EventCode: LONG,
        Param1: LONG_PTR,
        Param2: LONG_PTR,
    ) -> HRESULT,
    fn SetBorderColor(
        clrBorder: COLORREF,
    ) -> HRESULT,
}}
ENUM!{enum VMR_ASPECT_RATIO_MODE {
    VMR_ARMODE_NONE = 0,
    VMR_ARMODE_LETTER_BOX = 1,
}}
RIDL!{#[uuid(0x0eb1088c, 0x4dcd, 0x46f0, 0x87, 0x8f, 0x39, 0xda, 0xe8, 0x6a, 0x51, 0xb7)]
interface IVMRWindowlessControl(IVMRWindowlessControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetNativeVideoSize(
        lpWidth: *mut c_long,
        lpHeight: *mut c_long,
        lpARWidth: *mut c_long,
        lpARHeight: *mut c_long,
    ) -> HRESULT,
    fn GetMinIdealVideoSize(
        lpWidth: *mut c_long,
        lpHeight: *mut c_long,
    ) -> HRESULT,
    fn GetMaxIdealVideoSize(
        lpWidth: *mut c_long,
        lpHeight: *mut c_long,
    ) -> HRESULT,
    fn SetVideoPosition(
        lpSRCRect: LPRECT,
        lpDSTRect: LPRECT,
    ) -> HRESULT,
    fn GetVideoPosition(
        lpSRCRect: LPRECT,
        lpDSTRect: LPRECT,
    ) -> HRESULT,
    fn GetAspectRatioMode(
        lpAspectRatioMode: *mut DWORD,
    ) -> HRESULT,
    fn SetAspectRatioMode(
        AspectRatioMode: DWORD,
    ) -> HRESULT,
    fn SetVideoClippingWindow(
        hwnd: HWND,
    ) -> HRESULT,
    fn RepaintVideo(
        hwnd: HWND,
        hdc: HDC,
    ) -> HRESULT,
    fn DisplayModeChanged(
    ) -> HRESULT,
    fn GetCurrentImage(
        lpDib: *mut *mut BYTE,
    ) -> HRESULT,
    fn SetBorderColor(
        Clr: COLORREF,
    ) -> HRESULT,
    fn GetBorderColor(
        lpClr: *mut COLORREF,
    ) -> HRESULT,
    fn SetColorKey(
        Clr: COLORREF,
    ) -> HRESULT,
    fn GetColorKey(
        lpClr: *mut COLORREF,
    ) -> HRESULT,
}}
ENUM!{enum VMRMixerPrefs {
    MixerPref_NoDecimation = 1,
    MixerPref_DecimateOutput = 2,
    MixerPref_ARAdjustXorY = 4,
    MixerPref_DecimationReserved = 8,
    MixerPref_DecimateMask = 15,
    MixerPref_BiLinearFiltering = 16,
    MixerPref_PointFiltering = 32,
    MixerPref_FilteringMask = 240,
    MixerPref_RenderTargetRGB = 256,
    MixerPref_RenderTargetYUV = 4096,
    MixerPref_RenderTargetYUV420 = 512,
    MixerPref_RenderTargetYUV422 = 1024,
    MixerPref_RenderTargetYUV444 = 2048,
    MixerPref_RenderTargetReserved = 57344,
    MixerPref_RenderTargetMask = 65280,
    MixerPref_DynamicSwitchToBOB = 65536,
    MixerPref_DynamicDecimateBy2 = 131072,
    MixerPref_DynamicReserved = 786432,
    MixerPref_DynamicMask = 983040,
}}
STRUCT!{struct NORMALIZEDRECT {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}}
pub type PNORMALIZEDRECT = *mut NORMALIZEDRECT;
RIDL!{#[uuid(0x1c1a17b0, 0xbed0, 0x415d, 0x97, 0x4b, 0xdc, 0x66, 0x96, 0x13, 0x15, 0x99)]
interface IVMRMixerControl(IVMRMixerControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetAlpha(
        dwStreamId: DWORD,
        Alpha: c_float,
    ) -> HRESULT,
    fn GetAlpha(
        dwStreamId: DWORD,
        pAlpha: *mut c_float,
    ) -> HRESULT,
    fn SetZOrder(
        dwStreamId: DWORD,
        dwZ: DWORD,
    ) -> HRESULT,
    fn GetZOrder(
        dwStreamId: DWORD,
        pZ: *mut DWORD,
    ) -> HRESULT,
    fn SetOutputRect(
        dwStreamId: DWORD,
        pRect: *const NORMALIZEDRECT,
    ) -> HRESULT,
    fn GetOutputRect(
        dwStreamId: DWORD,
        pRect: *mut NORMALIZEDRECT,
    ) -> HRESULT,
    fn SetBackgroundClr(
        ClrBkg: DWORD,
    ) -> HRESULT,
    fn GetBackgroundClr(
        lpClrBkg: *const COLORREF,
    ) -> HRESULT,
    fn SetMixingPrefs(
        dwMixerPrefs: DWORD,
    ) -> HRESULT,
    fn GetMixingPrefs(
        pdwMixerPrefs: *mut DWORD,
    ) -> HRESULT,
}}
STRUCT!{struct VMRGUID {
    pGUID: *mut GUID,
    guid: GUID,
}}
STRUCT!{struct VMRMONITORINFO {
    guid: VMRGUID,
    rcMonitor: RECT,
    hMon: HMONITOR,
    dwFlags: DWORD,
    szDevice: [WCHAR; 32],
    szDescription: [WCHAR; 256],
    liDriverVersion: LARGE_INTEGER,
    dwVendorId: DWORD,
    dwDeviceId: DWORD,
    dwSubSysId: DWORD,
    dwRevision: DWORD,
}}
RIDL!{#[uuid(0x9cf0b1b6, 0xfbaa, 0x4b7f, 0x88, 0xcf, 0xcf, 0x1f, 0x13, 0x0a, 0x0d, 0xce)]
interface IVMRMonitorConfig(IVMRMonitorConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetMonitor(
        pGUID: *const VMRGUID,
    ) -> HRESULT,
    fn GetMonitor(
        pGUID: *mut VMRGUID,
    ) -> HRESULT,
    fn SetDefaultMonitor(
        pGUID: *const VMRGUID,
    ) -> HRESULT,
    fn GetDefaultMonitor(
        pGUID: *mut VMRGUID,
    ) -> HRESULT,
    fn GetAvailableMonitors(
        pInfo: *mut VMRMONITORINFO,
        dwMaxInfoArraySize: DWORD,
        pdwNumDevices: *mut DWORD,
    ) -> HRESULT,
}}
ENUM!{enum VMRRenderPrefs {
    RenderPrefs_RestrictToInitialMonitor = 0,
    RenderPrefs_ForceOffscreen = 1,
    RenderPrefs_ForceOverlays = 2,
    RenderPrefs_AllowOverlays = 0,
    RenderPrefs_AllowOffscreen = 0,
    RenderPrefs_DoNotRenderColorKeyAndBorder = 8,
    RenderPrefs_Reserved = 16,
    RenderPrefs_PreferAGPMemWhenMixing = 32,
    RenderPrefs_Mask = 63,
}}
ENUM!{enum VMRMode {
    VMRMode_Windowed = 1,
    VMRMode_Windowless = 2,
    VMRMode_Renderless = 4,
    VMRMode_Mask = 7,
}}
pub const MAX_NUMBER_OF_STREAMS: DWORD = 16;
RIDL!{#[uuid(0x9e5530c5, 0x7034, 0x48b4, 0xbb, 0x46, 0x0b, 0x8a, 0x6e, 0xfc, 0x8e, 0x36)]
interface IVMRFilterConfig(IVMRFilterConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetImageCompositor(
        lpVMRImgCompositor: *const IVMRImageCompositor,
    ) -> HRESULT,
    fn SetNumberOfStreams(
        dwMaxStreams: DWORD,
    ) -> HRESULT,
    fn GetNumberOfStreams(
        pdwMaxStreams: *mut DWORD,
    ) -> HRESULT,
    fn SetRenderingPrefs(
        dwRenderFlags: DWORD,
    ) -> HRESULT,
    fn GetRenderingPrefs(
        pdwRenderFlags: *mut DWORD,
    ) -> HRESULT,
    fn SetRenderingMode(
        mode: DWORD,
    ) -> HRESULT,
    fn GetRenderingMode(
        pMode: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xede80b5c, 0xbad6, 0x4623, 0xb5, 0x37, 0x65, 0x58, 0x6c, 0x9f, 0x8d, 0xfd)]
interface IVMRAspectRatioControl(IVMRAspectRatioControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetAspectRatioMode(
        lpdwARMode: *mut DWORD,
    ) -> HRESULT,
    fn SetAspectRatioMode(
        dwARMode: DWORD,
    ) -> HRESULT,
}}
ENUM!{enum VMRDeinterlacePrefs {
    DeinterlacePref_NextBest = 1,
    DeinterlacePref_BOB = 2,
    DeinterlacePref_Weave = 4,
    DeinterlacePref_Mask = 7,
}}
ENUM!{enum VMRDeinterlaceTech {
    DeinterlaceTech_Unknown = 0,
    DeinterlaceTech_BOBLineReplicate = 1,
    DeinterlaceTech_BOBVerticalStretch = 2,
    DeinterlaceTech_MedianFiltering = 4,
    DeinterlaceTech_EdgeFiltering = 16,
    DeinterlaceTech_FieldAdaptive = 32,
    DeinterlaceTech_PixelAdaptive = 64,
    DeinterlaceTech_MotionVectorSteered = 128,
}}
STRUCT!{struct VMRFrequency {
    dwNumerator: DWORD,
    dwDenominator: DWORD,
}}
STRUCT!{struct VMRVideoDesc {
    dwSize: DWORD,
    dwSampleWidth: DWORD,
    dwSampleHeight: DWORD,
    SingleFieldPerSample: BOOL,
    dwFourCC: DWORD,
    InputSampleFreq: VMRFrequency,
    OutputFrameFreq: VMRFrequency,
}}
STRUCT!{struct VMRDeinterlaceCaps {
    dwSize: DWORD,
    dwNumPreviousOutputFrames: DWORD,
    dwNumForwardRefSamples: DWORD,
    dwNumBackwardRefSamples: DWORD,
    DeinterlaceTechnology: VMRDeinterlaceTech,
}}
RIDL!{#[uuid(0xbb057577, 0x0db8, 0x4e6a, 0x87, 0xa7, 0x1a, 0x8c, 0x9a, 0x50, 0x5a, 0x0f)]
interface IVMRDeinterlaceControl(IVMRDeinterlaceControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetNumberOfDeinterlaceModes(
        lpVideoDescription: *const VMRVideoDesc,
        lpdwNumDeinterlaceModes: LPDWORD,
        lpDeinterlaceModes: LPGUID,
    ) -> HRESULT,
    fn GetDeinterlaceModeCaps(
        lpDeinterlaceMode: LPGUID,
        lpVideoDescription: *const VMRVideoDesc,
        lpDeinterlaceCaps: *mut VMRDeinterlaceCaps,
    ) -> HRESULT,
    fn GetDeinterlaceMode(
        dwStreamId: DWORD,
        lpDeinterlaceMode: LPGUID,
    ) -> HRESULT,
    fn SetDeinterlaceMode(
        dwStreamId: DWORD,
        lpDeinterlaceMode: LPGUID,
    ) -> HRESULT,
    fn GetDeinterlacePrefs(
        lpdwDeinterlacePrefs: LPDWORD,
    ) -> HRESULT,
    fn SetDeinterlacePrefs(
        dwDeinterlacePrefs: DWORD,
    ) -> HRESULT,
    fn GetActualDeinterlaceMode(
        dwStreamId: DWORD,
        lpDeinterlaceMode: LPGUID,
    ) -> HRESULT,
}}
STRUCT!{struct VMRALPHABITMAP {
    dwFlags: DWORD,
    hdc: HDC,
    pDDS: LPDIRECTDRAWSURFACE7,
    rSrc: RECT,
    rDest: NORMALIZEDRECT,
    fAlpha: c_float,
    clrSrcKey: COLORREF,
}}
pub type PVMRALPHABITMAP = *mut VMRALPHABITMAP;
pub const VMRBITMAP_DISABLE: DWORD = 1;
pub const VMRBITMAP_HDC: DWORD = 2;
pub const VMRBITMAP_ENTIREDDS: DWORD = 4;
pub const VMRBITMAP_SRCCOLORKEY: DWORD = 8;
pub const VMRBITMAP_SRCRECT: DWORD = 16;
RIDL!{#[uuid(0x1e673275, 0x0257, 0x40aa, 0xaf, 0x20, 0x7c, 0x60, 0x8d, 0x4a, 0x04, 0x28)]
interface IVMRMixerBitmap(IVMRMixerBitmapVtbl): IUnknown(IUnknownVtbl) {
    fn SetAlphaBitmap(
        pBmpParms: *const VMRALPHABITMAP,
    ) -> HRESULT,
    fn UpdateAlphaBitmapParameters(
        pBmpParms: PVMRALPHABITMAP,
    ) -> HRESULT,
    fn GetAlphaBitmapParameters(
        pBmpParms: PVMRALPHABITMAP,
    ) -> HRESULT,
}}
STRUCT!{struct VMRVIDEOSTREAMINFO {
    pddsVideoSurface: LPDIRECTDRAWSURFACE7,
    dwWidth: DWORD,
    dwHeight: DWORD,
    dwStrmID: DWORD,
    fAlpha: FLOAT,
    ddClrKey: DDCOLORKEY,
    rNormal: NORMALIZEDRECT,
}}
RIDL!{#[uuid(0x7a4fb5af, 0x479f, 0x4074, 0xbb, 0x40, 0xce, 0x67, 0x22, 0xe4, 0x3c, 0x82)]
interface IVMRImageCompositor(IVMRImageCompositorVtbl): IUnknown(IUnknownVtbl) {
    fn InitCompositionTarget(
        pD3DDevice: *const IUnknown,
        pddsRenderTarget: *const u32,
    ) -> HRESULT,
    fn TermCompositionTarget(
        pD3DDevice: *const IUnknown,
        pddsRenderTarget: *const u32,
    ) -> HRESULT,
    fn SetStreamMediaType(
        dwStrmID: DWORD,
        pmt: *const AM_MEDIA_TYPE,
        fTexture: BOOL,
    ) -> HRESULT,
    fn CompositeImage(
        pD3DDevice: *const IUnknown,
        pddsRenderTarget: LPDIRECTDRAWSURFACE7,
        pmtRenderTarget: *const AM_MEDIA_TYPE,
        rtStart: REFERENCE_TIME,
        rtEnd: REFERENCE_TIME,
        dwClrBkGnd: DWORD,
        pVideoStreamInfo: *const VMRVIDEOSTREAMINFO,
        cStreams: UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x058d1f11, 0x2a54, 0x4bef, 0xbd, 0x54, 0xdf, 0x70, 0x66, 0x26, 0xb7, 0x27)]
interface IVMRVideoStreamControl(IVMRVideoStreamControlVtbl): IUnknown(IUnknownVtbl) {
    fn SetColorKey(
        lpClrKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetColorKey(
        lpClrKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetStreamActiveState(
        fActive: BOOL,
    ) -> HRESULT,
    fn GetStreamActiveState(
        lpfActive: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa9849bbe, 0x9ec8, 0x4263, 0xb7, 0x64, 0x62, 0x73, 0x0f, 0x0d, 0x15, 0xd0)]
interface IVMRSurface(IVMRSurfaceVtbl): IUnknown(IUnknownVtbl) {
    fn IsSurfaceLocked(
    ) -> HRESULT,
    fn LockSurface(
        lpSurface: *mut *mut BYTE,
    ) -> HRESULT,
    fn UnlockSurface(
    ) -> HRESULT,
    fn GetSurface(
        lplpSurface: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9f3a1c85, 0x8555, 0x49ba, 0x93, 0x5f, 0xbe, 0x5b, 0x5b, 0x29, 0xd1, 0x78)]
interface IVMRImagePresenterConfig(IVMRImagePresenterConfigVtbl): IUnknown(IUnknownVtbl) {
    fn SetRenderingPrefs(
        dwRenderFlags: DWORD,
    ) -> HRESULT,
    fn GetRenderingPrefs(
        dwRenderFlags: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe6f7ce40, 0x4673, 0x44f1, 0x8f, 0x77, 0x54, 0x99, 0xd6, 0x8c, 0xb4, 0xea)]
interface IVMRImagePresenterExclModeConfig(IVMRImagePresenterExclModeConfigVtbl):
    IVMRImagePresenterConfig(IVMRImagePresenterConfigVtbl) {
    fn SetXlcModeDDObjAndPrimarySurface(
        lpDDObj: LPDIRECTDRAW7,
        lpPrimarySurf: LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn GetXlcModeDDObjAndPrimarySurface(
        lpDDObj: *mut LPDIRECTDRAW7,
        lpPrimarySurf: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaac18c18, 0xe186, 0x46d2, 0x82, 0x5d, 0xa1, 0xf8, 0xdc, 0x8e, 0x39, 0x5a)]
interface IVPManager(IVPManagerVtbl): IUnknown(IUnknownVtbl) {
    fn SetVideoPortIndex(
        dwVideoPortIndex: DWORD,
    ) -> HRESULT,
    fn GetVideoPortIndex(
        pdwVideoPortIndex: *mut DWORD,
    ) -> HRESULT,
}}
