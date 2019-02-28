// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! longhorn display driver model kernel mode thunk interfaces
use shared::basetsd::UINT64;
use shared::d3dukmdt::{
    D3DDDICB_SIGNALFLAGS, D3DDDI_ALLOCATIONLIST, D3DDDI_CREATECONTEXTFLAGS,
    D3DDDI_MAX_BROADCAST_CONTEXT, D3DDDI_MAX_OBJECT_SIGNALED, D3DDDI_MAX_OBJECT_WAITED_ON,
    D3DDDI_PATCHLOCATIONLIST, D3DDDI_SYNCHRONIZATIONOBJECTINFO,
    D3DDDI_SYNCHRONIZATIONOBJECTINFO2, D3DGPU_VIRTUAL_ADDRESS, D3DKMT_HANDLE,
};
use shared::minwindef::{UINT, ULONG};
use shared::ntdef::{HANDLE, LUID, VOID};
STRUCT!{struct D3DKMT_CREATEDEVICEFLAGS {
    bitfield: UINT,
}}
BITFIELD!{D3DKMT_CREATEDEVICEFLAGS bitfield: UINT [
    LegacyMode set_LegacyMode[0..1],
    RequestVSync set_RequestVSync[1..2],
    DisableGpuTimeout set_DisableGpuTimeout[2..3],
    Reserved set_Reserved[3..32],
]}
UNION!{union D3DKMT_CREATEDEVICE_u {
    [usize; 1],
    hAdapter hAdapter_mut: D3DKMT_HANDLE,
    pAdapter pAdapter_mut: *mut VOID,
}}
STRUCT!{struct D3DKMT_CREATEDEVICE {
    u: D3DKMT_CREATEDEVICE_u,
    Flags: D3DKMT_CREATEDEVICEFLAGS,
    hDevice: D3DKMT_HANDLE,
    pCommandBuffer: *mut VOID,
    CommandBufferSize: UINT,
    pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    AllocationListSize: UINT,
    pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    PatchLocationListSize: UINT,
}}
STRUCT!{struct D3DKMT_DESTROYDEVICE {
    hDevice: D3DKMT_HANDLE,
}}
ENUM!{enum D3DKMT_CLIENTHINT {
    D3DKMT_CLIENTHINT_UNKNOWN = 0,
    D3DKMT_CLIENTHINT_OPENGL = 1,
    D3DKMT_CLIENTHINT_CDD = 2,
    D3DKMT_CLIENTHINT_DX7 = 7,
    D3DKMT_CLIENTHINT_DX8 = 8,
    D3DKMT_CLIENTHINT_DX9 = 9,
    D3DKMT_CLIENTHINT_DX10 = 10,
}}
STRUCT!{struct D3DKMT_CREATECONTEXT {
    hDevice: D3DKMT_HANDLE,
    NodeOrdinal: UINT,
    EngineAffinity: UINT,
    Flags: D3DDDI_CREATECONTEXTFLAGS,
    pPrivateDriverData: *mut VOID,
    PrivateDriverDataSize: UINT,
    ClientHint: D3DKMT_CLIENTHINT,
    hContext: D3DKMT_HANDLE,
    pCommandBuffer: *mut VOID,
    CommandBufferSize: UINT,
    pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    AllocationListSize: UINT,
    pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    PatchLocationListSize: UINT,
    CommandBuffer: D3DGPU_VIRTUAL_ADDRESS,
}}
STRUCT!{struct D3DKMT_DESTROYCONTEXT {
    hContext: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DKMT_CREATESYNCHRONIZATIONOBJECT {
    hDevice: D3DKMT_HANDLE,
    Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO,
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DKMT_CREATESYNCHRONIZATIONOBJECT2 {
    hDevice: D3DKMT_HANDLE,
    Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO2,
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DKMT_DESTROYSYNCHRONIZATIONOBJECT {
    hSyncObject: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DKMT_OPENSYNCHRONIZATIONOBJECT {
    hSharedHandle: D3DKMT_HANDLE,
    hSyncObject: D3DKMT_HANDLE,
    Reserved: [UINT64; 8],
}}
STRUCT!{struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_WAITED_ON],
}}
STRUCT!{struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_Fence {
    FenceValue: UINT64,
}}
UNION!{union D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_u {
    [u64; 8],
    Fence Fence_mut: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_Fence,
    Reserved Reserved_mut: [UINT64; 8],
}}
STRUCT!{struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_WAITED_ON],
    u: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_u,
}}
STRUCT!{struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_SIGNALED],
    Flags: D3DDDICB_SIGNALFLAGS,
}}
STRUCT!{struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_Fence {
    FenceValue: UINT64,
}}
UNION!{union D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_u {
    [u64; 8],
    Fence Fence_mut: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_Fence,
    CpuEventHandle CpuEventHandle_mut: HANDLE,
    Reserved Reserved_mut: [UINT64; 8],
}}
STRUCT!{struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 {
    hContext: D3DKMT_HANDLE,
    ObjectCount: UINT,
    ObjectHandleArray: [D3DKMT_HANDLE; D3DDDI_MAX_OBJECT_SIGNALED],
    Flags: D3DDDICB_SIGNALFLAGS,
    BroadcastContextCount: ULONG,
    BroadcastContext: [D3DKMT_HANDLE; D3DDDI_MAX_BROADCAST_CONTEXT],
    u: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_u,
}}
// 2038
STRUCT!{struct D3DKMT_OPENADAPTERFROMLUID {
    AdapterLuid: LUID,
    hAdapter: D3DKMT_HANDLE,
}}
