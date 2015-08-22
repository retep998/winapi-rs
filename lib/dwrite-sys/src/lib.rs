// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dwrite.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    pub fn DWriteCreateFactory(
        factoryType: DWRITE_FACTORY_TYPE, iid: REFIID, factory: *mut *mut IUnknown
    ) -> HRESULT;
}
