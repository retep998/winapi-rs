// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Interfaces provided by DXGI.
//!
//! # References
//! [DXGI Interfaces, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff471322(v=vs.85).aspx)

#![allow(non_snake_case, dead_code)]

use winapi::{ HRESULT, ULONG, REFGUID,
    UINT, LPCWSTR, REFIID,
    LARGE_INTEGER, GUID, HWND,
    HMODULE, BOOL, INT,
    HANDLE, RECT, LUID,
    DWORD, UINT64, LPCSTR,
    SIZE_T, SECURITY_ATTRIBUTES, c_void };

use structures::*;
use enumerations::*;
use constants::*;

#[repr(C)] pub struct IUnknown { pub vtable: *mut IUnknownVtbl }
#[repr(C)] pub struct IDXGIAdapter { pub vtable: *mut IDXGIAdapterVtbl }
#[repr(C)] pub struct IDXGIAdapter1 { pub vtable: *mut IDXGIAdapter1Vtbl }
#[repr(C)] pub struct IDXGIAdapter2 { pub vtable: *mut IDXGIAdapter2Vtbl }
#[repr(C)] pub struct IDXGIDebug { pub vtable: *mut IDXGIDebugVtbl }
#[repr(C)] pub struct IDXGIDebug1 { pub vtable: *mut IDXGIDebug1Vtbl }
#[repr(C)] pub struct IDXGIDecodeSwapChain { pub vtable: *mut IDXGIDecodeSwapChainVtbl }
#[repr(C)] pub struct IDXGIDevice { pub vtable: *mut IDXGIDeviceVtbl }
#[repr(C)] pub struct IDXGIDevice1 { pub vtable: *mut IDXGIDevice1Vtbl }
#[repr(C)] pub struct IDXGIDevice2 { pub vtable: *mut IDXGIDevice2Vtbl }
#[repr(C)] pub struct IDXGIDevice3 { pub vtable: *mut IDXGIDevice3Vtbl }
#[repr(C)] pub struct IDXGIDeviceSubObject { pub vtable: *mut IDXGIDeviceSubObjectVtbl }
#[repr(C)] pub struct IDXGIDisplayControl { pub vtable: *mut IDXGIDisplayControlVtbl }
#[repr(C)] pub struct IDXGIFactory { pub vtable: *mut IDXGIFactoryVtbl }
#[repr(C)] pub struct IDXGIFactory1 { pub vtable: *mut IDXGIFactory1Vtbl }
#[repr(C)] pub struct IDXGIFactory2 { pub vtable: *mut IDXGIFactory2Vtbl }
#[repr(C)] pub struct IDXGIFactory3 { pub vtable: *mut IDXGIFactory3Vtbl }
#[repr(C)] pub struct IDXGIFactoryMedia { pub vtable: *mut IDXGIFactoryMediaVtbl }
#[repr(C)] pub struct IDXGIInfoQueue { pub vtable: *mut IDXGIInfoQueueVtbl }
#[repr(C)] pub struct IDXGIKeyedMutex { pub vtable: *mut IDXGIKeyedMutexVtbl }
#[repr(C)] pub struct IDXGIObject { pub vtable: *mut IDXGIObjectVtbl }
#[repr(C)] pub struct IDXGIOutput { pub vtable: *mut IDXGIOutputVtbl }
#[repr(C)] pub struct IDXGIOutput1 { pub vtable: *mut IDXGIOutput1Vtbl }
#[repr(C)] pub struct IDXGIOutput2 { pub vtable: *mut IDXGIOutput2Vtbl }
#[repr(C)] pub struct IDXGIOutputDuplication { pub vtable: *mut IDXGIOutputDuplicationVtbl }
#[repr(C)] pub struct IDXGIResource { pub vtable: *mut IDXGIResourceVtbl }
#[repr(C)] pub struct IDXGIResource1 { pub vtable: *mut IDXGIResource1Vtbl }
#[repr(C)] pub struct IDXGISurface { pub vtable: *mut IDXGISurfaceVtbl }
#[repr(C)] pub struct IDXGISurface1 { pub vtable: *mut IDXGISurface1Vtbl }
#[repr(C)] pub struct IDXGISurface2 { pub vtable: *mut IDXGISurface2Vtbl }
#[repr(C)] pub struct IDXGISwapChain { pub vtable: *mut IDXGISwapChainVtbl }
#[repr(C)] pub struct IDXGISwapChain1 { pub vtable: *mut IDXGISwapChain1Vtbl }
#[repr(C)] pub struct IDXGISwapChain2 { pub vtable: *mut IDXGISwapChain2Vtbl }
#[repr(C)] pub struct IDXGISwapChainMedia { pub vtable: *mut IDXGISwapChainMediaVtbl }

c_vtable!(
pub IUnknownVtbl of IUnknown, pub trait IUnknownT { 
    fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
    fn AddRef() -> ULONG,
    fn Release() -> ULONG,
} with heirs [
    pub IDXGIObjectVtbl of IDXGIObject, pub trait IDXGIObjectT {
        fn SetPrivateData(name: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
        fn SetPrivateDataInterface(name: REFGUID, unknown: *const IUnknown) -> HRESULT,
        fn GetPrivateData(name: REFGUID, data_size: UINT, data: *mut c_void) -> HRESULT,
        fn GetParent(riid: REFIID, parent: *mut *mut c_void) -> HRESULT,
    } with heirs [
        pub IDXGIAdapterVtbl of IDXGIAdapter, pub trait IDXGIAdapterT {
            fn EnumOutputs(output_i: UINT, output:*mut *mut IDXGIOutput) -> HRESULT,
            fn GetDesc(desc: *mut *mut DXGI_ADAPTER_DESC) -> HRESULT,
            fn CheckInterfaceSupport(interface_name: REFGUID, umd_version: LARGE_INTEGER) -> HRESULT,
        } with heirs [
            pub IDXGIAdapter1Vtbl of IDXGIAdapter1, pub trait IDXGIAdapter1T {
                fn GetDesc1(desc: *mut DXGI_ADAPTER_DESC1) -> HRESULT,
            }
            pub IDXGIAdapter2Vtbl of IDXGIAdapter2, pub trait IDXGIAdapter2T {
                fn GetDesc2(desc: *mut DXGI_ADAPTER_DESC2) -> HRESULT,
            }
        ]
        pub IDXGIDeviceVtbl of IDXGIDevice, pub trait IDXGIDeviceT {
            fn GetAdapter(adapter: *mut *mut IDXGIAdapter) -> HRESULT,
            fn CreateSurface(desc: *const DXGI_SURFACE_DESC, num_surfaces: UINT, usage: DXGI_USAGE, shared_resource: *const DXGI_SHARED_RESOURCE, surface: *mut *mut IDXGISurface) -> HRESULT,
            fn QueryResourceResidency(resources: *const IUnknown, residency_status: *mut DXGI_RESIDENCY, num_resources: UINT) -> HRESULT,
            fn SetGPUThreadPriority(priority: INT) -> HRESULT,
            fn GetGPUThreadPriority(priority: *mut INT) -> HRESULT,
        } with heirs [
            pub IDXGIDevice1Vtbl of IDXGIDevice1, pub trait IDXGIDevice1T {
                fn GetMaximumFrameLatency(max_latency: *mut UINT) -> HRESULT,
                fn SetMaximumFrameLatency(max_latency: UINT) -> HRESULT,
            } with heirs [
                pub IDXGIDevice2Vtbl of IDXGIDevice2, pub trait IDXGIDevice2T {
                    fn OfferResources(num_resources: UINT, resources: *const *mut IDXGIResource, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> HRESULT,
                    fn ReclaimResources(num_resources: UINT, resources: *const *mut IDXGIResource, discarded: *mut BOOL) -> HRESULT,
                    fn EnqueueSetEvent(event: HANDLE) -> HRESULT,
                } with heirs [
                    pub IDXGIDevice3Vtbl of IDXGIDevice3, pub trait IDXGIDevice3T {
                        fn Trim() -> (),
                    }
                ]
            ]
        ]
        pub IDXGIDeviceSubObjectVtbl of IDXGIDeviceSubObject, pub trait IDXGIDeviceSubObjectT {
            fn GetDevice(riid: REFIID, device: *mut *mut c_void) -> HRESULT,
        } with heirs [
            pub IDXGIKeyedMutexVtbl of IDXGIKeyedMutex, pub trait IDXGIKeyedMutexT {
                fn AcquireSync(key: UINT64, milliseconds: DWORD) -> HRESULT,
                fn ReleaseSync(key: UINT64) -> HRESULT,
            }
            pub IDXGIResourceVtbl of IDXGIResource, pub trait IDXGIResourceT {
                fn GetSharedHandle(shared_handle: *mut HANDLE) -> HRESULT,
                fn GetUsage(usage: *mut DXGI_USAGE) -> HRESULT,
                fn SetEvictionPriority(eviction_priority: UINT) -> HRESULT,
                fn GetEvictionPriority(evition_priority: *mut UINT) -> HRESULT,
            } with heirs [
                pub IDXGIResource1Vtbl of IDXGIResource1, pub trait IDXGIResource1T {
                    fn CreateSubresourceSurface(index: UINT, surface: *mut *mut IDXGISurface2) -> HRESULT,
                    fn CreateSharedHandle(attributes: *const SECURITY_ATTRIBUTES, access: DWORD, name: LPCWSTR, handle: *mut HANDLE) -> HRESULT,
                }
            ]
            pub IDXGISurfaceVtbl of IDXGISurface, pub trait IDXGISurfaceT {
                fn GetDesc(desc: *mut DXGI_SURFACE_DESC) -> HRESULT,
                fn Map(locked_rect: *mut DXGI_MAPPED_RECT, map_flags: UINT) -> HRESULT,
                fn Unmap() -> HRESULT,
            } with heirs [
                pub IDXGISurface1Vtbl of IDXGISurface1, pub trait IDXGISurface1T {
                    fn GetDC(discars: BOOL, dirtyrect: *mut RECT) -> HRESULT,
                    fn ReleaseDC(dirtyrect: *mut RECT) -> HRESULT,
                } with heirs [
                    pub IDXGISurface2Vtbl of IDXGISurface2, pub trait IDXGISurface2T {
                        fn GetResource(riid: REFIID, parent_resource: *mut *mut c_void, subresource_index: *mut UINT) -> HRESULT,
                    }
                ]
            ]
            pub IDXGISwapChainVtbl of IDXGISwapChain, pub trait IDXGISwapChainT {
                fn Present(synt_interval: UINT, flags: UINT) -> HRESULT,
                fn GetBuffer(buffer: UINT, riid: REFIID, surface: *mut *mut c_void) -> HRESULT,
                fn SetFullscreenState(fullscreen: BOOL, target: *mut IDXGIOutput) -> HRESULT,
                fn GetFullscreenState(fullscreen: *mut BOOL, target: *mut *mut IDXGIOutput) -> HRESULT,
                fn GetDesc(desc: *mut DXGI_SWAP_CHAIN_DESC) ->HRESULT,
                fn ResizeBuffers(bufcount: UINT, width: UINT, height: UINT, new_format: DXGI_FORMAT, swapchain_flags: UINT) -> HRESULT,
                fn ResizeTarget(new_target_params: *const DXGI_MODE_DESC) -> HRESULT,
                fn GetContainingOutput(output: *mut *mut IDXGIOutput) -> HRESULT,
                fn GetFrameStatistics(stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT,
                fn GetLastPresentCount(last_present_count: *mut UINT) -> HRESULT,
            } with heirs [
                pub IDXGISwapChain1Vtbl of IDXGISwapChain1, pub trait IDXGISwapChain1T {
                    fn GetDesc1(desc: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
                    fn GetFullscreenDesc(desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> HRESULT,
                    fn GetHwnd(hwnd: *mut HWND) -> HRESULT,
                    fn GetCoreWindow(refiid: REFIID, unk: *mut *mut c_void) -> HRESULT,
                    fn Present1(sync_interval: UINT, present_flags: UINT, present_parameters: *const DXGI_PRESENT_PARAMETERS) -> HRESULT,
                    fn IsTemporaryMonoSupported() -> BOOL,
                    fn GetRestrictToOutput(restrict_to_output: *mut *mut IDXGIOutput) -> HRESULT,
                    fn SetBackgroundColor(color: *const DXGI_RGBA) -> HRESULT,
                    fn GetBackgroundColor(color: *mut DXGI_RGBA) -> HRESULT,
                    fn SetRotation(rotation: DXGI_MODE_ROTATION) -> HRESULT,
                    fn GetRotation(rotation: *mut DXGI_MODE_ROTATION) -> HRESULT,
                } with heirs [
                    pub IDXGISwapChain2Vtbl of IDXGISwapChain2, pub trait IDXGISwapChain2T {
                        fn SetSourceSize(width: UINT, height: UINT) -> HRESULT,
                        fn GetSourceSize(width: *mut UINT, height: *mut UINT) -> HRESULT,
                        fn SetMaximumFrameLatency(max_latency: UINT) -> HRESULT,
                        fn GetMaximumFrameLatency(max_latency: *mut UINT) -> HRESULT,
                        fn GetFrameLatencyWaitableObject() -> HANDLE,
                        fn SetMatrixTransform(matrix: *const DXGI_MATRIX_3X2_F) -> HRESULT,
                        fn GetMatrixTransform(matrix: *mut DXGI_MATRIX_3X2_F) -> HRESULT,
                    }
                ]
            ]
        ]
        pub IDXGIFactoryVtbl of IDXGIFactory, pub trait IDXGIFactoryT {
            fn EnumAdapters(adapter_i: UINT, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
            fn MakeWindowAssociation(window_handle: HWND, flags: UINT) -> HRESULT,
            fn GetWindowAssociation(window_handle: *mut HWND) -> HRESULT,
            fn CreateSwapChain(device: *mut IUnknown, desc: *mut DXGI_SWAP_CHAIN_DESC, swapchain: *mut *mut IDXGISwapChain) -> HRESULT,
            fn CreateSoftwareAdapter(module: HMODULE, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
        } with heirs [
            pub IDXGIFactory1Vtbl of IDXGIFactory1, pub trait IDXGIFactory1T {
                fn EnumAdapters1(adapter_i: UINT, adapter: *mut *mut IDXGIAdapter1) -> HRESULT,
                fn IsCurrent() -> BOOL,
            } with heirs [
                pub IDXGIFactory2Vtbl of IDXGIFactory2, pub trait IDXGIFactory2T {
                    fn IsWindowedStereoEnabled() -> BOOL,
                    fn CreateSwapChainForHwnd(device: *mut IUnknown, hwnd: HWND, desc: *const DXGI_SWAP_CHAIN_DESC1, fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
                    fn CreateSwapChainForCoreWindow(device: *mut IUnknown, window: *mut IUnknown, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
                    fn GetSharedResourceAdapterLuid(resource: HANDLE, luid: *mut LUID) -> HRESULT,
                    fn RegisterStereoStatusWindow(window_handle: HWND, msg: UINT, cookie: *mut DWORD) -> HRESULT,
                    fn RegisterStereoStatusEvent(event_handle: HANDLE, cookie: *mut DWORD) -> HRESULT,
                    fn UnregisterStereoStatus(cookit: DWORD) -> (),
                    fn RegisterOcclusionStatusWindow(window_handle: HWND, msg: UINT, cookie: *mut DWORD) -> HRESULT,
                    fn RegisterOcclusionStatusEvent(event_handle: HANDLE, cookie: *mut DWORD) -> HRESULT,
                    fn UnregisterOcclusionStatus(cookie: DWORD) -> (),
                    fn CreateSwapChainForComposition(device: *mut IUnknown, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
                } with heirs [
                    pub IDXGIFactory3Vtbl of IDXGIFactory3, pub trait IDXGIFactory3T {
                        fn GetCreationFlags() -> UINT,
                    }
                ]
            ]
        ]
        pub IDXGIFactoryMediaVtbl of IDXGIFactoryMedia, pub trait IDXGIFactoryMediaT {
            fn CreateSwapChainForCompositionSurfaceHandle(device: *mut IUnknown, surface_handle: HANDLE, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
            fn CreateDecodeSwapChainForCompositionSurfaceHandle(device: *mut IUnknown, surface_handle: HANDLE, desc: *mut DXGI_DECODE_SWAP_CHAIN_DESC, yuv_decode_bufs: *mut IDXGIResource, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGIDecodeSwapChain) -> HRESULT,
        }
        pub IDXGIOutputVtbl of IDXGIOutput, pub trait IDXGIOutputT {
            fn GetDesc(desc: *mut DXGI_OUTPUT_DESC) -> HRESULT,
            fn GetDisplayModeList(enum_format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT, desc: *mut DXGI_MODE_DESC) -> HRESULT,
            fn FingClosestMatchingMode(mode_to_match: *const DXGI_MODE_DESC, closest_match: *mut DXGI_MODE_DESC, concerned_device: IUnknown) -> HRESULT,
            fn WaitForVBlank() -> HRESULT,
            fn TakeOwnerShip(device: *mut IUnknown, exclusive: BOOL) -> HRESULT,
            fn ReleaseOwnership() -> (),
            fn GetGammaControlCapabilities(gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HRESULT,
            fn SetGammaControl(array: *const DXGI_GAMMA_CONTROL) -> HRESULT,
            fn GetGammaControl(arrau: *mut DXGI_GAMMA_CONTROL) -> HRESULT,
            fn SetDisplaySurface(scanout_surface: *mut IDXGISurface) -> HRESULT,
            fn GetDispleySurfaceData(destination: *mut IDXGISurface) -> HRESULT,
            fn GetFrameStatistics(stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT,
        } with heirs [
            pub IDXGIOutput1Vtbl of IDXGIOutput1, pub trait IDXGIOutput1T {
                fn GetDisplayModeList1(enum_format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT, desc: *mut DXGI_MODE_DESC1) -> HRESULT,
                fn FindClosestMatchingMode1(mode_to_match: *const DXGI_MODE_DESC1, closest_match: *mut DXGI_MODE_DESC1, concerned_device: *mut IUnknown) -> HRESULT,
                fn GetDisplaySurfaceData1(destination: *mut IDXGIResource) -> HRESULT,
                fn DuplicateOutput(device: *mut IUnknown, output_duplication: *mut *mut IDXGIOutputDuplication) -> HRESULT,
            } with heirs [
                pub IDXGIOutput2Vtbl of IDXGIOutput2, pub trait IDXGIOutput2T {
                    fn SupportsOverlay() -> BOOL,
                }
            ]
        ]
        pub IDXGIOutputDuplicationVtbl of IDXGIOutputDuplication, pub trait IDXGIOutputDuplicationT {
            fn GetDesc(desc: *mut DXGI_OUTDUPL_DESC) -> (),
            fn AcquireNextFrame(timeout_ms: UINT, frame_info: *mut DXGI_OUTDUPL_FRAME_INFO, desktop_resource: *mut *mut IDXGIResource) -> HRESULT,
            fn GetFrameDirtyRects(dirty_rects_bufsize: UINT, dirtyrects_buf: *mut RECT, dirtyrects_bufsize_required: *mut UINT) -> HRESULT,
            fn GetFrameMoveRects(moverects_bufsize: UINT, moverect_buf: *mut DXGI_OUTDUPL_MOVE_RECT, moverects_bufsize_required: UINT) -> HRESULT,
            fn GetFramePointerShape(pointershape_bufsize: UINT, pointershape_buf: *mut c_void, pointershape_bufsize_required: *mut UINT, pointershape_info: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> HRESULT,
            fn MapDesktopSurface(locked_rect: *mut DXGI_MAPPED_RECT) -> HRESULT,
            fn UmMapDesktopSurface() -> HRESULT,
            fn ReleaseFrame() -> HRESULT,
        }
    ]
    pub IDXGIDebugVtbl of IDXGIDebug, pub trait IDXGIDebugT {
        fn ReportLiveObjects(apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT,
    } with heirs [
        pub IDXGIDebug1Vtbl of IDXGIDebug1, pub trait IDXGIDebug1T {
            fn EnableLeakTrackingForThread() -> (),
            fn DisableLeakTrackingForThread() -> (),
            fn IsLeakTrckingEnabledForThread() -> (),
        }
    ]
    pub IDXGIDecodeSwapChainVtbl of IDXGIDecodeSwapChain, pub trait IDXGIDecodeSwapChainT {
        fn GetColorSpace() -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
        fn GetDestSize(width: *mut UINT, height: *mut UINT) -> HRESULT,
        fn GetSourceRect(rect: *mut RECT) -> HRESULT,
        fn GetTargetRect(rect: *mut RECT) -> HRESULT,
        fn PresentBuffer(buffet_to_present: UINT, SyncInterval: UINT, flags: UINT) -> HRESULT,
        fn SetColorSpace(color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> HRESULT,
        fn SetDestSize(width: UINT, height: UINT) -> HRESULT,
        fn SetSourceRect(rect: *const RECT) -> HRESULT,
        fn SetTargetRect(rect: *const RECT) -> HRESULT,
    }
    pub IDXGIDisplayControlVtbl of IDXGIDisplayControl, pub trait IDXGIDisplayControlT {
        fn IsStereoEnabled() -> BOOL,
        fn SetStereoEnabled(enabled: BOOL) -> (),
    }
    pub IDXGIInfoQueueVtbl of IDXGIInfoQueue, pub trait IDXGIInfoQueueT {
        fn SetMessageCountLimit(producer: DXGI_DEBUG_ID, msg_count_limit: UINT64) -> HRESULT,
        fn ClearStoredMessages(producer: DXGI_DEBUG_ID) -> (),
        fn GetMessage(producer: DXGI_DEBUG_ID, message_i: UINT64, message: *mut DXGI_INFO_QUEUE_MESSAGE, msg_byte_len: *mut SIZE_T) -> HRESULT,
        fn GetNumStoredMessagesAllowedByRetrievalFilters(producer: DXGI_DEBUG_ID) -> UINT64,
        fn GetNumStoredMessages(producer: DXGI_DEBUG_ID) -> UINT64,
        fn GetNumMessagesDiscardedByMessageCountLimit(producer: DXGI_DEBUG_ID) -> UINT64,
        fn GetMessageCountLimit(producer: DXGI_DEBUG_ID) -> UINT64,
        fn GetNumMessagesAllowedByStorageFilter(producer: DXGI_DEBUG_ID) -> UINT64,
        fn GetNumMessagesDeniedByStorageFilter(producer: DXGI_DEBUG_ID) -> UINT64,
        fn AddStorageFilterEntries(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
        fn GetStorageFilter(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER, filter_byte_len: *mut SIZE_T) -> HRESULT,
        fn ClearStorageFilter(producer: DXGI_DEBUG_ID) -> (),
        fn PushEmptyStorageFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushDenyAllStorageFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushCopyOfStorageFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushStorageFilter(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
        fn PopStorageFilter(producer: DXGI_DEBUG_ID) -> (),
        fn GetStorageFilterStackSize(producer: DXGI_DEBUG_ID) -> UINT,
        fn AddRetrievalFilterEntries(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
        fn GetRetrievalFilter(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER, filter_byte_len: *mut SIZE_T) -> HRESULT,
        fn ClearRetrievalFilter(producer: DXGI_DEBUG_ID) -> (),
        fn PushEmptyRetrievalFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushDenyAllRetrievalFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushCopyOfRetrievalFilter(producer: DXGI_DEBUG_ID) -> HRESULT,
        fn PushRetrievalFilter(producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
        fn PopRetrievalFilter(producer: DXGI_DEBUG_ID) -> (),
        fn GetRetrievalFilterStackSize(producer: DXGI_DEBUG_ID) -> UINT,
        fn AddMessage(producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, desc: LPCSTR) -> HRESULT,
        fn AddApplicationMessage(severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, desc: LPCSTR) -> HRESULT,
        fn SetBreakOnCategory(producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, enable: BOOL) -> HRESULT,
        fn SetBreakOnSeverity(producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, enable: BOOL) -> HRESULT,
        fn SetBreakOnID(producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, enable: BOOL) -> HRESULT,
        fn GetBreakOnCategory(producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> BOOL,
        fn GetBreakOnSeverity(producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> BOOL,
        fn GetBreakOnID(producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> BOOL,
        fn SetMuteDebugOutput(producer: DXGI_DEBUG_ID, mute: BOOL) -> (),
        fn GetMuteDebugOutput(producer: DXGI_DEBUG_ID) -> BOOL,
    }
    pub IDXGISwapChainMediaVtbl of IDXGISwapChainMedia, pub trait IDXGISwapChainMediaT {
        fn GetFrameStatisticsMedia(stats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> HRESULT,
        fn SetPresentDuration(duration: UINT) -> HRESULT,
        fn CheckPresentDurationSupport(desired_present_duration: UINT, closest_smaller_present_duration: *mut UINT, closest_larger_present_duration: *mut UINT) -> HRESULT,
    }
]);