use shared::guiddef::{CLSID, GUID, REFCLSID, REFGUID};
use shared::minwindef::{BOOL, DWORD, ULONG};
use shared::windef::RECT;
use shared::wtypesbase::LPCOLESTR;
use um::oaidl::VARIANT;
use um::objidlbase::IStream;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG};
pub const FILTER_STATUS_OPAQUE: DWORD = (0x00000001);
pub const FILTER_STATUS_INVISIBLE: DWORD = (0x00000002);
pub const FILTER_STATUS_SURFACE: DWORD = (0x00000004);
pub const FILTER_STATUS_3DSURFACE: DWORD = (0x00000008);
pub const SURFACE_LOCK_EXCLUSIVE: DWORD = 0x01;
pub const SURFACE_LOCK_ALLOW_DISCARD: DWORD = 0x02;
pub const SURFACE_LOCK_WAIT: DWORD = 0x04;
pub const E_SURFACE_NOSURFACE: HRESULT = 0x8000C000;
pub const E_SURFACE_UNKNOWN_FORMAT: HRESULT = 0x8000C001;
pub const E_SURFACE_NOTMYPOINTER: HRESULT = 0x8000C002;
pub const E_SURFACE_DISCARDED: HRESULT = 0x8000C003;
pub const E_SURFACE_NODC: HRESULT = 0x8000C004;
pub const E_SURFACE_NOTMYDC: HRESULT = 0x8000C005;
pub const S_SURFACE_DISCARDED: HRESULT = 0x0000C003;
pub type BFID = GUID;
// typedef struct tagRGBQUAD RGBQUAD;
// EXTERN_C const GUID BFID_MONOCHROME;
// EXTERN_C const GUID BFID_RGB_4;
// EXTERN_C const GUID BFID_RGB_8;
// EXTERN_C const GUID BFID_RGB_555;
// EXTERN_C const GUID BFID_RGB_565;
// EXTERN_C const GUID BFID_RGB_24;
// EXTERN_C const GUID BFID_RGB_32;
// EXTERN_C const GUID BFID_RGBA_32;
// EXTERN_C const GUID BFID_GRAY_8;
// EXTERN_C const GUID BFID_GRAY_16;
// #define SID_SDirectDraw3 IID_IDirectDraw3
pub const COLOR_NO_TRANSPARENT: DWORD = 0xFFFFFFFF;
pub const IMGDECODE_EVENT_PROGRESS: DWORD = 0x01;
pub const IMGDECODE_EVENT_PALETTE: DWORD = 0x02;
pub const IMGDECODE_EVENT_BEGINBITS: DWORD = 0x04;
pub const IMGDECODE_EVENT_BITSCOMPLETE: DWORD = 0x08;
pub const IMGDECODE_EVENT_USEDDRAW: DWORD = 0x10;
pub const IMGDECODE_HINT_TOPDOWN: DWORD = 0x01;
pub const IMGDECODE_HINT_BOTTOMUP: DWORD = 0x02;
pub const IMGDECODE_HINT_FULLWIDTH: DWORD = 0x04;
pub const MAPMIME_DEFAULT: DWORD = 0;
pub const MAPMIME_CLSID: DWORD = 1;
pub const MAPMIME_DISABLE: DWORD = 2;
pub const MAPMIME_DEFAULT_ALWAYS: DWORD = 3;
// #define BFID_INDEXED_RGB_8 BFID_RGB_8
// #define BFID_INDEXED_RGB_4 BFID_RGB_4
// #define BFID_INDEXED_RGB_1 BFID_MONOCHROME
// EXTERN_C const GUID CLSID_IImageDecodeFilter;
// EXTERN_C const GUID NAMEDTIMER_DRAW;
pub const TIMERMODE_NORMAL: DWORD = 0;
pub const TIMERMODE_VISIBILITYAWARE: DWORD = 1;
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0000_v0_0_s_ifspec;
DEFINE_GUID!{IID_ITimerService,
    0x3050f35f, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b}
RIDL!{#[uuid(0x3050f35f, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b)]
interface ITimerService(ITimerServiceVtbl): IUnknown(IUnknownVtbl) {
    fn CreateTimer(
        pReferenceTimer: *mut ITimer,
        ppNewTimer: *mut *mut ITimer,
    ) -> HRESULT,
    fn GetNamedTimer(
        rguidName: REFGUID,
        ppTimer: *mut *mut ITimer,
    ) -> HRESULT,
    fn SetNamedTimerReference(
        rguidName: REFGUID,
        pReferenceTimer: *mut ITimer,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITimer,
    0x3050f360, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b}
RIDL!{#[uuid(0x3050f360, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b)]
interface ITimer(ITimerVtbl): IUnknown(IUnknownVtbl) {
    fn Advise(
        vtimeMin: VARIANT,
        vtimeMax: VARIANT,
        vtimeInterval: VARIANT,
        dwFlags: DWORD,
        pTimerSink: *mut ITimerSink,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn Freeze(
        fFreeze: BOOL,
    ) -> HRESULT,
    fn GetTime(
        pvtime: *mut VARIANT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITimerEx,
    0x30510414, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b}
RIDL!{#[uuid(0x30510414, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b)]
interface ITimerEx(ITimerExVtbl): ITimer(ITimerVtbl) {
    fn SetMode(
        dwMode: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_ITimerSink,
    0x3050f361, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b}
RIDL!{#[uuid(0x3050f361, 0x98b5, 0x11cf, 0xbb, 0x82, 0x00, 0xaa, 0x00, 0xbd, 0xce, 0x0b)]
interface ITimerSink(ITimerSinkVtbl): IUnknown(IUnknownVtbl) {
    fn OnTimer(
        vtimeAdvise: VARIANT,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0004_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0004_v0_0_s_ifspec;
pub use self::IID_ITimerService as SID_STimerService;
DEFINE_GUID!{IID_IMapMIMEToCLSID,
    0xd9e89500, 0x30fa, 0x11d0, 0xb7, 0x24, 0x00, 0xaa, 0x00, 0x6c, 0x1a, 0x01}
RIDL!{#[uuid(0xd9e89500, 0x30fa, 0x11d0, 0xb7, 0x24, 0x00, 0xaa, 0x00, 0x6c, 0x1a, 0x01)]
interface IMapMIMEToCLSID(IMapMIMEToCLSIDVtbl): IUnknown(IUnknownVtbl) {
    fn EnableDefaultMappings(
        bEnable: BOOL,
    ) -> HRESULT,
    fn MapMIMEToCLSID(
        pszMIMEType: LPCOLESTR,
        pCLSID: *mut CLSID,
    ) -> HRESULT,
    fn SetMapping(
        pszMIMEType: LPCOLESTR,
        dwMapMode: DWORD,
        clsid: REFCLSID,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IImageDecodeFilter,
    0xa3ccedf3, 0x2de2, 0x11d0, 0x86, 0xf4, 0x00, 0xa0, 0xc9, 0x13, 0xf7, 0x50}
RIDL!{#[uuid(0xa3ccedf3, 0x2de2, 0x11d0, 0x86, 0xf4, 0x00, 0xa0, 0xc9, 0x13, 0xf7, 0x50)]
interface IImageDecodeFilter(IImageDecodeFilterVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pEventSink: *mut IImageDecodeEventSink,
    ) -> HRESULT,
    fn Process(
        pStream: *mut IStream,
    ) -> HRESULT,
    fn Terminate(
        hrStatus: HRESULT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IImageDecodeEventSink,
    0xbaa342a0, 0x2ded, 0x11d0, 0x86, 0xf4, 0x00, 0xa0, 0xc9, 0x13, 0xf7, 0x50}
RIDL!{#[uuid(0xbaa342a0, 0x2ded, 0x11d0, 0x86, 0xf4, 0x00, 0xa0, 0xc9, 0x13, 0xf7, 0x50)]
interface IImageDecodeEventSink(IImageDecodeEventSinkVtbl): IUnknown(IUnknownVtbl) {
    fn GetSurface(
        nWidth: LONG,
        nHeight: LONG,
        bfid: REFGUID,
        nPasses: ULONG,
        dwHints: DWORD,
        ppSurface: *mut *mut IUnknown,
    ) -> HRESULT,
    fn OnBeginDecode(
        pdwEvents: *mut DWORD,
        pnFormats: *mut ULONG,
        ppFormats: *mut *mut BFID,
    ) -> HRESULT,
    fn OnBitsComplete() -> HRESULT,
    fn OnDecodeComplete(
        hrStatus: HRESULT,
    ) -> HRESULT,
    fn OnPalette() -> HRESULT,
    fn OnProgress(
        pBounds: *mut RECT,
        bComplete: BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IImageDecodeEventSink2,
    0x8ebd8a57, 0x8a96, 0x48c9, 0x84, 0xa6, 0x96, 0x2e, 0x2d, 0xb9, 0xc9, 0x31}
RIDL!{#[uuid(0x8ebd8a57, 0x8a96, 0x48c9, 0x84, 0xa6, 0x96, 0x2e, 0x2d, 0xb9, 0xc9, 0x31)]
interface IImageDecodeEventSink2(IImageDecodeEventSink2Vtbl):
    IImageDecodeEventSink(IImageDecodeEventSinkVtbl)
{
    fn IsAlphaPremultRequired(
        pfPremultAlpha: *mut BOOL,
    ) -> HRESULT,
}}
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0008_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0008_v0_0_s_ifspec;
// unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * );
// unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * );
