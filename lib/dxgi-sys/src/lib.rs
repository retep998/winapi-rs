// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dxgi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn CreateDXGIFactory();
    // pub fn CreateDXGIFactory1();
    // pub fn CreateDXGIFactory2();
    // pub fn DXGID3D10CreateDevice();
    // pub fn DXGID3D10CreateLayeredDevice();
    // pub fn DXGID3D10GetLayeredDeviceSize();
    // pub fn DXGID3D10RegisterLayers();
    // pub fn DXGIGetDebugInterface1();
    // pub fn DXGIReportAdapterConfiguration();
}
