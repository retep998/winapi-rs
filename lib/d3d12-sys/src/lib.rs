// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3d12.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    pub fn D3D12CreateDevice(
        pAdapter: *mut IUnknown, MinimumFeatureLevel: D3D_FEATURE_LEVEL, riid: REFIID,
        ppDevice: *mut *mut c_void
    ) -> HRESULT;

    pub fn D3D12GetDebugInterface(riid: REFIID, ppvDebug: *mut *mut c_void) -> HRESULT;

    pub fn D3D12SerializeRootSignature(
    	pRootSignature: *const D3D12_ROOT_SIGNATURE_DESC,
    	Version: D3D_ROOT_SIGNATURE_VERSION, ppBlob: *mut *mut ID3DBlob,
    	ppErrorBlob: *mut *mut ID3DBlob
    ) -> HRESULT;

    pub fn D3D12CreateRootSignatureDeserializer(
    	pSrcData: LPCVOID, SrcDataSizeInBytes: SIZE_T,
    	pRootSignatureDeserializerInterface: REFIID,
    	ppRootSignatureDeserializer: *mut *mut c_void
    ) -> HRESULT;
}
