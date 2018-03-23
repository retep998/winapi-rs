// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// TODO:It is a minimal implementation.
use shared::basetsd::UINT32;
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::BYTE;
use shared::ntdef::{HRESULT, PCSTR, PCWSTR};
use um::d2d1::D2D1_EXTEND_MODE;
use um::d2d1_1::D2D1_BUFFER_PRECISION;
use um::unknwnbase::IUnknown;
FN!{stdcall PD2D1_PROPERTY_SET_FUNCTION(
    effect: *const IUnknown,
    data: *const BYTE,
    dataSize: UINT32,
) -> HRESULT}
FN!{stdcall PD2D1_PROPERTY_GET_FUNCTION(
    effect: *const IUnknown,
    data: *mut BYTE,
    dataSize: UINT32,
    actualSize: *mut UINT32,
) -> HRESULT}
ENUM!{enum D2D1_CHANGE_TYPE {
    D2D1_CHANGE_TYPE_NONE = 0,
    D2D1_CHANGE_TYPE_PROPERTIES = 1,
    D2D1_CHANGE_TYPE_CONTEXT = 2,
    D2D1_CHANGE_TYPE_GRAPH = 3,
}}
ENUM!{enum D2D1_PIXEL_OPTIONS {
    D2D1_PIXEL_OPTIONS_NONE = 0,
    D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING = 1,
}}
ENUM!{enum D2D1_VERTEX_OPTIONS {
    D2D1_VERTEX_OPTIONS_NONE = 0,
    D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR = 1,
    D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER = 2,
    D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP = 4,
}}
ENUM!{enum D2D1_VERTEX_USAGE {
    D2D1_VERTEX_USAGE_STATIC = 0,
    D2D1_VERTEX_USAGE_DYNAMIC = 1,
}}
ENUM!{enum D2D1_BLEND_OPERATION {
    D2D1_BLEND_OPERATION_ADD = 1,
    D2D1_BLEND_OPERATION_SUBTRACT = 2,
    D2D1_BLEND_OPERATION_REV_SUBTRACT = 3,
    D2D1_BLEND_OPERATION_MIN = 4,
    D2D1_BLEND_OPERATION_MAX = 5,
}}
ENUM!{enum D2D1_BLEND {
    D2D1_BLEND_ZERO = 1,
    D2D1_BLEND_ONE = 2,
    D2D1_BLEND_SRC_COLOR = 3,
    D2D1_BLEND_INV_SRC_COLOR = 4,
    D2D1_BLEND_SRC_ALPHA = 5,
    D2D1_BLEND_INV_SRC_ALPHA = 6,
    D2D1_BLEND_DEST_ALPHA = 7,
    D2D1_BLEND_INV_DEST_ALPHA = 8,
    D2D1_BLEND_DEST_COLOR = 9,
    D2D1_BLEND_INV_DEST_COLOR = 10,
    D2D1_BLEND_SRC_ALPHA_SAT = 11,
    D2D1_BLEND_BLEND_FACTOR = 14,
    D2D1_BLEND_INV_BLEND_FACTOR = 15,
}}
ENUM!{enum D2D1_CHANNEL_DEPTH {
    D2D1_CHANNEL_DEPTH_DEFAULT = 0,
    D2D1_CHANNEL_DEPTH_1 = 1,
    D2D1_CHANNEL_DEPTH_4 = 4,
}}
ENUM!{enum D2D1_FILTER {
    D2D1_FILTER_MIN_MAG_MIP_POINT = 0x00,
    D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR = 0x01,
    D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x04,
    D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR = 0x05,
    D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT = 0x10,
    D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11,
    D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT = 0x14,
    D2D1_FILTER_MIN_MAG_MIP_LINEAR = 0x15,
    D2D1_FILTER_ANISOTROPIC = 0x55,
}}
ENUM!{enum D2D1_FEATURE {
    D2D1_FEATURE_DOUBLES = 0,
    D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS = 1,
}}
STRUCT!{struct D2D1_PROPERTY_BINDING {
    propertyName: PCWSTR,
    setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}}
STRUCT!{struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    extents: *const UINT32,
    dimensions: UINT32,
    bufferPrecision: D2D1_BUFFER_PRECISION,
    channelDepth: D2D1_CHANNEL_DEPTH,
    filter: D2D1_FILTER,
    extendModes: *const D2D1_EXTEND_MODE,
}}
STRUCT!{struct D2D1_INPUT_ELEMENT_DESC {
    semanticName: PCSTR,
    semanticIndex: UINT32,
    format: DXGI_FORMAT,
    inputSlot: UINT32,
    alignedByteOffset: UINT32,
}}
const D2D1_APPEND_ALIGNED_ELEMENT: UINT32 = 0xffffffff;
struct!{struct D2D1_VERTEX_BUFFER_PROPERTIES {
    inputCount: UINT32,
    usage: D2D1_VERTEX_USAGE,
    data: *const BYTE,
    byteWidth: UINT32,
}}
struct!{struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    shaderBufferWithInputSignature: *const BYTE,
    shaderBufferSize: UINT32,
    inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    elementCount: UINT32,
    stride: UINT32,
}}
struct!{struct D2D1_VERTEX_RANGE {
    startVertex: UINT32,
    vertexCount: UINT32,
}}
struct!{struct D2D1_BLEND_DESCRIPTION {
    sourceBlend: D2D1_BLEND,
    destinationBlend: D2D1_BLEND,
    blendOperation: D2D1_BLEND_OPERATION,
    sourceBlendAlpha: D2D1_BLEND,
    destinationBlendAlpha: D2D1_BLEND,
    blendOperationAlpha: D2D1_BLEND_OPERATION,
    blendFactor: [FLOAT; 4],
}}
struct!{struct D2D1_INPUT_DESCRIPTION {
    filter: D2D1_FILTER,
    leveOfDetailCount: UINT32,
}}
struct!{struct D2D1_FEATURE_DATA_DOUBLES {
    doublePrecisionFloatShaderOps: BOOL,
}}
struct!{struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: BOOL,
}}
DEFINE_GUID!{IID_ID2D1VertexBuffer,
    }
DEFINE_GUID!{IID_ID2D1ResourceTexture,
    }
DEFINE_GUID!{IID_ID2D1RenderInfo,
    }
DEFINE_GUID!{IID_ID2D1DrawInfo,
    }
DEFINE_GUID!{IID_ID2D1ComputeInfo,
    }
DEFINE_GUID!{IID_ID2D1TransformNode,
    }
DEFINE_GUID!{IID_ID2D1TransformGraph,
    }
DEFINE_GUID!{IID_ID2D1Transform,
    }
DEFINE_GUID!{IID_ID2D1DrawTransform,
    }
DEFINE_GUID!{IID_ID2D1ComputeTransform,
    }
DEFINE_GUID!{IID_ID2D1AnalysisTransform,
    }
DEFINE_GUID!{IID_ID2D1SourceTransform,
    }
DEFINE_GUID!{IID_ID2D1ConcreteTransform,
    }
DEFINE_GUID!{IID_ID2D1BlendTransform,
    }
DEFINE_GUID!{IID_ID2D1BorderTransform,
    }
DEFINE_GUID!{IID_ID2D1OffsetTransform,
    }
DEFINE_GUID!{IID_ID2D1BoundsAdjustmentTransform,
    }
DEFINE_GUID!{IID_ID2D1EffectImpl,
    }
DEFINE_GUID!{IID_ID2D1EffectContext,
    }
