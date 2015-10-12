// Copyright © 2015; Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgitype.h
pub const DXGI_CPU_ACCESS_NONE: ::DWORD = 0;
pub const DXGI_CPU_ACCESS_DYNAMIC: ::DWORD = 1;
pub const DXGI_CPU_ACCESS_READ_WRITE: ::DWORD = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: ::DWORD = 3;
pub const DXGI_CPU_ACCESS_FIELD: ::DWORD = 15;
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_USAGE {
    DXGI_USAGE_SHADER_INPUT = 1 << (0 + 4),
    DXGI_USAGE_RENDER_TARGET_OUTPUT = 1 << (1 + 4),
    DXGI_USAGE_BACK_BUFFER = 1 << (2 + 4),
    DXGI_USAGE_SHARED = 1 << (3 + 4),
    DXGI_USAGE_READ_ONLY = 1 << (4 + 4),
    DXGI_USAGE_DISCARD_ON_PRESENT = 1 << (5 + 4),
    DXGI_USAGE_UNORDERED_ACCESS = 1 << (6 + 4),
}
pub use self::DXGI_USAGE::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = ::D3DCOLORVALUE;
#[repr(C)] #[derive(Copy)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl Clone for DXGI_GAMMA_CONTROL {
    fn clone(&self) -> DXGI_GAMMA_CONTROL {
        *self
    }
}
#[repr(C)] #[derive(Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: ::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: ::UINT,
    pub ControlPointPositions: [f32; 1025],
}
impl Clone for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn clone(&self) -> DXGI_GAMMA_CONTROL_CAPABILITIES {
        *self
    }
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_RATIONAL {
    pub Numerator: ::UINT,
    pub Denominator: ::UINT,
}
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_MODE_SCANLINE_ORDER {
    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
    DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
    DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
    DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST,
}
pub use self::DXGI_MODE_SCANLINE_ORDER::*;
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_MODE_SCALING {
    DXGI_MODE_SCALING_UNSPECIFIED,
    DXGI_MODE_SCALING_CENTERED,
    DXGI_MODE_SCALING_STRETCHED,
}
pub use self::DXGI_MODE_SCALING::*;
#[repr(i32)] #[derive(Copy, Clone, Debug)] #[allow(unused_qualifications)]
pub enum DXGI_MODE_ROTATION {
    DXGI_MODE_ROTATION_UNSPECIFIED,
    DXGI_MODE_ROTATION_IDENTITY,
    DXGI_MODE_ROTATION_ROTATE90,
    DXGI_MODE_ROTATION_ROTATE180,
    DXGI_MODE_ROTATION_ROTATE270,
}
pub use self::DXGI_MODE_ROTATION::*;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_MODE_DESC {
    pub Width: ::UINT,
    pub Height: ::UINT,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: ::DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: ::UINT,
    pub Quality: ::UINT,
}
