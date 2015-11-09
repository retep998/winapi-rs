// Copyright © 2015, Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of d3d12.h
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn D3D12CreateDevice(
       pAdapter: *mut IUnknown, MinimumFeatureLevel: D3D_FEATURE_LEVEL,
       riid: REFGUID, ppDevice: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12CreateRootSignatureDeserializer(
       pSrcData: LPCVOID, SrcDataSizeInBytes: SIZE_T,
       pRootSignatureDeserializerInterface: REFGUID,
       ppRootSignatureDeserializer: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12GetDebugInterface(riid: REFGUID, ppvDebug: *mut *mut c_void) -> HRESULT;
    pub fn D3D12SerializeRootSignature(
       pRootSignature: *const D3D12_ROOT_SIGNATURE_DESC,
       Version: D3D_ROOT_SIGNATURE_VERSION, ppBlob: *mut *mut ID3DBlob,
       ppErrorBlob: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
