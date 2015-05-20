// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to comctl32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
	pub fn InitCommonControlsEx(lpInitCtrls: *const INITCOMMONCONTROLSEX) -> BOOL;
}
