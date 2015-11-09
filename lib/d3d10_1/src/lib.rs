// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3d10_1.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn D3D10CompileEffectFromMemory();
    // pub fn D3D10CompileShader();
    // pub fn D3D10CreateBlob();
    // pub fn D3D10CreateDevice1();
    // pub fn D3D10CreateDeviceAndSwapChain1();
    // pub fn D3D10CreateEffectFromMemory();
    // pub fn D3D10CreateEffectPoolFromMemory();
    // pub fn D3D10CreateStateBlock();
    // pub fn D3D10DisassembleEffect();
    // pub fn D3D10DisassembleShader();
    // pub fn D3D10GetGeometryShaderProfile();
    // pub fn D3D10GetInputAndOutputSignatureBlob();
    // pub fn D3D10GetInputSignatureBlob();
    // pub fn D3D10GetOutputSignatureBlob();
    // pub fn D3D10GetPixelShaderProfile();
    // pub fn D3D10GetShaderDebugInfo();
    // pub fn D3D10GetVersion();
    // pub fn D3D10GetVertexShaderProfile();
    // pub fn D3D10PreprocessShader();
    // pub fn D3D10ReflectShader();
    // pub fn D3D10RegisterLayers();
    // pub fn D3D10StateBlockMaskDifference();
    // pub fn D3D10StateBlockMaskDisableAll();
    // pub fn D3D10StateBlockMaskDisableCapture();
    // pub fn D3D10StateBlockMaskEnableAll();
    // pub fn D3D10StateBlockMaskEnableCapture();
    // pub fn D3D10StateBlockMaskGetSetting();
    // pub fn D3D10StateBlockMaskIntersect();
    // pub fn D3D10StateBlockMaskUnion();
}
