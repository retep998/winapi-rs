// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dxgitype.h
use ctypes::c_float;
use shared::d3d9types::D3DCOLORVALUE;
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::{BOOL, BYTE, DWORD, UINT, WORD};
use um::winnt::HRESULT;
pub use shared::dxgi::{
    DXGI_USAGE,
    DXGI_USAGE_SHADER_INPUT,
    DXGI_USAGE_RENDER_TARGET_OUTPUT,
    DXGI_USAGE_BACK_BUFFER,
    DXGI_USAGE_SHARED,
    DXGI_USAGE_READ_ONLY,
    DXGI_USAGE_DISCARD_ON_PRESENT,
    DXGI_USAGE_UNORDERED_ACCESS,
};
pub use shared::dxgicommon::*;
pub const _FACDXGI: DWORD = 0x87a;
#[inline]
pub fn MAKE_DXGI_HRESULT(code: WORD) -> HRESULT {
    MAKE_HRESULT!(1, _FACDXGI, code as DWORD) as HRESULT
}
#[inline]
pub fn MAKE_DXGI_STATUS(code: WORD) -> HRESULT {
    MAKE_HRESULT!(0, _FACDXGI, code as DWORD) as HRESULT
}
pub const DXGI_CPU_ACCESS_NONE: DWORD = 0;
pub const DXGI_CPU_ACCESS_DYNAMIC: DWORD = 1;
pub const DXGI_CPU_ACCESS_READ_WRITE: DWORD = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: DWORD = 3;
pub const DXGI_CPU_ACCESS_FIELD: DWORD = 15;
STRUCT!{struct DXGI_RGB {
    Red: c_float,
    Green: c_float,
    Blue: c_float,
}}
pub type DXGI_RGBA = D3DCOLORVALUE;
STRUCT!{struct DXGI_GAMMA_CONTROL {
    Scale: DXGI_RGB,
    Offset: DXGI_RGB,
    GammaCurve: [DXGI_RGB; 1025],
}}
STRUCT!{struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    ScaleAndOffsetSupported: BOOL,
    MaxConvertedValue: c_float,
    MinConvertedValue: c_float,
    NumGammaControlPoints: UINT,
    ControlPointPositions: [c_float; 1025],
}}
ENUM!{enum DXGI_MODE_SCANLINE_ORDER {
    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED = 0,
    DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE = 1,
    DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST = 2,
    DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST = 3,
}}
ENUM!{enum DXGI_MODE_SCALING {
    DXGI_MODE_SCALING_UNSPECIFIED = 0,
    DXGI_MODE_SCALING_CENTERED = 1,
    DXGI_MODE_SCALING_STRETCHED = 2,
}}
ENUM!{enum DXGI_MODE_ROTATION {
    DXGI_MODE_ROTATION_UNSPECIFIED = 0,
    DXGI_MODE_ROTATION_IDENTITY = 1,
    DXGI_MODE_ROTATION_ROTATE90 = 2,
    DXGI_MODE_ROTATION_ROTATE180 = 3,
    DXGI_MODE_ROTATION_ROTATE270 = 4,
}}
STRUCT!{struct DXGI_MODE_DESC {
    Width: UINT,
    Height: UINT,
    RefreshRate: DXGI_RATIONAL,
    Format: DXGI_FORMAT,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
}}
STRUCT!{struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    CodeCounts: [BYTE; 12],
    CodeValues: [BYTE; 12],
}}
STRUCT!{struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    CodeCounts: [BYTE; 16],
    CodeValues: [BYTE; 162],
}}
STRUCT!{struct DXGI_JPEG_QUANTIZATION_TABLE {
    Elements: [BYTE; 64],
}}
