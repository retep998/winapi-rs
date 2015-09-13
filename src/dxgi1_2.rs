// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_2.h

ENUM!{ enum DXGI_ALPHA_MODE {
    DXGI_ALPHA_MODE_UNSPECIFIED = 0x0,
    DXGI_ALPHA_MODE_PREMULTIPLIED = 0x1,
    DXGI_ALPHA_MODE_STRAIGHT = 0x2,
    DXGI_ALPHA_MODE_IGNORE = 0x3,
    DXGI_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY = 0x0,
    DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY = 0x1,
    DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY = 0x2,
    DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY = 0x3,
    DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY = 0x4,
}}

ENUM!{ enum DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY = 0x0,
    DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY = 0x1,
    DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY = 0x2,
    DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY = 0x3,
    DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY = 0x4,
}}

ENUM!{ enum DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME = 0x1,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR = 0x2,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR = 0x4,
}}

ENUM!{ enum DXGI_SCALING {
    DXGI_SCALING_STRETCH = 0x0,
    DXGI_SCALING_NONE = 0x1,
    DXGI_SCALING_ASPECT_RATIO_STRETCH = 0x2,
}}

ENUM!{ enum _DXGI_OFFER_RESOURCE_PRIORITY {
    DXGI_OFFER_RESOURCE_PRIORITY_LOW = 0x1,
    DXGI_OFFER_RESOURCE_PRIORITY_NORMAL = 0x2,
    DXGI_OFFER_RESOURCE_PRIORITY_HIGH = 0x3,
}}

#[repr(C)] #[derive(Copy)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [::WCHAR; 128],
    pub VendorId: ::UINT,
    pub DeviceId: ::UINT,
    pub SubSysId: ::UINT,
    pub Revision: ::UINT,
    pub DedicatedVideoMemory: ::SIZE_T,
    pub DedicatedSystemMemory: ::SIZE_T,
    pub SharedSystemMemory: ::SIZE_T,
    pub AdapterLuid: ::LUID,
    pub Flags: ::UINT,
    pub GraphicsPreemptionGranularity: ::DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: ::DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}

impl Clone for DXGI_ADAPTER_DESC2 {
    fn clone(&self) -> Self { *self }
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_MODE_DESC1 {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub RefreshRate: ::DXGI_RATIONAL,
    pub Format: ::DXGI_FORMAT,
    pub ScanlineOrdering: ::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: ::DXGI_MODE_SCALING,
    pub Stereo: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: ::DXGI_MODE_DESC,
    pub Rotation: ::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: ::LARGE_INTEGER,
    pub LastMouseUpdateTime: ::LARGE_INTEGER,
    pub AccumulatedFrames: ::UINT,
    pub RectsCoalesced: ::BOOL,
    pub ProtectedContentMaskedOut: ::BOOL,
    pub PointerPosition: ::DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: ::UINT,
    pub PointerShapeBufferSize: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: ::POINT,
    pub DestinationRect: ::RECT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: ::POINT,
    pub Visible: ::BOOL,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: ::UINT,
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Pitch: ::UINT,
    pub HotSpot: ::POINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: ::UINT,
    pub pDirtyRects: *mut ::RECT,
    pub pScrollRect: *mut ::RECT,
    pub pScrollOffset: *mut ::POINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Format: ::DXGI_FORMAT,
    pub Stereo: ::BOOL,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
    pub BufferUsage: ::DXGI_USAGE,
    pub BufferCount: ::UINT,
    pub Scaling: ::DXGI_SCALING,
    pub SwapEffect: ::DXGI_SWAP_EFFECT,
    pub AlphaMode: ::DXGI_ALPHA_MODE,
    pub Flags: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: ::DXGI_RATIONAL,
    pub ScanlineOrdering: ::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: ::DXGI_MODE_SCALING,
    pub Windowed: ::BOOL,
}

RIDL!(
interface IDXGIAdapter2(IDXGIAdapter2Vtbl): IDXGIAdapter1(IDXGIAdapter1Vtbl) {
    fn GetDesc2(
        &mut self, This: *mut ::IDXGIAdapter2, pDesc: *mut ::DXGI_ADAPTER_DESC2
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIDevice2(IDXGIDevice2Vtbl): IDXGIDevice1(IDXGIDevice1Vtbl) {
    fn OfferResources(
        &mut self, This: *mut ::IDXGIDevice2, NumResources: ::UINT,
        ppResources: *mut *mut ::IDXGIResource, Priority: ::DXGI_OFFER_RESOURCE_PRIORITY
    ) -> ::HRESULT,
    fn ReclaimResources(
        &mut self, This: *mut ::IDXGIDevice2, NumResources: ::UINT,
        ppResources: *mut *mut ::IDXGIResource, pDiscarded: *mut ::BOOL
    ) -> ::HRESULT,
    fn EnqueueSetEvent(
        &mut self, This: *mut ::IDXGIDevice2, hEvent: ::HANDLE
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIDisplayControl(IDXGIDisplayControlVtbl): IUnknown(IUnknownVtbl) {
    fn IsStereoEnabled(&mut self, This: *mut ::IDXGIDisplayControl) -> ::BOOL,
    fn SetStereoEnabled(
        &mut self, This: *mut ::IDXGIDisplayControl, enabled: ::BOOL
    ) -> ()
});

RIDL!(
interface IDXGIFactory2(IDXGIFactory2Vtbl): IDXGIFactory1(IDXGIFactory1Vtbl) {
    fn IsWindowedStereoEnabled(&mut self, This: *mut ::IDXGIFactory2) -> ::BOOL,
    fn CreateSwapChainForHwnd(
        &mut self, This: *mut ::IDXGIFactory2, pDevice: *mut ::IUnknown, hWnd: ::HWND,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1,
        pFullscreenDesc: *const ::DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        pRestrictToOutput: *mut ::IDXGIOutput, ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn CreateSwapChainForCoreWindow(
        &mut self, This: *mut ::IDXGIFactory2, pDevice: *mut ::IUnknown, pWindow: *mut ::IUnknown,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut ::IDXGIOutput,
        ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn GetSharedResourceAdapterLuid(
        &mut self, This: *mut ::IDXGIFactory2, hResource: ::HANDLE, pLuid: *mut ::LUID
    ) -> ::HRESULT,
    fn RegisterStereoStatusWindow(
        &mut self, This: *mut ::IDXGIFactory2, WindowHandle: ::HWND, wMsg: ::UINT,
        pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn RegisterStereoStatusEvent(
        &mut self, This: *mut ::IDXGIFactory2, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterStereoStatus(
        &mut self, This: *mut ::IDXGIFactory2, dwCookie: ::DWORD
    ) -> (),
    fn RegisterOcclusionStatusWindow(
        &mut self, This: *mut ::IDXGIFactory2, WindowHandle: ::HWND, wMsg: ::UINT,
        pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn RegisterOcclusionStatusEvent(
        &mut self, This: *mut ::IDXGIFactory2, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterOcclusionStatus(
        &mut self, This: *mut ::IDXGIFactory2, dwCookie: ::DWORD
    ) -> (),
    fn CreateSwapChainForComposition(
        &mut self, This: *mut ::IDXGIFactory2, pDevice: *mut ::IUnknown,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut ::IDXGIOutput,
        ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput1(IDXGIOutput1Vtbl): IDXGIOutput(IDXGIOutputVtbl) {
    fn GetDisplayModeList1(
        &mut self, This: *mut ::IDXGIOutput1, EnumFormat: ::DXGI_FORMAT, Flags: ::UINT,
        pNumModes: *mut ::UINT, pDesc: *mut ::DXGI_MODE_DESC1
    ) -> ::HRESULT,
    fn FindClosestMatchingMode1(
        &mut self, This: *mut ::IDXGIOutput1, pModeToMatch: *const ::DXGI_MODE_DESC1,
        pClosestMatch: *mut ::DXGI_MODE_DESC1, pConcernedDevice: *mut ::IUnknown
    ) -> ::HRESULT,
    fn GetDisplaySurfaceData1(
        &mut self, This: *mut ::IDXGIOutput1, pDestination: *mut ::IDXGIResource
    ) -> ::HRESULT,
    fn DuplicateOutput(
        &mut self, This: *mut ::IDXGIOutput1, pDevice: *mut ::IUnknown,
        ppOutputDuplication: *mut *mut ::IDXGIOutputDuplication
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutputDuplication(IDXGIOutputDuplicationVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(
        &mut self, This: *mut ::IDXGIOutputDuplication, pDesc: *mut ::DXGI_OUTDUPL_DESC
    ) -> (),
    fn AcquireNextFrame(
        &mut self, This: *mut ::IDXGIOutputDuplication, TimeoutInMilliseconds: ::UINT,
        pFrameInfo: *mut ::DXGI_OUTDUPL_FRAME_INFO, ppDesktopResource: *mut *mut ::IDXGIResource
    ) -> ::HRESULT,
    fn GetFrameDirtyRects(
        &mut self, This: *mut ::IDXGIOutputDuplication, DirtyRectsBufferSize: ::UINT,
        pDirtyRectsBuffer: *mut ::RECT, pDirtyRectsBufferSizeRequired: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFrameMoveRects(
        &mut self, This: *mut ::IDXGIOutputDuplication, MoveRectsBufferSize: ::UINT,
        pMoveRectBuffer: *mut ::DXGI_OUTDUPL_MOVE_RECT, pMoveRectsBufferSizeRequired: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFramePointerShape(
        &mut self, This: *mut ::IDXGIOutputDuplication, PointerShapeBufferSize: ::UINT,
        pPointerShapeBuffer: *mut ::c_void, pPointerShapeBufferSizeRequired: *mut ::UINT,
        pPointerShapeInfo: *mut ::DXGI_OUTDUPL_POINTER_SHAPE_INFO
    ) -> ::HRESULT,
    fn MapDesktopSurface(
        &mut self, This: *mut ::IDXGIOutputDuplication, pLockedRect: *mut ::DXGI_MAPPED_RECT
    ) -> ::HRESULT,
    fn UnMapDesktopSurface(
        &mut self, This: *mut ::IDXGIOutputDuplication
    ) -> ::HRESULT,
    fn ReleaseFrame(&mut self, This: *mut ::IDXGIOutputDuplication) -> ::HRESULT
});

RIDL!(
interface IDXGIResource1(IDXGIResource1Vtbl): IDXGIResource(IDXGIResourceVtbl) {
    fn CreateSubresourceSurface(
        &mut self, This: *mut ::IDXGIResource1, index: ::UINT, ppSurface: *mut *mut ::IDXGISurface2
    ) -> ::HRESULT,
    fn CreateSharedHandle(
        &mut self, This: *mut ::IDXGIResource1, pAttributes: *const ::SECURITY_ATTRIBUTES,
        dwAccess: ::DWORD, lpName: ::LPCWSTR, pHandle: *mut ::HANDLE
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISurface2(IDXGISurface2Vtbl): IDXGISurface1(IDXGISurface1Vtbl) {
    fn GetResource(
        &mut self, This: *mut ::IDXGISurface2, riid: ::REFGUID,
        ppParentResource: *mut *mut ::c_void, pSubresourceIndex: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain1(IDXGISwapChain1Vtbl): IDXGISwapChain(IDXGISwapChainVtbl) {
    fn GetDesc1(
        &mut self, This: *mut ::IDXGISwapChain1, pDesc: *mut ::DXGI_SWAP_CHAIN_DESC1
    ) -> ::HRESULT,
    fn GetFullscreenDesc(
        &mut self, This: *mut ::IDXGISwapChain1, pDesc: *mut ::DXGI_SWAP_CHAIN_FULLSCREEN_DESC
    ) -> ::HRESULT,
    fn GetHwnd(
        &mut self, This: *mut ::IDXGISwapChain1, pHwnd: *mut ::HWND
    ) -> ::HRESULT,
    fn GetCoreWindow(
        &mut self, This: *mut ::IDXGISwapChain1, refiid: ::REFGUID, ppUnk: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn Present1(
        &mut self, This: *mut ::IDXGISwapChain1, SyncInterval: ::UINT, PresentFlags: ::UINT,
        pPresentParameters: *const ::DXGI_PRESENT_PARAMETERS
    ) -> ::HRESULT,
    fn IsTemporaryMonoSupported(
        &mut self, This: *mut ::IDXGISwapChain1
    ) -> ::BOOL,
    fn GetRestrictToOutput(
        &mut self, This: *mut ::IDXGISwapChain1, ppRestrictToOutput: *mut *mut ::IDXGIOutput
    ) -> ::HRESULT,
    fn SetBackgroundColor(
        &mut self, This: *mut ::IDXGISwapChain1, pColor: *const ::DXGI_RGBA
    ) -> ::HRESULT,
    fn GetBackgroundColor(
        &mut self, This: *mut ::IDXGISwapChain1, pColor: *mut ::DXGI_RGBA
    ) -> ::HRESULT,
    fn SetRotation(
        &mut self, This: *mut ::IDXGISwapChain1, Rotation: ::DXGI_MODE_ROTATION
    ) -> ::HRESULT,
    fn GetRotation(
        &mut self, This: *mut ::IDXGISwapChain1, pRotation: *mut ::DXGI_MODE_ROTATION
    ) -> ::HRESULT
});

pub type DXGI_OFFER_RESOURCE_PRIORITY = ::_DXGI_OFFER_RESOURCE_PRIORITY;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: ::UINT = 8;
pub const DXGI_ENUM_MODES_STEREO: ::UINT = 4;
pub const DXGI_SHARED_RESOURCE_READ: ::UINT = 0x80000000;
pub const DXGI_SHARED_RESOURCE_WRITE: ::UINT = 1;
