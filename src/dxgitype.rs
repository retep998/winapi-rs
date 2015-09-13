// Copyright © 2015; Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgitype.h

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
    pub MaxConvertedValue: ::c_float,
    pub MinConvertedValue: ::c_float,
    pub NumGammaControlPoints: ::UINT,
    pub ControlPointPositions: [::c_float; 1025],
}

impl Clone for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn clone(&self) -> Self { *self }
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_RATIONAL {
    pub Numerator: ::UINT,
    pub Denominator: ::UINT,
}

ENUM!{ enum DXGI_MODE_SCANLINE_ORDER {
    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED = 0x0,
    DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE = 0x1,
    DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST = 0x2,
    DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST = 0x3,
}}

ENUM!{ enum DXGI_MODE_SCALING {
    DXGI_MODE_SCALING_UNSPECIFIED = 0x0,
    DXGI_MODE_SCALING_CENTERED = 0x1,
    DXGI_MODE_SCALING_STRETCHED = 0x2,
}}

ENUM!{ enum DXGI_MODE_ROTATION {
    DXGI_MODE_ROTATION_UNSPECIFIED = 0x0,
    DXGI_MODE_ROTATION_IDENTITY = 0x1,
    DXGI_MODE_ROTATION_ROTATE90 = 0x2,
    DXGI_MODE_ROTATION_ROTATE180 = 0x3,
    DXGI_MODE_ROTATION_ROTATE270 = 0x4,
}}

#[repr(C)] #[derive(Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [::BYTE; 16],
    pub CodeValues: [::BYTE; 162],
}

impl Clone for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn clone(&self) -> Self { *self }
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [::BYTE; 12],
    pub CodeValues: [::BYTE; 12],
}

#[repr(C)] #[derive(Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [::BYTE; 64],
}

impl Clone for DXGI_JPEG_QUANTIZATION_TABLE {
    fn clone(&self) -> Self { *self }
}

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

ENUM!{ enum DXGI_COLOR_SPACE_TYPE {
    DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709 = 0x0,
    DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709 = 0x1,
    DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709 = 0x2,
    DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020 = 0x3,
    DXGI_COLOR_SPACE_RESERVED = 0x4,
    DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601 = 0x5,
    DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 = 0x6,
    DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601 = 0x7,
    DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 = 0x8,
    DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709 = 0x9,
    DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020 = 0xA,
    DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020 = 0xB,
    DXGI_COLOR_SPACE_CUSTOM = 0xFFFFFFFF,
}}

pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: ::UINT = 0xfffffffe;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: ::UINT = 0xffffffff;
