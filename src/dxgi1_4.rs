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
        &mut self, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterHardwareContentProtectionTeardownStatus(
        &mut self, dwCookie: ::DWORD
    ) -> (),
    fn QueryVideoMemoryInfo(
        &mut self, NodeIndex: ::UINT, MemorySegmentGroup: ::DXGI_MEMORY_SEGMENT_GROUP,
        pVideoMemoryInfo: *mut ::DXGI_QUERY_VIDEO_MEMORY_INFO
    ) -> ::HRESULT,
    fn SetVideoMemoryReservation(
        &mut self, NodeIndex: ::UINT, MemorySegmentGroup: ::DXGI_MEMORY_SEGMENT_GROUP,
        Reservation: ::UINT64
    ) -> ::HRESULT,
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(
        &mut self, hEvent: ::HANDLE, pdwCookie: *mut ::DWORD
    ) -> ::HRESULT,
    fn UnregisterVideoMemoryBudgetChangeNotification(
        &mut self, dwCookie: ::DWORD
    ) -> ()
});

RIDL!(
interface IDXGIFactory4(IDXGIFactory4Vtbl): IDXGIFactory3(IDXGIFactory3Vtbl) {
    fn EnumAdapterByLuid(
        &mut self, AdapterLuid: ::LUID, riid: ::REFGUID, ppvAdapter: *mut *mut ::c_void
    ) -> ::HRESULT,
    fn EnumWarpAdapter(
        &mut self, riid: ::REFGUID, ppvAdapter: *mut *mut ::c_void
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput4(IDXGIOutput4Vtbl): IDXGIOutput3(IDXGIOutput3Vtbl) {
    fn CheckOverlayColorSpaceSupport(
        &mut self, Format: ::DXGI_FORMAT, ColorSpace: ::DXGI_COLOR_SPACE_TYPE,
        pConcernedDevice: *mut ::IUnknown, pFlags: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain3(IDXGISwapChain3Vtbl): IDXGISwapChain2(IDXGISwapChain2Vtbl) {
    fn GetCurrentBackBufferIndex(&mut self) -> ::UINT,
    fn CheckColorSpaceSupport(
        &mut self, ColorSpace: ::DXGI_COLOR_SPACE_TYPE, pColorSpaceSupport: *mut ::UINT
    ) -> ::HRESULT,
    fn SetColorSpace1(
        &mut self, ColorSpace: ::DXGI_COLOR_SPACE_TYPE
    ) -> ::HRESULT,
    fn ResizeBuffers1(
        &mut self, BufferCount: ::UINT, Width: ::UINT, Height: ::UINT, Format: ::DXGI_FORMAT,
        SwapChainFlags: ::UINT, pCreationNodeMask: *const ::UINT,
        ppPresentQueue: *mut *mut ::IUnknown
    ) -> ::HRESULT
});

DEFINE_GUID!(IID_IDXGISwapChain3,0x94d99bdb,0xf1f8,0x4ab0,0xb2,0x36,0x7d,0xa0,
    0x17,0x0e,0xda,0xb1);
DEFINE_GUID!(IID_IDXGIOutput4,0xdc7dca35,0x2196,0x414d,0x9F,0x53,0x61,0x78,
    0x84,0x03,0x2a,0x60);
DEFINE_GUID!(IID_IDXGIFactory4,0x1bc6ea02,0xef36,0x464f,0xbf,0x0c,0x21,0xca,
    0x39,0xe5,0x16,0x8a);
DEFINE_GUID!(IID_IDXGIAdapter3,0x645967A4,0x1392,0x4310,0xA7,0x98,0x80,0x53,
    0xCE,0x3E,0x93,0xFD);
