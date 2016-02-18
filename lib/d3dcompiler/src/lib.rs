// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3dcompiler.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn D3DCompile(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO, pInclude: *mut ID3DInclude, pEntrypoint: LPCSTR,
        pTarget: LPCSTR, Flags1: UINT, Flags2: UINT, ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCompile2(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO, pInclude: *mut ID3DInclude, pEntrypoint: LPCSTR,
        pTarget: LPCSTR, Flags1: UINT, Flags2: UINT, SecondaryDataFlags: UINT,
        pSecondaryData: LPCVOID, SecondaryDataSize: SIZE_T, ppCode: *mut *mut ID3DBlob,
        ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCompileFromFile(
        pFileName: LPCWSTR, pDefines: *const D3D_SHADER_MACRO, pInclude: *mut ID3DInclude,
        pEntrypoint: LPCSTR, pTarget: LPCSTR, Flags1: UINT, Flags2: UINT,
        ppCode: *mut *mut ID3DBlob, ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCompressShaders(
        uNumShaders: UINT, pShaderData: *mut D3D_SHADER_DATA, uFlags: UINT,
        ppCompressedData: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DCreateBlob(Size: SIZE_T, ppBlob: *mut *mut ID3DBlob) -> HRESULT;
    pub fn D3DCreateFunctionLinkingGraph(
        uFlags: UINT, ppFunctionLinkingGraph: *mut *mut ID3D11FunctionLinkingGraph,
    ) -> HRESULT;
    pub fn D3DCreateLinker(ppLinker: *mut *mut ID3D11Linker) -> HRESULT;
    pub fn D3DDecompressShaders(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, uNumShaders: UINT, uStartIndex: UINT,
        pIndices: *mut UINT, uFlags: UINT, ppShaders: *mut *mut ID3DBlob, pTotalShaders: *mut UINT,
    ) -> HRESULT;
    pub fn D3DDisassemble(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, Flags: UINT, szComments: LPCSTR,
        ppDisassembly: *mut *mut ID3DBlob,
    ) -> HRESULT;
    // pub fn D3DDisassemble10Effect(
    //     pEffect: *mut ID3D10Effect, Flags: UINT, ppDisassembly: *mut *mut ID3DBlob,
    // ) -> HRESULT;
    // pub fn D3DDisassemble11Trace();
    pub fn D3DDisassembleRegion(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, Flags: UINT, szComments: LPCSTR,
        StartByteOffset: SIZE_T, NumInsts: SIZE_T, pFinishByteOffset: *mut SIZE_T,
        ppDisassembly: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetBlobPart(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, Part: D3D_BLOB_PART, Flags: UINT,
        ppPart: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetDebugInfo(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, ppDebugInfo: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetInputAndOutputSignatureBlob(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetInputSignatureBlob(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetOutputSignatureBlob(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, ppSignatureBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DGetTraceInstructionOffsets(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, Flags: UINT, StartInstIndex: SIZE_T,
        NumInsts: SIZE_T, pOffsets: *mut SIZE_T, pTotalInsts: *mut SIZE_T,
    ) -> HRESULT;
    pub fn D3DLoadModule(
        pSrcData: LPCVOID, cbSrcDataSize: SIZE_T, ppModule: *mut *mut ID3D11Module,
    ) -> HRESULT;
    pub fn D3DPreprocess(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, pSourceName: LPCSTR,
        pDefines: *const D3D_SHADER_MACRO, pInclude: *mut ID3DInclude,
        ppCodeText: *mut *mut ID3DBlob, ppErrorMsgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DReadFileToBlob(pFileName: LPCWSTR, ppContents: *mut *mut ID3DBlob) -> HRESULT;
    pub fn D3DReflect(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, pInterface: REFIID, ppReflector: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3DReflectLibrary(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, riid: REFIID, ppReflector: *mut LPVOID,
    ) -> HRESULT;
    // pub fn D3DReturnFailure1();
    pub fn D3DSetBlobPart(
        pSrcData: LPCVOID, SrcDataSize: SIZE_T, Part: D3D_BLOB_PART, Flags: UINT, pPart: LPCVOID,
        PartSize: SIZE_T, ppNewShader: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DStripShader(
        pShaderBytecode: LPCVOID, BytecodeLength: SIZE_T, uStripFlags: UINT,
        ppStrippedBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
    pub fn D3DWriteBlobToFile(
        pBlob: *mut ID3DBlob, pFileName: LPCWSTR, bOverwrite: BOOL,
    ) -> HRESULT;
}
