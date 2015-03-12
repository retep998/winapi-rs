// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3d11.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn D3D11CoreCreateDevice();
    // pub fn D3D11CoreCreateLayeredDevice();
    // pub fn D3D11CoreGetLayeredDeviceSize();
    // pub fn D3D11CoreRegisterLayers();
    // pub fn D3D11CreateDevice();
    // pub fn D3D11CreateDeviceAndSwapChain();
    // pub fn D3DKMTCreateAllocation();
    // pub fn D3DKMTCreateContext();
    // pub fn D3DKMTCreateDevice();
    // pub fn D3DKMTCreateSynchronizationObject();
    // pub fn D3DKMTEscape();
    // pub fn D3DKMTGetContextSchedulingPriority();
    // pub fn D3DKMTGetDeviceState();
    // pub fn D3DKMTGetDisplayModeList();
    // pub fn D3DKMTGetMultisampleMethodList();
    // pub fn D3DKMTGetRuntimeData();
    // pub fn D3DKMTGetSharedPrimaryHandle();
    // pub fn D3DKMTLock();
    // pub fn D3DKMTOpenAdapterFromHdc();
    // pub fn D3DKMTOpenResource();
    // pub fn D3DKMTPresent();
    // pub fn D3DKMTQueryAllocationResidency();
    // pub fn D3DKMTQueryResourceInfo();
    // pub fn D3DKMTRender();
    // pub fn D3DKMTSetAllocationPriority();
    // pub fn D3DKMTSetContextSchedulingPriority();
    // pub fn D3DKMTSetDisplayMode();
    // pub fn D3DKMTSetGammaRamp();
    // pub fn D3DKMTSetVidPnSourceOwner();
    // pub fn D3DKMTWaitForVerticalBlankEvent();
}
