// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3d9.
#![cfg(windows)]
extern crate winapi;
use winapi::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    pub fn D3DPERF_BeginEvent(col: D3DCOLOR, wszName: LPCWSTR) -> INT;
    pub fn D3DPERF_EndEvent() -> INT;
    pub fn D3DPERF_GetStatus() -> DWORD;
    pub fn D3DPERF_QueryRepeatFrame() -> BOOL;
    pub fn D3DPERF_SetMarker(col: D3DCOLOR, wszName: LPCWSTR) -> ();
    pub fn D3DPERF_SetOptions(dwOptions: DWORD) -> ();
    pub fn D3DPERF_SetRegion(col: D3DCOLOR, wszName: LPCWSTR) -> ();
    pub fn Direct3DCreate9(SDKVersion: UINT) -> *mut IDirect3D9;
    pub fn Direct3DCreate9Ex(SDKVersion: UINT, arg1: *mut *mut IDirect3D9Ex) -> HRESULT;
}