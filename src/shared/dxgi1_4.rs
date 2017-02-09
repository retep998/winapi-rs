// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_4.h
use ctypes::{c_void};
use shared::basetsd::{UINT64};
use shared::dxgi1_2::{IDXGIAdapter2, IDXGIAdapter2Vtbl};
use shared::dxgi1_3::{
    IDXGIFactory3, IDXGIFactory3Vtbl, IDXGISwapChain2, IDXGISwapChain2Vtbl, IDXGIOutput3Vtbl,
    IDXGIOutput3
};
use shared::dxgiformat::{DXGI_FORMAT};
use shared::dxgitype::{DXGI_COLOR_SPACE_TYPE};
use shared::guiddef::{REFGUID};
use shared::minwindef::{UINT, DWORD};
use um::unknwnbase::{IUnknown};
use um::winnt::{HRESULT, LUID, HANDLE};

ENUM!{ enum DXGI_MEMORY_SEGMENT_GROUP {
    DXGI_MEMORY_SEGMENT_GROUP_LOCAL = 0,
    DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL = 1,
}}

ENUM!{ enum DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
}}

ENUM!{ enum DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT = 0x2,
}}

STRUCT!{struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    Budget: UINT64,
    CurrentUsage: UINT64,
    AvailableForReservation: UINT64,
    CurrentReservation: UINT64,
}}

RIDL!(
interface IDXGIAdapter3(IDXGIAdapter3Vtbl): IDXGIAdapter2(IDXGIAdapter2Vtbl) {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(
        hEvent: HANDLE, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn UnregisterHardwareContentProtectionTeardownStatus(
        dwCookie: DWORD
    ) -> (),
    fn QueryVideoMemoryInfo(
        NodeIndex: UINT, MemorySegmentGroup: DXGI_MEMORY_SEGMENT_GROUP,
        pVideoMemoryInfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO
    ) -> HRESULT,
    fn SetVideoMemoryReservation(
        NodeIndex: UINT, MemorySegmentGroup: DXGI_MEMORY_SEGMENT_GROUP,
        Reservation: UINT64
    ) -> HRESULT,
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(
        hEvent: HANDLE, pdwCookie: *mut DWORD
    ) -> HRESULT,
    fn UnregisterVideoMemoryBudgetChangeNotification(
        dwCookie: DWORD
    ) -> ()
});

RIDL!(
interface IDXGIFactory4(IDXGIFactory4Vtbl): IDXGIFactory3(IDXGIFactory3Vtbl) {
    fn EnumAdapterByLuid(
        AdapterLuid: LUID, riid: REFGUID, ppvAdapter: *mut *mut c_void
    ) -> HRESULT,
    fn EnumWarpAdapter(
        riid: REFGUID, ppvAdapter: *mut *mut c_void
    ) -> HRESULT
});

RIDL!(
interface IDXGIOutput4(IDXGIOutput4Vtbl): IDXGIOutput3(IDXGIOutput3Vtbl) {
    fn CheckOverlayColorSpaceSupport(
        Format: DXGI_FORMAT, ColorSpace: DXGI_COLOR_SPACE_TYPE,
        pConcernedDevice: *mut IUnknown, pFlags: *mut UINT
    ) -> HRESULT
});

RIDL!(
interface IDXGISwapChain3(IDXGISwapChain3Vtbl): IDXGISwapChain2(IDXGISwapChain2Vtbl) {
    fn GetCurrentBackBufferIndex() -> UINT,
    fn CheckColorSpaceSupport(
        ColorSpace: DXGI_COLOR_SPACE_TYPE, pColorSpaceSupport: *mut UINT
    ) -> HRESULT,
    fn SetColorSpace1(
        ColorSpace: DXGI_COLOR_SPACE_TYPE
    ) -> HRESULT,
    fn ResizeBuffers1(
        BufferCount: UINT, Width: UINT, Height: UINT, Format: DXGI_FORMAT,
        SwapChainFlags: UINT, pCreationNodeMask: *const UINT,
        ppPresentQueue: *mut *mut IUnknown
    ) -> HRESULT
});
