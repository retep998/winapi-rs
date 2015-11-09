// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dsound.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn DirectSoundCreate(
        pcGuidDevice: LPCGUID, ppDS: *mut LPDIRECTSOUND, pUnkOuter: LPUNKNOWN
    ) -> HRESULT;
}

