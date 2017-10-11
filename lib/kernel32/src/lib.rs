// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);
}
