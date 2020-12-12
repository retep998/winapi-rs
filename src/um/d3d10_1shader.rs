// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use um::d3d10shader::{D3D10_SHADER_VARIABLE_TYPE, D3D10_SHADER_VARIABLE_CLASS, ID3D10ShaderReflection, ID3D10ShaderReflectionVtbl, D3D10_SHADER_DESC, ID3D10ShaderReflectionConstantBuffer, D3D10_SIGNATURE_PARAMETER_DESC, D3D10_SHADER_INPUT_BIND_DESC, ID3D10ShaderReflectionVariable};
use shared::minwindef::{UINT, FLOAT, BOOL};
use um::winnt::{INT, LPCSTR};
use um::d3d10::D3D10_PRIMITIVE;
ENUM!{enum D3D10_SHADER_DEBUG_REGTYPE {
    D3D10_SHADER_DEBUG_REG_INPUT,
    D3D10_SHADER_DEBUG_REG_OUTPUT,
    D3D10_SHADER_DEBUG_REG_CBUFFER,
    D3D10_SHADER_DEBUG_REG_TBUFFER,
    D3D10_SHADER_DEBUG_REG_TEMP,
    D3D10_SHADER_DEBUG_REG_TEMPARRAY,
    D3D10_SHADER_DEBUG_REG_TEXTURE,
    D3D10_SHADER_DEBUG_REG_SAMPLER,
    D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER,
    D3D10_SHADER_DEBUG_REG_LITERAL,
    D3D10_SHADER_DEBUG_REG_UNUSED,
    D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS,
    D3D11_SHADER_DEBUG_REG_UAV,
    D3D10_SHADER_DEBUG_REG_FORCE_DWORD = 0x7fffffff,
}}
ENUM!{enum D3D10_SHADER_DEBUG_SCOPETYPE {
    D3D10_SHADER_DEBUG_SCOPE_GLOBAL,
    D3D10_SHADER_DEBUG_SCOPE_BLOCK,
    D3D10_SHADER_DEBUG_SCOPE_FORLOOP,
    D3D10_SHADER_DEBUG_SCOPE_STRUCT,
    D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS,
    D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK,
    D3D10_SHADER_DEBUG_SCOPE_NAMESPACE,
    D3D10_SHADER_DEBUG_SCOPE_ANNOTATION,
    D3D10_SHADER_DEBUG_SCOPE_FORCE_DWORD = 0x7fffffff,
}}
ENUM!{enum D3D10_SHADER_DEBUG_VARTYPE {
    D3D10_SHADER_DEBUG_VAR_VARIABLE,
    D3D10_SHADER_DEBUG_VAR_FUNCTION,
    D3D10_SHADER_DEBUG_VAR_FORCE_DWORD = 0x7fffffff,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    File: UINT,
    Line: UINT,
    Column: UINT,
    TokenLength: UINT,
    TokenId: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_VAR_INFO {
    TokenId: UINT,
    Type: D3D10_SHADER_VARIABLE_TYPE,
    Register: UINT,
    Component: UINT,
    ScopeVar: UINT,
    ScopeVarOffset: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_INPUT_INFO {
    Var: UINT,
    InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    InitialBank: UINT,
    InitialRegister: UINT,
    InitialComponent: UINT,
    InitialValue: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    TokenId: UINT,
    VarType: D3D10_SHADER_DEBUG_VARTYPE,
    Class: D3D10_SHADER_VARIABLE_CLASS,
    Rows: UINT,
    Columns: UINT,
    StructMemberScope: UINT,
    uArrayIndices: UINT,
    ArrayElements: UINT,
    ArrayStrides: UINT,
    uVariables: UINT,
    uFirstVariable: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    Name: UINT,
    uNameLen: UINT,
    uVariables: UINT,
    VariableData: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    Var: UINT,
    uValueMin: UINT,
    uValueMax: UINT,
    iValueMin: INT,
    iValueMax: INT,
    fValueMin: FLOAT,
    fValueMax: FLOAT,
    bNaNPossible: BOOL,
    bInfPossible: BOOL,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    OutputRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    OutputReg: UINT,
    TempArrayReg: UINT,
    OutputComponents: [UINT; 4],
    OutputVars: [D3D10_SHADER_DEBUG_OUTPUTVAR; 4],
    IndexReg: UINT,
    IndexComp: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_INST_INFO {
    Id: UINT,
    Opcode: UINT,
    uOutputs: UINT,
    pOutputs: [D3D10_SHADER_DEBUG_OUTPUTREG_INFO; 2],
    TokenId: UINT,
    NestingLevel: UINT,
    Scopes: UINT,
    ScopeInfo: UINT,
    AccessedVars: UINT,
    AccessedVarsInfo: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_FILE_INFO {
    FileName: UINT,
    FileNameLen: UINT,
    FileData: UINT,
    FileLen: UINT,
}}
STRUCT!{struct D3D10_SHADER_DEBUG_INFO {
    Size: UINT,
    Creator: UINT,
    EntrypointName: UINT,
    ShaderTarget: UINT,
    CompileFlags: UINT,
    Files: UINT,
    FileInfo: UINT,
    Instructions: UINT,
    InstructionInfo: UINT,
    Variables: UINT,
    VariableInfo: UINT,
    InputVariables: UINT,
    InputVariableInfo: UINT,
    Tokens: UINT,
    TokenInfo: UINT,
    Scopes: UINT,
    ScopeInfo: UINT,
    ScopeVariables: UINT,
    ScopeVariableInfo: UINT,
    UintOffset: UINT,
    StringOffset: UINT,
}}
RIDL!{#[uuid(0xc3457783, 0xa846, 0x47ce, 0x95, 0x20, 0xce, 0xa6, 0xf6, 0x6e, 0x74, 0x47)]
interface ID3D10ShaderReflection1(ID3D10ShaderReflection1Vtbl): ID3D10ShaderReflection(ID3D10ShaderReflectionVtbl) {
    fn GetDesc(
        pDesc: *mut D3D10_SHADER_DESC,
    ) -> (),
    fn GetConstantBufferByIndex(
        Index: UINT,
    ) -> *mut ID3D10ShaderReflectionConstantBuffer,
    fn GetConstantBufferByName (
        Name: LPCSTR,
    ) -> *mut ID3D10ShaderReflectionConstantBuffer,
    fn GetResourceBindingDesc(
        ResourceIndex: UINT,
        pDesc: *mut D3D10_SHADER_INPUT_BIND_DESC,
    ) -> (),
    fn GetInputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> (),
    fn GetOutputParameterDesc(
        ParameterIndex: UINT,
        pDesc: *mut D3D10_SIGNATURE_PARAMETER_DESC,
    ) -> (),
    fn GetVariableByName(
        Name: LPCSTR,
    ) -> *mut ID3D10ShaderReflectionVariable,
    fn GetResourceBindingDescByName(
        Name: LPCSTR,
        pDesc: *mut D3D10_SHADER_INPUT_BIND_DESC,
    ) -> (),
    fn GetMovInstructionCount(
        Count: *mut UINT,
    ) -> (),
    fn GetMovcInstructionCount(
        Count: *mut UINT,
    ) -> (),
    fn GetConversionInstructionCount(
        Count: *mut UINT,
    ) -> (),
    fn GetBitwiseInstructionCount(
        Count: *mut UINT,
    ) -> (),
    fn GetGSInputPrimitive(
        pPrim: *mut D3D10_PRIMITIVE,
    ) -> (),
    fn IsLevel9Shader(
        pbLevel9Shader: *mut BOOL,
    ) -> (),
    fn IsSampleFrequencyShader(
        pbSampleFrequency: *mut BOOL,
    ) -> (),
}}
DEFINE_GUID!{IID_ID3D10ShaderReflection1,
    0xc3457783, 0xa846, 0x47ce, 0x95, 0x20, 0xce, 0xa6, 0xf6, 0x6e, 0x74, 0x47}
