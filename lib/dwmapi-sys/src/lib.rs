// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dwmapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn DwmEnableBlurBehindWindow(hWnd: HWND, pBlurBehind: *const winapi::dwmapi::DWM_BLURBEHIND) -> HRESULT;
}
