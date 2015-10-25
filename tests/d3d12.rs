// Copyright © 2015, Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
#![cfg(target_env = "msvc")]
extern crate d3d12_sys;
use d3d12_sys::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
//    bb(D3D12CreateDevice);
//    bb(D3D12CreateRootSignatureDeserializer);
//    bb(D3D12GetDebugInterface);
//    bb(D3D12SerializeRootSignature);
}
