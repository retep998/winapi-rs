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

//! Enumerations provided by DXGI.
//!
//! # References
//! [DXGI Enumerations, MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ff471320(v=vs.85).aspx)

#![allow(non_snake_case, dead_code)]

#[repr(C)] pub enum DXGI_ADAPTER_FLAG {
    NONE = 0,
    REMOTE = 1,
    SOFTWARE = 2,
    FORCE_DWORD = 0xffffffff
}

#[repr(C)] pub enum DXGI_ALPHA_MODE {
    UNSPECIFIED = 0,
    PREMULTIPLIED = 1,
    STRAIGHT = 2,
    IGNORE = 3,
    FORCE_DWORD = 0xffffffff
}

#[repr(C)] pub enum DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    DMA_BUFFER_BOUNDARY = 0,
    DISPATCH_BOUNDARY = 1,
    THREAD_GROUP_BOUNDARY = 2,
    THREAD_BOUNDARY = 3,
    INSTRUCTION_BOUNDARY = 4
}

#[repr(C)] pub enum DXGI_DEBUG_RLO_FLAGS {
    SUMMARY = 0x1,
    DETAIL = 0x2,
    ALL = 0x3
}

#[repr(C)] pub enum DXGI_FORMAT {
    UNKNOWN = 0,
    R32G32B32A32_TYPELESS = 1,
    R32G32B32A32_FLOAT = 2,
    R32G32B32A32_UINT = 3,
    R32G32B32A32_SINT = 4,
    R32G32B32_TYPELESS = 5,
    R32G32B32_FLOAT = 6,
    R32G32B32_UINT = 7,
    R32G32B32_SINT = 8,
    R16G16B16A16_TYPELESS = 9,
    R16G16B16A16_FLOAT = 10,
    R16G16B16A16_UNORM = 11,
    R16G16B16A16_UINT = 12,
    R16G16B16A16_SNORM = 13,
    R16G16B16A16_SINT = 14,
    R32G32_TYPELESS = 15,
    R32G32_FLOAT = 16,
    R32G32_UINT = 17,
    R32G32_SINT = 18,
    R32G8X24_TYPELESS = 19,
    D32_FLOAT_S8X24_UINT = 20,
    R32_FLOAT_X8X24_TYPELESS = 21,
    X32_TYPELESS_G8X24_UINT = 22,
    R10G10B10A2_TYPELESS = 23,
    R10G10B10A2_UNORM = 24,
    R10G10B10A2_UINT = 25,
    R11G11B10_FLOAT = 26,
    R8G8B8A8_TYPELESS = 27,
    R8G8B8A8_UNORM = 28,
    R8G8B8A8_UNORM_SRGB = 29,
    R8G8B8A8_UINT = 30,
    R8G8B8A8_SNORM = 31,
    R8G8B8A8_SINT = 32,
    R16G16_TYPELESS = 33,
    R16G16_FLOAT = 34,
    R16G16_UNORM = 35,
    R16G16_UINT = 36,
    R16G16_SNORM = 37,
    R16G16_SINT = 38,
    R32_TYPELESS = 39,
    D32_FLOAT = 40,
    R32_FLOAT = 41,
    R32_UINT = 42,
    R32_SINT = 43,
    R24G8_TYPELESS = 44,
    D24_UNORM_S8_UINT = 45,
    R24_UNORM_X8_TYPELESS = 46,
    X24_TYPELESS_G8_UINT = 47,
    R8G8_TYPELESS = 48,
    R8G8_UNORM = 49,
    R8G8_UINT = 50,
    R8G8_SNORM = 51,
    R8G8_SINT = 52,
    R16_TYPELESS = 53,
    R16_FLOAT = 54,
    D16_UNORM = 55,
    R16_UNORM = 56,
    R16_UINT = 57,
    R16_SNORM = 58,
    R16_SINT = 59,
    R8_TYPELESS = 60,
    R8_UNORM = 61,
    R8_UINT = 62,
    R8_SNORM = 63,
    R8_SINT = 64,
    A8_UNORM = 65,
    R1_UNORM = 66,
    R9G9B9E5_SHAREDEXP = 67,
    R8G8_B8G8_UNORM = 68,
    G8R8_G8B8_UNORM = 69,
    BC1_TYPELESS = 70,
    BC1_UNORM = 71,
    BC1_UNORM_SRGB = 72,
    BC2_TYPELESS = 73,
    BC2_UNORM = 74,
    BC2_UNORM_SRGB = 75,
    BC3_TYPELESS = 76,
    BC3_UNORM = 77,
    BC3_UNORM_SRGB = 78,
    BC4_TYPELESS = 79,
    BC4_UNORM = 80,
    BC4_SNORM = 81,
    BC5_TYPELESS = 82,
    BC5_UNORM = 83,
    BC5_SNORM = 84,
    B5G6R5_UNORM = 85,
    B5G5R5A1_UNORM = 86,
    B8G8R8A8_UNORM = 87,
    B8G8R8X8_UNORM = 88,
    R10G10B10_XR_BIAS_A2_UNORM = 89,
    B8G8R8A8_TYPELESS = 90,
    B8G8R8A8_UNORM_SRGB = 91,
    B8G8R8X8_TYPELESS = 92,
    B8G8R8X8_UNORM_SRGB = 93,
    BC6H_TYPELESS = 94,
    BC6H_UF16 = 95,
    BC6H_SF16 = 96,
    BC7_TYPELESS = 97,
    BC7_UNORM = 98,
    BC7_UNORM_SRGB = 99,
    AYUV = 100,
    Y410 = 101,
    Y416 = 102,
    NV12 = 103,
    P010 = 104,
    P016 = 105,
    OPAQUE_420 = 106,
    YUY2 = 107,
    Y210 = 108,
    Y216 = 109,
    NV11 = 110,
    AI44 = 111,
    IA44 = 112,
    P8 = 113,
    A8P8 = 114,
    B4G4R4A4_UNORM = 115,
    FORCE_UINT = 0xffffffff
}

#[repr(C)] pub enum DXGI_FRAME_PRESENTATION_MODE {
    COMPOSED = 0,
    OVERLAY = 1,
    NONE = 2
}

#[repr(C)] pub enum DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    DMA_BUFFER_BOUNDARY = 0,
    PRIMITIVE_BOUNDARY = 1,
    TRIANGLE_BOUNDARY = 2,
    PIXEL_BOUNDARY = 3,
    INSTRUCTION_BOUNDARY = 4
}

#[repr(C)] pub enum DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    UNKNOWN = 0,
    MISCELLANEOUS = 1,
    INITIALIZATION = 2,
    CLEANUP = 3,
    COMPILATION = 4,
    STATE_CREATION = 5,
    STATE_SETTING = 6,
    STATE_GETTING = 7,
    RESOURCE_MANIPULATION = 8,
    EXECUTION = 9,
    SHADER = 10
}

#[repr(C)] pub enum DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    CORRUPTION = 0,
    ERROR = 2,
    WARNING = 3,
    INFO = 4,
    MESSAGE = 5
}

#[repr(C)] pub enum DXGI_MODE_ROTATION {
    UNSPECIFIED = 0,
    IDENTITY = 1,
    ROTATE90 = 2,
    ROTATE180 = 3,
    ROTATE270 = 4
}

#[repr(C)] pub enum DXGI_MODE_SCALING {
    UNSPECIFIED = 0,
    CENTERED = 1,
    STRETCHED = 2
}

#[repr(C)] pub enum DXGI_MODE_SCANLINE_ORDER {
    UNSPECIFIED = 0,
    PROGRESSIVE = 1,
    UPPER_FIELD_FIRST = 2,
    LOWER_FIELD_FIRST = 3
}

#[repr(C)] pub enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    NOMINAL_RANGE = 0x1,
    BT709 = 0x2,
    xvYCC = 0x4
}

#[repr(C)] pub enum DXGI_OFFER_RESOURCE_PRIORITY {
    LOW = 1,
    NORMAL = 2,
    HIGH = 3
}

#[repr(C)] pub enum DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    MONOCHROME = 0x1,
    COLOR = 0x2,
    MASKED_COLOR = 0x4
}

#[repr(C)] pub enum DXGI_RESIDENCY {
    FULLY_RESIDENT = 1,
    RESIDENT_IN_SHARED_MEMORY = 2,
    EVICTED_TO_DISK = 3
}

#[repr(C)] pub enum DXGI_SCALING {
    STRETCH = 0,
    NONE = 1,
    ASPECT_RATIO_STRETCH = 2
}

#[repr(C)] pub enum DXGI_SWAP_CHAIN_FLAG {
    NONPREROTATED = 1,
    ALLOW_MODE_SWITCH = 2,
    GDI_COMPATIBLE = 4,
    RESTRICTED_CONTENT = 8,
    RESTRICT_SHARED_RESOURCE_DRIVER = 16,
    DISPLAY_ONLY = 32,
    FRAME_LATENCY_WAITABLE_OBJECT = 64,
    FOREGROUND_LAYER = 128
}

#[repr(C)] pub enum DXGI_SWAP_EFFECT {
    DISCARD = 0,
    SEQUENTIAL = 1,
    FLIP_SEQUENTIAL = 3
}