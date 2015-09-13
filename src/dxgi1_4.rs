// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_4.h

ENUM!{ enum DXGI_MEMORY_SEGMENT_GROUP {
    DXGI_MEMORY_SEGMENT_GROUP_LOCAL = 0,
    DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL = 1,
}}

FLAGS!{ enum DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
}}

FLAGS!{ enum DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT = 0x2,
}}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: ::UINT64,
    pub CurrentUsage: ::UINT64,
    pub AvailableForReservation: ::UINT64,
    pub CurrentReservation: ::UINT64,
}

RIDL!(
interface IDXGIAdapter3(IDXGIAdapter3Vtbl): IDXGIAdapter2(IDXGIAdapter2Vtbl) {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(
        &mut self, This: *mut ::IDXGIAdapter3, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterHardwareContentProtectionTeardownStatus(
        &mut self, This: *mut ::IDXGIAdapter3, dwCookie: ::DWORD
    ) -> (),
    fn QueryVideoMemoryInfo(
        &mut self, This: *mut ::IDXGIAdapter3, NodeIndex: ::UINT,
        MemorySegmentGroup: ::DXGI_MEMORY_SEGMENT_GROUP,
        pVideoMemoryInfo: *mut ::DXGI_QUERY_VIDEO_MEMORY_INFO
    ) -> ::HRESULT,
    fn SetVideoMemoryReservation(
        &mut self, This: *mut ::IDXGIAdapter3, NodeIndex: ::UINT,
        MemorySegmentGroup: ::DXGI_MEMORY_SEGMENT_GROUP, Reservation: ::UINT64
    ) -> ::HRESULT,
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(
        &mut self, This: *mut ::IDXGIAdapter3, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterVideoMemoryBudgetChangeNotification(
        &mut self, This: *mut ::IDXGIAdapter3, dwCookie: ::DWORD
    ) -> ()
});

RIDL!(
interface IDXGIFactory4(IDXGIFactory4Vtbl): IDXGIFactory3(IDXGIFactory3Vtbl) {
    fn EnumAdapterByLuid(
        &mut self, This: *mut ::IDXGIFactory4, AdapterLuid: ::LUID, riid: ::REFGUID,
        ppvAdapter: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn EnumWarpAdapter(
        &mut self, This: *mut ::IDXGIFactory4, riid: ::REFGUID, ppvAdapter: *mut *mut ::c_void
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput4(IDXGIOutput4Vtbl): IDXGIOutput3(IDXGIOutput3Vtbl) {
    fn CheckOverlayColorSpaceSupport(
        &mut self, This: *mut ::IDXGIOutput4, Format: ::DXGI_FORMAT,
        ColorSpace: ::DXGI_COLOR_SPACE_TYPE, pConcernedDevice: *mut ::IUnknown, pFlags: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain3(IDXGISwapChain3Vtbl): IDXGISwapChain2(IDXGISwapChain2Vtbl) {
    fn GetCurrentBackBufferIndex(
        &mut self, This: *mut ::IDXGISwapChain3
    ) -> ::UINT,
    fn CheckColorSpaceSupport(
        &mut self, This: *mut ::IDXGISwapChain3, ColorSpace: ::DXGI_COLOR_SPACE_TYPE,
        pColorSpaceSupport: *mut ::UINT
    ) -> ::HRESULT,
    fn SetColorSpace1(
        &mut self, This: *mut ::IDXGISwapChain3, ColorSpace: ::DXGI_COLOR_SPACE_TYPE
    ) -> ::HRESULT,
    fn ResizeBuffers1(
        &mut self, This: *mut ::IDXGISwapChain3, BufferCount: ::UINT, Width: ::UINT,
        Height: ::UINT, Format: ::DXGI_FORMAT, SwapChainFlags: ::UINT,
        pCreationNodeMask: *const ::UINT, ppPresentQueue: *mut *mut ::IUnknown
    ) -> ::HRESULT
});

