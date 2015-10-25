// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dxgi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    pub fn CreateDXGIFactory(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
    pub fn CreateDXGIFactory1(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
    // pub fn DXGID3D10CreateDevice();
    // pub fn DXGID3D10CreateLayeredDevice();
    // pub fn DXGID3D10GetLayeredDeviceSize();
    // pub fn DXGID3D10RegisterLayers();
    // pub fn DXGIReportAdapterConfiguration();
}
#[cfg(target_env = "msvc")] 
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
//    pub fn CreateDXGIFactory2(
//        Flags: UINT, riid: REFGUID, ppFactory: *mut *mut c_void
//    ) -> HRESULT;
//    pub fn DXGIGetDebugInterface1(
//        Flags: UINT, riid: REFGUID, pDebug: *mut *mut c_void
//    ) -> HRESULT;
}
