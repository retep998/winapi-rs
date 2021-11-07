// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, UINT};
use um::d3d10::{
    D3D10_BLEND_DESC, D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT,
    D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT, D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT,
    D3D10_DEPTH_STENCIL_DESC, D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT, D3D10_RASTERIZER_DESC,
    D3D10_SAMPLER_DESC, ID3D10BlendState, ID3D10Buffer, ID3D10DepthStencilState,
    ID3D10DepthStencilView, ID3D10Device, ID3D10GeometryShader, ID3D10PixelShader,
    ID3D10RasterizerState, ID3D10RenderTargetView, ID3D10SamplerState, ID3D10ShaderResourceView,
    ID3D10VertexShader
};
use um::d3d10shader::{
    D3D10_SHADER_MACRO, D3D10_SHADER_VARIABLE_CLASS, D3D10_SHADER_VARIABLE_TYPE,
    D3D10_SIGNATURE_PARAMETER_DESC, ID3D10Include
};
use um::d3dcommon::ID3D10Blob;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, INT, LPCSTR};
ENUM!{enum D3D10_DEVICE_STATE_TYPES {
    D3D10_DST_SO_BUFFERS=1,
    D3D10_DST_OM_RENDER_TARGETS,
    D3D10_DST_OM_DEPTH_STENCIL_STATE,
    D3D10_DST_OM_BLEND_STATE,
    D3D10_DST_VS,
    D3D10_DST_VS_SAMPLERS,
    D3D10_DST_VS_SHADER_RESOURCES,
    D3D10_DST_VS_CONSTANT_BUFFERS,
    D3D10_DST_GS,
    D3D10_DST_GS_SAMPLERS,
    D3D10_DST_GS_SHADER_RESOURCES,
    D3D10_DST_GS_CONSTANT_BUFFERS,
    D3D10_DST_PS,
    D3D10_DST_PS_SAMPLERS,
    D3D10_DST_PS_SHADER_RESOURCES,
    D3D10_DST_PS_CONSTANT_BUFFERS,
    D3D10_DST_IA_VERTEX_BUFFERS,
    D3D10_DST_IA_INDEX_BUFFER,
    D3D10_DST_IA_INPUT_LAYOUT,
    D3D10_DST_IA_PRIMITIVE_TOPOLOGY,
    D3D10_DST_RS_VIEWPORTS,
    D3D10_DST_RS_SCISSOR_RECTS,
    D3D10_DST_RS_RASTERIZER_STATE,
    D3D10_DST_PREDICATION,
}}
STRUCT!{struct D3D10_STATE_BLOCK_MASK {
    VS: BYTE,
    VSSamplers: [BYTE; (D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT as usize+7)/8],
    VSShaderResources: [BYTE; (D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize+7)/8],
    VSConstantBuffers: [BYTE; (D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize+7)/8],
    GS: BYTE,
    GSSamplers: [BYTE; (D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT as usize+7)/8],
    GSShaderResources: [BYTE; (D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize+7)/8],
    GSConstantBuffers: [BYTE; (D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize+7)/8],
    PS: BYTE,
    PSSamplers: [BYTE; (D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT as usize+7)/8],
    PSShaderResources: [BYTE; (D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize+7)/8],
    PSConstantBuffers: [BYTE; (D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize+7)/8],
    IAVertexBuffers: [BYTE; (D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as usize+7)/8],
    IAIndexBuffer: BYTE,
    IAInputLayout: BYTE,
    IAPrimitiveTopology: BYTE,
    OMRenderTargets: BYTE,
    OMDepthStencilState: BYTE,
    OMBlendState: BYTE,
    RSViewports: BYTE,
    RSScissorRects: BYTE,
    RSRasterizerState: BYTE,
    SOBuffers: BYTE,
    Predication: BYTE,
}}
RIDL!{#[uuid(0x0803425a, 0x57f5, 0x4dd6, 0x94, 0x65, 0xa8, 0x75, 0x70, 0x83, 0x4a, 0x08)]
interface ID3D10StateBlock(ID3D10StateBlockVtbl): IUnknown(IUnknownVtbl) {
    fn Capture() -> (),
    fn Apply() -> (),
    fn ReleaseAllDeviceObjects() -> (),
    fn GetDevice(
        ppDevice: *mut *mut ID3D10Device,
    ) -> (),
}}
extern "system" {
    pub fn D3D10StateBlockMaskUnion(
        pA: *mut D3D10_STATE_BLOCK_MASK,
        pB: *mut D3D10_STATE_BLOCK_MASK,
        pResult: *mut D3D10_STATE_BLOCK_MASK,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskIntersect(
        pA: *mut D3D10_STATE_BLOCK_MASK,
        pB: *mut D3D10_STATE_BLOCK_MASK,
        pResult: *mut D3D10_STATE_BLOCK_MASK,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskDifference(
        pA: *mut D3D10_STATE_BLOCK_MASK,
        pB: *mut D3D10_STATE_BLOCK_MASK,
        pResult: *mut D3D10_STATE_BLOCK_MASK,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskEnableCapture(
        pMask: *mut D3D10_STATE_BLOCK_MASK,
        StateType: D3D10_DEVICE_STATE_TYPES,
        RangeStart: UINT,
        RangeLength: UINT,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskDisableCapture(
        pMask: *mut D3D10_STATE_BLOCK_MASK,
        StateType: D3D10_DEVICE_STATE_TYPES,
        RangeStart: UINT,
        RangeLength: UINT,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskEnableAll(
        pMask: *mut D3D10_STATE_BLOCK_MASK,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskDisableAll(
        pMask: *mut D3D10_STATE_BLOCK_MASK,
    ) -> HRESULT;
    pub fn D3D10StateBlockMaskGetSetting(
        pMask: *mut D3D10_STATE_BLOCK_MASK,
        StateType: D3D10_DEVICE_STATE_TYPES,
        Entry: UINT,
    ) -> HRESULT;
    pub fn D3D10CreateStateBlock(
        pDevice: *mut ID3D10Device,
        pStateBlockMask: *mut D3D10_STATE_BLOCK_MASK,
        ppStateBlock: *mut *mut ID3D10StateBlock,
    ) -> HRESULT;
}
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: DWORD = 1 << 0;
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: DWORD = 1 << 1;
pub const D3D10_EFFECT_SINGLE_THREADED: DWORD = 1 << 3;
pub const D3D10_EFFECT_VARIABLE_POOLED: DWORD = 1 << 0;
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: DWORD = 1 << 1;
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: DWORD = 1 << 2;
STRUCT!{struct D3D10_EFFECT_TYPE_DESC {
    TypeName: LPCSTR,
    Class: D3D10_SHADER_VARIABLE_CLASS,
    Type: D3D10_SHADER_VARIABLE_TYPE,
    Elements: UINT,
    Members: UINT,
    Rows: UINT,
    Columns: UINT,
    PackedSize: UINT,
    UnpackedSize: UINT,
    Stride: UINT,
}}
RIDL!{#[uuid(0x4e9e1ddc, 0xcd9d, 0x4772, 0xa8, 0x37, 0x00, 0x18, 0x0b, 0x9b, 0x88, 0xfd)]
interface ID3D10EffectType(ID3D10EffectTypeVtbl) {
    fn IsValid() -> BOOL,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_TYPE_DESC,
    ) -> (),
    fn GetMemberTypeByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectType,
    fn GetMemberTypeByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectType,
    fn GetMemberTypeBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectType,
    fn GetMemberName(
        Index: UINT,
    ) -> LPCSTR,
    fn GetMemberSemantic(
        Index: UINT,
    ) -> LPCSTR,
}}
STRUCT!{struct D3D10_EFFECT_VARIABLE_DESC {
    Name: LPCSTR,
    Semantic: LPCSTR,
    Flags: UINT,
    Annotations: UINT,
    BufferOffset: UINT,
    ExplicitBindPoint: UINT,
}}
RIDL!{#[uuid(0xae897105, 0x00e6, 0x45bf, 0xbb, 0x8e, 0x28, 0x1d, 0xd6, 0xdb, 0x8e, 0x1b)]
interface ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn GetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x00e48f7b, 0xd2c8, 0x49e8, 0xa8, 0x6c, 0x02, 0x2d, 0xee, 0x53, 0x43, 0x1f)]
interface ID3D10EffectScalarVariable(ID3D10EffectScalarVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn GetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn SetFloat(
        Value: FLOAT,
    ) -> (),
    fn GetFloat(
        pValue: *mut FLOAT,
    ) -> (),
    fn SetFloatArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetFloatArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn SetInt(
        Value: INT,
    ) -> (),
    fn GetInt(
        pValue: *mut INT,
    ) -> (),
    fn SetIntArray(
        pData: *mut INT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetIntArray(
        pData: *mut INT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn SetBool(
        Value: BOOL,
    ) -> (),
    fn GetBool(
        pValue: *mut BOOL,
    ) -> (),
    fn SetBoolArray(
        pData: *mut BOOL,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetBoolArray(
        pData: *mut BOOL,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x00e48f7b, 0xd2c8, 0x49e8, 0xa8, 0x6c, 0x02, 0x2d, 0xee, 0x53, 0x43, 0x1f)]
interface ID3D10EffectVectorVariable(ID3D10EffectVectorVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn GetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn SetBoolVector(
        pData: *mut BOOL,
    ) -> (),
    fn SetIntVector(
        pData: *mut INT,
    ) -> (),
    fn SetFloatVector(
        pData: *mut FLOAT,
    ) -> (),
    fn GetBoolVector(
        pData: *mut BOOL,
    ) -> (),
    fn GetIntVector(
        pData: *mut INT,
    ) -> (),
    fn GetFloatVector(
        pData: *mut FLOAT,
    ) -> (),
    fn SetBoolVectorArray(
        pData: *mut BOOL,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn SetIntVectorArray(
        pData: *mut INT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn SetFloatVectorArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetBoolVectorArray(
        pData: *mut BOOL,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetIntVectorArray(
        pData: *mut INT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetFloatVectorArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x50666c24, 0xb82f, 0x4eed, 0xa1, 0x72, 0x5b, 0x6e, 0x7e, 0x85, 0x22, 0xe0)]
interface ID3D10EffectMatrixVariable(ID3D10EffectMatrixVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn GetRawValue(
        pData: *mut c_void,
        Offset: UINT,
        ByteCount: UINT,
    ) -> (),
    fn SetMatrix(
        pData: *mut FLOAT,
    ) -> (),
    fn GetMatrix(
        pData: *mut FLOAT,
    ) -> (),
    fn SetMatrixArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetMatrixArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn SetMatrixTranspose(
        pData: *mut FLOAT,
    ) -> (),
    fn GetMatrixTranspose(
        pData: *mut FLOAT,
    ) -> (),
    fn SetMatrixTransposeArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetMatrixTransposeArray(
        pData: *mut FLOAT,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x71417501, 0x8df9, 0x4e0a, 0xa7, 0x8a, 0x25, 0x5f, 0x97, 0x56, 0xba, 0xff)]
interface ID3D10EffectStringVariable(ID3D10EffectStringVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetString(
        ppString: *mut *mut LPCSTR,
    ) -> (),
    fn GetStringArray(
        pData: *mut *mut LPCSTR,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0xc0a7157b, 0xd872, 0x4b1d, 0x80, 0x73, 0xef, 0xc2, 0xac, 0xd4, 0xb1, 0xfc)]
interface ID3D10EffectShaderResourceVariable(ID3D10EffectShaderResourceVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetResource(
        pResource: *mut ID3D10ShaderResourceView,
    ) -> (),
    fn GetResource(
        ppResource: *mut *mut ID3D10ShaderResourceView,
    ) -> (),
    fn SetResourceArray(
        pResources: *mut ID3D10ShaderResourceView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetResourceArray(
        ppResources: *mut *mut ID3D10ShaderResourceView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x28ca0cc3, 0xc2c9, 0x40bb, 0xb5, 0x7f, 0x67, 0xb7, 0x37, 0x12, 0x2b, 0x17)]
interface ID3D10EffectRenderTargetViewVariable(ID3D10EffectRenderTargetViewVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetRenderTarget(
        pResource: *mut ID3D10RenderTargetView,
    ) -> (),
    fn GetRenderTarget(
        ppResource: *mut *mut ID3D10RenderTargetView,
    ) -> (),
    fn SetRenderTargetArray(
        pResources: *mut ID3D10RenderTargetView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetRenderTargetArray(
        ppResources: *mut *mut ID3D10RenderTargetView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x3e02c918, 0xcc79, 0x4985, 0xb6, 0x22, 0x2d, 0x92, 0xad, 0x70, 0x16, 0x23)]
interface ID3D10EffectDepthStencilViewVariable(ID3D10EffectDepthStencilViewVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetDepthStencil(
        pResource: *mut ID3D10DepthStencilView,
    ) -> (),
    fn GetDepthStencil(
        ppResource: *mut *mut ID3D10DepthStencilView,
    ) -> (),
    fn SetDepthStencilArray(
        pResources: *mut ID3D10DepthStencilView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
    fn GetDepthStencilArray(
        ppResources: *mut *mut ID3D10DepthStencilView,
        Offset: UINT,
        Count: UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x56648f4d, 0xcc8b, 0x4444, 0xa5, 0xad, 0xb5, 0xa3, 0xd7, 0x6e, 0x91, 0xb3)]
interface ID3D10EffectConstantBuffer(ID3D10EffectConstantBufferVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn SetConstantBuffer(
        pConstantBuffer: *mut ID3D10Buffer,
    ) -> (),
    fn GetConstantBuffer(
        ppConstantBuffer: *mut *mut ID3D10Buffer,
    ) -> (),
    fn SetTextureBuffer(
        pTextureBuffer: *mut ID3D10ShaderResourceView,
    ) -> (),
    fn GetTextureBuffer(
        ppTextureBuffer: *mut *mut ID3D10ShaderResourceView,
    ) -> (),
}}
STRUCT!{struct D3D10_EFFECT_SHADER_DESC {
    pInputSignature: *const BYTE,
    IsInline: BOOL,
    pBytecode: *const BYTE,
    BytecodeLength: UINT,
    SODecl: LPCSTR,
    NumInputSignatureEntries: UINT,
    NumOutputSignatureEntries: UINT,
}}
RIDL!{#[uuid(0x80849279, 0xc799, 0x4797, 0x8c, 0x33, 0x04, 0x07, 0xa0, 0x7d, 0x9e, 0x06)]
interface ID3D10EffectShaderVariable(ID3D10EffectShaderVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetShaderDesc(
        ShaderIndex: UINT,
        pDesc: *mut D3D10_EFFECT_SHADER_DESC,
    ) -> (),
    fn GetVertexShader(
        ShaderIndex: UINT,
        ppVS: *mut *mut ID3D10VertexShader,
    ) -> (),
    fn GetGeometryShader(
        ShaderIndex: UINT,
        ppGS: *mut *mut ID3D10GeometryShader,
    ) -> (),
    fn GetPixelShader(
        ShaderIndex: UINT,
        ppPS: *mut *mut ID3D10PixelShader,
    ) -> (),
    fn GetInputSignatureElementDesc(
        ShaderIndex: UINT,
        Element: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> (),
    fn GetOutputSignatureElementDesc(
        ShaderIndex: UINT,
        Element: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> (),
}}
RIDL!{#[uuid(0x1fcd2294, 0xdf6d, 0x4eae, 0x86, 0xb3, 0x0e, 0x91, 0x60, 0xcf, 0xb0, 0x7b)]
interface ID3D10EffectBlendVariable(ID3D10EffectBlendVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetBlendState(
        Index: UINT,
        ppBlendState: *mut *mut ID3D10BlendState,
    ) -> (),
    fn GetBackingStore(
        Index: UINT,
        pBlendDesc: *mut D3D10_BLEND_DESC,
    ) -> (),
}}
RIDL!{#[uuid(0xaf482368, 0x330a, 0x46a5, 0x9a, 0x5c, 0x01, 0xc7, 0x1a, 0xf2, 0x4c, 0x8d)]
interface ID3D10EffectDepthStencilVariable(ID3D10EffectDepthStencilVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetDepthStencilState(
        Index: UINT,
        ppDepthStencilState: *mut *mut ID3D10DepthStencilState,
    ) -> (),
    fn GetBackingStore(
        Index: UINT,
        pDepthStencilDesc: *mut D3D10_DEPTH_STENCIL_DESC,
    ) -> (),
}}
RIDL!{#[uuid(0x21af9f0e, 0x4d94, 0x4ea9, 0x97, 0x85, 0x2c, 0xb7, 0x6b, 0x8c, 0x0b, 0x34)]
interface ID3D10EffectRasterizerVariable(ID3D10EffectRasterizerVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetRasterizerState(
        Index: UINT,
        ppRasterizerState: *mut *mut ID3D10RasterizerState,
    ) -> (),
    fn GetBackingStore(
        Index: UINT,
        pRasterizerDesc: *mut D3D10_RASTERIZER_DESC,
    ) -> (),
}}
RIDL!{#[uuid(0x6530d5c7, 0x07e9, 0x4271, 0xa4, 0x18, 0xe7, 0xce, 0x4b, 0xd1, 0xe4, 0x80)]
interface ID3D10EffectSamplerVariable(ID3D10EffectSamplerVariableVtbl):
    ID3D10EffectVariable(ID3D10EffectVariableVtbl) {
    fn IsValid() -> BOOL,
    fn GetType() -> *mut ID3D10EffectType,
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_VARIABLE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetMemberBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetElement(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetParentConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsScalar() -> *mut ID3D10EffectScalarVariable,
    fn AsVector() -> *mut ID3D10EffectVectorVariable,
    fn AsMatrix() -> *mut ID3D10EffectMatrixVariable,
    fn AsString() -> *mut ID3D10EffectStringVariable,
    fn AsShaderResource() -> *mut ID3D10EffectShaderResourceVariable,
    fn AsRenderTargetView() -> *mut ID3D10EffectRenderTargetViewVariable,
    fn AsDepthStencilView() -> *mut ID3D10EffectDepthStencilViewVariable,
    fn AsConstantBuffer() -> *mut ID3D10EffectConstantBuffer,
    fn AsShader() -> *mut ID3D10EffectShaderVariable,
    fn AsBlend() -> *mut ID3D10EffectBlendVariable,
    fn AsDepthStencil() -> *mut ID3D10EffectDepthStencilVariable,
    fn AsRasterizer() -> *mut ID3D10EffectRasterizerVariable,
    fn AsSampler() -> *mut ID3D10EffectSamplerVariable,
    fn GetSampler(
        Index: UINT,
        ppSampler: *mut *mut ID3D10SamplerState,
    ) -> (),
    fn GetBackingStore(
        Index: UINT,
        pSamplerDesc: *mut D3D10_SAMPLER_DESC,
    ) -> (),
}}
STRUCT!{struct D3D10_PASS_DESC {
    Name: LPCSTR,
    Annotations: UINT,
    pIAInputSignature: *mut BYTE,
    IAInputSignatureSize: SIZE_T,
    StencilRef: UINT,
    SampleMask: UINT,
    BlendFactor: [FLOAT; 4],
}}
STRUCT!{struct D3D10_PASS_SHADER_DESC {
    pShaderVariable: *mut ID3D10EffectShaderVariable,
    ShaderIndex: UINT,
}}
RIDL!{#[uuid(0x5cfbeb89, 0x1a06, 0x46e0, 0xb2, 0x82, 0xe3, 0xf9, 0xbf, 0xa3, 0x6a, 0x54)]
interface ID3D10EffectPass(ID3D10EffectPassVtbl) {
    fn IsValid() -> BOOL,
    fn GetDesc(
        pDesc: *mut D3D10_PASS_DESC,
    ) -> (),
    fn GetVertexShaderDesc(
        pDesc: *mut D3D10_PASS_SHADER_DESC,
    ) -> (),
    fn GetGeometryShaderDesc(
        pDesc: *mut D3D10_PASS_SHADER_DESC,
    ) -> (),
    fn GetPixelShaderDesc(
        pDesc: *mut D3D10_PASS_SHADER_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn Apply(
        Flags: UINT,
    ) -> (),
    fn ComputeStateBlockMask(
        pStateBlockMask: *mut D3D10_STATE_BLOCK_MASK,
    ) -> (),
}}
STRUCT!{struct D3D10_TECHNIQUE_DESC {
    Name: LPCSTR,
    Passes: UINT,
    Annotations: UINT,
}}
RIDL!{#[uuid(0xdb122ce8, 0xd1c9, 0x4292, 0xb2, 0x37, 0x24, 0xed, 0x3d, 0xe8, 0xb1, 0x75)]
interface ID3D10EffectTechnique(ID3D10EffectTechniqueVtbl) {
    fn IsValid() -> BOOL,
    fn GetDesc(
        pDesc: *mut D3D10_TECHNIQUE_DESC,
    ) -> (),
    fn GetAnnotationByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetAnnotationByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetPassByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectPass,
    fn GetPassByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectPass,
    fn ComputeStateBlockMask(
        pStateBlockMask: *mut D3D10_STATE_BLOCK_MASK,
    ) -> (),
}}
STRUCT!{struct D3D10_EFFECT_DESC {
    IsChildEffect: BOOL,
    ConstantBuffers: UINT,
    SharedConstantBuffers: UINT,
    GlobalVariables: UINT,
    SharedGlobalVariables: UINT,
    Techniques: UINT,
}}
RIDL!{#[uuid(0x51b0ca8b, 0xec0b, 0x4519, 0x87, 0x0d, 0x8e, 0xe1, 0xcb, 0x50, 0x17, 0xc7)]
interface ID3D10Effect(ID3D10EffectVtbl): IUnknown(IUnknownVtbl) {
    fn IsValid() -> BOOL,
    fn IsPool() -> BOOL,
    fn GetDevice(
        ppDevice: *mut *mut ID3D10Device,
    ) -> (),
    fn GetDesc(
        pDesc: *mut D3D10_EFFECT_DESC,
    ) -> (),
    fn GetConstantBufferByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectConstantBuffer,
    fn GetConstantBufferByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectConstantBuffer,
    fn GetVariableByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectVariable,
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetVariableBySemantic(
        Semantic: LPCSTR,
    ) -> *mut ID3D10EffectVariable,
    fn GetTechniqueByIndex(
        Index: UINT,
    ) -> *mut ID3D10EffectTechnique,
    fn GetTechniqueByName(
        Name: LPCSTR,
    ) -> *mut ID3D10EffectTechnique,
    fn Optimize() -> (),
    fn IsOptimized() -> BOOL,
}}
RIDL!{#[uuid(0x9537ab04, 0x3250, 0x412e, 0x82, 0x13, 0xfc, 0xd2, 0xf8, 0x67, 0x79, 0x33)]
interface ID3D10EffectPool(ID3D10EffectPoolVtbl): IUnknown(IUnknownVtbl) {
    fn AsEffect() -> *mut ID3D10Effect,
}}
extern "system" {
    pub fn D3D10CompileEffectFromMemory(
        pData: *mut c_void,
        DataLength: SIZE_T,
        pSrcFileName: LPCSTR,
        pDefines: *const D3D10_SHADER_MACRO,
        pInclude: *mut ID3D10Include,
        HLSLFlags: UINT,
        FXFlags: UINT,
        ppCompiledEffect: *mut *mut ID3D10Blob,
        ppErrors: *mut *mut ID3D10Blob,
    ) -> HRESULT;
    pub fn D3D10CreateEffectFromMemory(
        pData: *mut c_void,
        DataLength: SIZE_T,
        FXFlags: UINT,
        pDevice: *mut ID3D10Device,
        pEffectPool: *mut ID3D10EffectPool,
        ppEffect: *mut *mut ID3D10Effect,
    ) -> HRESULT;
    pub fn D3D10CreateEffectPoolFromMemory(
        pData: *mut c_void,
        DataLength: SIZE_T,
        FXFlags: UINT,
        pDevice: *mut ID3D10Device,
        pEffectPool: *mut *mut ID3D10EffectPool,
    ) -> HRESULT;
    pub fn D3D10DisassembleEffect(
        pEffect: *mut ID3D10Effect ,
        EnableColorCode: BOOL,
        ppDisassembly: *mut *mut ID3D10Blob   ,
    ) -> HRESULT;
}
DEFINE_GUID!{IID_ID3D10StateBlock,
    0x0803425a, 0x57f5, 0x4dd6, 0x94, 0x65, 0xa8, 0x75, 0x70, 0x83, 0x4a, 0x08}
DEFINE_GUID!{IID_ID3D10EffectType,
    0x4e9e1ddc, 0xcd9d, 0x4772, 0xa8, 0x37, 0x00, 0x18, 0x0b, 0x9b, 0x88, 0xfd}
DEFINE_GUID!{IID_ID3D10EffectVariable,
    0xae897105, 0x00e6, 0x45bf, 0xbb, 0x8e, 0x28, 0x1d, 0xd6, 0xdb, 0x8e, 0x1b}
DEFINE_GUID!{IID_ID3D10EffectScalarVariable,
    0x00e48f7b, 0xd2c8, 0x49e8, 0xa8, 0x6c, 0x02, 0x2d, 0xee, 0x53, 0x43, 0x1f}
DEFINE_GUID!{IID_ID3D10EffectVectorVariable,
    0x62b98c44, 0x1f82, 0x4c67, 0xbc, 0xd0, 0x72, 0xcf, 0x8f, 0x21, 0x7e, 0x81}
DEFINE_GUID!{IID_ID3D10EffectMatrixVariable,
    0x50666c24, 0xb82f, 0x4eed, 0xa1, 0x72, 0x5b, 0x6e, 0x7e, 0x85, 0x22, 0xe0}
DEFINE_GUID!{IID_ID3D10EffectStringVariable,
    0x71417501, 0x8df9, 0x4e0a, 0xa7, 0x8a, 0x25, 0x5f, 0x97, 0x56, 0xba, 0xff}
DEFINE_GUID!{IID_ID3D10EffectShaderResourceVariable,
    0xc0a7157b, 0xd872, 0x4b1d, 0x80, 0x73, 0xef, 0xc2, 0xac, 0xd4, 0xb1, 0xfc}
DEFINE_GUID!{IID_ID3D10EffectRenderTargetViewVariable,
    0x28ca0cc3, 0xc2c9, 0x40bb, 0xb5, 0x7f, 0x67, 0xb7, 0x37, 0x12, 0x2b, 0x17}
DEFINE_GUID!{IID_ID3D10EffectDepthStencilViewVariable,
    0x3e02c918, 0xcc79, 0x4985, 0xb6, 0x22, 0x2d, 0x92, 0xad, 0x70, 0x16, 0x23}
DEFINE_GUID!{IID_ID3D10EffectConstantBuffer,
    0x56648f4d, 0xcc8b, 0x4444, 0xa5, 0xad, 0xb5, 0xa3, 0xd7, 0x6e, 0x91, 0xb3}
DEFINE_GUID!{IID_ID3D10EffectShaderVariable,
    0x80849279, 0xc799, 0x4797, 0x8c, 0x33, 0x04, 0x07, 0xa0, 0x7d, 0x9e, 0x06}
DEFINE_GUID!{IID_ID3D10EffectBlendVariable,
    0x1fcd2294, 0xdf6d, 0x4eae, 0x86, 0xb3, 0x0e, 0x91, 0x60, 0xcf, 0xb0, 0x7b}
DEFINE_GUID!{IID_ID3D10EffectDepthStencilVariable,
    0xaf482368, 0x330a, 0x46a5, 0x9a, 0x5c, 0x01, 0xc7, 0x1a, 0xf2, 0x4c, 0x8d}
DEFINE_GUID!{IID_ID3D10EffectRasterizerVariable,
    0x21af9f0e, 0x4d94, 0x4ea9, 0x97, 0x85, 0x2c, 0xb7, 0x6b, 0x8c, 0x0b, 0x34}
DEFINE_GUID!{IID_ID3D10EffectSamplerVariable,
    0x6530d5c7, 0x07e9, 0x4271, 0xa4, 0x18, 0xe7, 0xce, 0x4b, 0xd1, 0xe4, 0x80}
DEFINE_GUID!{IID_ID3D10EffectPass,
    0x5cfbeb89, 0x1a06, 0x46e0, 0xb2, 0x82, 0xe3, 0xf9, 0xbf, 0xa3, 0x6a, 0x54}
DEFINE_GUID!{IID_ID3D10EffectTechnique,
    0xdb122ce8, 0xd1c9, 0x4292, 0xb2, 0x37, 0x24, 0xed, 0x3d, 0xe8, 0xb1, 0x75}
DEFINE_GUID!{IID_ID3D10Effect,
    0x51b0ca8b, 0xec0b, 0x4519, 0x87, 0x0d, 0x8e, 0xe1, 0xcb, 0x50, 0x17, 0xc7}
DEFINE_GUID!{IID_ID3D10EffectPool,
    0x9537ab04, 0x3250, 0x412e, 0x82, 0x13, 0xfc, 0xd2, 0xf8, 0x67, 0x79, 0x33}
