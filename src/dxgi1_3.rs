// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_3.h

ENUM!{ enum DXGI_FRAME_PRESENTATION_MODE {
    DXGI_FRAME_PRESENTATION_MODE_COMPOSED = 0,
    DXGI_FRAME_PRESENTATION_MODE_OVERLAY = 1,
    DXGI_FRAME_PRESENTATION_MODE_NONE = 2,
    DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE = 3,
}}

FLAGS!{ enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE = 0x1,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709 = 0x2,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC = 0x4,
}}

FLAGS!{ enum DXGI_OVERLAY_SUPPORT_FLAG {
    DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1,
    DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2,
}}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: ::UINT,
    pub PresentRefreshCount: ::UINT,
    pub SyncRefreshCount: ::UINT,
    pub SyncQPCTime: ::LARGE_INTEGER,
    pub SyncGPUTime: ::LARGE_INTEGER,
    pub CompositionMode: ::DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: ::FLOAT,
    pub _12: ::FLOAT,
    pub _21: ::FLOAT,
    pub _22: ::FLOAT,
    pub _31: ::FLOAT,
    pub _32: ::FLOAT,
}

RIDL!(
interface IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVtbl): IUnknown(IUnknownVtbl) {
    fn PresentBuffer(
        &mut self, BufferToPresent: ::UINT, SyncInterval: ::UINT, Flags: ::UINT
    ) -> ::HRESULT,
    fn SetSourceRect(&mut self, pRect: *const ::RECT) -> ::HRESULT,
    fn SetTargetRect(&mut self, pRect: *const ::RECT) -> ::HRESULT,
    fn SetDestSize(&mut self, Width: ::UINT, Height: ::UINT) -> ::HRESULT,
    fn GetSourceRect(&mut self, pRect: *mut ::RECT) -> ::HRESULT,
    fn GetTargetRect(&mut self, pRect: *mut ::RECT) -> ::HRESULT,
    fn GetDestSize(
        &mut self, pWidth: *mut ::UINT, pHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn SetColorSpace(
        &mut self, ColorSpace: ::DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
    ) -> ::HRESULT,
    fn GetColorSpace(&mut self) -> ::DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
});

RIDL!(
interface IDXGIDevice3(IDXGIDevice3Vtbl): IDXGIDevice2(IDXGIDevice2Vtbl) {
    fn Trim(&mut self) -> ()
});

RIDL!(
interface IDXGIFactory3(IDXGIFactory3Vtbl): IDXGIFactory2(IDXGIFactory2Vtbl) {
    fn GetCreationFlags(&mut self) -> ::UINT
});

RIDL!(
interface IDXGIFactoryMedia(IDXGIFactoryMediaVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSwapChainForCompositionSurfaceHandle(
        &mut self, pDevice: *mut ::IUnknown, hSurface: ::HANDLE,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut ::IDXGIOutput,
        ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(
        &mut self, pDevice: *mut ::IUnknown, hSurface: ::HANDLE,
        pDesc: *mut ::DXGI_DECODE_SWAP_CHAIN_DESC, pYuvDecodeBuffers: *mut ::IDXGIResource,
        pRestrictToOutput: *mut ::IDXGIOutput, ppSwapChain: *mut *mut ::IDXGIDecodeSwapChain
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput2(IDXGIOutput2Vtbl): IDXGIOutput1(IDXGIOutput1Vtbl) {
    fn SupportsOverlays(&mut self) -> ::BOOL
});

RIDL!(
interface IDXGIOutput3(IDXGIOutput3Vtbl): IDXGIOutput2(IDXGIOutput2Vtbl) {
    fn CheckOverlaySupport(
        &mut self, EnumFormat: ::DXGI_FORMAT, pConcernedDevice: *mut ::IUnknown,
        pFlags: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain2(IDXGISwapChain2Vtbl): IDXGISwapChain1(IDXGISwapChain1Vtbl) {
    fn SetSourceSize(&mut self, Width: ::UINT, Height: ::UINT) -> ::HRESULT,
    fn GetSourceSize(
        &mut self, pWidth: *mut ::UINT, pHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn SetMaximumFrameLatency(&mut self, MaxLatency: ::UINT) -> ::HRESULT,
    fn GetMaximumFrameLatency(&mut self, pMaxLatency: *mut ::UINT) -> ::HRESULT,
    fn GetFrameLatencyWaitableObject(&mut self) -> ::HANDLE,
    fn SetMatrixTransform(
        &mut self, pMatrix: *const ::DXGI_MATRIX_3X2_F
    ) -> ::HRESULT,
    fn GetMatrixTransform(
        &mut self, pMatrix: *mut ::DXGI_MATRIX_3X2_F
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChainMedia(IDXGISwapChainMediaVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrameStatisticsMedia(
        &mut self, pStats: *mut ::DXGI_FRAME_STATISTICS_MEDIA
    ) -> ::HRESULT,
    fn SetPresentDuration(&mut self, Duration: ::UINT) -> ::HRESULT,
    fn CheckPresentDurationSupport(
        &mut self, DesiredPresentDuration: ::UINT, pClosestSmallerPresentDuration: *mut ::UINT,
        pClosestLargerPresentDuration: *mut ::UINT
    ) -> ::HRESULT
});

pub const DXGI_CREATE_FACTORY_DEBUG: ::UINT = 0x1;

DEFINE_GUID!(IID_IDXGIDevice3,0x6007896c,0x3244,0x4afd,0xbf,0x18,0xa6,0xd3,0xbe,
    0xda,0x50,0x23);
DEFINE_GUID!(IID_IDXGISwapChain2,0xa8be2ac4,0x199f,0x4946,0xb3,0x31,0x79,0x59,
    0x9f,0xb9,0x8d,0xe7);
DEFINE_GUID!(IID_IDXGIOutput2,0x595e39d1,0x2724,0x4663,0x99,0xb1,0xda,0x96,0x9d,
    0xe2,0x83,0x64);
DEFINE_GUID!(IID_IDXGIFactory3,0x25483823,0xcd46,0x4c7d,0x86,0xca,0x47,0xaa,0x95,
    0xb8,0x37,0xbd);
DEFINE_GUID!(IID_IDXGIDecodeSwapChain,0x2633066b,0x4514,0x4c7a,0x8f,0xd8,0x12,
    0xea,0x98,0x05,0x9d,0x18);
DEFINE_GUID!(IID_IDXGIFactoryMedia,0x41e7d1f2,0xa591,0x4f7b,0xa2,0xe5,0xfa,0x9c,
    0x84,0x3e,0x1c,0x12);
DEFINE_GUID!(IID_IDXGISwapChainMedia,0xdd95b90b,0xf05f,0x4f6a,0xbd,0x65,0x25,
    0xbf,0xb2,0x64,0xbd,0x84);
DEFINE_GUID!(IID_IDXGIOutput3,0x8a6bb301,0x7e7e,0x41F4,0xa8,0xe0,0x5b,0x32,0xf7,
    0xf9,0x9b,0x18);
