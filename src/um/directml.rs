// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_void};
use shared::basetsd::{UINT64};
use shared::guiddef::{REFGUID, REFIID};
use shared::minwindef::{BOOL, FLOAT, UINT};
use um::d3d12::{
    D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_GPU_DESCRIPTOR_HANDLE, ID3D12CommandList,
    ID3D12Device, ID3D12Resource,
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, PCWSTR};
pub const DML_TENSOR_DIMENSION_COUNT_MAX: UINT = 5;
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: UINT = 256;
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: UINT = 256;
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: UINT = 16;
ENUM!{enum DML_TENSOR_DATA_TYPE {
    DML_TENSOR_DATA_TYPE_UNKNOWN = 0,
    DML_TENSOR_DATA_TYPE_FLOAT32 = 1,
    DML_TENSOR_DATA_TYPE_FLOAT16 = 2,
    DML_TENSOR_DATA_TYPE_UINT32 = 3,
    DML_TENSOR_DATA_TYPE_UINT16 = 4,
    DML_TENSOR_DATA_TYPE_UINT8 = 5,
    DML_TENSOR_DATA_TYPE_INT32 = 6,
    DML_TENSOR_DATA_TYPE_INT16 = 7,
    DML_TENSOR_DATA_TYPE_INT8 = 8,
}}
ENUM!{enum DML_TENSOR_TYPE {
    DML_TENSOR_TYPE_INVALID = 0,
    DML_TENSOR_TYPE_BUFFER = 1,
}}
ENUM!{enum DML_TENSOR_FLAGS {
    DML_TENSOR_FLAG_NONE = 0,
    DML_TENSOR_FLAG_OWNED_BY_DML = 1,
}}
STRUCT!{struct DML_BUFFER_TENSOR_DESC {
    DataType: DML_TENSOR_DATA_TYPE,
    Flags: DML_TENSOR_FLAGS,
    DimensionCount: UINT,
    Sizes: *const UINT,
    Strides: *const UINT,
    TotalTensorSizeInBytes: UINT64,
    GuaranteedBaseOffsetAlignment: UINT,
}}
STRUCT!{struct DML_TENSOR_DESC {
    Type: DML_TENSOR_TYPE,
    Desc: *const c_void,
}}
ENUM!{enum DML_OPERATOR_TYPE {
    DML_OPERATOR_INVALID = 0,
    DML_OPERATOR_ELEMENT_WISE_IDENTITY = 1,
    DML_OPERATOR_ELEMENT_WISE_ABS = 2,
    DML_OPERATOR_ELEMENT_WISE_ACOS = 3,
    DML_OPERATOR_ELEMENT_WISE_ADD = 4,
    DML_OPERATOR_ELEMENT_WISE_ASIN = 5,
    DML_OPERATOR_ELEMENT_WISE_ATAN = 6,
    DML_OPERATOR_ELEMENT_WISE_CEIL = 7,
    DML_OPERATOR_ELEMENT_WISE_CLIP = 8,
    DML_OPERATOR_ELEMENT_WISE_COS = 9,
    DML_OPERATOR_ELEMENT_WISE_DIVIDE = 10,
    DML_OPERATOR_ELEMENT_WISE_EXP = 11,
    DML_OPERATOR_ELEMENT_WISE_FLOOR = 12,
    DML_OPERATOR_ELEMENT_WISE_LOG = 13,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_AND = 14,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_EQUALS = 15,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN = 16,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN = 17,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_NOT = 18,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_OR = 19,
    DML_OPERATOR_ELEMENT_WISE_LOGICAL_XOR = 20,
    DML_OPERATOR_ELEMENT_WISE_MAX = 21,
    DML_OPERATOR_ELEMENT_WISE_MEAN = 22,
    DML_OPERATOR_ELEMENT_WISE_MIN = 23,
    DML_OPERATOR_ELEMENT_WISE_MULTIPLY = 24,
    DML_OPERATOR_ELEMENT_WISE_POW = 25,
    DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW = 26,
    DML_OPERATOR_ELEMENT_WISE_RECIP = 27,
    DML_OPERATOR_ELEMENT_WISE_SIN = 28,
    DML_OPERATOR_ELEMENT_WISE_SQRT = 29,
    DML_OPERATOR_ELEMENT_WISE_SUBTRACT = 30,
    DML_OPERATOR_ELEMENT_WISE_TAN = 31,
    DML_OPERATOR_ELEMENT_WISE_THRESHOLD = 32,
    DML_OPERATOR_ELEMENT_WISE_QUANTIZE_LINEAR = 33,
    DML_OPERATOR_ELEMENT_WISE_DEQUANTIZE_LINEAR = 34,
    DML_OPERATOR_ACTIVATION_ELU = 35,
    DML_OPERATOR_ACTIVATION_HARDMAX = 36,
    DML_OPERATOR_ACTIVATION_HARD_SIGMOID = 37,
    DML_OPERATOR_ACTIVATION_IDENTITY = 38,
    DML_OPERATOR_ACTIVATION_LEAKY_RELU = 39,
    DML_OPERATOR_ACTIVATION_LINEAR = 40,
    DML_OPERATOR_ACTIVATION_LOG_SOFTMAX = 41,
    DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU = 42,
    DML_OPERATOR_ACTIVATION_PARAMETRIC_SOFTPLUS = 43,
    DML_OPERATOR_ACTIVATION_RELU = 44,
    DML_OPERATOR_ACTIVATION_SCALED_ELU = 45,
    DML_OPERATOR_ACTIVATION_SCALED_TANH = 46,
    DML_OPERATOR_ACTIVATION_SIGMOID = 47,
    DML_OPERATOR_ACTIVATION_SOFTMAX = 48,
    DML_OPERATOR_ACTIVATION_SOFTPLUS = 49,
    DML_OPERATOR_ACTIVATION_SOFTSIGN = 50,
    DML_OPERATOR_ACTIVATION_TANH = 51,
    DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU = 52,
    DML_OPERATOR_CONVOLUTION = 53,
    DML_OPERATOR_GEMM = 54,
    DML_OPERATOR_REDUCE = 55,
    DML_OPERATOR_AVERAGE_POOLING = 56,
    DML_OPERATOR_LP_POOLING = 57,
    DML_OPERATOR_MAX_POOLING = 58,
    DML_OPERATOR_ROI_POOLING = 59,
    DML_OPERATOR_SLICE = 60,
    DML_OPERATOR_CAST = 61,
    DML_OPERATOR_SPLIT = 62,
    DML_OPERATOR_JOIN = 63,
    DML_OPERATOR_PADDING = 64,
    DML_OPERATOR_VALUE_SCALE_2D = 65,
    DML_OPERATOR_UPSAMPLE_2D = 66,
    DML_OPERATOR_GATHER = 67,
    DML_OPERATOR_SPACE_TO_DEPTH = 68,
    DML_OPERATOR_DEPTH_TO_SPACE = 69,
    DML_OPERATOR_TILE = 70,
    DML_OPERATOR_TOP_K = 71,
    DML_OPERATOR_BATCH_NORMALIZATION = 72,
    DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION = 73,
    DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION = 74,
    DML_OPERATOR_LP_NORMALIZATION = 75,
    DML_OPERATOR_RNN = 76,
    DML_OPERATOR_LSTM = 77,
    DML_OPERATOR_GRU = 78,
}}
ENUM!{enum DML_REDUCE_FUNCTION {
    DML_REDUCE_FUNCTION_ARGMAX = 0,
    DML_REDUCE_FUNCTION_ARGMIN = 1,
    DML_REDUCE_FUNCTION_AVERAGE = 2,
    DML_REDUCE_FUNCTION_L1 = 3,
    DML_REDUCE_FUNCTION_L2 = 4,
    DML_REDUCE_FUNCTION_LOG_SUM = 5,
    DML_REDUCE_FUNCTION_LOG_SUM_EXP = 6,
    DML_REDUCE_FUNCTION_MAX = 7,
    DML_REDUCE_FUNCTION_MIN = 8,
    DML_REDUCE_FUNCTION_MULTIPLY = 9,
    DML_REDUCE_FUNCTION_SUM = 10,
    DML_REDUCE_FUNCTION_SUM_SQUARE = 11,
}}
ENUM!{enum DML_MATRIX_TRANSFORM {
    DML_MATRIX_TRANSFORM_NONE = 0,
    DML_MATRIX_TRANSFORM_TRANSPOSE = 1,
}}
ENUM!{enum DML_CONVOLUTION_MODE {
    DML_CONVOLUTION_MODE_CONVOLUTION = 0,
    DML_CONVOLUTION_MODE_CROSS_CORRELATION = 1,
}}
ENUM!{enum DML_CONVOLUTION_DIRECTION {
    DML_CONVOLUTION_DIRECTION_FORWARD = 0,
    DML_CONVOLUTION_DIRECTION_BACKWARD = 1,
}}
ENUM!{enum DML_PADDING_MODE {
    DML_PADDING_MODE_CONSTANT = 0,
    DML_PADDING_MODE_EDGE = 1,
    DML_PADDING_MODE_REFLECTION = 2,
}}
ENUM!{enum DML_INTERPOLATION_MODE {
    DML_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0,
    DML_INTERPOLATION_MODE_LINEAR = 1,
}}
STRUCT!{struct DML_SCALE_BIAS {
    Scale: FLOAT,
    Bias: FLOAT,
}}
STRUCT!{struct DML_SIZE_2D {
    Width: UINT,
    Height: UINT,
}}
ENUM!{enum DML_RECURRENT_NETWORK_DIRECTION {
    DML_RECURRENT_NETWORK_DIRECTION_FORWARD = 0,
    DML_RECURRENT_NETWORK_DIRECTION_BACKWARD = 1,
    DML_RECURRENT_NETWORK_DIRECTION_BIDIRECTIONAL = 2,
}}
STRUCT!{struct DML_OPERATOR_DESC {
    Type: DML_OPERATOR_TYPE,
    Desc: *const c_void,
}}
STRUCT!{struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
    Min: FLOAT,
    Max: FLOAT,
}}
STRUCT!{struct DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    ExponentTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
    Exponent: FLOAT,
}}
STRUCT!{struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
}}
STRUCT!{struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleBias: *const DML_SCALE_BIAS,
    Min: FLOAT,
}}
STRUCT!{struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    ScaleTensor: *const DML_TENSOR_DESC,
    ZeroPointTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    ScaleTensor: *const DML_TENSOR_DESC,
    ZeroPointTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_ELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
    Beta: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
    Beta: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    SlopeTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
    Beta: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_RELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
    Gamma: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
    Beta: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Steepness: FLOAT,
}}
STRUCT!{struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_TANH_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Alpha: FLOAT,
}}
STRUCT!{struct DML_CONVOLUTION_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    FilterTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Mode: DML_CONVOLUTION_MODE,
    Direction: DML_CONVOLUTION_DIRECTION,
    DimensionCount: UINT,
    Strides: *const UINT,
    Dilations: *const UINT,
    StartPadding: *const UINT,
    EndPadding: *const UINT,
    OutputPadding: *const UINT,
    GroupCount: UINT,
    FusedActivation: *const DML_OPERATOR_DESC,
}}
STRUCT!{struct DML_GEMM_OPERATOR_DESC {
    ATensor: *const DML_TENSOR_DESC,
    BTensor: *const DML_TENSOR_DESC,
    CTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    TransA: DML_MATRIX_TRANSFORM,
    TransB: DML_MATRIX_TRANSFORM,
    Alpha: FLOAT,
    Beta: FLOAT,
    FusedActivation: *const DML_OPERATOR_DESC,
}}
STRUCT!{struct DML_REDUCE_OPERATOR_DESC {
    Function: DML_REDUCE_FUNCTION,
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    AxisCount: UINT,
    Axes: *const UINT,
}}
STRUCT!{struct DML_AVERAGE_POOLING_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    DimensionCount: UINT,
    Strides: *const UINT,
    WindowSize: *const UINT,
    StartPadding: *const UINT,
    EndPadding: *const UINT,
    IncludePadding: BOOL,
}}
STRUCT!{struct DML_LP_POOLING_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    DimensionCount: UINT,
    Strides: *const UINT,
    WindowSize: *const UINT,
    StartPadding: *const UINT,
    EndPadding: *const UINT,
    P: UINT,
}}
STRUCT!{struct DML_MAX_POOLING_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    DimensionCount: UINT,
    Strides: *const UINT,
    WindowSize: *const UINT,
    StartPadding: *const UINT,
    EndPadding: *const UINT,
}}
STRUCT!{struct DML_ROI_POOLING_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    ROITensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    SpatialScale: FLOAT,
    PooledSize: DML_SIZE_2D,
}}
STRUCT!{struct DML_SLICE_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    DimensionCount: UINT,
    Offsets: *const UINT,
    Sizes: *const UINT,
    Strides: *const UINT,
}}
STRUCT!{struct DML_CAST_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
}}
STRUCT!{struct DML_SPLIT_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    DML_SPLIT_OPERATOR_DESC: UINT,
    OutputTensors: *const DML_TENSOR_DESC,
    Axis: UINT,
}}
STRUCT!{struct DML_JOIN_OPERATOR_DESC {
    InputCount: UINT,
    InputTensors: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Axis: UINT,
}}
STRUCT!{struct DML_PADDING_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    PaddingMode: DML_PADDING_MODE,
    PaddingValue: FLOAT,
    DimensionCount: UINT,
    StartPadding: *const UINT,
    EndPadding: *const UINT,
}}
STRUCT!{struct DML_VALUE_SCALE_2D_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Scale: FLOAT,
    ChannelCount: UINT,
    Bias: *const FLOAT,
}}
STRUCT!{struct DML_UPSAMPLE_2D_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    ScaleSize: DML_SIZE_2D,
    InterpolationMode: DML_INTERPOLATION_MODE,
}}
STRUCT!{struct DML_GATHER_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    IndicesTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Axis: UINT,
    IndexDimensions: UINT,
}}
STRUCT!{struct DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    BlockSize: UINT,
}}
STRUCT!{struct DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    BlockSize: UINT,
}}
STRUCT!{struct DML_TILE_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    RepeatsCount: UINT,
    Repeats: *const UINT,
}}
STRUCT!{struct DML_TOP_K_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputValueTensor: *const DML_TENSOR_DESC,
    OutputIndexTensor: *const DML_TENSOR_DESC,
    Axis: UINT,
    K: UINT,
}}
STRUCT!{struct DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    MeanTensor: *const DML_TENSOR_DESC,
    VarianceTensor: *const DML_TENSOR_DESC,
    ScaleTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Spatial: BOOL,
    Epsilon: FLOAT,
    FusedActivation: *const DML_OPERATOR_DESC,
}}
STRUCT!{struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    ScaleTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    CrossChannel: BOOL,
    NormalizeVariance: BOOL,
    Epsilon: FLOAT,
    FusedActivation: *const DML_OPERATOR_DESC,
}}
STRUCT!{struct DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    CrossChannel: BOOL,
    LocalSize: UINT,
    Alpha: FLOAT,
    Beta: FLOAT,
    Bias: FLOAT,
}}
STRUCT!{struct DML_LP_NORMALIZATION_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    OutputTensor: *const DML_TENSOR_DESC,
    Axis: UINT,
    Epsilon: FLOAT,
    P: UINT,
}}
STRUCT!{struct DML_RNN_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    WeightTensor: *const DML_TENSOR_DESC,
    RecurrenceTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    HiddenInitTensor: *const DML_TENSOR_DESC,
    SequenceLengthsTensor: *const DML_TENSOR_DESC,
    OutputSequenceTensor: *const DML_TENSOR_DESC,
    OutputSingleTensor: *const DML_TENSOR_DESC,
    ActivationDescCount: UINT,
    ActivationDescs: *const DML_OPERATOR_DESC,
    Direction: DML_RECURRENT_NETWORK_DIRECTION,
}}
STRUCT!{struct DML_LSTM_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    WeightTensor: *const DML_TENSOR_DESC,
    RecurrenceTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    HiddenInitTensor: *const DML_TENSOR_DESC,
    CellMemInitTensor: *const DML_TENSOR_DESC,
    SequenceLengthsTensor: *const DML_TENSOR_DESC,
    PeepholeTensor: *const DML_TENSOR_DESC,
    OutputSequenceTensor: *const DML_TENSOR_DESC,
    OutputSingleTensor: *const DML_TENSOR_DESC,
    OutputCellSingleTensor: *const DML_TENSOR_DESC,
    ActivationDescCount: UINT,
    ActivationDescs: *const DML_OPERATOR_DESC,
    Direction: DML_RECURRENT_NETWORK_DIRECTION,
    ClipThreshold: FLOAT,
    UseClipThreshold: BOOL,
    CoupleInputForget: BOOL,
}}
STRUCT!{struct DML_GRU_OPERATOR_DESC {
    InputTensor: *const DML_TENSOR_DESC,
    WeightTensor: *const DML_TENSOR_DESC,
    RecurrenceTensor: *const DML_TENSOR_DESC,
    BiasTensor: *const DML_TENSOR_DESC,
    HiddenInitTensor: *const DML_TENSOR_DESC,
    SequenceLengthsTensor: *const DML_TENSOR_DESC,
    OutputSequenceTensor: *const DML_TENSOR_DESC,
    OutputSingleTensor: *const DML_TENSOR_DESC,
    ActivationDescCount: UINT,
    ActivationDescs: *const DML_OPERATOR_DESC,
    Direction: DML_RECURRENT_NETWORK_DIRECTION,
    LinearBeforeReset: BOOL,
}}
ENUM!{enum DML_FEATURE {
    DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT = 0,
}}
STRUCT!{struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    DataType: DML_TENSOR_DATA_TYPE,
}}
STRUCT!{struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    IsSupported: BOOL,
}}
STRUCT!{struct DML_BINDING_TABLE_DESC {
    Dispatchable: *mut IDMLDispatchable,
    CPUDescriptorHandle: D3D12_CPU_DESCRIPTOR_HANDLE,
    GPUDescriptorHandle: D3D12_GPU_DESCRIPTOR_HANDLE,
    SizeInDescriptors: UINT,
}}
ENUM!{enum DML_EXECUTION_FLAGS {
    DML_EXECUTION_FLAG_NONE = 0,
    DML_EXECUTION_FLAG_ALLOW_HALF_PRECISION_COMPUTATION = 0x1,
    DML_EXECUTION_FLAG_DISABLE_META_COMMANDS = 0x2,
    DML_EXECUTION_FLAG_DESCRIPTORS_VOLATILE = 0x4,
}}
ENUM!{enum DML_CREATE_DEVICE_FLAGS {
    DML_CREATE_DEVICE_FLAG_NONE = 0,
    DML_CREATE_DEVICE_FLAG_DEBUG = 0x1,
}}
extern "system" {
    pub fn DMLCreateDevice(
        d3d12Device: *mut ID3D12Device,
        flags: DML_CREATE_DEVICE_FLAGS,
        riid: REFIID,
        device: *mut *mut c_void,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xc8263aac, 0x9e0c, 0x4c22, 0x9b, 0x8e, 0x00, 0x75, 0x21, 0xa3, 0x31, 0x7c)]
interface IDMLObject(IDMLObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetPrivateData(
        guid: REFGUID,
        dataSize: *mut UINT,
        data: *mut c_void,
    ) -> HRESULT,
    fn SetPrivateData(
        guid: REFGUID,
        dataSize: UINT,
        data: *const c_void,
    ) -> HRESULT,
    fn SetPrivateDataInterface(
        guid: REFGUID,
        data: *mut IUnknown,
    ) -> HRESULT,
    fn SetName(
        name: PCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6dbd6437, 0x96fd, 0x423f, 0xa9, 0x8c, 0xae, 0x5e, 0x7c, 0x2a, 0x57, 0x3f)]
interface IDMLDevice(IDMLDeviceVtbl): IDMLObject(IDMLObjectVtbl) {
    fn CheckFeatureSupport(
        feature: DML_FEATURE,
        featureQueryDataSize: UINT,
        featureQueryData: *const c_void,
        featureSupportDataSize: UINT,
        featureSupportData: *mut c_void,
    ) -> HRESULT,
    fn CreateOperator(
        desc: *const DML_OPERATOR_DESC,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CompileOperator(
        op: *mut IDMLOperator,
        flags: DML_EXECUTION_FLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateOperatorInitializer(
        operatorCount: UINT,
        operators: *const *mut IDMLCompiledOperator,
        flags: DML_EXECUTION_FLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateCommandRecorder(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn CreateBindingTable(
        desc: *const DML_BINDING_TABLE_DESC,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn Evict(
        count: UINT,
        ppObjects: *const *mut IDMLPageable,
    ) -> HRESULT,
    fn MakeResident(
        count: UINT,
        ppObjects: *const *mut IDMLPageable,
    ) -> HRESULT,
    fn GetDeviceRemovedReason() -> HRESULT,
    fn GetParentDevice(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x27e83142, 0x8165, 0x49e3, 0x97, 0x4e, 0x2f, 0xd6, 0x6e, 0x4c, 0xb6, 0x9d)]
interface IDMLDeviceChild(IDMLDeviceChildVtbl): IDMLObject(IDMLObjectVtbl) {
    fn GetDevice(
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb1ab0825, 0x4542, 0x4a4b, 0x86, 0x17, 0x6d, 0xde, 0x6e, 0x8f, 0x62, 0x01)]
interface IDMLPageable(IDMLPageableVtbl): IDMLDeviceChild(IDMLDeviceChildVtbl) {
}}
RIDL!{#[uuid(0x26caae7a, 0x3081, 0x4633, 0x95, 0x81, 0x22, 0x6f, 0xbe, 0x57, 0x69, 0x5d)]
interface IDMLOperator(IDMLOperatorVtbl): IDMLDeviceChild(IDMLDeviceChildVtbl) {
}}
STRUCT!{struct DML_BINDING_PROPERTIES {
    RequiredDescriptorCount: UINT,
    TemporaryResourceSize: UINT64,
    PersistentResourceSize: UINT64,
}}
RIDL!{#[uuid(0xdcb821a8, 0x1039, 0x441e, 0x9f, 0x1c, 0xb1, 0x75, 0x9c, 0x2f, 0x3c, 0xec)]
interface IDMLDispatchable(IDMLDispatchableVtbl): IDMLPageable(IDMLPageableVtbl) {
    #[fixme] fn GetBindingProperties() -> DML_BINDING_PROPERTIES,
}}
RIDL!{#[uuid(0x6b15e56a, 0xbf5c, 0x4902, 0x92, 0xd8, 0xda, 0x3a, 0x65, 0x0a, 0xfe, 0xa4)]
interface IDMLCompiledOperator(IDMLCompiledOperatorVtbl): IDMLDispatchable(IDMLDispatchableVtbl) {
}}
RIDL!{#[uuid(0x427c1113, 0x435c, 0x469c, 0x86, 0x76, 0x4d, 0x5d, 0xd0, 0x72, 0xf8, 0x13)]
interface IDMLOperatorInitializer(IDMLOperatorInitializerVtbl):
    IDMLDispatchable(IDMLDispatchableVtbl)
{
    fn Reset(
        operatorCount: UINT,
        operators: *const *mut IDMLCompiledOperator,
    ) -> HRESULT,
}}
ENUM!{enum DML_BINDING_TYPE {
    DML_BINDING_TYPE_NONE = 0,
    DML_BINDING_TYPE_BUFFER = 1,
    DML_BINDING_TYPE_BUFFER_ARRAY = 2,
}}
STRUCT!{struct DML_BINDING_DESC {
    Type: DML_BINDING_TYPE,
    Desc: *const c_void,
}}
STRUCT!{struct DML_BUFFER_BINDING {
    Buffer: *mut ID3D12Resource,
    Offset: UINT64,
    SizeInBytes: UINT64,
}}
STRUCT!{struct DML_BUFFER_ARRAY_BINDING {
    BindingCount: UINT,
    Bindings: *const DML_BUFFER_BINDING,
}}
RIDL!{#[uuid(0x29c687dc, 0xde74, 0x4e3b, 0xab, 0x00, 0x11, 0x68, 0xf2, 0xfc, 0x3c, 0xfc)]
interface IDMLBindingTable(IDMLBindingTableVtbl): IDMLDeviceChild(IDMLDeviceChildVtbl) {
    fn BindInputs(
        bindingCount: UINT,
        bindings: *const DML_BINDING_DESC,
    ) -> (),
    fn BindOutputs(
        bindingCount: UINT,
        bindings: *const DML_BINDING_DESC,
    ) -> (),
    fn BindTemporaryResource(
        binding: *const DML_BINDING_DESC,
    ) -> (),
    fn BindPersistentResource(
        binding: *const DML_BINDING_DESC,
    ) -> (),
    fn Reset(
        desc: *const DML_BINDING_TABLE_DESC,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xe6857a76, 0x2e3e, 0x4fdd, 0xbf, 0xf4, 0x5d, 0x2b, 0xa1, 0x0f, 0xb4, 0x53)]
interface IDMLCommandRecorder(IDMLCommandRecorderVtbl): IDMLDeviceChild(IDMLDeviceChildVtbl) {
    fn RecordDispatch(
        commandList: *mut ID3D12CommandList,
        dispatchable: *mut IDMLDispatchable,
        bindings: *mut IDMLBindingTable,
    ) -> (),
}}
RIDL!{#[uuid(0x7d6f3ac9, 0x394a, 0x4ac3, 0x92, 0xa7, 0x39, 0x0c, 0xc5, 0x7a, 0x82, 0x17)]
interface IDMLDebugDevice(IDMLDebugDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn SetMuteDebugOutput(
        mute: BOOL,
    ) -> (),
}}
