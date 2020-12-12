// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, HMODULE, UINT, FLOAT, BOOL};
use um::d3d10::{ID3D10BlendState, ID3D10BlendStateVtbl, D3D10_BUFFER_SRV, D3D10_TEX1D_SRV, D3D10_TEX1D_ARRAY_SRV, D3D10_TEX2D_SRV, D3D10_TEX2D_ARRAY_SRV, D3D10_TEX2DMS_SRV, D3D10_TEX2DMS_ARRAY_SRV, D3D10_TEX3D_SRV, D3D10_TEXCUBE_SRV, ID3D10Device, ID3D10DeviceVtbl, ID3D10ShaderResourceView, ID3D10ShaderResourceViewVtbl, ID3D10Resource, D3D10_BLEND, D3D10_BLEND_OP};
use um::d3dcommon::D3D_SRV_DIMENSION;
use um::winnt::HRESULT;
use shared::dxgi::{IDXGIAdapter, DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use um::d3d10misc::D3D10_DRIVER_TYPE;
use shared::basetsd::UINT8;
use shared::dxgiformat::DXGI_FORMAT;

pub const D3D10_1_DEFAULT_SAMPLE_MASK: DWORD = 0xffffffff;
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: FLOAT = 0.6;
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: FLOAT = 0.6;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: DWORD = 32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: DWORD = 32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: DWORD = 128;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: DWORD = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: DWORD = 1;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: DWORD = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: DWORD = 1;
pub const D3D10_1_SHADER_MAJOR_VERSION: DWORD = 4;
pub const D3D10_1_SHADER_MINOR_VERSION: DWORD = 1;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: DWORD = 2048;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: DWORD = 256;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: DWORD = 4;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: DWORD = 1;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: DWORD = 64;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: DWORD = 32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: DWORD = 8;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: DWORD = 32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: DWORD = 32;
ENUM!{enum D3D10_FEATURE_LEVEL1 {
    D3D10_FEATURE_LEVEL_10_0 = 0xa000,
    D3D10_FEATURE_LEVEL_10_1 = 0xa100,
    D3D10_FEATURE_LEVEL_9_1	 = 0x9100,
    D3D10_FEATURE_LEVEL_9_2	 = 0x9200,
    D3D10_FEATURE_LEVEL_9_3	 = 0x9300,
}}
STRUCT!{struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    BlendEnable: BOOL,
    SrcBlend: D3D10_BLEND,
    DestBlend: D3D10_BLEND,
    BlendOp: D3D10_BLEND_OP,
    SrcBlendAlpha: D3D10_BLEND,
    DestBlendAlpha: D3D10_BLEND,
    BlendOpAlpha: D3D10_BLEND_OP,
    RenderTargetWriteMask: UINT8,
}}
STRUCT!{struct D3D10_BLEND_DESC1 {
    AlphaToCoverageEnable: BOOL,
    IndependentBlendEnable: BOOL,
    RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}}
RIDL!{#[uuid(0x9b7e4e00, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0)]
interface ID3D10BlendState1(ID3D10BlendState1Vtbl): ID3D10BlendState(ID3D10BlendStateVtbl) {
    fn GetDesc1(
        pDesc: *mut D3D10_BLEND_DESC1,
    ) -> (),
}}
STRUCT!{struct D3D10_TEXCUBE_ARRAY_SRV1 {
    MostDetailedMip: UINT,
    MipLevels: UINT,
    First2DArrayFace: UINT,
    NumCubes: UINT,
}}
pub type D3D10_SRV_DIMENSION1 = D3D_SRV_DIMENSION;
UNION!{union D3D10_SHADER_RESOURCE_VIEW_DESC1_u {
    [u32; 4],
    Buffer Buffer_mut: D3D10_BUFFER_SRV,
    Texture1D Texture1D_mut: D3D10_TEX1D_SRV,
    Texture1DArray Texture1DArray_mut: D3D10_TEX1D_ARRAY_SRV,
    Texture2D Texture2D_mut: D3D10_TEX2D_SRV,
    Texture2DArray Texture2DArray_mut: D3D10_TEX2D_ARRAY_SRV,
    Texture2DMS Texture2DMS_mut: D3D10_TEX2DMS_SRV,
    Texture2DMSArray Texture2DMSArray_mut: D3D10_TEX2DMS_ARRAY_SRV,
    Texture3D Texture3D_mut: D3D10_TEX3D_SRV,
    TextureCube TextureCube_mut: D3D10_TEXCUBE_SRV,
    TextureCubeArray TextureCubeArray_mut: D3D10_TEXCUBE_ARRAY_SRV1,
}}
STRUCT!{struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    Format: DXGI_FORMAT,
    ViewDimension: D3D10_SRV_DIMENSION1,
    u: D3D10_SHADER_RESOURCE_VIEW_DESC1_u ,
}}
RIDL!{#[uuid(0x9b7e4c87, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0)]
interface ID3D10ShaderResourceView1(ID3D10ShaderResourceView1Vtbl): ID3D10ShaderResourceView(ID3D10ShaderResourceViewVtbl) {
    fn GetDesc1(
        pDesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1,
    ) -> (),
}}
ENUM!{enum D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    D3D10_STANDARD_MULTISAMPLE_PATTERN	= 0xffffffff,
    D3D10_CENTER_MULTISAMPLE_PATTERN	= 0xfffffffe,
}}
RIDL!{#[uuid(0x9b7e4c8f, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0)]
interface ID3D10Device1(ID3D10Device1Vtbl): ID3D10Device(ID3D10DeviceVtbl) {
    fn CreateShaderResourceView1(
        pResource: *mut ID3D10Resource,
        pDesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1,
        ppSRView: *mut *mut ID3D10ShaderResourceView1,
    ) -> HRESULT,
    fn CreateBlendState1(
        pBlendStateDesc: *const D3D10_BLEND_DESC1,
        ppBlendState: *mut *mut ID3D10BlendState1,
    ) -> HRESULT,
    fn GetFeatureLevel() -> D3D10_FEATURE_LEVEL1,
}}
pub const D3D10_1_SDK_VERSION: DWORD = 0x20;
extern "system" {
    pub fn D3D10CreateDevice1(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D10_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        HardwareLevel: D3D10_FEATURE_LEVEL1,
        SDKVersion: UINT,
        ppDevice: *mut *mut ID3D10Device,
    ) -> HRESULT;
    pub fn D3D10CreateDeviceAndSwapChain1(
        pAdapter: *mut IDXGIAdapter,
        DriverType: D3D10_DRIVER_TYPE,
        Software: HMODULE,
        Flags: UINT,
        HardwareLevel: D3D10_FEATURE_LEVEL1,
        SDKVersion: UINT,
        pSwapChainDesc: *const DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain,
        ppDevice: *mut *mut ID3D10Device,
    ) -> HRESULT;
}
DEFINE_GUID!{IID_ID3D10BlendState1,
    0xedad8d99, 0x8a35, 0x4d6d, 0x85, 0x66, 0x2e, 0xa2, 0x76, 0xcd, 0xe1, 0x61}
DEFINE_GUID!{IID_ID3D10ShaderResourceView1,
    0x9b7e4c87, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
DEFINE_GUID!{IID_ID3D10Device1,
    0x9b7e4c8f, 0x342c, 0x4106, 0xa1, 0x9f, 0x4f, 0x27, 0x04, 0xf6, 0x89, 0xf0}
