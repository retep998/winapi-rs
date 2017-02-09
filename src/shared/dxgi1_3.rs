// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_3.h
use shared::dxgi::{IDXGIOutput, IDXGIResource};
use shared::dxgi1_2::{
    IDXGISwapChain1, IDXGIFactory2, IDXGIFactory2Vtbl, IDXGIDevice2, IDXGIDevice2Vtbl,
    IDXGISwapChain1Vtbl, IDXGIOutput1, IDXGIOutput1Vtbl, DXGI_SWAP_CHAIN_DESC1
};
use shared::dxgiformat::{DXGI_FORMAT};
use shared::minwindef::{UINT, BOOL, FLOAT};
use shared::windef::{RECT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, HANDLE, LARGE_INTEGER};

ENUM!{ enum DXGI_FRAME_PRESENTATION_MODE {
    DXGI_FRAME_PRESENTATION_MODE_COMPOSED = 0,
    DXGI_FRAME_PRESENTATION_MODE_OVERLAY = 1,
    DXGI_FRAME_PRESENTATION_MODE_NONE = 2,
    DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE = 3,
}}

ENUM!{ enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE = 0x1,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709 = 0x2,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC = 0x4,
}}

ENUM!{ enum DXGI_OVERLAY_SUPPORT_FLAG {
    DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1,
    DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2,
}}

STRUCT!{struct DXGI_DECODE_SWAP_CHAIN_DESC {
    Flags: UINT,
}}

STRUCT!{struct DXGI_FRAME_STATISTICS_MEDIA {
    PresentCount: UINT,
    PresentRefreshCount: UINT,
    SyncRefreshCount: UINT,
    SyncQPCTime: LARGE_INTEGER,
    SyncGPUTime: LARGE_INTEGER,
    CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    ApprovedPresentDuration: UINT,
}}

STRUCT!{struct DXGI_MATRIX_3X2_F {
    _11: FLOAT,
    _12: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
}}

RIDL!(
interface IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVtbl): IUnknown(IUnknownVtbl) {
    fn PresentBuffer(
        BufferToPresent: UINT, SyncInterval: UINT, Flags: UINT
    ) -> HRESULT,
    fn SetSourceRect(pRect: *const RECT) -> HRESULT,
    fn SetTargetRect(pRect: *const RECT) -> HRESULT,
    fn SetDestSize(Width: UINT, Height: UINT) -> HRESULT,
    fn GetSourceRect(pRect: *mut RECT) -> HRESULT,
    fn GetTargetRect(pRect: *mut RECT) -> HRESULT,
    fn GetDestSize(
        pWidth: *mut UINT, pHeight: *mut UINT
    ) -> HRESULT,
    fn SetColorSpace(
        ColorSpace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
    ) -> HRESULT,
    fn GetColorSpace() -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
});

RIDL!(
interface IDXGIDevice3(IDXGIDevice3Vtbl): IDXGIDevice2(IDXGIDevice2Vtbl) {
    fn Trim() -> ()
});

RIDL!(
interface IDXGIFactory3(IDXGIFactory3Vtbl): IDXGIFactory2(IDXGIFactory2Vtbl) {
    fn GetCreationFlags() -> UINT
});

RIDL!(
interface IDXGIFactoryMedia(IDXGIFactoryMediaVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSwapChainForCompositionSurfaceHandle(
        pDevice: *mut IUnknown, hSurface: HANDLE,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1
    ) -> HRESULT,
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(
        pDevice: *mut IUnknown, hSurface: HANDLE,
        pDesc: *mut DXGI_DECODE_SWAP_CHAIN_DESC, pYuvDecodeBuffers: *mut IDXGIResource,
        pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut *mut IDXGIDecodeSwapChain
    ) -> HRESULT
});

RIDL!(
interface IDXGIOutput2(IDXGIOutput2Vtbl): IDXGIOutput1(IDXGIOutput1Vtbl) {
    fn SupportsOverlays() -> BOOL
});

RIDL!(
interface IDXGIOutput3(IDXGIOutput3Vtbl): IDXGIOutput2(IDXGIOutput2Vtbl) {
    fn CheckOverlaySupport(
        EnumFormat: DXGI_FORMAT, pConcernedDevice: *mut IUnknown,
        pFlags: *mut UINT
    ) -> HRESULT
});

RIDL!(
interface IDXGISwapChain2(IDXGISwapChain2Vtbl): IDXGISwapChain1(IDXGISwapChain1Vtbl) {
    fn SetSourceSize(Width: UINT, Height: UINT) -> HRESULT,
    fn GetSourceSize(
        pWidth: *mut UINT, pHeight: *mut UINT
    ) -> HRESULT,
    fn SetMaximumFrameLatency(MaxLatency: UINT) -> HRESULT,
    fn GetMaximumFrameLatency(pMaxLatency: *mut UINT) -> HRESULT,
    fn GetFrameLatencyWaitableObject() -> HANDLE,
    fn SetMatrixTransform(
        pMatrix: *const DXGI_MATRIX_3X2_F
    ) -> HRESULT,
    fn GetMatrixTransform(
        pMatrix: *mut DXGI_MATRIX_3X2_F
    ) -> HRESULT
});

RIDL!(
interface IDXGISwapChainMedia(IDXGISwapChainMediaVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrameStatisticsMedia(
        pStats: *mut DXGI_FRAME_STATISTICS_MEDIA
    ) -> HRESULT,
    fn SetPresentDuration(Duration: UINT) -> HRESULT,
    fn CheckPresentDurationSupport(
        DesiredPresentDuration: UINT, pClosestSmallerPresentDuration: *mut UINT,
        pClosestLargerPresentDuration: *mut UINT
    ) -> HRESULT
});

pub const DXGI_CREATE_FACTORY_DEBUG: UINT = 0x1;
