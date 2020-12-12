// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::dxgi::{DXGI_SWAP_CHAIN_DESC, IDXGIAdapter, IDXGISwapChain};
use shared::minwindef::{HMODULE, UINT};
use um::d3d10::ID3D10Device;
use um::winnt::HRESULT;
ENUM!{enum D3D10_DRIVER_TYPE {
    D3D10_DRIVER_TYPE_HARDWARE  = 0,
    D3D10_DRIVER_TYPE_REFERENCE = 1,
    D3D10_DRIVER_TYPE_NULL      = 2,
    D3D10_DRIVER_TYPE_SOFTWARE  = 3,
    D3D10_DRIVER_TYPE_WARP      = 5,
}}
extern "system" {
    pub fn D3D10CreateDevice(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D10_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        SDKVersion: UINT,
        ppDevice: *mut *mut ID3D10Device,
    ) -> HRESULT;
    pub fn D3D10CreateDeviceAndSwapChain(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D10_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        SDKVersion: UINT,
        pSwapChainDesc: *const DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain,
        ppDevice: *mut *mut ID3D10Device,
    ) -> HRESULT;
}
DEFINE_GUID!{GUID_DeviceType,
    0xd722fb4d, 0x7a68, 0x437a, 0xb2, 0x0c, 0x58, 0x04, 0xee, 0x24, 0x94, 0xa6}
