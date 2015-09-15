// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_2.h

ENUM!{ enum DXGI_ALPHA_MODE {
    DXGI_ALPHA_MODE_UNSPECIFIED = 0,
    DXGI_ALPHA_MODE_PREMULTIPLIED = 1,
    DXGI_ALPHA_MODE_STRAIGHT = 2,
    DXGI_ALPHA_MODE_IGNORE = 3,
    DXGI_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}

ENUM!{ enum DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY = 0,
    DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY = 1,
    DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY = 2,
    DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY = 3,
    DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY = 4,
}}

ENUM!{ enum DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY = 0,
    DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY = 1,
    DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY = 2,
    DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY = 3,
    DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY = 4,
}}

ENUM!{ enum DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME = 1,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR = 2,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR = 4,
}}

ENUM!{ enum DXGI_SCALING {
    DXGI_SCALING_STRETCH = 0,
    DXGI_SCALING_NONE = 1,
    DXGI_SCALING_ASPECT_RATIO_STRETCH = 2,
}}

ENUM!{ enum _DXGI_OFFER_RESOURCE_PRIORITY {
    DXGI_OFFER_RESOURCE_PRIORITY_LOW = 1,
    DXGI_OFFER_RESOURCE_PRIORITY_NORMAL = 2,
    DXGI_OFFER_RESOURCE_PRIORITY_HIGH = 3,
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
    fn GetDesc2(&mut self, pDesc: *mut ::DXGI_ADAPTER_DESC2) -> ::HRESULT
});

RIDL!(
interface IDXGIDevice2(IDXGIDevice2Vtbl): IDXGIDevice1(IDXGIDevice1Vtbl) {
    fn OfferResources(
        &mut self, NumResources: ::UINT, ppResources: *mut *mut ::IDXGIResource,
        Priority: ::DXGI_OFFER_RESOURCE_PRIORITY
    ) -> ::HRESULT,
    fn ReclaimResources(
        &mut self, NumResources: ::UINT, ppResources: *mut *mut ::IDXGIResource,
        pDiscarded: *mut ::BOOL
    ) -> ::HRESULT,
    fn EnqueueSetEvent(&mut self, hEvent: ::HANDLE) -> ::HRESULT
});

RIDL!(
interface IDXGIDisplayControl(IDXGIDisplayControlVtbl): IUnknown(IUnknownVtbl) {
    fn IsStereoEnabled(&mut self) -> ::BOOL,
    fn SetStereoEnabled(&mut self, enabled: ::BOOL) -> ()
});

RIDL!(
interface IDXGIFactory2(IDXGIFactory2Vtbl): IDXGIFactory1(IDXGIFactory1Vtbl) {
    fn IsWindowedStereoEnabled(&mut self) -> ::BOOL,
    fn CreateSwapChainForHwnd(
        &mut self, pDevice: *mut ::IUnknown, hWnd: ::HWND, pDesc: *const ::DXGI_SWAP_CHAIN_DESC1,
        pFullscreenDesc: *const ::DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        pRestrictToOutput: *mut ::IDXGIOutput, ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn CreateSwapChainForCoreWindow(
        &mut self, pDevice: *mut ::IUnknown, pWindow: *mut ::IUnknown,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut ::IDXGIOutput,
        ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn GetSharedResourceAdapterLuid(
        &mut self, hResource: ::HANDLE, pLuid: *mut ::LUID
    ) -> ::HRESULT,
    fn RegisterStereoStatusWindow(
        &mut self, WindowHandle: ::HWND, wMsg: ::UINT, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn RegisterStereoStatusEvent(
        &mut self, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterStereoStatus(&mut self, dwCookie: ::DWORD) -> (),
    fn RegisterOcclusionStatusWindow(
        &mut self, WindowHandle: ::HWND, wMsg: ::UINT, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn RegisterOcclusionStatusEvent(
        &mut self, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterOcclusionStatus(&mut self, dwCookie: ::DWORD) -> (),
    fn CreateSwapChainForComposition(
        &mut self, pDevice: *mut ::IUnknown, pDesc: *const ::DXGI_SWAP_CHAIN_DESC1,
        pRestrictToOutput: *mut ::IDXGIOutput, ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput1(IDXGIOutput1Vtbl): IDXGIOutput(IDXGIOutputVtbl) {
    fn GetDisplayModeList1(
        &mut self, EnumFormat: ::DXGI_FORMAT, Flags: ::UINT, pNumModes: *mut ::UINT,
        pDesc: *mut ::DXGI_MODE_DESC1
    ) -> ::HRESULT,
    fn FindClosestMatchingMode1(
        &mut self, pModeToMatch: *const ::DXGI_MODE_DESC1, pClosestMatch: *mut ::DXGI_MODE_DESC1,
        pConcernedDevice: *mut ::IUnknown
    ) -> ::HRESULT,
    fn GetDisplaySurfaceData1(
        &mut self, pDestination: *mut ::IDXGIResource
    ) -> ::HRESULT,
    fn DuplicateOutput(
        &mut self, pDevice: *mut ::IUnknown,
        ppOutputDuplication: *mut *mut ::IDXGIOutputDuplication
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutputDuplication(IDXGIOutputDuplicationVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(&mut self, pDesc: *mut ::DXGI_OUTDUPL_DESC) -> (),
    fn AcquireNextFrame(
        &mut self, TimeoutInMilliseconds: ::UINT, pFrameInfo: *mut ::DXGI_OUTDUPL_FRAME_INFO,
        ppDesktopResource: *mut *mut ::IDXGIResource
    ) -> ::HRESULT,
    fn GetFrameDirtyRects(
        &mut self, DirtyRectsBufferSize: ::UINT, pDirtyRectsBuffer: *mut ::RECT,
        pDirtyRectsBufferSizeRequired: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFrameMoveRects(
        &mut self, MoveRectsBufferSize: ::UINT, pMoveRectBuffer: *mut ::DXGI_OUTDUPL_MOVE_RECT,
        pMoveRectsBufferSizeRequired: *mut ::UINT
    ) -> ::HRESULT,
    fn GetFramePointerShape(
        &mut self, PointerShapeBufferSize: ::UINT, pPointerShapeBuffer: *mut ::c_void,
        pPointerShapeBufferSizeRequired: *mut ::UINT,
        pPointerShapeInfo: *mut ::DXGI_OUTDUPL_POINTER_SHAPE_INFO
    ) -> ::HRESULT,
    fn MapDesktopSurface(
        &mut self, pLockedRect: *mut ::DXGI_MAPPED_RECT
    ) -> ::HRESULT,
    fn UnMapDesktopSurface(&mut self) -> ::HRESULT,
    fn ReleaseFrame(&mut self) -> ::HRESULT
});

RIDL!(
interface IDXGIResource1(IDXGIResource1Vtbl): IDXGIResource(IDXGIResourceVtbl) {
    fn CreateSubresourceSurface(
        &mut self, index: ::UINT, ppSurface: *mut *mut ::IDXGISurface2
    ) -> ::HRESULT,
    fn CreateSharedHandle(
        &mut self, pAttributes: *const ::SECURITY_ATTRIBUTES, dwAccess: ::DWORD, lpName: ::LPCWSTR,
        pHandle: *mut ::HANDLE
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISurface2(IDXGISurface2Vtbl): IDXGISurface1(IDXGISurface1Vtbl) {
    fn GetResource(
        &mut self, riid: ::REFGUID, ppParentResource: *mut *mut ::c_void,
        pSubresourceIndex: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain1(IDXGISwapChain1Vtbl): IDXGISwapChain(IDXGISwapChainVtbl) {
    fn GetDesc1(&mut self, pDesc: *mut ::DXGI_SWAP_CHAIN_DESC1) -> ::HRESULT,
    fn GetFullscreenDesc(
        &mut self, pDesc: *mut ::DXGI_SWAP_CHAIN_FULLSCREEN_DESC
    ) -> ::HRESULT,
    fn GetHwnd(&mut self, pHwnd: *mut ::HWND) -> ::HRESULT,
    fn GetCoreWindow(
        &mut self, refiid: ::REFGUID, ppUnk: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn Present1(
        &mut self, SyncInterval: ::UINT, PresentFlags: ::UINT,
        pPresentParameters: *const ::DXGI_PRESENT_PARAMETERS
    ) -> ::HRESULT,
    fn IsTemporaryMonoSupported(&mut self) -> ::BOOL,
    fn GetRestrictToOutput(
        &mut self, ppRestrictToOutput: *mut *mut ::IDXGIOutput
    ) -> ::HRESULT,
    fn SetBackgroundColor(&mut self, pColor: *const ::DXGI_RGBA) -> ::HRESULT,
    fn GetBackgroundColor(&mut self, pColor: *mut ::DXGI_RGBA) -> ::HRESULT,
    fn SetRotation(&mut self, Rotation: ::DXGI_MODE_ROTATION) -> ::HRESULT,
    fn GetRotation(&mut self, pRotation: *mut ::DXGI_MODE_ROTATION) -> ::HRESULT
});

pub type DXGI_OFFER_RESOURCE_PRIORITY = ::_DXGI_OFFER_RESOURCE_PRIORITY;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: ::UINT = 8;
pub const DXGI_ENUM_MODES_STEREO: ::UINT = 4;
pub const DXGI_SHARED_RESOURCE_READ: ::UINT = 0x80000000;
pub const DXGI_SHARED_RESOURCE_WRITE: ::UINT = 1;

DEFINE_GUID!(IID_IDXGIDisplayControl,0xea9dbf1a,0xc88e,0x4486,0x85,0x4a,0x98,
    0xaa,0x01,0x38,0xf3,0x0c);
DEFINE_GUID!(IID_IDXGIOutputDuplication,0x191cfac3,0xa341,0x470d,0xb2,0x6e,
    0xa8,0x64,0xf4,0x28,0x31,0x9c);
DEFINE_GUID!(IID_IDXGISurface2,0xaba496dd,0xb617,0x4cb8,0xa8,0x66,0xbc,0x44,
    0xd7,0xeb,0x1f,0xa2);
DEFINE_GUID!(IID_IDXGIResource1,0x30961379,0x4609,0x4a41,0x99,0x8e,0x54,0xfe,
    0x56,0x7e,0xe0,0xc1);
DEFINE_GUID!(IID_IDXGIDevice2,0x05008617,0xfbfd,0x4051,0xa7,0x90,0x14,0x48,
    0x84,0xb4,0xf6,0xa9);
DEFINE_GUID!(IID_IDXGISwapChain1,0x790a45f7,0x0d42,0x4876,0x98,0x3a,0x0a,0x55,
    0xcf,0xe6,0xf4,0xaa);
DEFINE_GUID!(IID_IDXGIFactory2,0x50c83a1c,0xe072,0x4c48,0x87,0xb0,0x36,0x30,
    0xfa,0x36,0xa6,0xd0);
DEFINE_GUID!(IID_IDXGIAdapter2,0x0AA1AE0A,0xFA0E,0x4B84,0x86,0x44,0xE0,0x5F,
    0xF8,0xE5,0xAC,0xB5);
DEFINE_GUID!(IID_IDXGIOutput1,0x00cddea8,0x939b,0x4b83,0xa3,0x40,0xa6,0x85,
    0x22,0x66,0x66,0xcc);
