// Copyright © 2015; Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi.h
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: ::UINT,
    pub PresentRefreshCount: ::UINT,
    pub SyncRefreshCount: ::UINT,
    pub SyncQPCTime: ::LARGE_INTEGER,
    pub SyncGPUTime: ::LARGE_INTEGER,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_MAPPED_RECT {
    Pitch: ::INT,
    pBits: *mut ::BYTE,
}
#[repr(C)] #[derive(Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [::WCHAR; 128],
    pub VectorId: ::UINT,
    pub DeviceId: ::UINT,
    pub SubSysId: ::UINT,
    pub Revision: ::UINT,
    pub DedicatedVideoMemory: ::SIZE_T,
    pub DedicatedSystemMemory: ::SIZE_T,
    pub SharedSystemMemory: ::SIZE_T,
    pub AdapterLuid: ::LUID,
}
impl Clone for DXGI_ADAPTER_DESC {
    fn clone(&self) -> DXGI_ADAPTER_DESC {
        *self
    }
}
#[repr(C)] #[derive(Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [::WCHAR; 32],
    pub DesktopCoordinates: ::RECT,
    pub AttachedToDesktop: ::BOOL,
    pub Rotation: ::DXGI_MODE_ROTATION,
    pub Monitor: ::HMONITOR,
}
impl Clone for DXGI_OUTPUT_DESC {
    fn clone(&self) -> DXGI_OUTPUT_DESC {
        *self
    }
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: ::HANDLE,
}
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: ::DWORD = 0x28000000;
pub const DXGI_RESOURCE_PRIORITY_LOW: ::DWORD = 0x50000000;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: ::DWORD = 0x78000000;
pub const DXGI_RESOURCE_PRIORITY_HIGH: ::DWORD = 0xa0000000;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: ::DWORD = 0xc8000000;
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_RESIDENCY {
    DXGI_RESIDENCY_FULLY_RESIDENT = 1,
    DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = 2,
    DXGI_RESIDENCY_EVICTED_TO_DISK = 3,
}
pub use self::DXGI_RESIDENCY::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SURFACE_DESC {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Format: ::DXGI_FORMAT,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
}
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD = 0,
    DXGI_SWAP_EFFECT_SEQUENTIAL = 1,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = 3,
}
pub use self::DXGI_SWAP_EFFECT::*;
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_SWAP_CHAIN_FLAG {
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = 1,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 2,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 4,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 8,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 16,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = 32,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 64,
    DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 128,
    DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 256,
    DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = 512,
}
pub use self::DXGI_SWAP_CHAIN_FLAG::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: ::DXGI_MODE_DESC,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
    pub BufferUsage: ::DXGI_USAGE,
    pub BufferCount: ::UINT,
    pub OutputWindow: ::HWND,
    pub Windowed: ::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: ::UINT,
}
RIDL!(
interface IDXGIObject(IDXGIObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetPrivateData(
        &mut self, Name: ::REFGUID, DataSize: ::UINT, pData: *const ::c_void
    ) -> ::HRESULT,
    fn SetPrivateDataInterface(&mut self, Name: ::REFGUID, pUnknown: *const ::IUnknown) -> ::HRESULT,
    fn GetPrivateData(
        &mut self, Name: ::REFGUID, pDataSize: *mut ::UINT, pData: *mut ::c_void
    ) -> ::HRESULT,
    fn GetParent(
        &mut self, riid: ::REFIID, ppParent: *mut *mut ::c_void
    ) -> ::HRESULT
});
RIDL!(
interface IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDevice(&mut self, riid: ::REFIID, ppDevice: *mut *mut ::c_void) -> ::HRESULT
});
RIDL!(
interface IDXGIResource(IDXGIResourceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetSharedHandle(&mut self, pSharedHandle: *mut ::HANDLE) -> ::HRESULT,
    fn GetUsage(&mut self, pUsage: *mut ::DXGI_USAGE) -> ::HRESULT,
    fn SetEvictionPriority(&mut self, EvictionPriority: ::UINT) -> ::HRESULT,
    fn GetEvictionPriority(&mut self, pEvictionPriority: *mut ::UINT) -> ::HRESULT
});
RIDL!(
interface IDXGIKeyedMutex(IDXGIKeyedMutexVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn AcquireSync(&mut self, Key: ::UINT64, dwMilliseconds: ::DWORD) -> ::HRESULT,
    fn ReleaseSync(&mut self, Key: ::UINT64) -> ::HRESULT
});
RIDL!(
interface IDXGISurface(IDXGISurfaceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetDesc(&mut self, pDesc: *mut DXGI_SURFACE_DESC) -> ::HRESULT,
    fn Map(&mut self, pLockedRect: *mut DXGI_MAPPED_RECT, MapFlags: ::UINT) -> ::HRESULT,
    fn Unmap(&mut self) -> ::HRESULT
});
RIDL!(
interface IDXGISurface1(IDXGISurface1Vtbl): IDXGISurface(IDXGISurfaceVtbl) {
    fn GetDC(&mut self, Discard: ::BOOL, phdc: *mut ::HDC) -> ::HRESULT,
    fn ReleaseDC(&mut self, pDirtyRect: *mut ::RECT) -> ::HRESULT
});
RIDL!(
interface IDXGIAdapter(IDXGIAdapterVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumOutputs(&mut self, Output: ::UINT, ppOutput: *mut *mut IDXGIOutput) -> ::HRESULT,
    fn GetDesc(&mut self, pDesc: *mut DXGI_ADAPTER_DESC) -> ::HRESULT,
    fn CheckInterfaceSupport(
        &mut self, InterfaceName: ::REFGUID, pUMDVersion: *mut ::LARGE_INTEGER
    ) -> ::HRESULT
});
RIDL!(
interface IDXGIOutput(IDXGIOutputVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(&mut self, pDesc: *mut DXGI_OUTPUT_DESC) -> ::HRESULT,
    fn GetDisplayModeList(
        &mut self, EnumFormat: ::DXGI_FORMAT, Flags: ::UINT, pNumModes: *mut ::UINT,
        pDesc: *mut ::DXGI_MODE_DESC
    ) -> ::HRESULT,
    fn FindClosestMatchingMode(
        &mut self, pModeToMatch: *const ::DXGI_MODE_DESC, pClosestMatch: *mut ::DXGI_MODE_DESC,
        pConcernedDevice: *mut ::IUnknown
    ) -> ::HRESULT,
    fn WaitForVBlank(&mut self) -> ::HRESULT,
    fn TakeOwnership(&mut self, pDevice: *mut ::IUnknown, Exclusive: ::BOOL) -> ::HRESULT,
    fn ReleaseOwnership(&mut self) -> (),
    fn GetGammaControlCapabilities(
        &mut self, pGammaCaps: *mut ::DXGI_GAMMA_CONTROL_CAPABILITIES
    ) -> ::HRESULT,
    fn SetGammaControl(&mut self, pArray: *const ::DXGI_GAMMA_CONTROL) -> ::HRESULT,
    fn GetGammaControl(&mut self, pArray: *mut ::DXGI_GAMMA_CONTROL) -> ::HRESULT,
    fn SetDisplaySurface(&mut self, pScanoutSurface: *mut IDXGISurface) -> ::HRESULT,
    fn GetDisplaySurfaceData(&mut self, pDestination: *mut IDXGISurface) -> ::HRESULT,
    fn GetFrameStatistics(&mut self, pStats: *mut DXGI_FRAME_STATISTICS) -> ::HRESULT
});
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: ::DWORD = 16;
pub const DXGI_PRESENT_TEST: ::DWORD = 0x00000001;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: ::DWORD = 0x00000002;
pub const DXGI_PRESENT_RESTART: ::DWORD = 0x00000004;
pub const DXGI_PRESENT_DO_NOT_WAIT: ::DWORD = 0x00000008;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: ::DWORD = 0x00000010;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: ::DWORD = 0x00000020;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: ::DWORD = 0x00000040;
pub const DXGI_PRESENT_USE_DURATION: ::DWORD = 0x00000100;
RIDL!(
interface IDXGISwapChain(IDXGISwapChainVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn Present(&mut self, SyncInterval: ::UINT, Flags: ::UINT) -> ::HRESULT,
    fn GetBuffer(
        &mut self, Buffer: ::UINT, riid: ::REFIID, ppSurface: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn SetFullscreenState(&mut self, Fullscreen: ::BOOL, pTarget: *mut IDXGIOutput) -> ::HRESULT,
    fn GetFullscreenState(
        &mut self, pFullscreen: *mut ::BOOL, ppTarget: *mut *mut IDXGIOutput
    ) -> ::HRESULT,
    fn GetDesc(&mut self, pDesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::HRESULT,
    fn ResizeBuffers(
        &mut self, BufferCount: ::UINT, Width: ::UINT, Height: ::UINT, NewFormat: ::DXGI_FORMAT,
        SwapChainFlags: ::UINT
    ) -> ::HRESULT,
    fn ResizeTarget(&mut self, pNewTargetParameters: *const ::DXGI_MODE_DESC) -> ::HRESULT,
    fn GetContainingOutput(&mut self, ppOutput: *mut *mut IDXGIOutput) -> ::HRESULT,
    fn GetFrameStatistics(&mut self, pStats: *mut DXGI_FRAME_STATISTICS) -> ::HRESULT,
    fn GetLastPresentCount(&mut self, pLastPresentCount: *mut ::UINT) -> ::HRESULT
});
RIDL!(
interface IDXGIFactory(IDXGIFactoryVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumAdapters(&mut self, Adapter: ::UINT, ppAdapter: *mut *mut IDXGIAdapter) -> ::HRESULT,
    fn MakeWindowAssociation(&mut self, WindowHandle: ::HWND, Flags: ::UINT) -> ::HRESULT,
    fn GetWindowAssociation(&mut self, pWindowHandle: *mut ::HWND) -> ::HRESULT,
    fn CreateSwapChan(
        &mut self, pDevice: *mut ::IUnknown, pDesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain
    ) -> ::HRESULT,
    fn CreateSoftwareAdapter(
        &mut self, Module: ::HMODULE, ppAdapter: *mut *mut IDXGIAdapter
    ) -> ::HRESULT
});
RIDL!(
interface IDXGIDevice(IDXGIDeviceVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetAdapter(&mut self, pAdapter: *mut *mut IDXGIAdapter) -> ::HRESULT,
    fn CreateSurface(
        &mut self, pDesc: *const DXGI_SURFACE_DESC, NumSurfaces: ::UINT, Usage: ::DXGI_USAGE,
        pSharedResource: *const DXGI_SHARED_RESOURCE, ppSurface: *mut *mut IDXGISurface
    ) -> ::HRESULT,
    fn QueryResourceResidency(
        &mut self, ppResources: *const *mut ::IUnknown, pResidencyStatus: *mut DXGI_RESIDENCY,
        NumResources: ::UINT
    ) -> ::HRESULT,
    fn SetGPUThreadPriority(&mut self, Priority: ::INT) -> ::HRESULT,
    fn GetGPUThreadPriority(&mut self, pPriority: *mut ::INT) -> ::HRESULT
});
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_ADAPTER_FLAG {
    DXGI_ADAPTER_FLAG_NONE,
    DXGI_ADAPTER_FLAG_REMOTE,
    DXGI_ADAPTER_FLAG_SOFTWARE,
}
pub use self::DXGI_ADAPTER_FLAG::*;
#[repr(C)] #[derive(Copy)]
pub struct DXGI_ADAPTER_DESC1 {
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
}
impl Clone for DXGI_ADAPTER_DESC1 {
    fn clone(&self) -> DXGI_ADAPTER_DESC1 {
        *self
    }
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [[::FLOAT; 2]; 8],
    pub WhitePoints: [[::FLOAT; 2]; 16],
}
RIDL!(
interface IDXGIFactory1(IDXGIFactory1Vtbl): IDXGIFactory(IDXGIFactoryVtbl) {
    fn EnumAdapters1(&mut self, Adapter: ::UINT, ppAdapter: *mut *mut IDXGIAdapter1) -> ::HRESULT,
    fn IsCurrent(&mut self) -> ::BOOL
});
RIDL!(
interface IDXGIAdapter1(IDXGIAdapter1Vtbl): IDXGIAdapter(IDXGIAdapterVtbl) {
    fn GetDesc1(&mut self, pDesc: *mut DXGI_ADAPTER_DESC1) -> ::HRESULT
});
RIDL!(
interface IDXGIDevice1(IDXGIDevice1Vtbl): IDXGIDevice(IDXGIDeviceVtbl) {
    fn SetMaximumFrameLatency(&mut self, MaxLatency: ::UINT) -> ::HRESULT,
    fn GetMaximumFrameLatency(&mut self, pMaxLatency: *mut ::UINT) -> ::HRESULT
});
