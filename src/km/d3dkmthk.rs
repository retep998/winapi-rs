// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! longhorn display driver model kernel mode thunk interfaces
use shared::d3dukmdt::{D3DDDI_ALLOCATIONLIST, D3DDDI_CREATECONTEXTFLAGS, D3DDDI_PATCHLOCATIONLIST, D3DGPU_VIRTUAL_ADDRESS, D3DKMT_HANDLE};
use shared::minwindef::UINT;
use shared::ntdef::VOID;
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
