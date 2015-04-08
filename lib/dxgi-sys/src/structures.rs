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

//! Structures provided by DXGI.
//!
//! # References
//! [DXGI Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff471323(v=vs.85).aspx)

// TODO: verify that all struct fields are in the right order,
// as that is the only thing that matters when doing FFI

#![allow(non_snake_case, dead_code, non_camel_case_types)]

use winapi::{ WCHAR, UINT, SIZE_T,
    LUID, LARGE_INTEGER, BOOL,
    FLOAT, RECT, HMONITOR,
    POINT, HANDLE, INT,
    BYTE, HWND, c_char,
    c_float, c_int };

use constants::*;
use enumerations::*;

pub type DXGI_INFO_QUEUE_MESSAGE_ID = c_int;

#[repr(C)] pub struct DXGI_ADAPTER_DESC {
    pub Description: [WCHAR; 128],
    pub VendorId: UINT,
    pub DeviceId: UINT,
    pub SubSysId: UINT,
    pub Revision: UINT,
    pub DedicatedVideoMemory: SIZE_T,
    pub DedicatedSystemMemory: SIZE_T,
    pub SharedSystemMemory: SIZE_T,
    pub AdapterLuid: LUID,
}

#[repr(C)] pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [WCHAR; 128],
    pub VendorId: UINT,
    pub DeviceId: UINT,
    pub SubSysId: UINT,
    pub Revision: UINT,
    pub DedicatedVideoMemory: SIZE_T,
    pub DedicatedSystemMemory: SIZE_T,
    pub SharedSystemMemory: SIZE_T,
    pub AdapterLuid: LUID,
    pub Flags: UINT,
}

#[repr(C)] pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [WCHAR; 128],
    pub VendorId: UINT,
    pub DeviceId: UINT,
    pub SubSysId: UINT,
    pub Revision: UINT,
    pub DedicatedVideoMemory: SIZE_T,
    pub DedicatedSystemMemory: SIZE_T,
    pub SharedSystemMemory: SIZE_T,
    pub AdapterLuid: LUID,
    pub Flags: UINT,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}

#[repr(C)] pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: UINT,
}

#[repr(C)] pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: UINT,
    pub PresentRefreshCount: UINT,
    pub SyncRefreshCount: UINT,
    pub SyncQPCTime: LARGE_INTEGER,
    pub SyncGPUTime: LARGE_INTEGER,
}

#[repr(C)] pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: UINT,
    pub PresentRefreshCount: UINT,
    pub SyncRefreshCount: UINT,
    pub SyncQPCTime: LARGE_INTEGER,
    pub SyncGPUTime: LARGE_INTEGER,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: UINT,
}

#[repr(C)] pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}

#[repr(C)] pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: BOOL,
    pub MaxConvertedValue: c_float,
    pub MinConvertedValue: c_float,
    pub NumGammaControlPoints: UINT,
    pub ControlPointPositions: [c_float; 1025],
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: UINT,
    pub pCategoryList: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: UINT,
    pub pSeverityList: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: UINT,
    pub pIDList: DXGI_INFO_QUEUE_MESSAGE_ID,
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: DXGI_DEBUG_ID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    pub pDescription: *const c_char,
    pub DescriptionByteLength: SIZE_T,
}

#[repr(C)] pub struct DXGI_MATRIX_3X2_F {
    pub _11: FLOAT,
    pub _12: FLOAT,
    pub _21: FLOAT,
    pub _22: FLOAT,
    pub _31: FLOAT,
    pub _32: FLOAT,
}

#[repr(C)] pub struct DXGI_MAPPED_RECT {
    pub Pitch: INT,
    pub pBits: *mut BYTE,
}

#[repr(C)] pub struct DXGI_MODE_DESC {
    pub Width: UINT,
    pub Height: UINT,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}

#[repr(C)] pub struct DXGI_MODE_DESC1 {
    pub Width: UINT,
    pub Height: UINT,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Stereo: BOOL,
}

#[repr(C)] pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [WCHAR; 32],
    pub DesktopCoordinates: RECT,
    pub AttachedToDesktop: BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: HMONITOR,
}

#[repr(C)] pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: BOOL,
}

#[repr(C)] pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: LARGE_INTEGER,
    pub LastMouseUpdateTime: LARGE_INTEGER,
    pub AccumulatedFrames: UINT,
    pub RectsCoalesced: BOOL,
    pub ProtectedContentMaskedOut: BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: UINT,
    pub PointerShapeBufferSize: UINT,
}

#[repr(C)] pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: POINT,
    pub DestinationRect: RECT,
}

#[repr(C)] pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: POINT,
    pub Visible: BOOL,
}

#[repr(C)] pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: UINT,
    pub Width: UINT,
    pub Height: UINT,
    pub Pitch: UINT,
    pub HotSpot: POINT,
}

#[repr(C)] pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: UINT,
    pub pDirtyRects: RECT,
    pub pScrollRect: RECT,
    pub pScrollOffset: POINT,
}

#[repr(C)] pub struct DXGI_RATIONAL {
    pub Numerator: UINT,
    pub Denominator: UINT,
}

#[repr(C)] pub struct DXGI_RGB {
    pub Red: c_float,
    pub Green: c_float,
    pub Blue: c_float,
}

#[repr(C)] pub struct DXGI_RGBA {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)] pub struct DXGI_SAMPLE_DESC {
    pub Count: UINT,
    pub Quality: UINT,
}

#[repr(C)] pub struct DXGI_SHARED_RESOURCE {
    pub Handle: HANDLE,
}

#[repr(C)] pub struct DXGI_SURFACE_DESC {
    pub Width: UINT,
    pub Height: UINT,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}

#[repr(C)] pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: UINT,
    pub OutputWindow: HWND,
    pub Windowed: BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: UINT,
}

#[repr(C)] pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: UINT,
    pub Height: UINT,
    pub Format: DXGI_FORMAT,
    pub Stereo: BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: UINT,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: UINT,
}

#[repr(C)] pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: BOOL,
}