// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_2.h
use ctypes::{c_void};
use shared::basetsd::{SIZE_T};
use shared::dxgi::{
    DXGI_MAPPED_RECT, DXGI_SWAP_EFFECT, IDXGIAdapter1, IDXGIAdapter1Vtbl, IDXGIDevice1,
    IDXGIDevice1Vtbl, IDXGIFactory1, IDXGIFactory1Vtbl, IDXGIObject, IDXGIObjectVtbl, IDXGIOutput,
    IDXGIOutputVtbl, IDXGISwapChain, IDXGISwapChainVtbl, IDXGISurface1, IDXGISurface1Vtbl,
    IDXGIResource, IDXGIResourceVtbl
};
use shared::dxgiformat::{DXGI_FORMAT};
use shared::dxgitype::{
    DXGI_MODE_ROTATION, DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_USAGE, DXGI_RGBA, DXGI_RATIONAL,
    DXGI_MODE_SCANLINE_ORDER, DXGI_MODE_SCALING
};
use shared::guiddef::{REFGUID};
use shared::minwindef::{UINT, BOOL, DWORD};
use shared::windef::{HWND, RECT, POINT};
use um::minwinbase::{SECURITY_ATTRIBUTES};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, HANDLE, LPCWSTR, LUID, LARGE_INTEGER, WCHAR};

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

STRUCT!{struct DXGI_ADAPTER_DESC2 {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
    Flags: UINT,
    GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}}

STRUCT!{struct DXGI_MODE_DESC1 {
    Width: UINT,
    Height: UINT,
    RefreshRate: DXGI_RATIONAL,
    Format: DXGI_FORMAT,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
    Stereo: BOOL,
}}

STRUCT!{struct DXGI_OUTDUPL_DESC {
    ModeDesc: DXGI_MODE_DESC,
    Rotation: DXGI_MODE_ROTATION,
    DesktopImageInSystemMemory: BOOL,
}}

STRUCT!{struct DXGI_OUTDUPL_FRAME_INFO {
    LastPresentTime: LARGE_INTEGER,
    LastMouseUpdateTime: LARGE_INTEGER,
    AccumulatedFrames: UINT,
    RectsCoalesced: BOOL,
    ProtectedContentMaskedOut: BOOL,
    PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    TotalMetadataBufferSize: UINT,
    PointerShapeBufferSize: UINT,
}}

STRUCT!{struct DXGI_OUTDUPL_MOVE_RECT {
    SourcePoint: POINT,
    DestinationRect: RECT,
}}

STRUCT!{struct DXGI_OUTDUPL_POINTER_POSITION {
    Position: POINT,
    Visible: BOOL,
}}

STRUCT!{struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    Type: UINT,
    Width: UINT,
    Height: UINT,
    Pitch: UINT,
    HotSpot: POINT,
}}

STRUCT!{struct DXGI_PRESENT_PARAMETERS {
    DirtyRectsCount: UINT,
    pDirtyRects: *mut RECT,
    pScrollRect: *mut RECT,
    pScrollOffset: *mut POINT,
}}

STRUCT!{struct DXGI_SWAP_CHAIN_DESC1 {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    Stereo: BOOL,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    Scaling: DXGI_SCALING,
    SwapEffect: DXGI_SWAP_EFFECT,
    AlphaMode: DXGI_ALPHA_MODE,
    Flags: UINT,
}}

STRUCT!{struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    RefreshRate: DXGI_RATIONAL,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
    Windowed: BOOL,
}}

RIDL!(
interface IDXGIAdapter2(IDXGIAdapter2Vtbl): IDXGIAdapter1(IDXGIAdapter1Vtbl) {
    fn GetDesc2(&self, pDesc: *mut DXGI_ADAPTER_DESC2) -> HRESULT
});

RIDL!(
interface IDXGIDevice2(IDXGIDevice2Vtbl): IDXGIDevice1(IDXGIDevice1Vtbl) {
    fn OfferResources(
        &self, NumResources: UINT, ppResources: *mut *mut IDXGIResource,
        Priority: DXGI_OFFER_RESOURCE_PRIORITY
    ) -> HRESULT,
    fn ReclaimResources(
        &self, NumResources: UINT, ppResources: *mut *mut IDXGIResource,
        pDiscarded: *mut BOOL
    ) -> HRESULT,
    fn EnqueueSetEvent(&self, hEvent: HANDLE) -> HRESULT
});

RIDL!(
interface IDXGIDisplayControl(IDXGIDisplayControlVtbl): IUnknown(IUnknownVtbl) {
    fn IsStereoEnabled(&self) -> BOOL,
    fn SetStereoEnabled(&self, enabled: BOOL) -> ()
});

RIDL!(
interface IDXGIFactory2(IDXGIFactory2Vtbl): IDXGIFactory1(IDXGIFactory1Vtbl) {
    fn IsWindowedStereoEnabled(&self) -> BOOL,
    fn CreateSwapChainForHwnd(
        &self, pDevice: *mut IUnknown, hWnd: HWND, pDesc: *const DXGI_SWAP_CHAIN_DESC1,
        pFullscreenDesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut *mut IDXGISwapChain1
    ) -> HRESULT,
    fn CreateSwapChainForCoreWindow(
        &self, pDevice: *mut IUnknown, pWindow: *mut IUnknown,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1
    ) -> HRESULT,
    fn GetSharedResourceAdapterLuid(
        &self, hResource: HANDLE, pLuid: *mut LUID
    ) -> HRESULT,
    fn RegisterStereoStatusWindow(
        &self, WindowHandle: HWND, wMsg: UINT, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn RegisterStereoStatusEvent(
        &self, hEvent: HANDLE, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn UnregisterStereoStatus(&self, dwCookie: DWORD) -> (),
    fn RegisterOcclusionStatusWindow(
        &self, WindowHandle: HWND, wMsg: UINT, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn RegisterOcclusionStatusEvent(
        &self, hEvent: HANDLE, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn UnregisterOcclusionStatus(&self, dwCookie: DWORD) -> (),
    fn CreateSwapChainForComposition(
        &self, pDevice: *mut IUnknown, pDesc: *const DXGI_SWAP_CHAIN_DESC1,
        pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut *mut IDXGISwapChain1
    ) -> HRESULT
});

RIDL!(
interface IDXGIOutput1(IDXGIOutput1Vtbl): IDXGIOutput(IDXGIOutputVtbl) {
    fn GetDisplayModeList1(
        &self, EnumFormat: DXGI_FORMAT, Flags: UINT, pNumModes: *mut UINT,
        pDesc: *mut DXGI_MODE_DESC1
    ) -> HRESULT,
    fn FindClosestMatchingMode1(
        &self, pModeToMatch: *const DXGI_MODE_DESC1, pClosestMatch: *mut DXGI_MODE_DESC1,
        pConcernedDevice: *mut IUnknown
    ) -> HRESULT,
    fn GetDisplaySurfaceData1(
        &self, pDestination: *mut IDXGIResource
    ) -> HRESULT,
    fn DuplicateOutput(
        &self, pDevice: *mut IUnknown,
        ppOutputDuplication: *mut *mut IDXGIOutputDuplication
    ) -> HRESULT
});

RIDL!(
interface IDXGIOutputDuplication(IDXGIOutputDuplicationVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(&self, pDesc: *mut DXGI_OUTDUPL_DESC) -> (),
    fn AcquireNextFrame(
        &self, TimeoutInMilliseconds: UINT, pFrameInfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppDesktopResource: *mut *mut IDXGIResource
    ) -> HRESULT,
    fn GetFrameDirtyRects(
        &self, DirtyRectsBufferSize: UINT, pDirtyRectsBuffer: *mut RECT,
        pDirtyRectsBufferSizeRequired: *mut UINT
    ) -> HRESULT,
    fn GetFrameMoveRects(
        &self, MoveRectsBufferSize: UINT, pMoveRectBuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pMoveRectsBufferSizeRequired: *mut UINT
    ) -> HRESULT,
    fn GetFramePointerShape(
        &self, PointerShapeBufferSize: UINT, pPointerShapeBuffer: *mut c_void,
        pPointerShapeBufferSizeRequired: *mut UINT,
        pPointerShapeInfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO
    ) -> HRESULT,
    fn MapDesktopSurface(
        &self, pLockedRect: *mut DXGI_MAPPED_RECT
    ) -> HRESULT,
    fn UnMapDesktopSurface(&self) -> HRESULT,
    fn ReleaseFrame(&self) -> HRESULT
});

RIDL!(
interface IDXGIResource1(IDXGIResource1Vtbl): IDXGIResource(IDXGIResourceVtbl) {
    fn CreateSubresourceSurface(
        &self, index: UINT, ppSurface: *mut *mut IDXGISurface2
    ) -> HRESULT,
    fn CreateSharedHandle(
        &self, pAttributes: *const SECURITY_ATTRIBUTES, dwAccess: DWORD, lpName: LPCWSTR,
        pHandle: *mut HANDLE
    ) -> HRESULT
});

RIDL!(
interface IDXGISurface2(IDXGISurface2Vtbl): IDXGISurface1(IDXGISurface1Vtbl) {
    fn GetResource(
        &self, riid: REFGUID, ppParentResource: *mut *mut c_void,
        pSubresourceIndex: *mut UINT
    ) -> HRESULT
});

RIDL!(
interface IDXGISwapChain1(IDXGISwapChain1Vtbl): IDXGISwapChain(IDXGISwapChainVtbl) {
    fn GetDesc1(&self, pDesc: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
    fn GetFullscreenDesc(
        &self, pDesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC
    ) -> HRESULT,
    fn GetHwnd(&self, pHwnd: *mut HWND) -> HRESULT,
    fn GetCoreWindow(
        &self, refiid: REFGUID, ppUnk: *mut *mut c_void
    ) -> HRESULT,
    fn Present1(
        &self, SyncInterval: UINT, PresentFlags: UINT,
        pPresentParameters: *const DXGI_PRESENT_PARAMETERS
    ) -> HRESULT,
    fn IsTemporaryMonoSupported(&self) -> BOOL,
    fn GetRestrictToOutput(
        &self, ppRestrictToOutput: *mut *mut IDXGIOutput
    ) -> HRESULT,
    fn SetBackgroundColor(&self, pColor: *const DXGI_RGBA) -> HRESULT,
    fn GetBackgroundColor(&self, pColor: *mut DXGI_RGBA) -> HRESULT,
    fn SetRotation(&self, Rotation: DXGI_MODE_ROTATION) -> HRESULT,
    fn GetRotation(&self, pRotation: *mut DXGI_MODE_ROTATION) -> HRESULT
});

pub type DXGI_OFFER_RESOURCE_PRIORITY = _DXGI_OFFER_RESOURCE_PRIORITY;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: UINT = 8;
pub const DXGI_ENUM_MODES_STEREO: UINT = 4;
pub const DXGI_SHARED_RESOURCE_READ: UINT = 0x80000000;
pub const DXGI_SHARED_RESOURCE_WRITE: UINT = 1;
