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

ENUM!{ enum DXGI_RESIDENCY {
    DXGI_RESIDENCY_FULLY_RESIDENT = 1,
    DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = 2,
    DXGI_RESIDENCY_EVICTED_TO_DISK = 3,
}}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SURFACE_DESC {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub Format: ::DXGI_FORMAT,
    pub SampleDesc: ::DXGI_SAMPLE_DESC,
}

ENUM!{ enum DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD = 0,
    DXGI_SWAP_EFFECT_SEQUENTIAL = 1,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = 3,
    DXGI_SWAP_EFFECT_FLIP_DISCARD = 4,
}}

FLAGS!{ enum DXGI_SWAP_CHAIN_FLAG {
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = 0x1,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 0x2,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 0x4,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 0x8,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 0x10,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = 0x20,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 0x40,
    DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 0x80,
    DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 0x100,
    DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = 0x200,
    DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED = 0x400,
}}

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
    fn CreateSwapChain(
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

FLAGS!{ enum DXGI_ADAPTER_FLAG {
    DXGI_ADAPTER_FLAG_NONE = 0x0,
    DXGI_ADAPTER_FLAG_REMOTE = 0x1,
    DXGI_ADAPTER_FLAG_SOFTWARE = 0x2,
    DXGI_ADAPTER_FLAG_FORCE_DWORD = 0xFFFFFFFF,
}}

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

pub type DXGI_USAGE = ::UINT;

pub const DXGI_CPU_ACCESS_DYNAMIC: ::UINT = 1;
pub const DXGI_CPU_ACCESS_FIELD: ::UINT = 15;
pub const DXGI_CPU_ACCESS_NONE: ::UINT = 0;
pub const DXGI_CPU_ACCESS_READ_WRITE: ::UINT = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: ::UINT = 3;
pub const DXGI_ENUM_MODES_INTERLACED: ::UINT = 1;
pub const DXGI_ENUM_MODES_SCALING: ::UINT = 2;
pub const DXGI_MAP_DISCARD: ::UINT = 4;
pub const DXGI_MAP_READ: ::UINT = 1;
pub const DXGI_MAP_WRITE: ::UINT = 2;
pub const DXGI_MWA_VALID: ::UINT = 0x7;
pub const DXGI_USAGE_BACK_BUFFER: ::UINT = 0x00000040;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: ::UINT = 0x00000200;
pub const DXGI_USAGE_READ_ONLY: ::UINT = 0x00000100;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: ::UINT = 0x00000020;
pub const DXGI_USAGE_SHADER_INPUT: ::UINT = 0x00000010;
pub const DXGI_USAGE_SHARED: ::UINT = 0x00000080;
pub const DXGI_USAGE_UNORDERED_ACCESS: ::UINT = 0x00000400;

DEFINE_GUID!(IID_IDXGIObject,0xaec22fb8,0x76f3,0x4639,0x9b,0xe0,0x28,0xeb,0x43,
    0xa6,0x7a,0x2e);
DEFINE_GUID!(IID_IDXGIDeviceSubObject,0x3d3e0379,0xf9de,0x4d58,0xbb,0x6c,0x18,
    0xd6,0x29,0x92,0xf1,0xa6);
DEFINE_GUID!(IID_IDXGIResource,0x035f3ab4,0x482e,0x4e50,0xb4,0x1f,0x8a,0x7f,
    0x8b,0xd8,0x96,0x0b);
DEFINE_GUID!(IID_IDXGIKeyedMutex,0x9d8e1289,0xd7b3,0x465f,0x81,0x26,0x25,0x0e,
    0x34,0x9a,0xf8,0x5d);
DEFINE_GUID!(IID_IDXGISurface,0xcafcb56c,0x6ac3,0x4889,0xbf,0x47,0x9e,0x23,
    0xbb,0xd2,0x60,0xec);
DEFINE_GUID!(IID_IDXGISurface1,0x4AE63092,0x6327,0x4c1b,0x80,0xAE,0xBF,0xE1,
    0x2E,0xA3,0x2B,0x86);
DEFINE_GUID!(IID_IDXGIAdapter,0x2411e7e1,0x12ac,0x4ccf,0xbd,0x14,0x97,0x98,
    0xe8,0x53,0x4d,0xc0);
DEFINE_GUID!(IID_IDXGIOutput,0xae02eedb,0xc735,0x4690,0x8d,0x52,0x5a,0x8d,
    0xc2,0x02,0x13,0xaa);
DEFINE_GUID!(IID_IDXGISwapChain,0x310d36a0,0xd2e7,0x4c0a,0xaa,0x04,0x6a,0x9d,
    0x23,0xb8,0x88,0x6a);
DEFINE_GUID!(IID_IDXGIFactory,0x7b7166ec,0x21c7,0x44ae,0xb2,0x1a,0xc9,0xae,
    0x32,0x1a,0xe3,0x69);
DEFINE_GUID!(IID_IDXGIDevice,0x54ec77fa,0x1377,0x44e6,0x8c,0x32,0x88,0xfd,0x5f,
    0x44,0xc8,0x4c);
DEFINE_GUID!(IID_IDXGIFactory1,0x770aae78,0xf26f,0x4dba,0xa8,0x29,0x25,0x3c,
    0x83,0xd1,0xb3,0x87);
DEFINE_GUID!(IID_IDXGIAdapter1,0x29038f61,0x3839,0x4626,0x91,0xfd,0x08,0x68,
    0x79,0x01,0x1a,0x05);
DEFINE_GUID!(IID_IDXGIDevice1,0x77db970f,0x6276,0x48ba,0xba,0x28,0x07,0x01,
    0x43,0xb4,0x39,0x2c);
