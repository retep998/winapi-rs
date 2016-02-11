// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3dcompiler.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn D3DCompile();
    // pub fn D3DCompile2();
    // pub fn D3DCompileFromFile();
    // pub fn D3DCompressShaders();
    // pub fn D3DCreateBlob();
    // pub fn D3DCreateFunctionLinkingGraph();
    // pub fn D3DCreateLinker();
    // pub fn D3DDecompressShaders();
    // pub fn D3DDisassemble();
    // pub fn D3DDisassemble10Effect();
    // pub fn D3DDisassemble11Trace();
    // pub fn D3DDisassembleRegion();
    // pub fn D3DGetBlobPart();
    // pub fn D3DGetDebugInfo();
    // pub fn D3DGetInputAndOutputSignatureBlob();
    // pub fn D3DGetInputSignatureBlob();
    // pub fn D3DGetOutputSignatureBlob();
    // pub fn D3DGetTraceInstructionOffsets();
    // pub fn D3DLoadModule();
    // pub fn D3DPreprocess();
    // pub fn D3DReadFileToBlob();
    // pub fn D3DReflect();
    // pub fn D3DReflectLibrary();
    // pub fn D3DReturnFailure1();
    // pub fn D3DSetBlobPart();
    // pub fn D3DStripShader();
    // pub fn D3DWriteBlobToFile();
}
