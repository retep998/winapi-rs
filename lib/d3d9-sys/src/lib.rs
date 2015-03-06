// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d3d9.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(target_arch = "x86")]
extern "cdecl" {
    // pub fn DebugSetLevel();
    // pub fn DebugSetMute();
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn D3DPERF_BeginEvent();
    // pub fn D3DPERF_EndEvent();
    // pub fn D3DPERF_GetStatus();
    // pub fn D3DPERF_QueryRepeatFrame();
    // pub fn D3DPERF_SetMarker();
    // pub fn D3DPERF_SetOptions();
    // pub fn D3DPERF_SetRegion();
    // pub fn Direct3DCreate9();
    // pub fn Direct3DCreate9Ex();
}
#[cfg(any(target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn DebugSetLevel();
    // pub fn DebugSetMute();
}
